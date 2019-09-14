#[doc = "Reader of register RX_UNDERSIZE_FRAMES_GOOD"]
pub type R = crate::R<u32, super::RX_UNDERSIZE_FRAMES_GOOD>;
#[doc = "Reader of field `RXUNDERSZG`"]
pub type RXUNDERSZG_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - This field indicates the number of frames received with length less than 64 bytes and without errors."]
    #[inline(always)]
    pub fn rxunderszg(&self) -> RXUNDERSZG_R {
        RXUNDERSZG_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
