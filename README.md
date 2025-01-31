# Doubly-Linked List in Rust

## Overview

This Rust library provides an implementation of a doubly-linked list using raw pointers (`*mut T`) while ensuring memory safety through careful pointer management. Unlike reference-counted (`Rc<RefCell<T>>`) approaches, this implementation avoids runtime borrow-checking overhead while maintaining efficient insertions, deletions, and traversal.

## Features

- Append elements to the back of the list
- Prepend elements to the front of the list
- Remove elements from both ends safely
- Iterate over elements efficiently
- Memory safety ensured through strict pointer management

## Example Usage

```{rust}
use doubly_linked_list::DoublyLinkedList;

fn main() {
    let mut list = DoublyLinkedList::new();
    list.append(1);
    list.append(2);
    list.append(3);

    while let Some(value) = list.pop_front() {
        println!("{}", value);
    }
}
```

Design Choices

1. Use of Raw Pointers (`*mut T`)

    Rust’s ownership model does not easily allow a doubly-linked list to be implemented with references (&T or &mut T) due to the borrowing restrictions.

    - Reason: Each node in a doubly-linked list must be linked to both the next and previous nodes, creating a cyclic ownership structure.

    - Why raw pointers? Using *mut T allows manual control over node relationships without requiring reference counting (Rc) or runtime borrow checking (RefCell).

2. Memory Safety Mechanisms

    - To ensure memory safety while using raw pointers:

    - Node allocation uses `Box<T>`, ensuring that nodes are heap-allocated and properly freed.

    - `Box::into_raw()` and `Box::from_raw()` convert between owned and raw pointers safely.

    - No dangling pointers: When a node is removed, its memory is freed properly.

    - No use-after-free: The list ensures nodes are accessed only when they are valid.

3. Drop Implementation for Cleanup

    Rust does not automatically free memory when using raw pointers. To avoid leaks, a custom Drop implementation ensures all nodes are deallocated when the list goes out of scope:

    ```{rust}
    impl<T> Drop for DoublyLinkedList<T> {
        fn drop(&mut self) {
            while self.pop_front().is_some() {}
        }
    }
    ```

4. Iterator Implementation

    To allow idiomatic Rust iteration, the list implements IntoIterator:

    ```{rust}
    impl<T> IntoIterator for DoublyLinkedList<T> {
        type Item = T;
        type IntoIter = IntoIter<T>;

        fn into_iter(self) -> Self::IntoIter {
            IntoIter(self)
        }
    }
    ```

    This allows usage like:

    ```{rust}
    for value in list.into_iter() {
        println!("{}", value);
    }
    ```

## Safety Considerations

While raw pointers are inherently unsafe, this implementation ensures safety by:

- Keeping ownership clear: Nodes are created with `Box<T>` and converted to raw pointers only when necessary.

- Enforcing proper deallocation: Nodes are freed as soon as they are removed, avoiding memory leaks.

- Preventing double frees: The list owns all nodes exclusively.

- No undefined behavior: All pointer dereferences are done within unsafe blocks with strict checks.

Performance Considerations

This implementation avoids unnecessary heap allocations by directly managing memory. The lack of `Rc` and `RefCell` ensures efficient, low-latency operations compared to other approaches. The time complexity remains:

- O(1) for insertions and deletions at either end.

- O(n) for traversal.

## Conclusion

This doubly-linked list implementation balances manual memory management with safety by using raw pointers carefully. By avoiding reference counting and interior mutability, it provides a fast and efficient data structure while maintaining Rust’s core safety guarantees.
