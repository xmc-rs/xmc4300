#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::ARBPR {
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
#[doc = "Possible values of the field `PRIO0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PRIO0R {
    #[doc = "Lowest priority is selected."]
    VALUE1,
    #[doc = "Highest priority is selected."]
    VALUE2,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl PRIO0R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PRIO0R::VALUE1 => 0,
            PRIO0R::VALUE2 => 3,
            PRIO0R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PRIO0R {
        match value {
            0 => PRIO0R::VALUE1,
            3 => PRIO0R::VALUE2,
            i => PRIO0R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == PRIO0R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == PRIO0R::VALUE2
    }
}
#[doc = "Possible values of the field `PRIO1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PRIO1R {
    #[doc = "Lowest priority is selected."]
    VALUE1,
    #[doc = "Highest priority is selected."]
    VALUE2,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl PRIO1R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PRIO1R::VALUE1 => 0,
            PRIO1R::VALUE2 => 3,
            PRIO1R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PRIO1R {
        match value {
            0 => PRIO1R::VALUE1,
            3 => PRIO1R::VALUE2,
            i => PRIO1R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == PRIO1R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == PRIO1R::VALUE2
    }
}
#[doc = "Possible values of the field `PRIO2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PRIO2R {
    #[doc = "Lowest priority is selected."]
    VALUE1,
    #[doc = "Highest priority is selected."]
    VALUE2,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl PRIO2R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PRIO2R::VALUE1 => 0,
            PRIO2R::VALUE2 => 3,
            PRIO2R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PRIO2R {
        match value {
            0 => PRIO2R::VALUE1,
            3 => PRIO2R::VALUE2,
            i => PRIO2R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == PRIO2R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == PRIO2R::VALUE2
    }
}
#[doc = "Possible values of the field `CSM0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CSM0R {
    #[doc = "Wait-for-start mode"]
    VALUE1,
    #[doc = "Cancel-inject-repeat mode, i.e. this source can cancel conversion of other sources."]
    VALUE2,
}
impl CSM0R {
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
            CSM0R::VALUE1 => false,
            CSM0R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CSM0R {
        match value {
            false => CSM0R::VALUE1,
            true => CSM0R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == CSM0R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == CSM0R::VALUE2
    }
}
#[doc = "Possible values of the field `CSM1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CSM1R {
    #[doc = "Wait-for-start mode"]
    VALUE1,
    #[doc = "Cancel-inject-repeat mode, i.e. this source can cancel conversion of other sources."]
    VALUE2,
}
impl CSM1R {
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
            CSM1R::VALUE1 => false,
            CSM1R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CSM1R {
        match value {
            false => CSM1R::VALUE1,
            true => CSM1R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == CSM1R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == CSM1R::VALUE2
    }
}
#[doc = "Possible values of the field `CSM2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CSM2R {
    #[doc = "Wait-for-start mode"]
    VALUE1,
    #[doc = "Cancel-inject-repeat mode, i.e. this source can cancel conversion of other sources."]
    VALUE2,
}
impl CSM2R {
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
            CSM2R::VALUE1 => false,
            CSM2R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CSM2R {
        match value {
            false => CSM2R::VALUE1,
            true => CSM2R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == CSM2R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == CSM2R::VALUE2
    }
}
#[doc = "Possible values of the field `ASEN0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ASEN0R {
    #[doc = "The corresponding arbitration slot is disabled and considered as empty. Pending conversion requests from the associated request source are disregarded."]
    VALUE1,
    #[doc = "The corresponding arbitration slot is enabled. Pending conversion requests from the associated request source are arbitrated."]
    VALUE2,
}
impl ASEN0R {
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
            ASEN0R::VALUE1 => false,
            ASEN0R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ASEN0R {
        match value {
            false => ASEN0R::VALUE1,
            true => ASEN0R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == ASEN0R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == ASEN0R::VALUE2
    }
}
#[doc = "Possible values of the field `ASEN1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ASEN1R {
    #[doc = "The corresponding arbitration slot is disabled and considered as empty. Pending conversion requests from the associated request source are disregarded."]
    VALUE1,
    #[doc = "The corresponding arbitration slot is enabled. Pending conversion requests from the associated request source are arbitrated."]
    VALUE2,
}
impl ASEN1R {
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
            ASEN1R::VALUE1 => false,
            ASEN1R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ASEN1R {
        match value {
            false => ASEN1R::VALUE1,
            true => ASEN1R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == ASEN1R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == ASEN1R::VALUE2
    }
}
#[doc = "Possible values of the field `ASEN2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ASEN2R {
    #[doc = "The corresponding arbitration slot is disabled and considered as empty. Pending conversion requests from the associated request source are disregarded."]
    VALUE1,
    #[doc = "The corresponding arbitration slot is enabled. Pending conversion requests from the associated request source are arbitrated."]
    VALUE2,
}
impl ASEN2R {
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
            ASEN2R::VALUE1 => false,
            ASEN2R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ASEN2R {
        match value {
            false => ASEN2R::VALUE1,
            true => ASEN2R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == ASEN2R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == ASEN2R::VALUE2
    }
}
#[doc = "Values that can be written to the field `PRIO0`"]
pub enum PRIO0W {
    #[doc = "Lowest priority is selected."]
    VALUE1,
    #[doc = "Highest priority is selected."]
    VALUE2,
}
impl PRIO0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PRIO0W::VALUE1 => 0,
            PRIO0W::VALUE2 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PRIO0W<'a> {
    w: &'a mut W,
}
impl<'a> _PRIO0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PRIO0W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Lowest priority is selected."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(PRIO0W::VALUE1)
    }
    #[doc = "Highest priority is selected."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(PRIO0W::VALUE2)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PRIO1`"]
pub enum PRIO1W {
    #[doc = "Lowest priority is selected."]
    VALUE1,
    #[doc = "Highest priority is selected."]
    VALUE2,
}
impl PRIO1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PRIO1W::VALUE1 => 0,
            PRIO1W::VALUE2 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PRIO1W<'a> {
    w: &'a mut W,
}
impl<'a> _PRIO1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PRIO1W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Lowest priority is selected."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(PRIO1W::VALUE1)
    }
    #[doc = "Highest priority is selected."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(PRIO1W::VALUE2)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PRIO2`"]
pub enum PRIO2W {
    #[doc = "Lowest priority is selected."]
    VALUE1,
    #[doc = "Highest priority is selected."]
    VALUE2,
}
impl PRIO2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PRIO2W::VALUE1 => 0,
            PRIO2W::VALUE2 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PRIO2W<'a> {
    w: &'a mut W,
}
impl<'a> _PRIO2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PRIO2W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Lowest priority is selected."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(PRIO2W::VALUE1)
    }
    #[doc = "Highest priority is selected."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(PRIO2W::VALUE2)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CSM0`"]
pub enum CSM0W {
    #[doc = "Wait-for-start mode"]
    VALUE1,
    #[doc = "Cancel-inject-repeat mode, i.e. this source can cancel conversion of other sources."]
    VALUE2,
}
impl CSM0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CSM0W::VALUE1 => false,
            CSM0W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CSM0W<'a> {
    w: &'a mut W,
}
impl<'a> _CSM0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CSM0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Wait-for-start mode"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(CSM0W::VALUE1)
    }
    #[doc = "Cancel-inject-repeat mode, i.e. this source can cancel conversion of other sources."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(CSM0W::VALUE2)
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
#[doc = "Values that can be written to the field `CSM1`"]
pub enum CSM1W {
    #[doc = "Wait-for-start mode"]
    VALUE1,
    #[doc = "Cancel-inject-repeat mode, i.e. this source can cancel conversion of other sources."]
    VALUE2,
}
impl CSM1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CSM1W::VALUE1 => false,
            CSM1W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CSM1W<'a> {
    w: &'a mut W,
}
impl<'a> _CSM1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CSM1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Wait-for-start mode"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(CSM1W::VALUE1)
    }
    #[doc = "Cancel-inject-repeat mode, i.e. this source can cancel conversion of other sources."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(CSM1W::VALUE2)
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
#[doc = "Values that can be written to the field `CSM2`"]
pub enum CSM2W {
    #[doc = "Wait-for-start mode"]
    VALUE1,
    #[doc = "Cancel-inject-repeat mode, i.e. this source can cancel conversion of other sources."]
    VALUE2,
}
impl CSM2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CSM2W::VALUE1 => false,
            CSM2W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CSM2W<'a> {
    w: &'a mut W,
}
impl<'a> _CSM2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CSM2W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Wait-for-start mode"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(CSM2W::VALUE1)
    }
    #[doc = "Cancel-inject-repeat mode, i.e. this source can cancel conversion of other sources."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(CSM2W::VALUE2)
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
#[doc = "Values that can be written to the field `ASEN0`"]
pub enum ASEN0W {
    #[doc = "The corresponding arbitration slot is disabled and considered as empty. Pending conversion requests from the associated request source are disregarded."]
    VALUE1,
    #[doc = "The corresponding arbitration slot is enabled. Pending conversion requests from the associated request source are arbitrated."]
    VALUE2,
}
impl ASEN0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ASEN0W::VALUE1 => false,
            ASEN0W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ASEN0W<'a> {
    w: &'a mut W,
}
impl<'a> _ASEN0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ASEN0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The corresponding arbitration slot is disabled and considered as empty. Pending conversion requests from the associated request source are disregarded."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(ASEN0W::VALUE1)
    }
    #[doc = "The corresponding arbitration slot is enabled. Pending conversion requests from the associated request source are arbitrated."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(ASEN0W::VALUE2)
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
#[doc = "Values that can be written to the field `ASEN1`"]
pub enum ASEN1W {
    #[doc = "The corresponding arbitration slot is disabled and considered as empty. Pending conversion requests from the associated request source are disregarded."]
    VALUE1,
    #[doc = "The corresponding arbitration slot is enabled. Pending conversion requests from the associated request source are arbitrated."]
    VALUE2,
}
impl ASEN1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ASEN1W::VALUE1 => false,
            ASEN1W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ASEN1W<'a> {
    w: &'a mut W,
}
impl<'a> _ASEN1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ASEN1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The corresponding arbitration slot is disabled and considered as empty. Pending conversion requests from the associated request source are disregarded."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(ASEN1W::VALUE1)
    }
    #[doc = "The corresponding arbitration slot is enabled. Pending conversion requests from the associated request source are arbitrated."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(ASEN1W::VALUE2)
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
#[doc = "Values that can be written to the field `ASEN2`"]
pub enum ASEN2W {
    #[doc = "The corresponding arbitration slot is disabled and considered as empty. Pending conversion requests from the associated request source are disregarded."]
    VALUE1,
    #[doc = "The corresponding arbitration slot is enabled. Pending conversion requests from the associated request source are arbitrated."]
    VALUE2,
}
impl ASEN2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ASEN2W::VALUE1 => false,
            ASEN2W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ASEN2W<'a> {
    w: &'a mut W,
}
impl<'a> _ASEN2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ASEN2W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The corresponding arbitration slot is disabled and considered as empty. Pending conversion requests from the associated request source are disregarded."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(ASEN2W::VALUE1)
    }
    #[doc = "The corresponding arbitration slot is enabled. Pending conversion requests from the associated request source are arbitrated."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(ASEN2W::VALUE2)
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
        const OFFSET: u8 = 26;
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
    #[doc = "Bits 0:1 - Priority of Request Source x"]
    #[inline]
    pub fn prio0(&self) -> PRIO0R {
        PRIO0R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 4:5 - Priority of Request Source x"]
    #[inline]
    pub fn prio1(&self) -> PRIO1R {
        PRIO1R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 8:9 - Priority of Request Source x"]
    #[inline]
    pub fn prio2(&self) -> PRIO2R {
        PRIO2R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 3 - Conversion Start Mode of Request Source x"]
    #[inline]
    pub fn csm0(&self) -> CSM0R {
        CSM0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - Conversion Start Mode of Request Source x"]
    #[inline]
    pub fn csm1(&self) -> CSM1R {
        CSM1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - Conversion Start Mode of Request Source x"]
    #[inline]
    pub fn csm2(&self) -> CSM2R {
        CSM2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 24 - Arbitration Slot 0 Enable"]
    #[inline]
    pub fn asen0(&self) -> ASEN0R {
        ASEN0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 25 - Arbitration Slot 1 Enable"]
    #[inline]
    pub fn asen1(&self) -> ASEN1R {
        ASEN1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 26 - Arbitration Slot 2 Enable"]
    #[inline]
    pub fn asen2(&self) -> ASEN2R {
        ASEN2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 26;
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
    #[doc = "Bits 0:1 - Priority of Request Source x"]
    #[inline]
    pub fn prio0(&mut self) -> _PRIO0W {
        _PRIO0W { w: self }
    }
    #[doc = "Bits 4:5 - Priority of Request Source x"]
    #[inline]
    pub fn prio1(&mut self) -> _PRIO1W {
        _PRIO1W { w: self }
    }
    #[doc = "Bits 8:9 - Priority of Request Source x"]
    #[inline]
    pub fn prio2(&mut self) -> _PRIO2W {
        _PRIO2W { w: self }
    }
    #[doc = "Bit 3 - Conversion Start Mode of Request Source x"]
    #[inline]
    pub fn csm0(&mut self) -> _CSM0W {
        _CSM0W { w: self }
    }
    #[doc = "Bit 7 - Conversion Start Mode of Request Source x"]
    #[inline]
    pub fn csm1(&mut self) -> _CSM1W {
        _CSM1W { w: self }
    }
    #[doc = "Bit 11 - Conversion Start Mode of Request Source x"]
    #[inline]
    pub fn csm2(&mut self) -> _CSM2W {
        _CSM2W { w: self }
    }
    #[doc = "Bit 24 - Arbitration Slot 0 Enable"]
    #[inline]
    pub fn asen0(&mut self) -> _ASEN0W {
        _ASEN0W { w: self }
    }
    #[doc = "Bit 25 - Arbitration Slot 1 Enable"]
    #[inline]
    pub fn asen1(&mut self) -> _ASEN1W {
        _ASEN1W { w: self }
    }
    #[doc = "Bit 26 - Arbitration Slot 2 Enable"]
    #[inline]
    pub fn asen2(&mut self) -> _ASEN2W {
        _ASEN2W { w: self }
    }
}
