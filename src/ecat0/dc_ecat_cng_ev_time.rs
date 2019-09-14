#[doc = "Reader of register DC_ECAT_CNG_EV_TIME"]
pub type R = crate::R<u32, super::DC_ECAT_CNG_EV_TIME>;
#[doc = "Reader of field `ECAT_CNG_EV_TIME`"]
pub type ECAT_CNG_EV_TIME_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Register captures local time of the beginning of the frame which causes at least one SyncManager to assert an ECAT event"]
    #[inline(always)]
    pub fn ecat_cng_ev_time(&self) -> ECAT_CNG_EV_TIME_R {
        ECAT_CNG_EV_TIME_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
