#[doc = "Reader of register DC_RCV_TIME_PORT1"]
pub type R = crate::R<u32, super::DC_RCV_TIME_PORT1>;
#[doc = "Reader of field `LOCAL_TIME_P1`"]
pub type LOCAL_TIME_P1_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Local time of the beginning of a frame"]
    #[inline(always)]
    pub fn local_time_p1(&self) -> LOCAL_TIME_P1_R {
        LOCAL_TIME_P1_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
