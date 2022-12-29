#[doc = "Register `ERR_LED` reader"]
pub struct R(crate::R<ERR_LED_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ERR_LED_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ERR_LED_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ERR_LED_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ERR_LED` writer"]
pub struct W(crate::W<ERR_LED_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ERR_LED_SPEC>;
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
impl From<crate::W<ERR_LED_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ERR_LED_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LED_CODE` reader - LED Code"]
pub type LED_CODE_R = crate::FieldReader<u8, LED_CODE_A>;
#[doc = "LED Code\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LED_CODE_A {
    #[doc = "0: OFF"]
    VALUE1 = 0,
    #[doc = "13: Blinking"]
    VALUE2 = 13,
    #[doc = "14: Flickering"]
    VALUE3 = 14,
    #[doc = "15: On"]
    VALUE4 = 15,
}
impl From<LED_CODE_A> for u8 {
    #[inline(always)]
    fn from(variant: LED_CODE_A) -> Self {
        variant as _
    }
}
impl LED_CODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<LED_CODE_A> {
        match self.bits {
            0 => Some(LED_CODE_A::VALUE1),
            13 => Some(LED_CODE_A::VALUE2),
            14 => Some(LED_CODE_A::VALUE3),
            15 => Some(LED_CODE_A::VALUE4),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == LED_CODE_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == LED_CODE_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == LED_CODE_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == LED_CODE_A::VALUE4
    }
}
#[doc = "Field `LED_CODE` writer - LED Code"]
pub type LED_CODE_W<'a, const O: u8> = crate::FieldWriter<'a, u8, ERR_LED_SPEC, u8, LED_CODE_A, 4, O>;
impl<'a, const O: u8> LED_CODE_W<'a, O> {
    #[doc = "OFF"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(LED_CODE_A::VALUE1)
    }
    #[doc = "Blinking"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(LED_CODE_A::VALUE2)
    }
    #[doc = "Flickering"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(LED_CODE_A::VALUE3)
    }
    #[doc = "On"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(LED_CODE_A::VALUE4)
    }
}
#[doc = "Field `EN_OVERR` reader - Enable Override"]
pub type EN_OVERR_R = crate::BitReader<EN_OVERR_A>;
#[doc = "Enable Override\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl EN_OVERR_R {
    #[doc = "Get enumerated values variant"]
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
        *self == EN_OVERR_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == EN_OVERR_A::VALUE2
    }
}
#[doc = "Field `EN_OVERR` writer - Enable Override"]
pub type EN_OVERR_W<'a, const O: u8> = crate::BitWriter<'a, u8, ERR_LED_SPEC, EN_OVERR_A, O>;
impl<'a, const O: u8> EN_OVERR_W<'a, O> {
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
}
impl R {
    #[doc = "Bits 0:3 - LED Code"]
    #[inline(always)]
    pub fn led_code(&self) -> LED_CODE_R {
        LED_CODE_R::new(self.bits & 0x0f)
    }
    #[doc = "Bit 4 - Enable Override"]
    #[inline(always)]
    pub fn en_overr(&self) -> EN_OVERR_R {
        EN_OVERR_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - LED Code"]
    #[inline(always)]
    #[must_use]
    pub fn led_code(&mut self) -> LED_CODE_W<0> {
        LED_CODE_W::new(self)
    }
    #[doc = "Bit 4 - Enable Override"]
    #[inline(always)]
    #[must_use]
    pub fn en_overr(&mut self) -> EN_OVERR_W<4> {
        EN_OVERR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RUN ERR Override\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [err_led](index.html) module"]
pub struct ERR_LED_SPEC;
impl crate::RegisterSpec for ERR_LED_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [err_led::R](R) reader structure"]
impl crate::Readable for ERR_LED_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [err_led::W](W) writer structure"]
impl crate::Writable for ERR_LED_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ERR_LED to value 0"]
impl crate::Resettable for ERR_LED_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
