/// # Doubly-Linked List Library
///
/// This library provides an implementation of a doubly-linked list in Rust.
///
/// ## Features
/// - Append elements to the list
/// - Prepend elements to the list
/// - Remove elements from the front or back
/// - Iterate through the list
///
/// ## Example
/// ```rust
/// use rust_doubly_linked_list::DoublyLinkedList;
///
/// fn main() {
///     let mut list = DoublyLinkedList::new();
///     list.append(1);
///     list.append(2);
///     list.append(3);
///
///     for value in list {
///         println!("{}", value);
///     }
/// }
/// ```

use std::ptr;

/// Represents a single element in the doubly-linked list
/// Each `Node` contains a value and pointers to both the previous and next nodes
pub struct Node<T> {
    value: T,
    prev: *mut Node<T>,
    next: *mut Node<T>,
}

impl<T> Node<T> {
    /// Creates a new node with the given value
    fn new(value: T) -> Box<Self> {
        Box::new(Node {
            value,
            prev: ptr::null_mut(),
            next: ptr::null_mut(),
        })
    }
}

/// The main structure for the doubly-linked list
/// Contains pointers to the head and tail of the list and tracks the number of elements
pub struct DoublyLinkedList<T> {
    head: *mut Node<T>,
    tail: *mut Node<T>,
    len: usize,
}

impl<T> DoublyLinkedList<T> {
    /// Creates a new, empty `DoublyLinkedList`
    pub fn new() -> Self {
        DoublyLinkedList {
            head: ptr::null_mut(),
            tail: ptr::null_mut(),
            len: 0,
        }
    }

    /// Adds a new node with the provided value to the end of the list
    /// Updates the tail pointer accordingly
    pub fn append(&mut self, value: T) {
        let mut new_node = Node::new(value);
        new_node.prev = self.tail;
        let new_node_ptr = Box::into_raw(new_node);

        unsafe {
            if !self.tail.is_null() {
                (*self.tail).next = new_node_ptr;
            }

            self.tail = new_node_ptr;

            if self.head.is_null() {
                self.head = new_node_ptr;
            }
        }
        self.len += 1;
    }

    /// Inserts a new node with the provided value at the beginning of the list
    /// Updates the head pointer accordingly
    pub fn prepend(&mut self, value: T) {
        let mut new_node = Node::new(value);
        new_node.next = self.head;
        let new_node_ptr = Box::into_raw(new_node);

        unsafe {
            if !self.head.is_null() {
                (*self.head).prev = new_node_ptr;
            }

            self.head = new_node_ptr;

            if self.tail.is_null() {
                self.tail = new_node_ptr;
            }
        }
        self.len += 1;
    }

    /// Removes the node at the front of the list
    /// Updates the head pointer and returns the value of the removed node
    pub fn pop_front(&mut self) -> Option<T> {
        if self.head.is_null() {
            return None;
        }

        unsafe {
            let old_head = Box::from_raw(self.head);
            self.head = old_head.next;

            if !self.head.is_null() {
                (*self.head).prev = ptr::null_mut();
            } else {
                self.tail = ptr::null_mut();
            }

            self.len -= 1;
            Some(old_head.value)
        }
    }

    /// Removes the node at the end of the list
    /// Updates the tail pointer and returns the value of the removed node
    pub fn pop_back(&mut self) -> Option<T> {
        if self.tail.is_null() {
            return None;
        }

        unsafe {
            let old_tail = Box::from_raw(self.tail);
            self.tail = old_tail.prev;

            if !self.tail.is_null() {
                (*self.tail).next = ptr::null_mut();
            } else {
                self.head = ptr::null_mut();
            }

            self.len -= 1;
            Some(old_tail.value)
        }
    }

    /// Returns the number of elements in the list
    pub fn len(&self) -> usize {
        self.len
    }

    /// Returns `true` if the list is empty
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }
}

/// Ensures all nodes in the list are properly deallocated when the list goes out of scope
impl<T> Drop for DoublyLinkedList<T> {
    fn drop(&mut self) {
        while self.pop_front().is_some() {}
    }
}

/// Enables consuming the list into an iterator
/// Allows for traversal and ownership transfer of elements
impl<T> IntoIterator for DoublyLinkedList<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter(self)
    }
}

/// A struct for consuming the doubly-linked list into an iterator
pub struct IntoIter<T>(DoublyLinkedList<T>);

/// Allows iterating over the list's elements by consuming them one at a time
impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop_front()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Tests appending values to the list
    #[test]
    fn test_append() {
        let mut list = DoublyLinkedList::new();
        list.append(1);
        list.append(2);
        list.append(3);

        assert_eq!(list.len(), 3);
        assert_eq!(list.pop_front(), Some(1));
        assert_eq!(list.pop_front(), Some(2));
        assert_eq!(list.pop_front(), Some(3));
    }

    /// Tests removing values from the front of the list
    #[test]
    fn test_pop_front() {
        let mut list = DoublyLinkedList::new();
        list.append(1);
        list.append(2);

        assert_eq!(list.pop_front(), Some(1));
        assert_eq!(list.pop_front(), Some(2));
        assert_eq!(list.pop_front(), None);
    }

    /// Tests removing values from the back of the list
    #[test]
    fn test_pop_back() {
        let mut list = DoublyLinkedList::new();
        list.append(1);
        list.append(2);

        assert_eq!(list.pop_back(), Some(2));
        assert_eq!(list.pop_back(), Some(1));
        assert_eq!(list.pop_back(), None);
    }

    /// Tests edge cases for an empty list
    #[test]
    fn test_empty_list() {
        let mut list: DoublyLinkedList<i32> = DoublyLinkedList::new();

        assert_eq!(list.len(), 0);
        assert_eq!(list.pop_front(), None);
        assert_eq!(list.pop_back(), None);
        assert!(list.is_empty());
    }

    /// Tests mixed operations on the list
    #[test]
    fn test_mixed_operations() {
        let mut list = DoublyLinkedList::new();
        list.append(1);
        list.append(2);
        list.prepend(0);

        assert_eq!(list.len(), 3);
        assert_eq!(list.pop_front(), Some(0));
        assert_eq!(list.pop_back(), Some(2));
        assert_eq!(list.pop_front(), Some(1));
        assert!(list.is_empty());
    }

    /// Tests iterator functionality
    #[test]
    fn test_iterator() {
        let mut list = DoublyLinkedList::new();
        list.append(1);
        list.append(2);
        list.append(3);

        let mut iter = list.into_iter();
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), None);
    }
}

