#[doc = "Register `DAINT` reader"]
pub struct R(crate::R<DAINT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DAINT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DAINT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DAINT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `InEpInt` reader - IN Endpoint Interrupt Bits"]
pub type IN_EP_INT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `OutEPInt` reader - OUT Endpoint Interrupt Bits"]
pub type OUT_EPINT_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - IN Endpoint Interrupt Bits"]
    #[inline(always)]
    pub fn in_ep_int(&self) -> IN_EP_INT_R {
        IN_EP_INT_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - OUT Endpoint Interrupt Bits"]
    #[inline(always)]
    pub fn out_epint(&self) -> OUT_EPINT_R {
        OUT_EPINT_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "Device All Endpoints Interrupt Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [daint](index.html) module"]
pub struct DAINT_SPEC;
impl crate::RegisterSpec for DAINT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [daint::R](R) reader structure"]
impl crate::Readable for DAINT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DAINT to value 0"]
impl crate::Resettable for DAINT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
