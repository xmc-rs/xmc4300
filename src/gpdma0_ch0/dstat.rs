#[doc = "Reader of register DSTAT"]
pub type R = crate::R<u32, super::DSTAT>;
#[doc = "Writer for register DSTAT"]
pub type W = crate::W<u32, super::DSTAT>;
#[doc = "Register DSTAT `reset()`'s with value 0"]
impl crate::ResetValue for super::DSTAT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DSTAT`"]
pub type DSTAT_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `DSTAT`"]
pub struct DSTAT_W<'a> {
    w: &'a mut W,
}
impl<'a> DSTAT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Destination Status"]
    #[inline(always)]
    pub fn dstat(&self) -> DSTAT_R {
        DSTAT_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Destination Status"]
    #[inline(always)]
    pub fn dstat(&mut self) -> DSTAT_W {
        DSTAT_W { w: self }
    }
}
