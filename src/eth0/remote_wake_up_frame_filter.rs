#[doc = "Register `REMOTE_WAKE_UP_FRAME_FILTER` reader"]
pub struct R(crate::R<REMOTE_WAKE_UP_FRAME_FILTER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<REMOTE_WAKE_UP_FRAME_FILTER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<REMOTE_WAKE_UP_FRAME_FILTER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<REMOTE_WAKE_UP_FRAME_FILTER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `REMOTE_WAKE_UP_FRAME_FILTER` writer"]
pub struct W(crate::W<REMOTE_WAKE_UP_FRAME_FILTER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<REMOTE_WAKE_UP_FRAME_FILTER_SPEC>;
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
impl From<crate::W<REMOTE_WAKE_UP_FRAME_FILTER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<REMOTE_WAKE_UP_FRAME_FILTER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WKUPFRMFTR` reader - Remote Wake-Up Frame Filter"]
pub type WKUPFRMFTR_R = crate::FieldReader<u32, u32>;
#[doc = "Field `WKUPFRMFTR` writer - Remote Wake-Up Frame Filter"]
pub type WKUPFRMFTR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, REMOTE_WAKE_UP_FRAME_FILTER_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Remote Wake-Up Frame Filter"]
    #[inline(always)]
    pub fn wkupfrmftr(&self) -> WKUPFRMFTR_R {
        WKUPFRMFTR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Remote Wake-Up Frame Filter"]
    #[inline(always)]
    #[must_use]
    pub fn wkupfrmftr(&mut self) -> WKUPFRMFTR_W<0> {
        WKUPFRMFTR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Remote Wake Up Frame Filter Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [remote_wake_up_frame_filter](index.html) module"]
pub struct REMOTE_WAKE_UP_FRAME_FILTER_SPEC;
impl crate::RegisterSpec for REMOTE_WAKE_UP_FRAME_FILTER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [remote_wake_up_frame_filter::R](R) reader structure"]
impl crate::Readable for REMOTE_WAKE_UP_FRAME_FILTER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [remote_wake_up_frame_filter::W](W) writer structure"]
impl crate::Writable for REMOTE_WAKE_UP_FRAME_FILTER_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets REMOTE_WAKE_UP_FRAME_FILTER to value 0"]
impl crate::Resettable for REMOTE_WAKE_UP_FRAME_FILTER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
