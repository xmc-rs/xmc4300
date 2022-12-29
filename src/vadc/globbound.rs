#[doc = "Register `GLOBBOUND` reader"]
pub struct R(crate::R<GLOBBOUND_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GLOBBOUND_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GLOBBOUND_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GLOBBOUND_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GLOBBOUND` writer"]
pub struct W(crate::W<GLOBBOUND_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GLOBBOUND_SPEC>;
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
impl From<crate::W<GLOBBOUND_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GLOBBOUND_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BOUNDARY0` reader - Boundary Value 0 for Limit Checking"]
pub type BOUNDARY0_R = crate::FieldReader<u16, u16>;
#[doc = "Field `BOUNDARY0` writer - Boundary Value 0 for Limit Checking"]
pub type BOUNDARY0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GLOBBOUND_SPEC, u16, u16, 12, O>;
#[doc = "Field `BOUNDARY1` reader - Boundary Value 1 for Limit Checking"]
pub type BOUNDARY1_R = crate::FieldReader<u16, u16>;
#[doc = "Field `BOUNDARY1` writer - Boundary Value 1 for Limit Checking"]
pub type BOUNDARY1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GLOBBOUND_SPEC, u16, u16, 12, O>;
impl R {
    #[doc = "Bits 0:11 - Boundary Value 0 for Limit Checking"]
    #[inline(always)]
    pub fn boundary0(&self) -> BOUNDARY0_R {
        BOUNDARY0_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - Boundary Value 1 for Limit Checking"]
    #[inline(always)]
    pub fn boundary1(&self) -> BOUNDARY1_R {
        BOUNDARY1_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Boundary Value 0 for Limit Checking"]
    #[inline(always)]
    #[must_use]
    pub fn boundary0(&mut self) -> BOUNDARY0_W<0> {
        BOUNDARY0_W::new(self)
    }
    #[doc = "Bits 16:27 - Boundary Value 1 for Limit Checking"]
    #[inline(always)]
    #[must_use]
    pub fn boundary1(&mut self) -> BOUNDARY1_W<16> {
        BOUNDARY1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Global Boundary Select Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [globbound](index.html) module"]
pub struct GLOBBOUND_SPEC;
impl crate::RegisterSpec for GLOBBOUND_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [globbound::R](R) reader structure"]
impl crate::Readable for GLOBBOUND_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [globbound::W](W) writer structure"]
impl crate::Writable for GLOBBOUND_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GLOBBOUND to value 0"]
impl crate::Resettable for GLOBBOUND_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
