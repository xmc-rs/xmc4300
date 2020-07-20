#[doc = "Reader of register DC_SYNC0_CYC_TIME"]
pub type R = crate::R<u32, super::DC_SYNC0_CYC_TIME>;
#[doc = "Writer for register DC_SYNC0_CYC_TIME"]
pub type W = crate::W<u32, super::DC_SYNC0_CYC_TIME>;
#[doc = "Register DC_SYNC0_CYC_TIME `reset()`'s with value 0"]
impl crate::ResetValue for super::DC_SYNC0_CYC_TIME {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Time between two consecutive SYNC0 pulses\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u32)]
pub enum TIME_BETWEEN_SYNC0_A {
    #[doc = "0: Single shot mode, generate only one SYNC0 pulse"]
    VALUE1 = 0,
}
impl From<TIME_BETWEEN_SYNC0_A> for u32 {
    #[inline(always)]
    fn from(variant: TIME_BETWEEN_SYNC0_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `TIME_BETWEEN_SYNC0`"]
pub type TIME_BETWEEN_SYNC0_R = crate::R<u32, TIME_BETWEEN_SYNC0_A>;
impl TIME_BETWEEN_SYNC0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u32, TIME_BETWEEN_SYNC0_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(TIME_BETWEEN_SYNC0_A::VALUE1),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == TIME_BETWEEN_SYNC0_A::VALUE1
    }
}
#[doc = "Write proxy for field `TIME_BETWEEN_SYNC0`"]
pub struct TIME_BETWEEN_SYNC0_W<'a> {
    w: &'a mut W,
}
impl<'a> TIME_BETWEEN_SYNC0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIME_BETWEEN_SYNC0_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Single shot mode, generate only one SYNC0 pulse"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(TIME_BETWEEN_SYNC0_A::VALUE1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Time between two consecutive SYNC0 pulses"]
    #[inline(always)]
    pub fn time_between_sync0(&self) -> TIME_BETWEEN_SYNC0_R {
        TIME_BETWEEN_SYNC0_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Time between two consecutive SYNC0 pulses"]
    #[inline(always)]
    pub fn time_between_sync0(&mut self) -> TIME_BETWEEN_SYNC0_W {
        TIME_BETWEEN_SYNC0_W { w: self }
    }
}
