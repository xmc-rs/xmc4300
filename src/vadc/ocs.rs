#[doc = "Register `OCS` reader"]
pub struct R(crate::R<OCS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OCS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OCS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OCS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OCS` writer"]
pub struct W(crate::W<OCS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OCS_SPEC>;
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
impl From<crate::W<OCS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OCS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Trigger Set for OTGB0/1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TGS_A {
    #[doc = "0: No Trigger Set output"]
    VALUE1 = 0,
    #[doc = "1: Trigger Set 1: TS16_SSIG, input sample signals"]
    VALUE2 = 1,
}
impl From<TGS_A> for u8 {
    #[inline(always)]
    fn from(variant: TGS_A) -> Self {
        variant as _
    }
}
#[doc = "Field `TGS` reader - Trigger Set for OTGB0/1"]
pub struct TGS_R(crate::FieldReader<u8, TGS_A>);
impl TGS_R {
    pub(crate) fn new(bits: u8) -> Self {
        TGS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TGS_A> {
        match self.bits {
            0 => Some(TGS_A::VALUE1),
            1 => Some(TGS_A::VALUE2),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == TGS_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == TGS_A::VALUE2
    }
}
impl core::ops::Deref for TGS_R {
    type Target = crate::FieldReader<u8, TGS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TGS` writer - Trigger Set for OTGB0/1"]
pub struct TGS_W<'a> {
    w: &'a mut W,
}
impl<'a> TGS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TGS_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "No Trigger Set output"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(TGS_A::VALUE1)
    }
    #[doc = "Trigger Set 1: TS16_SSIG, input sample signals"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(TGS_A::VALUE2)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
#[doc = "OTGB0/1 Bus Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TGB_A {
    #[doc = "0: Trigger Set is output on OTGB0"]
    VALUE1 = 0,
    #[doc = "1: Trigger Set is output on OTGB1"]
    VALUE2 = 1,
}
impl From<TGB_A> for bool {
    #[inline(always)]
    fn from(variant: TGB_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TGB` reader - OTGB0/1 Bus Select"]
pub struct TGB_R(crate::FieldReader<bool, TGB_A>);
impl TGB_R {
    pub(crate) fn new(bits: bool) -> Self {
        TGB_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TGB_A {
        match self.bits {
            false => TGB_A::VALUE1,
            true => TGB_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == TGB_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == TGB_A::VALUE2
    }
}
impl core::ops::Deref for TGB_R {
    type Target = crate::FieldReader<bool, TGB_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TGB` writer - OTGB0/1 Bus Select"]
pub struct TGB_W<'a> {
    w: &'a mut W,
}
impl<'a> TGB_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TGB_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Trigger Set is output on OTGB0"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(TGB_A::VALUE1)
    }
    #[doc = "Trigger Set is output on OTGB1"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(TGB_A::VALUE2)
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
#[doc = "Field `TG_P` writer - TGS, TGB Write Protection"]
pub struct TG_P_W<'a> {
    w: &'a mut W,
}
impl<'a> TG_P_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "OCDS Suspend Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SUS_A {
    #[doc = "0: Will not suspend"]
    VALUE1 = 0,
    #[doc = "1: Hard suspend: Clock is switched off immediately."]
    VALUE2 = 1,
    #[doc = "2: Soft suspend mode 0: Stop conversions after the currently running one is completed and its result has been stored. No change for the arbiter."]
    VALUE3 = 2,
    #[doc = "3: Soft suspend mode 1: Stop conversions after the currently running one is completed and its result has been stored. Stop arbiter after the current arbitration round."]
    VALUE4 = 3,
}
impl From<SUS_A> for u8 {
    #[inline(always)]
    fn from(variant: SUS_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SUS` reader - OCDS Suspend Control"]
pub struct SUS_R(crate::FieldReader<u8, SUS_A>);
impl SUS_R {
    pub(crate) fn new(bits: u8) -> Self {
        SUS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SUS_A> {
        match self.bits {
            0 => Some(SUS_A::VALUE1),
            1 => Some(SUS_A::VALUE2),
            2 => Some(SUS_A::VALUE3),
            3 => Some(SUS_A::VALUE4),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == SUS_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == SUS_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        **self == SUS_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        **self == SUS_A::VALUE4
    }
}
impl core::ops::Deref for SUS_R {
    type Target = crate::FieldReader<u8, SUS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SUS` writer - OCDS Suspend Control"]
pub struct SUS_W<'a> {
    w: &'a mut W,
}
impl<'a> SUS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SUS_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Will not suspend"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(SUS_A::VALUE1)
    }
    #[doc = "Hard suspend: Clock is switched off immediately."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(SUS_A::VALUE2)
    }
    #[doc = "Soft suspend mode 0: Stop conversions after the currently running one is completed and its result has been stored. No change for the arbiter."]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(SUS_A::VALUE3)
    }
    #[doc = "Soft suspend mode 1: Stop conversions after the currently running one is completed and its result has been stored. Stop arbiter after the current arbitration round."]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(SUS_A::VALUE4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | ((value as u32 & 0x0f) << 24);
        self.w
    }
}
#[doc = "Field `SUS_P` writer - SUS Write Protection"]
pub struct SUS_P_W<'a> {
    w: &'a mut W,
}
impl<'a> SUS_P_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | ((value as u32 & 0x01) << 28);
        self.w
    }
}
#[doc = "Suspend State\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SUSSTA_A {
    #[doc = "0: Module is not (yet) suspended"]
    VALUE1 = 0,
    #[doc = "1: Module is suspended"]
    VALUE2 = 1,
}
impl From<SUSSTA_A> for bool {
    #[inline(always)]
    fn from(variant: SUSSTA_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SUSSTA` reader - Suspend State"]
pub struct SUSSTA_R(crate::FieldReader<bool, SUSSTA_A>);
impl SUSSTA_R {
    pub(crate) fn new(bits: bool) -> Self {
        SUSSTA_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SUSSTA_A {
        match self.bits {
            false => SUSSTA_A::VALUE1,
            true => SUSSTA_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == SUSSTA_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == SUSSTA_A::VALUE2
    }
}
impl core::ops::Deref for SUSSTA_R {
    type Target = crate::FieldReader<bool, SUSSTA_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:1 - Trigger Set for OTGB0/1"]
    #[inline(always)]
    pub fn tgs(&self) -> TGS_R {
        TGS_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bit 2 - OTGB0/1 Bus Select"]
    #[inline(always)]
    pub fn tgb(&self) -> TGB_R {
        TGB_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bits 24:27 - OCDS Suspend Control"]
    #[inline(always)]
    pub fn sus(&self) -> SUS_R {
        SUS_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bit 29 - Suspend State"]
    #[inline(always)]
    pub fn sussta(&self) -> SUSSTA_R {
        SUSSTA_R::new(((self.bits >> 29) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Trigger Set for OTGB0/1"]
    #[inline(always)]
    pub fn tgs(&mut self) -> TGS_W {
        TGS_W { w: self }
    }
    #[doc = "Bit 2 - OTGB0/1 Bus Select"]
    #[inline(always)]
    pub fn tgb(&mut self) -> TGB_W {
        TGB_W { w: self }
    }
    #[doc = "Bit 3 - TGS, TGB Write Protection"]
    #[inline(always)]
    pub fn tg_p(&mut self) -> TG_P_W {
        TG_P_W { w: self }
    }
    #[doc = "Bits 24:27 - OCDS Suspend Control"]
    #[inline(always)]
    pub fn sus(&mut self) -> SUS_W {
        SUS_W { w: self }
    }
    #[doc = "Bit 28 - SUS Write Protection"]
    #[inline(always)]
    pub fn sus_p(&mut self) -> SUS_P_W {
        SUS_P_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "OCDS Control and Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ocs](index.html) module"]
pub struct OCS_SPEC;
impl crate::RegisterSpec for OCS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ocs::R](R) reader structure"]
impl crate::Readable for OCS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ocs::W](W) writer structure"]
impl crate::Writable for OCS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OCS to value 0"]
impl crate::Resettable for OCS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
