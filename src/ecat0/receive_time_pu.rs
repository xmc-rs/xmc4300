#[doc = "Reader of register RECEIVE_TIME_PU[%s]"]
pub type R = crate::R<u32, super::RECEIVE_TIME_PU>;
#[doc = "Reader of field `RECEIVE_TIME_PU`"]
pub type RECEIVE_TIME_PU_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Local time of the beginning of a frame"]
    #[inline(always)]
    pub fn receive_time_pu(&self) -> RECEIVE_TIME_PU_R {
        RECEIVE_TIME_PU_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
