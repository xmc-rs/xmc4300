#[doc = "Register `STATUS` reader"]
pub type R = crate::R<StatusSpec>;
#[doc = "Register `STATUS` writer"]
pub type W = crate::W<StatusSpec>;
#[doc = "Field `TI` reader - Transmit Interrupt"]
pub type TiR = crate::BitReader;
#[doc = "Field `TI` writer - Transmit Interrupt"]
pub type TiW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TPS` reader - Transmit Process Stopped"]
pub type TpsR = crate::BitReader;
#[doc = "Field `TPS` writer - Transmit Process Stopped"]
pub type TpsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TU` reader - Transmit Buffer Unavailable"]
pub type TuR = crate::BitReader;
#[doc = "Field `TU` writer - Transmit Buffer Unavailable"]
pub type TuW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TJT` reader - Transmit Jabber Timeout"]
pub type TjtR = crate::BitReader;
#[doc = "Field `TJT` writer - Transmit Jabber Timeout"]
pub type TjtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVF` reader - Receive Overflow"]
pub type OvfR = crate::BitReader;
#[doc = "Field `OVF` writer - Receive Overflow"]
pub type OvfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UNF` reader - Transmit Underflow"]
pub type UnfR = crate::BitReader;
#[doc = "Field `UNF` writer - Transmit Underflow"]
pub type UnfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RI` reader - Receive Interrupt"]
pub type RiR = crate::BitReader;
#[doc = "Field `RI` writer - Receive Interrupt"]
pub type RiW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RU` reader - Receive Buffer Unavailable"]
pub type RuR = crate::BitReader;
#[doc = "Field `RU` writer - Receive Buffer Unavailable"]
pub type RuW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RPS` reader - Receive Process Stopped"]
pub type RpsR = crate::BitReader;
#[doc = "Field `RPS` writer - Receive Process Stopped"]
pub type RpsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RWT` reader - Receive Watchdog Timeout"]
pub type RwtR = crate::BitReader;
#[doc = "Field `RWT` writer - Receive Watchdog Timeout"]
pub type RwtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ETI` reader - Early Transmit Interrupt"]
pub type EtiR = crate::BitReader;
#[doc = "Field `ETI` writer - Early Transmit Interrupt"]
pub type EtiW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FBI` reader - Fatal Bus Error Interrupt"]
pub type FbiR = crate::BitReader;
#[doc = "Field `FBI` writer - Fatal Bus Error Interrupt"]
pub type FbiW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERI` reader - Early Receive Interrupt"]
pub type EriR = crate::BitReader;
#[doc = "Field `ERI` writer - Early Receive Interrupt"]
pub type EriW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AIS` reader - Abnormal Interrupt Summary"]
pub type AisR = crate::BitReader;
#[doc = "Field `AIS` writer - Abnormal Interrupt Summary"]
pub type AisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NIS` reader - Normal Interrupt Summary"]
pub type NisR = crate::BitReader;
#[doc = "Field `NIS` writer - Normal Interrupt Summary"]
pub type NisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RS` reader - Received Process State"]
pub type RsR = crate::FieldReader;
#[doc = "Field `TS` reader - Transmit Process State"]
pub type TsR = crate::FieldReader;
#[doc = "Field `EB` reader - Error Bits"]
pub type EbR = crate::FieldReader;
#[doc = "Field `EMI` reader - ETH MMC Interrupt"]
pub type EmiR = crate::BitReader;
#[doc = "Field `EPI` reader - ETH PMT Interrupt"]
pub type EpiR = crate::BitReader;
#[doc = "Field `TTI` reader - Timestamp Trigger Interrupt"]
pub type TtiR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Transmit Interrupt"]
    #[inline(always)]
    pub fn ti(&self) -> TiR {
        TiR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmit Process Stopped"]
    #[inline(always)]
    pub fn tps(&self) -> TpsR {
        TpsR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Transmit Buffer Unavailable"]
    #[inline(always)]
    pub fn tu(&self) -> TuR {
        TuR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Transmit Jabber Timeout"]
    #[inline(always)]
    pub fn tjt(&self) -> TjtR {
        TjtR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Receive Overflow"]
    #[inline(always)]
    pub fn ovf(&self) -> OvfR {
        OvfR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Transmit Underflow"]
    #[inline(always)]
    pub fn unf(&self) -> UnfR {
        UnfR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Receive Interrupt"]
    #[inline(always)]
    pub fn ri(&self) -> RiR {
        RiR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Receive Buffer Unavailable"]
    #[inline(always)]
    pub fn ru(&self) -> RuR {
        RuR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Receive Process Stopped"]
    #[inline(always)]
    pub fn rps(&self) -> RpsR {
        RpsR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Receive Watchdog Timeout"]
    #[inline(always)]
    pub fn rwt(&self) -> RwtR {
        RwtR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Early Transmit Interrupt"]
    #[inline(always)]
    pub fn eti(&self) -> EtiR {
        EtiR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 13 - Fatal Bus Error Interrupt"]
    #[inline(always)]
    pub fn fbi(&self) -> FbiR {
        FbiR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Early Receive Interrupt"]
    #[inline(always)]
    pub fn eri(&self) -> EriR {
        EriR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Abnormal Interrupt Summary"]
    #[inline(always)]
    pub fn ais(&self) -> AisR {
        AisR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Normal Interrupt Summary"]
    #[inline(always)]
    pub fn nis(&self) -> NisR {
        NisR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:19 - Received Process State"]
    #[inline(always)]
    pub fn rs(&self) -> RsR {
        RsR::new(((self.bits >> 17) & 7) as u8)
    }
    #[doc = "Bits 20:22 - Transmit Process State"]
    #[inline(always)]
    pub fn ts(&self) -> TsR {
        TsR::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bits 23:25 - Error Bits"]
    #[inline(always)]
    pub fn eb(&self) -> EbR {
        EbR::new(((self.bits >> 23) & 7) as u8)
    }
    #[doc = "Bit 27 - ETH MMC Interrupt"]
    #[inline(always)]
    pub fn emi(&self) -> EmiR {
        EmiR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - ETH PMT Interrupt"]
    #[inline(always)]
    pub fn epi(&self) -> EpiR {
        EpiR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Timestamp Trigger Interrupt"]
    #[inline(always)]
    pub fn tti(&self) -> TtiR {
        TtiR::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Transmit Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn ti(&mut self) -> TiW<StatusSpec> {
        TiW::new(self, 0)
    }
    #[doc = "Bit 1 - Transmit Process Stopped"]
    #[inline(always)]
    #[must_use]
    pub fn tps(&mut self) -> TpsW<StatusSpec> {
        TpsW::new(self, 1)
    }
    #[doc = "Bit 2 - Transmit Buffer Unavailable"]
    #[inline(always)]
    #[must_use]
    pub fn tu(&mut self) -> TuW<StatusSpec> {
        TuW::new(self, 2)
    }
    #[doc = "Bit 3 - Transmit Jabber Timeout"]
    #[inline(always)]
    #[must_use]
    pub fn tjt(&mut self) -> TjtW<StatusSpec> {
        TjtW::new(self, 3)
    }
    #[doc = "Bit 4 - Receive Overflow"]
    #[inline(always)]
    #[must_use]
    pub fn ovf(&mut self) -> OvfW<StatusSpec> {
        OvfW::new(self, 4)
    }
    #[doc = "Bit 5 - Transmit Underflow"]
    #[inline(always)]
    #[must_use]
    pub fn unf(&mut self) -> UnfW<StatusSpec> {
        UnfW::new(self, 5)
    }
    #[doc = "Bit 6 - Receive Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn ri(&mut self) -> RiW<StatusSpec> {
        RiW::new(self, 6)
    }
    #[doc = "Bit 7 - Receive Buffer Unavailable"]
    #[inline(always)]
    #[must_use]
    pub fn ru(&mut self) -> RuW<StatusSpec> {
        RuW::new(self, 7)
    }
    #[doc = "Bit 8 - Receive Process Stopped"]
    #[inline(always)]
    #[must_use]
    pub fn rps(&mut self) -> RpsW<StatusSpec> {
        RpsW::new(self, 8)
    }
    #[doc = "Bit 9 - Receive Watchdog Timeout"]
    #[inline(always)]
    #[must_use]
    pub fn rwt(&mut self) -> RwtW<StatusSpec> {
        RwtW::new(self, 9)
    }
    #[doc = "Bit 10 - Early Transmit Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn eti(&mut self) -> EtiW<StatusSpec> {
        EtiW::new(self, 10)
    }
    #[doc = "Bit 13 - Fatal Bus Error Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn fbi(&mut self) -> FbiW<StatusSpec> {
        FbiW::new(self, 13)
    }
    #[doc = "Bit 14 - Early Receive Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn eri(&mut self) -> EriW<StatusSpec> {
        EriW::new(self, 14)
    }
    #[doc = "Bit 15 - Abnormal Interrupt Summary"]
    #[inline(always)]
    #[must_use]
    pub fn ais(&mut self) -> AisW<StatusSpec> {
        AisW::new(self, 15)
    }
    #[doc = "Bit 16 - Normal Interrupt Summary"]
    #[inline(always)]
    #[must_use]
    pub fn nis(&mut self) -> NisW<StatusSpec> {
        NisW::new(self, 16)
    }
}
#[doc = "Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`status::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`status::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StatusSpec;
impl crate::RegisterSpec for StatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`status::R`](R) reader structure"]
impl crate::Readable for StatusSpec {}
#[doc = "`write(|w| ..)` method takes [`status::W`](W) writer structure"]
impl crate::Writable for StatusSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets STATUS to value 0"]
impl crate::Resettable for StatusSpec {
    const RESET_VALUE: u32 = 0;
}
