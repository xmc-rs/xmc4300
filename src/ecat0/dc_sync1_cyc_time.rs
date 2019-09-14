#[doc = "Reader of register DC_SYNC1_CYC_TIME"]
pub type R = crate::R<u32, super::DC_SYNC1_CYC_TIME>;
#[doc = "Writer for register DC_SYNC1_CYC_TIME"]
pub type W = crate::W<u32, super::DC_SYNC1_CYC_TIME>;
#[doc = "Register DC_SYNC1_CYC_TIME `reset()`'s with value 0"]
impl crate::ResetValue for super::DC_SYNC1_CYC_TIME {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TIME_SYNC1_SYNC0`"]
pub type TIME_SYNC1_SYNC0_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `TIME_SYNC1_SYNC0`"]
pub struct TIME_SYNC1_SYNC0_W<'a> {
    w: &'a mut W,
}
impl<'a> TIME_SYNC1_SYNC0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Time between SYNC1 pulses and SYNC0 pulse"]
    #[inline(always)]
    pub fn time_sync1_sync0(&self) -> TIME_SYNC1_SYNC0_R {
        TIME_SYNC1_SYNC0_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Time between SYNC1 pulses and SYNC0 pulse"]
    #[inline(always)]
    pub fn time_sync1_sync0(&mut self) -> TIME_SYNC1_SYNC0_W {
        TIME_SYNC1_SYNC0_W { w: self }
    }
}
