#[doc = "Register `RUN_LED` reader"]
pub struct R(crate::R<RUN_LED_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RUN_LED_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RUN_LED_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RUN_LED_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RUN_LED` writer"]
pub struct W(crate::W<RUN_LED_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RUN_LED_SPEC>;
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
impl From<crate::W<RUN_LED_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RUN_LED_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "LED Code\n\nValue on reset: 14"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum LED_CODE_A {
    #[doc = "0: OFF - Init State"]
    VALUE1 = 0,
    #[doc = "1: Flash - SafeOp)"]
    VALUE2 = 1,
    #[doc = "13: Blinking - PreOp"]
    VALUE3 = 13,
    #[doc = "14: Flickering - Bootstrap"]
    VALUE4 = 14,
    #[doc = "15: On - Operational"]
    VALUE5 = 15,
}
impl From<LED_CODE_A> for u8 {
    #[inline(always)]
    fn from(variant: LED_CODE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `LED_CODE` reader - LED Code"]
pub struct LED_CODE_R(crate::FieldReader<u8, LED_CODE_A>);
impl LED_CODE_R {
    pub(crate) fn new(bits: u8) -> Self {
        LED_CODE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<LED_CODE_A> {
        match self.bits {
            0 => Some(LED_CODE_A::VALUE1),
            1 => Some(LED_CODE_A::VALUE2),
            13 => Some(LED_CODE_A::VALUE3),
            14 => Some(LED_CODE_A::VALUE4),
            15 => Some(LED_CODE_A::VALUE5),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == LED_CODE_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == LED_CODE_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        **self == LED_CODE_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        **self == LED_CODE_A::VALUE4
    }
    #[doc = "Checks if the value of the field is `VALUE5`"]
    #[inline(always)]
    pub fn is_value5(&self) -> bool {
        **self == LED_CODE_A::VALUE5
    }
}
impl core::ops::Deref for LED_CODE_R {
    type Target = crate::FieldReader<u8, LED_CODE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LED_CODE` writer - LED Code"]
pub struct LED_CODE_W<'a> {
    w: &'a mut W,
}
impl<'a> LED_CODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LED_CODE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "OFF - Init State"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(LED_CODE_A::VALUE1)
    }
    #[doc = "Flash - SafeOp)"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(LED_CODE_A::VALUE2)
    }
    #[doc = "Blinking - PreOp"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(LED_CODE_A::VALUE3)
    }
    #[doc = "Flickering - Bootstrap"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(LED_CODE_A::VALUE4)
    }
    #[doc = "On - Operational"]
    #[inline(always)]
    pub fn value5(self) -> &'a mut W {
        self.variant(LED_CODE_A::VALUE5)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u8 & 0x0f);
        self.w
    }
}
#[doc = "Enable Override\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EN_OVERR_A {
    #[doc = "0: Override disable"]
    VALUE1 = 0,
    #[doc = "1: Override enable"]
    VALUE2 = 1,
}
impl From<EN_OVERR_A> for bool {
    #[inline(always)]
    fn from(variant: EN_OVERR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EN_OVERR` reader - Enable Override"]
pub struct EN_OVERR_R(crate::FieldReader<bool, EN_OVERR_A>);
impl EN_OVERR_R {
    pub(crate) fn new(bits: bool) -> Self {
        EN_OVERR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EN_OVERR_A {
        match self.bits {
            false => EN_OVERR_A::VALUE1,
            true => EN_OVERR_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == EN_OVERR_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == EN_OVERR_A::VALUE2
    }
}
impl core::ops::Deref for EN_OVERR_R {
    type Target = crate::FieldReader<bool, EN_OVERR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EN_OVERR` writer - Enable Override"]
pub struct EN_OVERR_W<'a> {
    w: &'a mut W,
}
impl<'a> EN_OVERR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EN_OVERR_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Override disable"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(EN_OVERR_A::VALUE1)
    }
    #[doc = "Override enable"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(EN_OVERR_A::VALUE2)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u8 & 0x01) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - LED Code"]
    #[inline(always)]
    pub fn led_code(&self) -> LED_CODE_R {
        LED_CODE_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - Enable Override"]
    #[inline(always)]
    pub fn en_overr(&self) -> EN_OVERR_R {
        EN_OVERR_R::new(((self.bits >> 4) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - LED Code"]
    #[inline(always)]
    pub fn led_code(&mut self) -> LED_CODE_W {
        LED_CODE_W { w: self }
    }
    #[doc = "Bit 4 - Enable Override"]
    #[inline(always)]
    pub fn en_overr(&mut self) -> EN_OVERR_W {
        EN_OVERR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RUN LED Override\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [run_led](index.html) module"]
pub struct RUN_LED_SPEC;
impl crate::RegisterSpec for RUN_LED_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [run_led::R](R) reader structure"]
impl crate::Readable for RUN_LED_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [run_led::W](W) writer structure"]
impl crate::Writable for RUN_LED_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RUN_LED to value 0x0e"]
impl crate::Resettable for RUN_LED_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0e
    }
}
