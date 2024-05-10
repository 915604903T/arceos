use alloc::string::String;
use alloc::sync::{Arc, Weak};
use core::any::Any;
use core::sync::atomic::{AtomicU16, Ordering};
use hypercraft::{HyperResult, RegionOps};
use pci::config::{
    BarAllocTrait, RegionType, DEVICE_ID, PCI_CAP_ID_VNDR, PCI_CAP_VNDR_AND_NEXT_SIZE, REVISION_ID,
    SUBSYSTEM_ID, SUBSYSTEM_VENDOR_ID, SUB_CLASS_CODE, VENDOR_ID,
};
use pci::{le_write_u16, le_write_u32, AsAny, PciBus, PciConfig, PciDevBase, PciDevOps};
use spin::Mutex;

#[derive(Clone)]
pub struct DummyPciDevice<B: BarAllocTrait> {
    base: PciDevBase<B>,
    /// Device id
    dev_id: Arc<AtomicU16>,
    /// Device type
    device_type: u16,
}

impl<B: BarAllocTrait + 'static> DummyPciDevice<B> {
    pub fn new(
        name: String,
        devfn: u8,
        parent_bus: Weak<Mutex<PciBus<B>>>,
        device_type: u16,
    ) -> Self {
        Self {
            base: PciDevBase {
                id: name,
                config: PciConfig::<B>::new(0x1000, 3),
                devfn,
                parent_bus,
            },
            dev_id: Arc::new(AtomicU16::new(0)),
            device_type,
        }
    }

    fn device_type(&self) -> u16 {
        self.device_type
    }
}

impl<B: BarAllocTrait + 'static> AsAny for DummyPciDevice<B> {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

impl<B: BarAllocTrait + 'static> PciDevOps<B> for DummyPciDevice<B> {
    fn name(&self) -> String {
        self.base.id.clone()
    }

    fn pci_base(&self) -> &PciDevBase<B> {
        &self.base
    }

    fn pci_base_mut(&mut self) -> &mut PciDevBase<B> {
        &mut self.base
    }

    fn realize(mut self) -> HyperResult<()> {
        self.init_write_mask(false)?;
        self.init_write_clear_mask(false)?;
        let device_type = self.device_type();
        le_write_u16(&mut self.base.config.config, VENDOR_ID as usize, 0x1a1a)?;
        le_write_u16(
            &mut self.base.config.config,
            DEVICE_ID as usize,
            device_type as u16,
        )?;
        self.base.config.config[REVISION_ID] = 1;
        let class_id = 2;
        le_write_u16(
            &mut self.base.config.config,
            SUB_CLASS_CODE as usize,
            class_id,
        )?;
        le_write_u16(&mut self.base.config.config, SUBSYSTEM_VENDOR_ID, 0x1a1a)?;
        let subsysid = 0x40 + device_type as u16;
        le_write_u16(&mut self.base.config.config, SUBSYSTEM_ID, subsysid)?;
        // do not have real msi interrupt
        let cap_offset = self.base.config.add_pci_cap(0x5, 12 as usize)?;
        debug!("this is msi cap offset: {:#x}", cap_offset);

        self.base
            .config
            .register_bar(0x0 as usize, None, RegionType::Mem32Bit, false, 0x1000)?;

        let name = self.name();
        let devfn = self.base.devfn;
        let dev = Arc::new(Mutex::new(self));

        // Register device to pci bus. Now set it to the root bus.
        let pci_bus = dev.lock().base.parent_bus.upgrade().unwrap();
        let mut locked_pci_bus = pci_bus.lock();
        let pci_device = locked_pci_bus.devices.get(&devfn);
        if pci_device.is_none() {
            locked_pci_bus.devices.insert(devfn, dev.clone());
        } else {
            error!(
                "Devfn {:?} has been used by {:?}",
                &devfn,
                pci_device.unwrap().lock().name()
            );
        }

        Ok(())
    }

    fn write_config(&mut self, offset: usize, data: &[u8]) {
        self.base
            .config
            .write(offset, data, self.dev_id.load(Ordering::Relaxed));
    }
}
