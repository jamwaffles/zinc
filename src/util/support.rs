// Zinc, the bare metal stack for rust.
// Copyright 2014 Vladimir "farcaller" Pouzanov <farcaller@gmail.com>
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Support functions currently required by the linker for bare-metal targets.

pub use core::intrinsics::breakpoint;

/// Call the debugger and halts execution.
#[no_mangle]
pub extern fn abort() -> ! {
  unsafe {
    breakpoint();
  }
  loop {}
}

#[doc(hidden)]
#[no_mangle]
pub extern fn __aeabi_memcpy(dest: *mut u8, src: *const u8, n: usize) -> *mut u8 {
  unsafe {
    let mut i = 0;

    while i < n {
      *dest.offset(i as isize) = *src.offset(i as isize);

      i += 1;
    }

    return dest;
  }
}

// TODO(bgamari): This is only necessary for exception handling and
// can be removed when we have this issue resolved.
#[doc(hidden)]
#[no_mangle]
pub extern fn __aeabi_memset(dest: *mut u8, size: usize, value: u32) {
  unsafe {
    use core::intrinsics::volatile_set_memory;
    volatile_set_memory(dest, value as u8, size);
  }
}

#[doc(hidden)]
#[no_mangle]
pub extern fn __aeabi_memclr(dest: *mut u8, size: usize) {
  unsafe {
    use core::intrinsics::volatile_set_memory;
    volatile_set_memory(dest, 0, size);
  }
}

#[cfg(target_arch = "arm")]
#[inline(always)]
/// NOP instruction
pub fn nop() {
  unsafe { asm!("nop" :::: "volatile"); }
}

#[cfg(not(target_arch = "arm"))]
/// NOP instruction (mock)
pub fn nop() {
}

#[cfg(target_arch = "arm")]
#[inline(always)]
/// WFI instruction
pub fn wfi() {
    unsafe { asm!("wfi" :::: "volatile"); }
}

#[cfg(not(target_arch = "arm"))]
/// WFI instruction (mock)
pub fn wfi() {
}

/// Hack to get a static 'ioreg' reference from a raw pointer to the register
/// base
pub fn get_reg_ref<T>(t: *const T) -> &'static T {
  unsafe {
    &*t
  }
}
