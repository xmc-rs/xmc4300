#[doc = "Register `CHENREG` reader"]
pub struct R(crate::R<CHENREG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CHENREG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CHENREG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CHENREG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CHENREG` writer"]
pub struct W(crate::W<CHENREG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CHENREG_SPEC>;
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
impl From<crate::W<CHENREG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CHENREG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CH` reader - Enables/Disables the channel"]
pub type CH_R = crate::FieldReader<u8, CH_A>;
#[doc = "Enables/Disables the channel\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CH_A {
    #[doc = "0: Disable the Channel"]
    VALUE1 = 0,
    #[doc = "1: Enable the Channel"]
    VALUE2 = 1,
}
impl From<CH_A> for u8 {
    #[inline(always)]
    fn from(variant: CH_A) -> Self {
        variant as _
    }
}
impl CH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CH_A> {
        match self.bits {
            0 => Some(CH_A::VALUE1),
            1 => Some(CH_A::VALUE2),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CH_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CH_A::VALUE2
    }
}
#[doc = "Field `CH` writer - Enables/Disables the channel"]
pub type CH_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CHENREG_SPEC, u8, CH_A, 8, O>;
impl<'a, const O: u8> CH_W<'a, O> {
    #[doc = "Disable the Channel"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CH_A::VALUE1)
    }
    #[doc = "Enable the Channel"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CH_A::VALUE2)
    }
}
#[doc = "Field `WE_CH` writer - Channel enable write enable"]
pub type WE_CH_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CHENREG_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Enables/Disables the channel"]
    #[inline(always)]
    pub fn ch(&self) -> CH_R {
        CH_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Enables/Disables the channel"]
    #[inline(always)]
    #[must_use]
    pub fn ch(&mut self) -> CH_W<0> {
        CH_W::new(self)
    }
    #[doc = "Bits 8:15 - Channel enable write enable"]
    #[inline(always)]
    #[must_use]
    pub fn we_ch(&mut self) -> WE_CH_W<8> {
        WE_CH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPDMA Channel Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chenreg](index.html) module"]
pub struct CHENREG_SPEC;
impl crate::RegisterSpec for CHENREG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [chenreg::R](R) reader structure"]
impl crate::Readable for CHENREG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [chenreg::W](W) writer structure"]
impl crate::Writable for CHENREG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CHENREG to value 0"]
impl crate::Resettable for CHENREG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
