#[doc = "Reader of register FMMU_NUM"]
pub type R = crate::R<u8, super::FMMU_NUM>;
#[doc = "Reader of field `NUM_FMMU`"]
pub type NUM_FMMU_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - Number of supported FMMU channels (or entities) of the EtherCAT Slave Controller"]
    #[inline(always)]
    pub fn num_fmmu(&self) -> NUM_FMMU_R {
        NUM_FMMU_R::new((self.bits & 0xff) as u8)
    }
}
