#[doc = "Register `ICLASS[%s]` reader"]
pub struct R(crate::R<ICLASS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ICLASS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ICLASS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ICLASS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ICLASS[%s]` writer"]
pub struct W(crate::W<ICLASS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ICLASS_SPEC>;
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
impl From<crate::W<ICLASS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ICLASS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `STCS` reader - Sample Time Control for Standard Conversions"]
pub type STCS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `STCS` writer - Sample Time Control for Standard Conversions"]
pub type STCS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ICLASS_SPEC, u8, u8, 5, O>;
#[doc = "Field `CMS` reader - Conversion Mode for Standard Conversions"]
pub type CMS_R = crate::FieldReader<u8, CMS_A>;
#[doc = "Conversion Mode for Standard Conversions\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CMS_A {
    #[doc = "0: 12-bit conversion"]
    VALUE1 = 0,
    #[doc = "1: 10-bit conversion"]
    VALUE2 = 1,
    #[doc = "2: 8-bit conversion"]
    VALUE3 = 2,
    #[doc = "5: 10-bit fast compare mode"]
    VALUE6 = 5,
}
impl From<CMS_A> for u8 {
    #[inline(always)]
    fn from(variant: CMS_A) -> Self {
        variant as _
    }
}
impl CMS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CMS_A> {
        match self.bits {
            0 => Some(CMS_A::VALUE1),
            1 => Some(CMS_A::VALUE2),
            2 => Some(CMS_A::VALUE3),
            5 => Some(CMS_A::VALUE6),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CMS_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CMS_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == CMS_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE6`"]
    #[inline(always)]
    pub fn is_value6(&self) -> bool {
        *self == CMS_A::VALUE6
    }
}
#[doc = "Field `CMS` writer - Conversion Mode for Standard Conversions"]
pub type CMS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ICLASS_SPEC, u8, CMS_A, 3, O>;
impl<'a, const O: u8> CMS_W<'a, O> {
    #[doc = "12-bit conversion"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CMS_A::VALUE1)
    }
    #[doc = "10-bit conversion"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CMS_A::VALUE2)
    }
    #[doc = "8-bit conversion"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(CMS_A::VALUE3)
    }
    #[doc = "10-bit fast compare mode"]
    #[inline(always)]
    pub fn value6(self) -> &'a mut W {
        self.variant(CMS_A::VALUE6)
    }
}
#[doc = "Field `STCE` reader - Sample Time Control for EMUX Conversions"]
pub type STCE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `STCE` writer - Sample Time Control for EMUX Conversions"]
pub type STCE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ICLASS_SPEC, u8, u8, 5, O>;
#[doc = "Field `CME` reader - Conversion Mode for EMUX Conversions"]
pub type CME_R = crate::FieldReader<u8, CME_A>;
#[doc = "Conversion Mode for EMUX Conversions\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CME_A {
    #[doc = "0: 12-bit conversion"]
    VALUE1 = 0,
    #[doc = "1: 10-bit conversion"]
    VALUE2 = 1,
    #[doc = "2: 8-bit conversion"]
    VALUE3 = 2,
    #[doc = "5: 10-bit fast compare mode"]
    VALUE6 = 5,
}
impl From<CME_A> for u8 {
    #[inline(always)]
    fn from(variant: CME_A) -> Self {
        variant as _
    }
}
impl CME_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CME_A> {
        match self.bits {
            0 => Some(CME_A::VALUE1),
            1 => Some(CME_A::VALUE2),
            2 => Some(CME_A::VALUE3),
            5 => Some(CME_A::VALUE6),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CME_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CME_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == CME_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE6`"]
    #[inline(always)]
    pub fn is_value6(&self) -> bool {
        *self == CME_A::VALUE6
    }
}
#[doc = "Field `CME` writer - Conversion Mode for EMUX Conversions"]
pub type CME_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ICLASS_SPEC, u8, CME_A, 3, O>;
impl<'a, const O: u8> CME_W<'a, O> {
    #[doc = "12-bit conversion"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CME_A::VALUE1)
    }
    #[doc = "10-bit conversion"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CME_A::VALUE2)
    }
    #[doc = "8-bit conversion"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(CME_A::VALUE3)
    }
    #[doc = "10-bit fast compare mode"]
    #[inline(always)]
    pub fn value6(self) -> &'a mut W {
        self.variant(CME_A::VALUE6)
    }
}
impl R {
    #[doc = "Bits 0:4 - Sample Time Control for Standard Conversions"]
    #[inline(always)]
    pub fn stcs(&self) -> STCS_R {
        STCS_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:10 - Conversion Mode for Standard Conversions"]
    #[inline(always)]
    pub fn cms(&self) -> CMS_R {
        CMS_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 16:20 - Sample Time Control for EMUX Conversions"]
    #[inline(always)]
    pub fn stce(&self) -> STCE_R {
        STCE_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:26 - Conversion Mode for EMUX Conversions"]
    #[inline(always)]
    pub fn cme(&self) -> CME_R {
        CME_R::new(((self.bits >> 24) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Sample Time Control for Standard Conversions"]
    #[inline(always)]
    #[must_use]
    pub fn stcs(&mut self) -> STCS_W<0> {
        STCS_W::new(self)
    }
    #[doc = "Bits 8:10 - Conversion Mode for Standard Conversions"]
    #[inline(always)]
    #[must_use]
    pub fn cms(&mut self) -> CMS_W<8> {
        CMS_W::new(self)
    }
    #[doc = "Bits 16:20 - Sample Time Control for EMUX Conversions"]
    #[inline(always)]
    #[must_use]
    pub fn stce(&mut self) -> STCE_W<16> {
        STCE_W::new(self)
    }
    #[doc = "Bits 24:26 - Conversion Mode for EMUX Conversions"]
    #[inline(always)]
    #[must_use]
    pub fn cme(&mut self) -> CME_W<24> {
        CME_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Input Class Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iclass](index.html) module"]
pub struct ICLASS_SPEC;
impl crate::RegisterSpec for ICLASS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [iclass::R](R) reader structure"]
impl crate::Readable for ICLASS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [iclass::W](W) writer structure"]
impl crate::Writable for ICLASS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ICLASS[%s]
to value 0"]
impl crate::Resettable for ICLASS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
