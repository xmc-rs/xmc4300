#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PCR_IICMODE {
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
pub struct SLADR {
    bits: u16,
}
impl SLADR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = "Possible values of the field `ACK00`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ACK00R {
    #[doc = "The slave device is not sensitive to this address."]
    VALUE1,
    #[doc = "The slave device is sensitive to this address."]
    VALUE2,
}
impl ACK00R {
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
            ACK00R::VALUE1 => false,
            ACK00R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ACK00R {
        match value {
            false => ACK00R::VALUE1,
            true => ACK00R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == ACK00R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == ACK00R::VALUE2
    }
}
#[doc = "Possible values of the field `STIM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STIMR {
    #[doc = "A symbol contains 10 time quanta. The timing is adapted for standard mode (100 kBaud)."]
    VALUE1,
    #[doc = "A symbol contains 25 time quanta. The timing is adapted for fast mode (400 kBaud)."]
    VALUE2,
}
impl STIMR {
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
            STIMR::VALUE1 => false,
            STIMR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> STIMR {
        match value {
            false => STIMR::VALUE1,
            true => STIMR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == STIMR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == STIMR::VALUE2
    }
}
#[doc = "Possible values of the field `SCRIEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SCRIENR {
    #[doc = "The start condition interrupt is disabled."]
    VALUE1,
    #[doc = "The start condition interrupt is enabled."]
    VALUE2,
}
impl SCRIENR {
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
            SCRIENR::VALUE1 => false,
            SCRIENR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SCRIENR {
        match value {
            false => SCRIENR::VALUE1,
            true => SCRIENR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == SCRIENR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == SCRIENR::VALUE2
    }
}
#[doc = "Possible values of the field `RSCRIEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RSCRIENR {
    #[doc = "The repeated start condition interrupt is disabled."]
    VALUE1,
    #[doc = "The repeated start condition interrupt is enabled."]
    VALUE2,
}
impl RSCRIENR {
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
            RSCRIENR::VALUE1 => false,
            RSCRIENR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RSCRIENR {
        match value {
            false => RSCRIENR::VALUE1,
            true => RSCRIENR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == RSCRIENR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == RSCRIENR::VALUE2
    }
}
#[doc = "Possible values of the field `PCRIEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PCRIENR {
    #[doc = "The stop condition interrupt is disabled."]
    VALUE1,
    #[doc = "The stop condition interrupt is enabled."]
    VALUE2,
}
impl PCRIENR {
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
            PCRIENR::VALUE1 => false,
            PCRIENR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PCRIENR {
        match value {
            false => PCRIENR::VALUE1,
            true => PCRIENR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == PCRIENR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == PCRIENR::VALUE2
    }
}
#[doc = "Possible values of the field `NACKIEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NACKIENR {
    #[doc = "The non-acknowledge interrupt is disabled."]
    VALUE1,
    #[doc = "The non-acknowledge interrupt is enabled."]
    VALUE2,
}
impl NACKIENR {
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
            NACKIENR::VALUE1 => false,
            NACKIENR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> NACKIENR {
        match value {
            false => NACKIENR::VALUE1,
            true => NACKIENR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == NACKIENR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == NACKIENR::VALUE2
    }
}
#[doc = "Possible values of the field `ARLIEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ARLIENR {
    #[doc = "The arbitration lost interrupt is disabled."]
    VALUE1,
    #[doc = "The arbitration lost interrupt is enabled."]
    VALUE2,
}
impl ARLIENR {
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
            ARLIENR::VALUE1 => false,
            ARLIENR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ARLIENR {
        match value {
            false => ARLIENR::VALUE1,
            true => ARLIENR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == ARLIENR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == ARLIENR::VALUE2
    }
}
#[doc = "Possible values of the field `SRRIEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRRIENR {
    #[doc = "The slave read request interrupt is disabled."]
    VALUE1,
    #[doc = "The slave read request interrupt is enabled."]
    VALUE2,
}
impl SRRIENR {
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
            SRRIENR::VALUE1 => false,
            SRRIENR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SRRIENR {
        match value {
            false => SRRIENR::VALUE1,
            true => SRRIENR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == SRRIENR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == SRRIENR::VALUE2
    }
}
#[doc = "Possible values of the field `ERRIEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERRIENR {
    #[doc = "The error interrupt is disabled."]
    VALUE1,
    #[doc = "The error interrupt is enabled."]
    VALUE2,
}
impl ERRIENR {
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
            ERRIENR::VALUE1 => false,
            ERRIENR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ERRIENR {
        match value {
            false => ERRIENR::VALUE1,
            true => ERRIENR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == ERRIENR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == ERRIENR::VALUE2
    }
}
#[doc = "Possible values of the field `SACKDIS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SACKDISR {
    #[doc = "The generation of an active slave acknowledge is enabled (slave acknowledge with 0 level = more bytes can be received)."]
    VALUE1,
    #[doc = "The generation of an active slave acknowledge is disabled (slave acknowledge with 1 level = reception stopped)."]
    VALUE2,
}
impl SACKDISR {
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
            SACKDISR::VALUE1 => false,
            SACKDISR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SACKDISR {
        match value {
            false => SACKDISR::VALUE1,
            true => SACKDISR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == SACKDISR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == SACKDISR::VALUE2
    }
}
#[doc = r" Value of the field"]
pub struct HDELR {
    bits: u8,
}
impl HDELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `ACKIEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ACKIENR {
    #[doc = "The acknowledge interrupt is disabled."]
    VALUE1,
    #[doc = "The acknowledge interrupt is enabled."]
    VALUE2,
}
impl ACKIENR {
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
            ACKIENR::VALUE1 => false,
            ACKIENR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ACKIENR {
        match value {
            false => ACKIENR::VALUE1,
            true => ACKIENR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == ACKIENR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == ACKIENR::VALUE2
    }
}
#[doc = "Possible values of the field `MCLK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MCLKR {
    #[doc = "The MCLK generation is disabled and MCLK is 0."]
    VALUE1,
    #[doc = "The MCLK generation is enabled."]
    VALUE2,
}
impl MCLKR {
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
            MCLKR::VALUE1 => false,
            MCLKR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MCLKR {
        match value {
            false => MCLKR::VALUE1,
            true => MCLKR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == MCLKR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == MCLKR::VALUE2
    }
}
#[doc = r" Proxy"]
pub struct _SLADW<'a> {
    w: &'a mut W,
}
impl<'a> _SLADW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        const MASK: u16 = 65535;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ACK00`"]
pub enum ACK00W {
    #[doc = "The slave device is not sensitive to this address."]
    VALUE1,
    #[doc = "The slave device is sensitive to this address."]
    VALUE2,
}
impl ACK00W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ACK00W::VALUE1 => false,
            ACK00W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ACK00W<'a> {
    w: &'a mut W,
}
impl<'a> _ACK00W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ACK00W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The slave device is not sensitive to this address."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(ACK00W::VALUE1)
    }
    #[doc = "The slave device is sensitive to this address."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(ACK00W::VALUE2)
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
#[doc = "Values that can be written to the field `STIM`"]
pub enum STIMW {
    #[doc = "A symbol contains 10 time quanta. The timing is adapted for standard mode (100 kBaud)."]
    VALUE1,
    #[doc = "A symbol contains 25 time quanta. The timing is adapted for fast mode (400 kBaud)."]
    VALUE2,
}
impl STIMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            STIMW::VALUE1 => false,
            STIMW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _STIMW<'a> {
    w: &'a mut W,
}
impl<'a> _STIMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: STIMW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "A symbol contains 10 time quanta. The timing is adapted for standard mode (100 kBaud)."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(STIMW::VALUE1)
    }
    #[doc = "A symbol contains 25 time quanta. The timing is adapted for fast mode (400 kBaud)."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(STIMW::VALUE2)
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
        const OFFSET: u8 = 17;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SCRIEN`"]
pub enum SCRIENW {
    #[doc = "The start condition interrupt is disabled."]
    VALUE1,
    #[doc = "The start condition interrupt is enabled."]
    VALUE2,
}
impl SCRIENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SCRIENW::VALUE1 => false,
            SCRIENW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SCRIENW<'a> {
    w: &'a mut W,
}
impl<'a> _SCRIENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SCRIENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The start condition interrupt is disabled."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(SCRIENW::VALUE1)
    }
    #[doc = "The start condition interrupt is enabled."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(SCRIENW::VALUE2)
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
        const OFFSET: u8 = 18;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RSCRIEN`"]
pub enum RSCRIENW {
    #[doc = "The repeated start condition interrupt is disabled."]
    VALUE1,
    #[doc = "The repeated start condition interrupt is enabled."]
    VALUE2,
}
impl RSCRIENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RSCRIENW::VALUE1 => false,
            RSCRIENW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RSCRIENW<'a> {
    w: &'a mut W,
}
impl<'a> _RSCRIENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RSCRIENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The repeated start condition interrupt is disabled."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(RSCRIENW::VALUE1)
    }
    #[doc = "The repeated start condition interrupt is enabled."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(RSCRIENW::VALUE2)
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
        const OFFSET: u8 = 19;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PCRIEN`"]
pub enum PCRIENW {
    #[doc = "The stop condition interrupt is disabled."]
    VALUE1,
    #[doc = "The stop condition interrupt is enabled."]
    VALUE2,
}
impl PCRIENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PCRIENW::VALUE1 => false,
            PCRIENW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PCRIENW<'a> {
    w: &'a mut W,
}
impl<'a> _PCRIENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PCRIENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The stop condition interrupt is disabled."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(PCRIENW::VALUE1)
    }
    #[doc = "The stop condition interrupt is enabled."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(PCRIENW::VALUE2)
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
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `NACKIEN`"]
pub enum NACKIENW {
    #[doc = "The non-acknowledge interrupt is disabled."]
    VALUE1,
    #[doc = "The non-acknowledge interrupt is enabled."]
    VALUE2,
}
impl NACKIENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            NACKIENW::VALUE1 => false,
            NACKIENW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _NACKIENW<'a> {
    w: &'a mut W,
}
impl<'a> _NACKIENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: NACKIENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The non-acknowledge interrupt is disabled."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(NACKIENW::VALUE1)
    }
    #[doc = "The non-acknowledge interrupt is enabled."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(NACKIENW::VALUE2)
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
        const OFFSET: u8 = 21;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ARLIEN`"]
pub enum ARLIENW {
    #[doc = "The arbitration lost interrupt is disabled."]
    VALUE1,
    #[doc = "The arbitration lost interrupt is enabled."]
    VALUE2,
}
impl ARLIENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ARLIENW::VALUE1 => false,
            ARLIENW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ARLIENW<'a> {
    w: &'a mut W,
}
impl<'a> _ARLIENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ARLIENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The arbitration lost interrupt is disabled."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(ARLIENW::VALUE1)
    }
    #[doc = "The arbitration lost interrupt is enabled."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(ARLIENW::VALUE2)
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
        const OFFSET: u8 = 22;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SRRIEN`"]
pub enum SRRIENW {
    #[doc = "The slave read request interrupt is disabled."]
    VALUE1,
    #[doc = "The slave read request interrupt is enabled."]
    VALUE2,
}
impl SRRIENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SRRIENW::VALUE1 => false,
            SRRIENW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SRRIENW<'a> {
    w: &'a mut W,
}
impl<'a> _SRRIENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SRRIENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The slave read request interrupt is disabled."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(SRRIENW::VALUE1)
    }
    #[doc = "The slave read request interrupt is enabled."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(SRRIENW::VALUE2)
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
#[doc = "Values that can be written to the field `ERRIEN`"]
pub enum ERRIENW {
    #[doc = "The error interrupt is disabled."]
    VALUE1,
    #[doc = "The error interrupt is enabled."]
    VALUE2,
}
impl ERRIENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ERRIENW::VALUE1 => false,
            ERRIENW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ERRIENW<'a> {
    w: &'a mut W,
}
impl<'a> _ERRIENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ERRIENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The error interrupt is disabled."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(ERRIENW::VALUE1)
    }
    #[doc = "The error interrupt is enabled."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(ERRIENW::VALUE2)
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
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SACKDIS`"]
pub enum SACKDISW {
    #[doc = "The generation of an active slave acknowledge is enabled (slave acknowledge with 0 level = more bytes can be received)."]
    VALUE1,
    #[doc = "The generation of an active slave acknowledge is disabled (slave acknowledge with 1 level = reception stopped)."]
    VALUE2,
}
impl SACKDISW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SACKDISW::VALUE1 => false,
            SACKDISW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SACKDISW<'a> {
    w: &'a mut W,
}
impl<'a> _SACKDISW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SACKDISW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The generation of an active slave acknowledge is enabled (slave acknowledge with 0 level = more bytes can be received)."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(SACKDISW::VALUE1)
    }
    #[doc = "The generation of an active slave acknowledge is disabled (slave acknowledge with 1 level = reception stopped)."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(SACKDISW::VALUE2)
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
        const OFFSET: u8 = 25;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _HDELW<'a> {
    w: &'a mut W,
}
impl<'a> _HDELW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 26;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ACKIEN`"]
pub enum ACKIENW {
    #[doc = "The acknowledge interrupt is disabled."]
    VALUE1,
    #[doc = "The acknowledge interrupt is enabled."]
    VALUE2,
}
impl ACKIENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ACKIENW::VALUE1 => false,
            ACKIENW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ACKIENW<'a> {
    w: &'a mut W,
}
impl<'a> _ACKIENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ACKIENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The acknowledge interrupt is disabled."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(ACKIENW::VALUE1)
    }
    #[doc = "The acknowledge interrupt is enabled."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(ACKIENW::VALUE2)
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
#[doc = "Values that can be written to the field `MCLK`"]
pub enum MCLKW {
    #[doc = "The MCLK generation is disabled and MCLK is 0."]
    VALUE1,
    #[doc = "The MCLK generation is enabled."]
    VALUE2,
}
impl MCLKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MCLKW::VALUE1 => false,
            MCLKW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MCLKW<'a> {
    w: &'a mut W,
}
impl<'a> _MCLKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MCLKW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The MCLK generation is disabled and MCLK is 0."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(MCLKW::VALUE1)
    }
    #[doc = "The MCLK generation is enabled."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(MCLKW::VALUE2)
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
    #[doc = "Bits 0:15 - Slave Address"]
    #[inline]
    pub fn slad(&self) -> SLADR {
        let bits = {
            const MASK: u16 = 65535;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        SLADR { bits }
    }
    #[doc = "Bit 16 - Acknowledge 00H"]
    #[inline]
    pub fn ack00(&self) -> ACK00R {
        ACK00R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 17 - Symbol Timing"]
    #[inline]
    pub fn stim(&self) -> STIMR {
        STIMR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 18 - Start Condition Received Interrupt Enable"]
    #[inline]
    pub fn scrien(&self) -> SCRIENR {
        SCRIENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 19 - Repeated Start Condition Received Interrupt Enable"]
    #[inline]
    pub fn rscrien(&self) -> RSCRIENR {
        RSCRIENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 20 - Stop Condition Received Interrupt Enable"]
    #[inline]
    pub fn pcrien(&self) -> PCRIENR {
        PCRIENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 21 - Non-Acknowledge Interrupt Enable"]
    #[inline]
    pub fn nackien(&self) -> NACKIENR {
        NACKIENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 22 - Arbitration Lost Interrupt Enable"]
    #[inline]
    pub fn arlien(&self) -> ARLIENR {
        ARLIENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 23 - Slave Read Request Interrupt Enable"]
    #[inline]
    pub fn srrien(&self) -> SRRIENR {
        SRRIENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 24 - Error Interrupt Enable"]
    #[inline]
    pub fn errien(&self) -> ERRIENR {
        ERRIENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 25 - Slave Acknowledge Disable"]
    #[inline]
    pub fn sackdis(&self) -> SACKDISR {
        SACKDISR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 26:29 - Hardware Delay"]
    #[inline]
    pub fn hdel(&self) -> HDELR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        HDELR { bits }
    }
    #[doc = "Bit 30 - Acknowledge Interrupt Enable"]
    #[inline]
    pub fn ackien(&self) -> ACKIENR {
        ACKIENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 31 - Master Clock Enable"]
    #[inline]
    pub fn mclk(&self) -> MCLKR {
        MCLKR::_from({
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
    #[doc = "Bits 0:15 - Slave Address"]
    #[inline]
    pub fn slad(&mut self) -> _SLADW {
        _SLADW { w: self }
    }
    #[doc = "Bit 16 - Acknowledge 00H"]
    #[inline]
    pub fn ack00(&mut self) -> _ACK00W {
        _ACK00W { w: self }
    }
    #[doc = "Bit 17 - Symbol Timing"]
    #[inline]
    pub fn stim(&mut self) -> _STIMW {
        _STIMW { w: self }
    }
    #[doc = "Bit 18 - Start Condition Received Interrupt Enable"]
    #[inline]
    pub fn scrien(&mut self) -> _SCRIENW {
        _SCRIENW { w: self }
    }
    #[doc = "Bit 19 - Repeated Start Condition Received Interrupt Enable"]
    #[inline]
    pub fn rscrien(&mut self) -> _RSCRIENW {
        _RSCRIENW { w: self }
    }
    #[doc = "Bit 20 - Stop Condition Received Interrupt Enable"]
    #[inline]
    pub fn pcrien(&mut self) -> _PCRIENW {
        _PCRIENW { w: self }
    }
    #[doc = "Bit 21 - Non-Acknowledge Interrupt Enable"]
    #[inline]
    pub fn nackien(&mut self) -> _NACKIENW {
        _NACKIENW { w: self }
    }
    #[doc = "Bit 22 - Arbitration Lost Interrupt Enable"]
    #[inline]
    pub fn arlien(&mut self) -> _ARLIENW {
        _ARLIENW { w: self }
    }
    #[doc = "Bit 23 - Slave Read Request Interrupt Enable"]
    #[inline]
    pub fn srrien(&mut self) -> _SRRIENW {
        _SRRIENW { w: self }
    }
    #[doc = "Bit 24 - Error Interrupt Enable"]
    #[inline]
    pub fn errien(&mut self) -> _ERRIENW {
        _ERRIENW { w: self }
    }
    #[doc = "Bit 25 - Slave Acknowledge Disable"]
    #[inline]
    pub fn sackdis(&mut self) -> _SACKDISW {
        _SACKDISW { w: self }
    }
    #[doc = "Bits 26:29 - Hardware Delay"]
    #[inline]
    pub fn hdel(&mut self) -> _HDELW {
        _HDELW { w: self }
    }
    #[doc = "Bit 30 - Acknowledge Interrupt Enable"]
    #[inline]
    pub fn ackien(&mut self) -> _ACKIENW {
        _ACKIENW { w: self }
    }
    #[doc = "Bit 31 - Master Clock Enable"]
    #[inline]
    pub fn mclk(&mut self) -> _MCLKW {
        _MCLKW { w: self }
    }
}
