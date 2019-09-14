#[doc = "Reader of register DC_RCV_TIME_PORT0"]
pub type R = crate::R<u32, super::DC_RCV_TIME_PORT0>;
#[doc = "Reader of field `LOCAL_TIME_P0`"]
pub type LOCAL_TIME_P0_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Write by EtherCAT master"]
    #[inline(always)]
    pub fn local_time_p0(&self) -> LOCAL_TIME_P0_R {
        LOCAL_TIME_P0_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
