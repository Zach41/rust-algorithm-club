use std::cell::Cell;
use std::cmp::Ordering;

pub struct ArenaNode<'a, T: 'a> {
    parent: Cell<Option<&'a ArenaNode<'a, T>>>,
    first_child: Cell<Option<&'a ArenaNode<'a, T>>>,
    last_child: Cell<Option<&'a ArenaNode<'a, T>>>,
    previous_sibling: Cell<Option<&'a ArenaNode<'a, T>>>,
    next_sibling: Cell<Option<&'a ArenaNode<'a, T>>>,
    pub data: T,
}

fn same_ref<T>(a: &T, b: &T) -> bool {
    a as *const T == b as *const T
}

impl<'a, T> ArenaNode<'a, T> {
    pub fn new(data: T) -> ArenaNode<'a, T> {
        ArenaNode {
            parent: Cell::new(None),
            first_child: Cell::new(None),
            last_child: Cell::new(None),
            previous_sibling: Cell::new(None),
            next_sibling: Cell::new(None),
            data: data,
        }
    }

    pub fn parent(&self) -> Option<&ArenaNode<'a, T>> {
        self.parent.get()
    }

    pub fn first_child(&self) -> Option<&ArenaNode<'a, T>> {
        self.first_child.get()
    }

    pub fn last_child(&self) -> Option<&ArenaNode<'a, T>> {
        self.last_child.get()
    }

    pub fn previous_sibling(&self) -> Option<&ArenaNode<'a, T>> {
        self.previous_sibling.get()
    }

    pub fn next_sibling(&self) -> Option<&ArenaNode<'a, T>> {
        self.next_sibling.get()
    }

    pub fn same_node(&self, other: &ArenaNode<'a, T>) -> bool {
        same_ref(self, other)
    }

    pub fn ancestors(&'a self) -> Ancestors<'a, T> {
        Ancestors(Some(self))
    }

    pub fn preceding_siblings(&'a self) -> PrecedingSiblings<'a, T> {
        PrecedingSiblings(Some(self))
    }

    pub fn following_siblings(&'a self) -> FollowingSiblings<'a, T> {
        FollowingSiblings(Some(self))
    }

    pub fn children(&'a self) -> Children<'a, T> {
        Children(self.first_child())
    }

    pub fn reverse_children(&'a self) -> ReverseChildren<'a, T> {
        ReverseChildren(self.last_child())
    }

    pub fn traverse(&'a self) -> Traverse<'a, T> {
        Traverse {
            root: self,
            next: Some(NodeEdge::Start(self)),
        }
    }

    pub fn descendants(&'a self) -> Descendants<'a, T> {
        Descendants(self.traverse())
    }

    pub fn detach(&self) {
        let parent = self.parent.take();
        let next_sibling = self.next_sibling.take();
        let previous_sibling = self.previous_sibling.take();
        
        if let Some(next_sibling) = next_sibling {
            next_sibling.previous_sibling.set(previous_sibling);
        } else if let Some(parent) = parent {
            parent.last_child.set(previous_sibling);
        }

        if let Some(previous_sibling) = previous_sibling {
            previous_sibling.next_sibling.set(next_sibling);
        } else if let Some(parent) = parent {
            parent.first_child.set(next_sibling);
        }
    }

    pub fn append(&'a self, new_child: &'a ArenaNode<'a, T>) {
        new_child.detach();
        new_child.parent.set(Some(self));
        if let Some(last_child) = self.last_child() {
            new_child.previous_sibling.set(Some(last_child));
            last_child.next_sibling.set(Some(new_child));
        } else {
            self.first_child.set(Some(new_child));
        }
        self.last_child.set(Some(new_child));
    }

    pub fn prepend(&'a self, new_child: &'a ArenaNode<'a, T>) {
        new_child.detach();
        new_child.parent.set(Some(self));
        if let Some(first_child) = self.first_child() {
            first_child.previous_sibling.set(Some(new_child));
            new_child.next_sibling.set(Some(first_child));
        } else {
            self.last_child.set(Some(new_child));
        }
        self.first_child.set(Some(new_child));
    }

    pub fn insert_before(&'a self, new_sibling: &'a ArenaNode<'a, T>) {
        new_sibling.detach();
        let parent_opt = self.parent();
        let previous_sibling_opt = self.previous_sibling();

        if let Some(previous_sibling) = previous_sibling_opt {
            previous_sibling.next_sibling.set(Some(new_sibling));
            new_sibling.previous_sibling.set(Some(previous_sibling))
        } else if let Some(parent) = parent_opt {
            parent.first_child.set(Some(new_sibling));
        }
        self.previous_sibling.set(Some(new_sibling));
    }

    pub fn insert_after(&'a self, new_sibling: &'a ArenaNode<'a, T>) {
        new_sibling.detach();
        let parent = self.parent();
        let next_sibling = self.next_sibling();

        if let Some(next_sibling) = next_sibling {
            next_sibling.previous_sibling.set(Some(new_sibling));
            new_sibling.next_sibling.set(Some(next_sibling));
        } else if let Some(parent) = parent {
            parent.last_child.set(Some(new_sibling));
        }
        self.next_sibling.set(Some(new_sibling));
    }
}

impl<'a, T> ArenaNode<'a, T>
    where T: Ord {
    pub fn find_node(&'a self, value: &T) -> Option<&'a ArenaNode<'a, T>> {
        let traverse = self.traverse();
        for edge in traverse {
            if let NodeEdge::Start(node) = edge {
                if let Ordering::Equal = node.data.cmp(value) {
                    return Some(node);
                }
            }
        }
        None
    }
}


macro_rules! relationship_iterator {
    (#[$attr: meta] $name: ident: $next: ident) => {
        #[$attr]
        pub struct $name<'a, T: 'a>(Option<&'a ArenaNode<'a, T>>);

        impl<'a, T> Iterator for $name<'a, T> {
            type Item = &'a ArenaNode<'a, T>;

            fn next(&mut self) -> Option<Self::Item> {
                if let Some(node) = self.0.take() {
                    self.0 = node.$next();
                    Some(node)
                } else {
                    None
                }
            }
        }
    }
}

relationship_iterator! {
    #[doc = "An iterator of references to this node and its ancestors"]
    Ancestors: parent
}

relationship_iterator! {
    #[doc = "An iterator of references to this node and its preceding siblings"]
    PrecedingSiblings: previous_sibling
}

relationship_iterator! {
    #[doc = "An iterator of references to this node and its next siblings"]
    FollowingSiblings: next_sibling
}

relationship_iterator! {
    #[doc = "An iterator of references to this node's children"]
    Children: next_sibling
}

relationship_iterator! {
    #[doc = "An iterator of references to this node's children, in reverse order"]
    ReverseChildren: previous_sibling
}

pub enum NodeEdge<T> {
    Start(T),
    End(T),
}

pub struct Traverse<'a, T: 'a> {
    root: &'a ArenaNode<'a, T>,
    next: Option<NodeEdge<&'a ArenaNode<'a, T>>>,
}

impl<'a, T> Iterator for Traverse<'a, T> {
    type Item = NodeEdge<&'a ArenaNode<'a, T>>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(edge) = self.next.take() {
            self.next = match edge {
                NodeEdge::Start(node) => {
                    match node.first_child() {
                        Some(child) => NodeEdge::Start(child).into(),
                        None => NodeEdge::End(node).into(),
                    }
                },
                NodeEdge::End(node) => {
                    if node.same_node(self.root) {
                        None
                    } else {
                        match node.next_sibling() {
                            Some(sibling) => NodeEdge::Start(sibling).into(),
                            None => {
                                if let Some(parent) = node.parent() {
                                    NodeEdge::End(parent).into()
                                } else {
                                    None
                                }
                            }
                        }
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
    type Item = &'a ArenaNode<'a, T>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(edge) = self.0.next() {
                if let NodeEdge::Start(node) = edge {
                    return Some(node);
                }
            } else {
                return None;
            }
        }
    }
}
