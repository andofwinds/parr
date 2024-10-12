// (C) Andrey Baranov <andrey@elib.ru>, 2024.
//
//  This Source Code Form is subject to the terms of the Mozilla Public
//  License, v. 2.0. If a copy of the MPL was not distributed with this
//  file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![no_std]
#![doc = include_str!("../README.md")]

use core::{any::Any, ops::{Index, IndexMut}};

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
/// # The Pointer Array.
///
/// Holds pointer to a first element of the array (base address).
///
/// # Generics:
/// - `T`: Type of the each element in array.
pub struct Parr<T: Sized> ( *mut T );
impl<T: Sized> Parr<T> {
    /// ## Creates new array from the given `base` address pointer.
    ///
    /// ## Safety
    /// This function is safe but you need to make sure that pointer address is right.
    ///
    /// ## Generics:
    /// - `B`: Type of the `base` argument, anything that can be casted to `u64`.
    pub fn new<B: Into<u64>>(base: B) -> Self {
        Self ( base.into() as *mut T )
    }

    /// Creates a new `Self` from the given raw pointer.
    pub fn from_ptr<A: Any>(ptr: *const A) -> Self {
        Self (ptr as *mut T)
    }

    /// Creates a new `Self` from the given slice of `T`.
    pub fn from_slice(value: &[T]) -> Self {
        Self (
            value as *const _ as *mut T,
        )
    }

    /// Returns an array's base address.
    pub fn base(&self) -> u64 {
        self.0 as u64
    }
}
impl<T: Sized> From<&[T]> for Parr<T> {
    fn from(value: &[T]) -> Self {
        Self (
            value
                    as *const _
                    as *mut T
        )
    }
}
impl<T: Sized> Index<usize> for Parr<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        unsafe {
            &*self.0.add(index)
        }
    }
}
impl<T: Sized> IndexMut<usize> for Parr<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        unsafe {
            &mut *self.0.add(index)
        }
    }
}
impl <T: Sized+Default> Default for Parr<T> {
    fn default() -> Self {
        Self (
            &mut T::default()
        )
    }
}

#[cfg(test)]
mod tests {
    use core::mem::size_of;

    use crate::Parr;

    #[test]
    fn from_slice() {
        let arr: Parr<u8> = Parr::from([11_u8, 22, 33].as_slice());

        assert_eq!(arr[1], 22);
    }

    #[test]
    fn from_u64() {
        let arr: Parr<u8> = Parr::new([11_u8, 22, 33].as_ptr() as u64);

        assert_eq!(arr[1], 22);
    }

    #[test]
    fn from_ptr() {
        let raw_arr = [11_u8, 22, 33];
        let arr: Parr<u8> = Parr::from_ptr(raw_arr.as_ptr());

        assert_eq!(arr[1], 22);
    }

    #[test]
    fn change_state() {
        let raw_arr = [11_u8, 22, 33];  // Parr only keeps pointer so we need to store array to
                                        // some variable in order to prevent segfault when trynna
                                        // overwrite some array element.
        let mut arr: Parr<u8> = Parr::from_ptr(raw_arr.as_ptr());
        
        arr[1] = 42;

        assert_eq!(arr[1], 42);
    }

    #[test]
    fn foreign_volatile() {
        let members = [11_u8, 22, 33];
        let arr: Parr<u8> = Parr::from_ptr(members.as_ptr());

        // Volatile goes here.
        unsafe {
            let raw_member = (arr.base() + 1) as *mut u8; // Getting raw address of second member.
            *raw_member = 42;
        }

        assert_eq!(arr[1], 42);
    }

    #[test]
    fn foreign_access() {
        struct Msg {
            tags: Parr<u8>,
        }

        let tags: &[u8] = &[11_u8, 42, 56, 37];

        #[inline]
        fn new_msg(tags: &[u8]) {
            let msg = Msg {
                tags: Parr::from_slice(tags),
            };

            // Make a foreign access.
            unsafe {
                let elem: u8 = msg.tags[1];
                assert_eq!(elem, 42);
            }
        }

        let mut x: isize = 0;
        for i in 0..42 {
            x += i * 3 - 2;
        }

        new_msg(tags);
    }

    #[test]
    fn get_size() {
        assert_eq!(size_of::<Parr<u8>>(), 8); // 8 bytes on x64 systems,
                                              // so, it must be 4 on x32 systems.
    }

    #[test]
    fn struct_as_member() {
        #[derive(Debug, Copy, Clone, PartialEq)]
        struct Member {
            index: u8,
            state: u8,
        }

        let mut i: u8 = 0;
        let members: [Member; 5] = [Member {
            index: (|| {i += 1; i})(),
            state: i * 3,
        }; 5];

        let mut arr: Parr<Member> = Parr::from_ptr(members.as_ptr());

        arr[2] = Member{index:1, state:42};
        assert_eq!(arr[2], Member{index:1, state:42});
    }

    #[test]
    fn default() {
        struct Foo {
            bar: u8,
        }
        impl Default for Foo {
            fn default() -> Self {
                Self {
                    bar: 42,
                }
            }
        }

        let arr: Parr<Foo> = Parr::from_ptr([Foo::default()].as_ptr());

        assert_eq!(arr[0].bar, 42);
    }
}
