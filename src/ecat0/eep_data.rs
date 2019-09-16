#[doc = "Reader of register EEP_DATA[%s]"]
pub type R = crate::R<u32, super::EEP_DATA>;
#[doc = "Writer for register EEP_DATA[%s]"]
pub type W = crate::W<u32, super::EEP_DATA>;
#[doc = "Register EEP_DATA[%s] `reset()`'s with value 0"]
impl crate::ResetValue for super::EEP_DATA {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `EEP_DATA`"]
pub type EEP_DATA_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `EEP_DATA`"]
pub struct EEP_DATA_W<'a> {
    w: &'a mut W,
}
impl<'a> EEP_DATA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - EEPROM Data"]
    #[inline(always)]
    pub fn eep_data(&self) -> EEP_DATA_R {
        EEP_DATA_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - EEPROM Data"]
    #[inline(always)]
    pub fn eep_data(&mut self) -> EEP_DATA_W {
        EEP_DATA_W { w: self }
    }
}
