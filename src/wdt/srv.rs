#[doc = "Writer for register SRV"]
pub type W = crate::W<u32, super::SRV>;
#[doc = "Register SRV `reset()`'s with value 0"]
impl crate::ResetValue for super::SRV {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `SRV`"]
pub struct SRV_W<'a> {
    w: &'a mut W,
}
impl<'a> SRV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:31 - Service"]
    #[inline(always)]
    pub fn srv(&mut self) -> SRV_W {
        SRV_W { w: self }
    }
}
