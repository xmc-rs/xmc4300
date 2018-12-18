#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CFG {
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
#[doc = "Possible values of the field `CMI`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CMIR {
    #[doc = "CRC Mismatch Interrupt is disabled"]
    VALUE1,
    #[doc = "CRC Mismatch Interrupt is enabled"]
    VALUE2,
}
impl CMIR {
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
            CMIR::VALUE1 => false,
            CMIR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CMIR {
        match value {
            false => CMIR::VALUE1,
            true => CMIR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == CMIR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == CMIR::VALUE2
    }
}
#[doc = "Possible values of the field `CEI`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CEIR {
    #[doc = "Configuration Error Interrupt is disabled"]
    VALUE1,
    #[doc = "Configuration Error Interrupt is enabled"]
    VALUE2,
}
impl CEIR {
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
            CEIR::VALUE1 => false,
            CEIR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CEIR {
        match value {
            false => CEIR::VALUE1,
            true => CEIR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == CEIR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == CEIR::VALUE2
    }
}
#[doc = "Possible values of the field `LEI`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LEIR {
    #[doc = "Length Error Interrupt is disabled"]
    VALUE1,
    #[doc = "Length Error Interrupt is enabled"]
    VALUE2,
}
impl LEIR {
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
            LEIR::VALUE1 => false,
            LEIR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LEIR {
        match value {
            false => LEIR::VALUE1,
            true => LEIR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == LEIR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == LEIR::VALUE2
    }
}
#[doc = "Possible values of the field `BEI`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BEIR {
    #[doc = "Bus Error Interrupt is disabled"]
    VALUE1,
    #[doc = "Bus Error Interrupt is enabled"]
    VALUE2,
}
impl BEIR {
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
            BEIR::VALUE1 => false,
            BEIR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BEIR {
        match value {
            false => BEIR::VALUE1,
            true => BEIR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == BEIR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == BEIR::VALUE2
    }
}
#[doc = "Possible values of the field `CCE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CCER {
    #[doc = "CRC check comparison at the end of a message is disabled"]
    VALUE1,
    #[doc = "CRC check comparison at the end of a message is enabled"]
    VALUE2,
}
impl CCER {
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
            CCER::VALUE1 => false,
            CCER::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CCER {
        match value {
            false => CCER::VALUE1,
            true => CCER::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == CCER::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == CCER::VALUE2
    }
}
#[doc = "Possible values of the field `ALR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ALRR {
    #[doc = "Disables automatic reload of the LENGTH field."]
    VALUE1,
    #[doc = "Enables automatic reload of the LENGTH field at the end of a message."]
    VALUE2,
}
impl ALRR {
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
            ALRR::VALUE1 => false,
            ALRR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ALRR {
        match value {
            false => ALRR::VALUE1,
            true => ALRR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == ALRR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == ALRR::VALUE2
    }
}
#[doc = "Possible values of the field `REFIN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REFINR {
    #[doc = "IR Byte Wise Reflection is disabled"]
    VALUE1,
    #[doc = "IR Byte Wise Reflection is enabled"]
    VALUE2,
}
impl REFINR {
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
            REFINR::VALUE1 => false,
            REFINR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> REFINR {
        match value {
            false => REFINR::VALUE1,
            true => REFINR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == REFINR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == REFINR::VALUE2
    }
}
#[doc = "Possible values of the field `REFOUT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REFOUTR {
    #[doc = "CRC 32-bit wise is disabled"]
    VALUE1,
    #[doc = "CRC 32-bit wise is enabled"]
    VALUE2,
}
impl REFOUTR {
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
            REFOUTR::VALUE1 => false,
            REFOUTR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> REFOUTR {
        match value {
            false => REFOUTR::VALUE1,
            true => REFOUTR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == REFOUTR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == REFOUTR::VALUE2
    }
}
#[doc = "Possible values of the field `XSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum XSELR {
    #[doc = "0x00000000"]
    VALUE1,
    #[doc = "0xFFFFFFFF"]
    VALUE2,
}
impl XSELR {
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
            XSELR::VALUE1 => false,
            XSELR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> XSELR {
        match value {
            false => XSELR::VALUE1,
            true => XSELR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == XSELR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == XSELR::VALUE2
    }
}
#[doc = "Values that can be written to the field `CMI`"]
pub enum CMIW {
    #[doc = "CRC Mismatch Interrupt is disabled"]
    VALUE1,
    #[doc = "CRC Mismatch Interrupt is enabled"]
    VALUE2,
}
impl CMIW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CMIW::VALUE1 => false,
            CMIW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CMIW<'a> {
    w: &'a mut W,
}
impl<'a> _CMIW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CMIW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "CRC Mismatch Interrupt is disabled"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(CMIW::VALUE1)
    }
    #[doc = "CRC Mismatch Interrupt is enabled"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(CMIW::VALUE2)
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
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CEI`"]
pub enum CEIW {
    #[doc = "Configuration Error Interrupt is disabled"]
    VALUE1,
    #[doc = "Configuration Error Interrupt is enabled"]
    VALUE2,
}
impl CEIW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CEIW::VALUE1 => false,
            CEIW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CEIW<'a> {
    w: &'a mut W,
}
impl<'a> _CEIW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CEIW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Configuration Error Interrupt is disabled"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(CEIW::VALUE1)
    }
    #[doc = "Configuration Error Interrupt is enabled"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(CEIW::VALUE2)
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
        const OFFSET: u8 = 1;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `LEI`"]
pub enum LEIW {
    #[doc = "Length Error Interrupt is disabled"]
    VALUE1,
    #[doc = "Length Error Interrupt is enabled"]
    VALUE2,
}
impl LEIW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LEIW::VALUE1 => false,
            LEIW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LEIW<'a> {
    w: &'a mut W,
}
impl<'a> _LEIW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LEIW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Length Error Interrupt is disabled"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(LEIW::VALUE1)
    }
    #[doc = "Length Error Interrupt is enabled"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(LEIW::VALUE2)
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
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `BEI`"]
pub enum BEIW {
    #[doc = "Bus Error Interrupt is disabled"]
    VALUE1,
    #[doc = "Bus Error Interrupt is enabled"]
    VALUE2,
}
impl BEIW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BEIW::VALUE1 => false,
            BEIW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BEIW<'a> {
    w: &'a mut W,
}
impl<'a> _BEIW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BEIW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Bus Error Interrupt is disabled"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(BEIW::VALUE1)
    }
    #[doc = "Bus Error Interrupt is enabled"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(BEIW::VALUE2)
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
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CCE`"]
pub enum CCEW {
    #[doc = "CRC check comparison at the end of a message is disabled"]
    VALUE1,
    #[doc = "CRC check comparison at the end of a message is enabled"]
    VALUE2,
}
impl CCEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CCEW::VALUE1 => false,
            CCEW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CCEW<'a> {
    w: &'a mut W,
}
impl<'a> _CCEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CCEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "CRC check comparison at the end of a message is disabled"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(CCEW::VALUE1)
    }
    #[doc = "CRC check comparison at the end of a message is enabled"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(CCEW::VALUE2)
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
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ALR`"]
pub enum ALRW {
    #[doc = "Disables automatic reload of the LENGTH field."]
    VALUE1,
    #[doc = "Enables automatic reload of the LENGTH field at the end of a message."]
    VALUE2,
}
impl ALRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ALRW::VALUE1 => false,
            ALRW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ALRW<'a> {
    w: &'a mut W,
}
impl<'a> _ALRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ALRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disables automatic reload of the LENGTH field."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(ALRW::VALUE1)
    }
    #[doc = "Enables automatic reload of the LENGTH field at the end of a message."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(ALRW::VALUE2)
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
        const OFFSET: u8 = 5;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `REFIN`"]
pub enum REFINW {
    #[doc = "IR Byte Wise Reflection is disabled"]
    VALUE1,
    #[doc = "IR Byte Wise Reflection is enabled"]
    VALUE2,
}
impl REFINW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            REFINW::VALUE1 => false,
            REFINW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _REFINW<'a> {
    w: &'a mut W,
}
impl<'a> _REFINW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: REFINW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "IR Byte Wise Reflection is disabled"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(REFINW::VALUE1)
    }
    #[doc = "IR Byte Wise Reflection is enabled"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(REFINW::VALUE2)
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
#[doc = "Values that can be written to the field `REFOUT`"]
pub enum REFOUTW {
    #[doc = "CRC 32-bit wise is disabled"]
    VALUE1,
    #[doc = "CRC 32-bit wise is enabled"]
    VALUE2,
}
impl REFOUTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            REFOUTW::VALUE1 => false,
            REFOUTW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _REFOUTW<'a> {
    w: &'a mut W,
}
impl<'a> _REFOUTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: REFOUTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "CRC 32-bit wise is disabled"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(REFOUTW::VALUE1)
    }
    #[doc = "CRC 32-bit wise is enabled"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(REFOUTW::VALUE2)
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
        const OFFSET: u8 = 9;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `XSEL`"]
pub enum XSELW {
    #[doc = "0x00000000"]
    VALUE1,
    #[doc = "0xFFFFFFFF"]
    VALUE2,
}
impl XSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            XSELW::VALUE1 => false,
            XSELW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _XSELW<'a> {
    w: &'a mut W,
}
impl<'a> _XSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: XSELW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "0x00000000"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(XSELW::VALUE1)
    }
    #[doc = "0xFFFFFFFF"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(XSELW::VALUE2)
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
        const OFFSET: u8 = 10;
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
    #[doc = "Bit 0 - CRC Mismatch Interrupt"]
    #[inline]
    pub fn cmi(&self) -> CMIR {
        CMIR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Configuration Error Interrupt"]
    #[inline]
    pub fn cei(&self) -> CEIR {
        CEIR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Length Error Interrupt"]
    #[inline]
    pub fn lei(&self) -> LEIR {
        LEIR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Bus Error Interrupt"]
    #[inline]
    pub fn bei(&self) -> BEIR {
        BEIR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - CRC Check Comparison"]
    #[inline]
    pub fn cce(&self) -> CCER {
        CCER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Automatic Length Reload"]
    #[inline]
    pub fn alr(&self) -> ALRR {
        ALRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - IR Byte Wise Reflection"]
    #[inline]
    pub fn refin(&self) -> REFINR {
        REFINR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - CRC 32-Bit Wise Reflection"]
    #[inline]
    pub fn refout(&self) -> REFOUTR {
        REFOUTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - Selects the value to be xored with the final CRC"]
    #[inline]
    pub fn xsel(&self) -> XSELR {
        XSELR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 1792 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - CRC Mismatch Interrupt"]
    #[inline]
    pub fn cmi(&mut self) -> _CMIW {
        _CMIW { w: self }
    }
    #[doc = "Bit 1 - Configuration Error Interrupt"]
    #[inline]
    pub fn cei(&mut self) -> _CEIW {
        _CEIW { w: self }
    }
    #[doc = "Bit 2 - Length Error Interrupt"]
    #[inline]
    pub fn lei(&mut self) -> _LEIW {
        _LEIW { w: self }
    }
    #[doc = "Bit 3 - Bus Error Interrupt"]
    #[inline]
    pub fn bei(&mut self) -> _BEIW {
        _BEIW { w: self }
    }
    #[doc = "Bit 4 - CRC Check Comparison"]
    #[inline]
    pub fn cce(&mut self) -> _CCEW {
        _CCEW { w: self }
    }
    #[doc = "Bit 5 - Automatic Length Reload"]
    #[inline]
    pub fn alr(&mut self) -> _ALRW {
        _ALRW { w: self }
    }
    #[doc = "Bit 8 - IR Byte Wise Reflection"]
    #[inline]
    pub fn refin(&mut self) -> _REFINW {
        _REFINW { w: self }
    }
    #[doc = "Bit 9 - CRC 32-Bit Wise Reflection"]
    #[inline]
    pub fn refout(&mut self) -> _REFOUTW {
        _REFOUTW { w: self }
    }
    #[doc = "Bit 10 - Selects the value to be xored with the final CRC"]
    #[inline]
    pub fn xsel(&mut self) -> _XSELW {
        _XSELW { w: self }
    }
}
