#[doc = "Reader of register RX_OVERSIZE_FRAMES_GOOD"]
pub type R = crate::R<u32, super::RX_OVERSIZE_FRAMES_GOOD>;
#[doc = "Reader of field `RXOVERSZG`"]
pub type RXOVERSZG_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - This field indicates the number of frames received without errors, with length greater than the maxsize (1,518 or 1,522 for VLAN tagged frames; 2,000 bytes if enabled by setting MAC Configuration.2KPE)."]
    #[inline(always)]
    pub fn rxoverszg(&self) -> RXOVERSZG_R {
        RXOVERSZG_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
