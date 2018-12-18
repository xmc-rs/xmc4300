#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CEFLAG {
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
#[doc = "Possible values of the field `CEV0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CEV0R {
    #[doc = "No channel event"]
    VALUE1,
    #[doc = "A channel event has occurred"]
    VALUE2,
}
impl CEV0R {
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
            CEV0R::VALUE1 => false,
            CEV0R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CEV0R {
        match value {
            false => CEV0R::VALUE1,
            true => CEV0R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == CEV0R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == CEV0R::VALUE2
    }
}
#[doc = "Possible values of the field `CEV1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CEV1R {
    #[doc = "No channel event"]
    VALUE1,
    #[doc = "A channel event has occurred"]
    VALUE2,
}
impl CEV1R {
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
            CEV1R::VALUE1 => false,
            CEV1R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CEV1R {
        match value {
            false => CEV1R::VALUE1,
            true => CEV1R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == CEV1R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == CEV1R::VALUE2
    }
}
#[doc = "Possible values of the field `CEV2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CEV2R {
    #[doc = "No channel event"]
    VALUE1,
    #[doc = "A channel event has occurred"]
    VALUE2,
}
impl CEV2R {
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
            CEV2R::VALUE1 => false,
            CEV2R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CEV2R {
        match value {
            false => CEV2R::VALUE1,
            true => CEV2R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == CEV2R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == CEV2R::VALUE2
    }
}
#[doc = "Possible values of the field `CEV3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CEV3R {
    #[doc = "No channel event"]
    VALUE1,
    #[doc = "A channel event has occurred"]
    VALUE2,
}
impl CEV3R {
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
            CEV3R::VALUE1 => false,
            CEV3R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CEV3R {
        match value {
            false => CEV3R::VALUE1,
            true => CEV3R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == CEV3R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == CEV3R::VALUE2
    }
}
#[doc = "Possible values of the field `CEV4`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CEV4R {
    #[doc = "No channel event"]
    VALUE1,
    #[doc = "A channel event has occurred"]
    VALUE2,
}
impl CEV4R {
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
            CEV4R::VALUE1 => false,
            CEV4R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CEV4R {
        match value {
            false => CEV4R::VALUE1,
            true => CEV4R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == CEV4R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == CEV4R::VALUE2
    }
}
#[doc = "Possible values of the field `CEV5`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CEV5R {
    #[doc = "No channel event"]
    VALUE1,
    #[doc = "A channel event has occurred"]
    VALUE2,
}
impl CEV5R {
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
            CEV5R::VALUE1 => false,
            CEV5R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CEV5R {
        match value {
            false => CEV5R::VALUE1,
            true => CEV5R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == CEV5R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == CEV5R::VALUE2
    }
}
#[doc = "Possible values of the field `CEV6`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CEV6R {
    #[doc = "No channel event"]
    VALUE1,
    #[doc = "A channel event has occurred"]
    VALUE2,
}
impl CEV6R {
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
            CEV6R::VALUE1 => false,
            CEV6R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CEV6R {
        match value {
            false => CEV6R::VALUE1,
            true => CEV6R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == CEV6R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == CEV6R::VALUE2
    }
}
#[doc = "Possible values of the field `CEV7`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CEV7R {
    #[doc = "No channel event"]
    VALUE1,
    #[doc = "A channel event has occurred"]
    VALUE2,
}
impl CEV7R {
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
            CEV7R::VALUE1 => false,
            CEV7R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CEV7R {
        match value {
            false => CEV7R::VALUE1,
            true => CEV7R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == CEV7R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == CEV7R::VALUE2
    }
}
#[doc = "Values that can be written to the field `CEV0`"]
pub enum CEV0W {
    #[doc = "No channel event"]
    VALUE1,
    #[doc = "A channel event has occurred"]
    VALUE2,
}
impl CEV0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CEV0W::VALUE1 => false,
            CEV0W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CEV0W<'a> {
    w: &'a mut W,
}
impl<'a> _CEV0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CEV0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No channel event"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(CEV0W::VALUE1)
    }
    #[doc = "A channel event has occurred"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(CEV0W::VALUE2)
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
#[doc = "Values that can be written to the field `CEV1`"]
pub enum CEV1W {
    #[doc = "No channel event"]
    VALUE1,
    #[doc = "A channel event has occurred"]
    VALUE2,
}
impl CEV1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CEV1W::VALUE1 => false,
            CEV1W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CEV1W<'a> {
    w: &'a mut W,
}
impl<'a> _CEV1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CEV1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No channel event"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(CEV1W::VALUE1)
    }
    #[doc = "A channel event has occurred"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(CEV1W::VALUE2)
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
#[doc = "Values that can be written to the field `CEV2`"]
pub enum CEV2W {
    #[doc = "No channel event"]
    VALUE1,
    #[doc = "A channel event has occurred"]
    VALUE2,
}
impl CEV2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CEV2W::VALUE1 => false,
            CEV2W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CEV2W<'a> {
    w: &'a mut W,
}
impl<'a> _CEV2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CEV2W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No channel event"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(CEV2W::VALUE1)
    }
    #[doc = "A channel event has occurred"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(CEV2W::VALUE2)
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
#[doc = "Values that can be written to the field `CEV3`"]
pub enum CEV3W {
    #[doc = "No channel event"]
    VALUE1,
    #[doc = "A channel event has occurred"]
    VALUE2,
}
impl CEV3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CEV3W::VALUE1 => false,
            CEV3W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CEV3W<'a> {
    w: &'a mut W,
}
impl<'a> _CEV3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CEV3W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No channel event"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(CEV3W::VALUE1)
    }
    #[doc = "A channel event has occurred"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(CEV3W::VALUE2)
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
#[doc = "Values that can be written to the field `CEV4`"]
pub enum CEV4W {
    #[doc = "No channel event"]
    VALUE1,
    #[doc = "A channel event has occurred"]
    VALUE2,
}
impl CEV4W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CEV4W::VALUE1 => false,
            CEV4W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CEV4W<'a> {
    w: &'a mut W,
}
impl<'a> _CEV4W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CEV4W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No channel event"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(CEV4W::VALUE1)
    }
    #[doc = "A channel event has occurred"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(CEV4W::VALUE2)
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
#[doc = "Values that can be written to the field `CEV5`"]
pub enum CEV5W {
    #[doc = "No channel event"]
    VALUE1,
    #[doc = "A channel event has occurred"]
    VALUE2,
}
impl CEV5W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CEV5W::VALUE1 => false,
            CEV5W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CEV5W<'a> {
    w: &'a mut W,
}
impl<'a> _CEV5W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CEV5W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No channel event"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(CEV5W::VALUE1)
    }
    #[doc = "A channel event has occurred"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(CEV5W::VALUE2)
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
#[doc = "Values that can be written to the field `CEV6`"]
pub enum CEV6W {
    #[doc = "No channel event"]
    VALUE1,
    #[doc = "A channel event has occurred"]
    VALUE2,
}
impl CEV6W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CEV6W::VALUE1 => false,
            CEV6W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CEV6W<'a> {
    w: &'a mut W,
}
impl<'a> _CEV6W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CEV6W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No channel event"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(CEV6W::VALUE1)
    }
    #[doc = "A channel event has occurred"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(CEV6W::VALUE2)
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
#[doc = "Values that can be written to the field `CEV7`"]
pub enum CEV7W {
    #[doc = "No channel event"]
    VALUE1,
    #[doc = "A channel event has occurred"]
    VALUE2,
}
impl CEV7W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CEV7W::VALUE1 => false,
            CEV7W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CEV7W<'a> {
    w: &'a mut W,
}
impl<'a> _CEV7W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CEV7W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No channel event"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(CEV7W::VALUE1)
    }
    #[doc = "A channel event has occurred"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(CEV7W::VALUE2)
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
    #[doc = "Bit 0 - Channel Event for Channel 0"]
    #[inline]
    pub fn cev0(&self) -> CEV0R {
        CEV0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Channel Event for Channel 1"]
    #[inline]
    pub fn cev1(&self) -> CEV1R {
        CEV1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Channel Event for Channel 2"]
    #[inline]
    pub fn cev2(&self) -> CEV2R {
        CEV2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Channel Event for Channel 3"]
    #[inline]
    pub fn cev3(&self) -> CEV3R {
        CEV3R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Channel Event for Channel 4"]
    #[inline]
    pub fn cev4(&self) -> CEV4R {
        CEV4R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Channel Event for Channel 5"]
    #[inline]
    pub fn cev5(&self) -> CEV5R {
        CEV5R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - Channel Event for Channel 6"]
    #[inline]
    pub fn cev6(&self) -> CEV6R {
        CEV6R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - Channel Event for Channel 7"]
    #[inline]
    pub fn cev7(&self) -> CEV7R {
        CEV7R::_from({
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
    #[doc = "Bit 0 - Channel Event for Channel 0"]
    #[inline]
    pub fn cev0(&mut self) -> _CEV0W {
        _CEV0W { w: self }
    }
    #[doc = "Bit 1 - Channel Event for Channel 1"]
    #[inline]
    pub fn cev1(&mut self) -> _CEV1W {
        _CEV1W { w: self }
    }
    #[doc = "Bit 2 - Channel Event for Channel 2"]
    #[inline]
    pub fn cev2(&mut self) -> _CEV2W {
        _CEV2W { w: self }
    }
    #[doc = "Bit 3 - Channel Event for Channel 3"]
    #[inline]
    pub fn cev3(&mut self) -> _CEV3W {
        _CEV3W { w: self }
    }
    #[doc = "Bit 4 - Channel Event for Channel 4"]
    #[inline]
    pub fn cev4(&mut self) -> _CEV4W {
        _CEV4W { w: self }
    }
    #[doc = "Bit 5 - Channel Event for Channel 5"]
    #[inline]
    pub fn cev5(&mut self) -> _CEV5W {
        _CEV5W { w: self }
    }
    #[doc = "Bit 6 - Channel Event for Channel 6"]
    #[inline]
    pub fn cev6(&mut self) -> _CEV6W {
        _CEV6W { w: self }
    }
    #[doc = "Bit 7 - Channel Event for Channel 7"]
    #[inline]
    pub fn cev7(&mut self) -> _CEV7W {
        _CEV7W { w: self }
    }
}
