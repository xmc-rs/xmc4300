#[doc = "Reader of register CRC"]
pub type R = crate::R<u32, super::CRC>;
#[doc = "Writer for register CRC"]
pub type W = crate::W<u32, super::CRC>;
#[doc = "Register CRC `reset()`'s with value 0"]
impl crate::ResetValue for super::CRC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CRC`"]
pub type CRC_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `CRC`"]
pub struct CRC_W<'a> {
    w: &'a mut W,
}
impl<'a> CRC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - CRC Register"]
    #[inline(always)]
    pub fn crc(&self) -> CRC_R {
        CRC_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - CRC Register"]
    #[inline(always)]
    pub fn crc(&mut self) -> CRC_W {
        CRC_W { w: self }
    }
}
