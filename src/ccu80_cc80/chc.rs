#[doc = "Register `CHC` reader"]
pub struct R(crate::R<CHC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CHC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CHC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CHC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CHC` writer"]
pub struct W(crate::W<CHC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CHC_SPEC>;
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
impl From<crate::W<CHC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CHC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ASE` reader - Asymmetric PWM mode Enable"]
pub type ASE_R = crate::BitReader<ASE_A>;
#[doc = "Asymmetric PWM mode Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ASE_A {
    #[doc = "0: Asymmetric PWM is disabled"]
    VALUE1 = 0,
    #[doc = "1: Asymmetric PWM is enabled"]
    VALUE2 = 1,
}
impl From<ASE_A> for bool {
    #[inline(always)]
    fn from(variant: ASE_A) -> Self {
        variant as u8 != 0
    }
}
impl ASE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ASE_A {
        match self.bits {
            false => ASE_A::VALUE1,
            true => ASE_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == ASE_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == ASE_A::VALUE2
    }
}
#[doc = "Field `ASE` writer - Asymmetric PWM mode Enable"]
pub type ASE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHC_SPEC, ASE_A, O>;
impl<'a, const O: u8> ASE_W<'a, O> {
    #[doc = "Asymmetric PWM is disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(ASE_A::VALUE1)
    }
    #[doc = "Asymmetric PWM is enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(ASE_A::VALUE2)
    }
}
#[doc = "Field `OCS1` reader - Output selector for CCU8x.OUTy0"]
pub type OCS1_R = crate::BitReader<OCS1_A>;
#[doc = "Output selector for CCU8x.OUTy0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OCS1_A {
    #[doc = "0: CC8yST1 signal path is connected to the CCU8x.OUTy0"]
    VALUE1 = 0,
    #[doc = "1: Inverted CC8yST1 signal path is connected to the CCU8x.OUTy0"]
    VALUE2 = 1,
}
impl From<OCS1_A> for bool {
    #[inline(always)]
    fn from(variant: OCS1_A) -> Self {
        variant as u8 != 0
    }
}
impl OCS1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OCS1_A {
        match self.bits {
            false => OCS1_A::VALUE1,
            true => OCS1_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == OCS1_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == OCS1_A::VALUE2
    }
}
#[doc = "Field `OCS1` writer - Output selector for CCU8x.OUTy0"]
pub type OCS1_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHC_SPEC, OCS1_A, O>;
impl<'a, const O: u8> OCS1_W<'a, O> {
    #[doc = "CC8yST1 signal path is connected to the CCU8x.OUTy0"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(OCS1_A::VALUE1)
    }
    #[doc = "Inverted CC8yST1 signal path is connected to the CCU8x.OUTy0"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(OCS1_A::VALUE2)
    }
}
#[doc = "Field `OCS2` reader - Output selector for CCU8x.OUTy1"]
pub type OCS2_R = crate::BitReader<OCS2_A>;
#[doc = "Output selector for CCU8x.OUTy1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OCS2_A {
    #[doc = "0: Inverted CC8yST1 signal path is connected to the CCU8x.OUTy1"]
    VALUE1 = 0,
    #[doc = "1: CC8yST1 signal path is connected to the CCU8x.OUTy1"]
    VALUE2 = 1,
}
impl From<OCS2_A> for bool {
    #[inline(always)]
    fn from(variant: OCS2_A) -> Self {
        variant as u8 != 0
    }
}
impl OCS2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OCS2_A {
        match self.bits {
            false => OCS2_A::VALUE1,
            true => OCS2_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == OCS2_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == OCS2_A::VALUE2
    }
}
#[doc = "Field `OCS2` writer - Output selector for CCU8x.OUTy1"]
pub type OCS2_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHC_SPEC, OCS2_A, O>;
impl<'a, const O: u8> OCS2_W<'a, O> {
    #[doc = "Inverted CC8yST1 signal path is connected to the CCU8x.OUTy1"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(OCS2_A::VALUE1)
    }
    #[doc = "CC8yST1 signal path is connected to the CCU8x.OUTy1"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(OCS2_A::VALUE2)
    }
}
#[doc = "Field `OCS3` reader - Output selector for CCU8x.OUTy2"]
pub type OCS3_R = crate::BitReader<OCS3_A>;
#[doc = "Output selector for CCU8x.OUTy2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OCS3_A {
    #[doc = "0: CC8yST2 signal path is connected to the CCU8x.OUTy2"]
    VALUE1 = 0,
    #[doc = "1: Inverted CCST2 signal path is connected to the CCU8x.OUTy2"]
    VALUE2 = 1,
}
impl From<OCS3_A> for bool {
    #[inline(always)]
    fn from(variant: OCS3_A) -> Self {
        variant as u8 != 0
    }
}
impl OCS3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OCS3_A {
        match self.bits {
            false => OCS3_A::VALUE1,
            true => OCS3_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == OCS3_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == OCS3_A::VALUE2
    }
}
#[doc = "Field `OCS3` writer - Output selector for CCU8x.OUTy2"]
pub type OCS3_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHC_SPEC, OCS3_A, O>;
impl<'a, const O: u8> OCS3_W<'a, O> {
    #[doc = "CC8yST2 signal path is connected to the CCU8x.OUTy2"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(OCS3_A::VALUE1)
    }
    #[doc = "Inverted CCST2 signal path is connected to the CCU8x.OUTy2"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(OCS3_A::VALUE2)
    }
}
#[doc = "Field `OCS4` reader - Output selector for CCU8x.OUTy3"]
pub type OCS4_R = crate::BitReader<OCS4_A>;
#[doc = "Output selector for CCU8x.OUTy3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OCS4_A {
    #[doc = "0: Inverted CC8yST2 signal path is connected to the CCU8x.OUTy3"]
    VALUE1 = 0,
    #[doc = "1: CC8yST2 signal path is connected to the CCU8x.OUTy3"]
    VALUE2 = 1,
}
impl From<OCS4_A> for bool {
    #[inline(always)]
    fn from(variant: OCS4_A) -> Self {
        variant as u8 != 0
    }
}
impl OCS4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OCS4_A {
        match self.bits {
            false => OCS4_A::VALUE1,
            true => OCS4_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == OCS4_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == OCS4_A::VALUE2
    }
}
#[doc = "Field `OCS4` writer - Output selector for CCU8x.OUTy3"]
pub type OCS4_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHC_SPEC, OCS4_A, O>;
impl<'a, const O: u8> OCS4_W<'a, O> {
    #[doc = "Inverted CC8yST2 signal path is connected to the CCU8x.OUTy3"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(OCS4_A::VALUE1)
    }
    #[doc = "CC8yST2 signal path is connected to the CCU8x.OUTy3"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(OCS4_A::VALUE2)
    }
}
impl R {
    #[doc = "Bit 0 - Asymmetric PWM mode Enable"]
    #[inline(always)]
    pub fn ase(&self) -> ASE_R {
        ASE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Output selector for CCU8x.OUTy0"]
    #[inline(always)]
    pub fn ocs1(&self) -> OCS1_R {
        OCS1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Output selector for CCU8x.OUTy1"]
    #[inline(always)]
    pub fn ocs2(&self) -> OCS2_R {
        OCS2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Output selector for CCU8x.OUTy2"]
    #[inline(always)]
    pub fn ocs3(&self) -> OCS3_R {
        OCS3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Output selector for CCU8x.OUTy3"]
    #[inline(always)]
    pub fn ocs4(&self) -> OCS4_R {
        OCS4_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Asymmetric PWM mode Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ase(&mut self) -> ASE_W<0> {
        ASE_W::new(self)
    }
    #[doc = "Bit 1 - Output selector for CCU8x.OUTy0"]
    #[inline(always)]
    #[must_use]
    pub fn ocs1(&mut self) -> OCS1_W<1> {
        OCS1_W::new(self)
    }
    #[doc = "Bit 2 - Output selector for CCU8x.OUTy1"]
    #[inline(always)]
    #[must_use]
    pub fn ocs2(&mut self) -> OCS2_W<2> {
        OCS2_W::new(self)
    }
    #[doc = "Bit 3 - Output selector for CCU8x.OUTy2"]
    #[inline(always)]
    #[must_use]
    pub fn ocs3(&mut self) -> OCS3_W<3> {
        OCS3_W::new(self)
    }
    #[doc = "Bit 4 - Output selector for CCU8x.OUTy3"]
    #[inline(always)]
    #[must_use]
    pub fn ocs4(&mut self) -> OCS4_W<4> {
        OCS4_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Channel Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chc](index.html) module"]
pub struct CHC_SPEC;
impl crate::RegisterSpec for CHC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [chc::R](R) reader structure"]
impl crate::Readable for CHC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [chc::W](W) writer structure"]
impl crate::Writable for CHC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CHC to value 0"]
impl crate::Resettable for CHC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
