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

ioregs! (GPIO @ 0x50000000 = { //! General purpose input and output.
  0x504 => reg32 OUT { //! Write GPIO port.
    0..31 => PIN[32]: rw { //! Pins 0 to 31.
      0 => Low, //= Pin driver is low.
      1 => High, //= Pin driver is high.
    }
  }
  0x508 => reg32 OUTSET { //! Set individual bits in GPIO port.
    0..31 => PIN[32]: rw { //! Pins 0 to 31.
      0 => Low, //= Pin driver is low.
      1 => High, //= Pin driver is high.
    }
  }
  0x50c => reg32 OUTCLR { //! Clear individual bits in GPIO port.
    0..31 => PIN[32]: rw { //! Pins 0 to 31.
      0 => Low, //= Pin driver is low.
      1 => High, //= Pin driver is high.
    }
  }
  0x510 => reg32 IN { //! Read GPIO port.
    0..31 => PIN[32]: rw { //! Pins 0 to 31.
      0 => Low, //= Pin input is low.
      1 => High, //= Pin input is high.
    }
  }
  0x514 => reg32 DIR { //! Direction of GPIO pins.
    0..31 => PIN[32]: rw { //! Pins 0 to 31.
      0 => Input, //= Pin set as input.
      1 => Output, //= Pin set as output.
    }
  }
  0x518 => reg32 DIRSET { //! DIR set register.
    0..31 => PIN[32]: rw { //! Pins 0 to 31.
      0 => Input, //= Pin set as input.
      1 => Output, //= Pin set as output.
    }
  }
  0x51c => reg32 DIRCLR { //! DIR clear register.
    0..31 => PIN[32]: rw { //! Pins 0 to 31.
      0 => Input, //= Pin set as input.
      1 => Output, //= Pin set as output.
    }
  }
  0x700 => reg32 PIN_CNF[32] { //! Configuration of GPIO pins.
    0 => DIR: rw { //! Pin direction.
      0 => Input, //= Configure pin as an input pin.
      1 => Output, //= Configure pin as an output pin.
    }
    1 => INPUT: rw { //! Connect or disconnect input path.
      0 => Connect, //= Connect input pin.
      1 => Disconnect, //= Disconnect input pin.
    }
    2..3 => PULL: rw { //! Pull-up or -down configuration.
      0 => Disabled, //= No pull.
      1 => Pulldown, //= Pulldown on pin.
      3 => Pullup, //= Pullup on pin.
    }
    8..10 => DRIVE: rw { //! Drive configuration.
      0 => S0S1, //= Standard '0', Standard '1'.
      1 => H0S1, //= High '0', Standard '1'.
      2 => S0H1, //= Standard '0', High '1'.
      3 => H0H1, //= High '0', High '1'.
      4 => D0S1, //= Disconnected '0', Standard '1'.
      5 => D0H1, //= Disconnected '0', High '1'.
      6 => S0D1, //= Standard '0', Disconnected '1'.
      7 => H0D1, //= High '0', Disconnected '1'.
    }
    16..17 => SENSE: rw { //! Pin sensing mechanism.
      0 => Disabled, //= Disabled.
      2 => High, //= Wakeup on high level.
      3 => Low, //= Wakeup on low level.
    }
  }
});
