#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::G1ORCEN {
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
#[doc = "Possible values of the field `ENORC6`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENORC6R {
    #[doc = "Disabled"]
    CONST_0,
    #[doc = "Enabled"]
    CONST_1,
}
impl ENORC6R {
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
            ENORC6R::CONST_0 => false,
            ENORC6R::CONST_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ENORC6R {
        match value {
            false => ENORC6R::CONST_0,
            true => ENORC6R::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline]
    pub fn is_const_0(&self) -> bool {
        *self == ENORC6R::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline]
    pub fn is_const_1(&self) -> bool {
        *self == ENORC6R::CONST_1
    }
}
#[doc = "Possible values of the field `ENORC7`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENORC7R {
    #[doc = "Disabled"]
    CONST_0,
    #[doc = "Enabled"]
    CONST_1,
}
impl ENORC7R {
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
            ENORC7R::CONST_0 => false,
            ENORC7R::CONST_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ENORC7R {
        match value {
            false => ENORC7R::CONST_0,
            true => ENORC7R::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline]
    pub fn is_const_0(&self) -> bool {
        *self == ENORC7R::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline]
    pub fn is_const_1(&self) -> bool {
        *self == ENORC7R::CONST_1
    }
}
#[doc = "Values that can be written to the field `ENORC6`"]
pub enum ENORC6W {
    #[doc = "Disabled"]
    CONST_0,
    #[doc = "Enabled"]
    CONST_1,
}
impl ENORC6W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ENORC6W::CONST_0 => false,
            ENORC6W::CONST_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ENORC6W<'a> {
    w: &'a mut W,
}
impl<'a> _ENORC6W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ENORC6W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled"]
    #[inline]
    pub fn const_0(self) -> &'a mut W {
        self.variant(ENORC6W::CONST_0)
    }
    #[doc = "Enabled"]
    #[inline]
    pub fn const_1(self) -> &'a mut W {
        self.variant(ENORC6W::CONST_1)
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
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ENORC7`"]
pub enum ENORC7W {
    #[doc = "Disabled"]
    CONST_0,
    #[doc = "Enabled"]
    CONST_1,
}
impl ENORC7W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ENORC7W::CONST_0 => false,
            ENORC7W::CONST_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ENORC7W<'a> {
    w: &'a mut W,
}
impl<'a> _ENORC7W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ENORC7W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled"]
    #[inline]
    pub fn const_0(self) -> &'a mut W {
        self.variant(ENORC7W::CONST_0)
    }
    #[doc = "Enabled"]
    #[inline]
    pub fn const_1(self) -> &'a mut W {
        self.variant(ENORC7W::CONST_1)
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
        const OFFSET: u8 = 7;
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
    #[doc = "Bit 6 - Enable Out of Range Comparator, Channel 6"]
    #[inline]
    pub fn enorc6(&self) -> ENORC6R {
        ENORC6R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - Enable Out of Range Comparator, Channel 7"]
    #[inline]
    pub fn enorc7(&self) -> ENORC7R {
        ENORC7R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
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
    #[doc = "Bit 6 - Enable Out of Range Comparator, Channel 6"]
    #[inline]
    pub fn enorc6(&mut self) -> _ENORC6W {
        _ENORC6W { w: self }
    }
    #[doc = "Bit 7 - Enable Out of Range Comparator, Channel 7"]
    #[inline]
    pub fn enorc7(&mut self) -> _ENORC7W {
        _ENORC7W { w: self }
    }
}
