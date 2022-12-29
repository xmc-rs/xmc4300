#[doc = "Register `G0ORCEN` reader"]
pub struct R(crate::R<G0ORCEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<G0ORCEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<G0ORCEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<G0ORCEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `G0ORCEN` writer"]
pub struct W(crate::W<G0ORCEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<G0ORCEN_SPEC>;
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
impl From<crate::W<G0ORCEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<G0ORCEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ENORC6` reader - Enable Out of Range Comparator, Channel 6"]
pub type ENORC6_R = crate::BitReader<ENORC6_A>;
#[doc = "Enable Out of Range Comparator, Channel 6\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ENORC6_A {
    #[doc = "0: Disabled"]
    CONST_0 = 0,
    #[doc = "1: Enabled"]
    CONST_1 = 1,
}
impl From<ENORC6_A> for bool {
    #[inline(always)]
    fn from(variant: ENORC6_A) -> Self {
        variant as u8 != 0
    }
}
impl ENORC6_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENORC6_A {
        match self.bits {
            false => ENORC6_A::CONST_0,
            true => ENORC6_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == ENORC6_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == ENORC6_A::CONST_1
    }
}
#[doc = "Field `ENORC6` writer - Enable Out of Range Comparator, Channel 6"]
pub type ENORC6_W<'a, const O: u8> = crate::BitWriter<'a, u32, G0ORCEN_SPEC, ENORC6_A, O>;
impl<'a, const O: u8> ENORC6_W<'a, O> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(ENORC6_A::CONST_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(ENORC6_A::CONST_1)
    }
}
#[doc = "Field `ENORC7` reader - Enable Out of Range Comparator, Channel 7"]
pub type ENORC7_R = crate::BitReader<ENORC7_A>;
#[doc = "Enable Out of Range Comparator, Channel 7\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ENORC7_A {
    #[doc = "0: Disabled"]
    CONST_0 = 0,
    #[doc = "1: Enabled"]
    CONST_1 = 1,
}
impl From<ENORC7_A> for bool {
    #[inline(always)]
    fn from(variant: ENORC7_A) -> Self {
        variant as u8 != 0
    }
}
impl ENORC7_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENORC7_A {
        match self.bits {
            false => ENORC7_A::CONST_0,
            true => ENORC7_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == ENORC7_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == ENORC7_A::CONST_1
    }
}
#[doc = "Field `ENORC7` writer - Enable Out of Range Comparator, Channel 7"]
pub type ENORC7_W<'a, const O: u8> = crate::BitWriter<'a, u32, G0ORCEN_SPEC, ENORC7_A, O>;
impl<'a, const O: u8> ENORC7_W<'a, O> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(ENORC7_A::CONST_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(ENORC7_A::CONST_1)
    }
}
impl R {
    #[doc = "Bit 6 - Enable Out of Range Comparator, Channel 6"]
    #[inline(always)]
    pub fn enorc6(&self) -> ENORC6_R {
        ENORC6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Enable Out of Range Comparator, Channel 7"]
    #[inline(always)]
    pub fn enorc7(&self) -> ENORC7_R {
        ENORC7_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 6 - Enable Out of Range Comparator, Channel 6"]
    #[inline(always)]
    #[must_use]
    pub fn enorc6(&mut self) -> ENORC6_W<6> {
        ENORC6_W::new(self)
    }
    #[doc = "Bit 7 - Enable Out of Range Comparator, Channel 7"]
    #[inline(always)]
    #[must_use]
    pub fn enorc7(&mut self) -> ENORC7_W<7> {
        ENORC7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Out of Range Comparator Enable Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [g0orcen](index.html) module"]
pub struct G0ORCEN_SPEC;
impl crate::RegisterSpec for G0ORCEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [g0orcen::R](R) reader structure"]
impl crate::Readable for G0ORCEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [g0orcen::W](W) writer structure"]
impl crate::Writable for G0ORCEN_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets G0ORCEN to value 0"]
impl crate::Resettable for G0ORCEN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
