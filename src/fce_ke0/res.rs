#[doc = "Reader of register RES"]
pub type R = crate::R<u32, super::RES>;
#[doc = "Reader of field `RES`"]
pub type RES_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Result Register"]
    #[inline(always)]
    pub fn res(&self) -> RES_R {
        RES_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
