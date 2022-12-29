#[doc = "Register `ALIAS` reader"]
pub struct R(crate::R<ALIAS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ALIAS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ALIAS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ALIAS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ALIAS` writer"]
pub struct W(crate::W<ALIAS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ALIAS_SPEC>;
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
impl From<crate::W<ALIAS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ALIAS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ALIAS0` reader - Alias Value for CH0 Conversion Requests"]
pub type ALIAS0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ALIAS0` writer - Alias Value for CH0 Conversion Requests"]
pub type ALIAS0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ALIAS_SPEC, u8, u8, 5, O>;
#[doc = "Field `ALIAS1` reader - Alias Value for CH1 Conversion Requests"]
pub type ALIAS1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ALIAS1` writer - Alias Value for CH1 Conversion Requests"]
pub type ALIAS1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ALIAS_SPEC, u8, u8, 5, O>;
impl R {
    #[doc = "Bits 0:4 - Alias Value for CH0 Conversion Requests"]
    #[inline(always)]
    pub fn alias0(&self) -> ALIAS0_R {
        ALIAS0_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - Alias Value for CH1 Conversion Requests"]
    #[inline(always)]
    pub fn alias1(&self) -> ALIAS1_R {
        ALIAS1_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Alias Value for CH0 Conversion Requests"]
    #[inline(always)]
    #[must_use]
    pub fn alias0(&mut self) -> ALIAS0_W<0> {
        ALIAS0_W::new(self)
    }
    #[doc = "Bits 8:12 - Alias Value for CH1 Conversion Requests"]
    #[inline(always)]
    #[must_use]
    pub fn alias1(&mut self) -> ALIAS1_W<8> {
        ALIAS1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Alias Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [alias](index.html) module"]
pub struct ALIAS_SPEC;
impl crate::RegisterSpec for ALIAS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [alias::R](R) reader structure"]
impl crate::Readable for ALIAS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [alias::W](W) writer structure"]
impl crate::Writable for ALIAS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ALIAS to value 0x0100"]
impl crate::Resettable for ALIAS_SPEC {
    const RESET_VALUE: Self::Ux = 0x0100;
}
