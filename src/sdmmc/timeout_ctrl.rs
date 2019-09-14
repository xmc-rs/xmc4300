#[doc = "Reader of register TIMEOUT_CTRL"]
pub type R = crate::R<u8, super::TIMEOUT_CTRL>;
#[doc = "Writer for register TIMEOUT_CTRL"]
pub type W = crate::W<u8, super::TIMEOUT_CTRL>;
#[doc = "Register TIMEOUT_CTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::TIMEOUT_CTRL {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Data Timeout Counter Value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DAT_TIMEOUT_CNT_VAL_A {
    #[doc = "0: TMCLK * 2^13"]
    VALUE1,
    #[doc = "1: TMCLK * 2^14"]
    VALUE2,
    #[doc = "14: TMCLK * 2^27"]
    VALUE3,
}
impl From<DAT_TIMEOUT_CNT_VAL_A> for u8 {
    #[inline(always)]
    fn from(variant: DAT_TIMEOUT_CNT_VAL_A) -> Self {
        match variant {
            DAT_TIMEOUT_CNT_VAL_A::VALUE1 => 0,
            DAT_TIMEOUT_CNT_VAL_A::VALUE2 => 1,
            DAT_TIMEOUT_CNT_VAL_A::VALUE3 => 14,
        }
    }
}
#[doc = "Reader of field `DAT_TIMEOUT_CNT_VAL`"]
pub type DAT_TIMEOUT_CNT_VAL_R = crate::R<u8, DAT_TIMEOUT_CNT_VAL_A>;
impl DAT_TIMEOUT_CNT_VAL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, DAT_TIMEOUT_CNT_VAL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(DAT_TIMEOUT_CNT_VAL_A::VALUE1),
            1 => Val(DAT_TIMEOUT_CNT_VAL_A::VALUE2),
            14 => Val(DAT_TIMEOUT_CNT_VAL_A::VALUE3),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == DAT_TIMEOUT_CNT_VAL_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == DAT_TIMEOUT_CNT_VAL_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == DAT_TIMEOUT_CNT_VAL_A::VALUE3
    }
}
#[doc = "Write proxy for field `DAT_TIMEOUT_CNT_VAL`"]
pub struct DAT_TIMEOUT_CNT_VAL_W<'a> {
    w: &'a mut W,
}
impl<'a> DAT_TIMEOUT_CNT_VAL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DAT_TIMEOUT_CNT_VAL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "TMCLK * 2^13"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(DAT_TIMEOUT_CNT_VAL_A::VALUE1)
    }
    #[doc = "TMCLK * 2^14"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(DAT_TIMEOUT_CNT_VAL_A::VALUE2)
    }
    #[doc = "TMCLK * 2^27"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(DAT_TIMEOUT_CNT_VAL_A::VALUE3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u8) & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Data Timeout Counter Value"]
    #[inline(always)]
    pub fn dat_timeout_cnt_val(&self) -> DAT_TIMEOUT_CNT_VAL_R {
        DAT_TIMEOUT_CNT_VAL_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Data Timeout Counter Value"]
    #[inline(always)]
    pub fn dat_timeout_cnt_val(&mut self) -> DAT_TIMEOUT_CNT_VAL_W {
        DAT_TIMEOUT_CNT_VAL_W { w: self }
    }
}
