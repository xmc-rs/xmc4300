#[doc = "Register `MSID[%s]` reader"]
pub type R = crate::R<MSID_SPEC>;
#[doc = "Field `INDEX` reader - Message Pending Index"]
pub type INDEX_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:5 - Message Pending Index"]
    #[inline(always)]
    pub fn index(&self) -> INDEX_R {
        INDEX_R::new((self.bits & 0x3f) as u8)
    }
}
#[doc = "Message Index Register\n\nYou can [`read`](crate::Reg::read) this register and get [`msid::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MSID_SPEC;
impl crate::RegisterSpec for MSID_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`msid::R`](R) reader structure"]
impl crate::Readable for MSID_SPEC {}
#[doc = "`reset()` method sets MSID[%s]
to value 0x20"]
impl crate::Resettable for MSID_SPEC {
    const RESET_VALUE: u32 = 0x20;
}
