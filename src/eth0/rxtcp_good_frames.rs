#[doc = "Reader of register RXTCP_GOOD_FRAMES"]
pub type R = crate::R<u32, super::RXTCP_GOOD_FRAMES>;
#[doc = "Reader of field `RXTCPGDFRM`"]
pub type RXTCPGDFRM_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - This field indicates the number of good IP datagrams with a good TCP payload."]
    #[inline(always)]
    pub fn rxtcpgdfrm(&self) -> RXTCPGDFRM_R {
        RXTCPGDFRM_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
