#[doc = "Reader of register PRS"]
pub type R = crate::R<u32, super::PRS>;
#[doc = "Writer for register PRS"]
pub type W = crate::W<u32, super::PRS>;
#[doc = "Register PRS `reset()`'s with value 0"]
impl crate::ResetValue for super::PRS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PRS`"]
pub type PRS_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `PRS`"]
pub struct PRS_W<'a> {
    w: &'a mut W,
}
impl<'a> PRS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Period Register"]
    #[inline(always)]
    pub fn prs(&self) -> PRS_R {
        PRS_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Period Register"]
    #[inline(always)]
    pub fn prs(&mut self) -> PRS_W {
        PRS_W { w: self }
    }
}
