#[doc = "Register `CR` reader"]
pub type R = crate::R<CrSpec>;
#[doc = "Field `CR` reader - Compare Register"]
pub type CrR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Compare Register"]
    #[inline(always)]
    pub fn cr(&self) -> CrR {
        CrR::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Timer Compare Value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrSpec;
impl crate::RegisterSpec for CrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr::R`](R) reader structure"]
impl crate::Readable for CrSpec {}
#[doc = "`reset()` method sets CR to value 0"]
impl crate::Resettable for CrSpec {
    const RESET_VALUE: u32 = 0;
}
