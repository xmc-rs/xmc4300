#[doc = "Register `DC_CYC_START_TIME[%s]` reader"]
pub struct R(crate::R<DC_CYC_START_TIME_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DC_CYC_START_TIME_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DC_CYC_START_TIME_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DC_CYC_START_TIME_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DC_CYC_START_TIME[%s]` writer"]
pub struct W(crate::W<DC_CYC_START_TIME_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DC_CYC_START_TIME_SPEC>;
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
impl From<crate::W<DC_CYC_START_TIME_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DC_CYC_START_TIME_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DC_CYC_START_TIME` reader - Start Time Cyclic Operation"]
pub struct DC_CYC_START_TIME_R(crate::FieldReader<u32, u32>);
impl DC_CYC_START_TIME_R {
    pub(crate) fn new(bits: u32) -> Self {
        DC_CYC_START_TIME_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DC_CYC_START_TIME_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DC_CYC_START_TIME` writer - Start Time Cyclic Operation"]
pub struct DC_CYC_START_TIME_W<'a> {
    w: &'a mut W,
}
impl<'a> DC_CYC_START_TIME_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Start Time Cyclic Operation"]
    #[inline(always)]
    pub fn dc_cyc_start_time(&self) -> DC_CYC_START_TIME_R {
        DC_CYC_START_TIME_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Start Time Cyclic Operation"]
    #[inline(always)]
    pub fn dc_cyc_start_time(&mut self) -> DC_CYC_START_TIME_W {
        DC_CYC_START_TIME_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Start Time Cyclic Operation\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dc_cyc_start_time](index.html) module"]
pub struct DC_CYC_START_TIME_SPEC;
impl crate::RegisterSpec for DC_CYC_START_TIME_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dc_cyc_start_time::R](R) reader structure"]
impl crate::Readable for DC_CYC_START_TIME_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dc_cyc_start_time::W](W) writer structure"]
impl crate::Writable for DC_CYC_START_TIME_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DC_CYC_START_TIME[%s]
to value 0"]
impl crate::Resettable for DC_CYC_START_TIME_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
