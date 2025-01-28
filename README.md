
# Doubly-Linked List Library

This library implements a memory-safe **Doubly-Linked List** in Rust. It demonstrates how to manage pointers safely in a low-level data structure while leveraging Rust's ownership and borrowing model to prevent common issues like dangling pointers and memory leaks.

---

## Usage

Here's a basic example of how to use the `DoublyLinkedList`:

```rust
use doubly_linked_list::DoublyLinkedList;

fn main() {
    let mut list = DoublyLinkedList::new();

    // Append values to the list
    list.append(1);
    list.append(2);
    list.append(3);

    // Iterate through the list
    for value in &list {
        println!("{}", value);
    }

    // Pop elements from the front and back
    assert_eq!(list.pop_front(), Some(1));
    assert_eq!(list.pop_back(), Some(3));

    // Check if the list is empty
    assert!(list.is_empty());
}
```

---

## API Documentation

### Struct: `DoublyLinkedList<T>`

A doubly-linked list structure that manages its elements safely.

#### Methods

- **`new()`**: Creates a new, empty list.

```rust
let mut list = DoublyLinkedList::new();
```

- **`append(value: T)`**: Adds a value to the end of the list.

```rust
list.append(10);
```

- **`prepend(value: T)`**: Adds a value to the beginning of the list.

```rust
list.prepend(5);
```

- **`pop_front() -> Option<T>`**: Removes the front element and returns its value, if any.

```rust
let front = list.pop_front();
```

- **`pop_back() -> Option<T>`**: Removes the last element and returns its value, if any.

```rust
let back = list.pop_back();
```

- **`len() -> usize`**: Returns the number of elements in the list.

```rust
let size = list.len();
```

- **`is_empty() -> bool`**: Checks whether the list is empty.

```rust
assert!(list.is_empty());
```

#### Iteration

The library supports iteration by consuming the list into an iterator:

```rust
let mut list = DoublyLinkedList::new();
list.append(1);
list.append(2);
list.append(3);

let mut iter = list.into_iter();
assert_eq!(iter.next(), Some(1));
assert_eq!(iter.next(), Some(2));
assert_eq!(iter.next(), Some(3));
assert_eq!(iter.next(), None);
```

---

## Design Details

### Memory Safety

This implementation achieves memory safety by adhering to Rust's ownership model:

- **Heap Allocation**: Nodes are created using `Box`, ensuring they are heap-allocated and deallocated when no longer needed.
- **Pointer Management**: All raw pointers (`*mut Node<T>`) are carefully maintained to avoid invalid dereferencing.
- **Drop Implementation**: The `Drop` trait ensures all nodes are properly deallocated when the list is dropped.

### Why Not Use `Rc` and `RefCell`?

To demonstrate low-level control and avoid runtime overhead, this implementation avoids `Rc` (reference counting) and `RefCell` (runtime borrow checking). Instead, it relies on raw pointers with strict safety checks.

---

## Running Tests

To run the test suite:

```bash
cargo test
```

### What is Tested?

- **Basic Operations**: Append, prepend, pop from the front and back.
- **Edge Cases**: Operations on empty lists, mixed operations.
- **Iterators**: Consuming and traversing the list using iterators.

---

## License

This project is licensed under the MIT License. See the `LICENSE` file for details.
