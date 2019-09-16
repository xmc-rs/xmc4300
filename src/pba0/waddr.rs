#[doc = "Reader of register WADDR"]
pub type R = crate::R<u32, super::WADDR>;
#[doc = "Reader of field `WADDR`"]
pub type WADDR_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Write Error Address"]
    #[inline(always)]
    pub fn waddr(&self) -> WADDR_R {
        WADDR_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
