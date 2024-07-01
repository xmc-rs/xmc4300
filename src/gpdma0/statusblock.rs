#[doc = "Register `STATUSBLOCK` reader"]
pub type R = crate::R<STATUSBLOCK_SPEC>;
#[doc = "Field `CH0` reader - Interrupt Status for channel 0"]
pub type CH0_R = crate::BitReader;
#[doc = "Field `CH1` reader - Interrupt Status for channel 1"]
pub type CH1_R = crate::BitReader;
#[doc = "Field `CH2` reader - Interrupt Status for channel 2"]
pub type CH2_R = crate::BitReader;
#[doc = "Field `CH3` reader - Interrupt Status for channel 3"]
pub type CH3_R = crate::BitReader;
#[doc = "Field `CH4` reader - Interrupt Status for channel 4"]
pub type CH4_R = crate::BitReader;
#[doc = "Field `CH5` reader - Interrupt Status for channel 5"]
pub type CH5_R = crate::BitReader;
#[doc = "Field `CH6` reader - Interrupt Status for channel 6"]
pub type CH6_R = crate::BitReader;
#[doc = "Field `CH7` reader - Interrupt Status for channel 7"]
pub type CH7_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Interrupt Status for channel 0"]
    #[inline(always)]
    pub fn ch0(&self) -> CH0_R {
        CH0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Interrupt Status for channel 1"]
    #[inline(always)]
    pub fn ch1(&self) -> CH1_R {
        CH1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Interrupt Status for channel 2"]
    #[inline(always)]
    pub fn ch2(&self) -> CH2_R {
        CH2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Interrupt Status for channel 3"]
    #[inline(always)]
    pub fn ch3(&self) -> CH3_R {
        CH3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Interrupt Status for channel 4"]
    #[inline(always)]
    pub fn ch4(&self) -> CH4_R {
        CH4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Interrupt Status for channel 5"]
    #[inline(always)]
    pub fn ch5(&self) -> CH5_R {
        CH5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Interrupt Status for channel 6"]
    #[inline(always)]
    pub fn ch6(&self) -> CH6_R {
        CH6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Interrupt Status for channel 7"]
    #[inline(always)]
    pub fn ch7(&self) -> CH7_R {
        CH7_R::new(((self.bits >> 7) & 1) != 0)
    }
}
#[doc = "IntBlock Status\n\nYou can [`read`](crate::Reg::read) this register and get [`statusblock::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STATUSBLOCK_SPEC;
impl crate::RegisterSpec for STATUSBLOCK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`statusblock::R`](R) reader structure"]
impl crate::Readable for STATUSBLOCK_SPEC {}
#[doc = "`reset()` method sets STATUSBLOCK to value 0"]
impl crate::Resettable for STATUSBLOCK_SPEC {
    const RESET_VALUE: u32 = 0;
}
