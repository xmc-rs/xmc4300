#[doc = "Register `SYNC_MANAGER` reader"]
pub type R = crate::R<SyncManagerSpec>;
#[doc = "Field `NUM_SM` reader - Number of supported SyncManager channels (or entities) of the EtherCAT Slave Controller"]
pub type NumSmR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Number of supported SyncManager channels (or entities) of the EtherCAT Slave Controller"]
    #[inline(always)]
    pub fn num_sm(&self) -> NumSmR {
        NumSmR::new(self.bits)
    }
}
#[doc = "SyncManagers Supported\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sync_manager::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SyncManagerSpec;
impl crate::RegisterSpec for SyncManagerSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`sync_manager::R`](R) reader structure"]
impl crate::Readable for SyncManagerSpec {}
#[doc = "`reset()` method sets SYNC_MANAGER to value 0x08"]
impl crate::Resettable for SyncManagerSpec {
    const RESET_VALUE: u8 = 0x08;
}
