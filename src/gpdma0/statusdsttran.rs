#[doc = "Register `STATUSDSTTRAN` reader"]
pub type R = crate::R<StatusdsttranSpec>;
#[doc = "Field `CH0` reader - Interrupt Status for channel 0"]
pub type Ch0R = crate::BitReader;
#[doc = "Field `CH1` reader - Interrupt Status for channel 1"]
pub type Ch1R = crate::BitReader;
#[doc = "Field `CH2` reader - Interrupt Status for channel 2"]
pub type Ch2R = crate::BitReader;
#[doc = "Field `CH3` reader - Interrupt Status for channel 3"]
pub type Ch3R = crate::BitReader;
#[doc = "Field `CH4` reader - Interrupt Status for channel 4"]
pub type Ch4R = crate::BitReader;
#[doc = "Field `CH5` reader - Interrupt Status for channel 5"]
pub type Ch5R = crate::BitReader;
#[doc = "Field `CH6` reader - Interrupt Status for channel 6"]
pub type Ch6R = crate::BitReader;
#[doc = "Field `CH7` reader - Interrupt Status for channel 7"]
pub type Ch7R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Interrupt Status for channel 0"]
    #[inline(always)]
    pub fn ch0(&self) -> Ch0R {
        Ch0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Interrupt Status for channel 1"]
    #[inline(always)]
    pub fn ch1(&self) -> Ch1R {
        Ch1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Interrupt Status for channel 2"]
    #[inline(always)]
    pub fn ch2(&self) -> Ch2R {
        Ch2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Interrupt Status for channel 3"]
    #[inline(always)]
    pub fn ch3(&self) -> Ch3R {
        Ch3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Interrupt Status for channel 4"]
    #[inline(always)]
    pub fn ch4(&self) -> Ch4R {
        Ch4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Interrupt Status for channel 5"]
    #[inline(always)]
    pub fn ch5(&self) -> Ch5R {
        Ch5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Interrupt Status for channel 6"]
    #[inline(always)]
    pub fn ch6(&self) -> Ch6R {
        Ch6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Interrupt Status for channel 7"]
    #[inline(always)]
    pub fn ch7(&self) -> Ch7R {
        Ch7R::new(((self.bits >> 7) & 1) != 0)
    }
}
#[doc = "IntBlock Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`statusdsttran::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StatusdsttranSpec;
impl crate::RegisterSpec for StatusdsttranSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`statusdsttran::R`](R) reader structure"]
impl crate::Readable for StatusdsttranSpec {}
#[doc = "`reset()` method sets STATUSDSTTRAN to value 0"]
impl crate::Resettable for StatusdsttranSpec {
    const RESET_VALUE: u32 = 0;
}
