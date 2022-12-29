#[doc = "Register `DC_SPEED_COUNT_START` reader"]
pub struct R(crate::R<DC_SPEED_COUNT_START_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DC_SPEED_COUNT_START_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DC_SPEED_COUNT_START_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DC_SPEED_COUNT_START_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DC_SPEED_COUNT_START` writer"]
pub struct W(crate::W<DC_SPEED_COUNT_START_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DC_SPEED_COUNT_START_SPEC>;
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
impl From<crate::W<DC_SPEED_COUNT_START_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DC_SPEED_COUNT_START_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `COUNT_START` reader - Bandwidth for adjustment of local copy of System Time"]
pub type COUNT_START_R = crate::FieldReader<u16, u16>;
#[doc = "Field `COUNT_START` writer - Bandwidth for adjustment of local copy of System Time"]
pub type COUNT_START_W<'a, const O: u8> = crate::FieldWriter<'a, u16, DC_SPEED_COUNT_START_SPEC, u16, u16, 15, O>;
impl R {
    #[doc = "Bits 0:14 - Bandwidth for adjustment of local copy of System Time"]
    #[inline(always)]
    pub fn count_start(&self) -> COUNT_START_R {
        COUNT_START_R::new(self.bits & 0x7fff)
    }
}
impl W {
    #[doc = "Bits 0:14 - Bandwidth for adjustment of local copy of System Time"]
    #[inline(always)]
    #[must_use]
    pub fn count_start(&mut self) -> COUNT_START_W<0> {
        COUNT_START_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Speed Counter Start\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dc_speed_count_start](index.html) module"]
pub struct DC_SPEED_COUNT_START_SPEC;
impl crate::RegisterSpec for DC_SPEED_COUNT_START_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [dc_speed_count_start::R](R) reader structure"]
impl crate::Readable for DC_SPEED_COUNT_START_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dc_speed_count_start::W](W) writer structure"]
impl crate::Writable for DC_SPEED_COUNT_START_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DC_SPEED_COUNT_START to value 0x1000"]
impl crate::Resettable for DC_SPEED_COUNT_START_SPEC {
    const RESET_VALUE: Self::Ux = 0x1000;
}
