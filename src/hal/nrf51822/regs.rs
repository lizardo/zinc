#![allow(non_snake_case)]

use volatile_cell::VolatileCell;
use core::ops::Drop;

ioregs! (POWER @ 0x40000000 = { //! Power Control.
  0x524 => reg32 RAMON { //! Ram on/off.
    0 => ONRAM0: rw { //! RAM block 0 behaviour in ON mode.
      0 => RAM0Off, //= RAM block 0 OFF in ON mode.
      1 => RAM0On, //= RAM block 0 ON in ON mode.
    }
    1 => ONRAM1: rw { //! RAM block 1 behaviour in ON mode.
      0 => RAM1Off, //= RAM block 1 OFF in ON mode.
      1 => RAM1On, //= RAM block 1 ON in ON mode.
    }
    16 => OFFRAM0: rw { //! RAM block 0 behaviour in OFF mode.
      0 => RAM0Off, //= RAM block 0 OFF in OFF mode.
      1 => RAM0On, //= RAM block 0 ON in OFF mode.
    }
    17 => OFFRAM1: rw { //! RAM block 1 behaviour in OFF mode.
      0 => RAM1Off, //= RAM block 1 OFF in OFF mode.
      1 => RAM1On, //= RAM block 1 ON in OFF mode.
    }
  }
});
