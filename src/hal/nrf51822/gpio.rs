//! Routines for GPIO peripheral

use super::regs;

/// Configure GPIO pin for output
#[inline(always)]
pub fn configure_output(pin: usize)
{
  regs::GPIO().PIN_CNF[pin].ignoring_state()
    .set_DIR(regs::GPIO_PIN_CNF_DIR::Output)
    .set_INPUT(regs::GPIO_PIN_CNF_INPUT::Disconnect)
    .set_PULL(regs::GPIO_PIN_CNF_PULL::Disabled)
    .set_DRIVE(regs::GPIO_PIN_CNF_DRIVE::S0S1)
    .set_SENSE(regs::GPIO_PIN_CNF_SENSE::Disabled);
}

/// Clear the given GPIO pin
#[inline(always)]
pub fn pin_clear(pin: usize)
{
  regs::GPIO().OUTCLR.ignoring_state()
    .set_PIN(pin, regs::GPIO_OUTCLR_PIN::High);
}
