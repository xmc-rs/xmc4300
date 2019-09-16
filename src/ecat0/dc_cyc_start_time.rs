#[doc = "Reader of register DC_CYC_START_TIME[%s]"]
pub type R = crate::R<u32, super::DC_CYC_START_TIME>;
#[doc = "Writer for register DC_CYC_START_TIME[%s]"]
pub type W = crate::W<u32, super::DC_CYC_START_TIME>;
#[doc = "Register DC_CYC_START_TIME[%s] `reset()`'s with value 0"]
impl crate::ResetValue for super::DC_CYC_START_TIME {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DC_CYC_START_TIME`"]
pub type DC_CYC_START_TIME_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `DC_CYC_START_TIME`"]
pub struct DC_CYC_START_TIME_W<'a> {
    w: &'a mut W,
}
impl<'a> DC_CYC_START_TIME_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Start Time Cyclic Operation"]
    #[inline(always)]
    pub fn dc_cyc_start_time(&self) -> DC_CYC_START_TIME_R {
        DC_CYC_START_TIME_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Start Time Cyclic Operation"]
    #[inline(always)]
    pub fn dc_cyc_start_time(&mut self) -> DC_CYC_START_TIME_W {
        DC_CYC_START_TIME_W { w: self }
    }
}
