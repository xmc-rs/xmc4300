#[doc = "Register `STC` reader"]
pub struct R(crate::R<STC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `STC` writer"]
pub struct W(crate::W<STC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STC_SPEC>;
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
impl From<crate::W<STC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<STC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Cascaded shadow transfer enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CSE_A {
    #[doc = "0: Cascaded shadow transfer disabled"]
    VALUE1 = 0,
    #[doc = "1: Cascaded shadow transfer enabled"]
    VALUE2 = 1,
}
impl From<CSE_A> for bool {
    #[inline(always)]
    fn from(variant: CSE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CSE` reader - Cascaded shadow transfer enable"]
pub struct CSE_R(crate::FieldReader<bool, CSE_A>);
impl CSE_R {
    pub(crate) fn new(bits: bool) -> Self {
        CSE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CSE_A {
        match self.bits {
            false => CSE_A::VALUE1,
            true => CSE_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == CSE_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == CSE_A::VALUE2
    }
}
impl core::ops::Deref for CSE_R {
    type Target = crate::FieldReader<bool, CSE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CSE` writer - Cascaded shadow transfer enable"]
pub struct CSE_W<'a> {
    w: &'a mut W,
}
impl<'a> CSE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CSE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Cascaded shadow transfer disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CSE_A::VALUE1)
    }
    #[doc = "Cascaded shadow transfer enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CSE_A::VALUE2)
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "Shadow transfer mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum STM_A {
    #[doc = "0: Shadow transfer is done in Period Match and One match."]
    VALUE1 = 0,
    #[doc = "1: Shadow transfer is done only in Period Match."]
    VALUE2 = 1,
    #[doc = "2: Shadow transfer is done only in One Match."]
    VALUE3 = 2,
}
impl From<STM_A> for u8 {
    #[inline(always)]
    fn from(variant: STM_A) -> Self {
        variant as _
    }
}
#[doc = "Field `STM` reader - Shadow transfer mode"]
pub struct STM_R(crate::FieldReader<u8, STM_A>);
impl STM_R {
    pub(crate) fn new(bits: u8) -> Self {
        STM_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<STM_A> {
        match self.bits {
            0 => Some(STM_A::VALUE1),
            1 => Some(STM_A::VALUE2),
            2 => Some(STM_A::VALUE3),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == STM_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == STM_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        **self == STM_A::VALUE3
    }
}
impl core::ops::Deref for STM_R {
    type Target = crate::FieldReader<u8, STM_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STM` writer - Shadow transfer mode"]
pub struct STM_W<'a> {
    w: &'a mut W,
}
impl<'a> STM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: STM_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Shadow transfer is done in Period Match and One match."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(STM_A::VALUE1)
    }
    #[doc = "Shadow transfer is done only in Period Match."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(STM_A::VALUE2)
    }
    #[doc = "Shadow transfer is done only in One Match."]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(STM_A::VALUE3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 1)) | ((value as u32 & 0x03) << 1);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Cascaded shadow transfer enable"]
    #[inline(always)]
    pub fn cse(&self) -> CSE_R {
        CSE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 1:2 - Shadow transfer mode"]
    #[inline(always)]
    pub fn stm(&self) -> STM_R {
        STM_R::new(((self.bits >> 1) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Cascaded shadow transfer enable"]
    #[inline(always)]
    pub fn cse(&mut self) -> CSE_W {
        CSE_W { w: self }
    }
    #[doc = "Bits 1:2 - Shadow transfer mode"]
    #[inline(always)]
    pub fn stm(&mut self) -> STM_W {
        STM_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Shadow transfer control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stc](index.html) module"]
pub struct STC_SPEC;
impl crate::RegisterSpec for STC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [stc::R](R) reader structure"]
impl crate::Readable for STC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [stc::W](W) writer structure"]
impl crate::Writable for STC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets STC to value 0"]
impl crate::Resettable for STC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
