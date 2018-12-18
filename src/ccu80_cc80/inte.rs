#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::INTE {
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
#[doc = "Possible values of the field `PME`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PMER {
    #[doc = "Period Match interrupt is disabled"]
    VALUE1,
    #[doc = "Period Match interrupt is enabled"]
    VALUE2,
}
impl PMER {
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
            PMER::VALUE1 => false,
            PMER::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PMER {
        match value {
            false => PMER::VALUE1,
            true => PMER::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == PMER::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == PMER::VALUE2
    }
}
#[doc = "Possible values of the field `OME`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OMER {
    #[doc = "One Match interrupt is disabled"]
    VALUE1,
    #[doc = "One Match interrupt is enabled"]
    VALUE2,
}
impl OMER {
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
            OMER::VALUE1 => false,
            OMER::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> OMER {
        match value {
            false => OMER::VALUE1,
            true => OMER::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == OMER::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == OMER::VALUE2
    }
}
#[doc = "Possible values of the field `CMU1E`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CMU1ER {
    #[doc = "Compare Match while counting up interrupt is disabled"]
    VALUE1,
    #[doc = "Compare Match while counting up interrupt is enabled"]
    VALUE2,
}
impl CMU1ER {
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
            CMU1ER::VALUE1 => false,
            CMU1ER::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CMU1ER {
        match value {
            false => CMU1ER::VALUE1,
            true => CMU1ER::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == CMU1ER::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == CMU1ER::VALUE2
    }
}
#[doc = "Possible values of the field `CMD1E`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CMD1ER {
    #[doc = "Compare Match while counting down interrupt is disabled"]
    VALUE1,
    #[doc = "Compare Match while counting down interrupt is enabled"]
    VALUE2,
}
impl CMD1ER {
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
            CMD1ER::VALUE1 => false,
            CMD1ER::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CMD1ER {
        match value {
            false => CMD1ER::VALUE1,
            true => CMD1ER::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == CMD1ER::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == CMD1ER::VALUE2
    }
}
#[doc = "Possible values of the field `CMU2E`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CMU2ER {
    #[doc = "Compare Match while counting up interrupt is disabled"]
    VALUE1,
    #[doc = "Compare Match while counting up interrupt is enabled"]
    VALUE2,
}
impl CMU2ER {
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
            CMU2ER::VALUE1 => false,
            CMU2ER::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CMU2ER {
        match value {
            false => CMU2ER::VALUE1,
            true => CMU2ER::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == CMU2ER::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == CMU2ER::VALUE2
    }
}
#[doc = "Possible values of the field `CMD2E`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CMD2ER {
    #[doc = "Compare Match while counting down interrupt is disabled"]
    VALUE1,
    #[doc = "Compare Match while counting down interrupt is enabled"]
    VALUE2,
}
impl CMD2ER {
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
            CMD2ER::VALUE1 => false,
            CMD2ER::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CMD2ER {
        match value {
            false => CMD2ER::VALUE1,
            true => CMD2ER::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == CMD2ER::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == CMD2ER::VALUE2
    }
}
#[doc = "Possible values of the field `E0AE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum E0AER {
    #[doc = "Event 0 detection interrupt is disabled"]
    VALUE1,
    #[doc = "Event 0 detection interrupt is enabled"]
    VALUE2,
}
impl E0AER {
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
            E0AER::VALUE1 => false,
            E0AER::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> E0AER {
        match value {
            false => E0AER::VALUE1,
            true => E0AER::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == E0AER::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == E0AER::VALUE2
    }
}
#[doc = "Possible values of the field `E1AE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum E1AER {
    #[doc = "Event 1 detection interrupt is disabled"]
    VALUE1,
    #[doc = "Event 1 detection interrupt is enabled"]
    VALUE2,
}
impl E1AER {
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
            E1AER::VALUE1 => false,
            E1AER::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> E1AER {
        match value {
            false => E1AER::VALUE1,
            true => E1AER::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == E1AER::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == E1AER::VALUE2
    }
}
#[doc = "Possible values of the field `E2AE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum E2AER {
    #[doc = "Event 2 detection interrupt is disabled"]
    VALUE1,
    #[doc = "Event 2 detection interrupt is enabled"]
    VALUE2,
}
impl E2AER {
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
            E2AER::VALUE1 => false,
            E2AER::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> E2AER {
        match value {
            false => E2AER::VALUE1,
            true => E2AER::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == E2AER::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == E2AER::VALUE2
    }
}
#[doc = "Values that can be written to the field `PME`"]
pub enum PMEW {
    #[doc = "Period Match interrupt is disabled"]
    VALUE1,
    #[doc = "Period Match interrupt is enabled"]
    VALUE2,
}
impl PMEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PMEW::VALUE1 => false,
            PMEW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PMEW<'a> {
    w: &'a mut W,
}
impl<'a> _PMEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PMEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Period Match interrupt is disabled"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(PMEW::VALUE1)
    }
    #[doc = "Period Match interrupt is enabled"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(PMEW::VALUE2)
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
#[doc = "Values that can be written to the field `OME`"]
pub enum OMEW {
    #[doc = "One Match interrupt is disabled"]
    VALUE1,
    #[doc = "One Match interrupt is enabled"]
    VALUE2,
}
impl OMEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            OMEW::VALUE1 => false,
            OMEW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _OMEW<'a> {
    w: &'a mut W,
}
impl<'a> _OMEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: OMEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "One Match interrupt is disabled"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(OMEW::VALUE1)
    }
    #[doc = "One Match interrupt is enabled"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(OMEW::VALUE2)
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
#[doc = "Values that can be written to the field `CMU1E`"]
pub enum CMU1EW {
    #[doc = "Compare Match while counting up interrupt is disabled"]
    VALUE1,
    #[doc = "Compare Match while counting up interrupt is enabled"]
    VALUE2,
}
impl CMU1EW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CMU1EW::VALUE1 => false,
            CMU1EW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CMU1EW<'a> {
    w: &'a mut W,
}
impl<'a> _CMU1EW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CMU1EW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Compare Match while counting up interrupt is disabled"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(CMU1EW::VALUE1)
    }
    #[doc = "Compare Match while counting up interrupt is enabled"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(CMU1EW::VALUE2)
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
#[doc = "Values that can be written to the field `CMD1E`"]
pub enum CMD1EW {
    #[doc = "Compare Match while counting down interrupt is disabled"]
    VALUE1,
    #[doc = "Compare Match while counting down interrupt is enabled"]
    VALUE2,
}
impl CMD1EW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CMD1EW::VALUE1 => false,
            CMD1EW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CMD1EW<'a> {
    w: &'a mut W,
}
impl<'a> _CMD1EW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CMD1EW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Compare Match while counting down interrupt is disabled"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(CMD1EW::VALUE1)
    }
    #[doc = "Compare Match while counting down interrupt is enabled"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(CMD1EW::VALUE2)
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
#[doc = "Values that can be written to the field `CMU2E`"]
pub enum CMU2EW {
    #[doc = "Compare Match while counting up interrupt is disabled"]
    VALUE1,
    #[doc = "Compare Match while counting up interrupt is enabled"]
    VALUE2,
}
impl CMU2EW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CMU2EW::VALUE1 => false,
            CMU2EW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CMU2EW<'a> {
    w: &'a mut W,
}
impl<'a> _CMU2EW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CMU2EW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Compare Match while counting up interrupt is disabled"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(CMU2EW::VALUE1)
    }
    #[doc = "Compare Match while counting up interrupt is enabled"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(CMU2EW::VALUE2)
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
#[doc = "Values that can be written to the field `CMD2E`"]
pub enum CMD2EW {
    #[doc = "Compare Match while counting down interrupt is disabled"]
    VALUE1,
    #[doc = "Compare Match while counting down interrupt is enabled"]
    VALUE2,
}
impl CMD2EW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CMD2EW::VALUE1 => false,
            CMD2EW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CMD2EW<'a> {
    w: &'a mut W,
}
impl<'a> _CMD2EW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CMD2EW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Compare Match while counting down interrupt is disabled"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(CMD2EW::VALUE1)
    }
    #[doc = "Compare Match while counting down interrupt is enabled"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(CMD2EW::VALUE2)
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
#[doc = "Values that can be written to the field `E0AE`"]
pub enum E0AEW {
    #[doc = "Event 0 detection interrupt is disabled"]
    VALUE1,
    #[doc = "Event 0 detection interrupt is enabled"]
    VALUE2,
}
impl E0AEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            E0AEW::VALUE1 => false,
            E0AEW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _E0AEW<'a> {
    w: &'a mut W,
}
impl<'a> _E0AEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: E0AEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Event 0 detection interrupt is disabled"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(E0AEW::VALUE1)
    }
    #[doc = "Event 0 detection interrupt is enabled"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(E0AEW::VALUE2)
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
#[doc = "Values that can be written to the field `E1AE`"]
pub enum E1AEW {
    #[doc = "Event 1 detection interrupt is disabled"]
    VALUE1,
    #[doc = "Event 1 detection interrupt is enabled"]
    VALUE2,
}
impl E1AEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            E1AEW::VALUE1 => false,
            E1AEW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _E1AEW<'a> {
    w: &'a mut W,
}
impl<'a> _E1AEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: E1AEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Event 1 detection interrupt is disabled"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(E1AEW::VALUE1)
    }
    #[doc = "Event 1 detection interrupt is enabled"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(E1AEW::VALUE2)
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
#[doc = "Values that can be written to the field `E2AE`"]
pub enum E2AEW {
    #[doc = "Event 2 detection interrupt is disabled"]
    VALUE1,
    #[doc = "Event 2 detection interrupt is enabled"]
    VALUE2,
}
impl E2AEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            E2AEW::VALUE1 => false,
            E2AEW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _E2AEW<'a> {
    w: &'a mut W,
}
impl<'a> _E2AEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: E2AEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Event 2 detection interrupt is disabled"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(E2AEW::VALUE1)
    }
    #[doc = "Event 2 detection interrupt is enabled"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(E2AEW::VALUE2)
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
    #[doc = "Bit 0 - Period match while counting up enable"]
    #[inline]
    pub fn pme(&self) -> PMER {
        PMER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - One match while counting down enable"]
    #[inline]
    pub fn ome(&self) -> OMER {
        OMER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Channel 1 Compare match while counting up enable"]
    #[inline]
    pub fn cmu1e(&self) -> CMU1ER {
        CMU1ER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Channel 1 Compare match while counting down enable"]
    #[inline]
    pub fn cmd1e(&self) -> CMD1ER {
        CMD1ER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Channel 2 Compare match while counting up enable"]
    #[inline]
    pub fn cmu2e(&self) -> CMU2ER {
        CMU2ER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Channel 2 Compare match while counting down enable"]
    #[inline]
    pub fn cmd2e(&self) -> CMD2ER {
        CMD2ER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Event 0 interrupt enable"]
    #[inline]
    pub fn e0ae(&self) -> E0AER {
        E0AER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - Event 1 interrupt enable"]
    #[inline]
    pub fn e1ae(&self) -> E1AER {
        E1AER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - Event 2 interrupt enable"]
    #[inline]
    pub fn e2ae(&self) -> E2AER {
        E2AER::_from({
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
        W { bits: 0 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Period match while counting up enable"]
    #[inline]
    pub fn pme(&mut self) -> _PMEW {
        _PMEW { w: self }
    }
    #[doc = "Bit 1 - One match while counting down enable"]
    #[inline]
    pub fn ome(&mut self) -> _OMEW {
        _OMEW { w: self }
    }
    #[doc = "Bit 2 - Channel 1 Compare match while counting up enable"]
    #[inline]
    pub fn cmu1e(&mut self) -> _CMU1EW {
        _CMU1EW { w: self }
    }
    #[doc = "Bit 3 - Channel 1 Compare match while counting down enable"]
    #[inline]
    pub fn cmd1e(&mut self) -> _CMD1EW {
        _CMD1EW { w: self }
    }
    #[doc = "Bit 4 - Channel 2 Compare match while counting up enable"]
    #[inline]
    pub fn cmu2e(&mut self) -> _CMU2EW {
        _CMU2EW { w: self }
    }
    #[doc = "Bit 5 - Channel 2 Compare match while counting down enable"]
    #[inline]
    pub fn cmd2e(&mut self) -> _CMD2EW {
        _CMD2EW { w: self }
    }
    #[doc = "Bit 8 - Event 0 interrupt enable"]
    #[inline]
    pub fn e0ae(&mut self) -> _E0AEW {
        _E0AEW { w: self }
    }
    #[doc = "Bit 9 - Event 1 interrupt enable"]
    #[inline]
    pub fn e1ae(&mut self) -> _E1AEW {
        _E1AEW { w: self }
    }
    #[doc = "Bit 10 - Event 2 interrupt enable"]
    #[inline]
    pub fn e2ae(&mut self) -> _E2AEW {
        _E2AEW { w: self }
    }
}
