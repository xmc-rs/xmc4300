#[doc = "Register `PLLCON1` reader"]
pub struct R(crate::R<PLLCON1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PLLCON1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PLLCON1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PLLCON1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PLLCON1` writer"]
pub struct W(crate::W<PLLCON1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PLLCON1_SPEC>;
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
impl From<crate::W<PLLCON1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PLLCON1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `K1DIV` reader - K1-Divider Value"]
pub type K1DIV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `K1DIV` writer - K1-Divider Value"]
pub type K1DIV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PLLCON1_SPEC, u8, u8, 7, O>;
#[doc = "Field `NDIV` reader - N-Divider Value"]
pub type NDIV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `NDIV` writer - N-Divider Value"]
pub type NDIV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PLLCON1_SPEC, u8, u8, 7, O>;
#[doc = "Field `K2DIV` reader - K2-Divider Value"]
pub type K2DIV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `K2DIV` writer - K2-Divider Value"]
pub type K2DIV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PLLCON1_SPEC, u8, u8, 7, O>;
#[doc = "Field `PDIV` reader - P-Divider Value"]
pub type PDIV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PDIV` writer - P-Divider Value"]
pub type PDIV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PLLCON1_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:6 - K1-Divider Value"]
    #[inline(always)]
    pub fn k1div(&self) -> K1DIV_R {
        K1DIV_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - N-Divider Value"]
    #[inline(always)]
    pub fn ndiv(&self) -> NDIV_R {
        NDIV_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 16:22 - K2-Divider Value"]
    #[inline(always)]
    pub fn k2div(&self) -> K2DIV_R {
        K2DIV_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 24:27 - P-Divider Value"]
    #[inline(always)]
    pub fn pdiv(&self) -> PDIV_R {
        PDIV_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - K1-Divider Value"]
    #[inline(always)]
    #[must_use]
    pub fn k1div(&mut self) -> K1DIV_W<0> {
        K1DIV_W::new(self)
    }
    #[doc = "Bits 8:14 - N-Divider Value"]
    #[inline(always)]
    #[must_use]
    pub fn ndiv(&mut self) -> NDIV_W<8> {
        NDIV_W::new(self)
    }
    #[doc = "Bits 16:22 - K2-Divider Value"]
    #[inline(always)]
    #[must_use]
    pub fn k2div(&mut self) -> K2DIV_W<16> {
        K2DIV_W::new(self)
    }
    #[doc = "Bits 24:27 - P-Divider Value"]
    #[inline(always)]
    #[must_use]
    pub fn pdiv(&mut self) -> PDIV_W<24> {
        PDIV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PLL Configuration 1 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pllcon1](index.html) module"]
pub struct PLLCON1_SPEC;
impl crate::RegisterSpec for PLLCON1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pllcon1::R](R) reader structure"]
impl crate::Readable for PLLCON1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pllcon1::W](W) writer structure"]
impl crate::Writable for PLLCON1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PLLCON1 to value 0"]
impl crate::Resettable for PLLCON1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
