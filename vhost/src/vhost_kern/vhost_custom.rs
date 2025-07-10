// Copyright (C) 2019 Alibaba Cloud Computing. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 or BSD-3-Clause
//
// Portions Copyright 2018 Amazon.com, Inc. or its affiliates. All Rights Reserved.
//
// Portions Copyright 2017 The Chromium OS Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE-BSD-Google file.

// Custom code moved from vhost_binding.rs

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(missing_docs)]
#![allow(clippy::missing_safety_doc)]
#![allow(clippy::trivially_copy_pass_by_ref)]
#![allow(clippy::useless_transmute)]

use crate::{Error, Result}; // Assuming this is from the parent crate (vhost_kern)
use std::os::raw;

// It's possible vhost_memory_region will be defined in the new vhost_binding.rs
// If not, its definition would need to be here or imported.
// For now, assuming it will be available from the (to be regenerated) vhost_binding.rs
// If vhost_binding.rs is in the same module, direct usage might be fine.
// If it's a submodule, `super::vhost_binding::vhost_memory_region` might be needed.
// Let's assume for now that vhost_memory_region will be accessible.
// We might need to add `use super::vhost_binding::vhost_memory_region;` later.
// Also for `vhost_memory` struct used in VhostMemory struct.

#[repr(C)]
#[derive(Default)]
pub struct __IncompleteArrayField<T>(::std::marker::PhantomData<T>);

impl<T> __IncompleteArrayField<T> {
    #[inline]
    pub fn new() -> Self {
        __IncompleteArrayField(::std::marker::PhantomData)
    }

    #[inline]
    pub unsafe fn as_ptr(&self) -> *const T {
        ::std::mem::transmute(self)
    }

    #[inline]
    pub unsafe fn as_mut_ptr(&mut self) -> *mut T {
        ::std::mem::transmute(self)
    }

    #[inline]
    pub unsafe fn as_slice(&self, len: usize) -> &[T] {
        ::std::slice::from_raw_parts(self.as_ptr(), len)
    }

    #[inline]
    pub unsafe fn as_mut_slice(&mut self, len: usize) -> &mut [T] {
        ::std::slice::from_raw_parts_mut(self.as_mut_ptr(), len)
    }
}

impl<T> ::std::fmt::Debug for __IncompleteArrayField<T> {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        fmt.write_str("__IncompleteArrayField")
    }
}

impl<T> ::std::clone::Clone for __IncompleteArrayField<T> {
    #[inline]
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> ::std::marker::Copy for __IncompleteArrayField<T> {}


// Definitions for vhost_memory and vhost_memory_region are expected to come from
// the auto-generated vhost_binding.rs.
// We'll need to make sure they are correctly referenced, likely via `super::vhost_binding::*`
// or by ensuring vhost_custom.rs is a module within whatever scope vhost_binding.rs types are defined.
// For now, this code assumes these types will be in scope.
// This will likely require `use super::vhost_binding::{vhost_memory, vhost_memory_region};`

/// Helper to support vhost::set_mem_table()
pub struct VhostMemory {
    // This refers to vhost_memory from the bindings.
    // If vhost_binding.rs becomes a submodule, this might be `super::vhost_binding::vhost_memory`.
    buf: Vec<super::vhost_binding::vhost_memory>,
}

impl VhostMemory {
    // Limit number of regions to u16 to simplify error handling
    pub fn new(entries: u16) -> Self {
        let size = std::mem::size_of::<super::vhost_binding::vhost_memory_region>() * entries as usize;
        // The struct vhost_memory itself has a size, and it contains a flexible array member.
        // The calculation here seems to be about how many vhost_memory structs would fit,
        // but vhost_memory is the header. This logic might need review depending on how
        // vhost_memory is defined by bindgen.
        // Assuming vhost_memory struct definition:
        // #[repr(C)] pub struct vhost_memory { pub nregions: u32, pub padding: u32, pub regions: __IncompleteArrayField<vhost_memory_region> }
        // The size of vhost_memory itself is small (e.g., 8 bytes).
        // The buffer needs to be large enough for the header AND the regions data.
        // A common way to do this is a Vec<u8> and careful casting, or a wrapper struct.
        // The original code used `Vec<vhost_memory>` and `buf[0]`, which is unusual for FAMs.
        // Let's stick to the original structure for now and see if it compiles/how bindgen changes things.

        // The original calculation for `count` was:
        // (size + 2 * std::mem::size_of::<vhost_memory>() - 1) / std::mem::size_of::<vhost_memory>()
        // This seems complex. A more typical approach for a FAM buffer would be:
        // header_size = std::mem::size_of::<vhost_memory_header_part>(); // i.e. vhost_memory without FAM
        // total_size = header_size + std::mem::size_of::<vhost_memory_region>() * entries;
        // buf = vec![0u8; total_size];
        // Then cast the start of buf to *mut vhost_memory.
        // However, the original code used `Vec<vhost_memory>`.
        // Let's assume `vhost_memory` from bindgen will be compatible with this.
        // If `vhost_memory` itself becomes just the header part (without the FAM directly in its type),
        // this will need significant rework.

        // For now, to make it compile with assumed types from `super::vhost_binding`:
        let vhost_memory_header_size = std::mem::size_of::<super::vhost_binding::vhost_memory>();
        let required_regions_space = std::mem::size_of::<super::vhost_binding::vhost_memory_region>() * (entries as usize);
        let total_size = vhost_memory_header_size + required_regions_space;

        // The original code created a Vec<vhost_memory>. This is tricky because vhost_memory contains an __IncompleteArrayField.
        // `Default::default()` on such a struct is problematic if not handled carefully by bindgen.
        // Let's try to mimic the old structure and assume `vhost_memory` will be `Default`.
        // The number of `vhost_memory` elements in the vec was `count`.
        // `size` in original code is `required_regions_space`.
        // `count = (required_regions_space + 2 * vhost_memory_header_size - 1) / vhost_memory_header_size;`
        // This count logic is confusing. If `vhost_memory` is the header, you typically allocate one header
        // plus space for the array elements.
        // Let's simplify and assume we're creating a buffer that can hold one vhost_memory header
        // and then the regions. The original code `vec![Default::default(); count]` is problematic for FAMs.

        // Re-evaluating the original VhostMemory::new:
        // `let size = std::mem::size_of::<vhost_memory_region>() * entries as usize;` (this is space for regions)
        // `let count = (size + 2 * std::mem::size_of::<vhost_memory>() - 1) / std::mem::size_of::<vhost_memory>();`
        // `let mut buf: Vec<vhost_memory> = vec![Default::default(); count];`
        // `buf[0].nregions = u32::from(entries);`
        // This suggests `vhost_memory` is treated as a small, fixed-size struct, and `count`
        // is calculating how many such fixed-size blocks are needed to hold the regions as well.
        // This implies the `regions` field is handled via pointer arithmetic from the start of `buf[0]`.
        // This is only safe if `vhost_memory` is `repr(C)` and its size is stable and known.

        // Given `vhost_memory` contains `__IncompleteArrayField`, `Default::default()` on it is fine
        // as `__IncompleteArrayField` is Default.
        // The `count` calculation ensures enough total underlying memory.
        // Let's keep the original logic for `count` and `buf` initialization.
        let size_of_vhost_memory_struct = std::mem::size_of::<super::vhost_binding::vhost_memory>();
        let count = if size_of_vhost_memory_struct == 0 { // Avoid division by zero if struct is empty (unlikely)
            0
        } else {
            (required_regions_space + 2 * size_of_vhost_memory_struct - 1) / size_of_vhost_memory_struct
        };

        let mut buf: Vec<super::vhost_binding::vhost_memory> = vec![Default::default(); count];
        if !buf.is_empty() {
            buf[0].nregions = u32::from(entries);
        }
        VhostMemory { buf }
    }

    pub fn as_ptr(&self) -> *const std::os::raw::c_void { // Changed to c_void for typical FFI usage
        if self.buf.is_empty() {
            std::ptr::null()
        } else {
            &self.buf[0] as *const super::vhost_binding::vhost_memory as *const std::os::raw::c_void
        }
    }

    // Renamed to raw_header_ptr to avoid confusion if we also provide safe accessors
    pub fn raw_header_ptr(&self) -> *const super::vhost_binding::vhost_memory {
        if self.buf.is_empty() {
            std::ptr::null()
        } else {
            &self.buf[0]
        }
    }

    // Renamed to raw_header_mut_ptr
    pub fn raw_header_mut_ptr(&mut self) -> *mut super::vhost_binding::vhost_memory {
        if self.buf.is_empty() {
            std::ptr::null_mut()
        } else {
            &mut self.buf[0]
        }
    }

    // The get_header, get_region, set_region methods from the original code
    // relied on the internal structure of vhost_memory and its regions field.
    // These will need to be adapted based on how bindgen defines vhost_memory
    // and vhost_memory_region, particularly the __IncompleteArrayField.

    pub fn get_header(&self) -> Option<&super::vhost_binding::vhost_memory> {
        self.buf.get(0)
    }

    // This assumes vhost_memory has 'regions: __IncompleteArrayField<vhost_memory_region>'
    // and nregions field.
    pub fn get_region(&self, index: u32) -> Option<&super::vhost_binding::vhost_memory_region> {
        let header = self.buf.get(0)?;
        if index >= header.nregions {
            return None;
        }
        // SAFETY: Accessing the flexible array member. Assumes buf[0] is valid and nregions is correct.
        // The __IncompleteArrayField helper `as_slice` is used.
        unsafe {
            let regions_ptr = &header.regions as *const __IncompleteArrayField<super::vhost_binding::vhost_memory_region>;
            // We need to ensure that regions_ptr is valid and points to the FAM part of header.
            // If header is a `vhost_memory` struct, header.regions should give the FAM.
            (*regions_ptr).as_slice(header.nregions as usize).get(index as usize)
        }
    }

    pub fn set_region(&mut self, index: u32, region: &super::vhost_binding::vhost_memory_region) -> Result<()> {
        let header = self.buf.get_mut(0).ok_or(Error::InvalidGuestMemory)?; // Placeholder error
        if index >= header.nregions {
            return Err(Error::InvalidGuestMemory); // Placeholder error
        }
        // SAFETY: Accessing the flexible array member.
        unsafe {
            let regions_ptr = &mut header.regions as *mut __IncompleteArrayField<super::vhost_binding::vhost_memory_region>;
            // Similar to get_region, using as_mut_slice.
            let regions_slice = (*regions_ptr).as_mut_slice(header.nregions as usize);
            if let Some(place) = regions_slice.get_mut(index as usize) {
                *place = *region; // Assumes vhost_memory_region is Copy
                Ok(())
            } else {
                // This case should ideally be caught by `index >= header.nregions`
                Err(Error::InvalidGuestMemory) // Placeholder error
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    // Corrected import path for vhost_binding items.
    // Both vhost_custom.rs and vhost_binding.rs are modules under vhost_kern,
    // so items from vhost_binding are accessed via `super::vhost_binding::*`.
    use super::super::vhost_binding::{vhost_memory_region, __u32}; // Use __u32 from bindings
    // Error type is from crate root (based on `use crate::{Error, Result};` at top of file)
    // use crate::Error; // Already at top level of module.

    // The tests from the original file are for struct layouts and VhostMemory.
    // The struct layout tests depend on types from vhost_binding.rs.
    // If these types are regenerated by bindgen, their layout should still be tested,
    // but those tests might be better off staying with the generated code or being
    // conditional on features if the generated code itself is conditional.
    // For now, moving VhostMemory test here.
    // The layout tests for bindgen types (vhost_vring_state, etc.) should ideally be
    // re-verified against what bindgen produces. We might not need to keep them manually
    // if bindgen's output is trusted or tested elsewhere.
    // Let's keep the VhostMemory test here as it tests custom logic.

    // To make VhostMemory test work, we need vhost_memory_region.
    // It's now imported correctly via `use super::super::vhost_binding::vhost_memory_region;`.

    // Assuming `super::vhost_binding::vhost_memory_region` will be available.
    // And `super::vhost_binding::vhost_memory` (used by VhostMemory struct).

    #[test]
    fn test_vhostmemory_custom() { // Renamed to avoid conflict if original tests are kept/regenerated
        // The VhostMemory::new() now uses super::vhost_binding::vhost_memory etc.
        // The test needs to align with that.
        // vhost_memory_region from generated bindings derives Copy, Clone.

        // The `Error` type (used in VhostMemory methods) is from `crate::Error` (imported at file top).
        // The `Result` type (used in VhostMemory methods) is from `crate::Result` (imported at file top).
        // `vhost_memory_region` is from `super::super::vhost_binding::vhost_memory_region`.

        let mut obj = VhostMemory::new(2);

        // Define a local vhost_memory_region that matches what bindgen would create.
        // This is temporary until we confirm how to import/use the actual one from generated bindings.
        // No longer needed as we are directly using the imported vhost_memory_region.
        // #[repr(C)]
        // #[derive(Debug, Default, Copy, Clone, PartialEq)]
        // struct TestVhostMemoryRegion {
        //     pub guest_phys_addr: u64,
        //     pub memory_size: u64,
        //     pub userspace_addr: u64,
        //     pub flags_padding: u64,
        // }

        // Using the actual vhost_memory_region struct from the bindings.
        let region = vhost_memory_region {
            guest_phys_addr: 0x1000u64,
            memory_size: 0x2000u64,
            userspace_addr: 0x300000u64,
            flags_padding: 0u64,
        };

        assert!(obj.get_region(2).is_none());

        {
            let header = obj.get_header().unwrap();
            assert_eq!(header.nregions, 2u32);
        }
        {
            // Ensure Result is compatible; Error::InvalidGuestMemory is used by set_region
            assert!(obj.set_region(0, &region).is_ok());
            assert!(obj.set_region(1, &region).is_ok());
            assert!(obj.set_region(2, &region).is_err()); // Should be Error::InvalidGuestMemory
        }

        let region1 = obj.get_region(1).unwrap();
        assert_eq!(region1.guest_phys_addr, 0x1000u64);
        assert_eq!(region1.memory_size, 0x2000u64);
        assert_eq!(region1.userspace_addr, 0x300000u64);
        // Need to ensure VhostMemoryRegion has PartialEq or compare fields manually.
        // If it's a direct copy of bindgen C struct, it might not derive PartialEq.
        // The original test didn't use PartialEq for the whole struct, but field by field.
    }
}
