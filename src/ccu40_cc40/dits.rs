#[doc = "Register `DITS` reader"]
pub struct R(crate::R<DITS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DITS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DITS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DITS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DITS` writer"]
pub struct W(crate::W<DITS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DITS_SPEC>;
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
impl From<crate::W<DITS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DITS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DCVS` reader - Dither Shadow Compare Value"]
pub type DCVS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DCVS` writer - Dither Shadow Compare Value"]
pub type DCVS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DITS_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:3 - Dither Shadow Compare Value"]
    #[inline(always)]
    pub fn dcvs(&self) -> DCVS_R {
        DCVS_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Dither Shadow Compare Value"]
    #[inline(always)]
    #[must_use]
    pub fn dcvs(&mut self) -> DCVS_W<0> {
        DCVS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Dither Shadow Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dits](index.html) module"]
pub struct DITS_SPEC;
impl crate::RegisterSpec for DITS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dits::R](R) reader structure"]
impl crate::Readable for DITS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dits::W](W) writer structure"]
impl crate::Writable for DITS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DITS to value 0"]
impl crate::Resettable for DITS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
