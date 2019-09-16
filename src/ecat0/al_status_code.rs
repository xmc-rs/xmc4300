#[doc = "Reader of register AL_STATUS_CODE"]
pub type R = crate::R<u16, super::AL_STATUS_CODE>;
#[doc = "Writer for register AL_STATUS_CODE"]
pub type W = crate::W<u16, super::AL_STATUS_CODE>;
#[doc = "Register AL_STATUS_CODE `reset()`'s with value 0"]
impl crate::ResetValue for super::AL_STATUS_CODE {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `AL_S_CODE`"]
pub type AL_S_CODE_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `AL_S_CODE`"]
pub struct AL_S_CODE_W<'a> {
    w: &'a mut W,
}
impl<'a> AL_S_CODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u16) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - AL Status Code"]
    #[inline(always)]
    pub fn al_s_code(&self) -> AL_S_CODE_R {
        AL_S_CODE_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - AL Status Code"]
    #[inline(always)]
    pub fn al_s_code(&mut self) -> AL_S_CODE_W {
        AL_S_CODE_W { w: self }
    }
}
