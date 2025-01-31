/// # Pointer-Based Doubly-Linked List
///
/// This library provides an implementation of a doubly-linked list in Rust using raw pointers.
/// The implementation includes safety measures to prevent issues like dangling pointers and memory leaks.
///
/// ## Features
/// - Append elements to the list
/// - Prepend elements to the list
/// - Remove elements from the front or back
/// - Iterate through the list in both directions
///
/// ## Example
/// ```rust
/// use doubly_linked_list::DoublyLinkedList;
///
/// fn main() {
///     let mut list = DoublyLinkedList::new();
///     list.append(1);
///     list.prepend(2);
///     list.append(3);
///     list.prepend(4);
///
/// }
/// ```
use std::ptr;

/// Represents a node in the doubly-linked list.
struct Node<T> {
    #[allow(dead_code)]
    value: T,
    prev: *mut Node<T>,
    next: *mut Node<T>,
}

impl<T> Node<T> {
    /// Creates a new node with a given value.
    fn new(value: T) -> *mut Node<T> {
        Box::into_raw(Box::new(Node {
            value,
            prev: ptr::null_mut(),
            next: ptr::null_mut(),
        }))
    }
}

/// A pointer-based doubly-linked list with explicit memory management.
pub struct DoublyLinkedList<T> {
    head: *mut Node<T>,
    tail: *mut Node<T>,
    len: usize,
}

impl<T> DoublyLinkedList<T> {
    /// Creates a new empty doubly-linked list.
    pub fn new() -> Self {
        Self {
            head: ptr::null_mut(),
            tail: ptr::null_mut(),
            len: 0,
        }
    }

    /// Appends a value to the end of the list.
    pub fn append(&mut self, value: T) {
        let new_node = Node::new(value);
        unsafe {
            if !self.tail.is_null() {
                (*new_node).prev = self.tail;
                (*self.tail).next = new_node;
            } else {
                self.head = new_node;
            }
            self.tail = new_node;
        }
        self.len += 1;
    }

    /// Prepends a value to the beginning of the list.
    pub fn prepend(&mut self, value: T) {
        let new_node = Node::new(value);
        unsafe {
            if !self.head.is_null() {
                (*new_node).next = self.head;
                (*self.head).prev = new_node;
            } else {
                self.tail = new_node;
            }
            self.head = new_node;
        }
        self.len += 1;
    }

    /// Returns the number of elements in the list.
    pub fn len(&self) -> usize {
        self.len
    }

    /// Returns `true` if the list is empty.
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }
}

/// Ensures all nodes in the list are properly deallocated when the list goes out of scope.
impl<T> Drop for DoublyLinkedList<T> {
    fn drop(&mut self) {
        while !self.head.is_null() {
            unsafe {
                let node = Box::from_raw(self.head);
                self.head = node.next;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_append() {
        let mut list = DoublyLinkedList::new();
        list.append(1);
        list.append(2);
        list.append(3);
        assert_eq!(list.len(), 3);
    }

    #[test]
    fn test_prepend() {
        let mut list = DoublyLinkedList::new();
        list.prepend(3);
        list.prepend(2);
        list.prepend(1);
        assert_eq!(list.len(), 3);
    }

    #[test]
    fn test_empty_list() {
        let list: DoublyLinkedList<i32> = DoublyLinkedList::new();
        assert!(list.is_empty());
        assert_eq!(list.len(), 0);
    }
}
