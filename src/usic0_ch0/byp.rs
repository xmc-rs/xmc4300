#[doc = "Reader of register BYP"]
pub type R = crate::R<u32, super::BYP>;
#[doc = "Writer for register BYP"]
pub type W = crate::W<u32, super::BYP>;
#[doc = "Register BYP `reset()`'s with value 0"]
impl crate::ResetValue for super::BYP {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `BDATA`"]
pub type BDATA_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `BDATA`"]
pub struct BDATA_W<'a> {
    w: &'a mut W,
}
impl<'a> BDATA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Bypass Data"]
    #[inline(always)]
    pub fn bdata(&self) -> BDATA_R {
        BDATA_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Bypass Data"]
    #[inline(always)]
    pub fn bdata(&mut self) -> BDATA_W {
        BDATA_W { w: self }
    }
}
