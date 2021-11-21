#[doc = "Register `PWRMON` reader"]
pub struct R(crate::R<PWRMON_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PWRMON_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PWRMON_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PWRMON_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PWRMON` writer"]
pub struct W(crate::W<PWRMON_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PWRMON_SPEC>;
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
impl From<crate::W<PWRMON_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PWRMON_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `THRS` reader - Threshold"]
pub struct THRS_R(crate::FieldReader<u8, u8>);
impl THRS_R {
    pub(crate) fn new(bits: u8) -> Self {
        THRS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for THRS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `THRS` writer - Threshold"]
pub struct THRS_W<'a> {
    w: &'a mut W,
}
impl<'a> THRS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
#[doc = "Field `INTV` reader - Interval"]
pub struct INTV_R(crate::FieldReader<u8, u8>);
impl INTV_R {
    pub(crate) fn new(bits: u8) -> Self {
        INTV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INTV_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INTV` writer - Interval"]
pub struct INTV_W<'a> {
    w: &'a mut W,
}
impl<'a> INTV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
#[doc = "Field `ENB` reader - Enable"]
pub struct ENB_R(crate::FieldReader<bool, bool>);
impl ENB_R {
    pub(crate) fn new(bits: bool) -> Self {
        ENB_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ENB_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ENB` writer - Enable"]
pub struct ENB_W<'a> {
    w: &'a mut W,
}
impl<'a> ENB_W<'a> {
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
impl R {
    #[doc = "Bits 0:7 - Threshold"]
    #[inline(always)]
    pub fn thrs(&self) -> THRS_R {
        THRS_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Interval"]
    #[inline(always)]
    pub fn intv(&self) -> INTV_R {
        INTV_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 16 - Enable"]
    #[inline(always)]
    pub fn enb(&self) -> ENB_R {
        ENB_R::new(((self.bits >> 16) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Threshold"]
    #[inline(always)]
    pub fn thrs(&mut self) -> THRS_W {
        THRS_W { w: self }
    }
    #[doc = "Bits 8:15 - Interval"]
    #[inline(always)]
    pub fn intv(&mut self) -> INTV_W {
        INTV_W { w: self }
    }
    #[doc = "Bit 16 - Enable"]
    #[inline(always)]
    pub fn enb(&mut self) -> ENB_W {
        ENB_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Power Monitor Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwrmon](index.html) module"]
pub struct PWRMON_SPEC;
impl crate::RegisterSpec for PWRMON_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pwrmon::R](R) reader structure"]
impl crate::Readable for PWRMON_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pwrmon::W](W) writer structure"]
impl crate::Writable for PWRMON_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PWRMON to value 0"]
impl crate::Resettable for PWRMON_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
