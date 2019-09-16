#[doc = "Reader of register RXTCP_ERROR_FRAMES"]
pub type R = crate::R<u32, super::RXTCP_ERROR_FRAMES>;
#[doc = "Reader of field `RXTCPERRFRM`"]
pub type RXTCPERRFRM_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - This field indicates the number of good IP datagrams whose TCP payload has a checksum error."]
    #[inline(always)]
    pub fn rxtcperrfrm(&self) -> RXTCPERRFRM_R {
        RXTCPERRFRM_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
