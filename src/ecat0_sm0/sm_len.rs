#[doc = "Register `SM_LEN` reader"]
pub struct R(crate::R<SM_LEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SM_LEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SM_LEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SM_LEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `NO_BYTES` reader - Number of bytes assigned to SyncManager"]
pub type NO_BYTES_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Number of bytes assigned to SyncManager"]
    #[inline(always)]
    pub fn no_bytes(&self) -> NO_BYTES_R {
        NO_BYTES_R::new(self.bits)
    }
}
#[doc = "Length SyncManager 0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sm_len](index.html) module"]
pub struct SM_LEN_SPEC;
impl crate::RegisterSpec for SM_LEN_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [sm_len::R](R) reader structure"]
impl crate::Readable for SM_LEN_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SM_LEN to value 0"]
impl crate::Resettable for SM_LEN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
