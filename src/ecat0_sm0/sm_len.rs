#[doc = "Register `SM_LEN` reader"]
pub type R = crate::R<SmLenSpec>;
#[doc = "Field `NO_BYTES` reader - Number of bytes assigned to SyncManager"]
pub type NoBytesR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Number of bytes assigned to SyncManager"]
    #[inline(always)]
    pub fn no_bytes(&self) -> NoBytesR {
        NoBytesR::new(self.bits)
    }
}
#[doc = "Length SyncManager 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sm_len::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SmLenSpec;
impl crate::RegisterSpec for SmLenSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`sm_len::R`](R) reader structure"]
impl crate::Readable for SmLenSpec {}
#[doc = "`reset()` method sets SM_LEN to value 0"]
impl crate::Resettable for SmLenSpec {
    const RESET_VALUE: u16 = 0;
}
