use std::cell::RefCell;
use std::fmt::{self, Debug, Formatter};
use std::rc::Rc;

type NodeId = usize;

/// A node in the computational graph.
pub struct Node {
    value: RefCell<Option<u32>>,
    is_hint: bool,
    parents: Vec<NodeId>,
    operation: RefCell<Option<Box<dyn Fn(u32, u32) -> u32>>>,
}

impl Node {
    pub fn new(value: Option<u32>, is_hint: bool, parents: Vec<NodeId>) -> Self {
        Self {
            value: RefCell::new(value),
            is_hint,
            parents,
            operation: RefCell::new(None),
        }
    }
}

impl Debug for Node {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("Node")
            .field("value", &self.value)
            .field("is_hint", &self.is_hint)
            .field("parents", &self.parents)
            .finish()
    }
}

/// A builder that will be used to create a computational graph.
pub struct Builder {
    nodes: Vec<Rc<Node>>,
    constraints: Vec<(NodeId, NodeId)>,
    node_counter: NodeId,
}

impl Builder {
    /// Creates a new builder.
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            constraints: Vec::new(),
            node_counter: 0,
        }
    }

    fn create_node(&mut self, value: Option<u32>, is_hint: bool, parents: Vec<NodeId>) -> NodeId {
        let node = Rc::new(Node::new(value, is_hint, parents));
        self.nodes.push(node);
        self.node_counter += 1;
        self.node_counter - 1
    }

    /// Initializes a node in the graph.
    pub fn init(&mut self) -> NodeId {
        self.create_node(None, false, Vec::new())
    }

    /// Initializes a node in a graph, set to a constant value.
    pub fn constant(&mut self, value: u32) -> NodeId {
        self.create_node(Some(value), false, Vec::new())
    }

    /// Adds an operation between 2 nodes in the graph, returning a new node.
    fn add_operation(&mut self, a: &NodeId, b: &NodeId, operation: Box<dyn Fn(u32, u32) -> u32>) -> NodeId {
        let node_id = self.create_node(None, false, vec![*a, *b]);
        *self.nodes[node_id].operation.borrow_mut() = Some(operation);
        node_id
    }

    /// Adds 2 nodes in the graph, returning a new node.
    pub fn add(&mut self, a: &NodeId, b: &NodeId) -> NodeId {
        self.add_operation(a, b, Box::new(|a, b| a + b))
    }

    /// Multiplies 2 nodes in the graph, returning a new node.
    pub fn mul(&mut self, a: &NodeId, b: &NodeId) -> NodeId {
        self.add_operation(a, b, Box::new(|a, b| a * b))
    }

    /// Asserts that 2 nodes are equal.
    pub fn assert_equal(&mut self, a: NodeId, b: NodeId) {
        self.constraints.push((a, b));
    }

    fn fill_node(&self, node_id: NodeId) -> bool {
        let node = &self.nodes[node_id];
        if node.value.borrow().is_none() {
            let parent_values: Vec<Option<u32>> = node
                .parents
                .iter()
                .map(|&id| *self.nodes[id].value.borrow())
                .collect();

            if parent_values.iter().all(|v| v.is_some()) {
                let parent_values: Vec<u32> = parent_values.into_iter().map(|v| v.unwrap()).collect();
                if let Some(operation) = &*node.operation.borrow() {
                    let result = match parent_values.len() {
                        2 => operation(parent_values[0], parent_values[1]),
                        1 => operation(parent_values[0], parent_values[0]),
                        _ => panic!("Unsupported number of parent values"),
                    };
                    *node.value.borrow_mut() = Some(result);
                    println!("Filling node {} with value {}", node_id, result);
                    return true;
                }
            }
        }
        false
    }

    /// Fills in all the nodes of the graph based on some inputs.
    pub fn fill_nodes(&mut self, inputs: Vec<Option<u32>>) {
        for (node_id, value) in inputs.iter().enumerate() {
            if let Some(value) = value {
                println!("Setting input node {} to value {}", node_id, value);
                *self.nodes[node_id].value.borrow_mut() = Some(*value);
            }
        }

        loop {
            let mut filled_any = false;
            for node_id in 0..self.nodes.len() {
                if self.fill_node(node_id) {
                    filled_any = true;
                }
            }
            if !filled_any {
                break;
            }
        }
    }

    /// Given a graph that has `fill_nodes` already called on it
    /// checks that all the constraints hold.
    pub fn check_constraints(&self) -> bool {
        for (a, b) in &self.constraints {
            let a_value = self.nodes[*a].value.borrow();
            let b_value = self.nodes[*b].value.borrow();
            println!(
                "Checking constraint: node {} value {} == node {} value {}",
                a,
                a_value.unwrap(),
                b,
                b_value.unwrap()
            );
            if *a_value != *b_value {
                return false;
            }
        }
        true
    }

    /// An API for hinting values that allows you to perform operations
    /// like division or computing square roots.
    pub fn hint<F>(&mut self, value_func: F, depends_on: Vec<NodeId>) -> NodeId
    where
        F: 'static + Fn(&[u32]) -> u32,
    {
        let node_id = self.create_node(None, true, depends_on.clone());
        let nodes = self.nodes.clone();
        {
            let mut operation = self.nodes[node_id].operation.borrow_mut();
            *operation = Some(Box::new(move |_, _| {
                let parent_values: Vec<u32> = depends_on
                    .iter()
                    .map(|&id| nodes[id].value.borrow().expect("Parent value should be filled"))
                    .collect();
                value_func(&parent_values)
            }));
        }
        node_id
    }
}
