use std::cell::RefCell;
use std::fmt;
use std::rc::Rc;

use super::measurement::Measurement;
use super::node::Node;
use super::position::SwitchgearPosition;
use super::terminal::Terminal;

/// Component Type
#[derive(Debug, PartialEq)]
pub enum ComponentType {
    /// Circuit Breaker
    CircuitBreaker,
    /// Disconnector
    Disconnector,
    /// Earthing Switch
    EarthingSwitch,
    /// Voltage Transformer
    VoltageTransformer,
    /// Transformer
    Transformer,
}

impl fmt::Display for ComponentType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let typestr: &str;
        match self {
            ComponentType::CircuitBreaker => typestr = "CircuitBreaker",
            ComponentType::Disconnector => typestr = "Disconnector",
            ComponentType::EarthingSwitch => typestr = "Earthing Switch",
            ComponentType::VoltageTransformer => typestr = "Voltage Transformer",
            ComponentType::Transformer => typestr = "Transformer",
        }
        write!(f, "{}", typestr)
    }
}

/// Trait to define components. Each component should have a [ComponentType] and at least one [Terminal]
pub trait Component {
    /// Constructor; sets the component name
    fn new(name: &str) -> Self
    where
        Self: Sized;

    /// Returns the [ComponentType] of the component
    fn r#type(&self) -> ComponentType;
    /// Returns the name of the component
    fn name(&self) -> &String;
    /// Returns the terminal with a given index, or an error if the component has less terminals than the given index.
    fn terminal(&self, index: usize) -> Result<&RefCell<Terminal>, String>;

    /// Connect the component to a node on a given terminal index
    ///
    /// Only allow a connection if
    /// * The given node is not already connected to another terminal
    /// * The given terminal index exists (the component has less terminals than the given index)
    /// * The given terminal is not already connected to another node
    fn connect(&self, node: Rc<Node>, terminal_index: usize) -> Result<(), String> {
        let mut i = 0;
        loop {
            if i == terminal_index {
                i += 1;
                continue;
            }
            match self.terminal(i) {
                Err(_) => {
                    break;
                }
                Ok(t) => {
                    if let Ok(n) = t.borrow().get_node() {
                        if Rc::ptr_eq(&node, &n) {
                            return Err(format!(
                                "Component {} is already connected to node {} on terminal {}",
                                self.name(),
                                node.name(),
                                i
                            ));
                        }
                    }
                }
            }
            i += 1;
        }
        let t = self.terminal(terminal_index)?;
        t.borrow_mut().connect(node)?;
        Ok(())
    }

    /// Disconnect the component from the given node. Returns an error if not connected to it.
    fn disconnect(&self, node: Rc<Node>) -> Result<(), String> {
        let mut i = 0;
        while let Ok(t) = self.terminal(i) {
            let mut t = t.borrow_mut();
            if let Ok(n) = t.get_node() {
                if Rc::ptr_eq(&n, &node) {
                    return t.disconnect();
                }
            }
            i += 1;
        }
        Err(format!(
            "Component {} is not connnected to node {}",
            self.name(),
            node.name()
        ))
    }

    /// Get component [SwitchgearPosition], only implemented for switchgear
    fn position(&self) -> Result<&RefCell<SwitchgearPosition>, String> {
        Err(format!(
            "Components of type {} have no position",
            self.r#type()
        ))
    }

    /// Open switchgear
    fn open(&self) -> Result<(), String> {
        let pos = self.position()?;
        pos.borrow_mut().open()?;
        Ok(())
    }

    /// Close switchgear
    fn close(&self) -> Result<(), String> {
        let pos = self.position()?;
        pos.borrow_mut().close()?;
        Ok(())
    }

    /// Update measurement value
    fn update(&self, _value: f64) -> Result<(), String> {
        Err(format!(
            "Components of type {} have no measurement",
            self.r#type()
        ))
    }

    /// Get measurement value
    fn value(&self) -> Result<f64, String> {
        Err(format!(
            "Components of type {} have no measurement",
            self.r#type()
        ))
    }
}

impl fmt::Display for dyn Component {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Component {} of type {:?}", self.name(), self.r#type())
    }
}

/// Circuit Breaker
pub struct CircuitBreaker {
    name: String,
    position: RefCell<SwitchgearPosition>,
    terminals: [RefCell<Terminal>; 2],
}

impl Component for CircuitBreaker {
    fn new(name: &str) -> CircuitBreaker {
        CircuitBreaker {
            name: name.to_string(),
            position: RefCell::new(SwitchgearPosition::new()),
            terminals: [RefCell::new(Terminal::new()), RefCell::new(Terminal::new())],
        }
    }

    fn r#type(&self) -> ComponentType {
        ComponentType::CircuitBreaker
    }

    fn name(&self) -> &String {
        &self.name
    }

    fn position(&self) -> Result<&RefCell<SwitchgearPosition>, String> {
        Ok(&self.position)
    }

    fn terminal(&self, index: usize) -> Result<&RefCell<Terminal>, String> {
        match self.terminals.get(index) {
            Some(t) => Ok(t),
            None => Err(format!("Component {} of type {:?} does not have a terminal with index {}; it only has {} terminals", self.name, self.r#type(), index, self.terminals.len())),
        }
    }
}

/// Disconnector
pub struct Disconnector {
    name: String,
    position: RefCell<SwitchgearPosition>,
    terminals: [RefCell<Terminal>; 2],
}

impl Component for Disconnector {
    fn new(name: &str) -> Disconnector {
        Disconnector {
            name: name.to_string(),
            position: RefCell::new(SwitchgearPosition::new()),
            terminals: [RefCell::new(Terminal::new()), RefCell::new(Terminal::new())],
        }
    }

    fn r#type(&self) -> ComponentType {
        ComponentType::Disconnector
    }

    fn name(&self) -> &String {
        &self.name
    }

    fn position(&self) -> Result<&RefCell<SwitchgearPosition>, String> {
        Ok(&self.position)
    }

    fn terminal(&self, index: usize) -> Result<&RefCell<Terminal>, String> {
        match self.terminals.get(index) {
            Some(t) => Ok(t),
            None => Err(format!("Component {} of type {:?} does not have a terminal with index {}; it only has {} terminals", self.name, self.r#type(), index, self.terminals.len())),
        }
    }
}

/// Earthing Switch
pub struct EarthingSwitch {
    name: String,
    position: RefCell<SwitchgearPosition>,
    terminals: [RefCell<Terminal>; 1],
}

impl Component for EarthingSwitch {
    fn new(name: &str) -> EarthingSwitch {
        EarthingSwitch {
            name: name.to_string(),
            position: RefCell::new(SwitchgearPosition::new()),
            terminals: [RefCell::new(Terminal::new()); 1],
        }
    }

    fn r#type(&self) -> ComponentType {
        ComponentType::EarthingSwitch
    }

    fn name(&self) -> &String {
        &self.name
    }

    fn terminal(&self, index: usize) -> Result<&RefCell<Terminal>, String> {
        match self.terminals.get(index) {
            Some(t) => Ok(t),
            None => Err(format!("Component {} of type {:?} does not have a terminal with index {}; it only has {} terminals", self.name, self.r#type(), index, self.terminals.len())),
        }
    }

    fn position(&self) -> Result<&RefCell<SwitchgearPosition>, String> {
        Ok(&self.position)
    }
}

/// Voltage Transformer
pub struct VoltageTransformer {
    name: String,
    measurement: RefCell<Measurement>,
    terminals: [RefCell<Terminal>; 1],
}

impl Component for VoltageTransformer {
    fn new(name: &str) -> VoltageTransformer {
        VoltageTransformer {
            name: name.to_string(),
            measurement: RefCell::new(Measurement::new()),
            terminals: [RefCell::new(Terminal::new())],
        }
    }

    fn r#type(&self) -> ComponentType {
        ComponentType::VoltageTransformer
    }

    fn name(&self) -> &String {
        &self.name
    }

    fn terminal(&self, index: usize) -> Result<&RefCell<Terminal>, String> {
        match self.terminals.get(index) {
            Some(t) => Ok(t),
            None => Err(format!("Component {} of type {:?} does not have a terminal with index {}; it only has {} terminals", self.name, self.r#type(), index, self.terminals.len())),
        }
    }

    fn update(&self, value: f64) -> Result<(), String> {
        self.measurement.borrow_mut().update(value);
        Ok(())
    }

    fn value(&self) -> Result<f64, String> {
        Ok(self.measurement.borrow().value())
    }
}

/// Transformer
pub struct Transformer {
    name: String,
    terminals: [RefCell<Terminal>; 3],
}

impl Component for Transformer {
    fn new(name: &str) -> Transformer {
        Transformer {
            name: name.to_string(),
            terminals: [
                RefCell::new(Terminal::new()),
                RefCell::new(Terminal::new()),
                RefCell::new(Terminal::new()),
            ],
        }
    }

    fn r#type(&self) -> ComponentType {
        ComponentType::Transformer
    }

    fn name(&self) -> &String {
        &self.name
    }

    fn terminal(&self, index: usize) -> Result<&RefCell<Terminal>, String> {
        match self.terminals.get(index) {
            Some(t) => Ok(t),
            None => Err(format!("Component {} of type {:?} does not have a terminal with index {}; it only has {} terminals", self.name, self.r#type(), index, self.terminals.len())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_components() -> (
        CircuitBreaker,
        Disconnector,
        EarthingSwitch,
        VoltageTransformer,
        Transformer,
    ) {
        let cb = CircuitBreaker::new("cb");
        let ds = Disconnector::new("ds");
        let es = EarthingSwitch::new("es");
        let vt = VoltageTransformer::new("vt");
        let tf = Transformer::new("tf");

        (cb, ds, es, vt, tf)
    }

    #[test]
    fn component_names() {
        let (cb, ds, es, vt, tf) = create_test_components();

        assert_eq!(cb.name(), "cb");
        assert_eq!(ds.name(), "ds");
        assert_eq!(es.name(), "es");
        assert_eq!(vt.name(), "vt");
        assert_eq!(tf.name(), "tf");
    }

    #[test]
    fn component_types() {
        let (cb, ds, es, vt, tf) = create_test_components();

        assert_eq!(cb.r#type(), ComponentType::CircuitBreaker);
        assert_eq!(ds.r#type(), ComponentType::Disconnector);
        assert_eq!(es.r#type(), ComponentType::EarthingSwitch);
        assert_eq!(vt.r#type(), ComponentType::VoltageTransformer);
        assert_eq!(tf.r#type(), ComponentType::Transformer)
    }

    #[test]
    fn component_terminals() {
        let (cb, ds, es, vt, tf) = create_test_components();

        assert_eq!(cb.terminals.len(), 2);
        assert_eq!(ds.terminals.len(), 2);
        assert_eq!(es.terminals.len(), 1);
        assert_eq!(vt.terminals.len(), 1);
        assert_eq!(tf.terminals.len(), 3);

        assert!(cb.terminal(2).is_err());
        assert!(ds.terminal(2).is_err());
        assert!(es.terminal(1).is_err());
        assert!(vt.terminal(1).is_err());
        assert!(tf.terminal(3).is_err());
    }

    #[test]
    fn component_connect() {
        let n = Rc::new(Node::new("node"));
        let n2 = Rc::new(Node::new("node2"));
        let cb = CircuitBreaker::new("cb");

        assert!(cb.disconnect(n.clone()).is_err());
        assert!(cb.connect(n.clone(), 2).is_err());
        assert!(cb.connect(n.clone(), 0).is_ok());
        assert!(cb.connect(n.clone(), 0).is_err());
        assert!(cb.connect(n.clone(), 1).is_err());
        assert!(cb.connect(n2.clone(), 1).is_ok());
        assert!(cb.disconnect(n2.clone()).is_ok());
        assert!(cb.connect(n2.clone(), 0).is_err());
    }

    #[test]
    fn component_openclose() {
        let (cb, ds, es, vt, tf) = create_test_components();

        assert!(vt.close().is_err());
        assert!(tf.close().is_err());

        assert!(cb.open().is_err());
        assert!(cb.close().is_ok());
        assert!(cb.close().is_err());
        assert!(cb.open().is_ok());

        assert!(ds.open().is_err());
        assert!(ds.close().is_ok());
        assert!(ds.close().is_err());
        assert!(ds.open().is_ok());

        assert!(es.open().is_err());
        assert!(es.close().is_ok());
        assert!(es.close().is_err());
        assert!(es.open().is_ok());
    }

    #[test]
    fn component_update() {
        let (cb, ds, es, vt, tf) = create_test_components();

        assert!(cb.update(0f64).is_err());
        assert!(ds.update(0f64).is_err());
        assert!(es.update(0f64).is_err());
        assert!(tf.update(0f64).is_err());

        assert_eq!(vt.value().unwrap(), 0.0);
        assert!(vt.update(1578.51758).is_ok());
        assert_eq!(vt.value().unwrap(), 1578.51758);
    }
}
