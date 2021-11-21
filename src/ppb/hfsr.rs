#[doc = "Register `HFSR` reader"]
pub struct R(crate::R<HFSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HFSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HFSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HFSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HFSR` writer"]
pub struct W(crate::W<HFSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HFSR_SPEC>;
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
impl From<crate::W<HFSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HFSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "BusFault on vector table read\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VECTTBL_A {
    #[doc = "0: no BusFault on vector table read"]
    VALUE1 = 0,
    #[doc = "1: BusFault on vector table read"]
    VALUE2 = 1,
}
impl From<VECTTBL_A> for bool {
    #[inline(always)]
    fn from(variant: VECTTBL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VECTTBL` reader - BusFault on vector table read"]
pub struct VECTTBL_R(crate::FieldReader<bool, VECTTBL_A>);
impl VECTTBL_R {
    pub(crate) fn new(bits: bool) -> Self {
        VECTTBL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VECTTBL_A {
        match self.bits {
            false => VECTTBL_A::VALUE1,
            true => VECTTBL_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == VECTTBL_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == VECTTBL_A::VALUE2
    }
}
impl core::ops::Deref for VECTTBL_R {
    type Target = crate::FieldReader<bool, VECTTBL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VECTTBL` writer - BusFault on vector table read"]
pub struct VECTTBL_W<'a> {
    w: &'a mut W,
}
impl<'a> VECTTBL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: VECTTBL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "no BusFault on vector table read"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(VECTTBL_A::VALUE1)
    }
    #[doc = "BusFault on vector table read"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(VECTTBL_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Forced HardFault\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FORCED_A {
    #[doc = "0: no forced HardFault"]
    VALUE1 = 0,
    #[doc = "1: forced HardFault."]
    VALUE2 = 1,
}
impl From<FORCED_A> for bool {
    #[inline(always)]
    fn from(variant: FORCED_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FORCED` reader - Forced HardFault"]
pub struct FORCED_R(crate::FieldReader<bool, FORCED_A>);
impl FORCED_R {
    pub(crate) fn new(bits: bool) -> Self {
        FORCED_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FORCED_A {
        match self.bits {
            false => FORCED_A::VALUE1,
            true => FORCED_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == FORCED_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == FORCED_A::VALUE2
    }
}
impl core::ops::Deref for FORCED_R {
    type Target = crate::FieldReader<bool, FORCED_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FORCED` writer - Forced HardFault"]
pub struct FORCED_W<'a> {
    w: &'a mut W,
}
impl<'a> FORCED_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FORCED_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "no forced HardFault"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(FORCED_A::VALUE1)
    }
    #[doc = "forced HardFault."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(FORCED_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | ((value as u32 & 0x01) << 30);
        self.w
    }
}
#[doc = "Field `DEBUGEVT` reader - Reserved for Debug use"]
pub struct DEBUGEVT_R(crate::FieldReader<bool, bool>);
impl DEBUGEVT_R {
    pub(crate) fn new(bits: bool) -> Self {
        DEBUGEVT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DEBUGEVT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DEBUGEVT` writer - Reserved for Debug use"]
pub struct DEBUGEVT_W<'a> {
    w: &'a mut W,
}
impl<'a> DEBUGEVT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | ((value as u32 & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bit 1 - BusFault on vector table read"]
    #[inline(always)]
    pub fn vecttbl(&self) -> VECTTBL_R {
        VECTTBL_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Forced HardFault"]
    #[inline(always)]
    pub fn forced(&self) -> FORCED_R {
        FORCED_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Reserved for Debug use"]
    #[inline(always)]
    pub fn debugevt(&self) -> DEBUGEVT_R {
        DEBUGEVT_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - BusFault on vector table read"]
    #[inline(always)]
    pub fn vecttbl(&mut self) -> VECTTBL_W {
        VECTTBL_W { w: self }
    }
    #[doc = "Bit 30 - Forced HardFault"]
    #[inline(always)]
    pub fn forced(&mut self) -> FORCED_W {
        FORCED_W { w: self }
    }
    #[doc = "Bit 31 - Reserved for Debug use"]
    #[inline(always)]
    pub fn debugevt(&mut self) -> DEBUGEVT_W {
        DEBUGEVT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "HardFault Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hfsr](index.html) module"]
pub struct HFSR_SPEC;
impl crate::RegisterSpec for HFSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hfsr::R](R) reader structure"]
impl crate::Readable for HFSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hfsr::W](W) writer structure"]
impl crate::Writable for HFSR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HFSR to value 0"]
impl crate::Resettable for HFSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
