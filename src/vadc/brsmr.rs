#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::BRSMR {
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
#[doc = "Possible values of the field `ENGT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENGTR {
    #[doc = "No conversion requests are issued"]
    VALUE1,
    #[doc = "Conversion requests are issued if at least one pending bit is set"]
    VALUE2,
    #[doc = "Conversion requests are issued if at least one pending bit is set and REQGTx = 1."]
    VALUE3,
    #[doc = "Conversion requests are issued if at least one pending bit is set and REQGTx = 0."]
    VALUE4,
}
impl ENGTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            ENGTR::VALUE1 => 0,
            ENGTR::VALUE2 => 1,
            ENGTR::VALUE3 => 2,
            ENGTR::VALUE4 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> ENGTR {
        match value {
            0 => ENGTR::VALUE1,
            1 => ENGTR::VALUE2,
            2 => ENGTR::VALUE3,
            3 => ENGTR::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == ENGTR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == ENGTR::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == ENGTR::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == ENGTR::VALUE4
    }
}
#[doc = "Possible values of the field `ENTR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENTRR {
    #[doc = "External trigger disabled"]
    VALUE1,
    #[doc = "The selected edge at the selected trigger input signal REQTR generates the load event"]
    VALUE2,
}
impl ENTRR {
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
            ENTRR::VALUE1 => false,
            ENTRR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ENTRR {
        match value {
            false => ENTRR::VALUE1,
            true => ENTRR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == ENTRR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == ENTRR::VALUE2
    }
}
#[doc = "Possible values of the field `ENSI`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENSIR {
    #[doc = "No request source interrupt"]
    VALUE1,
    #[doc = "A request source interrupt is generated upon a request source event (last pending conversion is finished)"]
    VALUE2,
}
impl ENSIR {
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
            ENSIR::VALUE1 => false,
            ENSIR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ENSIR {
        match value {
            false => ENSIR::VALUE1,
            true => ENSIR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == ENSIR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == ENSIR::VALUE2
    }
}
#[doc = "Possible values of the field `SCAN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SCANR {
    #[doc = "No autoscan"]
    VALUE1,
    #[doc = "Autoscan functionality enabled: a request source event automatically generates a load event"]
    VALUE2,
}
impl SCANR {
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
            SCANR::VALUE1 => false,
            SCANR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SCANR {
        match value {
            false => SCANR::VALUE1,
            true => SCANR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == SCANR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == SCANR::VALUE2
    }
}
#[doc = "Possible values of the field `LDM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LDMR {
    #[doc = "Overwrite mode: Copy all bits from the select registers to the pending registers upon a load event"]
    VALUE1,
    #[doc = "Combine mode: Set all pending bits that are set in the select registers upon a load event (logic OR)"]
    VALUE2,
}
impl LDMR {
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
            LDMR::VALUE1 => false,
            LDMR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LDMR {
        match value {
            false => LDMR::VALUE1,
            true => LDMR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == LDMR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == LDMR::VALUE2
    }
}
#[doc = "Possible values of the field `REQGT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REQGTR {
    #[doc = "The gate input is low"]
    VALUE1,
    #[doc = "The gate input is high"]
    VALUE2,
}
impl REQGTR {
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
            REQGTR::VALUE1 => false,
            REQGTR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> REQGTR {
        match value {
            false => REQGTR::VALUE1,
            true => REQGTR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == REQGTR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == REQGTR::VALUE2
    }
}
#[doc = "Possible values of the field `RPTDIS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RPTDISR {
    #[doc = "A cancelled conversion is repeated"]
    VALUE1,
    #[doc = "A cancelled conversion is discarded"]
    VALUE2,
}
impl RPTDISR {
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
            RPTDISR::VALUE1 => false,
            RPTDISR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RPTDISR {
        match value {
            false => RPTDISR::VALUE1,
            true => RPTDISR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == RPTDISR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == RPTDISR::VALUE2
    }
}
#[doc = "Values that can be written to the field `ENGT`"]
pub enum ENGTW {
    #[doc = "No conversion requests are issued"]
    VALUE1,
    #[doc = "Conversion requests are issued if at least one pending bit is set"]
    VALUE2,
    #[doc = "Conversion requests are issued if at least one pending bit is set and REQGTx = 1."]
    VALUE3,
    #[doc = "Conversion requests are issued if at least one pending bit is set and REQGTx = 0."]
    VALUE4,
}
impl ENGTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            ENGTW::VALUE1 => 0,
            ENGTW::VALUE2 => 1,
            ENGTW::VALUE3 => 2,
            ENGTW::VALUE4 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ENGTW<'a> {
    w: &'a mut W,
}
impl<'a> _ENGTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ENGTW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "No conversion requests are issued"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(ENGTW::VALUE1)
    }
    #[doc = "Conversion requests are issued if at least one pending bit is set"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(ENGTW::VALUE2)
    }
    #[doc = "Conversion requests are issued if at least one pending bit is set and REQGTx = 1."]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(ENGTW::VALUE3)
    }
    #[doc = "Conversion requests are issued if at least one pending bit is set and REQGTx = 0."]
    #[inline]
    pub fn value4(self) -> &'a mut W {
        self.variant(ENGTW::VALUE4)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ENTR`"]
pub enum ENTRW {
    #[doc = "External trigger disabled"]
    VALUE1,
    #[doc = "The selected edge at the selected trigger input signal REQTR generates the load event"]
    VALUE2,
}
impl ENTRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ENTRW::VALUE1 => false,
            ENTRW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ENTRW<'a> {
    w: &'a mut W,
}
impl<'a> _ENTRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ENTRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "External trigger disabled"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(ENTRW::VALUE1)
    }
    #[doc = "The selected edge at the selected trigger input signal REQTR generates the load event"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(ENTRW::VALUE2)
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
#[doc = "Values that can be written to the field `ENSI`"]
pub enum ENSIW {
    #[doc = "No request source interrupt"]
    VALUE1,
    #[doc = "A request source interrupt is generated upon a request source event (last pending conversion is finished)"]
    VALUE2,
}
impl ENSIW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ENSIW::VALUE1 => false,
            ENSIW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ENSIW<'a> {
    w: &'a mut W,
}
impl<'a> _ENSIW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ENSIW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No request source interrupt"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(ENSIW::VALUE1)
    }
    #[doc = "A request source interrupt is generated upon a request source event (last pending conversion is finished)"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(ENSIW::VALUE2)
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
#[doc = "Values that can be written to the field `SCAN`"]
pub enum SCANW {
    #[doc = "No autoscan"]
    VALUE1,
    #[doc = "Autoscan functionality enabled: a request source event automatically generates a load event"]
    VALUE2,
}
impl SCANW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SCANW::VALUE1 => false,
            SCANW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SCANW<'a> {
    w: &'a mut W,
}
impl<'a> _SCANW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SCANW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No autoscan"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(SCANW::VALUE1)
    }
    #[doc = "Autoscan functionality enabled: a request source event automatically generates a load event"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(SCANW::VALUE2)
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
#[doc = "Values that can be written to the field `LDM`"]
pub enum LDMW {
    #[doc = "Overwrite mode: Copy all bits from the select registers to the pending registers upon a load event"]
    VALUE1,
    #[doc = "Combine mode: Set all pending bits that are set in the select registers upon a load event (logic OR)"]
    VALUE2,
}
impl LDMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LDMW::VALUE1 => false,
            LDMW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LDMW<'a> {
    w: &'a mut W,
}
impl<'a> _LDMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LDMW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Overwrite mode: Copy all bits from the select registers to the pending registers upon a load event"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(LDMW::VALUE1)
    }
    #[doc = "Combine mode: Set all pending bits that are set in the select registers upon a load event (logic OR)"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(LDMW::VALUE2)
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
#[doc = "Values that can be written to the field `CLRPND`"]
pub enum CLRPNDW {
    #[doc = "No action"]
    VALUE1,
    #[doc = "The bits in registers BRSPNDx are cleared"]
    VALUE2,
}
impl CLRPNDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CLRPNDW::VALUE1 => false,
            CLRPNDW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CLRPNDW<'a> {
    w: &'a mut W,
}
impl<'a> _CLRPNDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CLRPNDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No action"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(CLRPNDW::VALUE1)
    }
    #[doc = "The bits in registers BRSPNDx are cleared"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(CLRPNDW::VALUE2)
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
#[doc = "Values that can be written to the field `LDEV`"]
pub enum LDEVW {
    #[doc = "No action"]
    VALUE1,
    #[doc = "A load event is generated"]
    VALUE2,
}
impl LDEVW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LDEVW::VALUE1 => false,
            LDEVW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LDEVW<'a> {
    w: &'a mut W,
}
impl<'a> _LDEVW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LDEVW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No action"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(LDEVW::VALUE1)
    }
    #[doc = "A load event is generated"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(LDEVW::VALUE2)
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
#[doc = "Values that can be written to the field `RPTDIS`"]
pub enum RPTDISW {
    #[doc = "A cancelled conversion is repeated"]
    VALUE1,
    #[doc = "A cancelled conversion is discarded"]
    VALUE2,
}
impl RPTDISW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RPTDISW::VALUE1 => false,
            RPTDISW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RPTDISW<'a> {
    w: &'a mut W,
}
impl<'a> _RPTDISW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RPTDISW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "A cancelled conversion is repeated"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(RPTDISW::VALUE1)
    }
    #[doc = "A cancelled conversion is discarded"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(RPTDISW::VALUE2)
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
    #[doc = "Bits 0:1 - Enable Gate"]
    #[inline]
    pub fn engt(&self) -> ENGTR {
        ENGTR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 2 - Enable External Trigger"]
    #[inline]
    pub fn entr(&self) -> ENTRR {
        ENTRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Enable Source Interrupt"]
    #[inline]
    pub fn ensi(&self) -> ENSIR {
        ENSIR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Autoscan Enable"]
    #[inline]
    pub fn scan(&self) -> SCANR {
        SCANR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Autoscan Source Load Event Mode"]
    #[inline]
    pub fn ldm(&self) -> LDMR {
        LDMR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - Request Gate Level"]
    #[inline]
    pub fn reqgt(&self) -> REQGTR {
        REQGTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 16 - Repeat Disable"]
    #[inline]
    pub fn rptdis(&self) -> RPTDISR {
        RPTDISR::_from({
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
    #[doc = "Bits 0:1 - Enable Gate"]
    #[inline]
    pub fn engt(&mut self) -> _ENGTW {
        _ENGTW { w: self }
    }
    #[doc = "Bit 2 - Enable External Trigger"]
    #[inline]
    pub fn entr(&mut self) -> _ENTRW {
        _ENTRW { w: self }
    }
    #[doc = "Bit 3 - Enable Source Interrupt"]
    #[inline]
    pub fn ensi(&mut self) -> _ENSIW {
        _ENSIW { w: self }
    }
    #[doc = "Bit 4 - Autoscan Enable"]
    #[inline]
    pub fn scan(&mut self) -> _SCANW {
        _SCANW { w: self }
    }
    #[doc = "Bit 5 - Autoscan Source Load Event Mode"]
    #[inline]
    pub fn ldm(&mut self) -> _LDMW {
        _LDMW { w: self }
    }
    #[doc = "Bit 8 - Clear Pending Bits"]
    #[inline]
    pub fn clrpnd(&mut self) -> _CLRPNDW {
        _CLRPNDW { w: self }
    }
    #[doc = "Bit 9 - Generate Load Event"]
    #[inline]
    pub fn ldev(&mut self) -> _LDEVW {
        _LDEVW { w: self }
    }
    #[doc = "Bit 16 - Repeat Disable"]
    #[inline]
    pub fn rptdis(&mut self) -> _RPTDISW {
        _RPTDISW { w: self }
    }
}
