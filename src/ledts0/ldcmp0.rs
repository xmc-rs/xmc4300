#[doc = "Register `LDCMP0` reader"]
pub struct R(crate::R<LDCMP0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LDCMP0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LDCMP0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LDCMP0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LDCMP0` writer"]
pub struct W(crate::W<LDCMP0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LDCMP0_SPEC>;
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
impl From<crate::W<LDCMP0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LDCMP0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CMP_LD0` reader - Compare Value for LED COL\\[x\\]"]
pub type CMP_LD0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CMP_LD0` writer - Compare Value for LED COL\\[x\\]"]
pub type CMP_LD0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LDCMP0_SPEC, u8, u8, 8, O>;
#[doc = "Field `CMP_LD1` reader - Compare Value for LED COL\\[x\\]"]
pub type CMP_LD1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CMP_LD1` writer - Compare Value for LED COL\\[x\\]"]
pub type CMP_LD1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LDCMP0_SPEC, u8, u8, 8, O>;
#[doc = "Field `CMP_LD2` reader - Compare Value for LED COL\\[x\\]"]
pub type CMP_LD2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CMP_LD2` writer - Compare Value for LED COL\\[x\\]"]
pub type CMP_LD2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LDCMP0_SPEC, u8, u8, 8, O>;
#[doc = "Field `CMP_LD3` reader - Compare Value for LED COL\\[x\\]"]
pub type CMP_LD3_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CMP_LD3` writer - Compare Value for LED COL\\[x\\]"]
pub type CMP_LD3_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LDCMP0_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Compare Value for LED COL\\[x\\]"]
    #[inline(always)]
    pub fn cmp_ld0(&self) -> CMP_LD0_R {
        CMP_LD0_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Compare Value for LED COL\\[x\\]"]
    #[inline(always)]
    pub fn cmp_ld1(&self) -> CMP_LD1_R {
        CMP_LD1_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Compare Value for LED COL\\[x\\]"]
    #[inline(always)]
    pub fn cmp_ld2(&self) -> CMP_LD2_R {
        CMP_LD2_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Compare Value for LED COL\\[x\\]"]
    #[inline(always)]
    pub fn cmp_ld3(&self) -> CMP_LD3_R {
        CMP_LD3_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Compare Value for LED COL\\[x\\]"]
    #[inline(always)]
    #[must_use]
    pub fn cmp_ld0(&mut self) -> CMP_LD0_W<0> {
        CMP_LD0_W::new(self)
    }
    #[doc = "Bits 8:15 - Compare Value for LED COL\\[x\\]"]
    #[inline(always)]
    #[must_use]
    pub fn cmp_ld1(&mut self) -> CMP_LD1_W<8> {
        CMP_LD1_W::new(self)
    }
    #[doc = "Bits 16:23 - Compare Value for LED COL\\[x\\]"]
    #[inline(always)]
    #[must_use]
    pub fn cmp_ld2(&mut self) -> CMP_LD2_W<16> {
        CMP_LD2_W::new(self)
    }
    #[doc = "Bits 24:31 - Compare Value for LED COL\\[x\\]"]
    #[inline(always)]
    #[must_use]
    pub fn cmp_ld3(&mut self) -> CMP_LD3_W<24> {
        CMP_LD3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LED Compare Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ldcmp0](index.html) module"]
pub struct LDCMP0_SPEC;
impl crate::RegisterSpec for LDCMP0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ldcmp0::R](R) reader structure"]
impl crate::Readable for LDCMP0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ldcmp0::W](W) writer structure"]
impl crate::Writable for LDCMP0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LDCMP0 to value 0"]
impl crate::Resettable for LDCMP0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
