#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PSL {
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
#[doc = "Possible values of the field `PSL11`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PSL11R {
    #[doc = "Passive Level is LOW"]
    VALUE1,
    #[doc = "Passive Level is HIGH"]
    VALUE2,
}
impl PSL11R {
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
            PSL11R::VALUE1 => false,
            PSL11R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PSL11R {
        match value {
            false => PSL11R::VALUE1,
            true => PSL11R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == PSL11R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == PSL11R::VALUE2
    }
}
#[doc = "Possible values of the field `PSL12`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PSL12R {
    #[doc = "Passive Level is LOW"]
    VALUE1,
    #[doc = "Passive Level is HIGH"]
    VALUE2,
}
impl PSL12R {
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
            PSL12R::VALUE1 => false,
            PSL12R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PSL12R {
        match value {
            false => PSL12R::VALUE1,
            true => PSL12R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == PSL12R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == PSL12R::VALUE2
    }
}
#[doc = "Possible values of the field `PSL21`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PSL21R {
    #[doc = "Passive Level is LOW"]
    VALUE1,
    #[doc = "Passive Level is HIGH"]
    VALUE2,
}
impl PSL21R {
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
            PSL21R::VALUE1 => false,
            PSL21R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PSL21R {
        match value {
            false => PSL21R::VALUE1,
            true => PSL21R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == PSL21R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == PSL21R::VALUE2
    }
}
#[doc = "Possible values of the field `PSL22`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PSL22R {
    #[doc = "Passive Level is LOW"]
    VALUE1,
    #[doc = "Passive Level is HIGH"]
    VALUE2,
}
impl PSL22R {
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
            PSL22R::VALUE1 => false,
            PSL22R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PSL22R {
        match value {
            false => PSL22R::VALUE1,
            true => PSL22R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == PSL22R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == PSL22R::VALUE2
    }
}
#[doc = "Values that can be written to the field `PSL11`"]
pub enum PSL11W {
    #[doc = "Passive Level is LOW"]
    VALUE1,
    #[doc = "Passive Level is HIGH"]
    VALUE2,
}
impl PSL11W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PSL11W::VALUE1 => false,
            PSL11W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PSL11W<'a> {
    w: &'a mut W,
}
impl<'a> _PSL11W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PSL11W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Passive Level is LOW"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(PSL11W::VALUE1)
    }
    #[doc = "Passive Level is HIGH"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(PSL11W::VALUE2)
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
#[doc = "Values that can be written to the field `PSL12`"]
pub enum PSL12W {
    #[doc = "Passive Level is LOW"]
    VALUE1,
    #[doc = "Passive Level is HIGH"]
    VALUE2,
}
impl PSL12W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PSL12W::VALUE1 => false,
            PSL12W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PSL12W<'a> {
    w: &'a mut W,
}
impl<'a> _PSL12W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PSL12W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Passive Level is LOW"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(PSL12W::VALUE1)
    }
    #[doc = "Passive Level is HIGH"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(PSL12W::VALUE2)
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
#[doc = "Values that can be written to the field `PSL21`"]
pub enum PSL21W {
    #[doc = "Passive Level is LOW"]
    VALUE1,
    #[doc = "Passive Level is HIGH"]
    VALUE2,
}
impl PSL21W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PSL21W::VALUE1 => false,
            PSL21W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PSL21W<'a> {
    w: &'a mut W,
}
impl<'a> _PSL21W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PSL21W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Passive Level is LOW"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(PSL21W::VALUE1)
    }
    #[doc = "Passive Level is HIGH"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(PSL21W::VALUE2)
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
#[doc = "Values that can be written to the field `PSL22`"]
pub enum PSL22W {
    #[doc = "Passive Level is LOW"]
    VALUE1,
    #[doc = "Passive Level is HIGH"]
    VALUE2,
}
impl PSL22W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PSL22W::VALUE1 => false,
            PSL22W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PSL22W<'a> {
    w: &'a mut W,
}
impl<'a> _PSL22W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PSL22W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Passive Level is LOW"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(PSL22W::VALUE1)
    }
    #[doc = "Passive Level is HIGH"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(PSL22W::VALUE2)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Output Passive Level for CCU8x.OUTy0"]
    #[inline]
    pub fn psl11(&self) -> PSL11R {
        PSL11R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Output Passive Level for CCU8x.OUTy1"]
    #[inline]
    pub fn psl12(&self) -> PSL12R {
        PSL12R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Output Passive Level for CCU8x.OUTy2"]
    #[inline]
    pub fn psl21(&self) -> PSL21R {
        PSL21R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Output Passive Level for CCU8x.OUTy3"]
    #[inline]
    pub fn psl22(&self) -> PSL22R {
        PSL22R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
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
    #[doc = "Bit 0 - Output Passive Level for CCU8x.OUTy0"]
    #[inline]
    pub fn psl11(&mut self) -> _PSL11W {
        _PSL11W { w: self }
    }
    #[doc = "Bit 1 - Output Passive Level for CCU8x.OUTy1"]
    #[inline]
    pub fn psl12(&mut self) -> _PSL12W {
        _PSL12W { w: self }
    }
    #[doc = "Bit 2 - Output Passive Level for CCU8x.OUTy2"]
    #[inline]
    pub fn psl21(&mut self) -> _PSL21W {
        _PSL21W { w: self }
    }
    #[doc = "Bit 3 - Output Passive Level for CCU8x.OUTy3"]
    #[inline]
    pub fn psl22(&mut self) -> _PSL22W {
        _PSL22W { w: self }
    }
}
