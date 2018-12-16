#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::EXOCON {
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
#[doc = "Possible values of the field `ISS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ISSR {
    #[doc = "The peripheral trigger function is disabled"]
    VALUE1,
    #[doc = "Input ERU_OGUy1 is selected"]
    VALUE2,
    #[doc = "Input ERU_OGUy2 is selected"]
    VALUE3,
    #[doc = "Input ERU_OGUy3 is selected"]
    VALUE4,
}
impl ISSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            ISSR::VALUE1 => 0,
            ISSR::VALUE2 => 1,
            ISSR::VALUE3 => 2,
            ISSR::VALUE4 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> ISSR {
        match value {
            0 => ISSR::VALUE1,
            1 => ISSR::VALUE2,
            2 => ISSR::VALUE3,
            3 => ISSR::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == ISSR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == ISSR::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == ISSR::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == ISSR::VALUE4
    }
}
#[doc = "Possible values of the field `GEEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GEENR {
    #[doc = "The event detection is disabled"]
    VALUE1,
    #[doc = "The event detection is enabled"]
    VALUE2,
}
impl GEENR {
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
            GEENR::VALUE1 => false,
            GEENR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> GEENR {
        match value {
            false => GEENR::VALUE1,
            true => GEENR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == GEENR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == GEENR::VALUE2
    }
}
#[doc = "Possible values of the field `PDR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDRR {
    #[doc = "A pattern miss is detected"]
    VALUE1,
    #[doc = "A pattern match is detected"]
    VALUE2,
}
impl PDRR {
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
            PDRR::VALUE1 => false,
            PDRR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PDRR {
        match value {
            false => PDRR::VALUE1,
            true => PDRR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == PDRR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == PDRR::VALUE2
    }
}
#[doc = "Possible values of the field `GP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPR {
    #[doc = "ERU_GOUTy is always disabled and ERU_IOUTy can not be activated"]
    VALUE1,
    #[doc = "ERU_GOUTy is always enabled and ERU_IOUTy becomes activated with each activation of ERU_TOUTy"]
    VALUE2,
    #[doc = "ERU_GOUTy is equal to ERU_PDOUTy and ERU_IOUTy becomes activated with an activation of ERU_TOUTy while the desired pattern is detected (pattern match PDR = 1)"]
    VALUE3,
    #[doc = "ERU_GOUTy is inverted to ERU_PDOUTy and ERU_IOUTy becomes activated with an activation of ERU_TOUTy while the desired pattern is not detected (pattern miss PDR = 0)"]
    VALUE4,
}
impl GPR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            GPR::VALUE1 => 0,
            GPR::VALUE2 => 1,
            GPR::VALUE3 => 2,
            GPR::VALUE4 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> GPR {
        match value {
            0 => GPR::VALUE1,
            1 => GPR::VALUE2,
            2 => GPR::VALUE3,
            3 => GPR::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == GPR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == GPR::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == GPR::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == GPR::VALUE4
    }
}
#[doc = "Possible values of the field `IPEN0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IPEN0R {
    #[doc = "Flag EXICONx.FL is excluded from the pattern detection"]
    VALUE1,
    #[doc = "Flag EXICONx.FL is included in the pattern detection"]
    VALUE2,
}
impl IPEN0R {
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
            IPEN0R::VALUE1 => false,
            IPEN0R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> IPEN0R {
        match value {
            false => IPEN0R::VALUE1,
            true => IPEN0R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == IPEN0R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == IPEN0R::VALUE2
    }
}
#[doc = "Possible values of the field `IPEN1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IPEN1R {
    #[doc = "Flag EXICONx.FL is excluded from the pattern detection"]
    VALUE1,
    #[doc = "Flag EXICONx.FL is included in the pattern detection"]
    VALUE2,
}
impl IPEN1R {
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
            IPEN1R::VALUE1 => false,
            IPEN1R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> IPEN1R {
        match value {
            false => IPEN1R::VALUE1,
            true => IPEN1R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == IPEN1R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == IPEN1R::VALUE2
    }
}
#[doc = "Possible values of the field `IPEN2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IPEN2R {
    #[doc = "Flag EXICONx.FL is excluded from the pattern detection"]
    VALUE1,
    #[doc = "Flag EXICONx.FL is included in the pattern detection"]
    VALUE2,
}
impl IPEN2R {
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
            IPEN2R::VALUE1 => false,
            IPEN2R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> IPEN2R {
        match value {
            false => IPEN2R::VALUE1,
            true => IPEN2R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == IPEN2R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == IPEN2R::VALUE2
    }
}
#[doc = "Possible values of the field `IPEN3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IPEN3R {
    #[doc = "Flag EXICONx.FL is excluded from the pattern detection"]
    VALUE1,
    #[doc = "Flag EXICONx.FL is included in the pattern detection"]
    VALUE2,
}
impl IPEN3R {
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
            IPEN3R::VALUE1 => false,
            IPEN3R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> IPEN3R {
        match value {
            false => IPEN3R::VALUE1,
            true => IPEN3R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == IPEN3R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == IPEN3R::VALUE2
    }
}
#[doc = "Values that can be written to the field `ISS`"]
pub enum ISSW {
    #[doc = "The peripheral trigger function is disabled"]
    VALUE1,
    #[doc = "Input ERU_OGUy1 is selected"]
    VALUE2,
    #[doc = "Input ERU_OGUy2 is selected"]
    VALUE3,
    #[doc = "Input ERU_OGUy3 is selected"]
    VALUE4,
}
impl ISSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            ISSW::VALUE1 => 0,
            ISSW::VALUE2 => 1,
            ISSW::VALUE3 => 2,
            ISSW::VALUE4 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ISSW<'a> {
    w: &'a mut W,
}
impl<'a> _ISSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ISSW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "The peripheral trigger function is disabled"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(ISSW::VALUE1)
    }
    #[doc = "Input ERU_OGUy1 is selected"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(ISSW::VALUE2)
    }
    #[doc = "Input ERU_OGUy2 is selected"]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(ISSW::VALUE3)
    }
    #[doc = "Input ERU_OGUy3 is selected"]
    #[inline]
    pub fn value4(self) -> &'a mut W {
        self.variant(ISSW::VALUE4)
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
#[doc = "Values that can be written to the field `GEEN`"]
pub enum GEENW {
    #[doc = "The event detection is disabled"]
    VALUE1,
    #[doc = "The event detection is enabled"]
    VALUE2,
}
impl GEENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            GEENW::VALUE1 => false,
            GEENW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _GEENW<'a> {
    w: &'a mut W,
}
impl<'a> _GEENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GEENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The event detection is disabled"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(GEENW::VALUE1)
    }
    #[doc = "The event detection is enabled"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(GEENW::VALUE2)
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
#[doc = "Values that can be written to the field `GP`"]
pub enum GPW {
    #[doc = "ERU_GOUTy is always disabled and ERU_IOUTy can not be activated"]
    VALUE1,
    #[doc = "ERU_GOUTy is always enabled and ERU_IOUTy becomes activated with each activation of ERU_TOUTy"]
    VALUE2,
    #[doc = "ERU_GOUTy is equal to ERU_PDOUTy and ERU_IOUTy becomes activated with an activation of ERU_TOUTy while the desired pattern is detected (pattern match PDR = 1)"]
    VALUE3,
    #[doc = "ERU_GOUTy is inverted to ERU_PDOUTy and ERU_IOUTy becomes activated with an activation of ERU_TOUTy while the desired pattern is not detected (pattern miss PDR = 0)"]
    VALUE4,
}
impl GPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            GPW::VALUE1 => 0,
            GPW::VALUE2 => 1,
            GPW::VALUE3 => 2,
            GPW::VALUE4 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _GPW<'a> {
    w: &'a mut W,
}
impl<'a> _GPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GPW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "ERU_GOUTy is always disabled and ERU_IOUTy can not be activated"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(GPW::VALUE1)
    }
    #[doc = "ERU_GOUTy is always enabled and ERU_IOUTy becomes activated with each activation of ERU_TOUTy"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(GPW::VALUE2)
    }
    #[doc = "ERU_GOUTy is equal to ERU_PDOUTy and ERU_IOUTy becomes activated with an activation of ERU_TOUTy while the desired pattern is detected (pattern match PDR = 1)"]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(GPW::VALUE3)
    }
    #[doc = "ERU_GOUTy is inverted to ERU_PDOUTy and ERU_IOUTy becomes activated with an activation of ERU_TOUTy while the desired pattern is not detected (pattern miss PDR = 0)"]
    #[inline]
    pub fn value4(self) -> &'a mut W {
        self.variant(GPW::VALUE4)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `IPEN0`"]
pub enum IPEN0W {
    #[doc = "Flag EXICONx.FL is excluded from the pattern detection"]
    VALUE1,
    #[doc = "Flag EXICONx.FL is included in the pattern detection"]
    VALUE2,
}
impl IPEN0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            IPEN0W::VALUE1 => false,
            IPEN0W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _IPEN0W<'a> {
    w: &'a mut W,
}
impl<'a> _IPEN0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: IPEN0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Flag EXICONx.FL is excluded from the pattern detection"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(IPEN0W::VALUE1)
    }
    #[doc = "Flag EXICONx.FL is included in the pattern detection"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(IPEN0W::VALUE2)
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
#[doc = "Values that can be written to the field `IPEN1`"]
pub enum IPEN1W {
    #[doc = "Flag EXICONx.FL is excluded from the pattern detection"]
    VALUE1,
    #[doc = "Flag EXICONx.FL is included in the pattern detection"]
    VALUE2,
}
impl IPEN1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            IPEN1W::VALUE1 => false,
            IPEN1W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _IPEN1W<'a> {
    w: &'a mut W,
}
impl<'a> _IPEN1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: IPEN1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Flag EXICONx.FL is excluded from the pattern detection"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(IPEN1W::VALUE1)
    }
    #[doc = "Flag EXICONx.FL is included in the pattern detection"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(IPEN1W::VALUE2)
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
#[doc = "Values that can be written to the field `IPEN2`"]
pub enum IPEN2W {
    #[doc = "Flag EXICONx.FL is excluded from the pattern detection"]
    VALUE1,
    #[doc = "Flag EXICONx.FL is included in the pattern detection"]
    VALUE2,
}
impl IPEN2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            IPEN2W::VALUE1 => false,
            IPEN2W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _IPEN2W<'a> {
    w: &'a mut W,
}
impl<'a> _IPEN2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: IPEN2W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Flag EXICONx.FL is excluded from the pattern detection"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(IPEN2W::VALUE1)
    }
    #[doc = "Flag EXICONx.FL is included in the pattern detection"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(IPEN2W::VALUE2)
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
#[doc = "Values that can be written to the field `IPEN3`"]
pub enum IPEN3W {
    #[doc = "Flag EXICONx.FL is excluded from the pattern detection"]
    VALUE1,
    #[doc = "Flag EXICONx.FL is included in the pattern detection"]
    VALUE2,
}
impl IPEN3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            IPEN3W::VALUE1 => false,
            IPEN3W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _IPEN3W<'a> {
    w: &'a mut W,
}
impl<'a> _IPEN3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: IPEN3W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Flag EXICONx.FL is excluded from the pattern detection"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(IPEN3W::VALUE1)
    }
    #[doc = "Flag EXICONx.FL is included in the pattern detection"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(IPEN3W::VALUE2)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:1 - Internal Trigger Source Selection"]
    #[inline]
    pub fn iss(&self) -> ISSR {
        ISSR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 2 - Gating Event Enable"]
    #[inline]
    pub fn geen(&self) -> GEENR {
        GEENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Pattern Detection Result Flag"]
    #[inline]
    pub fn pdr(&self) -> PDRR {
        PDRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 4:5 - Gating Selection for Pattern Detection Result"]
    #[inline]
    pub fn gp(&self) -> GPR {
        GPR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 12 - Pattern Detection Enable for ETL0"]
    #[inline]
    pub fn ipen0(&self) -> IPEN0R {
        IPEN0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 13 - Pattern Detection Enable for ETL1"]
    #[inline]
    pub fn ipen1(&self) -> IPEN1R {
        IPEN1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 14 - Pattern Detection Enable for ETL2"]
    #[inline]
    pub fn ipen2(&self) -> IPEN2R {
        IPEN2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 15 - Pattern Detection Enable for ETL3"]
    #[inline]
    pub fn ipen3(&self) -> IPEN3R {
        IPEN3R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 8 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:1 - Internal Trigger Source Selection"]
    #[inline]
    pub fn iss(&mut self) -> _ISSW {
        _ISSW { w: self }
    }
    #[doc = "Bit 2 - Gating Event Enable"]
    #[inline]
    pub fn geen(&mut self) -> _GEENW {
        _GEENW { w: self }
    }
    #[doc = "Bits 4:5 - Gating Selection for Pattern Detection Result"]
    #[inline]
    pub fn gp(&mut self) -> _GPW {
        _GPW { w: self }
    }
    #[doc = "Bit 12 - Pattern Detection Enable for ETL0"]
    #[inline]
    pub fn ipen0(&mut self) -> _IPEN0W {
        _IPEN0W { w: self }
    }
    #[doc = "Bit 13 - Pattern Detection Enable for ETL1"]
    #[inline]
    pub fn ipen1(&mut self) -> _IPEN1W {
        _IPEN1W { w: self }
    }
    #[doc = "Bit 14 - Pattern Detection Enable for ETL2"]
    #[inline]
    pub fn ipen2(&mut self) -> _IPEN2W {
        _IPEN2W { w: self }
    }
    #[doc = "Bit 15 - Pattern Detection Enable for ETL3"]
    #[inline]
    pub fn ipen3(&mut self) -> _IPEN3W {
        _IPEN3W { w: self }
    }
}
