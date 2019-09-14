#[doc = "Reader of register SSTAT"]
pub type R = crate::R<u32, super::SSTAT>;
#[doc = "Writer for register SSTAT"]
pub type W = crate::W<u32, super::SSTAT>;
#[doc = "Register SSTAT `reset()`'s with value 0"]
impl crate::ResetValue for super::SSTAT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SSTAT`"]
pub type SSTAT_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `SSTAT`"]
pub struct SSTAT_W<'a> {
    w: &'a mut W,
}
impl<'a> SSTAT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Source Status"]
    #[inline(always)]
    pub fn sstat(&self) -> SSTAT_R {
        SSTAT_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Source Status"]
    #[inline(always)]
    pub fn sstat(&mut self) -> SSTAT_W {
        SSTAT_W { w: self }
    }
}
