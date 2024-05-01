#[doc = "Register `TCCLR` writer"]
pub type W = crate::W<TcclrSpec>;
#[doc = "Field `TRBC` writer - Timer Run Bit Clear"]
pub type TrbcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TCC` writer - Timer Clear"]
pub type TccW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DITC` writer - Dither Counter Clear"]
pub type DitcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DTC1C` writer - Dead Time Counter 1 Clear"]
pub type Dtc1cW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DTC2C` writer - Dead Time Counter 2 Clear"]
pub type Dtc2cW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Timer Run Bit Clear"]
    #[inline(always)]
    #[must_use]
    pub fn trbc(&mut self) -> TrbcW<TcclrSpec> {
        TrbcW::new(self, 0)
    }
    #[doc = "Bit 1 - Timer Clear"]
    #[inline(always)]
    #[must_use]
    pub fn tcc(&mut self) -> TccW<TcclrSpec> {
        TccW::new(self, 1)
    }
    #[doc = "Bit 2 - Dither Counter Clear"]
    #[inline(always)]
    #[must_use]
    pub fn ditc(&mut self) -> DitcW<TcclrSpec> {
        DitcW::new(self, 2)
    }
    #[doc = "Bit 3 - Dead Time Counter 1 Clear"]
    #[inline(always)]
    #[must_use]
    pub fn dtc1c(&mut self) -> Dtc1cW<TcclrSpec> {
        Dtc1cW::new(self, 3)
    }
    #[doc = "Bit 4 - Dead Time Counter 2 Clear"]
    #[inline(always)]
    #[must_use]
    pub fn dtc2c(&mut self) -> Dtc2cW<TcclrSpec> {
        Dtc2cW::new(self, 4)
    }
}
#[doc = "Slice Timer Clear\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tcclr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TcclrSpec;
impl crate::RegisterSpec for TcclrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`tcclr::W`](W) writer structure"]
impl crate::Writable for TcclrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TCCLR to value 0"]
impl crate::Resettable for TcclrSpec {
    const RESET_VALUE: u32 = 0;
}
