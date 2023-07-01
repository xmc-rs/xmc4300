#[doc = "Register `SYNC_MANAGER` reader"]
pub struct R(crate::R<SYNC_MANAGER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYNC_MANAGER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYNC_MANAGER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYNC_MANAGER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `NUM_SM` reader - Number of supported SyncManager channels (or entities) of the EtherCAT Slave Controller"]
pub type NUM_SM_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Number of supported SyncManager channels (or entities) of the EtherCAT Slave Controller"]
    #[inline(always)]
    pub fn num_sm(&self) -> NUM_SM_R {
        NUM_SM_R::new(self.bits)
    }
}
#[doc = "SyncManagers Supported\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sync_manager](index.html) module"]
pub struct SYNC_MANAGER_SPEC;
impl crate::RegisterSpec for SYNC_MANAGER_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [sync_manager::R](R) reader structure"]
impl crate::Readable for SYNC_MANAGER_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SYNC_MANAGER to value 0x08"]
impl crate::Resettable for SYNC_MANAGER_SPEC {
    const RESET_VALUE: Self::Ux = 0x08;
}
