#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::TBCTR {
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
#[doc = "Possible values of the field `STBTM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STBTMR {
    #[doc = "Trigger mode 0: While TRBSR.STBT=1, a standard buffer event will be generated whenever there is a data transfer to TBUF or data write to INx (depending on TBCTR.LOF setting). STBT is cleared when TRBSR.TBFLVL=TBCTR.LIMIT."]
    VALUE1,
    #[doc = "Trigger mode 1: While TRBSR.STBT=1, a standard buffer event will be generated whenever there is a data transfer to TBUF or data write to INx (depending on TBCTR.LOF setting). STBT is cleared when TRBSR.TBFLVL=TBCTR.SIZE."]
    VALUE2,
}
impl STBTMR {
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
            STBTMR::VALUE1 => false,
            STBTMR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> STBTMR {
        match value {
            false => STBTMR::VALUE1,
            true => STBTMR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == STBTMR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == STBTMR::VALUE2
    }
}
#[doc = "Possible values of the field `STBTEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STBTENR {
    #[doc = "The standard transmit buffer event trigger through bit TRBSR.STBT is disabled."]
    VALUE1,
    #[doc = "The standard transmit buffer event trigger through bit TRBSR.STBT is enabled."]
    VALUE2,
}
impl STBTENR {
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
            STBTENR::VALUE1 => false,
            STBTENR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> STBTENR {
        match value {
            false => STBTENR::VALUE1,
            true => STBTENR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == STBTENR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == STBTENR::VALUE2
    }
}
#[doc = "Possible values of the field `STBINP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STBINPR {
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
impl STBINPR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            STBINPR::VALUE1 => 0,
            STBINPR::VALUE2 => 1,
            STBINPR::VALUE3 => 2,
            STBINPR::VALUE4 => 3,
            STBINPR::VALUE5 => 4,
            STBINPR::VALUE6 => 5,
            STBINPR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> STBINPR {
        match value {
            0 => STBINPR::VALUE1,
            1 => STBINPR::VALUE2,
            2 => STBINPR::VALUE3,
            3 => STBINPR::VALUE4,
            4 => STBINPR::VALUE5,
            5 => STBINPR::VALUE6,
            i => STBINPR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == STBINPR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == STBINPR::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == STBINPR::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == STBINPR::VALUE4
    }
    #[doc = "Checks if the value of the field is `VALUE5`"]
    #[inline]
    pub fn is_value5(&self) -> bool {
        *self == STBINPR::VALUE5
    }
    #[doc = "Checks if the value of the field is `VALUE6`"]
    #[inline]
    pub fn is_value6(&self) -> bool {
        *self == STBINPR::VALUE6
    }
}
#[doc = "Possible values of the field `ATBINP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ATBINPR {
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
impl ATBINPR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            ATBINPR::VALUE1 => 0,
            ATBINPR::VALUE2 => 1,
            ATBINPR::VALUE3 => 2,
            ATBINPR::VALUE4 => 3,
            ATBINPR::VALUE5 => 4,
            ATBINPR::VALUE6 => 5,
            ATBINPR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> ATBINPR {
        match value {
            0 => ATBINPR::VALUE1,
            1 => ATBINPR::VALUE2,
            2 => ATBINPR::VALUE3,
            3 => ATBINPR::VALUE4,
            4 => ATBINPR::VALUE5,
            5 => ATBINPR::VALUE6,
            i => ATBINPR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == ATBINPR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == ATBINPR::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == ATBINPR::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == ATBINPR::VALUE4
    }
    #[doc = "Checks if the value of the field is `VALUE5`"]
    #[inline]
    pub fn is_value5(&self) -> bool {
        *self == ATBINPR::VALUE5
    }
    #[doc = "Checks if the value of the field is `VALUE6`"]
    #[inline]
    pub fn is_value6(&self) -> bool {
        *self == ATBINPR::VALUE6
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
#[doc = "Possible values of the field `LOF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LOFR {
    #[doc = "A standard transmit buffer event occurs when the filling level equals the limit value and gets lower due to transmission of a data word."]
    VALUE1,
    #[doc = "A standard transmit buffer interrupt event occurs when the filling level equals the limit value and gets bigger due to a write access to a data input location INx."]
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
#[doc = "Possible values of the field `STBIEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STBIENR {
    #[doc = "The standard transmit buffer interrupt generation is disabled."]
    VALUE1,
    #[doc = "The standard transmit buffer interrupt generation is enabled."]
    VALUE2,
}
impl STBIENR {
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
            STBIENR::VALUE1 => false,
            STBIENR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> STBIENR {
        match value {
            false => STBIENR::VALUE1,
            true => STBIENR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == STBIENR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == STBIENR::VALUE2
    }
}
#[doc = "Possible values of the field `TBERIEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TBERIENR {
    #[doc = "The transmit buffer error interrupt generation is disabled."]
    VALUE1,
    #[doc = "The transmit buffer error interrupt generation is enabled."]
    VALUE2,
}
impl TBERIENR {
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
            TBERIENR::VALUE1 => false,
            TBERIENR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TBERIENR {
        match value {
            false => TBERIENR::VALUE1,
            true => TBERIENR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == TBERIENR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == TBERIENR::VALUE2
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
#[doc = "Values that can be written to the field `STBTM`"]
pub enum STBTMW {
    #[doc = "Trigger mode 0: While TRBSR.STBT=1, a standard buffer event will be generated whenever there is a data transfer to TBUF or data write to INx (depending on TBCTR.LOF setting). STBT is cleared when TRBSR.TBFLVL=TBCTR.LIMIT."]
    VALUE1,
    #[doc = "Trigger mode 1: While TRBSR.STBT=1, a standard buffer event will be generated whenever there is a data transfer to TBUF or data write to INx (depending on TBCTR.LOF setting). STBT is cleared when TRBSR.TBFLVL=TBCTR.SIZE."]
    VALUE2,
}
impl STBTMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            STBTMW::VALUE1 => false,
            STBTMW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _STBTMW<'a> {
    w: &'a mut W,
}
impl<'a> _STBTMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: STBTMW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Trigger mode 0: While TRBSR.STBT=1, a standard buffer event will be generated whenever there is a data transfer to TBUF or data write to INx (depending on TBCTR.LOF setting). STBT is cleared when TRBSR.TBFLVL=TBCTR.LIMIT."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(STBTMW::VALUE1)
    }
    #[doc = "Trigger mode 1: While TRBSR.STBT=1, a standard buffer event will be generated whenever there is a data transfer to TBUF or data write to INx (depending on TBCTR.LOF setting). STBT is cleared when TRBSR.TBFLVL=TBCTR.SIZE."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(STBTMW::VALUE2)
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
#[doc = "Values that can be written to the field `STBTEN`"]
pub enum STBTENW {
    #[doc = "The standard transmit buffer event trigger through bit TRBSR.STBT is disabled."]
    VALUE1,
    #[doc = "The standard transmit buffer event trigger through bit TRBSR.STBT is enabled."]
    VALUE2,
}
impl STBTENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            STBTENW::VALUE1 => false,
            STBTENW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _STBTENW<'a> {
    w: &'a mut W,
}
impl<'a> _STBTENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: STBTENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The standard transmit buffer event trigger through bit TRBSR.STBT is disabled."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(STBTENW::VALUE1)
    }
    #[doc = "The standard transmit buffer event trigger through bit TRBSR.STBT is enabled."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(STBTENW::VALUE2)
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
#[doc = "Values that can be written to the field `STBINP`"]
pub enum STBINPW {
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
impl STBINPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            STBINPW::VALUE1 => 0,
            STBINPW::VALUE2 => 1,
            STBINPW::VALUE3 => 2,
            STBINPW::VALUE4 => 3,
            STBINPW::VALUE5 => 4,
            STBINPW::VALUE6 => 5,
        }
    }
}
#[doc = r" Proxy"]
pub struct _STBINPW<'a> {
    w: &'a mut W,
}
impl<'a> _STBINPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: STBINPW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Output SR0 becomes activated."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(STBINPW::VALUE1)
    }
    #[doc = "Output SR1 becomes activated."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(STBINPW::VALUE2)
    }
    #[doc = "Output SR2 becomes activated."]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(STBINPW::VALUE3)
    }
    #[doc = "Output SR3 becomes activated."]
    #[inline]
    pub fn value4(self) -> &'a mut W {
        self.variant(STBINPW::VALUE4)
    }
    #[doc = "Output SR4 becomes activated."]
    #[inline]
    pub fn value5(self) -> &'a mut W {
        self.variant(STBINPW::VALUE5)
    }
    #[doc = "Output SR5 becomes activated."]
    #[inline]
    pub fn value6(self) -> &'a mut W {
        self.variant(STBINPW::VALUE6)
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
#[doc = "Values that can be written to the field `ATBINP`"]
pub enum ATBINPW {
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
impl ATBINPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            ATBINPW::VALUE1 => 0,
            ATBINPW::VALUE2 => 1,
            ATBINPW::VALUE3 => 2,
            ATBINPW::VALUE4 => 3,
            ATBINPW::VALUE5 => 4,
            ATBINPW::VALUE6 => 5,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ATBINPW<'a> {
    w: &'a mut W,
}
impl<'a> _ATBINPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ATBINPW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Output SR0 becomes activated."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(ATBINPW::VALUE1)
    }
    #[doc = "Output SR1 becomes activated."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(ATBINPW::VALUE2)
    }
    #[doc = "Output SR2 becomes activated."]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(ATBINPW::VALUE3)
    }
    #[doc = "Output SR3 becomes activated."]
    #[inline]
    pub fn value4(self) -> &'a mut W {
        self.variant(ATBINPW::VALUE4)
    }
    #[doc = "Output SR4 becomes activated."]
    #[inline]
    pub fn value5(self) -> &'a mut W {
        self.variant(ATBINPW::VALUE5)
    }
    #[doc = "Output SR5 becomes activated."]
    #[inline]
    pub fn value6(self) -> &'a mut W {
        self.variant(ATBINPW::VALUE6)
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
#[doc = "Values that can be written to the field `LOF`"]
pub enum LOFW {
    #[doc = "A standard transmit buffer event occurs when the filling level equals the limit value and gets lower due to transmission of a data word."]
    VALUE1,
    #[doc = "A standard transmit buffer interrupt event occurs when the filling level equals the limit value and gets bigger due to a write access to a data input location INx."]
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
    #[doc = "A standard transmit buffer event occurs when the filling level equals the limit value and gets lower due to transmission of a data word."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(LOFW::VALUE1)
    }
    #[doc = "A standard transmit buffer interrupt event occurs when the filling level equals the limit value and gets bigger due to a write access to a data input location INx."]
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
#[doc = "Values that can be written to the field `STBIEN`"]
pub enum STBIENW {
    #[doc = "The standard transmit buffer interrupt generation is disabled."]
    VALUE1,
    #[doc = "The standard transmit buffer interrupt generation is enabled."]
    VALUE2,
}
impl STBIENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            STBIENW::VALUE1 => false,
            STBIENW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _STBIENW<'a> {
    w: &'a mut W,
}
impl<'a> _STBIENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: STBIENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The standard transmit buffer interrupt generation is disabled."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(STBIENW::VALUE1)
    }
    #[doc = "The standard transmit buffer interrupt generation is enabled."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(STBIENW::VALUE2)
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
#[doc = "Values that can be written to the field `TBERIEN`"]
pub enum TBERIENW {
    #[doc = "The transmit buffer error interrupt generation is disabled."]
    VALUE1,
    #[doc = "The transmit buffer error interrupt generation is enabled."]
    VALUE2,
}
impl TBERIENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TBERIENW::VALUE1 => false,
            TBERIENW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TBERIENW<'a> {
    w: &'a mut W,
}
impl<'a> _TBERIENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TBERIENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The transmit buffer error interrupt generation is disabled."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(TBERIENW::VALUE1)
    }
    #[doc = "The transmit buffer error interrupt generation is enabled."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(TBERIENW::VALUE2)
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
    #[doc = "Bit 14 - Standard Transmit Buffer Trigger Mode"]
    #[inline]
    pub fn stbtm(&self) -> STBTMR {
        STBTMR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 15 - Standard Transmit Buffer Trigger Enable"]
    #[inline]
    pub fn stbten(&self) -> STBTENR {
        STBTENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 16:18 - Standard Transmit Buffer Interrupt Node Pointer"]
    #[inline]
    pub fn stbinp(&self) -> STBINPR {
        STBINPR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 19:21 - Alternative Transmit Buffer Interrupt Node Pointer"]
    #[inline]
    pub fn atbinp(&self) -> ATBINPR {
        ATBINPR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 19;
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
    #[doc = "Bit 28 - Buffer Event on Limit Overflow"]
    #[inline]
    pub fn lof(&self) -> LOFR {
        LOFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 30 - Standard Transmit Buffer Interrupt Enable"]
    #[inline]
    pub fn stbien(&self) -> STBIENR {
        STBIENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 31 - Transmit Buffer Error Interrupt Enable"]
    #[inline]
    pub fn tberien(&self) -> TBERIENR {
        TBERIENR::_from({
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
    #[doc = "Bit 14 - Standard Transmit Buffer Trigger Mode"]
    #[inline]
    pub fn stbtm(&mut self) -> _STBTMW {
        _STBTMW { w: self }
    }
    #[doc = "Bit 15 - Standard Transmit Buffer Trigger Enable"]
    #[inline]
    pub fn stbten(&mut self) -> _STBTENW {
        _STBTENW { w: self }
    }
    #[doc = "Bits 16:18 - Standard Transmit Buffer Interrupt Node Pointer"]
    #[inline]
    pub fn stbinp(&mut self) -> _STBINPW {
        _STBINPW { w: self }
    }
    #[doc = "Bits 19:21 - Alternative Transmit Buffer Interrupt Node Pointer"]
    #[inline]
    pub fn atbinp(&mut self) -> _ATBINPW {
        _ATBINPW { w: self }
    }
    #[doc = "Bits 24:26 - Buffer Size"]
    #[inline]
    pub fn size(&mut self) -> _SIZEW {
        _SIZEW { w: self }
    }
    #[doc = "Bit 28 - Buffer Event on Limit Overflow"]
    #[inline]
    pub fn lof(&mut self) -> _LOFW {
        _LOFW { w: self }
    }
    #[doc = "Bit 30 - Standard Transmit Buffer Interrupt Enable"]
    #[inline]
    pub fn stbien(&mut self) -> _STBIENW {
        _STBIENW { w: self }
    }
    #[doc = "Bit 31 - Transmit Buffer Error Interrupt Enable"]
    #[inline]
    pub fn tberien(&mut self) -> _TBERIENW {
        _TBERIENW { w: self }
    }
}
