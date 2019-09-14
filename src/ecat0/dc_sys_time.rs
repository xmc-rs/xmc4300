#[doc = "Writer for register DC_SYS_TIME"]
pub type W = crate::W<u32, super::DC_SYS_TIME>;
#[doc = "Register DC_SYS_TIME `reset()`'s with value 0"]
impl crate::ResetValue for super::DC_SYS_TIME {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `WRITE_ACCESS`"]
pub struct WRITE_ACCESS_W<'a> {
    w: &'a mut W,
}
impl<'a> WRITE_ACCESS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:31 - Write access"]
    #[inline(always)]
    pub fn write_access(&mut self) -> WRITE_ACCESS_W {
        WRITE_ACCESS_W { w: self }
    }
}
