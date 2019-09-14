#[doc = "Reader of register LLP"]
pub type R = crate::R<u32, super::LLP>;
#[doc = "Writer for register LLP"]
pub type W = crate::W<u32, super::LLP>;
#[doc = "Register LLP `reset()`'s with value 0"]
impl crate::ResetValue for super::LLP {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `LOC`"]
pub type LOC_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `LOC`"]
pub struct LOC_W<'a> {
    w: &'a mut W,
}
impl<'a> LOC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3fff_ffff << 2)) | (((value as u32) & 0x3fff_ffff) << 2);
        self.w
    }
}
impl R {
    #[doc = "Bits 2:31 - Starting Address In Memory"]
    #[inline(always)]
    pub fn loc(&self) -> LOC_R {
        LOC_R::new(((self.bits >> 2) & 0x3fff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 2:31 - Starting Address In Memory"]
    #[inline(always)]
    pub fn loc(&mut self) -> LOC_W {
        LOC_W { w: self }
    }
}
