#[doc = "Reader of register CRS"]
pub type R = crate::R<u32, super::CRS>;
#[doc = "Writer for register CRS"]
pub type W = crate::W<u32, super::CRS>;
#[doc = "Register CRS `reset()`'s with value 0"]
impl crate::ResetValue for super::CRS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CRS`"]
pub type CRS_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `CRS`"]
pub struct CRS_W<'a> {
    w: &'a mut W,
}
impl<'a> CRS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Compare Register"]
    #[inline(always)]
    pub fn crs(&self) -> CRS_R {
        CRS_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Compare Register"]
    #[inline(always)]
    pub fn crs(&mut self) -> CRS_W {
        CRS_W { w: self }
    }
}
