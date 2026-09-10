//! System configuration: the registers that wire peripherals to each other
//! rather than to the outside world.
//!
//! Only `EXTISS` is covered so far, through
//! [`ExtiLine::source`](crate::exti::ExtiLine::source).

use crate::pac;
use crate::rcu::Rcu;

/// Owns the SYSCFG registers.
pub struct Syscfg {
    syscfg: pac::Syscfg,
}

impl Syscfg {
    /// Points EXTI line `line` at the port encoded by `code`.
    ///
    /// # Panics
    ///
    /// If `line` is above 15 — the lines above that have no source to select.
    pub(crate) fn set_extiss(&mut self, line: u8, code: u8) {
        match line {
            l @ 0..=3 => self.syscfg.extiss0().modify(|_, w| match l {
                0 => unsafe { w.exti0_ss().bits(code) },
                1 => unsafe { w.exti1_ss().bits(code) },
                2 => unsafe { w.exti2_ss().bits(code) },
                _ => unsafe { w.exti3_ss().bits(code) },
            }),
            l @ 4..=7 => self.syscfg.extiss1().modify(|_, w| match l {
                4 => unsafe { w.exti4_ss().bits(code) },
                5 => unsafe { w.exti5_ss().bits(code) },
                6 => unsafe { w.exti6_ss().bits(code) },
                _ => unsafe { w.exti7_ss().bits(code) },
            }),
            l @ 8..=11 => self.syscfg.extiss2().modify(|_, w| match l {
                8 => unsafe { w.exti8_ss().bits(code) },
                9 => unsafe { w.exti9_ss().bits(code) },
                10 => unsafe { w.exti10_ss().bits(code) },
                _ => unsafe { w.exti11_ss().bits(code) },
            }),
            l @ 12..=15 => self.syscfg.extiss3().modify(|_, w| match l {
                12 => unsafe { w.exti12_ss().bits(code) },
                13 => unsafe { w.exti13_ss().bits(code) },
                14 => unsafe { w.exti14_ss().bits(code) },
                _ => unsafe { w.exti15_ss().bits(code) },
            }),
            _ => unreachable!(),
        }
    }
}

/// Takes the raw peripheral into [`Syscfg`].
pub trait SyscfgExt {
    /// Switches the SYSCFG clock on and hands back the owning type.
    ///
    /// The clock bit is shared with the comparator, so CMP is clocked too; see
    /// [`Rcu::enable_cfgcmp`].
    fn constrain(self, rcu: &mut Rcu) -> Syscfg;
}

impl SyscfgExt for pac::Syscfg {
    fn constrain(self, rcu: &mut Rcu) -> Syscfg {
        rcu.enable_cfgcmp();
        Syscfg { syscfg: self }
    }
}
