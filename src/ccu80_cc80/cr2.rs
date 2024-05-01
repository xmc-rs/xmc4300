#[doc = "Register `CR2` reader"]
pub type R = crate::R<CR2_SPEC>;
#[doc = "Field `CR2` reader - Compare Register for Channel 2"]
pub type CR2_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Compare Register for Channel 2"]
    #[inline(always)]
    pub fn cr2(&self) -> CR2_R {
        CR2_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Channel 2 Compare Value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr2::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CR2_SPEC;
impl crate::RegisterSpec for CR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr2::R`](R) reader structure"]
impl crate::Readable for CR2_SPEC {}
#[doc = "`reset()` method sets CR2 to value 0"]
impl crate::Resettable for CR2_SPEC {
    const RESET_VALUE: u32 = 0;
}
