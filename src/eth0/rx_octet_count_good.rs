#[doc = "Reader of register RX_OCTET_COUNT_GOOD"]
pub type R = crate::R<u32, super::RX_OCTET_COUNT_GOOD>;
#[doc = "Reader of field `RXOCTG`"]
pub type RXOCTG_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - This field indicates the number of bytes received, exclusive of preamble, only in good frames."]
    #[inline(always)]
    pub fn rxoctg(&self) -> RXOCTG_R {
        RXOCTG_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
