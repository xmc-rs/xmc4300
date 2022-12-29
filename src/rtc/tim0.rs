#[doc = "Register `TIM0` reader"]
pub struct R(crate::R<TIM0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIM0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIM0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIM0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TIM0` writer"]
pub struct W(crate::W<TIM0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIM0_SPEC>;
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
impl From<crate::W<TIM0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIM0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SE` reader - Seconds Time Value"]
pub type SE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SE` writer - Seconds Time Value"]
pub type SE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TIM0_SPEC, u8, u8, 6, O>;
#[doc = "Field `MI` reader - Minutes Time Value"]
pub type MI_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MI` writer - Minutes Time Value"]
pub type MI_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TIM0_SPEC, u8, u8, 6, O>;
#[doc = "Field `HO` reader - Hours Time Value"]
pub type HO_R = crate::FieldReader<u8, u8>;
#[doc = "Field `HO` writer - Hours Time Value"]
pub type HO_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TIM0_SPEC, u8, u8, 5, O>;
#[doc = "Field `DA` reader - Days Time Value"]
pub type DA_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DA` writer - Days Time Value"]
pub type DA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TIM0_SPEC, u8, u8, 5, O>;
impl R {
    #[doc = "Bits 0:5 - Seconds Time Value"]
    #[inline(always)]
    pub fn se(&self) -> SE_R {
        SE_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:13 - Minutes Time Value"]
    #[inline(always)]
    pub fn mi(&self) -> MI_R {
        MI_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:20 - Hours Time Value"]
    #[inline(always)]
    pub fn ho(&self) -> HO_R {
        HO_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28 - Days Time Value"]
    #[inline(always)]
    pub fn da(&self) -> DA_R {
        DA_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Seconds Time Value"]
    #[inline(always)]
    #[must_use]
    pub fn se(&mut self) -> SE_W<0> {
        SE_W::new(self)
    }
    #[doc = "Bits 8:13 - Minutes Time Value"]
    #[inline(always)]
    #[must_use]
    pub fn mi(&mut self) -> MI_W<8> {
        MI_W::new(self)
    }
    #[doc = "Bits 16:20 - Hours Time Value"]
    #[inline(always)]
    #[must_use]
    pub fn ho(&mut self) -> HO_W<16> {
        HO_W::new(self)
    }
    #[doc = "Bits 24:28 - Days Time Value"]
    #[inline(always)]
    #[must_use]
    pub fn da(&mut self) -> DA_W<24> {
        DA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RTC Time Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim0](index.html) module"]
pub struct TIM0_SPEC;
impl crate::RegisterSpec for TIM0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tim0::R](R) reader structure"]
impl crate::Readable for TIM0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tim0::W](W) writer structure"]
impl crate::Writable for TIM0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TIM0 to value 0"]
impl crate::Resettable for TIM0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
