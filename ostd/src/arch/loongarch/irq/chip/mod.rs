// SPDX-License-Identifier: MPL-2.0

#![short_vis_path::add(arch)]

mod eiointc;

use loongArch64::register::ecfg::LineBasedInterrupt;

use self::eiointc::Eiointc;
use crate::arch::irq;

pub(in arch) fn init() {
    // FIXME: Support SMP in LoongArch
    Eiointc::init(1);
    for i in irq::IRQ_NUM_MIN..=irq::IRQ_NUM_MAX {
        Eiointc::enable(i);
    }
    loongArch64::register::ecfg::set_lie(
        LineBasedInterrupt::HWI0
            | LineBasedInterrupt::HWI1
            | LineBasedInterrupt::HWI2
            | LineBasedInterrupt::HWI3
            | LineBasedInterrupt::HWI4
            | LineBasedInterrupt::HWI5
            | LineBasedInterrupt::HWI6
            | LineBasedInterrupt::HWI7,
    );
}

pub(in arch) fn claim() -> Option<u8> {
    Eiointc::claim()
}

pub(in arch) fn complete(irq: u8) {
    Eiointc::complete(irq);
}
