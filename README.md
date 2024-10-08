# The **P**ointer **Arr**ay (PArr).

## Provides a type to meke raw pointer indexable as an array, just like in C!

# Examples

## New from raw address
```rust
let arr: Parr<u8> = Parr::new(&[11_u8, 22, 33, 44] as *const _ as u64);
assert_eq!(arr[1], 22);
```
Same code in C:
```c
uint8_t arr[] = {11, 22, 33, 44};
// arr[1] == 22
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
