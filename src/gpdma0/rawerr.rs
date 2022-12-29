#[doc = "Register `RAWERR` reader"]
pub struct R(crate::R<RAWERR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RAWERR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RAWERR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RAWERR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RAWERR` writer"]
pub struct W(crate::W<RAWERR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RAWERR_SPEC>;
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
impl From<crate::W<RAWERR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RAWERR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CH0` reader - Raw Interrupt Status for channel 0"]
pub type CH0_R = crate::BitReader<bool>;
#[doc = "Field `CH0` writer - Raw Interrupt Status for channel 0"]
pub type CH0_W<'a, const O: u8> = crate::BitWriter<'a, u32, RAWERR_SPEC, bool, O>;
#[doc = "Field `CH1` reader - Raw Interrupt Status for channel 1"]
pub type CH1_R = crate::BitReader<bool>;
#[doc = "Field `CH1` writer - Raw Interrupt Status for channel 1"]
pub type CH1_W<'a, const O: u8> = crate::BitWriter<'a, u32, RAWERR_SPEC, bool, O>;
#[doc = "Field `CH2` reader - Raw Interrupt Status for channel 2"]
pub type CH2_R = crate::BitReader<bool>;
#[doc = "Field `CH2` writer - Raw Interrupt Status for channel 2"]
pub type CH2_W<'a, const O: u8> = crate::BitWriter<'a, u32, RAWERR_SPEC, bool, O>;
#[doc = "Field `CH3` reader - Raw Interrupt Status for channel 3"]
pub type CH3_R = crate::BitReader<bool>;
#[doc = "Field `CH3` writer - Raw Interrupt Status for channel 3"]
pub type CH3_W<'a, const O: u8> = crate::BitWriter<'a, u32, RAWERR_SPEC, bool, O>;
#[doc = "Field `CH4` reader - Raw Interrupt Status for channel 4"]
pub type CH4_R = crate::BitReader<bool>;
#[doc = "Field `CH4` writer - Raw Interrupt Status for channel 4"]
pub type CH4_W<'a, const O: u8> = crate::BitWriter<'a, u32, RAWERR_SPEC, bool, O>;
#[doc = "Field `CH5` reader - Raw Interrupt Status for channel 5"]
pub type CH5_R = crate::BitReader<bool>;
#[doc = "Field `CH5` writer - Raw Interrupt Status for channel 5"]
pub type CH5_W<'a, const O: u8> = crate::BitWriter<'a, u32, RAWERR_SPEC, bool, O>;
#[doc = "Field `CH6` reader - Raw Interrupt Status for channel 6"]
pub type CH6_R = crate::BitReader<bool>;
#[doc = "Field `CH6` writer - Raw Interrupt Status for channel 6"]
pub type CH6_W<'a, const O: u8> = crate::BitWriter<'a, u32, RAWERR_SPEC, bool, O>;
#[doc = "Field `CH7` reader - Raw Interrupt Status for channel 7"]
pub type CH7_R = crate::BitReader<bool>;
#[doc = "Field `CH7` writer - Raw Interrupt Status for channel 7"]
pub type CH7_W<'a, const O: u8> = crate::BitWriter<'a, u32, RAWERR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Raw Interrupt Status for channel 0"]
    #[inline(always)]
    pub fn ch0(&self) -> CH0_R {
        CH0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Raw Interrupt Status for channel 1"]
    #[inline(always)]
    pub fn ch1(&self) -> CH1_R {
        CH1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Raw Interrupt Status for channel 2"]
    #[inline(always)]
    pub fn ch2(&self) -> CH2_R {
        CH2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Raw Interrupt Status for channel 3"]
    #[inline(always)]
    pub fn ch3(&self) -> CH3_R {
        CH3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Raw Interrupt Status for channel 4"]
    #[inline(always)]
    pub fn ch4(&self) -> CH4_R {
        CH4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Raw Interrupt Status for channel 5"]
    #[inline(always)]
    pub fn ch5(&self) -> CH5_R {
        CH5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Raw Interrupt Status for channel 6"]
    #[inline(always)]
    pub fn ch6(&self) -> CH6_R {
        CH6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Raw Interrupt Status for channel 7"]
    #[inline(always)]
    pub fn ch7(&self) -> CH7_R {
        CH7_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Raw Interrupt Status for channel 0"]
    #[inline(always)]
    #[must_use]
    pub fn ch0(&mut self) -> CH0_W<0> {
        CH0_W::new(self)
    }
    #[doc = "Bit 1 - Raw Interrupt Status for channel 1"]
    #[inline(always)]
    #[must_use]
    pub fn ch1(&mut self) -> CH1_W<1> {
        CH1_W::new(self)
    }
    #[doc = "Bit 2 - Raw Interrupt Status for channel 2"]
    #[inline(always)]
    #[must_use]
    pub fn ch2(&mut self) -> CH2_W<2> {
        CH2_W::new(self)
    }
    #[doc = "Bit 3 - Raw Interrupt Status for channel 3"]
    #[inline(always)]
    #[must_use]
    pub fn ch3(&mut self) -> CH3_W<3> {
        CH3_W::new(self)
    }
    #[doc = "Bit 4 - Raw Interrupt Status for channel 4"]
    #[inline(always)]
    #[must_use]
    pub fn ch4(&mut self) -> CH4_W<4> {
        CH4_W::new(self)
    }
    #[doc = "Bit 5 - Raw Interrupt Status for channel 5"]
    #[inline(always)]
    #[must_use]
    pub fn ch5(&mut self) -> CH5_W<5> {
        CH5_W::new(self)
    }
    #[doc = "Bit 6 - Raw Interrupt Status for channel 6"]
    #[inline(always)]
    #[must_use]
    pub fn ch6(&mut self) -> CH6_W<6> {
        CH6_W::new(self)
    }
    #[doc = "Bit 7 - Raw Interrupt Status for channel 7"]
    #[inline(always)]
    #[must_use]
    pub fn ch7(&mut self) -> CH7_W<7> {
        CH7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Raw IntErr Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rawerr](index.html) module"]
pub struct RAWERR_SPEC;
impl crate::RegisterSpec for RAWERR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rawerr::R](R) reader structure"]
impl crate::Readable for RAWERR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rawerr::W](W) writer structure"]
impl crate::Writable for RAWERR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RAWERR to value 0"]
impl crate::Resettable for RAWERR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
