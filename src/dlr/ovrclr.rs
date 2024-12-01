#[doc = "Register `OVRCLR` writer"]
pub type W = crate::W<OVRCLR_SPEC>;
#[doc = "Field `LN0` writer - Line 0 Overrun Status Clear"]
pub type LN0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LN1` writer - Line 1 Overrun Status Clear"]
pub type LN1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LN2` writer - Line 2 Overrun Status Clear"]
pub type LN2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LN3` writer - Line 3 Overrun Status Clear"]
pub type LN3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LN4` writer - Line 4 Overrun Status Clear"]
pub type LN4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LN5` writer - Line 5 Overrun Status Clear"]
pub type LN5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LN6` writer - Line 6 Overrun Status Clear"]
pub type LN6_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LN7` writer - Line 7 Overrun Status Clear"]
pub type LN7_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Line 0 Overrun Status Clear"]
    #[inline(always)]
    pub fn ln0(&mut self) -> LN0_W<OVRCLR_SPEC> {
        LN0_W::new(self, 0)
    }
    #[doc = "Bit 1 - Line 1 Overrun Status Clear"]
    #[inline(always)]
    pub fn ln1(&mut self) -> LN1_W<OVRCLR_SPEC> {
        LN1_W::new(self, 1)
    }
    #[doc = "Bit 2 - Line 2 Overrun Status Clear"]
    #[inline(always)]
    pub fn ln2(&mut self) -> LN2_W<OVRCLR_SPEC> {
        LN2_W::new(self, 2)
    }
    #[doc = "Bit 3 - Line 3 Overrun Status Clear"]
    #[inline(always)]
    pub fn ln3(&mut self) -> LN3_W<OVRCLR_SPEC> {
        LN3_W::new(self, 3)
    }
    #[doc = "Bit 4 - Line 4 Overrun Status Clear"]
    #[inline(always)]
    pub fn ln4(&mut self) -> LN4_W<OVRCLR_SPEC> {
        LN4_W::new(self, 4)
    }
    #[doc = "Bit 5 - Line 5 Overrun Status Clear"]
    #[inline(always)]
    pub fn ln5(&mut self) -> LN5_W<OVRCLR_SPEC> {
        LN5_W::new(self, 5)
    }
    #[doc = "Bit 6 - Line 6 Overrun Status Clear"]
    #[inline(always)]
    pub fn ln6(&mut self) -> LN6_W<OVRCLR_SPEC> {
        LN6_W::new(self, 6)
    }
    #[doc = "Bit 7 - Line 7 Overrun Status Clear"]
    #[inline(always)]
    pub fn ln7(&mut self) -> LN7_W<OVRCLR_SPEC> {
        LN7_W::new(self, 7)
    }
}
#[doc = "Overrun Clear\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ovrclr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OVRCLR_SPEC;
impl crate::RegisterSpec for OVRCLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`ovrclr::W`](W) writer structure"]
impl crate::Writable for OVRCLR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OVRCLR to value 0"]
impl crate::Resettable for OVRCLR_SPEC {
    const RESET_VALUE: u32 = 0;
}
