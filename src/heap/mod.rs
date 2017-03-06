pub struct Heap<T: PartialOrd> {
    inner: Vec<T>,
}

pub fn parent_index(idx: usize) -> usize {
    assert!(idx > 0);
    (idx - 1) / 2
}

pub fn lchild_index(idx: usize) -> usize {
    2 * idx + 1
}

pub fn rchild_index(idx: usize) -> usize {
    2 * idx + 2
}

impl<T: PartialOrd> Heap<T> {
    pub fn count(&self) -> usize {
        self.inner.len()
    }

    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    fn shift_down(&mut self, idx: usize, heapsz: usize) {
        let mut parent_idx = idx;
        loop {
            let lchild_idx = lchild_index(parent_idx);
            let rchild_idx = rchild_index(parent_idx);

            let mut first = parent_idx;
            if lchild_idx < heapsz && self.inner[lchild_idx] > self.inner[first] {
                first = lchild_idx;
            }
            if rchild_idx < heapsz && self.inner[rchild_idx] > self.inner[first] {
                first = rchild_idx;
            }
            if first == parent_idx {
                break;
            }
            self.inner.swap(first, parent_idx);
            parent_idx = first;
        }
    }

    fn shift_up(&mut self, idx: usize) {
        let mut child_idx = idx;
        while child_idx > 0 {
            let parent_idx = parent_index(child_idx);
            if self.inner[child_idx] > self.inner[parent_idx] {
                self.inner.swap(child_idx, parent_idx);
                child_idx = parent_idx;
            } else {
                break;
            }
        }
    }

    pub fn insert(&mut self, value: T) {
        self.inner.push(value);
        let len = self.inner.len();
        self.shift_up(len - 1);
    }

    pub fn remove(&mut self) -> Option<T> {
        if self.inner.is_empty() {
            None
        } else {
            let len = self.inner.len();
            self.inner.swap(0, len - 1);
            let ret = self.inner.pop();
            self.shift_down(0, len - 1);
            ret
        }
    }

    pub fn new() -> Heap<T> {
        Heap { inner: Vec::new() }
    }

    fn build_heap(&mut self) {
        let len = self.inner.len();
        for i in 0..self.inner.len() / 2 {
            let idx = self.inner.len() / 2 - 1 - i;
            self.shift_down(idx, len);
        }
    }
}

impl<T: Clone + PartialOrd> Heap<T> {
    pub fn peek(&self) -> Option<T> {
        if self.inner.is_empty() {
            None
        } else {
            Some(self.inner[0].clone())
        }
    }

    pub fn with_array(array: &[T]) -> Heap<T> {
        let mut inner = Vec::new();
        inner.extend_from_slice(array);
        let mut heap = Heap { inner: inner };
        heap.build_heap();
        heap
    }
}

#[cfg(test)]
mod test;
