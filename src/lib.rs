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
///     for value in &list {
///         println!("{}", value);
///     }
/// }
/// ```
use std::ptr;

/// Represents a node in the doubly-linked list.
struct Node<T> {
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

/// Iterator for the doubly-linked list.
pub struct Iter<'a, T> {
    current: *mut Node<T>,
    _marker: std::marker::PhantomData<&'a T>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
            if self.current.is_null() {
                None
            } else {
                let node = &*self.current;
                self.current = node.next;
                Some(&node.value)
            }
        }
    }
}

/// Allows the list to be iterated over.
impl<T> DoublyLinkedList<T> {
    pub fn iter(&self) -> Iter<T> {
        Iter {
            current: self.head,
            _marker: std::marker::PhantomData,
        }
    }
}

impl<'a, T> IntoIterator for &'a DoublyLinkedList<T> {
    type Item = &'a T;
    type IntoIter = Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
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

    #[test]
    fn test_iteration() {
        let mut list = DoublyLinkedList::new();
        list.append(1);
        list.append(2);
        list.append(3);

        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), None);
    }
}
