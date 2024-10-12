# The **P**ointer **Arr**ay (PArr).

## Provides a type to make raw pointer indexable as an array, just like in C!

# Examples

## A very basic usage
```rust
let arr: [u8; _] = [11_u8, 22, 33];  // Store array to some variable which is Parr will be point to.
let parr: Parr<u8> = Parr::from_ptr(arr.as_ptr());  // Create a Parr which is pointing to the out array.

assert_eq!(parr[1], 22);    // It works!
```

## Usage in structures
```rust
#[repr(C)]
struct Foo {
    arr: Parr<u8>,
}
```
Same code in C:
```c
struct foo {
    uint8_t* arr,
};
```

## Usage in function arguments
```rust
extern "C" fn foo(arr: Parr<u8>) { }
```
Same code in C:
```c
void foo(uint8_t* arr) { }
```

# Changelog

- `0.1.0`: First release.
- `0.1.1`: Raw pointer behavior.
- `0.1.2`: Mutability.
- `0.1.3`: The core `Default` trait.
