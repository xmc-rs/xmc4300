#[doc = "Register `RAWSRCTRAN` reader"]
pub type R = crate::R<RawsrctranSpec>;
#[doc = "Register `RAWSRCTRAN` writer"]
pub type W = crate::W<RawsrctranSpec>;
#[doc = "Field `CH0` reader - Raw Interrupt Status for channel 0"]
pub type Ch0R = crate::BitReader;
#[doc = "Field `CH0` writer - Raw Interrupt Status for channel 0"]
pub type Ch0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1` reader - Raw Interrupt Status for channel 1"]
pub type Ch1R = crate::BitReader;
#[doc = "Field `CH1` writer - Raw Interrupt Status for channel 1"]
pub type Ch1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH2` reader - Raw Interrupt Status for channel 2"]
pub type Ch2R = crate::BitReader;
#[doc = "Field `CH2` writer - Raw Interrupt Status for channel 2"]
pub type Ch2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH3` reader - Raw Interrupt Status for channel 3"]
pub type Ch3R = crate::BitReader;
#[doc = "Field `CH3` writer - Raw Interrupt Status for channel 3"]
pub type Ch3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH4` reader - Raw Interrupt Status for channel 4"]
pub type Ch4R = crate::BitReader;
#[doc = "Field `CH4` writer - Raw Interrupt Status for channel 4"]
pub type Ch4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH5` reader - Raw Interrupt Status for channel 5"]
pub type Ch5R = crate::BitReader;
#[doc = "Field `CH5` writer - Raw Interrupt Status for channel 5"]
pub type Ch5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH6` reader - Raw Interrupt Status for channel 6"]
pub type Ch6R = crate::BitReader;
#[doc = "Field `CH6` writer - Raw Interrupt Status for channel 6"]
pub type Ch6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH7` reader - Raw Interrupt Status for channel 7"]
pub type Ch7R = crate::BitReader;
#[doc = "Field `CH7` writer - Raw Interrupt Status for channel 7"]
pub type Ch7W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Raw Interrupt Status for channel 0"]
    #[inline(always)]
    pub fn ch0(&self) -> Ch0R {
        Ch0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Raw Interrupt Status for channel 1"]
    #[inline(always)]
    pub fn ch1(&self) -> Ch1R {
        Ch1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Raw Interrupt Status for channel 2"]
    #[inline(always)]
    pub fn ch2(&self) -> Ch2R {
        Ch2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Raw Interrupt Status for channel 3"]
    #[inline(always)]
    pub fn ch3(&self) -> Ch3R {
        Ch3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Raw Interrupt Status for channel 4"]
    #[inline(always)]
    pub fn ch4(&self) -> Ch4R {
        Ch4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Raw Interrupt Status for channel 5"]
    #[inline(always)]
    pub fn ch5(&self) -> Ch5R {
        Ch5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Raw Interrupt Status for channel 6"]
    #[inline(always)]
    pub fn ch6(&self) -> Ch6R {
        Ch6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Raw Interrupt Status for channel 7"]
    #[inline(always)]
    pub fn ch7(&self) -> Ch7R {
        Ch7R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Raw Interrupt Status for channel 0"]
    #[inline(always)]
    #[must_use]
    pub fn ch0(&mut self) -> Ch0W<RawsrctranSpec> {
        Ch0W::new(self, 0)
    }
    #[doc = "Bit 1 - Raw Interrupt Status for channel 1"]
    #[inline(always)]
    #[must_use]
    pub fn ch1(&mut self) -> Ch1W<RawsrctranSpec> {
        Ch1W::new(self, 1)
    }
    #[doc = "Bit 2 - Raw Interrupt Status for channel 2"]
    #[inline(always)]
    #[must_use]
    pub fn ch2(&mut self) -> Ch2W<RawsrctranSpec> {
        Ch2W::new(self, 2)
    }
    #[doc = "Bit 3 - Raw Interrupt Status for channel 3"]
    #[inline(always)]
    #[must_use]
    pub fn ch3(&mut self) -> Ch3W<RawsrctranSpec> {
        Ch3W::new(self, 3)
    }
    #[doc = "Bit 4 - Raw Interrupt Status for channel 4"]
    #[inline(always)]
    #[must_use]
    pub fn ch4(&mut self) -> Ch4W<RawsrctranSpec> {
        Ch4W::new(self, 4)
    }
    #[doc = "Bit 5 - Raw Interrupt Status for channel 5"]
    #[inline(always)]
    #[must_use]
    pub fn ch5(&mut self) -> Ch5W<RawsrctranSpec> {
        Ch5W::new(self, 5)
    }
    #[doc = "Bit 6 - Raw Interrupt Status for channel 6"]
    #[inline(always)]
    #[must_use]
    pub fn ch6(&mut self) -> Ch6W<RawsrctranSpec> {
        Ch6W::new(self, 6)
    }
    #[doc = "Bit 7 - Raw Interrupt Status for channel 7"]
    #[inline(always)]
    #[must_use]
    pub fn ch7(&mut self) -> Ch7W<RawsrctranSpec> {
        Ch7W::new(self, 7)
    }
}
#[doc = "Raw IntSrcTran Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rawsrctran::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rawsrctran::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RawsrctranSpec;
impl crate::RegisterSpec for RawsrctranSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rawsrctran::R`](R) reader structure"]
impl crate::Readable for RawsrctranSpec {}
#[doc = "`write(|w| ..)` method takes [`rawsrctran::W`](W) writer structure"]
impl crate::Writable for RawsrctranSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RAWSRCTRAN to value 0"]
impl crate::Resettable for RawsrctranSpec {
    const RESET_VALUE: u32 = 0;
}
