use std::collections::{BinaryHeap, HashMap};

use uuid::Uuid;

pub trait QueueId {
    fn set_id(&mut self, uuid: Uuid);
    fn id(&self) -> Uuid;
}

/// A queue which keeps a history of dequeued items
#[derive(Debug, Clone, Default)]
pub struct Queue<T>
where
    T: QueueId + Ord + Clone,
{
    history: HashMap<Uuid, T>,
    queue: BinaryHeap<T>,
}

impl<T: Ord + QueueId + Clone> Queue<T> {
    /// Creates an empty queue
    pub fn new() -> Self {
        Queue {
            history: HashMap::new(),
            queue: BinaryHeap::new(),
        }
    }

    /// Gets the item at the front of the queue
    // pub fn front(&self) -> Option<&T> {
    // self.get(self.front)
    // }

    /// Checks if an item at a given index has been dequeued
    // pub fn was_dequeued(&self, ndx: usize) -> bool {
    // ndx < self.front
    // }

    /// Enqueues an item to the back of the queue
    pub fn enqueue(&mut self, mut item: T) -> Uuid {
        let id = Uuid::new_v4();
        item.set_id(id);
        self.queue.push(item.clone());
        self.history.insert(id, item);
        id
    }

    /// Dequeues an item from the front of the queue
    pub fn dequeue_mut(&mut self) -> Option<&mut T> {
        self.queue
            .pop()
            .and_then(|item| self.history.get_mut(&item.id()))
    }

    /// Gets an item at the specified index in the queue
    pub fn get(&self, id: &Uuid) -> Option<&T> {
        self.history.get(id)
    }

    /// Gets an item at the specified index in the queue with mutable access
    pub fn get_mut(&mut self, id: &Uuid) -> Option<&mut T> {
        self.history.get_mut(id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn enqueue() {
    //     let mut queue = Queue::<i32>::new();
    //     let id = queue.enqueue(1);
    //     assert_eq!(0, id);
    // }

    // #[test]
    // fn dequeue() {
    //     let mut queue = Queue::<i32>::new();
    //     let _id = queue.enqueue(1);
    //     let val = queue.dequeue_mut();
    //     assert!(val.is_some());
    //     let val = *val.unwrap();
    //     assert_eq!(1, val);
    //     assert!(queue.dequeue_mut().is_none());
    // }

    // #[test]
    // fn get() {
    //     let mut queue = Queue::<i32>::new();
    //     let id = queue.enqueue(1);
    //     let val = queue.get(id);
    //     assert!(val.is_some());
    //     let val = *val.unwrap();
    //     assert_eq!(1, val);
    // }
}
