#[doc = "Reader of register HAINTMSK"]
pub type R = crate::R<u32, super::HAINTMSK>;
#[doc = "Writer for register HAINTMSK"]
pub type W = crate::W<u32, super::HAINTMSK>;
#[doc = "Register HAINTMSK `reset()`'s with value 0"]
impl crate::ResetValue for super::HAINTMSK {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `HAINTMsk`"]
pub type HAINTMSK_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `HAINTMsk`"]
pub struct HAINTMSK_W<'a> {
    w: &'a mut W,
}
impl<'a> HAINTMSK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3fff) | ((value as u32) & 0x3fff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:13 - Channel Interrupt Mask"]
    #[inline(always)]
    pub fn haintmsk(&self) -> HAINTMSK_R {
        HAINTMSK_R::new((self.bits & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:13 - Channel Interrupt Mask"]
    #[inline(always)]
    pub fn haintmsk(&mut self) -> HAINTMSK_W {
        HAINTMSK_W { w: self }
    }
}
