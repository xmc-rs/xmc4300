#[doc = "Reader of register TX_OSIZE_FRAMES_GOOD"]
pub type R = crate::R<u32, super::TX_OSIZE_FRAMES_GOOD>;
#[doc = "Reader of field `TXOSIZG`"]
pub type TXOSIZG_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - This field indicates the number of frames transmitted without errors and with length greater than the maxsize (1,518 or 1,522 bytes for VLAN tagged frames; 2000 bytes if enabled by setting MAC Configuration.2KPE)."]
    #[inline(always)]
    pub fn txosizg(&self) -> TXOSIZG_R {
        TXOSIZG_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
