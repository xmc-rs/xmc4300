#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PSR_IISMODE {
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
#[doc = "Possible values of the field `WA`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WAR {
    #[doc = "WA has been sampled 0."]
    VALUE1,
    #[doc = "WA has been sampled 1."]
    VALUE2,
}
impl WAR {
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
            WAR::VALUE1 => false,
            WAR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WAR {
        match value {
            false => WAR::VALUE1,
            true => WAR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == WAR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == WAR::VALUE2
    }
}
#[doc = "Possible values of the field `DX2S`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DX2SR {
    #[doc = "DX2S is 0."]
    VALUE1,
    #[doc = "DX2S is 1."]
    VALUE2,
}
impl DX2SR {
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
            DX2SR::VALUE1 => false,
            DX2SR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DX2SR {
        match value {
            false => DX2SR::VALUE1,
            true => DX2SR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == DX2SR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == DX2SR::VALUE2
    }
}
#[doc = "Possible values of the field `DX2TEV`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DX2TEVR {
    #[doc = "The DX2T signal has not been activated."]
    VALUE1,
    #[doc = "The DX2T signal has been activated."]
    VALUE2,
}
impl DX2TEVR {
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
            DX2TEVR::VALUE1 => false,
            DX2TEVR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DX2TEVR {
        match value {
            false => DX2TEVR::VALUE1,
            true => DX2TEVR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == DX2TEVR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == DX2TEVR::VALUE2
    }
}
#[doc = "Possible values of the field `WAFE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WAFER {
    #[doc = "A WA falling edge has not been generated."]
    VALUE1,
    #[doc = "A WA falling edge has been generated."]
    VALUE2,
}
impl WAFER {
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
            WAFER::VALUE1 => false,
            WAFER::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WAFER {
        match value {
            false => WAFER::VALUE1,
            true => WAFER::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == WAFER::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == WAFER::VALUE2
    }
}
#[doc = "Possible values of the field `WARE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WARER {
    #[doc = "A WA rising edge has not been generated."]
    VALUE1,
    #[doc = "A WA rising edge has been generated."]
    VALUE2,
}
impl WARER {
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
            WARER::VALUE1 => false,
            WARER::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WARER {
        match value {
            false => WARER::VALUE1,
            true => WARER::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == WARER::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == WARER::VALUE2
    }
}
#[doc = "Possible values of the field `END`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENDR {
    #[doc = "The WA generation has not yet ended (if it is running and WAGEN has been cleared)."]
    VALUE1,
    #[doc = "The WA generation has ended (if it has been running)."]
    VALUE2,
}
impl ENDR {
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
            ENDR::VALUE1 => false,
            ENDR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ENDR {
        match value {
            false => ENDR::VALUE1,
            true => ENDR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == ENDR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == ENDR::VALUE2
    }
}
#[doc = "Possible values of the field `RSIF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RSIFR {
    #[doc = "A receiver start event has not occurred."]
    VALUE1,
    #[doc = "A receiver start event has occurred."]
    VALUE2,
}
impl RSIFR {
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
            RSIFR::VALUE1 => false,
            RSIFR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RSIFR {
        match value {
            false => RSIFR::VALUE1,
            true => RSIFR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == RSIFR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == RSIFR::VALUE2
    }
}
#[doc = "Possible values of the field `DLIF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DLIFR {
    #[doc = "A data lost event has not occurred."]
    VALUE1,
    #[doc = "A data lost event has occurred."]
    VALUE2,
}
impl DLIFR {
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
            DLIFR::VALUE1 => false,
            DLIFR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DLIFR {
        match value {
            false => DLIFR::VALUE1,
            true => DLIFR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == DLIFR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == DLIFR::VALUE2
    }
}
#[doc = "Possible values of the field `TSIF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TSIFR {
    #[doc = "A transmit shift event has not occurred."]
    VALUE1,
    #[doc = "A transmit shift event has occurred."]
    VALUE2,
}
impl TSIFR {
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
            TSIFR::VALUE1 => false,
            TSIFR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TSIFR {
        match value {
            false => TSIFR::VALUE1,
            true => TSIFR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == TSIFR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == TSIFR::VALUE2
    }
}
#[doc = "Possible values of the field `TBIF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TBIFR {
    #[doc = "A transmit buffer event has not occurred."]
    VALUE1,
    #[doc = "A transmit buffer event has occurred."]
    VALUE2,
}
impl TBIFR {
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
            TBIFR::VALUE1 => false,
            TBIFR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TBIFR {
        match value {
            false => TBIFR::VALUE1,
            true => TBIFR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == TBIFR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == TBIFR::VALUE2
    }
}
#[doc = "Possible values of the field `RIF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RIFR {
    #[doc = "A receive event has not occurred."]
    VALUE1,
    #[doc = "A receive event has occurred."]
    VALUE2,
}
impl RIFR {
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
            RIFR::VALUE1 => false,
            RIFR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RIFR {
        match value {
            false => RIFR::VALUE1,
            true => RIFR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == RIFR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == RIFR::VALUE2
    }
}
#[doc = "Possible values of the field `AIF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AIFR {
    #[doc = "An alternative receive event has not occurred."]
    VALUE1,
    #[doc = "An alternative receive event has occurred."]
    VALUE2,
}
impl AIFR {
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
            AIFR::VALUE1 => false,
            AIFR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> AIFR {
        match value {
            false => AIFR::VALUE1,
            true => AIFR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == AIFR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == AIFR::VALUE2
    }
}
#[doc = "Possible values of the field `BRGIF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BRGIFR {
    #[doc = "A baud rate generator event has not occurred."]
    VALUE1,
    #[doc = "A baud rate generator event has occurred."]
    VALUE2,
}
impl BRGIFR {
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
            BRGIFR::VALUE1 => false,
            BRGIFR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BRGIFR {
        match value {
            false => BRGIFR::VALUE1,
            true => BRGIFR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == BRGIFR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == BRGIFR::VALUE2
    }
}
#[doc = "Values that can be written to the field `WA`"]
pub enum WAW {
    #[doc = "WA has been sampled 0."]
    VALUE1,
    #[doc = "WA has been sampled 1."]
    VALUE2,
}
impl WAW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WAW::VALUE1 => false,
            WAW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WAW<'a> {
    w: &'a mut W,
}
impl<'a> _WAW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WAW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "WA has been sampled 0."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(WAW::VALUE1)
    }
    #[doc = "WA has been sampled 1."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(WAW::VALUE2)
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
#[doc = "Values that can be written to the field `DX2S`"]
pub enum DX2SW {
    #[doc = "DX2S is 0."]
    VALUE1,
    #[doc = "DX2S is 1."]
    VALUE2,
}
impl DX2SW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DX2SW::VALUE1 => false,
            DX2SW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DX2SW<'a> {
    w: &'a mut W,
}
impl<'a> _DX2SW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DX2SW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "DX2S is 0."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(DX2SW::VALUE1)
    }
    #[doc = "DX2S is 1."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(DX2SW::VALUE2)
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
#[doc = "Values that can be written to the field `DX2TEV`"]
pub enum DX2TEVW {
    #[doc = "The DX2T signal has not been activated."]
    VALUE1,
    #[doc = "The DX2T signal has been activated."]
    VALUE2,
}
impl DX2TEVW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DX2TEVW::VALUE1 => false,
            DX2TEVW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DX2TEVW<'a> {
    w: &'a mut W,
}
impl<'a> _DX2TEVW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DX2TEVW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The DX2T signal has not been activated."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(DX2TEVW::VALUE1)
    }
    #[doc = "The DX2T signal has been activated."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(DX2TEVW::VALUE2)
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
#[doc = "Values that can be written to the field `WAFE`"]
pub enum WAFEW {
    #[doc = "A WA falling edge has not been generated."]
    VALUE1,
    #[doc = "A WA falling edge has been generated."]
    VALUE2,
}
impl WAFEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WAFEW::VALUE1 => false,
            WAFEW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WAFEW<'a> {
    w: &'a mut W,
}
impl<'a> _WAFEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WAFEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "A WA falling edge has not been generated."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(WAFEW::VALUE1)
    }
    #[doc = "A WA falling edge has been generated."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(WAFEW::VALUE2)
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
#[doc = "Values that can be written to the field `WARE`"]
pub enum WAREW {
    #[doc = "A WA rising edge has not been generated."]
    VALUE1,
    #[doc = "A WA rising edge has been generated."]
    VALUE2,
}
impl WAREW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WAREW::VALUE1 => false,
            WAREW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WAREW<'a> {
    w: &'a mut W,
}
impl<'a> _WAREW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WAREW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "A WA rising edge has not been generated."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(WAREW::VALUE1)
    }
    #[doc = "A WA rising edge has been generated."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(WAREW::VALUE2)
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
#[doc = "Values that can be written to the field `END`"]
pub enum ENDW {
    #[doc = "The WA generation has not yet ended (if it is running and WAGEN has been cleared)."]
    VALUE1,
    #[doc = "The WA generation has ended (if it has been running)."]
    VALUE2,
}
impl ENDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ENDW::VALUE1 => false,
            ENDW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ENDW<'a> {
    w: &'a mut W,
}
impl<'a> _ENDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ENDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The WA generation has not yet ended (if it is running and WAGEN has been cleared)."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(ENDW::VALUE1)
    }
    #[doc = "The WA generation has ended (if it has been running)."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(ENDW::VALUE2)
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
#[doc = "Values that can be written to the field `RSIF`"]
pub enum RSIFW {
    #[doc = "A receiver start event has not occurred."]
    VALUE1,
    #[doc = "A receiver start event has occurred."]
    VALUE2,
}
impl RSIFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RSIFW::VALUE1 => false,
            RSIFW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RSIFW<'a> {
    w: &'a mut W,
}
impl<'a> _RSIFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RSIFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "A receiver start event has not occurred."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(RSIFW::VALUE1)
    }
    #[doc = "A receiver start event has occurred."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(RSIFW::VALUE2)
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
#[doc = "Values that can be written to the field `DLIF`"]
pub enum DLIFW {
    #[doc = "A data lost event has not occurred."]
    VALUE1,
    #[doc = "A data lost event has occurred."]
    VALUE2,
}
impl DLIFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DLIFW::VALUE1 => false,
            DLIFW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DLIFW<'a> {
    w: &'a mut W,
}
impl<'a> _DLIFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DLIFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "A data lost event has not occurred."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(DLIFW::VALUE1)
    }
    #[doc = "A data lost event has occurred."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(DLIFW::VALUE2)
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
#[doc = "Values that can be written to the field `TSIF`"]
pub enum TSIFW {
    #[doc = "A transmit shift event has not occurred."]
    VALUE1,
    #[doc = "A transmit shift event has occurred."]
    VALUE2,
}
impl TSIFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TSIFW::VALUE1 => false,
            TSIFW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TSIFW<'a> {
    w: &'a mut W,
}
impl<'a> _TSIFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TSIFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "A transmit shift event has not occurred."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(TSIFW::VALUE1)
    }
    #[doc = "A transmit shift event has occurred."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(TSIFW::VALUE2)
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
#[doc = "Values that can be written to the field `TBIF`"]
pub enum TBIFW {
    #[doc = "A transmit buffer event has not occurred."]
    VALUE1,
    #[doc = "A transmit buffer event has occurred."]
    VALUE2,
}
impl TBIFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TBIFW::VALUE1 => false,
            TBIFW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TBIFW<'a> {
    w: &'a mut W,
}
impl<'a> _TBIFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TBIFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "A transmit buffer event has not occurred."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(TBIFW::VALUE1)
    }
    #[doc = "A transmit buffer event has occurred."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(TBIFW::VALUE2)
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
#[doc = "Values that can be written to the field `RIF`"]
pub enum RIFW {
    #[doc = "A receive event has not occurred."]
    VALUE1,
    #[doc = "A receive event has occurred."]
    VALUE2,
}
impl RIFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RIFW::VALUE1 => false,
            RIFW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RIFW<'a> {
    w: &'a mut W,
}
impl<'a> _RIFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RIFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "A receive event has not occurred."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(RIFW::VALUE1)
    }
    #[doc = "A receive event has occurred."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(RIFW::VALUE2)
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
        const OFFSET: u8 = 14;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `AIF`"]
pub enum AIFW {
    #[doc = "An alternative receive event has not occurred."]
    VALUE1,
    #[doc = "An alternative receive event has occurred."]
    VALUE2,
}
impl AIFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            AIFW::VALUE1 => false,
            AIFW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _AIFW<'a> {
    w: &'a mut W,
}
impl<'a> _AIFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: AIFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "An alternative receive event has not occurred."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(AIFW::VALUE1)
    }
    #[doc = "An alternative receive event has occurred."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(AIFW::VALUE2)
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
        const OFFSET: u8 = 15;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `BRGIF`"]
pub enum BRGIFW {
    #[doc = "A baud rate generator event has not occurred."]
    VALUE1,
    #[doc = "A baud rate generator event has occurred."]
    VALUE2,
}
impl BRGIFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BRGIFW::VALUE1 => false,
            BRGIFW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BRGIFW<'a> {
    w: &'a mut W,
}
impl<'a> _BRGIFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BRGIFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "A baud rate generator event has not occurred."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(BRGIFW::VALUE1)
    }
    #[doc = "A baud rate generator event has occurred."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(BRGIFW::VALUE2)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Word Address"]
    #[inline]
    pub fn wa(&self) -> WAR {
        WAR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - DX2S Status"]
    #[inline]
    pub fn dx2s(&self) -> DX2SR {
        DX2SR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - DX2T Event Detected"]
    #[inline]
    pub fn dx2tev(&self) -> DX2TEVR {
        DX2TEVR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - WA Falling Edge Event"]
    #[inline]
    pub fn wafe(&self) -> WAFER {
        WAFER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - WA Rising Edge Event"]
    #[inline]
    pub fn ware(&self) -> WARER {
        WARER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - WA Generation End"]
    #[inline]
    pub fn end(&self) -> ENDR {
        ENDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - Receiver Start Indication Flag"]
    #[inline]
    pub fn rsif(&self) -> RSIFR {
        RSIFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - Data Lost Indication Flag"]
    #[inline]
    pub fn dlif(&self) -> DLIFR {
        DLIFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 12 - Transmit Shift Indication Flag"]
    #[inline]
    pub fn tsif(&self) -> TSIFR {
        TSIFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 13 - Transmit Buffer Indication Flag"]
    #[inline]
    pub fn tbif(&self) -> TBIFR {
        TBIFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 14 - Receive Indication Flag"]
    #[inline]
    pub fn rif(&self) -> RIFR {
        RIFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 15 - Alternative Receive Indication Flag"]
    #[inline]
    pub fn aif(&self) -> AIFR {
        AIFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 16 - Baud Rate Generator Indication Flag"]
    #[inline]
    pub fn brgif(&self) -> BRGIFR {
        BRGIFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
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
    #[doc = "Bit 0 - Word Address"]
    #[inline]
    pub fn wa(&mut self) -> _WAW {
        _WAW { w: self }
    }
    #[doc = "Bit 1 - DX2S Status"]
    #[inline]
    pub fn dx2s(&mut self) -> _DX2SW {
        _DX2SW { w: self }
    }
    #[doc = "Bit 3 - DX2T Event Detected"]
    #[inline]
    pub fn dx2tev(&mut self) -> _DX2TEVW {
        _DX2TEVW { w: self }
    }
    #[doc = "Bit 4 - WA Falling Edge Event"]
    #[inline]
    pub fn wafe(&mut self) -> _WAFEW {
        _WAFEW { w: self }
    }
    #[doc = "Bit 5 - WA Rising Edge Event"]
    #[inline]
    pub fn ware(&mut self) -> _WAREW {
        _WAREW { w: self }
    }
    #[doc = "Bit 6 - WA Generation End"]
    #[inline]
    pub fn end(&mut self) -> _ENDW {
        _ENDW { w: self }
    }
    #[doc = "Bit 10 - Receiver Start Indication Flag"]
    #[inline]
    pub fn rsif(&mut self) -> _RSIFW {
        _RSIFW { w: self }
    }
    #[doc = "Bit 11 - Data Lost Indication Flag"]
    #[inline]
    pub fn dlif(&mut self) -> _DLIFW {
        _DLIFW { w: self }
    }
    #[doc = "Bit 12 - Transmit Shift Indication Flag"]
    #[inline]
    pub fn tsif(&mut self) -> _TSIFW {
        _TSIFW { w: self }
    }
    #[doc = "Bit 13 - Transmit Buffer Indication Flag"]
    #[inline]
    pub fn tbif(&mut self) -> _TBIFW {
        _TBIFW { w: self }
    }
    #[doc = "Bit 14 - Receive Indication Flag"]
    #[inline]
    pub fn rif(&mut self) -> _RIFW {
        _RIFW { w: self }
    }
    #[doc = "Bit 15 - Alternative Receive Indication Flag"]
    #[inline]
    pub fn aif(&mut self) -> _AIFW {
        _AIFW { w: self }
    }
    #[doc = "Bit 16 - Baud Rate Generator Indication Flag"]
    #[inline]
    pub fn brgif(&mut self) -> _BRGIFW {
        _BRGIFW { w: self }
    }
}
