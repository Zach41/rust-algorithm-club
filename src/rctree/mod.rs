use std::cell::{self, RefCell};
use std::rc::{Rc, Weak};
use std::ops::{Deref, DerefMut};
use std::fmt;

pub struct NodeRef<T>(Rc<RefCell<Node<T>>>);

#[derive(Debug)]
struct Node<T> {
    parent: WeakLink<T>,
    first_child: Link<T>,
    last_child: WeakLink<T>,
    next_sibling: Link<T>,
    previous_sibling: WeakLink<T>,
    data: T,        
}

impl<T> Node<T> {
    fn detach(&mut self) {
        let parent_weak = self.parent.take();
        let previous_sibling_weak = self.previous_sibling.take();
        let next_sibling_strong = self.next_sibling.take();

        let previous_sibling_opt = previous_sibling_weak.as_ref().and_then(|weak| weak.upgrade());
        
        if let Some(next_sibling_ref) = next_sibling_strong.as_ref() {
            let mut next_sibling_borrow_mut = next_sibling_ref.borrow_mut();
            next_sibling_borrow_mut.previous_sibling = previous_sibling_weak;
        } else if let Some(parent_ref) = parent_weak.as_ref() {
            if let Some(parent_strong) = parent_ref.upgrade() {
                let mut parent_strong_borrow_mut = parent_strong.borrow_mut();
                parent_strong_borrow_mut.last_child = previous_sibling_weak;
            }
        }

        if let Some(previous_sibling_strong) = previous_sibling_opt {
            let mut previous_sibling_strong_borrow_mut = previous_sibling_strong.borrow_mut();
            previous_sibling_strong_borrow_mut.next_sibling = next_sibling_strong;
        } else if let Some(parent_ref) = parent_weak.as_ref() {
            if let Some(parent_strong) = parent_ref.upgrade() {
                let mut parent_strong_mut = parent_strong.borrow_mut();
                parent_strong_mut.first_child = next_sibling_strong;
            }
        }
    }
}

type Link<T> = Option<Rc<RefCell<Node<T>>>>;
type WeakLink<T> = Option<Weak<RefCell<Node<T>>>>;


impl<T> Clone for NodeRef<T> {
    fn clone(&self) -> NodeRef<T> {
        NodeRef(self.0.clone())
    }
}

impl<T> fmt::Debug for NodeRef<T>
    where T: fmt::Debug {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(&*self.0.borrow(), f)
    }
}

fn same_rc<T>(a: &Rc<T>, b: &Rc<T>) -> bool {
    let a: *const T = &**a;
    let b: *const T = &**b;
    a == b
}

macro_rules! try_opt {
    ($expr: expr) => {
        match $expr {
            Some(value) => value,
            None => return None,
        }
    }
}

impl<T> NodeRef<T> {
    fn new(data: T) -> NodeRef<T> {
        NodeRef (Rc::new(RefCell::new(Node {
            parent: None,
            first_child: None,
            last_child: None,
            next_sibling: None,
            previous_sibling: None,
            data: data,
        })))
    }

    pub fn parent(&self) -> Option<NodeRef<T>> {
        Some(NodeRef(try_opt!(try_opt!(self.0.borrow().parent.as_ref()).upgrade())))
    }

    pub fn first_child(&self) -> Option<NodeRef<T>> {
        Some(NodeRef(try_opt!(self.0.borrow().first_child.as_ref()).clone()))
    }

    pub fn last_child(&self) -> Option<NodeRef<T>> {
        Some(NodeRef(try_opt!(try_opt!(self.0.borrow().last_child.as_ref()).upgrade())))
    }

    pub fn next_sibling(&self) -> Option<NodeRef<T>> {
        Some(NodeRef(try_opt!(self.0.borrow().next_sibling.as_ref()).clone()))
    }

    pub fn previous_sibling(&self) -> Option<NodeRef<T>> {
        Some(NodeRef(try_opt!(try_opt!(self.0.borrow().previous_sibling.as_ref()).upgrade())))
    }

    /// Return a shared reference to this node's data
    pub fn borrow(&self) -> Ref<T> {
        Ref {
            _ref: self.0.borrow()
        }
    }
    
    /// Return a mutable reference to this node's data
    pub fn borrow_mut(&mut self) -> RefMut<T> {
        RefMut {
            _ref: self.0.borrow_mut()
        }
    }

    pub fn same_node(&self, other: &NodeRef<T>) -> bool {
        same_rc(&self.0, &other.0)
    }

    /// Return an iterator of references to this node and its ancestors
    pub fn ancestors(&self) -> Ancestors<T> {
        Ancestors(Some(self.clone()))
    }

    /// Return an iterator of references to this node and its preceding siblings
    pub fn preceding_siblings(&self) -> PrecedingSiblings<T> {
        PrecedingSiblings(Some(self.clone()))
    }

    /// Return an iterator of references to this node and its next siblings
    pub fn following_siblings(&self) -> FollowingSiblings<T> {
        FollowingSiblings(Some(self.clone()))
    }

    /// Return an iterator of reference to this node's children
    pub fn children(&self) -> Children<T> {
        Children(self.first_child())
    }

    /// Return an iterator of references to this node's children, in reverse order
    pub fn reverse_children(&self) -> ReverseChildren<T> {
        ReverseChildren(self.last_child())
    }

    /// Return an iterator of references to this node and its descendants, in preorder traverse order
    pub fn descendants(&self) -> Descendants<T> {
        Descendants(self.traverse())
    }

    pub fn traverse(&self) -> Traverse<T> {
        Traverse {
            root: self.clone(),
            next: Some(NodeEdge::Start(self.clone())),
        }
    }

    /// Detach a node from its parent and siblings
    pub fn detach(&self) {
        self.0.borrow_mut().detach()
    }

    /// Append a new child to this node, after existing children
    pub fn append(&self, new_child: NodeRef<T>) {
        let mut self_borrow_mut = self.0.borrow_mut();
        let mut last_child_opt = None;

        {
            let mut new_child_borrow_mut = new_child.0.borrow_mut();
            new_child_borrow_mut.detach();
            new_child_borrow_mut.parent = Some(Rc::downgrade(&self.0));
            if let Some(last_child_weak) = self_borrow_mut.last_child.take() {
                if let Some(last_child_strong) = last_child_weak.upgrade() {
                    new_child_borrow_mut.previous_sibling = Some(last_child_weak);
                    last_child_opt = last_child_strong.into();
                }
            }
            self_borrow_mut.last_child = Some(Rc::downgrade(&new_child.0));
        }

        if let Some(last_child_strong) = last_child_opt {
            let mut last_child_borrow_mut = last_child_strong.borrow_mut();
            last_child_borrow_mut.next_sibling = Some(new_child.0);
        } else {
            debug_assert!(self_borrow_mut.first_child.is_none());
            self_borrow_mut.first_child = Some(new_child.0);
        }        
    }

    /// Prepend a new child to this node, before existing children
    pub fn prepend(&self, new_child: NodeRef<T>) {
        let mut self_borrow_mut = self.0.borrow_mut();
        {
            let mut new_child_borrow_mut = new_child.0.borrow_mut();
            new_child_borrow_mut.detach();
            new_child_borrow_mut.parent = Some(Rc::downgrade(&self.0));
            match self_borrow_mut.first_child.take() {
                Some(first_child_strong) => {
                    {
                        let mut first_child_borrow_mut = first_child_strong.borrow_mut();
                        debug_assert!(first_child_borrow_mut.previous_sibling.is_none());
                        first_child_borrow_mut.previous_sibling = Some(Rc::downgrade(&new_child.0));
                    }
                    new_child_borrow_mut.next_sibling = Some(first_child_strong);
                },
                None => {
                    debug_assert!(self_borrow_mut.first_child.is_none());
                    self_borrow_mut.last_child = Some(Rc::downgrade(&new_child.0));
                }            
            }
        }
        self_borrow_mut.first_child = Some(new_child.0);
    }

    /// Insert a new sibling after this node
    pub fn insert_after(&self, new_sibling: NodeRef<T>) {
        let mut self_borrow_mut = self.0.borrow_mut();
        {
            let mut new_sibling_borrow_mut = new_sibling.0.borrow_mut();
            new_sibling_borrow_mut.detach();
            new_sibling_borrow_mut.parent = self_borrow_mut.parent.clone();
            new_sibling_borrow_mut.previous_sibling = Some(Rc::downgrade(&self.0));
            match self_borrow_mut.next_sibling.take() {
                Some(next_sibling_strong) => {
                    {
                        let mut next_sibling_borrow_mut = next_sibling_strong.borrow_mut();
                        next_sibling_borrow_mut.previous_sibling = Some(Rc::downgrade(&new_sibling.0));
                    }
                    new_sibling_borrow_mut.next_sibling = Some(next_sibling_strong);
                },
                None => {
                    if let Some(parent_ref) = self_borrow_mut.parent.as_ref() {
                        if let Some(parent_strong) = parent_ref.upgrade() {
                            let mut parent_borrow_mut = parent_strong.borrow_mut();
                            parent_borrow_mut.last_child = Some(Rc::downgrade(&new_sibling.0));
                        }
                    }
                }
            }
        }
        self_borrow_mut.next_sibling = Some(new_sibling.0);
    }

    /// Insert a new sibling before this node
    pub fn insert_before(&self, new_sibling: NodeRef<T>) {
        let mut self_borrow_mut = self.0.borrow_mut();
        let mut previous_child_opt = None;
        {
            let mut new_sibling_borrow_mut = new_sibling.0.borrow_mut();
            new_sibling_borrow_mut.detach();
            new_sibling_borrow_mut.parent = self_borrow_mut.parent.clone();
            new_sibling_borrow_mut.next_sibling = Some(self.0.clone());
            if let Some(previous_weak) = self_borrow_mut.previous_sibling.take() {
                if let Some(previous_strong) = previous_weak.upgrade() {
                    new_sibling_borrow_mut.previous_sibling = Some(previous_weak);
                    previous_child_opt = Some(previous_strong);
                }
            }
            self_borrow_mut.previous_sibling = Some(Rc::downgrade(&new_sibling.0));            
        }
        if let Some(previous_strong) = previous_child_opt {
            let mut previous_borrow_mut = previous_strong.borrow_mut();
            previous_borrow_mut.next_sibling = Some(new_sibling.0);
        } else {
            if let Some(parent_ref) = self_borrow_mut.parent.as_ref() {
                if let Some(parent_strong) = parent_ref.upgrade() {
                    let mut parent_borrow_mut = parent_strong.borrow_mut();
                    parent_borrow_mut.first_child = Some(new_sibling.0);
                }
            }
        }
    }
}

impl<T: Ord> NodeRef<T> {
    /// Find node with value, we take the first node occurred, if not found, return None
    pub fn find_node(&self, value: &T) -> Option<NodeRef<T>> {
        use std::cmp::Ordering;
        let mut traverse_nodes = self.traverse();
        while let Some(edge) = traverse_nodes.next() {
            match edge {
                NodeEdge::Start(node) => {
                    let borrowed_value = &*node.borrow();
                    match borrowed_value.cmp(value) {
                        Ordering::Equal => { return Some(node.clone()); }
                        _ => {}
                    }
                },
                _ => {}
            }
        }
        None
    }
}

pub struct Ref<'a, T: 'a> {
    _ref: cell::Ref<'a, Node<T>>,
}

pub struct RefMut<'a, T: 'a> {
    _ref: cell::RefMut<'a, Node<T>>,
}

impl<'a, T> Deref for Ref<'a, T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self._ref.data
    }
}

impl<'a, T> Deref for RefMut<'a, T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self._ref.data
    }
}

impl<'a, T> DerefMut for RefMut<'a, T> {
    fn deref_mut(&mut self) -> &mut T {
        &mut self._ref.data
    }
}


macro_rules! impl_node_iterator {
    ($name: ident, $next: expr) => {
        impl<T> Iterator for $name<T> {
            type Item = NodeRef<T>;

            fn next(&mut self) -> Option<NodeRef<T>> {
                match self.0.take() {
                    Some(node) => {
                        self.0 = $next(&node);
                        Some(node)
                    },
                    None => None
                }
            }
        }
    }
}

pub struct Ancestors<T> (Option<NodeRef<T>>);
impl_node_iterator!(Ancestors, |node: &NodeRef<T>| node.parent());

pub struct PrecedingSiblings<T>(Option<NodeRef<T>>);
impl_node_iterator!(PrecedingSiblings, |node: &NodeRef<T>| node.previous_sibling());

pub struct FollowingSiblings<T>(Option<NodeRef<T>>);
impl_node_iterator!(FollowingSiblings, |node: &NodeRef<T>| node.next_sibling());

pub struct Children<T>(Option<NodeRef<T>>);
impl_node_iterator!(Children, |node: &NodeRef<T>| node.next_sibling());

pub struct ReverseChildren<T>(Option<NodeRef<T>>);
impl_node_iterator!(ReverseChildren, |node: &NodeRef<T>| node.previous_sibling());

pub struct Descendants<T>(Traverse<T>);

impl<T> Iterator for Descendants<T> {
    type Item = NodeRef<T>;

    fn next(&mut self) -> Option<NodeRef<T>> {
        loop {
            match self.0.next() {
                Some(NodeEdge::Start(node)) => return Some(node),
                Some(NodeEdge::End(_)) => (),
                None => return None
            }
        }
    }
}

#[derive(Debug, Clone)]
pub enum NodeEdge<T> {
    Start(NodeRef<T>),
    End(NodeRef<T>),
}

pub struct Traverse<T> {
    root: NodeRef<T>,
    next: Option<NodeEdge<T>>,
}

impl<T> Iterator for Traverse<T> {
    type Item = NodeEdge<T>;

    fn next(&mut self) -> Option<NodeEdge<T>> {
        match self.next.take() {
            Some(item) => {
                self.next = match item {
                    NodeEdge::Start(ref node) => {
                        match node.first_child() {
                            Some(child) => Some(NodeEdge::Start(child)),
                            None => Some(NodeEdge::End(node.clone())),
                        }
                    },
                    NodeEdge::End(ref node) => {
                        if node.same_node(&self.root) {
                            None
                        } else {
                            match node.next_sibling() {
                                Some(next_sibling) => Some(NodeEdge::Start(next_sibling)),
                                None => {
                                    match node.parent() {
                                        Some(parent) => Some(NodeEdge::End(parent)),
                                        None => None,
                                    }
                                }
                            }
                        }
                    }
                };
                Some(item)
            },
            None => None
        }
    }
}

#[cfg(test)]
mod test;
