use std::cell::RefCell;
use std::fmt;
use std::rc::Rc;

use super::component::*;

/// A node, which can be connected to a set of [Component] via their [Terminal].
pub struct Node {
    name: String,
    children: RefCell<Vec<Rc<dyn Component>>>,
}

impl Node {
    /// Constructor sets the node name
    pub fn new(name: &str) -> Node {
        Node {
            name: name.to_string(),
            children: RefCell::new(vec![]),
        }
    }

    /// Return node name
    pub fn name(&self) -> &String {
        &self.name
    }

    /// Add component to node
    pub fn add_component(&self, c: Rc<dyn Component>) -> Result<(), String> {
        let index = self
            .children
            .borrow()
            .iter()
            .position(|x| Rc::ptr_eq(x, &c));
        match index {
            Some(_) => Err(format!(
                "Failed to add component {} to node {} - Component already exists on node",
                c.name(),
                self.name()
            )),
            None => {
                self.children.borrow_mut().push(c);
                Ok(())
            }
        }
    }

    /// Remove component from node
    pub fn remove_component(&self, c: Rc<dyn Component>) -> Result<(), String> {
        let index = self
            .children
            .borrow()
            .iter()
            .position(|x| Rc::ptr_eq(x, &c));
        match index {
            Some(i) => {
                self.children.borrow_mut().remove(i);
                Ok(())
            }
            None => Err(format!(
                "Failed to remove component {} from node {} - Component does not exist on node",
                c.name(),
                self.name()
            )),
        }
    }
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Node {}", self.name())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn node_name() {
        let n = Node::new("node");
        assert_eq!(n.name(), "node")
    }

    #[test]
    fn node_component() {
        let n = Node::new("node");

        let cb: Rc<dyn Component> = Rc::new(CircuitBreaker::new("cb"));
        let ds: Rc<dyn Component> = Rc::new(Disconnector::new("ds"));

        assert!(n.children.borrow().len() == 0);
        n.add_component(cb.clone()).unwrap();
        assert!(Rc::ptr_eq(&cb, &n.children.borrow()[0]));
        assert!(n.add_component(cb.clone()).is_err());
        assert!(n.children.borrow().len() == 1);

        n.add_component(ds.clone()).unwrap();
        assert!(Rc::ptr_eq(&ds, &n.children.borrow()[1]));
        assert!(n.children.borrow().len() == 2);

        n.remove_component(cb.clone()).unwrap();
        assert!(n.remove_component(cb).is_err());
        assert!(Rc::ptr_eq(&ds, &n.children.borrow()[0]));
        assert!(n.children.borrow().len() == 1);
        n.remove_component(ds).unwrap();
        assert!(n.children.borrow().len() == 0);
    }
}
