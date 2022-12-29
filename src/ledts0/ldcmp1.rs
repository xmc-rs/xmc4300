#[doc = "Register `LDCMP1` reader"]
pub struct R(crate::R<LDCMP1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LDCMP1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LDCMP1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LDCMP1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LDCMP1` writer"]
pub struct W(crate::W<LDCMP1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LDCMP1_SPEC>;
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
impl From<crate::W<LDCMP1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LDCMP1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CMP_LD4` reader - Compare Value for LED COL\\[x\\]"]
pub type CMP_LD4_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CMP_LD4` writer - Compare Value for LED COL\\[x\\]"]
pub type CMP_LD4_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LDCMP1_SPEC, u8, u8, 8, O>;
#[doc = "Field `CMP_LD5` reader - Compare Value for LED COL\\[x\\]"]
pub type CMP_LD5_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CMP_LD5` writer - Compare Value for LED COL\\[x\\]"]
pub type CMP_LD5_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LDCMP1_SPEC, u8, u8, 8, O>;
#[doc = "Field `CMP_LD6` reader - Compare Value for LED COL\\[x\\]"]
pub type CMP_LD6_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CMP_LD6` writer - Compare Value for LED COL\\[x\\]"]
pub type CMP_LD6_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LDCMP1_SPEC, u8, u8, 8, O>;
#[doc = "Field `CMP_LDA_TSCOM` reader - Compare Value for LED COLA / Common Compare Value for Touch-sense Pad Turns"]
pub type CMP_LDA_TSCOM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CMP_LDA_TSCOM` writer - Compare Value for LED COLA / Common Compare Value for Touch-sense Pad Turns"]
pub type CMP_LDA_TSCOM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LDCMP1_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Compare Value for LED COL\\[x\\]"]
    #[inline(always)]
    pub fn cmp_ld4(&self) -> CMP_LD4_R {
        CMP_LD4_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Compare Value for LED COL\\[x\\]"]
    #[inline(always)]
    pub fn cmp_ld5(&self) -> CMP_LD5_R {
        CMP_LD5_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Compare Value for LED COL\\[x\\]"]
    #[inline(always)]
    pub fn cmp_ld6(&self) -> CMP_LD6_R {
        CMP_LD6_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Compare Value for LED COLA / Common Compare Value for Touch-sense Pad Turns"]
    #[inline(always)]
    pub fn cmp_lda_tscom(&self) -> CMP_LDA_TSCOM_R {
        CMP_LDA_TSCOM_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Compare Value for LED COL\\[x\\]"]
    #[inline(always)]
    #[must_use]
    pub fn cmp_ld4(&mut self) -> CMP_LD4_W<0> {
        CMP_LD4_W::new(self)
    }
    #[doc = "Bits 8:15 - Compare Value for LED COL\\[x\\]"]
    #[inline(always)]
    #[must_use]
    pub fn cmp_ld5(&mut self) -> CMP_LD5_W<8> {
        CMP_LD5_W::new(self)
    }
    #[doc = "Bits 16:23 - Compare Value for LED COL\\[x\\]"]
    #[inline(always)]
    #[must_use]
    pub fn cmp_ld6(&mut self) -> CMP_LD6_W<16> {
        CMP_LD6_W::new(self)
    }
    #[doc = "Bits 24:31 - Compare Value for LED COLA / Common Compare Value for Touch-sense Pad Turns"]
    #[inline(always)]
    #[must_use]
    pub fn cmp_lda_tscom(&mut self) -> CMP_LDA_TSCOM_W<24> {
        CMP_LDA_TSCOM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LED Compare Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ldcmp1](index.html) module"]
pub struct LDCMP1_SPEC;
impl crate::RegisterSpec for LDCMP1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ldcmp1::R](R) reader structure"]
impl crate::Readable for LDCMP1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ldcmp1::W](W) writer structure"]
impl crate::Writable for LDCMP1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LDCMP1 to value 0"]
impl crate::Resettable for LDCMP1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
