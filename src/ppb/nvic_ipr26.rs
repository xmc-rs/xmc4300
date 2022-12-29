#[doc = "Register `NVIC_IPR26` reader"]
pub struct R(crate::R<NVIC_IPR26_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<NVIC_IPR26_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<NVIC_IPR26_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<NVIC_IPR26_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `NVIC_IPR26` writer"]
pub struct W(crate::W<NVIC_IPR26_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<NVIC_IPR26_SPEC>;
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
impl From<crate::W<NVIC_IPR26_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<NVIC_IPR26_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PRI_0` reader - Priority value 0"]
pub type PRI_0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PRI_0` writer - Priority value 0"]
pub type PRI_0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, NVIC_IPR26_SPEC, u8, u8, 8, O>;
#[doc = "Field `PRI_1` reader - Priority value 1"]
pub type PRI_1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PRI_1` writer - Priority value 1"]
pub type PRI_1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, NVIC_IPR26_SPEC, u8, u8, 8, O>;
#[doc = "Field `PRI_2` reader - Priority value 2"]
pub type PRI_2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PRI_2` writer - Priority value 2"]
pub type PRI_2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, NVIC_IPR26_SPEC, u8, u8, 8, O>;
#[doc = "Field `PRI_3` reader - Priority value 3"]
pub type PRI_3_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PRI_3` writer - Priority value 3"]
pub type PRI_3_W<'a, const O: u8> = crate::FieldWriter<'a, u32, NVIC_IPR26_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Priority value 0"]
    #[inline(always)]
    pub fn pri_0(&self) -> PRI_0_R {
        PRI_0_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Priority value 1"]
    #[inline(always)]
    pub fn pri_1(&self) -> PRI_1_R {
        PRI_1_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Priority value 2"]
    #[inline(always)]
    pub fn pri_2(&self) -> PRI_2_R {
        PRI_2_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Priority value 3"]
    #[inline(always)]
    pub fn pri_3(&self) -> PRI_3_R {
        PRI_3_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Priority value 0"]
    #[inline(always)]
    #[must_use]
    pub fn pri_0(&mut self) -> PRI_0_W<0> {
        PRI_0_W::new(self)
    }
    #[doc = "Bits 8:15 - Priority value 1"]
    #[inline(always)]
    #[must_use]
    pub fn pri_1(&mut self) -> PRI_1_W<8> {
        PRI_1_W::new(self)
    }
    #[doc = "Bits 16:23 - Priority value 2"]
    #[inline(always)]
    #[must_use]
    pub fn pri_2(&mut self) -> PRI_2_W<16> {
        PRI_2_W::new(self)
    }
    #[doc = "Bits 24:31 - Priority value 3"]
    #[inline(always)]
    #[must_use]
    pub fn pri_3(&mut self) -> PRI_3_W<24> {
        PRI_3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Priority Register 26\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nvic_ipr26](index.html) module"]
pub struct NVIC_IPR26_SPEC;
impl crate::RegisterSpec for NVIC_IPR26_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [nvic_ipr26::R](R) reader structure"]
impl crate::Readable for NVIC_IPR26_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [nvic_ipr26::W](W) writer structure"]
impl crate::Writable for NVIC_IPR26_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets NVIC_IPR26 to value 0"]
impl crate::Resettable for NVIC_IPR26_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
