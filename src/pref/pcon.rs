#[doc = "Register `PCON` reader"]
pub struct R(crate::R<PCON_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PCON_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PCON_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PCON_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PCON` writer"]
pub struct W(crate::W<PCON_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PCON_SPEC>;
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
impl From<crate::W<PCON_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PCON_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Instruction Prefetch Buffer Bypass\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IBYP_A {
    #[doc = "0: Instruction prefetch buffer not bypassed."]
    CONST_0 = 0,
    #[doc = "1: Instruction prefetch buffer bypassed."]
    CONST_1 = 1,
}
impl From<IBYP_A> for bool {
    #[inline(always)]
    fn from(variant: IBYP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IBYP` reader - Instruction Prefetch Buffer Bypass"]
pub struct IBYP_R(crate::FieldReader<bool, IBYP_A>);
impl IBYP_R {
    pub(crate) fn new(bits: bool) -> Self {
        IBYP_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IBYP_A {
        match self.bits {
            false => IBYP_A::CONST_0,
            true => IBYP_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        **self == IBYP_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        **self == IBYP_A::CONST_1
    }
}
impl core::ops::Deref for IBYP_R {
    type Target = crate::FieldReader<bool, IBYP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IBYP` writer - Instruction Prefetch Buffer Bypass"]
pub struct IBYP_W<'a> {
    w: &'a mut W,
}
impl<'a> IBYP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IBYP_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Instruction prefetch buffer not bypassed."]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(IBYP_A::CONST_0)
    }
    #[doc = "Instruction prefetch buffer bypassed."]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(IBYP_A::CONST_1)
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
#[doc = "Instruction Prefetch Buffer Invalidate\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IINV_AW {
    #[doc = "0: No effect."]
    CONST_0 = 0,
    #[doc = "1: Initiate invalidation of entire instruction cache."]
    CONST_1 = 1,
}
impl From<IINV_AW> for bool {
    #[inline(always)]
    fn from(variant: IINV_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IINV` writer - Instruction Prefetch Buffer Invalidate"]
pub struct IINV_W<'a> {
    w: &'a mut W,
}
impl<'a> IINV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IINV_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No effect."]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(IINV_AW::CONST_0)
    }
    #[doc = "Initiate invalidation of entire instruction cache."]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(IINV_AW::CONST_1)
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
#[doc = "Data Buffer Bypass\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DBYP_A {
    #[doc = "0: Prefetch Data buffer not bypassed."]
    VALUE1 = 0,
    #[doc = "1: Prefetch Data buffer bypassed."]
    VALUE2 = 1,
}
impl From<DBYP_A> for bool {
    #[inline(always)]
    fn from(variant: DBYP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DBYP` reader - Data Buffer Bypass"]
pub struct DBYP_R(crate::FieldReader<bool, DBYP_A>);
impl DBYP_R {
    pub(crate) fn new(bits: bool) -> Self {
        DBYP_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DBYP_A {
        match self.bits {
            false => DBYP_A::VALUE1,
            true => DBYP_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == DBYP_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == DBYP_A::VALUE2
    }
}
impl core::ops::Deref for DBYP_R {
    type Target = crate::FieldReader<bool, DBYP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DBYP` writer - Data Buffer Bypass"]
pub struct DBYP_W<'a> {
    w: &'a mut W,
}
impl<'a> DBYP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DBYP_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Prefetch Data buffer not bypassed."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(DBYP_A::VALUE1)
    }
    #[doc = "Prefetch Data buffer bypassed."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(DBYP_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Instruction Prefetch Buffer Bypass"]
    #[inline(always)]
    pub fn ibyp(&self) -> IBYP_R {
        IBYP_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 4 - Data Buffer Bypass"]
    #[inline(always)]
    pub fn dbyp(&self) -> DBYP_R {
        DBYP_R::new(((self.bits >> 4) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Instruction Prefetch Buffer Bypass"]
    #[inline(always)]
    pub fn ibyp(&mut self) -> IBYP_W {
        IBYP_W { w: self }
    }
    #[doc = "Bit 1 - Instruction Prefetch Buffer Invalidate"]
    #[inline(always)]
    pub fn iinv(&mut self) -> IINV_W {
        IINV_W { w: self }
    }
    #[doc = "Bit 4 - Data Buffer Bypass"]
    #[inline(always)]
    pub fn dbyp(&mut self) -> DBYP_W {
        DBYP_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Prefetch Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pcon](index.html) module"]
pub struct PCON_SPEC;
impl crate::RegisterSpec for PCON_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pcon::R](R) reader structure"]
impl crate::Readable for PCON_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pcon::W](W) writer structure"]
impl crate::Writable for PCON_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PCON to value 0"]
impl crate::Resettable for PCON_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
