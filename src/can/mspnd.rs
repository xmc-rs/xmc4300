#[doc = "Reader of register MSPND[%s]"]
pub type R = crate::R<u32, super::MSPND>;
#[doc = "Writer for register MSPND[%s]"]
pub type W = crate::W<u32, super::MSPND>;
#[doc = "Register MSPND[%s] `reset()`'s with value 0"]
impl crate::ResetValue for super::MSPND {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PND`"]
pub type PND_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `PND`"]
pub struct PND_W<'a> {
    w: &'a mut W,
}
impl<'a> PND_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Message Pending"]
    #[inline(always)]
    pub fn pnd(&self) -> PND_R {
        PND_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Message Pending"]
    #[inline(always)]
    pub fn pnd(&mut self) -> PND_W {
        PND_W { w: self }
    }
}
