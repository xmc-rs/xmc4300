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
#[doc = "Field `WE_CH` writer - Channel enable write enable"]
pub struct WE_CH_W<'a> {
    w: &'a mut W,
}
impl<'a> WE_CH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
#[doc = "Enables/Disables the channel\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Field `CH` reader - Enables/Disables the channel"]
pub struct CH_R(crate::FieldReader<u8, CH_A>);
impl CH_R {
    pub(crate) fn new(bits: u8) -> Self {
        CH_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
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
        **self == CH_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == CH_A::VALUE2
    }
}
impl core::ops::Deref for CH_R {
    type Target = crate::FieldReader<u8, CH_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH` writer - Enables/Disables the channel"]
pub struct CH_W<'a> {
    w: &'a mut W,
}
impl<'a> CH_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CH_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
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
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Enables/Disables the channel"]
    #[inline(always)]
    pub fn ch(&self) -> CH_R {
        CH_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 8:15 - Channel enable write enable"]
    #[inline(always)]
    pub fn we_ch(&mut self) -> WE_CH_W {
        WE_CH_W { w: self }
    }
    #[doc = "Bits 0:7 - Enables/Disables the channel"]
    #[inline(always)]
    pub fn ch(&mut self) -> CH_W {
        CH_W { w: self }
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
}
#[doc = "`reset()` method sets CHENREG to value 0"]
impl crate::Resettable for CHENREG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
