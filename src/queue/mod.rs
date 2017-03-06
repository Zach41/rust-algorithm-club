pub struct Queue<T> {
    inner: Vec<Option<T>>,
    head: usize,
}

impl<T> Queue<T> {
    pub fn new() -> Queue<T> {
        Queue {
            inner: Vec::new(),
            head: 0,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.count() == 0
    }

    pub fn count(&self) -> usize {
        self.inner.len() - self.head            
    }

    pub fn enqueue(&mut self, value: T) {
        self.inner.push(Some(value))
    }

    pub fn dequeue(&mut self) -> Option<T> {
        if self.is_empty() {
            None
        } else {
            let ret = self.inner[self.head].take();
            self.head += 1;

            let percentage = self.head as f64 / self.inner.len() as f64;
            if self.inner.len() > 50 && percentage > 0.25 {
                let _ = self.inner.drain(..self.head);
                self.head = 0;
            }
            ret
        }
    }
}

impl<T: Clone> Queue<T> {
    pub fn front(&self) -> Option<T> {
        if self.is_empty() {
            None
        } else {
            self.inner[self.head].clone()
        }
    }
}

#[cfg(test)]
mod test;
