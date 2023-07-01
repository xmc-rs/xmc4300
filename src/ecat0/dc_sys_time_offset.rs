#[doc = "Register `DC_SYS_TIME_OFFSET[%s]` reader"]
pub struct R(crate::R<DC_SYS_TIME_OFFSET_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DC_SYS_TIME_OFFSET_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DC_SYS_TIME_OFFSET_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DC_SYS_TIME_OFFSET_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DC_SYS_TIME_OFFSET[%s]` writer"]
pub struct W(crate::W<DC_SYS_TIME_OFFSET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DC_SYS_TIME_OFFSET_SPEC>;
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
impl From<crate::W<DC_SYS_TIME_OFFSET_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DC_SYS_TIME_OFFSET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DC_SYS_TIME_OFFSET` reader - Difference between local time and System Time"]
pub type DC_SYS_TIME_OFFSET_R = crate::FieldReader<u32>;
#[doc = "Field `DC_SYS_TIME_OFFSET` writer - Difference between local time and System Time"]
pub type DC_SYS_TIME_OFFSET_W<'a, const O: u8> = crate::FieldWriter<'a, DC_SYS_TIME_OFFSET_SPEC, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - Difference between local time and System Time"]
    #[inline(always)]
    pub fn dc_sys_time_offset(&self) -> DC_SYS_TIME_OFFSET_R {
        DC_SYS_TIME_OFFSET_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Difference between local time and System Time"]
    #[inline(always)]
    #[must_use]
    pub fn dc_sys_time_offset(&mut self) -> DC_SYS_TIME_OFFSET_W<0> {
        DC_SYS_TIME_OFFSET_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Difference between local time and System Time\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dc_sys_time_offset](index.html) module"]
pub struct DC_SYS_TIME_OFFSET_SPEC;
impl crate::RegisterSpec for DC_SYS_TIME_OFFSET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dc_sys_time_offset::R](R) reader structure"]
impl crate::Readable for DC_SYS_TIME_OFFSET_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dc_sys_time_offset::W](W) writer structure"]
impl crate::Writable for DC_SYS_TIME_OFFSET_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DC_SYS_TIME_OFFSET[%s]
to value 0"]
impl crate::Resettable for DC_SYS_TIME_OFFSET_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
