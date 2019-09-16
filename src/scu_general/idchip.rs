#[doc = "Reader of register IDCHIP"]
pub type R = crate::R<u32, super::IDCHIP>;
#[doc = "Reader of field `IDCHIP`"]
pub type IDCHIP_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Chip ID"]
    #[inline(always)]
    pub fn idchip(&self) -> IDCHIP_R {
        IDCHIP_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
