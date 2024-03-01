#[doc = "Register `MAC_FRAME_FILTER` reader"]
pub type R = crate::R<MacFrameFilterSpec>;
#[doc = "Register `MAC_FRAME_FILTER` writer"]
pub type W = crate::W<MacFrameFilterSpec>;
#[doc = "Field `PR` reader - Promiscuous Mode"]
pub type PrR = crate::BitReader;
#[doc = "Field `PR` writer - Promiscuous Mode"]
pub type PrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HUC` reader - Hash Unicast"]
pub type HucR = crate::BitReader;
#[doc = "Field `HUC` writer - Hash Unicast"]
pub type HucW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HMC` reader - Hash Multicast"]
pub type HmcR = crate::BitReader;
#[doc = "Field `HMC` writer - Hash Multicast"]
pub type HmcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DAIF` reader - DA Inverse Filtering"]
pub type DaifR = crate::BitReader;
#[doc = "Field `DAIF` writer - DA Inverse Filtering"]
pub type DaifW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PM` reader - Pass All Multicast"]
pub type PmR = crate::BitReader;
#[doc = "Field `PM` writer - Pass All Multicast"]
pub type PmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBF` reader - Disable Broadcast Frames"]
pub type DbfR = crate::BitReader;
#[doc = "Field `DBF` writer - Disable Broadcast Frames"]
pub type DbfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCF` reader - Pass Control Frames"]
pub type PcfR = crate::FieldReader;
#[doc = "Field `PCF` writer - Pass Control Frames"]
pub type PcfW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SAIF` reader - SA Inverse Filtering"]
pub type SaifR = crate::BitReader;
#[doc = "Field `SAIF` writer - SA Inverse Filtering"]
pub type SaifW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SAF` reader - Source Address Filter Enable"]
pub type SafR = crate::BitReader;
#[doc = "Field `SAF` writer - Source Address Filter Enable"]
pub type SafW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HPF` reader - Hash or Perfect Filter"]
pub type HpfR = crate::BitReader;
#[doc = "Field `HPF` writer - Hash or Perfect Filter"]
pub type HpfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VTFE` reader - VLAN Tag Filter Enable"]
pub type VtfeR = crate::BitReader;
#[doc = "Field `VTFE` writer - VLAN Tag Filter Enable"]
pub type VtfeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IPFE` reader - Layer 3 and Layer 4 Filter Enable"]
pub type IpfeR = crate::BitReader;
#[doc = "Field `DNTU` reader - Drop non-TCP/UDP over IP Frames"]
pub type DntuR = crate::BitReader;
#[doc = "Field `RA` reader - Receive All"]
pub type RaR = crate::BitReader;
#[doc = "Field `RA` writer - Receive All"]
pub type RaW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Promiscuous Mode"]
    #[inline(always)]
    pub fn pr(&self) -> PrR {
        PrR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Hash Unicast"]
    #[inline(always)]
    pub fn huc(&self) -> HucR {
        HucR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Hash Multicast"]
    #[inline(always)]
    pub fn hmc(&self) -> HmcR {
        HmcR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DA Inverse Filtering"]
    #[inline(always)]
    pub fn daif(&self) -> DaifR {
        DaifR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Pass All Multicast"]
    #[inline(always)]
    pub fn pm(&self) -> PmR {
        PmR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Disable Broadcast Frames"]
    #[inline(always)]
    pub fn dbf(&self) -> DbfR {
        DbfR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - Pass Control Frames"]
    #[inline(always)]
    pub fn pcf(&self) -> PcfR {
        PcfR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 8 - SA Inverse Filtering"]
    #[inline(always)]
    pub fn saif(&self) -> SaifR {
        SaifR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Source Address Filter Enable"]
    #[inline(always)]
    pub fn saf(&self) -> SafR {
        SafR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Hash or Perfect Filter"]
    #[inline(always)]
    pub fn hpf(&self) -> HpfR {
        HpfR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 16 - VLAN Tag Filter Enable"]
    #[inline(always)]
    pub fn vtfe(&self) -> VtfeR {
        VtfeR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 20 - Layer 3 and Layer 4 Filter Enable"]
    #[inline(always)]
    pub fn ipfe(&self) -> IpfeR {
        IpfeR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Drop non-TCP/UDP over IP Frames"]
    #[inline(always)]
    pub fn dntu(&self) -> DntuR {
        DntuR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 31 - Receive All"]
    #[inline(always)]
    pub fn ra(&self) -> RaR {
        RaR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Promiscuous Mode"]
    #[inline(always)]
    #[must_use]
    pub fn pr(&mut self) -> PrW<MacFrameFilterSpec> {
        PrW::new(self, 0)
    }
    #[doc = "Bit 1 - Hash Unicast"]
    #[inline(always)]
    #[must_use]
    pub fn huc(&mut self) -> HucW<MacFrameFilterSpec> {
        HucW::new(self, 1)
    }
    #[doc = "Bit 2 - Hash Multicast"]
    #[inline(always)]
    #[must_use]
    pub fn hmc(&mut self) -> HmcW<MacFrameFilterSpec> {
        HmcW::new(self, 2)
    }
    #[doc = "Bit 3 - DA Inverse Filtering"]
    #[inline(always)]
    #[must_use]
    pub fn daif(&mut self) -> DaifW<MacFrameFilterSpec> {
        DaifW::new(self, 3)
    }
    #[doc = "Bit 4 - Pass All Multicast"]
    #[inline(always)]
    #[must_use]
    pub fn pm(&mut self) -> PmW<MacFrameFilterSpec> {
        PmW::new(self, 4)
    }
    #[doc = "Bit 5 - Disable Broadcast Frames"]
    #[inline(always)]
    #[must_use]
    pub fn dbf(&mut self) -> DbfW<MacFrameFilterSpec> {
        DbfW::new(self, 5)
    }
    #[doc = "Bits 6:7 - Pass Control Frames"]
    #[inline(always)]
    #[must_use]
    pub fn pcf(&mut self) -> PcfW<MacFrameFilterSpec> {
        PcfW::new(self, 6)
    }
    #[doc = "Bit 8 - SA Inverse Filtering"]
    #[inline(always)]
    #[must_use]
    pub fn saif(&mut self) -> SaifW<MacFrameFilterSpec> {
        SaifW::new(self, 8)
    }
    #[doc = "Bit 9 - Source Address Filter Enable"]
    #[inline(always)]
    #[must_use]
    pub fn saf(&mut self) -> SafW<MacFrameFilterSpec> {
        SafW::new(self, 9)
    }
    #[doc = "Bit 10 - Hash or Perfect Filter"]
    #[inline(always)]
    #[must_use]
    pub fn hpf(&mut self) -> HpfW<MacFrameFilterSpec> {
        HpfW::new(self, 10)
    }
    #[doc = "Bit 16 - VLAN Tag Filter Enable"]
    #[inline(always)]
    #[must_use]
    pub fn vtfe(&mut self) -> VtfeW<MacFrameFilterSpec> {
        VtfeW::new(self, 16)
    }
    #[doc = "Bit 31 - Receive All"]
    #[inline(always)]
    #[must_use]
    pub fn ra(&mut self) -> RaW<MacFrameFilterSpec> {
        RaW::new(self, 31)
    }
}
#[doc = "MAC Frame Filter\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mac_frame_filter::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mac_frame_filter::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MacFrameFilterSpec;
impl crate::RegisterSpec for MacFrameFilterSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mac_frame_filter::R`](R) reader structure"]
impl crate::Readable for MacFrameFilterSpec {}
#[doc = "`write(|w| ..)` method takes [`mac_frame_filter::W`](W) writer structure"]
impl crate::Writable for MacFrameFilterSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MAC_FRAME_FILTER to value 0"]
impl crate::Resettable for MacFrameFilterSpec {
    const RESET_VALUE: u32 = 0;
}
