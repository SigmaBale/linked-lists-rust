# Linked Lists in Rust

Various examples of linked lists that you learn to build from [Learning Rust With Entirely Too Many Linked Lists](https://rust-unofficial.github.io/too-many-lists/index.html) (~290 pages).

## Bad Stack

Singly-linked list with a bad stack, module [`bad_singly_linked.rs`](https://github.com/SigmaBale/linked-lists-rust/blob/main/src/bad_singly_linked.rs).

## Ok Stack

Singly-linked list with good stack, module [`ok_singly_linked.rs`](https://github.com/SigmaBale/linked-lists-rust/blob/main/src/ok_singly_linked.rs).

## Persistent Stack

Singly-linked list using reference-counted pointers Rc, module [`rc_singly_linked.rs`](https://github.com/SigmaBale/linked-lists-rust/blob/main/src/rc_singly_linked.rs).

## Bad Deque

Doubly-linked list (stores head and tail of the list) using Rc and RefCell (interior mutability), module [`bad_deque.rs`](https://github.com/SigmaBale/linked-lists-rust/blob/main/src/bad_deque.rs).

## Ok Queue (unsafe)

Singly-linked queue type list that uses unsafe code and also does not break [Stacked Borrows Aliasing model](https://plv.mpi-sws.org/rustbelt/stacked-borrows/), module [`ok_queue_unsafe.rs`](https://github.com/SigmaBale/linked-lists-rust/blob/main/src/ok_queue_unsafe.rs).

## Linked List (doubly-linked-deque)

Production ready Linked List, module [`production_queue_unsafe.rs`](https://github.com/SigmaBale/linked-lists-rust/blob/main/src/production_queue_unsafe.rs).