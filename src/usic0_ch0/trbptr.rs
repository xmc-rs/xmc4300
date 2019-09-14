#[doc = "Reader of register TRBPTR"]
pub type R = crate::R<u32, super::TRBPTR>;
#[doc = "Reader of field `TDIPTR`"]
pub type TDIPTR_R = crate::R<u8, u8>;
#[doc = "Reader of field `TDOPTR`"]
pub type TDOPTR_R = crate::R<u8, u8>;
#[doc = "Reader of field `RDIPTR`"]
pub type RDIPTR_R = crate::R<u8, u8>;
#[doc = "Reader of field `RDOPTR`"]
pub type RDOPTR_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:5 - Transmitter Data Input Pointer"]
    #[inline(always)]
    pub fn tdiptr(&self) -> TDIPTR_R {
        TDIPTR_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:13 - Transmitter Data Output Pointer"]
    #[inline(always)]
    pub fn tdoptr(&self) -> TDOPTR_R {
        TDOPTR_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:21 - Receiver Data Input Pointer"]
    #[inline(always)]
    pub fn rdiptr(&self) -> RDIPTR_R {
        RDIPTR_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 24:29 - Receiver Data Output Pointer"]
    #[inline(always)]
    pub fn rdoptr(&self) -> RDOPTR_R {
        RDOPTR_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
