#[doc = "Register `RAWTFR` reader"]
pub type R = crate::R<RAWTFR_SPEC>;
#[doc = "Register `RAWTFR` writer"]
pub type W = crate::W<RAWTFR_SPEC>;
#[doc = "Field `CH0` reader - Raw Interrupt Status for channel 0"]
pub type CH0_R = crate::BitReader;
#[doc = "Field `CH0` writer - Raw Interrupt Status for channel 0"]
pub type CH0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1` reader - Raw Interrupt Status for channel 1"]
pub type CH1_R = crate::BitReader;
#[doc = "Field `CH1` writer - Raw Interrupt Status for channel 1"]
pub type CH1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH2` reader - Raw Interrupt Status for channel 2"]
pub type CH2_R = crate::BitReader;
#[doc = "Field `CH2` writer - Raw Interrupt Status for channel 2"]
pub type CH2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH3` reader - Raw Interrupt Status for channel 3"]
pub type CH3_R = crate::BitReader;
#[doc = "Field `CH3` writer - Raw Interrupt Status for channel 3"]
pub type CH3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH4` reader - Raw Interrupt Status for channel 4"]
pub type CH4_R = crate::BitReader;
#[doc = "Field `CH4` writer - Raw Interrupt Status for channel 4"]
pub type CH4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH5` reader - Raw Interrupt Status for channel 5"]
pub type CH5_R = crate::BitReader;
#[doc = "Field `CH5` writer - Raw Interrupt Status for channel 5"]
pub type CH5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH6` reader - Raw Interrupt Status for channel 6"]
pub type CH6_R = crate::BitReader;
#[doc = "Field `CH6` writer - Raw Interrupt Status for channel 6"]
pub type CH6_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH7` reader - Raw Interrupt Status for channel 7"]
pub type CH7_R = crate::BitReader;
#[doc = "Field `CH7` writer - Raw Interrupt Status for channel 7"]
pub type CH7_W<'a, REG> = crate::BitWriter<'a, REG>;
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
    pub fn ch0(&mut self) -> CH0_W<RAWTFR_SPEC> {
        CH0_W::new(self, 0)
    }
    #[doc = "Bit 1 - Raw Interrupt Status for channel 1"]
    #[inline(always)]
    #[must_use]
    pub fn ch1(&mut self) -> CH1_W<RAWTFR_SPEC> {
        CH1_W::new(self, 1)
    }
    #[doc = "Bit 2 - Raw Interrupt Status for channel 2"]
    #[inline(always)]
    #[must_use]
    pub fn ch2(&mut self) -> CH2_W<RAWTFR_SPEC> {
        CH2_W::new(self, 2)
    }
    #[doc = "Bit 3 - Raw Interrupt Status for channel 3"]
    #[inline(always)]
    #[must_use]
    pub fn ch3(&mut self) -> CH3_W<RAWTFR_SPEC> {
        CH3_W::new(self, 3)
    }
    #[doc = "Bit 4 - Raw Interrupt Status for channel 4"]
    #[inline(always)]
    #[must_use]
    pub fn ch4(&mut self) -> CH4_W<RAWTFR_SPEC> {
        CH4_W::new(self, 4)
    }
    #[doc = "Bit 5 - Raw Interrupt Status for channel 5"]
    #[inline(always)]
    #[must_use]
    pub fn ch5(&mut self) -> CH5_W<RAWTFR_SPEC> {
        CH5_W::new(self, 5)
    }
    #[doc = "Bit 6 - Raw Interrupt Status for channel 6"]
    #[inline(always)]
    #[must_use]
    pub fn ch6(&mut self) -> CH6_W<RAWTFR_SPEC> {
        CH6_W::new(self, 6)
    }
    #[doc = "Bit 7 - Raw Interrupt Status for channel 7"]
    #[inline(always)]
    #[must_use]
    pub fn ch7(&mut self) -> CH7_W<RAWTFR_SPEC> {
        CH7_W::new(self, 7)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Raw IntTfr Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rawtfr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rawtfr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RAWTFR_SPEC;
impl crate::RegisterSpec for RAWTFR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rawtfr::R`](R) reader structure"]
impl crate::Readable for RAWTFR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rawtfr::W`](W) writer structure"]
impl crate::Writable for RAWTFR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RAWTFR to value 0"]
impl crate::Resettable for RAWTFR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
