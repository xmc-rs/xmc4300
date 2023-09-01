#[doc = "Register `DAINT` reader"]
pub type R = crate::R<DAINT_SPEC>;
#[doc = "Field `InEpInt` reader - IN Endpoint Interrupt Bits"]
pub type IN_EP_INT_R = crate::FieldReader<u16>;
#[doc = "Field `OutEPInt` reader - OUT Endpoint Interrupt Bits"]
pub type OUT_EPINT_R = crate::FieldReader<u16>;
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
#[doc = "Device All Endpoints Interrupt Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`daint::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DAINT_SPEC;
impl crate::RegisterSpec for DAINT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`daint::R`](R) reader structure"]
impl crate::Readable for DAINT_SPEC {}
#[doc = "`reset()` method sets DAINT to value 0"]
impl crate::Resettable for DAINT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
