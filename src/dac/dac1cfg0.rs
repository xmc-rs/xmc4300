#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::DAC1CFG0 {
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
pub struct FREQR {
    bits: u32,
}
impl FREQR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
}
#[doc = "Possible values of the field `MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MODER {
    #[doc = "disable/switch-off DAC"]
    VALUE1,
    #[doc = "Single Value Mode"]
    VALUE2,
    #[doc = "Data Mode"]
    VALUE3,
    #[doc = "Patgen Mode"]
    VALUE4,
    #[doc = "Noise Mode"]
    VALUE5,
    #[doc = "Ramp Mode"]
    VALUE6,
    #[doc = "na"]
    VALUE7,
    #[doc = "na"]
    VALUE8,
}
impl MODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            MODER::VALUE1 => 0,
            MODER::VALUE2 => 1,
            MODER::VALUE3 => 2,
            MODER::VALUE4 => 3,
            MODER::VALUE5 => 4,
            MODER::VALUE6 => 5,
            MODER::VALUE7 => 6,
            MODER::VALUE8 => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> MODER {
        match value {
            0 => MODER::VALUE1,
            1 => MODER::VALUE2,
            2 => MODER::VALUE3,
            3 => MODER::VALUE4,
            4 => MODER::VALUE5,
            5 => MODER::VALUE6,
            6 => MODER::VALUE7,
            7 => MODER::VALUE8,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == MODER::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == MODER::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == MODER::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == MODER::VALUE4
    }
    #[doc = "Checks if the value of the field is `VALUE5`"]
    #[inline]
    pub fn is_value5(&self) -> bool {
        *self == MODER::VALUE5
    }
    #[doc = "Checks if the value of the field is `VALUE6`"]
    #[inline]
    pub fn is_value6(&self) -> bool {
        *self == MODER::VALUE6
    }
    #[doc = "Checks if the value of the field is `VALUE7`"]
    #[inline]
    pub fn is_value7(&self) -> bool {
        *self == MODER::VALUE7
    }
    #[doc = "Checks if the value of the field is `VALUE8`"]
    #[inline]
    pub fn is_value8(&self) -> bool {
        *self == MODER::VALUE8
    }
}
#[doc = "Possible values of the field `SIGN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SIGNR {
    #[doc = "DAC expects unsigned input data"]
    VALUE1,
    #[doc = "DAC expects signed input data"]
    VALUE2,
}
impl SIGNR {
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
            SIGNR::VALUE1 => false,
            SIGNR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SIGNR {
        match value {
            false => SIGNR::VALUE1,
            true => SIGNR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == SIGNR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == SIGNR::VALUE2
    }
}
#[doc = r" Value of the field"]
pub struct FIFOINDR {
    bits: u8,
}
impl FIFOINDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `FIFOEMP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FIFOEMPR {
    #[doc = "FIFO not empty"]
    VALUE1,
    #[doc = "FIFO empty"]
    VALUE2,
}
impl FIFOEMPR {
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
            FIFOEMPR::VALUE1 => false,
            FIFOEMPR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FIFOEMPR {
        match value {
            false => FIFOEMPR::VALUE1,
            true => FIFOEMPR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == FIFOEMPR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == FIFOEMPR::VALUE2
    }
}
#[doc = "Possible values of the field `FIFOFUL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FIFOFULR {
    #[doc = "FIFO not full"]
    VALUE1,
    #[doc = "FIFO full"]
    VALUE2,
}
impl FIFOFULR {
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
            FIFOFULR::VALUE1 => false,
            FIFOFULR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FIFOFULR {
        match value {
            false => FIFOFULR::VALUE1,
            true => FIFOFULR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == FIFOFULR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == FIFOFULR::VALUE2
    }
}
#[doc = "Possible values of the field `NEGATE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NEGATER {
    #[doc = "DAC output not negated"]
    VALUE1,
    #[doc = "DAC output negated"]
    VALUE2,
}
impl NEGATER {
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
            NEGATER::VALUE1 => false,
            NEGATER::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> NEGATER {
        match value {
            false => NEGATER::VALUE1,
            true => NEGATER::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == NEGATER::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == NEGATER::VALUE2
    }
}
#[doc = "Possible values of the field `SIGNEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SIGNENR {
    #[doc = "disable"]
    VALUE1,
    #[doc = "enable"]
    VALUE2,
}
impl SIGNENR {
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
            SIGNENR::VALUE1 => false,
            SIGNENR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SIGNENR {
        match value {
            false => SIGNENR::VALUE1,
            true => SIGNENR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == SIGNENR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == SIGNENR::VALUE2
    }
}
#[doc = "Possible values of the field `SREN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRENR {
    #[doc = "disable"]
    VALUE1,
    #[doc = "enable"]
    VALUE2,
}
impl SRENR {
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
            SRENR::VALUE1 => false,
            SRENR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SRENR {
        match value {
            false => SRENR::VALUE1,
            true => SRENR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == SRENR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == SRENR::VALUE2
    }
}
#[doc = "Possible values of the field `RUN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RUNR {
    #[doc = "DAC1 channel disabled"]
    VALUE1,
    #[doc = "DAC1 channel in operation"]
    VALUE2,
}
impl RUNR {
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
            RUNR::VALUE1 => false,
            RUNR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RUNR {
        match value {
            false => RUNR::VALUE1,
            true => RUNR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == RUNR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == RUNR::VALUE2
    }
}
#[doc = r" Proxy"]
pub struct _FREQW<'a> {
    w: &'a mut W,
}
impl<'a> _FREQW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        const MASK: u32 = 1048575;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `MODE`"]
pub enum MODEW {
    #[doc = "disable/switch-off DAC"]
    VALUE1,
    #[doc = "Single Value Mode"]
    VALUE2,
    #[doc = "Data Mode"]
    VALUE3,
    #[doc = "Patgen Mode"]
    VALUE4,
    #[doc = "Noise Mode"]
    VALUE5,
    #[doc = "Ramp Mode"]
    VALUE6,
    #[doc = "na"]
    VALUE7,
    #[doc = "na"]
    VALUE8,
}
impl MODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            MODEW::VALUE1 => 0,
            MODEW::VALUE2 => 1,
            MODEW::VALUE3 => 2,
            MODEW::VALUE4 => 3,
            MODEW::VALUE5 => 4,
            MODEW::VALUE6 => 5,
            MODEW::VALUE7 => 6,
            MODEW::VALUE8 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _MODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MODEW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "disable/switch-off DAC"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(MODEW::VALUE1)
    }
    #[doc = "Single Value Mode"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(MODEW::VALUE2)
    }
    #[doc = "Data Mode"]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(MODEW::VALUE3)
    }
    #[doc = "Patgen Mode"]
    #[inline]
    pub fn value4(self) -> &'a mut W {
        self.variant(MODEW::VALUE4)
    }
    #[doc = "Noise Mode"]
    #[inline]
    pub fn value5(self) -> &'a mut W {
        self.variant(MODEW::VALUE5)
    }
    #[doc = "Ramp Mode"]
    #[inline]
    pub fn value6(self) -> &'a mut W {
        self.variant(MODEW::VALUE6)
    }
    #[doc = "na"]
    #[inline]
    pub fn value7(self) -> &'a mut W {
        self.variant(MODEW::VALUE7)
    }
    #[doc = "na"]
    #[inline]
    pub fn value8(self) -> &'a mut W {
        self.variant(MODEW::VALUE8)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SIGN`"]
pub enum SIGNW {
    #[doc = "DAC expects unsigned input data"]
    VALUE1,
    #[doc = "DAC expects signed input data"]
    VALUE2,
}
impl SIGNW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SIGNW::VALUE1 => false,
            SIGNW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SIGNW<'a> {
    w: &'a mut W,
}
impl<'a> _SIGNW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SIGNW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "DAC expects unsigned input data"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(SIGNW::VALUE1)
    }
    #[doc = "DAC expects signed input data"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(SIGNW::VALUE2)
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
#[doc = "Values that can be written to the field `NEGATE`"]
pub enum NEGATEW {
    #[doc = "DAC output not negated"]
    VALUE1,
    #[doc = "DAC output negated"]
    VALUE2,
}
impl NEGATEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            NEGATEW::VALUE1 => false,
            NEGATEW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _NEGATEW<'a> {
    w: &'a mut W,
}
impl<'a> _NEGATEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: NEGATEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "DAC output not negated"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(NEGATEW::VALUE1)
    }
    #[doc = "DAC output negated"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(NEGATEW::VALUE2)
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
#[doc = "Values that can be written to the field `SIGNEN`"]
pub enum SIGNENW {
    #[doc = "disable"]
    VALUE1,
    #[doc = "enable"]
    VALUE2,
}
impl SIGNENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SIGNENW::VALUE1 => false,
            SIGNENW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SIGNENW<'a> {
    w: &'a mut W,
}
impl<'a> _SIGNENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SIGNENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "disable"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(SIGNENW::VALUE1)
    }
    #[doc = "enable"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(SIGNENW::VALUE2)
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
#[doc = "Values that can be written to the field `SREN`"]
pub enum SRENW {
    #[doc = "disable"]
    VALUE1,
    #[doc = "enable"]
    VALUE2,
}
impl SRENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SRENW::VALUE1 => false,
            SRENW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SRENW<'a> {
    w: &'a mut W,
}
impl<'a> _SRENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SRENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "disable"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(SRENW::VALUE1)
    }
    #[doc = "enable"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(SRENW::VALUE2)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:19 - Integer Frequency Divider Value"]
    #[inline]
    pub fn freq(&self) -> FREQR {
        let bits = {
            const MASK: u32 = 1048575;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u32
        };
        FREQR { bits }
    }
    #[doc = "Bits 20:22 - Enables and sets the Mode for DAC1"]
    #[inline]
    pub fn mode(&self) -> MODER {
        MODER::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 23 - Selects between signed and unsigned DAC1 mode"]
    #[inline]
    pub fn sign(&self) -> SIGNR {
        SIGNR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 24:25 - Current write position inside the data FIFO"]
    #[inline]
    pub fn fifoind(&self) -> FIFOINDR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        FIFOINDR { bits }
    }
    #[doc = "Bit 26 - Indicate if the FIFO is empty"]
    #[inline]
    pub fn fifoemp(&self) -> FIFOEMPR {
        FIFOEMPR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 27 - Indicate if the FIFO is full"]
    #[inline]
    pub fn fifoful(&self) -> FIFOFULR {
        FIFOFULR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 28 - Negates the DAC1 output"]
    #[inline]
    pub fn negate(&self) -> NEGATER {
        NEGATER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 29 - Enable sign output of DAC1 pattern generator"]
    #[inline]
    pub fn signen(&self) -> SIGNENR {
        SIGNENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 29;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 30 - Enable DAC1 service request interrupt generation"]
    #[inline]
    pub fn sren(&self) -> SRENR {
        SRENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 31 - RUN indicates the current DAC1 operation status"]
    #[inline]
    pub fn run(&self) -> RUNR {
        RUNR::_from({
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
    #[doc = "Bits 0:19 - Integer Frequency Divider Value"]
    #[inline]
    pub fn freq(&mut self) -> _FREQW {
        _FREQW { w: self }
    }
    #[doc = "Bits 20:22 - Enables and sets the Mode for DAC1"]
    #[inline]
    pub fn mode(&mut self) -> _MODEW {
        _MODEW { w: self }
    }
    #[doc = "Bit 23 - Selects between signed and unsigned DAC1 mode"]
    #[inline]
    pub fn sign(&mut self) -> _SIGNW {
        _SIGNW { w: self }
    }
    #[doc = "Bit 28 - Negates the DAC1 output"]
    #[inline]
    pub fn negate(&mut self) -> _NEGATEW {
        _NEGATEW { w: self }
    }
    #[doc = "Bit 29 - Enable sign output of DAC1 pattern generator"]
    #[inline]
    pub fn signen(&mut self) -> _SIGNENW {
        _SIGNENW { w: self }
    }
    #[doc = "Bit 30 - Enable DAC1 service request interrupt generation"]
    #[inline]
    pub fn sren(&mut self) -> _SRENW {
        _SRENW { w: self }
    }
}
