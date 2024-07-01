#[doc = "Register `SYNC_MANAGER` reader"]
pub type R = crate::R<SYNC_MANAGER_SPEC>;
#[doc = "Field `NUM_SM` reader - Number of supported SyncManager channels (or entities) of the EtherCAT Slave Controller"]
pub type NUM_SM_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Number of supported SyncManager channels (or entities) of the EtherCAT Slave Controller"]
    #[inline(always)]
    pub fn num_sm(&self) -> NUM_SM_R {
        NUM_SM_R::new(self.bits)
    }
}
#[doc = "SyncManagers Supported\n\nYou can [`read`](crate::Reg::read) this register and get [`sync_manager::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SYNC_MANAGER_SPEC;
impl crate::RegisterSpec for SYNC_MANAGER_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`sync_manager::R`](R) reader structure"]
impl crate::Readable for SYNC_MANAGER_SPEC {}
#[doc = "`reset()` method sets SYNC_MANAGER to value 0x08"]
impl crate::Resettable for SYNC_MANAGER_SPEC {
    const RESET_VALUE: u8 = 0x08;
}
