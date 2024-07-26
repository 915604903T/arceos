use core::arch::global_asm;
use tock_registers::interfaces::*;

global_asm!(include_str!("exception.S"));
use crate::arch::TrapFrame;

use core::sync::atomic::{AtomicUsize, Ordering};

type VmExitHandler = unsafe extern "C" fn();

#[no_mangle]
static mut LOWER_AARCH64_SYNCHROUNOUS_HANDLER: VmExitHandler = dummy_vmexit_handler;

#[no_mangle]
static mut LOWER_AARCH64_IRQ_HANDLER: VmExitHandler = dummy_vmexit_handler;

unsafe extern "C" fn dummy_vmexit_handler() {}

#[allow(dead_code)]
pub unsafe fn register_lower_aarch64_synchronous_handler(handler: VmExitHandler) {
    LOWER_AARCH64_SYNCHROUNOUS_HANDLER = handler;
}

#[allow(dead_code)]
pub unsafe fn register_lower_aarch64_irq_handler(handler: VmExitHandler) {
    LOWER_AARCH64_IRQ_HANDLER = handler;
}

// type ExceptionHandler = fn(&mut TrapFrame);

// const EMPTY_HANDLER_VALUE: AtomicUsize = AtomicUsize::new(0);

// static LOWER_AARCH64_HANDLERS: [AtomicUsize; MAX_EXCEPTION_COUNT] =
//     [EMPTY_HANDLER_VALUE; MAX_EXCEPTION_COUNT];

// const MAX_EXCEPTION_COUNT: usize = 64;

// #[allow(dead_code)]
// pub fn register_exception_handler_aarch64(exception_class: usize, handler: ExceptionHandler) -> bool {
//     if exception_class < MAX_EXCEPTION_COUNT {
//         LOWER_AARCH64_HANDLERS[exception_class].store(handler as usize, Ordering::SeqCst);
//         return true;
//     }
//     return false;
// }

// fn call_handler(exception_class: usize, tf: &mut TrapFrame) -> bool {
//     if exception_class < MAX_EXCEPTION_COUNT {
//         let handler = LOWER_AARCH64_HANDLERS[exception_class].load(Ordering::Acquire);
//         if handler != 0 {
//             let handler: ExceptionHandler = unsafe { core::mem::transmute(handler) };
//             handler(tf);
//             return true;
//         } else {
//             return false;
//         }
//     }else {
//         false
//     }
// }

#[repr(u8)]
#[derive(Debug)]
#[allow(dead_code)]
enum TrapKind {
    Synchronous = 0,
    Irq = 1,
    Fiq = 2,
    SError = 3,
}

#[repr(u8)]
#[derive(Debug)]
#[allow(dead_code)]
enum TrapSource {
    CurrentSpEl0 = 0,
    CurrentSpElx = 1,
    LowerAArch64 = 2,
    LowerAArch32 = 3,
}

/// deal with invalid aarch64 synchronous exception
#[no_mangle]
fn invalid_exception_el2(tf: &mut TrapFrame, kind: TrapKind, source: TrapSource) {
    panic!(
        "Invalid exception {:?} from {:?}:\n{:#x?}",
        kind, source, tf
    );
}

// /// deal with lower aarch64 interruption exception
// #[no_mangle]
// fn current_spxel_irq(ctx: &mut TrapFrame) {
//     lower_aarch64_irq(ctx);
// }

// /// deal with lower aarch64 interruption exception
// #[no_mangle]
// fn lower_aarch64_irq(ctx: &mut TrapFrame) {
//     let (irq, src) = gicc_get_current_irq();
//     debug!("src {} id{}", src, irq);
//     crate::trap::handle_irq_extern_hv(irq, src, ctx);
// }

// /// deal with lower aarch64 synchronous exception
// #[no_mangle]
// fn lower_aarch64_synchronous(tf: &mut TrapFrame) {
//     debug!(
//         "enter lower_aarch64_synchronous exception class:0x{:X}",
//         exception_class()
//     );
//     // 0x16: hvc_handler
//     // 0x24: data_abort_handler
//     call_handler(exception_class(), tf);
// }
