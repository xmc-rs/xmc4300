#[doc = "Reader of register DIEPDMAB"]
pub type R = crate::R<u32, super::DIEPDMAB>;
#[doc = "Reader of field `DMABufferAddr`"]
pub type DMABUFFERADDR_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - DMA Buffer Address"]
    #[inline(always)]
    pub fn dmabuffer_addr(&self) -> DMABUFFERADDR_R {
        DMABUFFERADDR_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
