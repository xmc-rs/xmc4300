#[doc = "Register `PR` reader"]
pub type R = crate::R<PrSpec>;
#[doc = "Field `PR` reader - Period Register"]
pub type PrR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Period Register"]
    #[inline(always)]
    pub fn pr(&self) -> PrR {
        PrR::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Timer Period Value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PrSpec;
impl crate::RegisterSpec for PrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pr::R`](R) reader structure"]
impl crate::Readable for PrSpec {}
#[doc = "`reset()` method sets PR to value 0"]
impl crate::Resettable for PrSpec {
    const RESET_VALUE: u32 = 0;
}
