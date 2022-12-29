#[doc = "Register `PDR1` reader"]
pub struct R(crate::R<PDR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PDR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PDR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PDR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PDR1` writer"]
pub struct W(crate::W<PDR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PDR1_SPEC>;
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
impl From<crate::W<PDR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PDR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PD8` reader - Pad Driver Mode for Pn.8"]
pub type PD8_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PD8` writer - Pad Driver Mode for Pn.8"]
pub type PD8_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PDR1_SPEC, u8, u8, 3, O>;
#[doc = "Field `PD9` reader - Pad Driver Mode for Pn.9"]
pub type PD9_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PD9` writer - Pad Driver Mode for Pn.9"]
pub type PD9_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PDR1_SPEC, u8, u8, 3, O>;
#[doc = "Field `PD10` reader - Pad Driver Mode for Pn.10"]
pub type PD10_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PD10` writer - Pad Driver Mode for Pn.10"]
pub type PD10_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PDR1_SPEC, u8, u8, 3, O>;
#[doc = "Field `PD11` reader - Pad Driver Mode for Pn.11"]
pub type PD11_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PD11` writer - Pad Driver Mode for Pn.11"]
pub type PD11_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PDR1_SPEC, u8, u8, 3, O>;
#[doc = "Field `PD12` reader - Pad Driver Mode for Pn.12"]
pub type PD12_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PD12` writer - Pad Driver Mode for Pn.12"]
pub type PD12_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PDR1_SPEC, u8, u8, 3, O>;
#[doc = "Field `PD13` reader - Pad Driver Mode for Pn.13"]
pub type PD13_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PD13` writer - Pad Driver Mode for Pn.13"]
pub type PD13_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PDR1_SPEC, u8, u8, 3, O>;
#[doc = "Field `PD14` reader - Pad Driver Mode for Pn.14"]
pub type PD14_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PD14` writer - Pad Driver Mode for Pn.14"]
pub type PD14_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PDR1_SPEC, u8, u8, 3, O>;
#[doc = "Field `PD15` reader - Pad Driver Mode for Pn.15"]
pub type PD15_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PD15` writer - Pad Driver Mode for Pn.15"]
pub type PD15_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PDR1_SPEC, u8, u8, 3, O>;
impl R {
    #[doc = "Bits 0:2 - Pad Driver Mode for Pn.8"]
    #[inline(always)]
    pub fn pd8(&self) -> PD8_R {
        PD8_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:6 - Pad Driver Mode for Pn.9"]
    #[inline(always)]
    pub fn pd9(&self) -> PD9_R {
        PD9_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:10 - Pad Driver Mode for Pn.10"]
    #[inline(always)]
    pub fn pd10(&self) -> PD10_R {
        PD10_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 12:14 - Pad Driver Mode for Pn.11"]
    #[inline(always)]
    pub fn pd11(&self) -> PD11_R {
        PD11_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 16:18 - Pad Driver Mode for Pn.12"]
    #[inline(always)]
    pub fn pd12(&self) -> PD12_R {
        PD12_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 20:22 - Pad Driver Mode for Pn.13"]
    #[inline(always)]
    pub fn pd13(&self) -> PD13_R {
        PD13_R::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bits 24:26 - Pad Driver Mode for Pn.14"]
    #[inline(always)]
    pub fn pd14(&self) -> PD14_R {
        PD14_R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bits 28:30 - Pad Driver Mode for Pn.15"]
    #[inline(always)]
    pub fn pd15(&self) -> PD15_R {
        PD15_R::new(((self.bits >> 28) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Pad Driver Mode for Pn.8"]
    #[inline(always)]
    #[must_use]
    pub fn pd8(&mut self) -> PD8_W<0> {
        PD8_W::new(self)
    }
    #[doc = "Bits 4:6 - Pad Driver Mode for Pn.9"]
    #[inline(always)]
    #[must_use]
    pub fn pd9(&mut self) -> PD9_W<4> {
        PD9_W::new(self)
    }
    #[doc = "Bits 8:10 - Pad Driver Mode for Pn.10"]
    #[inline(always)]
    #[must_use]
    pub fn pd10(&mut self) -> PD10_W<8> {
        PD10_W::new(self)
    }
    #[doc = "Bits 12:14 - Pad Driver Mode for Pn.11"]
    #[inline(always)]
    #[must_use]
    pub fn pd11(&mut self) -> PD11_W<12> {
        PD11_W::new(self)
    }
    #[doc = "Bits 16:18 - Pad Driver Mode for Pn.12"]
    #[inline(always)]
    #[must_use]
    pub fn pd12(&mut self) -> PD12_W<16> {
        PD12_W::new(self)
    }
    #[doc = "Bits 20:22 - Pad Driver Mode for Pn.13"]
    #[inline(always)]
    #[must_use]
    pub fn pd13(&mut self) -> PD13_W<20> {
        PD13_W::new(self)
    }
    #[doc = "Bits 24:26 - Pad Driver Mode for Pn.14"]
    #[inline(always)]
    #[must_use]
    pub fn pd14(&mut self) -> PD14_W<24> {
        PD14_W::new(self)
    }
    #[doc = "Bits 28:30 - Pad Driver Mode for Pn.15"]
    #[inline(always)]
    #[must_use]
    pub fn pd15(&mut self) -> PD15_W<28> {
        PD15_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Port 1 Pad Driver Mode 1 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdr1](index.html) module"]
pub struct PDR1_SPEC;
impl crate::RegisterSpec for PDR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pdr1::R](R) reader structure"]
impl crate::Readable for PDR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pdr1::W](W) writer structure"]
impl crate::Writable for PDR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PDR1 to value 0x2222_2222"]
impl crate::Resettable for PDR1_SPEC {
    const RESET_VALUE: Self::Ux = 0x2222_2222;
}
