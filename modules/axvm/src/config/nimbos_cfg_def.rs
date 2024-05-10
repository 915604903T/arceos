use alloc::vec::Vec;

use page_table_entry::MappingFlags;

use crate::mm::GuestMemoryRegion;

pub fn nimbos_memory_regions_setup(regions: &mut Vec<GuestMemoryRegion>) {
    let guest_memory_regions = [
        GuestMemoryRegion {
            // syscall forwarder region
            gpa: 0x6700_0000,
            hpa: 0x700_0000,
            size: 0x0100_0000,
            flags: MappingFlags::READ | MappingFlags::WRITE | MappingFlags::EXECUTE,
        },
        // GuestMemoryRegion {
        //     // PCI
        //     gpa: 0x8000_0000,
        //     hpa: 0x8000_0000,
        //     size: 0x1000_0000,
        //     flags: MappingFlags::READ | MappingFlags::WRITE,
        // },
        // GuestMemoryRegion {
        //     gpa: 0xfe00_0000,
        //     hpa: 0xfe00_0000,
        //     size: 0x1_0000,
        //     flags: MappingFlags::READ | MappingFlags::WRITE | MappingFlags::DEVICE,
        // },
        // GuestMemoryRegion {
        //     gpa: 0xfeb0_0000,
        //     hpa: 0xfeb0_0000,
        //     size: 0x10_0000,
        //     flags: MappingFlags::READ | MappingFlags::WRITE | MappingFlags::DEVICE,
        // },
        GuestMemoryRegion {
            // IO APIC
            gpa: 0xfec0_0000,
            hpa: 0xfec0_0000,
            size: 0x1000,
            flags: MappingFlags::READ | MappingFlags::WRITE,
        },
        GuestMemoryRegion {
            // HPET
            gpa: 0xfed0_0000,
            hpa: 0xfed0_0000,
            size: 0x1000,
            flags: MappingFlags::READ | MappingFlags::WRITE,
        },
        GuestMemoryRegion {
            // Local APIC
            gpa: 0xfee0_0000,
            hpa: 0xfee0_0000,
            size: 0x1000,
            flags: MappingFlags::READ | MappingFlags::WRITE,
        },
        // SCF: memory region for shared memory should be configged here.
    ];
    for r in guest_memory_regions {
        regions.push(r);
    }
}
