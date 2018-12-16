#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::BFL {
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
#[doc = "Possible values of the field `BFL0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BFL0R {
    #[doc = "Passive state: result has not yet crossed the activation boundary (see bitfield BFAy), or selected gate signal is inactive, or this boundary flag is disabled"]
    VALUE1,
    #[doc = "Active state: result has crossed the activation boundary"]
    VALUE2,
}
impl BFL0R {
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
            BFL0R::VALUE1 => false,
            BFL0R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BFL0R {
        match value {
            false => BFL0R::VALUE1,
            true => BFL0R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == BFL0R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == BFL0R::VALUE2
    }
}
#[doc = "Possible values of the field `BFL1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BFL1R {
    #[doc = "Passive state: result has not yet crossed the activation boundary (see bitfield BFAy), or selected gate signal is inactive, or this boundary flag is disabled"]
    VALUE1,
    #[doc = "Active state: result has crossed the activation boundary"]
    VALUE2,
}
impl BFL1R {
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
            BFL1R::VALUE1 => false,
            BFL1R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BFL1R {
        match value {
            false => BFL1R::VALUE1,
            true => BFL1R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == BFL1R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == BFL1R::VALUE2
    }
}
#[doc = "Possible values of the field `BFL2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BFL2R {
    #[doc = "Passive state: result has not yet crossed the activation boundary (see bitfield BFAy), or selected gate signal is inactive, or this boundary flag is disabled"]
    VALUE1,
    #[doc = "Active state: result has crossed the activation boundary"]
    VALUE2,
}
impl BFL2R {
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
            BFL2R::VALUE1 => false,
            BFL2R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BFL2R {
        match value {
            false => BFL2R::VALUE1,
            true => BFL2R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == BFL2R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == BFL2R::VALUE2
    }
}
#[doc = "Possible values of the field `BFL3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BFL3R {
    #[doc = "Passive state: result has not yet crossed the activation boundary (see bitfield BFAy), or selected gate signal is inactive, or this boundary flag is disabled"]
    VALUE1,
    #[doc = "Active state: result has crossed the activation boundary"]
    VALUE2,
}
impl BFL3R {
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
            BFL3R::VALUE1 => false,
            BFL3R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BFL3R {
        match value {
            false => BFL3R::VALUE1,
            true => BFL3R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == BFL3R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == BFL3R::VALUE2
    }
}
#[doc = "Possible values of the field `BFA0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BFA0R {
    #[doc = "Set boundary flag BFLy if result is above the defined band or compare value, clear if below"]
    VALUE1,
    #[doc = "Set boundary flag BFLy if result is below the defined band or compare value, clear if above"]
    VALUE2,
}
impl BFA0R {
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
            BFA0R::VALUE1 => false,
            BFA0R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BFA0R {
        match value {
            false => BFA0R::VALUE1,
            true => BFA0R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == BFA0R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == BFA0R::VALUE2
    }
}
#[doc = "Possible values of the field `BFA1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BFA1R {
    #[doc = "Set boundary flag BFLy if result is above the defined band or compare value, clear if below"]
    VALUE1,
    #[doc = "Set boundary flag BFLy if result is below the defined band or compare value, clear if above"]
    VALUE2,
}
impl BFA1R {
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
            BFA1R::VALUE1 => false,
            BFA1R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BFA1R {
        match value {
            false => BFA1R::VALUE1,
            true => BFA1R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == BFA1R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == BFA1R::VALUE2
    }
}
#[doc = "Possible values of the field `BFA2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BFA2R {
    #[doc = "Set boundary flag BFLy if result is above the defined band or compare value, clear if below"]
    VALUE1,
    #[doc = "Set boundary flag BFLy if result is below the defined band or compare value, clear if above"]
    VALUE2,
}
impl BFA2R {
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
            BFA2R::VALUE1 => false,
            BFA2R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BFA2R {
        match value {
            false => BFA2R::VALUE1,
            true => BFA2R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == BFA2R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == BFA2R::VALUE2
    }
}
#[doc = "Possible values of the field `BFA3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BFA3R {
    #[doc = "Set boundary flag BFLy if result is above the defined band or compare value, clear if below"]
    VALUE1,
    #[doc = "Set boundary flag BFLy if result is below the defined band or compare value, clear if above"]
    VALUE2,
}
impl BFA3R {
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
            BFA3R::VALUE1 => false,
            BFA3R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BFA3R {
        match value {
            false => BFA3R::VALUE1,
            true => BFA3R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == BFA3R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == BFA3R::VALUE2
    }
}
#[doc = "Possible values of the field `BFI0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BFI0R {
    #[doc = "Use BFLy directly"]
    VALUE1,
    #[doc = "Invert value and use BFLy"]
    VALUE2,
}
impl BFI0R {
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
            BFI0R::VALUE1 => false,
            BFI0R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BFI0R {
        match value {
            false => BFI0R::VALUE1,
            true => BFI0R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == BFI0R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == BFI0R::VALUE2
    }
}
#[doc = "Possible values of the field `BFI1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BFI1R {
    #[doc = "Use BFLy directly"]
    VALUE1,
    #[doc = "Invert value and use BFLy"]
    VALUE2,
}
impl BFI1R {
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
            BFI1R::VALUE1 => false,
            BFI1R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BFI1R {
        match value {
            false => BFI1R::VALUE1,
            true => BFI1R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == BFI1R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == BFI1R::VALUE2
    }
}
#[doc = "Possible values of the field `BFI2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BFI2R {
    #[doc = "Use BFLy directly"]
    VALUE1,
    #[doc = "Invert value and use BFLy"]
    VALUE2,
}
impl BFI2R {
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
            BFI2R::VALUE1 => false,
            BFI2R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BFI2R {
        match value {
            false => BFI2R::VALUE1,
            true => BFI2R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == BFI2R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == BFI2R::VALUE2
    }
}
#[doc = "Possible values of the field `BFI3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BFI3R {
    #[doc = "Use BFLy directly"]
    VALUE1,
    #[doc = "Invert value and use BFLy"]
    VALUE2,
}
impl BFI3R {
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
            BFI3R::VALUE1 => false,
            BFI3R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BFI3R {
        match value {
            false => BFI3R::VALUE1,
            true => BFI3R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == BFI3R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == BFI3R::VALUE2
    }
}
#[doc = "Values that can be written to the field `BFA0`"]
pub enum BFA0W {
    #[doc = "Set boundary flag BFLy if result is above the defined band or compare value, clear if below"]
    VALUE1,
    #[doc = "Set boundary flag BFLy if result is below the defined band or compare value, clear if above"]
    VALUE2,
}
impl BFA0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BFA0W::VALUE1 => false,
            BFA0W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BFA0W<'a> {
    w: &'a mut W,
}
impl<'a> _BFA0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BFA0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Set boundary flag BFLy if result is above the defined band or compare value, clear if below"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(BFA0W::VALUE1)
    }
    #[doc = "Set boundary flag BFLy if result is below the defined band or compare value, clear if above"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(BFA0W::VALUE2)
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
#[doc = "Values that can be written to the field `BFA1`"]
pub enum BFA1W {
    #[doc = "Set boundary flag BFLy if result is above the defined band or compare value, clear if below"]
    VALUE1,
    #[doc = "Set boundary flag BFLy if result is below the defined band or compare value, clear if above"]
    VALUE2,
}
impl BFA1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BFA1W::VALUE1 => false,
            BFA1W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BFA1W<'a> {
    w: &'a mut W,
}
impl<'a> _BFA1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BFA1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Set boundary flag BFLy if result is above the defined band or compare value, clear if below"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(BFA1W::VALUE1)
    }
    #[doc = "Set boundary flag BFLy if result is below the defined band or compare value, clear if above"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(BFA1W::VALUE2)
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
#[doc = "Values that can be written to the field `BFA2`"]
pub enum BFA2W {
    #[doc = "Set boundary flag BFLy if result is above the defined band or compare value, clear if below"]
    VALUE1,
    #[doc = "Set boundary flag BFLy if result is below the defined band or compare value, clear if above"]
    VALUE2,
}
impl BFA2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BFA2W::VALUE1 => false,
            BFA2W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BFA2W<'a> {
    w: &'a mut W,
}
impl<'a> _BFA2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BFA2W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Set boundary flag BFLy if result is above the defined band or compare value, clear if below"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(BFA2W::VALUE1)
    }
    #[doc = "Set boundary flag BFLy if result is below the defined band or compare value, clear if above"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(BFA2W::VALUE2)
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
#[doc = "Values that can be written to the field `BFA3`"]
pub enum BFA3W {
    #[doc = "Set boundary flag BFLy if result is above the defined band or compare value, clear if below"]
    VALUE1,
    #[doc = "Set boundary flag BFLy if result is below the defined band or compare value, clear if above"]
    VALUE2,
}
impl BFA3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BFA3W::VALUE1 => false,
            BFA3W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BFA3W<'a> {
    w: &'a mut W,
}
impl<'a> _BFA3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BFA3W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Set boundary flag BFLy if result is above the defined band or compare value, clear if below"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(BFA3W::VALUE1)
    }
    #[doc = "Set boundary flag BFLy if result is below the defined band or compare value, clear if above"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(BFA3W::VALUE2)
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
        const OFFSET: u8 = 11;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `BFI0`"]
pub enum BFI0W {
    #[doc = "Use BFLy directly"]
    VALUE1,
    #[doc = "Invert value and use BFLy"]
    VALUE2,
}
impl BFI0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BFI0W::VALUE1 => false,
            BFI0W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BFI0W<'a> {
    w: &'a mut W,
}
impl<'a> _BFI0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BFI0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Use BFLy directly"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(BFI0W::VALUE1)
    }
    #[doc = "Invert value and use BFLy"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(BFI0W::VALUE2)
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
#[doc = "Values that can be written to the field `BFI1`"]
pub enum BFI1W {
    #[doc = "Use BFLy directly"]
    VALUE1,
    #[doc = "Invert value and use BFLy"]
    VALUE2,
}
impl BFI1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BFI1W::VALUE1 => false,
            BFI1W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BFI1W<'a> {
    w: &'a mut W,
}
impl<'a> _BFI1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BFI1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Use BFLy directly"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(BFI1W::VALUE1)
    }
    #[doc = "Invert value and use BFLy"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(BFI1W::VALUE2)
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
#[doc = "Values that can be written to the field `BFI2`"]
pub enum BFI2W {
    #[doc = "Use BFLy directly"]
    VALUE1,
    #[doc = "Invert value and use BFLy"]
    VALUE2,
}
impl BFI2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BFI2W::VALUE1 => false,
            BFI2W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BFI2W<'a> {
    w: &'a mut W,
}
impl<'a> _BFI2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BFI2W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Use BFLy directly"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(BFI2W::VALUE1)
    }
    #[doc = "Invert value and use BFLy"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(BFI2W::VALUE2)
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
#[doc = "Values that can be written to the field `BFI3`"]
pub enum BFI3W {
    #[doc = "Use BFLy directly"]
    VALUE1,
    #[doc = "Invert value and use BFLy"]
    VALUE2,
}
impl BFI3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BFI3W::VALUE1 => false,
            BFI3W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BFI3W<'a> {
    w: &'a mut W,
}
impl<'a> _BFI3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BFI3W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Use BFLy directly"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(BFI3W::VALUE1)
    }
    #[doc = "Invert value and use BFLy"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(BFI3W::VALUE2)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Boundary Flag 0"]
    #[inline]
    pub fn bfl0(&self) -> BFL0R {
        BFL0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Boundary Flag 1"]
    #[inline]
    pub fn bfl1(&self) -> BFL1R {
        BFL1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Boundary Flag 2"]
    #[inline]
    pub fn bfl2(&self) -> BFL2R {
        BFL2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Boundary Flag 3"]
    #[inline]
    pub fn bfl3(&self) -> BFL3R {
        BFL3R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Boundary Flag 0 Activation Select"]
    #[inline]
    pub fn bfa0(&self) -> BFA0R {
        BFA0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - Boundary Flag 1 Activation Select"]
    #[inline]
    pub fn bfa1(&self) -> BFA1R {
        BFA1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - Boundary Flag 2 Activation Select"]
    #[inline]
    pub fn bfa2(&self) -> BFA2R {
        BFA2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - Boundary Flag 3 Activation Select"]
    #[inline]
    pub fn bfa3(&self) -> BFA3R {
        BFA3R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 16 - Boundary Flag 0 Inversion Control"]
    #[inline]
    pub fn bfi0(&self) -> BFI0R {
        BFI0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 17 - Boundary Flag 1 Inversion Control"]
    #[inline]
    pub fn bfi1(&self) -> BFI1R {
        BFI1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 18 - Boundary Flag 2 Inversion Control"]
    #[inline]
    pub fn bfi2(&self) -> BFI2R {
        BFI2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 19 - Boundary Flag 3 Inversion Control"]
    #[inline]
    pub fn bfi3(&self) -> BFI3R {
        BFI3R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 19;
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
    #[doc = "Bit 8 - Boundary Flag 0 Activation Select"]
    #[inline]
    pub fn bfa0(&mut self) -> _BFA0W {
        _BFA0W { w: self }
    }
    #[doc = "Bit 9 - Boundary Flag 1 Activation Select"]
    #[inline]
    pub fn bfa1(&mut self) -> _BFA1W {
        _BFA1W { w: self }
    }
    #[doc = "Bit 10 - Boundary Flag 2 Activation Select"]
    #[inline]
    pub fn bfa2(&mut self) -> _BFA2W {
        _BFA2W { w: self }
    }
    #[doc = "Bit 11 - Boundary Flag 3 Activation Select"]
    #[inline]
    pub fn bfa3(&mut self) -> _BFA3W {
        _BFA3W { w: self }
    }
    #[doc = "Bit 16 - Boundary Flag 0 Inversion Control"]
    #[inline]
    pub fn bfi0(&mut self) -> _BFI0W {
        _BFI0W { w: self }
    }
    #[doc = "Bit 17 - Boundary Flag 1 Inversion Control"]
    #[inline]
    pub fn bfi1(&mut self) -> _BFI1W {
        _BFI1W { w: self }
    }
    #[doc = "Bit 18 - Boundary Flag 2 Inversion Control"]
    #[inline]
    pub fn bfi2(&mut self) -> _BFI2W {
        _BFI2W { w: self }
    }
    #[doc = "Bit 19 - Boundary Flag 3 Inversion Control"]
    #[inline]
    pub fn bfi3(&mut self) -> _BFI3W {
        _BFI3W { w: self }
    }
}
