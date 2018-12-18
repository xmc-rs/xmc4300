#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::GLOBTF {
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
pub struct CDGRR {
    bits: u8,
}
impl CDGRR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `CDEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CDENR {
    #[doc = "All diagnostic pull devices are disconnected"]
    VALUE1,
    #[doc = "Diagnostic pull devices connected as selected by bitfield CDSEL"]
    VALUE2,
}
impl CDENR {
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
            CDENR::VALUE1 => false,
            CDENR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CDENR {
        match value {
            false => CDENR::VALUE1,
            true => CDENR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == CDENR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == CDENR::VALUE2
    }
}
#[doc = "Possible values of the field `CDSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CDSELR {
    #[doc = "Connected to VAREF"]
    VALUE1,
    #[doc = "Connected to VAGND"]
    VALUE2,
    #[doc = "Connected to 1/3rd VAREF"]
    VALUE3,
    #[doc = "Connected to 2/3rd VAREF"]
    VALUE4,
}
impl CDSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CDSELR::VALUE1 => 0,
            CDSELR::VALUE2 => 1,
            CDSELR::VALUE3 => 2,
            CDSELR::VALUE4 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CDSELR {
        match value {
            0 => CDSELR::VALUE1,
            1 => CDSELR::VALUE2,
            2 => CDSELR::VALUE3,
            3 => CDSELR::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == CDSELR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == CDSELR::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == CDSELR::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == CDSELR::VALUE4
    }
}
#[doc = "Possible values of the field `PDD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDDR {
    #[doc = "Disconnected"]
    VALUE1,
    #[doc = "The pull-down diagnostics device is active"]
    VALUE2,
}
impl PDDR {
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
            PDDR::VALUE1 => false,
            PDDR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PDDR {
        match value {
            false => PDDR::VALUE1,
            true => PDDR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == PDDR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == PDDR::VALUE2
    }
}
#[doc = r" Proxy"]
pub struct _CDGRW<'a> {
    w: &'a mut W,
}
impl<'a> _CDGRW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CDEN`"]
pub enum CDENW {
    #[doc = "All diagnostic pull devices are disconnected"]
    VALUE1,
    #[doc = "Diagnostic pull devices connected as selected by bitfield CDSEL"]
    VALUE2,
}
impl CDENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CDENW::VALUE1 => false,
            CDENW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CDENW<'a> {
    w: &'a mut W,
}
impl<'a> _CDENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CDENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "All diagnostic pull devices are disconnected"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(CDENW::VALUE1)
    }
    #[doc = "Diagnostic pull devices connected as selected by bitfield CDSEL"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(CDENW::VALUE2)
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
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CDSEL`"]
pub enum CDSELW {
    #[doc = "Connected to VAREF"]
    VALUE1,
    #[doc = "Connected to VAGND"]
    VALUE2,
    #[doc = "Connected to 1/3rd VAREF"]
    VALUE3,
    #[doc = "Connected to 2/3rd VAREF"]
    VALUE4,
}
impl CDSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CDSELW::VALUE1 => 0,
            CDSELW::VALUE2 => 1,
            CDSELW::VALUE3 => 2,
            CDSELW::VALUE4 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CDSELW<'a> {
    w: &'a mut W,
}
impl<'a> _CDSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CDSELW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Connected to VAREF"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(CDSELW::VALUE1)
    }
    #[doc = "Connected to VAGND"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(CDSELW::VALUE2)
    }
    #[doc = "Connected to 1/3rd VAREF"]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(CDSELW::VALUE3)
    }
    #[doc = "Connected to 2/3rd VAREF"]
    #[inline]
    pub fn value4(self) -> &'a mut W {
        self.variant(CDSELW::VALUE4)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 9;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CDWC`"]
pub enum CDWCW {
    #[doc = "No write access to parameters"]
    VALUE1,
    #[doc = "Bitfields CDSEL, CDEN, CDGR can be written"]
    VALUE2,
}
impl CDWCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CDWCW::VALUE1 => false,
            CDWCW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CDWCW<'a> {
    w: &'a mut W,
}
impl<'a> _CDWCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CDWCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No write access to parameters"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(CDWCW::VALUE1)
    }
    #[doc = "Bitfields CDSEL, CDEN, CDGR can be written"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(CDWCW::VALUE2)
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
#[doc = "Values that can be written to the field `PDD`"]
pub enum PDDW {
    #[doc = "Disconnected"]
    VALUE1,
    #[doc = "The pull-down diagnostics device is active"]
    VALUE2,
}
impl PDDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PDDW::VALUE1 => false,
            PDDW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PDDW<'a> {
    w: &'a mut W,
}
impl<'a> _PDDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PDDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disconnected"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(PDDW::VALUE1)
    }
    #[doc = "The pull-down diagnostics device is active"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(PDDW::VALUE2)
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
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `MDWC`"]
pub enum MDWCW {
    #[doc = "No write access to parameters"]
    VALUE1,
    #[doc = "Bitfield PDD can be written"]
    VALUE2,
}
impl MDWCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MDWCW::VALUE1 => false,
            MDWCW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MDWCW<'a> {
    w: &'a mut W,
}
impl<'a> _MDWCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MDWCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No write access to parameters"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(MDWCW::VALUE1)
    }
    #[doc = "Bitfield PDD can be written"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(MDWCW::VALUE2)
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
        const OFFSET: u8 = 23;
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
    #[doc = "Bits 4:7 - Converter Diagnostics Group"]
    #[inline]
    pub fn cdgr(&self) -> CDGRR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CDGRR { bits }
    }
    #[doc = "Bit 8 - Converter Diagnostics Enable"]
    #[inline]
    pub fn cden(&self) -> CDENR {
        CDENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 9:10 - Converter Diagnostics Pull-Devices Select"]
    #[inline]
    pub fn cdsel(&self) -> CDSELR {
        CDSELR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 16 - Pull-Down Diagnostics Enable"]
    #[inline]
    pub fn pdd(&self) -> PDDR {
        PDDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
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
    #[doc = "Bits 4:7 - Converter Diagnostics Group"]
    #[inline]
    pub fn cdgr(&mut self) -> _CDGRW {
        _CDGRW { w: self }
    }
    #[doc = "Bit 8 - Converter Diagnostics Enable"]
    #[inline]
    pub fn cden(&mut self) -> _CDENW {
        _CDENW { w: self }
    }
    #[doc = "Bits 9:10 - Converter Diagnostics Pull-Devices Select"]
    #[inline]
    pub fn cdsel(&mut self) -> _CDSELW {
        _CDSELW { w: self }
    }
    #[doc = "Bit 15 - Write Control for Conversion Diagnostics"]
    #[inline]
    pub fn cdwc(&mut self) -> _CDWCW {
        _CDWCW { w: self }
    }
    #[doc = "Bit 16 - Pull-Down Diagnostics Enable"]
    #[inline]
    pub fn pdd(&mut self) -> _PDDW {
        _PDDW { w: self }
    }
    #[doc = "Bit 23 - Write Control for Multiplexer Diagnostics"]
    #[inline]
    pub fn mdwc(&mut self) -> _MDWCW {
        _MDWCW { w: self }
    }
}
