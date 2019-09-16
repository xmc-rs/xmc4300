#[doc = "Reader of register RXUDP_ERROR_FRAMES"]
pub type R = crate::R<u32, super::RXUDP_ERROR_FRAMES>;
#[doc = "Reader of field `RXUDPERRFRM`"]
pub type RXUDPERRFRM_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - This field indicates the number of good IP datagrams whose UDP payload has a checksum error."]
    #[inline(always)]
    pub fn rxudperrfrm(&self) -> RXUDPERRFRM_R {
        RXUDPERRFRM_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
