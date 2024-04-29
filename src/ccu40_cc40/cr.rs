#[doc = "Register `CR` reader"]
pub type R = crate::R<CR_SPEC>;
#[doc = "Field `CR` reader - Compare Register"]
pub type CR_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Compare Register"]
    #[inline(always)]
    pub fn cr(&self) -> CR_R {
        CR_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Timer Compare Value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CR_SPEC;
impl crate::RegisterSpec for CR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr::R`](R) reader structure"]
impl crate::Readable for CR_SPEC {}
#[doc = "`reset()` method sets CR to value 0"]
impl crate::Resettable for CR_SPEC {
    const RESET_VALUE: u32 = 0;
}
