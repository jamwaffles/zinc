// Zinc, the bare metal stack for rust.
// Copyright 2014 Ben Gamari <bgamari@gmail.com>
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

/*!
ARM Cortex M4 floating point unit module

Cortex M4 FPU registers start at 0xE000_EF30.

The FPU is disabled by default on reset, so must be enabled with
`fpu::enable_fpu()` in initialisation code.
*/

#[inline(always)]
fn get_reg() -> &'static reg::FPU {
  unsafe { &*(0xE000_EF30 as *mut reg::FPU) }
}

/// Enable Full Access FPU mode
///
/// 0x0 = access denied
/// 0x1 = privileged access
/// 0x2 = reserved access
/// 0x3 = full access
pub fn enable_fpu() {
  get_reg().cpac.set_cp10(0x3);
  get_reg().cpac.set_cp11(0x3);
}

/// Disable FPU
pub fn disable_fpu() {
  get_reg().cpac.set_cp10(0x0);
  get_reg().cpac.set_cp11(0x0);
}

mod reg {
  use volatile_cell::VolatileCell;
  use core::ops::Drop;

  ioregs!(FPU = {
    0xd88     => reg32 cpac {      //! Coprocessor access control
      20..21 => cp10,
      22..23 => cp11,
    }
  });
}
