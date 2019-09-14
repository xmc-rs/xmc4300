#[doc = "Reader of register PSC"]
pub type R = crate::R<u32, super::PSC>;
#[doc = "Writer for register PSC"]
pub type W = crate::W<u32, super::PSC>;
#[doc = "Register PSC `reset()`'s with value 0"]
impl crate::ResetValue for super::PSC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PSIV`"]
pub type PSIV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PSIV`"]
pub struct PSIV_W<'a> {
    w: &'a mut W,
}
impl<'a> PSIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Prescaler Initial Value"]
    #[inline(always)]
    pub fn psiv(&self) -> PSIV_R {
        PSIV_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Prescaler Initial Value"]
    #[inline(always)]
    pub fn psiv(&mut self) -> PSIV_W {
        PSIV_W { w: self }
    }
}
