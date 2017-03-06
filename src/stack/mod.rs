#[derive(Debug)]
pub struct Stack<T> {
    inner: Vec<T>
}

impl<T> Stack<T> {
    pub fn new() -> Stack<T> {
        Stack {
            inner: Vec::new()
        }
    }
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    pub fn len(&self) -> usize {
        self.inner.len()
    }

    pub fn push(&mut self, value: T) {
        self.inner.push(value)
    }

    pub fn pop(&mut self) -> Option<T> {
        self.inner.pop()
    }

    pub fn top(&self) -> Option<&T> {
        if self.inner.is_empty() {
            None
        } else {
            Some(&self.inner[self.inner.len() - 1])
        }
    }
}

impl<T> Iterator for Stack<T> {
    type Item = T;
    fn next(&mut self) -> Option<T> {
        self.inner.pop()
    }
}

#[cfg(test)]
mod test;
