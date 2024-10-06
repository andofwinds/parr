# The **P**ointer **Arr**ay.

## Turn your pointer into array!

Provides a C-like unknown-length array type. (The `T[]` type).

# Examples.
## Simple `[1; 2; 3]` array from slice.
Rust:
```rust
let arr = Parr::<u8>::from([1, 2, 3].as_slice());
assert_eq!(arr[1], 2);
```
C equivalent:
```C
uint8_t arr[] = {1, 2, 3};
```

## From pointer.
```rust
let arr = Parr::<u8>::new([1, 2, 3].as_ptr() as u64);
assert_eq!(arr[1], 2);
```
