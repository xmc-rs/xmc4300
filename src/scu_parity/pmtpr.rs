#[doc = "Reader of register PMTPR"]
pub type R = crate::R<u32, super::PMTPR>;
#[doc = "Writer for register PMTPR"]
pub type W = crate::W<u32, super::PMTPR>;
#[doc = "Register PMTPR `reset()`'s with value 0"]
impl crate::ResetValue for super::PMTPR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PRD`"]
pub type PRD_R = crate::R<u8, u8>;
#[doc = "Reader of field `PWR`"]
pub type PWR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PWR`"]
pub struct PWR_W<'a> {
    w: &'a mut W,
}
impl<'a> PWR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 8:15 - Parity Read Values for Memory Test"]
    #[inline(always)]
    pub fn prd(&self) -> PRD_R {
        PRD_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 0:7 - Parity Write Values for Memory Test"]
    #[inline(always)]
    pub fn pwr(&self) -> PWR_R {
        PWR_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Parity Write Values for Memory Test"]
    #[inline(always)]
    pub fn pwr(&mut self) -> PWR_W {
        PWR_W { w: self }
    }
}
