use std::ops::{Index, IndexMut};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct NodeId {
    index: usize,
}

#[derive(Clone)]
pub struct Node<T> {
    parent: Option<NodeId>,
    next_sibling: Option<NodeId>,
    previous_sibling: Option<NodeId>,
    first_child: Option<NodeId>,
    last_child: Option<NodeId>,

    pub data: T,
}

#[derive(Clone)]
pub struct Arena<T> {
    nodes: Vec<Node<T>>,
}

impl<T> Arena<T> {
    pub fn new() -> Arena<T> {
        Arena {
            nodes: Vec::new(),
        }
    }

    pub fn new_node(&mut self, data: T) -> NodeId {
        let idx = self.nodes.len();
        self.nodes.push(
            Node {
                parent: None,
                next_sibling: None,
                previous_sibling: None,
                first_child: None,
                last_child: None,

                data: data,
            }
        );
        NodeId { index: idx }
    }
}

impl<T> Index<NodeId> for Arena<T> {
    type Output = Node<T>;
    fn index(&self, index: NodeId) -> &Node<T> {
        &self.nodes[index.index]
    }
}

impl<T> IndexMut<NodeId> for Arena<T> {
    fn index_mut(&mut self, index: NodeId) -> &mut Node<T> {
        &mut self.nodes[index.index]
    }
}

impl<T> Node<T> {
    pub fn parent(&self) -> Option<NodeId> { self.parent }
    pub fn previous_sibling(&self) -> Option<NodeId> { self.previous_sibling }
    pub fn next_sibling(&self) -> Option<NodeId> { self.next_sibling }
    pub fn first_child(&self) -> Option<NodeId> { self.first_child }
    pub fn last_child(&self) -> Option<NodeId> { self.last_child }
}

impl NodeId {
    pub fn ancestors<T>(self, arena: &Arena<T>) -> Ancestors<T> {
        Ancestors {
            arena: arena,
            node: Some(self),
        }
    }

    pub fn preceding_siblings<T>(self, arena: &Arena<T>) -> PrecedingSiblings<T> {
        PrecedingSiblings {
            arena: arena,
            node: Some(self),
        }
    }

    pub fn following_siblings<T>(self, arena: &Arena<T>) -> FollowingSiblings<T> {
        FollowingSiblings {
            arena: arena,
            node: Some(self),
        }
    }

    pub fn children<T>(self, arena: &Arena<T>) -> Children<T> {
        let first_child = arena[self].first_child();
        Children {
            arena: arena,
            node: first_child,
        }
    }

    pub fn reverse_children<T>(self, arena: &Arena<T>) -> ReverseChildren<T> {
        let last_child = arena[self].last_child();
        ReverseChildren {
            arena: arena,
            node: last_child,
        }
    }

    pub fn traverse<T>(self, arena: &Arena<T>) -> Traverse<T> {
        Traverse {
            arena: arena,
            root: self,
            next: Some(NodeEdge::Start(self)),
        }
    }

    pub fn descendants<T>(self, arena: &Arena<T>) -> Descendants<T> {
        Descendants(self.traverse(arena))
    }

    pub fn detach<T>(self, arena: &mut Arena<T>) {
        let parent = arena[self].parent();
        let previous_sibling = arena[self].previous_sibling();
        let next_sibling = arena[self].next_sibling();

        if let Some(previous_sibling) = previous_sibling {
            arena[previous_sibling].next_sibling = next_sibling;
        } else if let Some(parent) = parent {
            arena[parent].first_child = next_sibling;
        }

        if let Some(next_sibling) = next_sibling {
            arena[next_sibling].previous_sibling = previous_sibling;            
        } else if let Some(parent) = parent {
            arena[parent].last_child = previous_sibling;
        }
    }

    pub fn insert_before<T>(self, arena: &mut Arena<T>, node_idx: NodeId) {
        node_idx.detach(arena);
        
        let parent = arena[self].parent();
        let previous_sibling = arena[self].previous_sibling();

        if let Some(previous_sibling) = previous_sibling {
            arena[previous_sibling].next_sibling = Some(node_idx);
            arena[node_idx].previous_sibling = Some(previous_sibling);
        } else if let Some(parent) = parent {
            arena[parent].first_child = Some(node_idx);
        }

        arena[node_idx].next_sibling = Some(self);
        arena[self].previous_sibling = Some(node_idx);
        arena[node_idx].parent = parent;
    }

    pub fn insert_after<T>(self, arena: &mut Arena<T>, node_idx: NodeId) {
        node_idx.detach(arena);
        
        let parent = arena[self].parent();
        let next_sibling = arena[self].next_sibling();

        arena[node_idx].parent = parent;
        arena[node_idx].previous_sibling = Some(self);
        arena[self].next_sibling = Some(node_idx);

        if let Some(next_sibling) = next_sibling {
            arena[node_idx].next_sibling = Some(next_sibling);
            arena[next_sibling].previous_sibling = Some(node_idx);
        } else if let Some(parent) = parent {
            arena[parent].last_child = Some(node_idx);
        }
    }

    pub fn append<T>(self, arena: &mut Arena<T>, node_idx: NodeId) {
        node_idx.detach(arena);
        arena[node_idx].parent = Some(self);

        if let Some(last_child) = arena[self].last_child() {
            arena[last_child].next_sibling = Some(node_idx);
            arena[node_idx].previous_sibling = Some(last_child);
        } else {
            arena[self].first_child = Some(node_idx);
        }
        arena[self].last_child = Some(node_idx);
    }

    pub fn prepend<T>(self, arena: &mut Arena<T>, node_idx: NodeId) {
        node_idx.detach(arena);
        arena[node_idx].parent = Some(self);

        if let Some(first_child) = arena[self].first_child() {
            arena[first_child].previous_sibling = Some(node_idx);
            arena[node_idx].next_sibling = Some(first_child);
        } else {
            arena[self].last_child = Some(node_idx);
        }
        arena[self].first_child = Some(node_idx);
    }

    pub fn find_node<T: Ord>(self, arena: &Arena<T>, data: T) -> Option<NodeId> {
        let descendants = self.descendants(arena);
        for node in descendants {
            if arena[node].data == data {
                return Some(node)
            }
        }
        None
    }
}

pub struct Ancestors<'a, T: 'a> {
    arena: &'a Arena<T>,
    node: Option<NodeId>,
}

pub struct PrecedingSiblings<'a, T: 'a> {
    arena: &'a Arena<T>,
    node: Option<NodeId>,
}

pub struct FollowingSiblings<'a, T: 'a> {
    arena: &'a Arena<T>,
    node: Option<NodeId>,
}

pub struct Children<'a, T: 'a> {
    arena: &'a Arena<T>,
    node: Option<NodeId>,
}

pub struct ReverseChildren<'a, T: 'a> {
    arena: &'a Arena<T>,
    node: Option<NodeId>,
}

macro_rules! impl_node_iterator {
    ($name: ident: $next: ident) => {
        impl<'a, T> Iterator for $name<'a, T> {
            type Item = NodeId;

            fn next(&mut self) -> Option<NodeId> {
                if let Some(idx) = self.node.take() {
                    self.node = self.arena[idx].$next();
                    Some(idx)
                } else {
                    None
                }
            }
        }
    }
}

impl_node_iterator! {
    Ancestors: parent
}

impl_node_iterator! {
    PrecedingSiblings: previous_sibling
}

impl_node_iterator! {
    FollowingSiblings: next_sibling
}

impl_node_iterator! {
    Children: next_sibling
}

impl_node_iterator! {
    ReverseChildren: previous_sibling
}

pub struct Traverse<'a, T: 'a> {
    arena: &'a Arena<T>,
    root: NodeId,
    next: Option<NodeEdge<NodeId>>,
}

#[derive(Clone, Debug)]
pub enum NodeEdge<T> {
    Start(T),
    End(T),
}

impl<'a, T> Iterator for Traverse<'a, T> {
    type Item = NodeEdge<NodeId>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(edge) = self.next.take() {
            self.next = match edge {
                NodeEdge::Start(node) => {
                    if let Some(child) = self.arena[node].first_child() {
                        Some(NodeEdge::Start(child))
                    } else {
                        Some(NodeEdge::End(node))
                    }
                },
                NodeEdge::End(node) => {
                    if node == self.root {
                        None
                    } else if let Some(sibling) = self.arena[node].next_sibling() {
                            Some(NodeEdge::Start(sibling))
                        } else if let Some(parent) = self.arena[node].parent() {
                            Some(NodeEdge::End(parent))
                        } else {
                            None
                        }
                }
            };
            Some(edge)
        } else {
            None
        }
    }
}

pub struct Descendants<'a, T: 'a>(Traverse<'a, T>);

impl<'a, T> Iterator for Descendants<'a, T> {
    type Item = NodeId;

    fn next(&mut self) -> Option<NodeId> {
        loop {
            match self.0.next() {
                Some(NodeEdge::Start(node)) => return Some(node),
                Some(NodeEdge::End(_)) => {},
                None => return None
            }
        }
    }
}

#[cfg(test)]
mod test;

