#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PEEN {
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
#[doc = "Possible values of the field `PEENPS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PEENPSR {
    #[doc = "Disabled"]
    CONST_0,
    #[doc = "Enabled"]
    CONST_1,
}
impl PEENPSR {
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
            PEENPSR::CONST_0 => false,
            PEENPSR::CONST_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PEENPSR {
        match value {
            false => PEENPSR::CONST_0,
            true => PEENPSR::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline]
    pub fn is_const_0(&self) -> bool {
        *self == PEENPSR::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline]
    pub fn is_const_1(&self) -> bool {
        *self == PEENPSR::CONST_1
    }
}
#[doc = "Possible values of the field `PEENDS1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PEENDS1R {
    #[doc = "Disabled"]
    CONST_0,
    #[doc = "Enabled"]
    CONST_1,
}
impl PEENDS1R {
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
            PEENDS1R::CONST_0 => false,
            PEENDS1R::CONST_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PEENDS1R {
        match value {
            false => PEENDS1R::CONST_0,
            true => PEENDS1R::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline]
    pub fn is_const_0(&self) -> bool {
        *self == PEENDS1R::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline]
    pub fn is_const_1(&self) -> bool {
        *self == PEENDS1R::CONST_1
    }
}
#[doc = "Possible values of the field `PEENU0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PEENU0R {
    #[doc = "Disabled"]
    CONST_0,
    #[doc = "Enabled"]
    CONST_1,
}
impl PEENU0R {
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
            PEENU0R::CONST_0 => false,
            PEENU0R::CONST_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PEENU0R {
        match value {
            false => PEENU0R::CONST_0,
            true => PEENU0R::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline]
    pub fn is_const_0(&self) -> bool {
        *self == PEENU0R::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline]
    pub fn is_const_1(&self) -> bool {
        *self == PEENU0R::CONST_1
    }
}
#[doc = "Possible values of the field `PEENU1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PEENU1R {
    #[doc = "Disabled"]
    CONST_0,
    #[doc = "Enabled"]
    CONST_1,
}
impl PEENU1R {
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
            PEENU1R::CONST_0 => false,
            PEENU1R::CONST_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PEENU1R {
        match value {
            false => PEENU1R::CONST_0,
            true => PEENU1R::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline]
    pub fn is_const_0(&self) -> bool {
        *self == PEENU1R::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline]
    pub fn is_const_1(&self) -> bool {
        *self == PEENU1R::CONST_1
    }
}
#[doc = "Possible values of the field `PEENMC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PEENMCR {
    #[doc = "Disabled"]
    CONST_0,
    #[doc = "Enabled"]
    CONST_1,
}
impl PEENMCR {
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
            PEENMCR::CONST_0 => false,
            PEENMCR::CONST_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PEENMCR {
        match value {
            false => PEENMCR::CONST_0,
            true => PEENMCR::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline]
    pub fn is_const_0(&self) -> bool {
        *self == PEENMCR::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline]
    pub fn is_const_1(&self) -> bool {
        *self == PEENMCR::CONST_1
    }
}
#[doc = "Possible values of the field `PEENPPRF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PEENPPRFR {
    #[doc = "Disabled"]
    CONST_0,
    #[doc = "Enabled"]
    CONST_1,
}
impl PEENPPRFR {
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
            PEENPPRFR::CONST_0 => false,
            PEENPPRFR::CONST_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PEENPPRFR {
        match value {
            false => PEENPPRFR::CONST_0,
            true => PEENPPRFR::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline]
    pub fn is_const_0(&self) -> bool {
        *self == PEENPPRFR::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline]
    pub fn is_const_1(&self) -> bool {
        *self == PEENPPRFR::CONST_1
    }
}
#[doc = "Possible values of the field `PEENUSB`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PEENUSBR {
    #[doc = "Disabled"]
    CONST_0,
    #[doc = "Enabled"]
    CONST_1,
}
impl PEENUSBR {
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
            PEENUSBR::CONST_0 => false,
            PEENUSBR::CONST_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PEENUSBR {
        match value {
            false => PEENUSBR::CONST_0,
            true => PEENUSBR::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline]
    pub fn is_const_0(&self) -> bool {
        *self == PEENUSBR::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline]
    pub fn is_const_1(&self) -> bool {
        *self == PEENUSBR::CONST_1
    }
}
#[doc = "Possible values of the field `PEENETH0TX`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PEENETH0TXR {
    #[doc = "Disabled"]
    CONST_0,
    #[doc = "Enabled"]
    CONST_1,
}
impl PEENETH0TXR {
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
            PEENETH0TXR::CONST_0 => false,
            PEENETH0TXR::CONST_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PEENETH0TXR {
        match value {
            false => PEENETH0TXR::CONST_0,
            true => PEENETH0TXR::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline]
    pub fn is_const_0(&self) -> bool {
        *self == PEENETH0TXR::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline]
    pub fn is_const_1(&self) -> bool {
        *self == PEENETH0TXR::CONST_1
    }
}
#[doc = "Possible values of the field `PEENETH0RX`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PEENETH0RXR {
    #[doc = "Disabled"]
    CONST_0,
    #[doc = "Enabled"]
    CONST_1,
}
impl PEENETH0RXR {
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
            PEENETH0RXR::CONST_0 => false,
            PEENETH0RXR::CONST_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PEENETH0RXR {
        match value {
            false => PEENETH0RXR::CONST_0,
            true => PEENETH0RXR::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline]
    pub fn is_const_0(&self) -> bool {
        *self == PEENETH0RXR::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline]
    pub fn is_const_1(&self) -> bool {
        *self == PEENETH0RXR::CONST_1
    }
}
#[doc = "Possible values of the field `PEENSD0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PEENSD0R {
    #[doc = "Disabled"]
    CONST_0,
    #[doc = "Enabled"]
    CONST_1,
}
impl PEENSD0R {
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
            PEENSD0R::CONST_0 => false,
            PEENSD0R::CONST_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PEENSD0R {
        match value {
            false => PEENSD0R::CONST_0,
            true => PEENSD0R::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline]
    pub fn is_const_0(&self) -> bool {
        *self == PEENSD0R::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline]
    pub fn is_const_1(&self) -> bool {
        *self == PEENSD0R::CONST_1
    }
}
#[doc = "Possible values of the field `PEENSD1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PEENSD1R {
    #[doc = "Disabled"]
    CONST_0,
    #[doc = "Enabled"]
    CONST_1,
}
impl PEENSD1R {
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
            PEENSD1R::CONST_0 => false,
            PEENSD1R::CONST_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PEENSD1R {
        match value {
            false => PEENSD1R::CONST_0,
            true => PEENSD1R::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline]
    pub fn is_const_0(&self) -> bool {
        *self == PEENSD1R::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline]
    pub fn is_const_1(&self) -> bool {
        *self == PEENSD1R::CONST_1
    }
}
#[doc = "Possible values of the field `PEENECAT0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PEENECAT0R {
    #[doc = "Disabled"]
    CONST_0,
    #[doc = "Enabled"]
    CONST_1,
}
impl PEENECAT0R {
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
            PEENECAT0R::CONST_0 => false,
            PEENECAT0R::CONST_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PEENECAT0R {
        match value {
            false => PEENECAT0R::CONST_0,
            true => PEENECAT0R::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline]
    pub fn is_const_0(&self) -> bool {
        *self == PEENECAT0R::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline]
    pub fn is_const_1(&self) -> bool {
        *self == PEENECAT0R::CONST_1
    }
}
#[doc = "Values that can be written to the field `PEENPS`"]
pub enum PEENPSW {
    #[doc = "Disabled"]
    CONST_0,
    #[doc = "Enabled"]
    CONST_1,
}
impl PEENPSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PEENPSW::CONST_0 => false,
            PEENPSW::CONST_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PEENPSW<'a> {
    w: &'a mut W,
}
impl<'a> _PEENPSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PEENPSW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled"]
    #[inline]
    pub fn const_0(self) -> &'a mut W {
        self.variant(PEENPSW::CONST_0)
    }
    #[doc = "Enabled"]
    #[inline]
    pub fn const_1(self) -> &'a mut W {
        self.variant(PEENPSW::CONST_1)
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
#[doc = "Values that can be written to the field `PEENDS1`"]
pub enum PEENDS1W {
    #[doc = "Disabled"]
    CONST_0,
    #[doc = "Enabled"]
    CONST_1,
}
impl PEENDS1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PEENDS1W::CONST_0 => false,
            PEENDS1W::CONST_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PEENDS1W<'a> {
    w: &'a mut W,
}
impl<'a> _PEENDS1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PEENDS1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled"]
    #[inline]
    pub fn const_0(self) -> &'a mut W {
        self.variant(PEENDS1W::CONST_0)
    }
    #[doc = "Enabled"]
    #[inline]
    pub fn const_1(self) -> &'a mut W {
        self.variant(PEENDS1W::CONST_1)
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
#[doc = "Values that can be written to the field `PEENU0`"]
pub enum PEENU0W {
    #[doc = "Disabled"]
    CONST_0,
    #[doc = "Enabled"]
    CONST_1,
}
impl PEENU0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PEENU0W::CONST_0 => false,
            PEENU0W::CONST_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PEENU0W<'a> {
    w: &'a mut W,
}
impl<'a> _PEENU0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PEENU0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled"]
    #[inline]
    pub fn const_0(self) -> &'a mut W {
        self.variant(PEENU0W::CONST_0)
    }
    #[doc = "Enabled"]
    #[inline]
    pub fn const_1(self) -> &'a mut W {
        self.variant(PEENU0W::CONST_1)
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
#[doc = "Values that can be written to the field `PEENU1`"]
pub enum PEENU1W {
    #[doc = "Disabled"]
    CONST_0,
    #[doc = "Enabled"]
    CONST_1,
}
impl PEENU1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PEENU1W::CONST_0 => false,
            PEENU1W::CONST_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PEENU1W<'a> {
    w: &'a mut W,
}
impl<'a> _PEENU1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PEENU1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled"]
    #[inline]
    pub fn const_0(self) -> &'a mut W {
        self.variant(PEENU1W::CONST_0)
    }
    #[doc = "Enabled"]
    #[inline]
    pub fn const_1(self) -> &'a mut W {
        self.variant(PEENU1W::CONST_1)
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
#[doc = "Values that can be written to the field `PEENMC`"]
pub enum PEENMCW {
    #[doc = "Disabled"]
    CONST_0,
    #[doc = "Enabled"]
    CONST_1,
}
impl PEENMCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PEENMCW::CONST_0 => false,
            PEENMCW::CONST_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PEENMCW<'a> {
    w: &'a mut W,
}
impl<'a> _PEENMCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PEENMCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled"]
    #[inline]
    pub fn const_0(self) -> &'a mut W {
        self.variant(PEENMCW::CONST_0)
    }
    #[doc = "Enabled"]
    #[inline]
    pub fn const_1(self) -> &'a mut W {
        self.variant(PEENMCW::CONST_1)
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
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PEENPPRF`"]
pub enum PEENPPRFW {
    #[doc = "Disabled"]
    CONST_0,
    #[doc = "Enabled"]
    CONST_1,
}
impl PEENPPRFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PEENPPRFW::CONST_0 => false,
            PEENPPRFW::CONST_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PEENPPRFW<'a> {
    w: &'a mut W,
}
impl<'a> _PEENPPRFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PEENPPRFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled"]
    #[inline]
    pub fn const_0(self) -> &'a mut W {
        self.variant(PEENPPRFW::CONST_0)
    }
    #[doc = "Enabled"]
    #[inline]
    pub fn const_1(self) -> &'a mut W {
        self.variant(PEENPPRFW::CONST_1)
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
        const OFFSET: u8 = 13;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PEENUSB`"]
pub enum PEENUSBW {
    #[doc = "Disabled"]
    CONST_0,
    #[doc = "Enabled"]
    CONST_1,
}
impl PEENUSBW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PEENUSBW::CONST_0 => false,
            PEENUSBW::CONST_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PEENUSBW<'a> {
    w: &'a mut W,
}
impl<'a> _PEENUSBW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PEENUSBW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled"]
    #[inline]
    pub fn const_0(self) -> &'a mut W {
        self.variant(PEENUSBW::CONST_0)
    }
    #[doc = "Enabled"]
    #[inline]
    pub fn const_1(self) -> &'a mut W {
        self.variant(PEENUSBW::CONST_1)
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
#[doc = "Values that can be written to the field `PEENETH0TX`"]
pub enum PEENETH0TXW {
    #[doc = "Disabled"]
    CONST_0,
    #[doc = "Enabled"]
    CONST_1,
}
impl PEENETH0TXW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PEENETH0TXW::CONST_0 => false,
            PEENETH0TXW::CONST_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PEENETH0TXW<'a> {
    w: &'a mut W,
}
impl<'a> _PEENETH0TXW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PEENETH0TXW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled"]
    #[inline]
    pub fn const_0(self) -> &'a mut W {
        self.variant(PEENETH0TXW::CONST_0)
    }
    #[doc = "Enabled"]
    #[inline]
    pub fn const_1(self) -> &'a mut W {
        self.variant(PEENETH0TXW::CONST_1)
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
#[doc = "Values that can be written to the field `PEENETH0RX`"]
pub enum PEENETH0RXW {
    #[doc = "Disabled"]
    CONST_0,
    #[doc = "Enabled"]
    CONST_1,
}
impl PEENETH0RXW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PEENETH0RXW::CONST_0 => false,
            PEENETH0RXW::CONST_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PEENETH0RXW<'a> {
    w: &'a mut W,
}
impl<'a> _PEENETH0RXW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PEENETH0RXW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled"]
    #[inline]
    pub fn const_0(self) -> &'a mut W {
        self.variant(PEENETH0RXW::CONST_0)
    }
    #[doc = "Enabled"]
    #[inline]
    pub fn const_1(self) -> &'a mut W {
        self.variant(PEENETH0RXW::CONST_1)
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
#[doc = "Values that can be written to the field `PEENSD0`"]
pub enum PEENSD0W {
    #[doc = "Disabled"]
    CONST_0,
    #[doc = "Enabled"]
    CONST_1,
}
impl PEENSD0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PEENSD0W::CONST_0 => false,
            PEENSD0W::CONST_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PEENSD0W<'a> {
    w: &'a mut W,
}
impl<'a> _PEENSD0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PEENSD0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled"]
    #[inline]
    pub fn const_0(self) -> &'a mut W {
        self.variant(PEENSD0W::CONST_0)
    }
    #[doc = "Enabled"]
    #[inline]
    pub fn const_1(self) -> &'a mut W {
        self.variant(PEENSD0W::CONST_1)
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
#[doc = "Values that can be written to the field `PEENSD1`"]
pub enum PEENSD1W {
    #[doc = "Disabled"]
    CONST_0,
    #[doc = "Enabled"]
    CONST_1,
}
impl PEENSD1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PEENSD1W::CONST_0 => false,
            PEENSD1W::CONST_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PEENSD1W<'a> {
    w: &'a mut W,
}
impl<'a> _PEENSD1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PEENSD1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled"]
    #[inline]
    pub fn const_0(self) -> &'a mut W {
        self.variant(PEENSD1W::CONST_0)
    }
    #[doc = "Enabled"]
    #[inline]
    pub fn const_1(self) -> &'a mut W {
        self.variant(PEENSD1W::CONST_1)
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
#[doc = "Values that can be written to the field `PEENECAT0`"]
pub enum PEENECAT0W {
    #[doc = "Disabled"]
    CONST_0,
    #[doc = "Enabled"]
    CONST_1,
}
impl PEENECAT0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PEENECAT0W::CONST_0 => false,
            PEENECAT0W::CONST_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PEENECAT0W<'a> {
    w: &'a mut W,
}
impl<'a> _PEENECAT0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PEENECAT0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled"]
    #[inline]
    pub fn const_0(self) -> &'a mut W {
        self.variant(PEENECAT0W::CONST_0)
    }
    #[doc = "Enabled"]
    #[inline]
    pub fn const_1(self) -> &'a mut W {
        self.variant(PEENECAT0W::CONST_1)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Parity Error Enable for PSRAM"]
    #[inline]
    pub fn peenps(&self) -> PEENPSR {
        PEENPSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Parity Error Enable for DSRAM1"]
    #[inline]
    pub fn peends1(&self) -> PEENDS1R {
        PEENDS1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Parity Error Enable for USIC0 Memory"]
    #[inline]
    pub fn peenu0(&self) -> PEENU0R {
        PEENU0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - Parity Error Enable for USIC1 Memory"]
    #[inline]
    pub fn peenu1(&self) -> PEENU1R {
        PEENU1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 12 - Parity Error Enable for MultiCAN Memory"]
    #[inline]
    pub fn peenmc(&self) -> PEENMCR {
        PEENMCR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 13 - Parity Error Enable for PMU Prefetch Memory"]
    #[inline]
    pub fn peenpprf(&self) -> PEENPPRFR {
        PEENPPRFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 16 - Parity Error Enable for USB Memory"]
    #[inline]
    pub fn peenusb(&self) -> PEENUSBR {
        PEENUSBR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 17 - Parity Error Enable for ETH TX Memory"]
    #[inline]
    pub fn peeneth0tx(&self) -> PEENETH0TXR {
        PEENETH0TXR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 18 - Parity Error Enable for ETH RX Memory"]
    #[inline]
    pub fn peeneth0rx(&self) -> PEENETH0RXR {
        PEENETH0RXR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 19 - Parity Error Enable for SDMMC Memory 0"]
    #[inline]
    pub fn peensd0(&self) -> PEENSD0R {
        PEENSD0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 20 - Parity Error Enable for SDMMC Memory 1"]
    #[inline]
    pub fn peensd1(&self) -> PEENSD1R {
        PEENSD1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 24 - Parity Error Enable for ECAT0 Memory"]
    #[inline]
    pub fn peenecat0(&self) -> PEENECAT0R {
        PEENECAT0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 24;
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
    #[doc = "Bit 0 - Parity Error Enable for PSRAM"]
    #[inline]
    pub fn peenps(&mut self) -> _PEENPSW {
        _PEENPSW { w: self }
    }
    #[doc = "Bit 1 - Parity Error Enable for DSRAM1"]
    #[inline]
    pub fn peends1(&mut self) -> _PEENDS1W {
        _PEENDS1W { w: self }
    }
    #[doc = "Bit 8 - Parity Error Enable for USIC0 Memory"]
    #[inline]
    pub fn peenu0(&mut self) -> _PEENU0W {
        _PEENU0W { w: self }
    }
    #[doc = "Bit 9 - Parity Error Enable for USIC1 Memory"]
    #[inline]
    pub fn peenu1(&mut self) -> _PEENU1W {
        _PEENU1W { w: self }
    }
    #[doc = "Bit 12 - Parity Error Enable for MultiCAN Memory"]
    #[inline]
    pub fn peenmc(&mut self) -> _PEENMCW {
        _PEENMCW { w: self }
    }
    #[doc = "Bit 13 - Parity Error Enable for PMU Prefetch Memory"]
    #[inline]
    pub fn peenpprf(&mut self) -> _PEENPPRFW {
        _PEENPPRFW { w: self }
    }
    #[doc = "Bit 16 - Parity Error Enable for USB Memory"]
    #[inline]
    pub fn peenusb(&mut self) -> _PEENUSBW {
        _PEENUSBW { w: self }
    }
    #[doc = "Bit 17 - Parity Error Enable for ETH TX Memory"]
    #[inline]
    pub fn peeneth0tx(&mut self) -> _PEENETH0TXW {
        _PEENETH0TXW { w: self }
    }
    #[doc = "Bit 18 - Parity Error Enable for ETH RX Memory"]
    #[inline]
    pub fn peeneth0rx(&mut self) -> _PEENETH0RXW {
        _PEENETH0RXW { w: self }
    }
    #[doc = "Bit 19 - Parity Error Enable for SDMMC Memory 0"]
    #[inline]
    pub fn peensd0(&mut self) -> _PEENSD0W {
        _PEENSD0W { w: self }
    }
    #[doc = "Bit 20 - Parity Error Enable for SDMMC Memory 1"]
    #[inline]
    pub fn peensd1(&mut self) -> _PEENSD1W {
        _PEENSD1W { w: self }
    }
    #[doc = "Bit 24 - Parity Error Enable for ECAT0 Memory"]
    #[inline]
    pub fn peenecat0(&mut self) -> _PEENECAT0W {
        _PEENECAT0W { w: self }
    }
}
