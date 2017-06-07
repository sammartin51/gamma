// -------------------------CHILD_ITER
pub struct NodeIter<'a, T:'a> {
    arena: &'a Arena<T>,
    cur_node: Option<NodeId>,
}

impl<'a, T:'a> Iterator for NodeIter<'a, T> {
    fn next(&self) -> Option<NodeId> {
        match self.cur_node {
            Some(x) => self.arena.nodes[x.index].next_sibling,
            _ => None,
        }
    }
}

// --------------------------NODE
pub struct Node<T> {
    parent: Option<NodeId>,
    previous_sibling: Option<NodeId>,
    next_sibling: Option<NodeId>,
    first_child: Option<NodeId>,
    last_child: Option<NodeId>,

    pub data: T,
}


// ------------------------NODE ID
#[derive(Debug, Clone, Copy)]
pub struct NodeId {
    pub index: usize,
}

// function to create an optional nodeid from a number
impl NodeId {
    pub fn opt(index : usize) -> Option<NodeId>{
        Some(NodeId { index: index })
    }
}


// ------------------------ARENA
pub struct Arena<T> {
    nodes: Vec<Node<T>>,
}

impl <T> Arena<T> {
    pub fn new(root : T) -> Arena<T> {
        Arena {
            nodes : vec![
                Node {
                    parent: None,
                    previous_sibling: None,
                    next_sibling: None,
                    first_child: None,
                    last_child: None,

                    data: root
                } ],
        }
    }
    
    pub fn get_node(&mut self, id: Option<NodeId>) -> Option<&mut Node<T>> {
        match id {
            Some(x) => Some(&mut self.nodes[x.index]),
            None => None
        }
    }

    pub fn add_node(&mut self,
                    data : T,
                    parent_id : Option<NodeId>) -> Option<NodeId> {

        // Process the parent
        let parent_id = match parent_id {
            // default to the root
            None => 0,
            Some(x) => x.index
        };
        
        // Get the next free index
        let child_id = self.nodes.len();

        // Push the node into the arena
        self.nodes.push(
            Node {
                parent: None,
                first_child: None,
                last_child: None,
                previous_sibling: None,
                next_sibling: None,
                data: data,
            });

        // the child's previous sibling
        self.nodes[child_id].previous_sibling
            // will depend on whether the last child exists
            = match self.nodes[parent_id].last_child {
                // no last child, no link needs made
                None => None,
                // if there is a last child...
                Some(x) =>  {
                    // set it's next sibling to our new node
                    // while we're in here
                    self.nodes[x.index].next_sibling =
                        NodeId::opt(child_id);
                    // the last child is the previous sibling
                    Some(x)
                }
            };

        // The first child of the parent
        self.nodes[parent_id].first_child
            // will depend on whether the first child already exists
            = match self.nodes[parent_id].first_child {
                // if not, set that thing
                None => NodeId::opt(child_id),
                // if there is, don't touch it
                Some(x) => Some (x)
            };
        // new child has to be the last in any case
        self.nodes[parent_id].last_child = NodeId::opt(child_id);
        
        // set a reference to the node we just added
        NodeId::opt(child_id)
    }

    fn children(&self, cur_node: NodeId) -> NodeIter<T> {
        NodeIter {
            arena: &self,
            cur_node: self.nodes[cur_node.index].first_child
        }
    }
    // Need a function to return an iterator of mutable references to children
    
 
}


// to implement list
/*

- a way to iterate over children of a node
- - need some sort of reference to the arena
- - 

add node to area
add this node's indext to parent
reference back to parent

}

- implement get children
- implement get parent

 */

