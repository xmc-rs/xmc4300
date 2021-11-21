#[doc = "Register `NBTR` reader"]
pub struct R(crate::R<NBTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<NBTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<NBTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<NBTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `NBTR` writer"]
pub struct W(crate::W<NBTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<NBTR_SPEC>;
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
impl From<crate::W<NBTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<NBTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BRP` reader - Baud Rate Prescaler"]
pub struct BRP_R(crate::FieldReader<u8, u8>);
impl BRP_R {
    pub(crate) fn new(bits: u8) -> Self {
        BRP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BRP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BRP` writer - Baud Rate Prescaler"]
pub struct BRP_W<'a> {
    w: &'a mut W,
}
impl<'a> BRP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | (value as u32 & 0x3f);
        self.w
    }
}
#[doc = "Field `SJW` reader - (Re) Synchronization Jump Width"]
pub struct SJW_R(crate::FieldReader<u8, u8>);
impl SJW_R {
    pub(crate) fn new(bits: u8) -> Self {
        SJW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SJW_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SJW` writer - (Re) Synchronization Jump Width"]
pub struct SJW_W<'a> {
    w: &'a mut W,
}
impl<'a> SJW_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | ((value as u32 & 0x03) << 6);
        self.w
    }
}
#[doc = "Field `TSEG1` reader - Time Segment Before Sample Point"]
pub struct TSEG1_R(crate::FieldReader<u8, u8>);
impl TSEG1_R {
    pub(crate) fn new(bits: u8) -> Self {
        TSEG1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TSEG1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TSEG1` writer - Time Segment Before Sample Point"]
pub struct TSEG1_W<'a> {
    w: &'a mut W,
}
impl<'a> TSEG1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | ((value as u32 & 0x0f) << 8);
        self.w
    }
}
#[doc = "Field `TSEG2` reader - Time Segment After Sample Point"]
pub struct TSEG2_R(crate::FieldReader<u8, u8>);
impl TSEG2_R {
    pub(crate) fn new(bits: u8) -> Self {
        TSEG2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TSEG2_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TSEG2` writer - Time Segment After Sample Point"]
pub struct TSEG2_W<'a> {
    w: &'a mut W,
}
impl<'a> TSEG2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 12)) | ((value as u32 & 0x07) << 12);
        self.w
    }
}
#[doc = "Divide Prescaler Clock by 8\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIV8_A {
    #[doc = "0: A time quantum lasts (BRP+1) clock cycles."]
    VALUE1 = 0,
    #[doc = "1: A time quantum lasts 8 (BRP+1) clock cycles."]
    VALUE2 = 1,
}
impl From<DIV8_A> for bool {
    #[inline(always)]
    fn from(variant: DIV8_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIV8` reader - Divide Prescaler Clock by 8"]
pub struct DIV8_R(crate::FieldReader<bool, DIV8_A>);
impl DIV8_R {
    pub(crate) fn new(bits: bool) -> Self {
        DIV8_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIV8_A {
        match self.bits {
            false => DIV8_A::VALUE1,
            true => DIV8_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == DIV8_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == DIV8_A::VALUE2
    }
}
impl core::ops::Deref for DIV8_R {
    type Target = crate::FieldReader<bool, DIV8_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIV8` writer - Divide Prescaler Clock by 8"]
pub struct DIV8_W<'a> {
    w: &'a mut W,
}
impl<'a> DIV8_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIV8_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "A time quantum lasts (BRP+1) clock cycles."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(DIV8_A::VALUE1)
    }
    #[doc = "A time quantum lasts 8 (BRP+1) clock cycles."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(DIV8_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | ((value as u32 & 0x01) << 15);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5 - Baud Rate Prescaler"]
    #[inline(always)]
    pub fn brp(&self) -> BRP_R {
        BRP_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:7 - (Re) Synchronization Jump Width"]
    #[inline(always)]
    pub fn sjw(&self) -> SJW_R {
        SJW_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 8:11 - Time Segment Before Sample Point"]
    #[inline(always)]
    pub fn tseg1(&self) -> TSEG1_R {
        TSEG1_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:14 - Time Segment After Sample Point"]
    #[inline(always)]
    pub fn tseg2(&self) -> TSEG2_R {
        TSEG2_R::new(((self.bits >> 12) & 0x07) as u8)
    }
    #[doc = "Bit 15 - Divide Prescaler Clock by 8"]
    #[inline(always)]
    pub fn div8(&self) -> DIV8_R {
        DIV8_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - Baud Rate Prescaler"]
    #[inline(always)]
    pub fn brp(&mut self) -> BRP_W {
        BRP_W { w: self }
    }
    #[doc = "Bits 6:7 - (Re) Synchronization Jump Width"]
    #[inline(always)]
    pub fn sjw(&mut self) -> SJW_W {
        SJW_W { w: self }
    }
    #[doc = "Bits 8:11 - Time Segment Before Sample Point"]
    #[inline(always)]
    pub fn tseg1(&mut self) -> TSEG1_W {
        TSEG1_W { w: self }
    }
    #[doc = "Bits 12:14 - Time Segment After Sample Point"]
    #[inline(always)]
    pub fn tseg2(&mut self) -> TSEG2_W {
        TSEG2_W { w: self }
    }
    #[doc = "Bit 15 - Divide Prescaler Clock by 8"]
    #[inline(always)]
    pub fn div8(&mut self) -> DIV8_W {
        DIV8_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Node Bit Timing Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nbtr](index.html) module"]
pub struct NBTR_SPEC;
impl crate::RegisterSpec for NBTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [nbtr::R](R) reader structure"]
impl crate::Readable for NBTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [nbtr::W](W) writer structure"]
impl crate::Writable for NBTR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets NBTR to value 0"]
impl crate::Resettable for NBTR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
