#[doc = "Register `TIMEOUT_CTRL` reader"]
pub struct R(crate::R<TIMEOUT_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIMEOUT_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIMEOUT_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIMEOUT_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TIMEOUT_CTRL` writer"]
pub struct W(crate::W<TIMEOUT_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIMEOUT_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<TIMEOUT_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIMEOUT_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Data Timeout Counter Value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DAT_TIMEOUT_CNT_VAL_A {
    #[doc = "0: TMCLK * 2^13"]
    VALUE1 = 0,
    #[doc = "1: TMCLK * 2^14"]
    VALUE2 = 1,
    #[doc = "14: TMCLK * 2^27"]
    VALUE3 = 14,
}
impl From<DAT_TIMEOUT_CNT_VAL_A> for u8 {
    #[inline(always)]
    fn from(variant: DAT_TIMEOUT_CNT_VAL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `DAT_TIMEOUT_CNT_VAL` reader - Data Timeout Counter Value"]
pub struct DAT_TIMEOUT_CNT_VAL_R(crate::FieldReader<u8, DAT_TIMEOUT_CNT_VAL_A>);
impl DAT_TIMEOUT_CNT_VAL_R {
    pub(crate) fn new(bits: u8) -> Self {
        DAT_TIMEOUT_CNT_VAL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<DAT_TIMEOUT_CNT_VAL_A> {
        match self.bits {
            0 => Some(DAT_TIMEOUT_CNT_VAL_A::VALUE1),
            1 => Some(DAT_TIMEOUT_CNT_VAL_A::VALUE2),
            14 => Some(DAT_TIMEOUT_CNT_VAL_A::VALUE3),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == DAT_TIMEOUT_CNT_VAL_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == DAT_TIMEOUT_CNT_VAL_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        **self == DAT_TIMEOUT_CNT_VAL_A::VALUE3
    }
}
impl core::ops::Deref for DAT_TIMEOUT_CNT_VAL_R {
    type Target = crate::FieldReader<u8, DAT_TIMEOUT_CNT_VAL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DAT_TIMEOUT_CNT_VAL` writer - Data Timeout Counter Value"]
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
        self.w.bits = (self.w.bits & !0x0f) | (value as u8 & 0x0f);
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
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timeout Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timeout_ctrl](index.html) module"]
pub struct TIMEOUT_CTRL_SPEC;
impl crate::RegisterSpec for TIMEOUT_CTRL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [timeout_ctrl::R](R) reader structure"]
impl crate::Readable for TIMEOUT_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [timeout_ctrl::W](W) writer structure"]
impl crate::Writable for TIMEOUT_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TIMEOUT_CTRL to value 0"]
impl crate::Resettable for TIMEOUT_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
