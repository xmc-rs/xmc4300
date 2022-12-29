#[doc = "Register `PSL` reader"]
pub struct R(crate::R<PSL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PSL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PSL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PSL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PSL` writer"]
pub struct W(crate::W<PSL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PSL_SPEC>;
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
impl From<crate::W<PSL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PSL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PSL` reader - Output Passive Level"]
pub type PSL_R = crate::BitReader<PSL_A>;
#[doc = "Output Passive Level\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PSL_A {
    #[doc = "0: Passive Level is LOW"]
    VALUE1 = 0,
    #[doc = "1: Passive Level is HIGH"]
    VALUE2 = 1,
}
impl From<PSL_A> for bool {
    #[inline(always)]
    fn from(variant: PSL_A) -> Self {
        variant as u8 != 0
    }
}
impl PSL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PSL_A {
        match self.bits {
            false => PSL_A::VALUE1,
            true => PSL_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PSL_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PSL_A::VALUE2
    }
}
#[doc = "Field `PSL` writer - Output Passive Level"]
pub type PSL_W<'a, const O: u8> = crate::BitWriter<'a, u32, PSL_SPEC, PSL_A, O>;
impl<'a, const O: u8> PSL_W<'a, O> {
    #[doc = "Passive Level is LOW"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(PSL_A::VALUE1)
    }
    #[doc = "Passive Level is HIGH"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(PSL_A::VALUE2)
    }
}
impl R {
    #[doc = "Bit 0 - Output Passive Level"]
    #[inline(always)]
    pub fn psl(&self) -> PSL_R {
        PSL_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Output Passive Level"]
    #[inline(always)]
    #[must_use]
    pub fn psl(&mut self) -> PSL_W<0> {
        PSL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Passive Level Config\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [psl](index.html) module"]
pub struct PSL_SPEC;
impl crate::RegisterSpec for PSL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [psl::R](R) reader structure"]
impl crate::Readable for PSL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [psl::W](W) writer structure"]
impl crate::Writable for PSL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PSL to value 0"]
impl crate::Resettable for PSL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
