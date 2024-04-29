#[doc = "Register `INTERRUPT_ENABLE` reader"]
pub type R = crate::R<INTERRUPT_ENABLE_SPEC>;
#[doc = "Register `INTERRUPT_ENABLE` writer"]
pub type W = crate::W<INTERRUPT_ENABLE_SPEC>;
#[doc = "Field `TIE` reader - Transmit Interrupt Enable"]
pub type TIE_R = crate::BitReader;
#[doc = "Field `TIE` writer - Transmit Interrupt Enable"]
pub type TIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSE` reader - Transmit Stopped Enable"]
pub type TSE_R = crate::BitReader;
#[doc = "Field `TSE` writer - Transmit Stopped Enable"]
pub type TSE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TUE` reader - Transmit Buffer Unvailable Enable"]
pub type TUE_R = crate::BitReader;
#[doc = "Field `TUE` writer - Transmit Buffer Unvailable Enable"]
pub type TUE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TJE` reader - Transmit Jabber Timeout Enable"]
pub type TJE_R = crate::BitReader;
#[doc = "Field `TJE` writer - Transmit Jabber Timeout Enable"]
pub type TJE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVE` reader - Overflow Interrupt Enable"]
pub type OVE_R = crate::BitReader;
#[doc = "Field `OVE` writer - Overflow Interrupt Enable"]
pub type OVE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UNE` reader - Underflow Interrupt Enable"]
pub type UNE_R = crate::BitReader;
#[doc = "Field `UNE` writer - Underflow Interrupt Enable"]
pub type UNE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RIE` reader - Receive Interrupt Enable"]
pub type RIE_R = crate::BitReader;
#[doc = "Field `RIE` writer - Receive Interrupt Enable"]
pub type RIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RUE` reader - Receive Buffer Unavailable Enable"]
pub type RUE_R = crate::BitReader;
#[doc = "Field `RUE` writer - Receive Buffer Unavailable Enable"]
pub type RUE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSE` reader - Receive Stopped Enable"]
pub type RSE_R = crate::BitReader;
#[doc = "Field `RSE` writer - Receive Stopped Enable"]
pub type RSE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RWE` reader - Receive Watchdog Timeout Enable"]
pub type RWE_R = crate::BitReader;
#[doc = "Field `RWE` writer - Receive Watchdog Timeout Enable"]
pub type RWE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ETE` reader - Early Transmit Interrupt Enable"]
pub type ETE_R = crate::BitReader;
#[doc = "Field `ETE` writer - Early Transmit Interrupt Enable"]
pub type ETE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FBE` reader - Fatal Bus Error Enable"]
pub type FBE_R = crate::BitReader;
#[doc = "Field `FBE` writer - Fatal Bus Error Enable"]
pub type FBE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERE` reader - Early Receive Interrupt Enable"]
pub type ERE_R = crate::BitReader;
#[doc = "Field `ERE` writer - Early Receive Interrupt Enable"]
pub type ERE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AIE` reader - Abnormal Interrupt Summary Enable"]
pub type AIE_R = crate::BitReader;
#[doc = "Field `AIE` writer - Abnormal Interrupt Summary Enable"]
pub type AIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NIE` reader - Normal Interrupt Summary Enable"]
pub type NIE_R = crate::BitReader;
#[doc = "Field `NIE` writer - Normal Interrupt Summary Enable"]
pub type NIE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Transmit Interrupt Enable"]
    #[inline(always)]
    pub fn tie(&self) -> TIE_R {
        TIE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmit Stopped Enable"]
    #[inline(always)]
    pub fn tse(&self) -> TSE_R {
        TSE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Transmit Buffer Unvailable Enable"]
    #[inline(always)]
    pub fn tue(&self) -> TUE_R {
        TUE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Transmit Jabber Timeout Enable"]
    #[inline(always)]
    pub fn tje(&self) -> TJE_R {
        TJE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Overflow Interrupt Enable"]
    #[inline(always)]
    pub fn ove(&self) -> OVE_R {
        OVE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Underflow Interrupt Enable"]
    #[inline(always)]
    pub fn une(&self) -> UNE_R {
        UNE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Receive Interrupt Enable"]
    #[inline(always)]
    pub fn rie(&self) -> RIE_R {
        RIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Receive Buffer Unavailable Enable"]
    #[inline(always)]
    pub fn rue(&self) -> RUE_R {
        RUE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Receive Stopped Enable"]
    #[inline(always)]
    pub fn rse(&self) -> RSE_R {
        RSE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Receive Watchdog Timeout Enable"]
    #[inline(always)]
    pub fn rwe(&self) -> RWE_R {
        RWE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Early Transmit Interrupt Enable"]
    #[inline(always)]
    pub fn ete(&self) -> ETE_R {
        ETE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 13 - Fatal Bus Error Enable"]
    #[inline(always)]
    pub fn fbe(&self) -> FBE_R {
        FBE_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Early Receive Interrupt Enable"]
    #[inline(always)]
    pub fn ere(&self) -> ERE_R {
        ERE_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Abnormal Interrupt Summary Enable"]
    #[inline(always)]
    pub fn aie(&self) -> AIE_R {
        AIE_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Normal Interrupt Summary Enable"]
    #[inline(always)]
    pub fn nie(&self) -> NIE_R {
        NIE_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Transmit Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tie(&mut self) -> TIE_W<INTERRUPT_ENABLE_SPEC> {
        TIE_W::new(self, 0)
    }
    #[doc = "Bit 1 - Transmit Stopped Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tse(&mut self) -> TSE_W<INTERRUPT_ENABLE_SPEC> {
        TSE_W::new(self, 1)
    }
    #[doc = "Bit 2 - Transmit Buffer Unvailable Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tue(&mut self) -> TUE_W<INTERRUPT_ENABLE_SPEC> {
        TUE_W::new(self, 2)
    }
    #[doc = "Bit 3 - Transmit Jabber Timeout Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tje(&mut self) -> TJE_W<INTERRUPT_ENABLE_SPEC> {
        TJE_W::new(self, 3)
    }
    #[doc = "Bit 4 - Overflow Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ove(&mut self) -> OVE_W<INTERRUPT_ENABLE_SPEC> {
        OVE_W::new(self, 4)
    }
    #[doc = "Bit 5 - Underflow Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn une(&mut self) -> UNE_W<INTERRUPT_ENABLE_SPEC> {
        UNE_W::new(self, 5)
    }
    #[doc = "Bit 6 - Receive Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rie(&mut self) -> RIE_W<INTERRUPT_ENABLE_SPEC> {
        RIE_W::new(self, 6)
    }
    #[doc = "Bit 7 - Receive Buffer Unavailable Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rue(&mut self) -> RUE_W<INTERRUPT_ENABLE_SPEC> {
        RUE_W::new(self, 7)
    }
    #[doc = "Bit 8 - Receive Stopped Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rse(&mut self) -> RSE_W<INTERRUPT_ENABLE_SPEC> {
        RSE_W::new(self, 8)
    }
    #[doc = "Bit 9 - Receive Watchdog Timeout Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rwe(&mut self) -> RWE_W<INTERRUPT_ENABLE_SPEC> {
        RWE_W::new(self, 9)
    }
    #[doc = "Bit 10 - Early Transmit Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ete(&mut self) -> ETE_W<INTERRUPT_ENABLE_SPEC> {
        ETE_W::new(self, 10)
    }
    #[doc = "Bit 13 - Fatal Bus Error Enable"]
    #[inline(always)]
    #[must_use]
    pub fn fbe(&mut self) -> FBE_W<INTERRUPT_ENABLE_SPEC> {
        FBE_W::new(self, 13)
    }
    #[doc = "Bit 14 - Early Receive Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ere(&mut self) -> ERE_W<INTERRUPT_ENABLE_SPEC> {
        ERE_W::new(self, 14)
    }
    #[doc = "Bit 15 - Abnormal Interrupt Summary Enable"]
    #[inline(always)]
    #[must_use]
    pub fn aie(&mut self) -> AIE_W<INTERRUPT_ENABLE_SPEC> {
        AIE_W::new(self, 15)
    }
    #[doc = "Bit 16 - Normal Interrupt Summary Enable"]
    #[inline(always)]
    #[must_use]
    pub fn nie(&mut self) -> NIE_W<INTERRUPT_ENABLE_SPEC> {
        NIE_W::new(self, 16)
    }
}
#[doc = "Interrupt Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`interrupt_enable::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`interrupt_enable::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTERRUPT_ENABLE_SPEC;
impl crate::RegisterSpec for INTERRUPT_ENABLE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`interrupt_enable::R`](R) reader structure"]
impl crate::Readable for INTERRUPT_ENABLE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`interrupt_enable::W`](W) writer structure"]
impl crate::Writable for INTERRUPT_ENABLE_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTERRUPT_ENABLE to value 0"]
impl crate::Resettable for INTERRUPT_ENABLE_SPEC {
    const RESET_VALUE: u32 = 0;
}
