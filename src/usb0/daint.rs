#[doc = "Register `DAINT` reader"]
pub type R = crate::R<DaintSpec>;
#[doc = "Field `InEpInt` reader - IN Endpoint Interrupt Bits"]
pub type InEpIntR = crate::FieldReader<u16>;
#[doc = "Field `OutEPInt` reader - OUT Endpoint Interrupt Bits"]
pub type OutEpintR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - IN Endpoint Interrupt Bits"]
    #[inline(always)]
    pub fn in_ep_int(&self) -> InEpIntR {
        InEpIntR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - OUT Endpoint Interrupt Bits"]
    #[inline(always)]
    pub fn out_epint(&self) -> OutEpintR {
        OutEpintR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "Device All Endpoints Interrupt Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`daint::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DaintSpec;
impl crate::RegisterSpec for DaintSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`daint::R`](R) reader structure"]
impl crate::Readable for DaintSpec {}
#[doc = "`reset()` method sets DAINT to value 0"]
impl crate::Resettable for DaintSpec {
    const RESET_VALUE: u32 = 0;
}
