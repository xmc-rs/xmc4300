#[doc = "Register `DC_SYS_TIME_FIL_DEPTH` reader"]
pub struct R(crate::R<DC_SYS_TIME_FIL_DEPTH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DC_SYS_TIME_FIL_DEPTH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DC_SYS_TIME_FIL_DEPTH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DC_SYS_TIME_FIL_DEPTH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DC_SYS_TIME_FIL_DEPTH` writer"]
pub struct W(crate::W<DC_SYS_TIME_FIL_DEPTH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DC_SYS_TIME_FIL_DEPTH_SPEC>;
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
impl From<crate::W<DC_SYS_TIME_FIL_DEPTH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DC_SYS_TIME_FIL_DEPTH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FILTER_DEPTH` reader - Filter depth for averaging the received System Time deviation"]
pub type FILTER_DEPTH_R = crate::FieldReader;
#[doc = "Field `FILTER_DEPTH` writer - Filter depth for averaging the received System Time deviation"]
pub type FILTER_DEPTH_W<'a, const O: u8> = crate::FieldWriter<'a, DC_SYS_TIME_FIL_DEPTH_SPEC, 4, O>;
impl R {
    #[doc = "Bits 0:3 - Filter depth for averaging the received System Time deviation"]
    #[inline(always)]
    pub fn filter_depth(&self) -> FILTER_DEPTH_R {
        FILTER_DEPTH_R::new(self.bits & 0x0f)
    }
}
impl W {
    #[doc = "Bits 0:3 - Filter depth for averaging the received System Time deviation"]
    #[inline(always)]
    #[must_use]
    pub fn filter_depth(&mut self) -> FILTER_DEPTH_W<0> {
        FILTER_DEPTH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "System Time Difference Filter Depth\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dc_sys_time_fil_depth](index.html) module"]
pub struct DC_SYS_TIME_FIL_DEPTH_SPEC;
impl crate::RegisterSpec for DC_SYS_TIME_FIL_DEPTH_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [dc_sys_time_fil_depth::R](R) reader structure"]
impl crate::Readable for DC_SYS_TIME_FIL_DEPTH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dc_sys_time_fil_depth::W](W) writer structure"]
impl crate::Writable for DC_SYS_TIME_FIL_DEPTH_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DC_SYS_TIME_FIL_DEPTH to value 0x04"]
impl crate::Resettable for DC_SYS_TIME_FIL_DEPTH_SPEC {
    const RESET_VALUE: Self::Ux = 0x04;
}
