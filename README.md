# Pointer-Based Doubly-Linked List in Rust

This library provides an implementation of a doubly-linked list in Rust using raw pointers. It includes safety measures to minimize common pitfalls associated with raw pointers, such as dangling pointers and memory leaks. The implementation also includes an iterator for easy traversal of elements.

## Features

- Append elements to the list

- Prepend elements to the list

- Remove elements from the front or back

- Iterate through the list using Rust's Iterator trait

- Explicit memory management for performance optimization

## Example Usage

```{rust}
use doubly_linked_list::DoublyLinkedList;

fn main() {
    let mut list = DoublyLinkedList::new();
    list.append(1);
    list.prepend(2);
    list.append(3);
    list.prepend(4);

    for value in &list {
        println!("{}", value);
    }
}
```

## Implementation Details

### Data Structure

The `DoublyLinkedList<T>` consists of nodes that are connected using raw pointers. Each node contains:

- A value: `T`

- A prev pointer to the previous node

- A next pointer to the next node

### Memory Management

- Nodes are allocated on the heap using `Box::into_raw`.

- When a node is removed, it is safely deallocated using `Box::from_raw`.

- The `Drop` trait ensures all nodes are properly deallocated when the list goes out of scope.

### Iterators

The library implements:

- `Iterator` for forward traversal.

- `IntoIterator` for consuming iteration.

### Safety Considerations

- Raw pointers (`*mut T`) are used instead of smart pointers.

- Safety checks are implemented to ensure proper handling of pointers.

- No null pointer dereferencing or double frees occur.

## Running Tests

The implementation includes unit tests to verify correctness:

```{bash}
cargo test
```

## Why Use Raw Pointers?

Raw pointers are used to avoid the overhead of smart pointers (`Rc`, `RefCell`) and to provide fine-grained control over memory allocation and deallocation. This ensures:

- Better performance by avoiding reference counting overhead

- Explicit memory management, which is useful for low-level programming

- Learning experience for understanding Rust's ownership model

## License

This project is licensed under the MIT License.
