#[doc = "Reader of register SYNC_MANAGER"]
pub type R = crate::R<u8, super::SYNC_MANAGER>;
#[doc = "Reader of field `NUM_SM`"]
pub type NUM_SM_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - Number of supported SyncManager channels (or entities) of the EtherCAT Slave Controller"]
    #[inline(always)]
    pub fn num_sm(&self) -> NUM_SM_R {
        NUM_SM_R::new((self.bits & 0xff) as u8)
    }
}
