use std::cell::RefCell;
use std::fmt::Display;
use std::rc::Rc;

/// A type alias for a reference-counted, mutable `Node`.
pub type NodeRef<T> = Rc<RefCell<Node<T>>>;

/// A struct representing a collection of nodes.
pub struct Arena<T> {
    nodes: Vec<NodeRef<T>>,
}


impl<T> Arena<T> {
    /// Creates a new, empty `Arena`.
    ///
    /// # Examples
    ///
    /// ```
    /// let arena: Arena<i32> = Arena::new(); 
    /// ```
    pub fn new() -> Self {
        Arena { nodes: Vec::new() }
    }


    /// Adds a new node with the given value to the arena.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to be stored in the new node.
    ///
    /// # Returns
    ///
    /// A reference-counted, mutable reference to the newly created node.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut arena = Arena::new(); 
    /// let node = arena.add_node(10); 
    /// ```
    pub fn add_node(&mut self, value: T) -> NodeRef<T> {
        let node = new_node_ref(value);

        self.nodes.push(Rc::clone(&node));

        node
    }

    /// Retrieves the root node of the arena, if it exists.
    ///
    /// # Returns
    ///
    /// An `Option` containing a reference-counted, mutable reference to the root node, or `None` if the arena is empty.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut arena = Arena::new(); 
    /// arena.add_node(10); 
    /// let root = arena.get_root(); 
    /// ```
    pub fn get_root(&self) -> Option<NodeRef<T>> {
        self.nodes.first().cloned()
    }
}

impl<T> Default for Arena<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Display> Display for Arena<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "[")?;
        for (index, node) in self.nodes.iter().enumerate() {
            if index != 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}", node.borrow().value)?;
        }
        write!(f, "]")
    }
}

pub struct Node<T> {
    value: T,
    edges: Vec<NodeRef<T>>,
}

impl<T: Display> Display for Node<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl<T> Node<T> {
    pub fn new(value: T) -> Self {
        Node {
            value,
            edges: Vec::new(),
        }
    }
    
    /// Returns a reference to the value stored in the node.
    ///
    /// # Examples
    ///
    /// ```
    ///  let node = Node::new(10); 
    /// assert_eq!(*node.value(), 10); 
    /// ```
    pub fn value(&self) -> &T {
        &self.value
    }

    /// Adds an edge to the node.
    ///
    /// # Arguments
    ///
    /// * `edge` - A reference to the node to be added as an edge.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut arena = Arena::new();
    /// let mut node1 = arena.add_node(10);
    /// let node2 = arena.add_node(20);
    /// node1.add_edges(node2); 
    ///```
    pub fn add_edges(&mut self, edge: NodeRef<T>) {
        self.edges.push(edge);
    }


    /// Returns a reference to the list of edges.
    ///
    /// # Examples
    ///
    /// ```
    /// let node = Node::new(10); 
    /// let edges = node.get_edges(); 
    /// ``` 
    pub fn get_edges(&self) -> &Vec<NodeRef<T>> {
        &self.edges
    }
}

/// Creates a new reference-counted, mutable `Node`.
///
/// # Arguments
///
/// * `value` - The value to be stored in the node.
///
/// # Returns
///
/// A reference-counted, mutable reference to the newly created node.
///
/// # Examples
///
/// ```
/// let node = new_node_ref(10); 
/// ``` 
fn new_node_ref<T>(value: T) -> NodeRef<T> {
    Rc::new(RefCell::new(Node::new(value)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_node() {
        let mut arena: Arena<i32> = Arena::new();
        let node = arena.add_node(10);
        assert_eq!(arena.nodes.len(), 1);
        assert_eq!(*node.borrow().value(), 10);
    }

    #[test]
    fn test_add_edge() {
        let mut arena: Arena<i32> = Arena::new();
        let node1 = arena.add_node(10);
        let node2 = arena.add_node(20);

        node1.borrow_mut().add_edges(Rc::clone(&node2));

        assert_eq!(node1.borrow().edges.len(), 1);
        assert_eq!(*node1.borrow().edges[0].borrow().value(), 20);
    }

    #[test]
    fn test_display_arena() {
        let mut arena: Arena<i32> = Arena::new();
        arena.add_node(10);
        arena.add_node(20);

        assert_eq!(format!("{}", arena), "[10, 20]");
    }

    #[test]
    fn test_display_node() {
        let node = Node {
            value: 10,
            edges: Vec::new(),
        };

        assert_eq!(format!("{}", node), "10");
    }
}
