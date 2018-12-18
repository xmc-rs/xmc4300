#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PCR_IISMODE {
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
#[doc = "Possible values of the field `WAGEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WAGENR {
    #[doc = "The IIS can be used as slave. The generation of the word address signal is disabled. The output signal WA is 0. The MCLKO signal generation depends on PCR.MCLK."]
    VALUE1,
    #[doc = "The IIS can be used as master. The generation of the word address signal is enabled. The signal starts with a 0 after being enabled. The generation of MCLK is enabled, independent of PCR.MCLK. After clearing WAGEN, the USIC module stops the generation of the WA signal within the next 4 WA periods."]
    VALUE2,
}
impl WAGENR {
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
            WAGENR::VALUE1 => false,
            WAGENR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WAGENR {
        match value {
            false => WAGENR::VALUE1,
            true => WAGENR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == WAGENR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == WAGENR::VALUE2
    }
}
#[doc = "Possible values of the field `DTEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DTENR {
    #[doc = "The changes of the WA input signal are ignored and no transfers take place."]
    VALUE1,
    #[doc = "Transfers are enabled."]
    VALUE2,
}
impl DTENR {
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
            DTENR::VALUE1 => false,
            DTENR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DTENR {
        match value {
            false => DTENR::VALUE1,
            true => DTENR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == DTENR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == DTENR::VALUE2
    }
}
#[doc = "Possible values of the field `SELINV`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SELINVR {
    #[doc = "The SELOx outputs have the same polarity as the WA signal."]
    VALUE1,
    #[doc = "The SELOx outputs have the inverted polarity to the WA signal."]
    VALUE2,
}
impl SELINVR {
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
            SELINVR::VALUE1 => false,
            SELINVR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SELINVR {
        match value {
            false => SELINVR::VALUE1,
            true => SELINVR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == SELINVR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == SELINVR::VALUE2
    }
}
#[doc = "Possible values of the field `WAFEIEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WAFEIENR {
    #[doc = "A protocol interrupt is not activated if a falling edge of WA is generated."]
    VALUE1,
    #[doc = "A protocol interrupt is activated if a falling edge of WA is generated."]
    VALUE2,
}
impl WAFEIENR {
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
            WAFEIENR::VALUE1 => false,
            WAFEIENR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WAFEIENR {
        match value {
            false => WAFEIENR::VALUE1,
            true => WAFEIENR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == WAFEIENR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == WAFEIENR::VALUE2
    }
}
#[doc = "Possible values of the field `WAREIEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WAREIENR {
    #[doc = "A protocol interrupt is not activated if a rising edge of WA is generated."]
    VALUE1,
    #[doc = "A protocol interrupt is activated if a rising edge of WA is generated."]
    VALUE2,
}
impl WAREIENR {
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
            WAREIENR::VALUE1 => false,
            WAREIENR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WAREIENR {
        match value {
            false => WAREIENR::VALUE1,
            true => WAREIENR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == WAREIENR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == WAREIENR::VALUE2
    }
}
#[doc = "Possible values of the field `ENDIEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENDIENR {
    #[doc = "A protocol interrupt is not activated."]
    VALUE1,
    #[doc = "A protocol interrupt is activated."]
    VALUE2,
}
impl ENDIENR {
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
            ENDIENR::VALUE1 => false,
            ENDIENR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ENDIENR {
        match value {
            false => ENDIENR::VALUE1,
            true => ENDIENR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == ENDIENR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == ENDIENR::VALUE2
    }
}
#[doc = "Possible values of the field `DX2TIEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DX2TIENR {
    #[doc = "A protocol interrupt is not generated if DX2T is active."]
    VALUE1,
    #[doc = "A protocol interrupt is generated if DX2T is active."]
    VALUE2,
}
impl DX2TIENR {
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
            DX2TIENR::VALUE1 => false,
            DX2TIENR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DX2TIENR {
        match value {
            false => DX2TIENR::VALUE1,
            true => DX2TIENR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == DX2TIENR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == DX2TIENR::VALUE2
    }
}
#[doc = r" Value of the field"]
pub struct TDELR {
    bits: u8,
}
impl TDELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `MCLK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MCLKR {
    #[doc = "The MCLK generation is disabled and MCLK is 0."]
    VALUE1,
    #[doc = "The MCLK generation is enabled."]
    VALUE2,
}
impl MCLKR {
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
            MCLKR::VALUE1 => false,
            MCLKR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MCLKR {
        match value {
            false => MCLKR::VALUE1,
            true => MCLKR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == MCLKR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == MCLKR::VALUE2
    }
}
#[doc = "Values that can be written to the field `WAGEN`"]
pub enum WAGENW {
    #[doc = "The IIS can be used as slave. The generation of the word address signal is disabled. The output signal WA is 0. The MCLKO signal generation depends on PCR.MCLK."]
    VALUE1,
    #[doc = "The IIS can be used as master. The generation of the word address signal is enabled. The signal starts with a 0 after being enabled. The generation of MCLK is enabled, independent of PCR.MCLK. After clearing WAGEN, the USIC module stops the generation of the WA signal within the next 4 WA periods."]
    VALUE2,
}
impl WAGENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WAGENW::VALUE1 => false,
            WAGENW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WAGENW<'a> {
    w: &'a mut W,
}
impl<'a> _WAGENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WAGENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The IIS can be used as slave. The generation of the word address signal is disabled. The output signal WA is 0. The MCLKO signal generation depends on PCR.MCLK."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(WAGENW::VALUE1)
    }
    #[doc = "The IIS can be used as master. The generation of the word address signal is enabled. The signal starts with a 0 after being enabled. The generation of MCLK is enabled, independent of PCR.MCLK. After clearing WAGEN, the USIC module stops the generation of the WA signal within the next 4 WA periods."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(WAGENW::VALUE2)
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
#[doc = "Values that can be written to the field `DTEN`"]
pub enum DTENW {
    #[doc = "The changes of the WA input signal are ignored and no transfers take place."]
    VALUE1,
    #[doc = "Transfers are enabled."]
    VALUE2,
}
impl DTENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DTENW::VALUE1 => false,
            DTENW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DTENW<'a> {
    w: &'a mut W,
}
impl<'a> _DTENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DTENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The changes of the WA input signal are ignored and no transfers take place."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(DTENW::VALUE1)
    }
    #[doc = "Transfers are enabled."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(DTENW::VALUE2)
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
#[doc = "Values that can be written to the field `SELINV`"]
pub enum SELINVW {
    #[doc = "The SELOx outputs have the same polarity as the WA signal."]
    VALUE1,
    #[doc = "The SELOx outputs have the inverted polarity to the WA signal."]
    VALUE2,
}
impl SELINVW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SELINVW::VALUE1 => false,
            SELINVW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SELINVW<'a> {
    w: &'a mut W,
}
impl<'a> _SELINVW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SELINVW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The SELOx outputs have the same polarity as the WA signal."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(SELINVW::VALUE1)
    }
    #[doc = "The SELOx outputs have the inverted polarity to the WA signal."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(SELINVW::VALUE2)
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
#[doc = "Values that can be written to the field `WAFEIEN`"]
pub enum WAFEIENW {
    #[doc = "A protocol interrupt is not activated if a falling edge of WA is generated."]
    VALUE1,
    #[doc = "A protocol interrupt is activated if a falling edge of WA is generated."]
    VALUE2,
}
impl WAFEIENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WAFEIENW::VALUE1 => false,
            WAFEIENW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WAFEIENW<'a> {
    w: &'a mut W,
}
impl<'a> _WAFEIENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WAFEIENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "A protocol interrupt is not activated if a falling edge of WA is generated."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(WAFEIENW::VALUE1)
    }
    #[doc = "A protocol interrupt is activated if a falling edge of WA is generated."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(WAFEIENW::VALUE2)
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
#[doc = "Values that can be written to the field `WAREIEN`"]
pub enum WAREIENW {
    #[doc = "A protocol interrupt is not activated if a rising edge of WA is generated."]
    VALUE1,
    #[doc = "A protocol interrupt is activated if a rising edge of WA is generated."]
    VALUE2,
}
impl WAREIENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WAREIENW::VALUE1 => false,
            WAREIENW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WAREIENW<'a> {
    w: &'a mut W,
}
impl<'a> _WAREIENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WAREIENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "A protocol interrupt is not activated if a rising edge of WA is generated."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(WAREIENW::VALUE1)
    }
    #[doc = "A protocol interrupt is activated if a rising edge of WA is generated."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(WAREIENW::VALUE2)
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
#[doc = "Values that can be written to the field `ENDIEN`"]
pub enum ENDIENW {
    #[doc = "A protocol interrupt is not activated."]
    VALUE1,
    #[doc = "A protocol interrupt is activated."]
    VALUE2,
}
impl ENDIENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ENDIENW::VALUE1 => false,
            ENDIENW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ENDIENW<'a> {
    w: &'a mut W,
}
impl<'a> _ENDIENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ENDIENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "A protocol interrupt is not activated."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(ENDIENW::VALUE1)
    }
    #[doc = "A protocol interrupt is activated."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(ENDIENW::VALUE2)
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
#[doc = "Values that can be written to the field `DX2TIEN`"]
pub enum DX2TIENW {
    #[doc = "A protocol interrupt is not generated if DX2T is active."]
    VALUE1,
    #[doc = "A protocol interrupt is generated if DX2T is active."]
    VALUE2,
}
impl DX2TIENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DX2TIENW::VALUE1 => false,
            DX2TIENW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DX2TIENW<'a> {
    w: &'a mut W,
}
impl<'a> _DX2TIENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DX2TIENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "A protocol interrupt is not generated if DX2T is active."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(DX2TIENW::VALUE1)
    }
    #[doc = "A protocol interrupt is generated if DX2T is active."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(DX2TIENW::VALUE2)
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
#[doc = r" Proxy"]
pub struct _TDELW<'a> {
    w: &'a mut W,
}
impl<'a> _TDELW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 63;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `MCLK`"]
pub enum MCLKW {
    #[doc = "The MCLK generation is disabled and MCLK is 0."]
    VALUE1,
    #[doc = "The MCLK generation is enabled."]
    VALUE2,
}
impl MCLKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MCLKW::VALUE1 => false,
            MCLKW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MCLKW<'a> {
    w: &'a mut W,
}
impl<'a> _MCLKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MCLKW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The MCLK generation is disabled and MCLK is 0."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(MCLKW::VALUE1)
    }
    #[doc = "The MCLK generation is enabled."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(MCLKW::VALUE2)
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
        const OFFSET: u8 = 31;
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
    #[doc = "Bit 0 - WA Generation Enable"]
    #[inline]
    pub fn wagen(&self) -> WAGENR {
        WAGENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Data Transfers Enable"]
    #[inline]
    pub fn dten(&self) -> DTENR {
        DTENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Select Inversion"]
    #[inline]
    pub fn selinv(&self) -> SELINVR {
        SELINVR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - WA Falling Edge Interrupt Enable"]
    #[inline]
    pub fn wafeien(&self) -> WAFEIENR {
        WAFEIENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - WA Rising Edge Interrupt Enable"]
    #[inline]
    pub fn wareien(&self) -> WAREIENR {
        WAREIENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - END Interrupt Enable"]
    #[inline]
    pub fn endien(&self) -> ENDIENR {
        ENDIENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 15 - DX2T Interrupt Enable"]
    #[inline]
    pub fn dx2tien(&self) -> DX2TIENR {
        DX2TIENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 16:21 - Transfer Delay"]
    #[inline]
    pub fn tdel(&self) -> TDELR {
        let bits = {
            const MASK: u8 = 63;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        TDELR { bits }
    }
    #[doc = "Bit 31 - Master Clock Enable"]
    #[inline]
    pub fn mclk(&self) -> MCLKR {
        MCLKR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 31;
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
    #[doc = "Bit 0 - WA Generation Enable"]
    #[inline]
    pub fn wagen(&mut self) -> _WAGENW {
        _WAGENW { w: self }
    }
    #[doc = "Bit 1 - Data Transfers Enable"]
    #[inline]
    pub fn dten(&mut self) -> _DTENW {
        _DTENW { w: self }
    }
    #[doc = "Bit 2 - Select Inversion"]
    #[inline]
    pub fn selinv(&mut self) -> _SELINVW {
        _SELINVW { w: self }
    }
    #[doc = "Bit 4 - WA Falling Edge Interrupt Enable"]
    #[inline]
    pub fn wafeien(&mut self) -> _WAFEIENW {
        _WAFEIENW { w: self }
    }
    #[doc = "Bit 5 - WA Rising Edge Interrupt Enable"]
    #[inline]
    pub fn wareien(&mut self) -> _WAREIENW {
        _WAREIENW { w: self }
    }
    #[doc = "Bit 6 - END Interrupt Enable"]
    #[inline]
    pub fn endien(&mut self) -> _ENDIENW {
        _ENDIENW { w: self }
    }
    #[doc = "Bit 15 - DX2T Interrupt Enable"]
    #[inline]
    pub fn dx2tien(&mut self) -> _DX2TIENW {
        _DX2TIENW { w: self }
    }
    #[doc = "Bits 16:21 - Transfer Delay"]
    #[inline]
    pub fn tdel(&mut self) -> _TDELW {
        _TDELW { w: self }
    }
    #[doc = "Bit 31 - Master Clock Enable"]
    #[inline]
    pub fn mclk(&mut self) -> _MCLKW {
        _MCLKW { w: self }
    }
}
