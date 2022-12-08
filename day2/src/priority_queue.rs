use std::cmp::PartialOrd;
use std::default::Default;
use std::slice::Iter;

#[derive(Debug, Default)]
pub struct PriorityQueue<T> {
    data: Vec<T>
}

impl<T: PartialOrd + Default> PriorityQueue<T> {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn push(&mut self, x: T) {
        let i = self.data.partition_point(|t| &x < t);
        self.data.insert(i, x);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.data.pop()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn iter(&self) -> Iter<'_, T> {
        self.data.iter()
    }
}

impl<T> IntoIterator for PriorityQueue<T> {
    type Item = T;
    type IntoIter = std::vec::IntoIter<T>;
    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter()
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn add() {
        let mut pq = PriorityQueue::new();
        pq.push(3);
        pq.push(1);
        pq.push(2);
        assert_eq!(pq.len(), 3);
        assert_eq!(pq.pop(), Some(1));
        assert_eq!(pq.pop(), Some(2));
        assert_eq!(pq.pop(), Some(3));
        assert!(pq.is_empty());
    }

    #[test]
    fn test_iter() {
        let mut pq = PriorityQueue::new();
        pq.push(7);
        pq.push(9);
        pq.push(8);
        assert_eq!(pq.iter().collect::<Vec<_>>(), &[&9, &8, &7]);
        assert_eq!(pq.into_iter().collect::<Vec<_>>(), &[9, 8, 7]);
    }
}
