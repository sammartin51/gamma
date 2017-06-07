#[derive(Debug)]
pub struct Node<T> {
    pub data: T,
    children: Vec<Node<T>>,
}

impl<T> Node<T> {
    pub fn new(data: T) -> Node<T> {
        Node { data: data, children: vec![] }
    }

    pub fn add_child(&mut self, child: Node<T>) {
        self.children.push(child);
    }

    pub fn zipper(self) -> NodeZipper<T> {
        NodeZipper { node: self, parent: None, index_in_parent: 0 }
    }
}

#[derive(Debug)]
pub struct NodeZipper<T> {
    pub node: Node<T>,
    parent: Option<Box<NodeZipper<T>>>,
    index_in_parent: usize,
}

impl<T> NodeZipper<T> {
    pub fn child(mut self, index: usize) -> NodeZipper<T> {
        // remove specified child for safe keeping
        let child = self.node.children.swap_remove(index);

        // return a new node_zipper
        NodeZipper {
            node: child,
            parent: Some(Box::new(self)),
            index_in_parent: index,
        }
    }

    pub fn parent(self) -> NodeZipper<T> {
        // destructure this nodeziper
        let NodeZipper { node, parent, index_in_parent } = self;

        // destructure the parent
        // maybe we don't like unwrap here? calling parent
        // on the root may be bad
        let NodeZipper {
            node: mut parent_node,
            parent:parent_parent,
            index_in_parent: parent_index_in_parent,
        } = *parent.unwrap();

        // insert the node of this zipper back into
        // the parent
        // make sure it goes back in the right place
        parent_node.children.push(node);
        let len = parent_node.children.len();
        parent_node.children.swap(index_in_parent, len - 1);

        // return new nodezipper focuse on the parent
        NodeZipper {
            node: parent_node,
            parent: parent_parent,
            index_in_parent: parent_index_in_parent,
        }
    }
}
           
