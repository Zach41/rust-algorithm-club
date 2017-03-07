use super::heap::Heap;

pub struct PriorityQueue<T> {
    heap: Heap<T>,
}

impl<T: PartialOrd> PriorityQueue<T> {
    pub fn is_empty(&self) -> bool {
        self.heap.is_empty()
    }

    pub fn count(&self) -> usize {
        self.heap.count()
    }

    pub fn enqueue(&mut self, value: T) {
        self.heap.insert(value);
    }

    pub fn dequeue(&mut self) -> Option<T> {
        self.heap.remove()
    }

    pub fn new() -> PriorityQueue<T> {
        PriorityQueue {
            heap: Heap::new(),
        }
    }
}

impl<T: PartialOrd + Clone> PriorityQueue<T> {
    pub fn peek(&self) -> Option<T> {
        self.heap.peek()
    }
}

#[cfg(test)]
mod test;
