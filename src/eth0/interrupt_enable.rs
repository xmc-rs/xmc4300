#[doc = "Register `INTERRUPT_ENABLE` reader"]
pub type R = crate::R<InterruptEnableSpec>;
#[doc = "Register `INTERRUPT_ENABLE` writer"]
pub type W = crate::W<InterruptEnableSpec>;
#[doc = "Field `TIE` reader - Transmit Interrupt Enable"]
pub type TieR = crate::BitReader;
#[doc = "Field `TIE` writer - Transmit Interrupt Enable"]
pub type TieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSE` reader - Transmit Stopped Enable"]
pub type TseR = crate::BitReader;
#[doc = "Field `TSE` writer - Transmit Stopped Enable"]
pub type TseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TUE` reader - Transmit Buffer Unvailable Enable"]
pub type TueR = crate::BitReader;
#[doc = "Field `TUE` writer - Transmit Buffer Unvailable Enable"]
pub type TueW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TJE` reader - Transmit Jabber Timeout Enable"]
pub type TjeR = crate::BitReader;
#[doc = "Field `TJE` writer - Transmit Jabber Timeout Enable"]
pub type TjeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVE` reader - Overflow Interrupt Enable"]
pub type OveR = crate::BitReader;
#[doc = "Field `OVE` writer - Overflow Interrupt Enable"]
pub type OveW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UNE` reader - Underflow Interrupt Enable"]
pub type UneR = crate::BitReader;
#[doc = "Field `UNE` writer - Underflow Interrupt Enable"]
pub type UneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RIE` reader - Receive Interrupt Enable"]
pub type RieR = crate::BitReader;
#[doc = "Field `RIE` writer - Receive Interrupt Enable"]
pub type RieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RUE` reader - Receive Buffer Unavailable Enable"]
pub type RueR = crate::BitReader;
#[doc = "Field `RUE` writer - Receive Buffer Unavailable Enable"]
pub type RueW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSE` reader - Receive Stopped Enable"]
pub type RseR = crate::BitReader;
#[doc = "Field `RSE` writer - Receive Stopped Enable"]
pub type RseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RWE` reader - Receive Watchdog Timeout Enable"]
pub type RweR = crate::BitReader;
#[doc = "Field `RWE` writer - Receive Watchdog Timeout Enable"]
pub type RweW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ETE` reader - Early Transmit Interrupt Enable"]
pub type EteR = crate::BitReader;
#[doc = "Field `ETE` writer - Early Transmit Interrupt Enable"]
pub type EteW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FBE` reader - Fatal Bus Error Enable"]
pub type FbeR = crate::BitReader;
#[doc = "Field `FBE` writer - Fatal Bus Error Enable"]
pub type FbeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERE` reader - Early Receive Interrupt Enable"]
pub type EreR = crate::BitReader;
#[doc = "Field `ERE` writer - Early Receive Interrupt Enable"]
pub type EreW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AIE` reader - Abnormal Interrupt Summary Enable"]
pub type AieR = crate::BitReader;
#[doc = "Field `AIE` writer - Abnormal Interrupt Summary Enable"]
pub type AieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NIE` reader - Normal Interrupt Summary Enable"]
pub type NieR = crate::BitReader;
#[doc = "Field `NIE` writer - Normal Interrupt Summary Enable"]
pub type NieW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Transmit Interrupt Enable"]
    #[inline(always)]
    pub fn tie(&self) -> TieR {
        TieR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmit Stopped Enable"]
    #[inline(always)]
    pub fn tse(&self) -> TseR {
        TseR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Transmit Buffer Unvailable Enable"]
    #[inline(always)]
    pub fn tue(&self) -> TueR {
        TueR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Transmit Jabber Timeout Enable"]
    #[inline(always)]
    pub fn tje(&self) -> TjeR {
        TjeR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Overflow Interrupt Enable"]
    #[inline(always)]
    pub fn ove(&self) -> OveR {
        OveR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Underflow Interrupt Enable"]
    #[inline(always)]
    pub fn une(&self) -> UneR {
        UneR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Receive Interrupt Enable"]
    #[inline(always)]
    pub fn rie(&self) -> RieR {
        RieR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Receive Buffer Unavailable Enable"]
    #[inline(always)]
    pub fn rue(&self) -> RueR {
        RueR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Receive Stopped Enable"]
    #[inline(always)]
    pub fn rse(&self) -> RseR {
        RseR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Receive Watchdog Timeout Enable"]
    #[inline(always)]
    pub fn rwe(&self) -> RweR {
        RweR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Early Transmit Interrupt Enable"]
    #[inline(always)]
    pub fn ete(&self) -> EteR {
        EteR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 13 - Fatal Bus Error Enable"]
    #[inline(always)]
    pub fn fbe(&self) -> FbeR {
        FbeR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Early Receive Interrupt Enable"]
    #[inline(always)]
    pub fn ere(&self) -> EreR {
        EreR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Abnormal Interrupt Summary Enable"]
    #[inline(always)]
    pub fn aie(&self) -> AieR {
        AieR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Normal Interrupt Summary Enable"]
    #[inline(always)]
    pub fn nie(&self) -> NieR {
        NieR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Transmit Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tie(&mut self) -> TieW<InterruptEnableSpec> {
        TieW::new(self, 0)
    }
    #[doc = "Bit 1 - Transmit Stopped Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tse(&mut self) -> TseW<InterruptEnableSpec> {
        TseW::new(self, 1)
    }
    #[doc = "Bit 2 - Transmit Buffer Unvailable Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tue(&mut self) -> TueW<InterruptEnableSpec> {
        TueW::new(self, 2)
    }
    #[doc = "Bit 3 - Transmit Jabber Timeout Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tje(&mut self) -> TjeW<InterruptEnableSpec> {
        TjeW::new(self, 3)
    }
    #[doc = "Bit 4 - Overflow Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ove(&mut self) -> OveW<InterruptEnableSpec> {
        OveW::new(self, 4)
    }
    #[doc = "Bit 5 - Underflow Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn une(&mut self) -> UneW<InterruptEnableSpec> {
        UneW::new(self, 5)
    }
    #[doc = "Bit 6 - Receive Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rie(&mut self) -> RieW<InterruptEnableSpec> {
        RieW::new(self, 6)
    }
    #[doc = "Bit 7 - Receive Buffer Unavailable Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rue(&mut self) -> RueW<InterruptEnableSpec> {
        RueW::new(self, 7)
    }
    #[doc = "Bit 8 - Receive Stopped Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rse(&mut self) -> RseW<InterruptEnableSpec> {
        RseW::new(self, 8)
    }
    #[doc = "Bit 9 - Receive Watchdog Timeout Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rwe(&mut self) -> RweW<InterruptEnableSpec> {
        RweW::new(self, 9)
    }
    #[doc = "Bit 10 - Early Transmit Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ete(&mut self) -> EteW<InterruptEnableSpec> {
        EteW::new(self, 10)
    }
    #[doc = "Bit 13 - Fatal Bus Error Enable"]
    #[inline(always)]
    #[must_use]
    pub fn fbe(&mut self) -> FbeW<InterruptEnableSpec> {
        FbeW::new(self, 13)
    }
    #[doc = "Bit 14 - Early Receive Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ere(&mut self) -> EreW<InterruptEnableSpec> {
        EreW::new(self, 14)
    }
    #[doc = "Bit 15 - Abnormal Interrupt Summary Enable"]
    #[inline(always)]
    #[must_use]
    pub fn aie(&mut self) -> AieW<InterruptEnableSpec> {
        AieW::new(self, 15)
    }
    #[doc = "Bit 16 - Normal Interrupt Summary Enable"]
    #[inline(always)]
    #[must_use]
    pub fn nie(&mut self) -> NieW<InterruptEnableSpec> {
        NieW::new(self, 16)
    }
}
#[doc = "Interrupt Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`interrupt_enable::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`interrupt_enable::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct InterruptEnableSpec;
impl crate::RegisterSpec for InterruptEnableSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`interrupt_enable::R`](R) reader structure"]
impl crate::Readable for InterruptEnableSpec {}
#[doc = "`write(|w| ..)` method takes [`interrupt_enable::W`](W) writer structure"]
impl crate::Writable for InterruptEnableSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTERRUPT_ENABLE to value 0"]
impl crate::Resettable for InterruptEnableSpec {
    const RESET_VALUE: u32 = 0;
}
