#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CHASS {
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
#[doc = "Possible values of the field `ASSCH0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ASSCH0R {
    #[doc = "Channel y can be a background channel converted with lowest priority"]
    VALUE1,
    #[doc = "Channel y is a priority channel within group x"]
    VALUE2,
}
impl ASSCH0R {
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
            ASSCH0R::VALUE1 => false,
            ASSCH0R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ASSCH0R {
        match value {
            false => ASSCH0R::VALUE1,
            true => ASSCH0R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == ASSCH0R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == ASSCH0R::VALUE2
    }
}
#[doc = "Possible values of the field `ASSCH1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ASSCH1R {
    #[doc = "Channel y can be a background channel converted with lowest priority"]
    VALUE1,
    #[doc = "Channel y is a priority channel within group x"]
    VALUE2,
}
impl ASSCH1R {
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
            ASSCH1R::VALUE1 => false,
            ASSCH1R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ASSCH1R {
        match value {
            false => ASSCH1R::VALUE1,
            true => ASSCH1R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == ASSCH1R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == ASSCH1R::VALUE2
    }
}
#[doc = "Possible values of the field `ASSCH2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ASSCH2R {
    #[doc = "Channel y can be a background channel converted with lowest priority"]
    VALUE1,
    #[doc = "Channel y is a priority channel within group x"]
    VALUE2,
}
impl ASSCH2R {
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
            ASSCH2R::VALUE1 => false,
            ASSCH2R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ASSCH2R {
        match value {
            false => ASSCH2R::VALUE1,
            true => ASSCH2R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == ASSCH2R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == ASSCH2R::VALUE2
    }
}
#[doc = "Possible values of the field `ASSCH3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ASSCH3R {
    #[doc = "Channel y can be a background channel converted with lowest priority"]
    VALUE1,
    #[doc = "Channel y is a priority channel within group x"]
    VALUE2,
}
impl ASSCH3R {
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
            ASSCH3R::VALUE1 => false,
            ASSCH3R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ASSCH3R {
        match value {
            false => ASSCH3R::VALUE1,
            true => ASSCH3R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == ASSCH3R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == ASSCH3R::VALUE2
    }
}
#[doc = "Possible values of the field `ASSCH4`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ASSCH4R {
    #[doc = "Channel y can be a background channel converted with lowest priority"]
    VALUE1,
    #[doc = "Channel y is a priority channel within group x"]
    VALUE2,
}
impl ASSCH4R {
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
            ASSCH4R::VALUE1 => false,
            ASSCH4R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ASSCH4R {
        match value {
            false => ASSCH4R::VALUE1,
            true => ASSCH4R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == ASSCH4R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == ASSCH4R::VALUE2
    }
}
#[doc = "Possible values of the field `ASSCH5`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ASSCH5R {
    #[doc = "Channel y can be a background channel converted with lowest priority"]
    VALUE1,
    #[doc = "Channel y is a priority channel within group x"]
    VALUE2,
}
impl ASSCH5R {
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
            ASSCH5R::VALUE1 => false,
            ASSCH5R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ASSCH5R {
        match value {
            false => ASSCH5R::VALUE1,
            true => ASSCH5R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == ASSCH5R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == ASSCH5R::VALUE2
    }
}
#[doc = "Possible values of the field `ASSCH6`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ASSCH6R {
    #[doc = "Channel y can be a background channel converted with lowest priority"]
    VALUE1,
    #[doc = "Channel y is a priority channel within group x"]
    VALUE2,
}
impl ASSCH6R {
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
            ASSCH6R::VALUE1 => false,
            ASSCH6R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ASSCH6R {
        match value {
            false => ASSCH6R::VALUE1,
            true => ASSCH6R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == ASSCH6R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == ASSCH6R::VALUE2
    }
}
#[doc = "Possible values of the field `ASSCH7`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ASSCH7R {
    #[doc = "Channel y can be a background channel converted with lowest priority"]
    VALUE1,
    #[doc = "Channel y is a priority channel within group x"]
    VALUE2,
}
impl ASSCH7R {
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
            ASSCH7R::VALUE1 => false,
            ASSCH7R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ASSCH7R {
        match value {
            false => ASSCH7R::VALUE1,
            true => ASSCH7R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == ASSCH7R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == ASSCH7R::VALUE2
    }
}
#[doc = "Values that can be written to the field `ASSCH0`"]
pub enum ASSCH0W {
    #[doc = "Channel y can be a background channel converted with lowest priority"]
    VALUE1,
    #[doc = "Channel y is a priority channel within group x"]
    VALUE2,
}
impl ASSCH0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ASSCH0W::VALUE1 => false,
            ASSCH0W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ASSCH0W<'a> {
    w: &'a mut W,
}
impl<'a> _ASSCH0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ASSCH0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Channel y can be a background channel converted with lowest priority"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(ASSCH0W::VALUE1)
    }
    #[doc = "Channel y is a priority channel within group x"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(ASSCH0W::VALUE2)
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
#[doc = "Values that can be written to the field `ASSCH1`"]
pub enum ASSCH1W {
    #[doc = "Channel y can be a background channel converted with lowest priority"]
    VALUE1,
    #[doc = "Channel y is a priority channel within group x"]
    VALUE2,
}
impl ASSCH1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ASSCH1W::VALUE1 => false,
            ASSCH1W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ASSCH1W<'a> {
    w: &'a mut W,
}
impl<'a> _ASSCH1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ASSCH1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Channel y can be a background channel converted with lowest priority"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(ASSCH1W::VALUE1)
    }
    #[doc = "Channel y is a priority channel within group x"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(ASSCH1W::VALUE2)
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
#[doc = "Values that can be written to the field `ASSCH2`"]
pub enum ASSCH2W {
    #[doc = "Channel y can be a background channel converted with lowest priority"]
    VALUE1,
    #[doc = "Channel y is a priority channel within group x"]
    VALUE2,
}
impl ASSCH2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ASSCH2W::VALUE1 => false,
            ASSCH2W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ASSCH2W<'a> {
    w: &'a mut W,
}
impl<'a> _ASSCH2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ASSCH2W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Channel y can be a background channel converted with lowest priority"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(ASSCH2W::VALUE1)
    }
    #[doc = "Channel y is a priority channel within group x"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(ASSCH2W::VALUE2)
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
#[doc = "Values that can be written to the field `ASSCH3`"]
pub enum ASSCH3W {
    #[doc = "Channel y can be a background channel converted with lowest priority"]
    VALUE1,
    #[doc = "Channel y is a priority channel within group x"]
    VALUE2,
}
impl ASSCH3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ASSCH3W::VALUE1 => false,
            ASSCH3W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ASSCH3W<'a> {
    w: &'a mut W,
}
impl<'a> _ASSCH3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ASSCH3W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Channel y can be a background channel converted with lowest priority"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(ASSCH3W::VALUE1)
    }
    #[doc = "Channel y is a priority channel within group x"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(ASSCH3W::VALUE2)
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
#[doc = "Values that can be written to the field `ASSCH4`"]
pub enum ASSCH4W {
    #[doc = "Channel y can be a background channel converted with lowest priority"]
    VALUE1,
    #[doc = "Channel y is a priority channel within group x"]
    VALUE2,
}
impl ASSCH4W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ASSCH4W::VALUE1 => false,
            ASSCH4W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ASSCH4W<'a> {
    w: &'a mut W,
}
impl<'a> _ASSCH4W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ASSCH4W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Channel y can be a background channel converted with lowest priority"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(ASSCH4W::VALUE1)
    }
    #[doc = "Channel y is a priority channel within group x"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(ASSCH4W::VALUE2)
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
#[doc = "Values that can be written to the field `ASSCH5`"]
pub enum ASSCH5W {
    #[doc = "Channel y can be a background channel converted with lowest priority"]
    VALUE1,
    #[doc = "Channel y is a priority channel within group x"]
    VALUE2,
}
impl ASSCH5W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ASSCH5W::VALUE1 => false,
            ASSCH5W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ASSCH5W<'a> {
    w: &'a mut W,
}
impl<'a> _ASSCH5W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ASSCH5W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Channel y can be a background channel converted with lowest priority"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(ASSCH5W::VALUE1)
    }
    #[doc = "Channel y is a priority channel within group x"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(ASSCH5W::VALUE2)
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
#[doc = "Values that can be written to the field `ASSCH6`"]
pub enum ASSCH6W {
    #[doc = "Channel y can be a background channel converted with lowest priority"]
    VALUE1,
    #[doc = "Channel y is a priority channel within group x"]
    VALUE2,
}
impl ASSCH6W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ASSCH6W::VALUE1 => false,
            ASSCH6W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ASSCH6W<'a> {
    w: &'a mut W,
}
impl<'a> _ASSCH6W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ASSCH6W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Channel y can be a background channel converted with lowest priority"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(ASSCH6W::VALUE1)
    }
    #[doc = "Channel y is a priority channel within group x"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(ASSCH6W::VALUE2)
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
#[doc = "Values that can be written to the field `ASSCH7`"]
pub enum ASSCH7W {
    #[doc = "Channel y can be a background channel converted with lowest priority"]
    VALUE1,
    #[doc = "Channel y is a priority channel within group x"]
    VALUE2,
}
impl ASSCH7W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ASSCH7W::VALUE1 => false,
            ASSCH7W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ASSCH7W<'a> {
    w: &'a mut W,
}
impl<'a> _ASSCH7W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ASSCH7W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Channel y can be a background channel converted with lowest priority"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(ASSCH7W::VALUE1)
    }
    #[doc = "Channel y is a priority channel within group x"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(ASSCH7W::VALUE2)
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
    #[doc = "Bit 0 - Assignment for Channel 0"]
    #[inline]
    pub fn assch0(&self) -> ASSCH0R {
        ASSCH0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Assignment for Channel 1"]
    #[inline]
    pub fn assch1(&self) -> ASSCH1R {
        ASSCH1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Assignment for Channel 2"]
    #[inline]
    pub fn assch2(&self) -> ASSCH2R {
        ASSCH2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Assignment for Channel 3"]
    #[inline]
    pub fn assch3(&self) -> ASSCH3R {
        ASSCH3R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Assignment for Channel 4"]
    #[inline]
    pub fn assch4(&self) -> ASSCH4R {
        ASSCH4R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Assignment for Channel 5"]
    #[inline]
    pub fn assch5(&self) -> ASSCH5R {
        ASSCH5R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - Assignment for Channel 6"]
    #[inline]
    pub fn assch6(&self) -> ASSCH6R {
        ASSCH6R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - Assignment for Channel 7"]
    #[inline]
    pub fn assch7(&self) -> ASSCH7R {
        ASSCH7R::_from({
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
    #[doc = "Bit 0 - Assignment for Channel 0"]
    #[inline]
    pub fn assch0(&mut self) -> _ASSCH0W {
        _ASSCH0W { w: self }
    }
    #[doc = "Bit 1 - Assignment for Channel 1"]
    #[inline]
    pub fn assch1(&mut self) -> _ASSCH1W {
        _ASSCH1W { w: self }
    }
    #[doc = "Bit 2 - Assignment for Channel 2"]
    #[inline]
    pub fn assch2(&mut self) -> _ASSCH2W {
        _ASSCH2W { w: self }
    }
    #[doc = "Bit 3 - Assignment for Channel 3"]
    #[inline]
    pub fn assch3(&mut self) -> _ASSCH3W {
        _ASSCH3W { w: self }
    }
    #[doc = "Bit 4 - Assignment for Channel 4"]
    #[inline]
    pub fn assch4(&mut self) -> _ASSCH4W {
        _ASSCH4W { w: self }
    }
    #[doc = "Bit 5 - Assignment for Channel 5"]
    #[inline]
    pub fn assch5(&mut self) -> _ASSCH5W {
        _ASSCH5W { w: self }
    }
    #[doc = "Bit 6 - Assignment for Channel 6"]
    #[inline]
    pub fn assch6(&mut self) -> _ASSCH6W {
        _ASSCH6W { w: self }
    }
    #[doc = "Bit 7 - Assignment for Channel 7"]
    #[inline]
    pub fn assch7(&mut self) -> _ASSCH7W {
        _ASSCH7W { w: self }
    }
}
