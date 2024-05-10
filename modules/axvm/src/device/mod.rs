#[cfg(target_arch = "x86_64")]
mod x86_64;

use pci::config::{BarAllocTrait, RegionType};
#[cfg(target_arch = "x86_64")]
pub use x86_64::*;

mod dummy_pci;
mod virtio;

use axalloc::global_allocator;
use hypercraft::{HyperError, HyperResult, MmioOps, PioOps, RegionOps, VirtMsrOps};

use core::ptr::NonNull;

#[derive(Clone)]
pub struct BarAllocImpl;
impl BarAllocTrait for BarAllocImpl {
    fn alloc(region_type: RegionType, size: u64) -> HyperResult<u64> {
        // The size is already 4k aligned
        if region_type != RegionType::Io {
            let pages = (size / 0x1000) as usize;
            if let Ok(vaddr) = global_allocator().alloc_pages(pages, 0x1000) {
                Ok(vaddr as u64)
            } else {
                Err(HyperError::InvalidBarAddress)
            }
        } else {
            Err(HyperError::InvalidBarAddress)
        }
    }

    fn dealloc(region_type: RegionType, vaddr: u64, size: u64) -> HyperResult<()> {
        if region_type != RegionType::Io {
            let pages = (size / 0x1000) as usize;
            global_allocator().dealloc_pages(vaddr as usize, pages);
            Ok(())
        } else {
            Err(HyperError::InvalidBarAddress)
        }
    }
}
