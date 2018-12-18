#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::RBCTR {
    #[doc = r" Modifies the contents of the register"]
    #[inline]
    pub fn modify<F>(&self, f: F)
    where
        for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
    {
        let bits = self.register.get();
        let r = R { bits: bits };
        let mut w = W { bits: bits };
        f(&r, &mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
    #[doc = r" Writes to the register"]
    #[inline]
    pub fn write<F>(&self, f: F)
    where
        F: FnOnce(&mut W) -> &mut W,
    {
        let mut w = W::reset_value();
        f(&mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Writes the reset value to the register"]
    #[inline]
    pub fn reset(&self) {
        self.write(|w| w)
    }
}
#[doc = r" Value of the field"]
pub struct LIMITR {
    bits: u8,
}
impl LIMITR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `SRBTM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRBTMR {
    #[doc = "Trigger mode 0: While TRBSR.SRBT=1, a standard receive buffer event will be generated whenever there is a new data received or data read out (depending on RBCTR.LOF setting). SRBT is cleared when TRBSR.RBFLVL=RBCTR.LIMIT."]
    VALUE1,
    #[doc = "Trigger mode 1: While TRBSR.SRBT=1, a standard receive buffer event will be generated whenever there is a new data received or data read out (depending on RBCTR.LOF setting). SRBT is cleared when TRBSR.RBFLVL=0."]
    VALUE2,
}
impl SRBTMR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            SRBTMR::VALUE1 => false,
            SRBTMR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SRBTMR {
        match value {
            false => SRBTMR::VALUE1,
            true => SRBTMR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == SRBTMR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == SRBTMR::VALUE2
    }
}
#[doc = "Possible values of the field `SRBTEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRBTENR {
    #[doc = "The standard receive buffer event trigger through bit TRBSR.SRBT is disabled."]
    VALUE1,
    #[doc = "The standard receive buffer event trigger through bit TRBSR.SRBT is enabled."]
    VALUE2,
}
impl SRBTENR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            SRBTENR::VALUE1 => false,
            SRBTENR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SRBTENR {
        match value {
            false => SRBTENR::VALUE1,
            true => SRBTENR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == SRBTENR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == SRBTENR::VALUE2
    }
}
#[doc = "Possible values of the field `SRBINP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRBINPR {
    #[doc = "Output SR0 becomes activated."]
    VALUE1,
    #[doc = "Output SR1 becomes activated."]
    VALUE2,
    #[doc = "Output SR2 becomes activated."]
    VALUE3,
    #[doc = "Output SR3 becomes activated."]
    VALUE4,
    #[doc = "Output SR4 becomes activated."]
    VALUE5,
    #[doc = "Output SR5 becomes activated."]
    VALUE6,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl SRBINPR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SRBINPR::VALUE1 => 0,
            SRBINPR::VALUE2 => 1,
            SRBINPR::VALUE3 => 2,
            SRBINPR::VALUE4 => 3,
            SRBINPR::VALUE5 => 4,
            SRBINPR::VALUE6 => 5,
            SRBINPR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SRBINPR {
        match value {
            0 => SRBINPR::VALUE1,
            1 => SRBINPR::VALUE2,
            2 => SRBINPR::VALUE3,
            3 => SRBINPR::VALUE4,
            4 => SRBINPR::VALUE5,
            5 => SRBINPR::VALUE6,
            i => SRBINPR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == SRBINPR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == SRBINPR::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == SRBINPR::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == SRBINPR::VALUE4
    }
    #[doc = "Checks if the value of the field is `VALUE5`"]
    #[inline]
    pub fn is_value5(&self) -> bool {
        *self == SRBINPR::VALUE5
    }
    #[doc = "Checks if the value of the field is `VALUE6`"]
    #[inline]
    pub fn is_value6(&self) -> bool {
        *self == SRBINPR::VALUE6
    }
}
#[doc = "Possible values of the field `ARBINP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ARBINPR {
    #[doc = "Output SR0 becomes activated."]
    VALUE1,
    #[doc = "Output SR1 becomes activated."]
    VALUE2,
    #[doc = "Output SR2 becomes activated."]
    VALUE3,
    #[doc = "Output SR3 becomes activated."]
    VALUE4,
    #[doc = "Output SR4 becomes activated."]
    VALUE5,
    #[doc = "Output SR5 becomes activated."]
    VALUE6,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl ARBINPR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            ARBINPR::VALUE1 => 0,
            ARBINPR::VALUE2 => 1,
            ARBINPR::VALUE3 => 2,
            ARBINPR::VALUE4 => 3,
            ARBINPR::VALUE5 => 4,
            ARBINPR::VALUE6 => 5,
            ARBINPR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> ARBINPR {
        match value {
            0 => ARBINPR::VALUE1,
            1 => ARBINPR::VALUE2,
            2 => ARBINPR::VALUE3,
            3 => ARBINPR::VALUE4,
            4 => ARBINPR::VALUE5,
            5 => ARBINPR::VALUE6,
            i => ARBINPR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == ARBINPR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == ARBINPR::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == ARBINPR::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == ARBINPR::VALUE4
    }
    #[doc = "Checks if the value of the field is `VALUE5`"]
    #[inline]
    pub fn is_value5(&self) -> bool {
        *self == ARBINPR::VALUE5
    }
    #[doc = "Checks if the value of the field is `VALUE6`"]
    #[inline]
    pub fn is_value6(&self) -> bool {
        *self == ARBINPR::VALUE6
    }
}
#[doc = "Possible values of the field `RCIM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RCIMR {
    #[doc = "RCI\\[4\\] = PERR, RCI\\[3:0\\] = WLEN"]
    VALUE1,
    #[doc = "RCI\\[4\\] = SOF, RCI\\[3:0\\] = WLEN"]
    VALUE2,
    #[doc = "RCI\\[4\\] = 0, RCI\\[3:0\\] = WLEN"]
    VALUE3,
    #[doc = "RCI\\[4\\] = PERR, RCI\\[3\\] = PAR, RCI\\[2:1\\] = 00B, RCI\\[0\\] = SOF"]
    VALUE4,
}
impl RCIMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            RCIMR::VALUE1 => 0,
            RCIMR::VALUE2 => 1,
            RCIMR::VALUE3 => 2,
            RCIMR::VALUE4 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> RCIMR {
        match value {
            0 => RCIMR::VALUE1,
            1 => RCIMR::VALUE2,
            2 => RCIMR::VALUE3,
            3 => RCIMR::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == RCIMR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == RCIMR::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == RCIMR::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == RCIMR::VALUE4
    }
}
#[doc = "Possible values of the field `SIZE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SIZER {
    #[doc = "The FIFO mechanism is disabled. The buffer does not accept any request for data."]
    VALUE1,
    #[doc = "The FIFO buffer contains 2 entries."]
    VALUE2,
    #[doc = "The FIFO buffer contains 4 entries."]
    VALUE3,
    #[doc = "The FIFO buffer contains 8 entries."]
    VALUE4,
    #[doc = "The FIFO buffer contains 16 entries."]
    VALUE5,
    #[doc = "The FIFO buffer contains 32 entries."]
    VALUE6,
    #[doc = "The FIFO buffer contains 64 entries."]
    VALUE7,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl SIZER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SIZER::VALUE1 => 0,
            SIZER::VALUE2 => 1,
            SIZER::VALUE3 => 2,
            SIZER::VALUE4 => 3,
            SIZER::VALUE5 => 4,
            SIZER::VALUE6 => 5,
            SIZER::VALUE7 => 6,
            SIZER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SIZER {
        match value {
            0 => SIZER::VALUE1,
            1 => SIZER::VALUE2,
            2 => SIZER::VALUE3,
            3 => SIZER::VALUE4,
            4 => SIZER::VALUE5,
            5 => SIZER::VALUE6,
            6 => SIZER::VALUE7,
            i => SIZER::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == SIZER::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == SIZER::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == SIZER::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == SIZER::VALUE4
    }
    #[doc = "Checks if the value of the field is `VALUE5`"]
    #[inline]
    pub fn is_value5(&self) -> bool {
        *self == SIZER::VALUE5
    }
    #[doc = "Checks if the value of the field is `VALUE6`"]
    #[inline]
    pub fn is_value6(&self) -> bool {
        *self == SIZER::VALUE6
    }
    #[doc = "Checks if the value of the field is `VALUE7`"]
    #[inline]
    pub fn is_value7(&self) -> bool {
        *self == SIZER::VALUE7
    }
}
#[doc = "Possible values of the field `RNM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RNMR {
    #[doc = "Filling level mode: A standard receive buffer event occurs when the filling level equals the limit value and changes, either due to a read access from OUTR (LOF = 0) or due to a new received data word (LOF = 1)."]
    VALUE1,
    #[doc = "RCI mode: A standard receive buffer event occurs when register OUTR is updated with a new value if the corresponding value in OUTR.RCI\\[4\\] = 0. If OUTR.RCI\\[4\\] = 1, an alternative receive buffer event occurs instead of the standard receive buffer event."]
    VALUE2,
}
impl RNMR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            RNMR::VALUE1 => false,
            RNMR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RNMR {
        match value {
            false => RNMR::VALUE1,
            true => RNMR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == RNMR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == RNMR::VALUE2
    }
}
#[doc = "Possible values of the field `LOF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LOFR {
    #[doc = "A standard receive buffer event occurs when the filling level equals the limit value and gets lower due to a read access from OUTR."]
    VALUE1,
    #[doc = "A standard receive buffer event occurs when the filling level equals the limit value and gets bigger due to the reception of a new data word."]
    VALUE2,
}
impl LOFR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            LOFR::VALUE1 => false,
            LOFR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LOFR {
        match value {
            false => LOFR::VALUE1,
            true => LOFR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == LOFR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == LOFR::VALUE2
    }
}
#[doc = "Possible values of the field `ARBIEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ARBIENR {
    #[doc = "The alternative receive buffer interrupt generation is disabled."]
    VALUE1,
    #[doc = "The alternative receive buffer interrupt generation is enabled."]
    VALUE2,
}
impl ARBIENR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            ARBIENR::VALUE1 => false,
            ARBIENR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ARBIENR {
        match value {
            false => ARBIENR::VALUE1,
            true => ARBIENR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == ARBIENR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == ARBIENR::VALUE2
    }
}
#[doc = "Possible values of the field `SRBIEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRBIENR {
    #[doc = "The standard receive buffer interrupt generation is disabled."]
    VALUE1,
    #[doc = "The standard receive buffer interrupt generation is enabled."]
    VALUE2,
}
impl SRBIENR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            SRBIENR::VALUE1 => false,
            SRBIENR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SRBIENR {
        match value {
            false => SRBIENR::VALUE1,
            true => SRBIENR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == SRBIENR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == SRBIENR::VALUE2
    }
}
#[doc = "Possible values of the field `RBERIEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RBERIENR {
    #[doc = "The receive buffer error interrupt generation is disabled."]
    VALUE1,
    #[doc = "The receive buffer error interrupt generation is enabled."]
    VALUE2,
}
impl RBERIENR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            RBERIENR::VALUE1 => false,
            RBERIENR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RBERIENR {
        match value {
            false => RBERIENR::VALUE1,
            true => RBERIENR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == RBERIENR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == RBERIENR::VALUE2
    }
}
#[doc = r" Proxy"]
pub struct _DPTRW<'a> {
    w: &'a mut W,
}
impl<'a> _DPTRW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 63;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _LIMITW<'a> {
    w: &'a mut W,
}
impl<'a> _LIMITW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 63;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SRBTM`"]
pub enum SRBTMW {
    #[doc = "Trigger mode 0: While TRBSR.SRBT=1, a standard receive buffer event will be generated whenever there is a new data received or data read out (depending on RBCTR.LOF setting). SRBT is cleared when TRBSR.RBFLVL=RBCTR.LIMIT."]
    VALUE1,
    #[doc = "Trigger mode 1: While TRBSR.SRBT=1, a standard receive buffer event will be generated whenever there is a new data received or data read out (depending on RBCTR.LOF setting). SRBT is cleared when TRBSR.RBFLVL=0."]
    VALUE2,
}
impl SRBTMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SRBTMW::VALUE1 => false,
            SRBTMW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SRBTMW<'a> {
    w: &'a mut W,
}
impl<'a> _SRBTMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SRBTMW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Trigger mode 0: While TRBSR.SRBT=1, a standard receive buffer event will be generated whenever there is a new data received or data read out (depending on RBCTR.LOF setting). SRBT is cleared when TRBSR.RBFLVL=RBCTR.LIMIT."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(SRBTMW::VALUE1)
    }
    #[doc = "Trigger mode 1: While TRBSR.SRBT=1, a standard receive buffer event will be generated whenever there is a new data received or data read out (depending on RBCTR.LOF setting). SRBT is cleared when TRBSR.RBFLVL=0."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(SRBTMW::VALUE2)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 14;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SRBTEN`"]
pub enum SRBTENW {
    #[doc = "The standard receive buffer event trigger through bit TRBSR.SRBT is disabled."]
    VALUE1,
    #[doc = "The standard receive buffer event trigger through bit TRBSR.SRBT is enabled."]
    VALUE2,
}
impl SRBTENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SRBTENW::VALUE1 => false,
            SRBTENW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SRBTENW<'a> {
    w: &'a mut W,
}
impl<'a> _SRBTENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SRBTENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The standard receive buffer event trigger through bit TRBSR.SRBT is disabled."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(SRBTENW::VALUE1)
    }
    #[doc = "The standard receive buffer event trigger through bit TRBSR.SRBT is enabled."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(SRBTENW::VALUE2)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 15;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SRBINP`"]
pub enum SRBINPW {
    #[doc = "Output SR0 becomes activated."]
    VALUE1,
    #[doc = "Output SR1 becomes activated."]
    VALUE2,
    #[doc = "Output SR2 becomes activated."]
    VALUE3,
    #[doc = "Output SR3 becomes activated."]
    VALUE4,
    #[doc = "Output SR4 becomes activated."]
    VALUE5,
    #[doc = "Output SR5 becomes activated."]
    VALUE6,
}
impl SRBINPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SRBINPW::VALUE1 => 0,
            SRBINPW::VALUE2 => 1,
            SRBINPW::VALUE3 => 2,
            SRBINPW::VALUE4 => 3,
            SRBINPW::VALUE5 => 4,
            SRBINPW::VALUE6 => 5,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SRBINPW<'a> {
    w: &'a mut W,
}
impl<'a> _SRBINPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SRBINPW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Output SR0 becomes activated."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(SRBINPW::VALUE1)
    }
    #[doc = "Output SR1 becomes activated."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(SRBINPW::VALUE2)
    }
    #[doc = "Output SR2 becomes activated."]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(SRBINPW::VALUE3)
    }
    #[doc = "Output SR3 becomes activated."]
    #[inline]
    pub fn value4(self) -> &'a mut W {
        self.variant(SRBINPW::VALUE4)
    }
    #[doc = "Output SR4 becomes activated."]
    #[inline]
    pub fn value5(self) -> &'a mut W {
        self.variant(SRBINPW::VALUE5)
    }
    #[doc = "Output SR5 becomes activated."]
    #[inline]
    pub fn value6(self) -> &'a mut W {
        self.variant(SRBINPW::VALUE6)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ARBINP`"]
pub enum ARBINPW {
    #[doc = "Output SR0 becomes activated."]
    VALUE1,
    #[doc = "Output SR1 becomes activated."]
    VALUE2,
    #[doc = "Output SR2 becomes activated."]
    VALUE3,
    #[doc = "Output SR3 becomes activated."]
    VALUE4,
    #[doc = "Output SR4 becomes activated."]
    VALUE5,
    #[doc = "Output SR5 becomes activated."]
    VALUE6,
}
impl ARBINPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            ARBINPW::VALUE1 => 0,
            ARBINPW::VALUE2 => 1,
            ARBINPW::VALUE3 => 2,
            ARBINPW::VALUE4 => 3,
            ARBINPW::VALUE5 => 4,
            ARBINPW::VALUE6 => 5,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ARBINPW<'a> {
    w: &'a mut W,
}
impl<'a> _ARBINPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ARBINPW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Output SR0 becomes activated."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(ARBINPW::VALUE1)
    }
    #[doc = "Output SR1 becomes activated."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(ARBINPW::VALUE2)
    }
    #[doc = "Output SR2 becomes activated."]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(ARBINPW::VALUE3)
    }
    #[doc = "Output SR3 becomes activated."]
    #[inline]
    pub fn value4(self) -> &'a mut W {
        self.variant(ARBINPW::VALUE4)
    }
    #[doc = "Output SR4 becomes activated."]
    #[inline]
    pub fn value5(self) -> &'a mut W {
        self.variant(ARBINPW::VALUE5)
    }
    #[doc = "Output SR5 becomes activated."]
    #[inline]
    pub fn value6(self) -> &'a mut W {
        self.variant(ARBINPW::VALUE6)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 19;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RCIM`"]
pub enum RCIMW {
    #[doc = "RCI\\[4\\] = PERR, RCI\\[3:0\\] = WLEN"]
    VALUE1,
    #[doc = "RCI\\[4\\] = SOF, RCI\\[3:0\\] = WLEN"]
    VALUE2,
    #[doc = "RCI\\[4\\] = 0, RCI\\[3:0\\] = WLEN"]
    VALUE3,
    #[doc = "RCI\\[4\\] = PERR, RCI\\[3\\] = PAR, RCI\\[2:1\\] = 00B, RCI\\[0\\] = SOF"]
    VALUE4,
}
impl RCIMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            RCIMW::VALUE1 => 0,
            RCIMW::VALUE2 => 1,
            RCIMW::VALUE3 => 2,
            RCIMW::VALUE4 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RCIMW<'a> {
    w: &'a mut W,
}
impl<'a> _RCIMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RCIMW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "RCI\\[4\\] = PERR, RCI\\[3:0\\] = WLEN"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(RCIMW::VALUE1)
    }
    #[doc = "RCI\\[4\\] = SOF, RCI\\[3:0\\] = WLEN"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(RCIMW::VALUE2)
    }
    #[doc = "RCI\\[4\\] = 0, RCI\\[3:0\\] = WLEN"]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(RCIMW::VALUE3)
    }
    #[doc = "RCI\\[4\\] = PERR, RCI\\[3\\] = PAR, RCI\\[2:1\\] = 00B, RCI\\[0\\] = SOF"]
    #[inline]
    pub fn value4(self) -> &'a mut W {
        self.variant(RCIMW::VALUE4)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 22;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SIZE`"]
pub enum SIZEW {
    #[doc = "The FIFO mechanism is disabled. The buffer does not accept any request for data."]
    VALUE1,
    #[doc = "The FIFO buffer contains 2 entries."]
    VALUE2,
    #[doc = "The FIFO buffer contains 4 entries."]
    VALUE3,
    #[doc = "The FIFO buffer contains 8 entries."]
    VALUE4,
    #[doc = "The FIFO buffer contains 16 entries."]
    VALUE5,
    #[doc = "The FIFO buffer contains 32 entries."]
    VALUE6,
    #[doc = "The FIFO buffer contains 64 entries."]
    VALUE7,
}
impl SIZEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SIZEW::VALUE1 => 0,
            SIZEW::VALUE2 => 1,
            SIZEW::VALUE3 => 2,
            SIZEW::VALUE4 => 3,
            SIZEW::VALUE5 => 4,
            SIZEW::VALUE6 => 5,
            SIZEW::VALUE7 => 6,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SIZEW<'a> {
    w: &'a mut W,
}
impl<'a> _SIZEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SIZEW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "The FIFO mechanism is disabled. The buffer does not accept any request for data."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(SIZEW::VALUE1)
    }
    #[doc = "The FIFO buffer contains 2 entries."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(SIZEW::VALUE2)
    }
    #[doc = "The FIFO buffer contains 4 entries."]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(SIZEW::VALUE3)
    }
    #[doc = "The FIFO buffer contains 8 entries."]
    #[inline]
    pub fn value4(self) -> &'a mut W {
        self.variant(SIZEW::VALUE4)
    }
    #[doc = "The FIFO buffer contains 16 entries."]
    #[inline]
    pub fn value5(self) -> &'a mut W {
        self.variant(SIZEW::VALUE5)
    }
    #[doc = "The FIFO buffer contains 32 entries."]
    #[inline]
    pub fn value6(self) -> &'a mut W {
        self.variant(SIZEW::VALUE6)
    }
    #[doc = "The FIFO buffer contains 64 entries."]
    #[inline]
    pub fn value7(self) -> &'a mut W {
        self.variant(SIZEW::VALUE7)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RNM`"]
pub enum RNMW {
    #[doc = "Filling level mode: A standard receive buffer event occurs when the filling level equals the limit value and changes, either due to a read access from OUTR (LOF = 0) or due to a new received data word (LOF = 1)."]
    VALUE1,
    #[doc = "RCI mode: A standard receive buffer event occurs when register OUTR is updated with a new value if the corresponding value in OUTR.RCI\\[4\\] = 0. If OUTR.RCI\\[4\\] = 1, an alternative receive buffer event occurs instead of the standard receive buffer event."]
    VALUE2,
}
impl RNMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RNMW::VALUE1 => false,
            RNMW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RNMW<'a> {
    w: &'a mut W,
}
impl<'a> _RNMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RNMW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Filling level mode: A standard receive buffer event occurs when the filling level equals the limit value and changes, either due to a read access from OUTR (LOF = 0) or due to a new received data word (LOF = 1)."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(RNMW::VALUE1)
    }
    #[doc = "RCI mode: A standard receive buffer event occurs when register OUTR is updated with a new value if the corresponding value in OUTR.RCI\\[4\\] = 0. If OUTR.RCI\\[4\\] = 1, an alternative receive buffer event occurs instead of the standard receive buffer event."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(RNMW::VALUE2)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 27;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `LOF`"]
pub enum LOFW {
    #[doc = "A standard receive buffer event occurs when the filling level equals the limit value and gets lower due to a read access from OUTR."]
    VALUE1,
    #[doc = "A standard receive buffer event occurs when the filling level equals the limit value and gets bigger due to the reception of a new data word."]
    VALUE2,
}
impl LOFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LOFW::VALUE1 => false,
            LOFW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LOFW<'a> {
    w: &'a mut W,
}
impl<'a> _LOFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LOFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "A standard receive buffer event occurs when the filling level equals the limit value and gets lower due to a read access from OUTR."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(LOFW::VALUE1)
    }
    #[doc = "A standard receive buffer event occurs when the filling level equals the limit value and gets bigger due to the reception of a new data word."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(LOFW::VALUE2)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 28;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ARBIEN`"]
pub enum ARBIENW {
    #[doc = "The alternative receive buffer interrupt generation is disabled."]
    VALUE1,
    #[doc = "The alternative receive buffer interrupt generation is enabled."]
    VALUE2,
}
impl ARBIENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ARBIENW::VALUE1 => false,
            ARBIENW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ARBIENW<'a> {
    w: &'a mut W,
}
impl<'a> _ARBIENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ARBIENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The alternative receive buffer interrupt generation is disabled."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(ARBIENW::VALUE1)
    }
    #[doc = "The alternative receive buffer interrupt generation is enabled."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(ARBIENW::VALUE2)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 29;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SRBIEN`"]
pub enum SRBIENW {
    #[doc = "The standard receive buffer interrupt generation is disabled."]
    VALUE1,
    #[doc = "The standard receive buffer interrupt generation is enabled."]
    VALUE2,
}
impl SRBIENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SRBIENW::VALUE1 => false,
            SRBIENW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SRBIENW<'a> {
    w: &'a mut W,
}
impl<'a> _SRBIENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SRBIENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The standard receive buffer interrupt generation is disabled."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(SRBIENW::VALUE1)
    }
    #[doc = "The standard receive buffer interrupt generation is enabled."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(SRBIENW::VALUE2)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 30;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RBERIEN`"]
pub enum RBERIENW {
    #[doc = "The receive buffer error interrupt generation is disabled."]
    VALUE1,
    #[doc = "The receive buffer error interrupt generation is enabled."]
    VALUE2,
}
impl RBERIENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RBERIENW::VALUE1 => false,
            RBERIENW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RBERIENW<'a> {
    w: &'a mut W,
}
impl<'a> _RBERIENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RBERIENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The receive buffer error interrupt generation is disabled."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(RBERIENW::VALUE1)
    }
    #[doc = "The receive buffer error interrupt generation is enabled."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(RBERIENW::VALUE2)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 31;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 8:13 - Limit For Interrupt Generation"]
    #[inline]
    pub fn limit(&self) -> LIMITR {
        let bits = {
            const MASK: u8 = 63;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        LIMITR { bits }
    }
    #[doc = "Bit 14 - Standard Receive Buffer Trigger Mode"]
    #[inline]
    pub fn srbtm(&self) -> SRBTMR {
        SRBTMR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 15 - Standard Receive Buffer Trigger Enable"]
    #[inline]
    pub fn srbten(&self) -> SRBTENR {
        SRBTENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 16:18 - Standard Receive Buffer Interrupt Node Pointer"]
    #[inline]
    pub fn srbinp(&self) -> SRBINPR {
        SRBINPR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 19:21 - Alternative Receive Buffer Interrupt Node Pointer"]
    #[inline]
    pub fn arbinp(&self) -> ARBINPR {
        ARBINPR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 22:23 - Receiver Control Information Mode"]
    #[inline]
    pub fn rcim(&self) -> RCIMR {
        RCIMR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 24:26 - Buffer Size"]
    #[inline]
    pub fn size(&self) -> SIZER {
        SIZER::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 27 - Receiver Notification Mode"]
    #[inline]
    pub fn rnm(&self) -> RNMR {
        RNMR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 28 - Buffer Event on Limit Overflow"]
    #[inline]
    pub fn lof(&self) -> LOFR {
        LOFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 29 - Alternative Receive Buffer Interrupt Enable"]
    #[inline]
    pub fn arbien(&self) -> ARBIENR {
        ARBIENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 29;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 30 - Standard Receive Buffer Interrupt Enable"]
    #[inline]
    pub fn srbien(&self) -> SRBIENR {
        SRBIENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 31 - Receive Buffer Error Interrupt Enable"]
    #[inline]
    pub fn rberien(&self) -> RBERIENR {
        RBERIENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 31;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 0 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:5 - Data Pointer"]
    #[inline]
    pub fn dptr(&mut self) -> _DPTRW {
        _DPTRW { w: self }
    }
    #[doc = "Bits 8:13 - Limit For Interrupt Generation"]
    #[inline]
    pub fn limit(&mut self) -> _LIMITW {
        _LIMITW { w: self }
    }
    #[doc = "Bit 14 - Standard Receive Buffer Trigger Mode"]
    #[inline]
    pub fn srbtm(&mut self) -> _SRBTMW {
        _SRBTMW { w: self }
    }
    #[doc = "Bit 15 - Standard Receive Buffer Trigger Enable"]
    #[inline]
    pub fn srbten(&mut self) -> _SRBTENW {
        _SRBTENW { w: self }
    }
    #[doc = "Bits 16:18 - Standard Receive Buffer Interrupt Node Pointer"]
    #[inline]
    pub fn srbinp(&mut self) -> _SRBINPW {
        _SRBINPW { w: self }
    }
    #[doc = "Bits 19:21 - Alternative Receive Buffer Interrupt Node Pointer"]
    #[inline]
    pub fn arbinp(&mut self) -> _ARBINPW {
        _ARBINPW { w: self }
    }
    #[doc = "Bits 22:23 - Receiver Control Information Mode"]
    #[inline]
    pub fn rcim(&mut self) -> _RCIMW {
        _RCIMW { w: self }
    }
    #[doc = "Bits 24:26 - Buffer Size"]
    #[inline]
    pub fn size(&mut self) -> _SIZEW {
        _SIZEW { w: self }
    }
    #[doc = "Bit 27 - Receiver Notification Mode"]
    #[inline]
    pub fn rnm(&mut self) -> _RNMW {
        _RNMW { w: self }
    }
    #[doc = "Bit 28 - Buffer Event on Limit Overflow"]
    #[inline]
    pub fn lof(&mut self) -> _LOFW {
        _LOFW { w: self }
    }
    #[doc = "Bit 29 - Alternative Receive Buffer Interrupt Enable"]
    #[inline]
    pub fn arbien(&mut self) -> _ARBIENW {
        _ARBIENW { w: self }
    }
    #[doc = "Bit 30 - Standard Receive Buffer Interrupt Enable"]
    #[inline]
    pub fn srbien(&mut self) -> _SRBIENW {
        _SRBIENW { w: self }
    }
    #[doc = "Bit 31 - Receive Buffer Error Interrupt Enable"]
    #[inline]
    pub fn rberien(&mut self) -> _RBERIENW {
        _RBERIENW { w: self }
    }
}
