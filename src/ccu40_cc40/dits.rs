#[doc = "Reader of register DITS"]
pub type R = crate::R<u32, super::DITS>;
#[doc = "Writer for register DITS"]
pub type W = crate::W<u32, super::DITS>;
#[doc = "Register DITS `reset()`'s with value 0"]
impl crate::ResetValue for super::DITS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DCVS`"]
pub type DCVS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DCVS`"]
pub struct DCVS_W<'a> {
    w: &'a mut W,
}
impl<'a> DCVS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Dither Shadow Compare Value"]
    #[inline(always)]
    pub fn dcvs(&self) -> DCVS_R {
        DCVS_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Dither Shadow Compare Value"]
    #[inline(always)]
    pub fn dcvs(&mut self) -> DCVS_W {
        DCVS_W { w: self }
    }
}
