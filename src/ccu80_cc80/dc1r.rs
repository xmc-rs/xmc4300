#[doc = "Register `DC1R` reader"]
pub struct R(crate::R<DC1R_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DC1R_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DC1R_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DC1R_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DC1R` writer"]
pub struct W(crate::W<DC1R_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DC1R_SPEC>;
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
impl From<crate::W<DC1R_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DC1R_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DT1R` reader - Rise Value for Dead Time of Channel 1"]
pub type DT1R_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DT1R` writer - Rise Value for Dead Time of Channel 1"]
pub type DT1R_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DC1R_SPEC, u8, u8, 8, O>;
#[doc = "Field `DT1F` reader - Fall Value for Dead Time of Channel 1"]
pub type DT1F_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DT1F` writer - Fall Value for Dead Time of Channel 1"]
pub type DT1F_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DC1R_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Rise Value for Dead Time of Channel 1"]
    #[inline(always)]
    pub fn dt1r(&self) -> DT1R_R {
        DT1R_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Fall Value for Dead Time of Channel 1"]
    #[inline(always)]
    pub fn dt1f(&self) -> DT1F_R {
        DT1F_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Rise Value for Dead Time of Channel 1"]
    #[inline(always)]
    #[must_use]
    pub fn dt1r(&mut self) -> DT1R_W<0> {
        DT1R_W::new(self)
    }
    #[doc = "Bits 8:15 - Fall Value for Dead Time of Channel 1"]
    #[inline(always)]
    #[must_use]
    pub fn dt1f(&mut self) -> DT1F_W<8> {
        DT1F_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Channel 1 Dead Time Values\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dc1r](index.html) module"]
pub struct DC1R_SPEC;
impl crate::RegisterSpec for DC1R_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dc1r::R](R) reader structure"]
impl crate::Readable for DC1R_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dc1r::W](W) writer structure"]
impl crate::Writable for DC1R_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DC1R to value 0"]
impl crate::Resettable for DC1R_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
