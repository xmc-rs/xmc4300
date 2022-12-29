#[doc = "Register `TSVAL` reader"]
pub struct R(crate::R<TSVAL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TSVAL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TSVAL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TSVAL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TSVAL` writer"]
pub struct W(crate::W<TSVAL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TSVAL_SPEC>;
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
impl From<crate::W<TSVAL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TSVAL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TSCTRVALR` reader - Shadow TS-Counter (Read)"]
pub type TSCTRVALR_R = crate::FieldReader<u16, u16>;
#[doc = "Field `TSCTRVAL` reader - TS-Counter Value"]
pub type TSCTRVAL_R = crate::FieldReader<u16, u16>;
#[doc = "Field `TSCTRVAL` writer - TS-Counter Value"]
pub type TSCTRVAL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TSVAL_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - Shadow TS-Counter (Read)"]
    #[inline(always)]
    pub fn tsctrvalr(&self) -> TSCTRVALR_R {
        TSCTRVALR_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - TS-Counter Value"]
    #[inline(always)]
    pub fn tsctrval(&self) -> TSCTRVAL_R {
        TSCTRVAL_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:31 - TS-Counter Value"]
    #[inline(always)]
    #[must_use]
    pub fn tsctrval(&mut self) -> TSCTRVAL_W<16> {
        TSCTRVAL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Touch-sense TS-Counter Value\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tsval](index.html) module"]
pub struct TSVAL_SPEC;
impl crate::RegisterSpec for TSVAL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tsval::R](R) reader structure"]
impl crate::Readable for TSVAL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tsval::W](W) writer structure"]
impl crate::Writable for TSVAL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TSVAL to value 0"]
impl crate::Resettable for TSVAL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
