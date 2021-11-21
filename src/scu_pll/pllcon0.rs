#[doc = "Register `PLLCON0` reader"]
pub struct R(crate::R<PLLCON0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PLLCON0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PLLCON0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PLLCON0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PLLCON0` writer"]
pub struct W(crate::W<PLLCON0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PLLCON0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<PLLCON0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PLLCON0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "VCO Bypass\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VCOBYP_A {
    #[doc = "0: Normal operation, VCO is not bypassed"]
    CONST_0 = 0,
    #[doc = "1: Prescaler Mode, VCO is bypassed"]
    CONST_1 = 1,
}
impl From<VCOBYP_A> for bool {
    #[inline(always)]
    fn from(variant: VCOBYP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VCOBYP` reader - VCO Bypass"]
pub struct VCOBYP_R(crate::FieldReader<bool, VCOBYP_A>);
impl VCOBYP_R {
    pub(crate) fn new(bits: bool) -> Self {
        VCOBYP_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VCOBYP_A {
        match self.bits {
            false => VCOBYP_A::CONST_0,
            true => VCOBYP_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        **self == VCOBYP_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        **self == VCOBYP_A::CONST_1
    }
}
impl core::ops::Deref for VCOBYP_R {
    type Target = crate::FieldReader<bool, VCOBYP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VCOBYP` writer - VCO Bypass"]
pub struct VCOBYP_W<'a> {
    w: &'a mut W,
}
impl<'a> VCOBYP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: VCOBYP_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Normal operation, VCO is not bypassed"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(VCOBYP_A::CONST_0)
    }
    #[doc = "Prescaler Mode, VCO is bypassed"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(VCOBYP_A::CONST_1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "VCO Power Saving Mode\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VCOPWD_A {
    #[doc = "0: Normal behavior"]
    CONST_0 = 0,
    #[doc = "1: The VCO is put into a Power Saving Mode and can no longer be used. Only the Bypass and Prescaler Mode are active if previously selected."]
    CONST_1 = 1,
}
impl From<VCOPWD_A> for bool {
    #[inline(always)]
    fn from(variant: VCOPWD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VCOPWD` reader - VCO Power Saving Mode"]
pub struct VCOPWD_R(crate::FieldReader<bool, VCOPWD_A>);
impl VCOPWD_R {
    pub(crate) fn new(bits: bool) -> Self {
        VCOPWD_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VCOPWD_A {
        match self.bits {
            false => VCOPWD_A::CONST_0,
            true => VCOPWD_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        **self == VCOPWD_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        **self == VCOPWD_A::CONST_1
    }
}
impl core::ops::Deref for VCOPWD_R {
    type Target = crate::FieldReader<bool, VCOPWD_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VCOPWD` writer - VCO Power Saving Mode"]
pub struct VCOPWD_W<'a> {
    w: &'a mut W,
}
impl<'a> VCOPWD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: VCOPWD_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Normal behavior"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(VCOPWD_A::CONST_0)
    }
    #[doc = "The VCO is put into a Power Saving Mode and can no longer be used. Only the Bypass and Prescaler Mode are active if previously selected."]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(VCOPWD_A::CONST_1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "VCO Trim Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VCOTR_A {
    #[doc = "0: VCO bandwidth is operation in the normal range. VCO output frequency is between 260 and 520 MHz for a input frequency between 8 and 16 MHz."]
    CONST_0 = 0,
    #[doc = "1: VCO bandwidth is operation in the test range. VCO output frequency is between 260 and 520 MHz for a input frequency between 8 and 16 MHz."]
    CONST_1 = 1,
}
impl From<VCOTR_A> for bool {
    #[inline(always)]
    fn from(variant: VCOTR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VCOTR` reader - VCO Trim Control"]
pub struct VCOTR_R(crate::FieldReader<bool, VCOTR_A>);
impl VCOTR_R {
    pub(crate) fn new(bits: bool) -> Self {
        VCOTR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VCOTR_A {
        match self.bits {
            false => VCOTR_A::CONST_0,
            true => VCOTR_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        **self == VCOTR_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        **self == VCOTR_A::CONST_1
    }
}
impl core::ops::Deref for VCOTR_R {
    type Target = crate::FieldReader<bool, VCOTR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VCOTR` writer - VCO Trim Control"]
pub struct VCOTR_W<'a> {
    w: &'a mut W,
}
impl<'a> VCOTR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: VCOTR_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "VCO bandwidth is operation in the normal range. VCO output frequency is between 260 and 520 MHz for a input frequency between 8 and 16 MHz."]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(VCOTR_A::CONST_0)
    }
    #[doc = "VCO bandwidth is operation in the test range. VCO output frequency is between 260 and 520 MHz for a input frequency between 8 and 16 MHz."]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(VCOTR_A::CONST_1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Disconnect Oscillator from VCO\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FINDIS_A {
    #[doc = "0: connect oscillator to the VCO part"]
    CONST_0 = 0,
    #[doc = "1: disconnect oscillator from the VCO part."]
    CONST_1 = 1,
}
impl From<FINDIS_A> for bool {
    #[inline(always)]
    fn from(variant: FINDIS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FINDIS` reader - Disconnect Oscillator from VCO"]
pub struct FINDIS_R(crate::FieldReader<bool, FINDIS_A>);
impl FINDIS_R {
    pub(crate) fn new(bits: bool) -> Self {
        FINDIS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FINDIS_A {
        match self.bits {
            false => FINDIS_A::CONST_0,
            true => FINDIS_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        **self == FINDIS_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        **self == FINDIS_A::CONST_1
    }
}
impl core::ops::Deref for FINDIS_R {
    type Target = crate::FieldReader<bool, FINDIS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FINDIS` writer - Disconnect Oscillator from VCO"]
pub struct FINDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> FINDIS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FINDIS_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "connect oscillator to the VCO part"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(FINDIS_A::CONST_0)
    }
    #[doc = "disconnect oscillator from the VCO part."]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(FINDIS_A::CONST_1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "Oscillator Disconnect Disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OSCDISCDIS_A {
    #[doc = "0: In case of a PLL loss-of-lock bit FINDIS is set"]
    CONST_0 = 0,
    #[doc = "1: In case of a PLL loss-of-lock bit FINDIS is cleared"]
    CONST_1 = 1,
}
impl From<OSCDISCDIS_A> for bool {
    #[inline(always)]
    fn from(variant: OSCDISCDIS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OSCDISCDIS` reader - Oscillator Disconnect Disable"]
pub struct OSCDISCDIS_R(crate::FieldReader<bool, OSCDISCDIS_A>);
impl OSCDISCDIS_R {
    pub(crate) fn new(bits: bool) -> Self {
        OSCDISCDIS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OSCDISCDIS_A {
        match self.bits {
            false => OSCDISCDIS_A::CONST_0,
            true => OSCDISCDIS_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        **self == OSCDISCDIS_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        **self == OSCDISCDIS_A::CONST_1
    }
}
impl core::ops::Deref for OSCDISCDIS_R {
    type Target = crate::FieldReader<bool, OSCDISCDIS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OSCDISCDIS` writer - Oscillator Disconnect Disable"]
pub struct OSCDISCDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> OSCDISCDIS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OSCDISCDIS_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "In case of a PLL loss-of-lock bit FINDIS is set"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(OSCDISCDIS_A::CONST_0)
    }
    #[doc = "In case of a PLL loss-of-lock bit FINDIS is cleared"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(OSCDISCDIS_A::CONST_1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "PLL Power Saving Mode\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PLLPWD_A {
    #[doc = "0: Normal behavior"]
    CONST_0 = 0,
    #[doc = "1: The complete PLL block is put into a Power Saving Mode and can no longer be used. Only the Bypass Mode is active if previously selected."]
    CONST_1 = 1,
}
impl From<PLLPWD_A> for bool {
    #[inline(always)]
    fn from(variant: PLLPWD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PLLPWD` reader - PLL Power Saving Mode"]
pub struct PLLPWD_R(crate::FieldReader<bool, PLLPWD_A>);
impl PLLPWD_R {
    pub(crate) fn new(bits: bool) -> Self {
        PLLPWD_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PLLPWD_A {
        match self.bits {
            false => PLLPWD_A::CONST_0,
            true => PLLPWD_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        **self == PLLPWD_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        **self == PLLPWD_A::CONST_1
    }
}
impl core::ops::Deref for PLLPWD_R {
    type Target = crate::FieldReader<bool, PLLPWD_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PLLPWD` writer - PLL Power Saving Mode"]
pub struct PLLPWD_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLPWD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PLLPWD_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Normal behavior"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(PLLPWD_A::CONST_0)
    }
    #[doc = "The complete PLL block is put into a Power Saving Mode and can no longer be used. Only the Bypass Mode is active if previously selected."]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(PLLPWD_A::CONST_1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
#[doc = "Oscillator Watchdog Reset\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OSCRES_A {
    #[doc = "0: The Oscillator Watchdog of the PLL is not reset and remains active"]
    CONST_0 = 0,
    #[doc = "1: The Oscillator Watchdog of the PLL is reset"]
    CONST_1 = 1,
}
impl From<OSCRES_A> for bool {
    #[inline(always)]
    fn from(variant: OSCRES_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OSCRES` reader - Oscillator Watchdog Reset"]
pub struct OSCRES_R(crate::FieldReader<bool, OSCRES_A>);
impl OSCRES_R {
    pub(crate) fn new(bits: bool) -> Self {
        OSCRES_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OSCRES_A {
        match self.bits {
            false => OSCRES_A::CONST_0,
            true => OSCRES_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        **self == OSCRES_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        **self == OSCRES_A::CONST_1
    }
}
impl core::ops::Deref for OSCRES_R {
    type Target = crate::FieldReader<bool, OSCRES_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OSCRES` writer - Oscillator Watchdog Reset"]
pub struct OSCRES_W<'a> {
    w: &'a mut W,
}
impl<'a> OSCRES_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OSCRES_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The Oscillator Watchdog of the PLL is not reset and remains active"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(OSCRES_A::CONST_0)
    }
    #[doc = "The Oscillator Watchdog of the PLL is reset"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(OSCRES_A::CONST_1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 17)) | ((value as u32 & 0x01) << 17);
        self.w
    }
}
#[doc = "Field `RESLD` writer - Restart VCO Lock Detection"]
pub struct RESLD_W<'a> {
    w: &'a mut W,
}
impl<'a> RESLD_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 18)) | ((value as u32 & 0x01) << 18);
        self.w
    }
}
#[doc = "Automatic Oscillator Calibration Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AOTREN_A {
    #[doc = "0: Disable"]
    CONST_0 = 0,
    #[doc = "1: Enable"]
    CONST_1 = 1,
}
impl From<AOTREN_A> for bool {
    #[inline(always)]
    fn from(variant: AOTREN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AOTREN` reader - Automatic Oscillator Calibration Enable"]
pub struct AOTREN_R(crate::FieldReader<bool, AOTREN_A>);
impl AOTREN_R {
    pub(crate) fn new(bits: bool) -> Self {
        AOTREN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AOTREN_A {
        match self.bits {
            false => AOTREN_A::CONST_0,
            true => AOTREN_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        **self == AOTREN_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        **self == AOTREN_A::CONST_1
    }
}
impl core::ops::Deref for AOTREN_R {
    type Target = crate::FieldReader<bool, AOTREN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AOTREN` writer - Automatic Oscillator Calibration Enable"]
pub struct AOTREN_W<'a> {
    w: &'a mut W,
}
impl<'a> AOTREN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AOTREN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(AOTREN_A::CONST_0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(AOTREN_A::CONST_1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 19)) | ((value as u32 & 0x01) << 19);
        self.w
    }
}
#[doc = "Factory Oscillator Calibration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FOTR_A {
    #[doc = "0: No effect"]
    CONST_0 = 0,
    #[doc = "1: Force fixed-value trimming"]
    CONST_1 = 1,
}
impl From<FOTR_A> for bool {
    #[inline(always)]
    fn from(variant: FOTR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FOTR` reader - Factory Oscillator Calibration"]
pub struct FOTR_R(crate::FieldReader<bool, FOTR_A>);
impl FOTR_R {
    pub(crate) fn new(bits: bool) -> Self {
        FOTR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FOTR_A {
        match self.bits {
            false => FOTR_A::CONST_0,
            true => FOTR_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        **self == FOTR_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        **self == FOTR_A::CONST_1
    }
}
impl core::ops::Deref for FOTR_R {
    type Target = crate::FieldReader<bool, FOTR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FOTR` writer - Factory Oscillator Calibration"]
pub struct FOTR_W<'a> {
    w: &'a mut W,
}
impl<'a> FOTR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FOTR_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(FOTR_A::CONST_0)
    }
    #[doc = "Force fixed-value trimming"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(FOTR_A::CONST_1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 20)) | ((value as u32 & 0x01) << 20);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - VCO Bypass"]
    #[inline(always)]
    pub fn vcobyp(&self) -> VCOBYP_R {
        VCOBYP_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - VCO Power Saving Mode"]
    #[inline(always)]
    pub fn vcopwd(&self) -> VCOPWD_R {
        VCOPWD_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - VCO Trim Control"]
    #[inline(always)]
    pub fn vcotr(&self) -> VCOTR_R {
        VCOTR_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Disconnect Oscillator from VCO"]
    #[inline(always)]
    pub fn findis(&self) -> FINDIS_R {
        FINDIS_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Oscillator Disconnect Disable"]
    #[inline(always)]
    pub fn oscdiscdis(&self) -> OSCDISCDIS_R {
        OSCDISCDIS_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 16 - PLL Power Saving Mode"]
    #[inline(always)]
    pub fn pllpwd(&self) -> PLLPWD_R {
        PLLPWD_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Oscillator Watchdog Reset"]
    #[inline(always)]
    pub fn oscres(&self) -> OSCRES_R {
        OSCRES_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Automatic Oscillator Calibration Enable"]
    #[inline(always)]
    pub fn aotren(&self) -> AOTREN_R {
        AOTREN_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Factory Oscillator Calibration"]
    #[inline(always)]
    pub fn fotr(&self) -> FOTR_R {
        FOTR_R::new(((self.bits >> 20) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - VCO Bypass"]
    #[inline(always)]
    pub fn vcobyp(&mut self) -> VCOBYP_W {
        VCOBYP_W { w: self }
    }
    #[doc = "Bit 1 - VCO Power Saving Mode"]
    #[inline(always)]
    pub fn vcopwd(&mut self) -> VCOPWD_W {
        VCOPWD_W { w: self }
    }
    #[doc = "Bit 2 - VCO Trim Control"]
    #[inline(always)]
    pub fn vcotr(&mut self) -> VCOTR_W {
        VCOTR_W { w: self }
    }
    #[doc = "Bit 4 - Disconnect Oscillator from VCO"]
    #[inline(always)]
    pub fn findis(&mut self) -> FINDIS_W {
        FINDIS_W { w: self }
    }
    #[doc = "Bit 6 - Oscillator Disconnect Disable"]
    #[inline(always)]
    pub fn oscdiscdis(&mut self) -> OSCDISCDIS_W {
        OSCDISCDIS_W { w: self }
    }
    #[doc = "Bit 16 - PLL Power Saving Mode"]
    #[inline(always)]
    pub fn pllpwd(&mut self) -> PLLPWD_W {
        PLLPWD_W { w: self }
    }
    #[doc = "Bit 17 - Oscillator Watchdog Reset"]
    #[inline(always)]
    pub fn oscres(&mut self) -> OSCRES_W {
        OSCRES_W { w: self }
    }
    #[doc = "Bit 18 - Restart VCO Lock Detection"]
    #[inline(always)]
    pub fn resld(&mut self) -> RESLD_W {
        RESLD_W { w: self }
    }
    #[doc = "Bit 19 - Automatic Oscillator Calibration Enable"]
    #[inline(always)]
    pub fn aotren(&mut self) -> AOTREN_W {
        AOTREN_W { w: self }
    }
    #[doc = "Bit 20 - Factory Oscillator Calibration"]
    #[inline(always)]
    pub fn fotr(&mut self) -> FOTR_W {
        FOTR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PLL Configuration 0 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pllcon0](index.html) module"]
pub struct PLLCON0_SPEC;
impl crate::RegisterSpec for PLLCON0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pllcon0::R](R) reader structure"]
impl crate::Readable for PLLCON0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pllcon0::W](W) writer structure"]
impl crate::Writable for PLLCON0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PLLCON0 to value 0x0003_0003"]
impl crate::Resettable for PLLCON0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0003_0003
    }
}
