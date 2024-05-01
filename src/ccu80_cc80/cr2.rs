#[doc = "Register `CR2` reader"]
pub type R = crate::R<Cr2Spec>;
#[doc = "Field `CR2` reader - Compare Register for Channel 2"]
pub type Cr2R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Compare Register for Channel 2"]
    #[inline(always)]
    pub fn cr2(&self) -> Cr2R {
        Cr2R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Channel 2 Compare Value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr2::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cr2Spec;
impl crate::RegisterSpec for Cr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr2::R`](R) reader structure"]
impl crate::Readable for Cr2Spec {}
#[doc = "`reset()` method sets CR2 to value 0"]
impl crate::Resettable for Cr2Spec {
    const RESET_VALUE: u32 = 0;
}
