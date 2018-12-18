#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PSR_SSCMODE {
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
#[doc = "Possible values of the field `MSLS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MSLSR {
    #[doc = "The internal signal MSLS is inactive (0)."]
    VALUE1,
    #[doc = "The internal signal MSLS is active (1)."]
    VALUE2,
}
impl MSLSR {
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
            MSLSR::VALUE1 => false,
            MSLSR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MSLSR {
        match value {
            false => MSLSR::VALUE1,
            true => MSLSR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == MSLSR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == MSLSR::VALUE2
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
#[doc = "Possible values of the field `MSLSEV`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MSLSEVR {
    #[doc = "The MSLS signal has not changed its state."]
    VALUE1,
    #[doc = "The MSLS signal has changed its state."]
    VALUE2,
}
impl MSLSEVR {
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
            MSLSEVR::VALUE1 => false,
            MSLSEVR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MSLSEVR {
        match value {
            false => MSLSEVR::VALUE1,
            true => MSLSEVR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == MSLSEVR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == MSLSEVR::VALUE2
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
#[doc = "Possible values of the field `PARERR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PARERRR {
    #[doc = "A parity error event has not been activated."]
    VALUE1,
    #[doc = "A parity error event has been activated."]
    VALUE2,
}
impl PARERRR {
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
            PARERRR::VALUE1 => false,
            PARERRR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PARERRR {
        match value {
            false => PARERRR::VALUE1,
            true => PARERRR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == PARERRR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == PARERRR::VALUE2
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
#[doc = "Values that can be written to the field `MSLS`"]
pub enum MSLSW {
    #[doc = "The internal signal MSLS is inactive (0)."]
    VALUE1,
    #[doc = "The internal signal MSLS is active (1)."]
    VALUE2,
}
impl MSLSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MSLSW::VALUE1 => false,
            MSLSW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MSLSW<'a> {
    w: &'a mut W,
}
impl<'a> _MSLSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MSLSW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The internal signal MSLS is inactive (0)."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(MSLSW::VALUE1)
    }
    #[doc = "The internal signal MSLS is active (1)."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(MSLSW::VALUE2)
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
#[doc = "Values that can be written to the field `MSLSEV`"]
pub enum MSLSEVW {
    #[doc = "The MSLS signal has not changed its state."]
    VALUE1,
    #[doc = "The MSLS signal has changed its state."]
    VALUE2,
}
impl MSLSEVW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MSLSEVW::VALUE1 => false,
            MSLSEVW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MSLSEVW<'a> {
    w: &'a mut W,
}
impl<'a> _MSLSEVW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MSLSEVW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The MSLS signal has not changed its state."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(MSLSEVW::VALUE1)
    }
    #[doc = "The MSLS signal has changed its state."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(MSLSEVW::VALUE2)
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
#[doc = "Values that can be written to the field `PARERR`"]
pub enum PARERRW {
    #[doc = "A parity error event has not been activated."]
    VALUE1,
    #[doc = "A parity error event has been activated."]
    VALUE2,
}
impl PARERRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PARERRW::VALUE1 => false,
            PARERRW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PARERRW<'a> {
    w: &'a mut W,
}
impl<'a> _PARERRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PARERRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "A parity error event has not been activated."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(PARERRW::VALUE1)
    }
    #[doc = "A parity error event has been activated."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(PARERRW::VALUE2)
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
    #[doc = "Bit 0 - MSLS Status"]
    #[inline]
    pub fn msls(&self) -> MSLSR {
        MSLSR::_from({
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
    #[doc = "Bit 2 - MSLS Event Detected"]
    #[inline]
    pub fn mslsev(&self) -> MSLSEVR {
        MSLSEVR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
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
    #[doc = "Bit 4 - Parity Error Event Detected"]
    #[inline]
    pub fn parerr(&self) -> PARERRR {
        PARERRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
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
    #[doc = "Bit 0 - MSLS Status"]
    #[inline]
    pub fn msls(&mut self) -> _MSLSW {
        _MSLSW { w: self }
    }
    #[doc = "Bit 1 - DX2S Status"]
    #[inline]
    pub fn dx2s(&mut self) -> _DX2SW {
        _DX2SW { w: self }
    }
    #[doc = "Bit 2 - MSLS Event Detected"]
    #[inline]
    pub fn mslsev(&mut self) -> _MSLSEVW {
        _MSLSEVW { w: self }
    }
    #[doc = "Bit 3 - DX2T Event Detected"]
    #[inline]
    pub fn dx2tev(&mut self) -> _DX2TEVW {
        _DX2TEVW { w: self }
    }
    #[doc = "Bit 4 - Parity Error Event Detected"]
    #[inline]
    pub fn parerr(&mut self) -> _PARERRW {
        _PARERRW { w: self }
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
