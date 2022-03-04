/// A queue which keeps a history of dequeued items
#[derive(Debug, Clone, Default)]
pub struct Queue<T> {
    items: Vec<T>,
    front: usize,
}

impl<T> Queue<T> {
    /// Creates an empty queue
    pub fn new() -> Self {
        Queue {
            items: Vec::new(),
            front: 0,
        }
    }

    /// Gets the item at the front of the queue
    pub fn front(&self) -> Option<&T> {
        self.get(self.front)
    }

    /// Checks if an item at a given index has been dequeued
    pub fn was_dequeued(&self, ndx: usize) -> bool {
        ndx < self.front
    }

    /// Enqueues an item to the back of the queue
    pub fn enqueue(&mut self, item: T) -> usize {
        self.items.push(item);
        self.items.len() - 1
    }

    /// Dequeues an item from the front of the queue
    pub fn dequeue_mut(&mut self) -> Option<&mut T> {
        let item = self.items.get_mut(self.front);
        if item.is_some() {
            self.front += 1;
        }
        item
    }

    /// Gets an item at the specified index in the queue
    pub fn get(&self, ndx: usize) -> Option<&T> {
        self.items.get(ndx)
    }

    /// Gets an item at the specified index in the queue with mutable access
    pub fn get_mut(&mut self, ndx: usize) -> Option<&mut T> {
        self.items.get_mut(ndx)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn enqueue() {
        let mut queue = Queue::<i32>::new();
        let id = queue.enqueue(1);
        assert_eq!(0, id);
    }

    #[test]
    fn dequeue() {
        let mut queue = Queue::<i32>::new();
        let _id = queue.enqueue(1);
        let val = queue.dequeue_mut();
        assert!(val.is_some());
        let val = *val.unwrap();
        assert_eq!(1, val);
        assert!(queue.dequeue_mut().is_none());
    }

    #[test]
    fn get() {
        let mut queue = Queue::<i32>::new();
        let id = queue.enqueue(1);
        let val = queue.get(id);
        assert!(val.is_some());
        let val = *val.unwrap();
        assert_eq!(1, val);
    }
}
