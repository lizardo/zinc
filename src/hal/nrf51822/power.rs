//! Routines for power management.

use super::regs;


/// Power on RAM blocks 0 and 1 as recommended by PAN #16 (see nRF51822-PAN v2.4)
#[inline(always)]
pub fn ram_power_on()
{
  regs::POWER().RAMON
    .set_ONRAM0(regs::POWER_RAMON_ONRAM0::RAM0On)
    .set_ONRAM1(regs::POWER_RAMON_ONRAM1::RAM1On);
}
