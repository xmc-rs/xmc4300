#[doc = "Register `SM_LEN` reader"]
pub type R = crate::R<SM_LEN_SPEC>;
#[doc = "Field `NO_BYTES` reader - Number of bytes assigned to SyncManager"]
pub type NO_BYTES_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Number of bytes assigned to SyncManager"]
    #[inline(always)]
    pub fn no_bytes(&self) -> NO_BYTES_R {
        NO_BYTES_R::new(self.bits)
    }
}
#[doc = "Length SyncManager 0\n\nYou can [`read`](crate::Reg::read) this register and get [`sm_len::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SM_LEN_SPEC;
impl crate::RegisterSpec for SM_LEN_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`sm_len::R`](R) reader structure"]
impl crate::Readable for SM_LEN_SPEC {}
#[doc = "`reset()` method sets SM_LEN to value 0"]
impl crate::Resettable for SM_LEN_SPEC {
    const RESET_VALUE: u16 = 0;
}
