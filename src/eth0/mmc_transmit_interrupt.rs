#[doc = "Register `MMC_TRANSMIT_INTERRUPT` reader"]
pub type R = crate::R<MMC_TRANSMIT_INTERRUPT_SPEC>;
#[doc = "Field `TXGBOCTIS` reader - MMC Transmit Good Bad Octet Counter Interrupt Status"]
pub type TXGBOCTIS_R = crate::BitReader;
#[doc = "Field `TXGBFRMIS` reader - MMC Transmit Good Bad Frame Counter Interrupt Status"]
pub type TXGBFRMIS_R = crate::BitReader;
#[doc = "Field `TXBCGFIS` reader - MMC Transmit Broadcast Good Frame Counter Interrupt Status"]
pub type TXBCGFIS_R = crate::BitReader;
#[doc = "Field `TXMCGFIS` reader - MMC Transmit Multicast Good Frame Counter Interrupt Status"]
pub type TXMCGFIS_R = crate::BitReader;
#[doc = "Field `TX64OCTGBFIS` reader - MMC Transmit 64 Octet Good Bad Frame Counter Interrupt Status."]
pub type TX64OCTGBFIS_R = crate::BitReader;
#[doc = "Field `TX65T127OCTGBFIS` reader - MMC Transmit 65 to 127 Octet Good Bad Frame Counter Interrupt Status"]
pub type TX65T127OCTGBFIS_R = crate::BitReader;
#[doc = "Field `TX128T255OCTGBFIS` reader - MMC Transmit 128 to 255 Octet Good Bad Frame Counter Interrupt Status"]
pub type TX128T255OCTGBFIS_R = crate::BitReader;
#[doc = "Field `TX256T511OCTGBFIS` reader - MMC Transmit 256 to 511 Octet Good Bad Frame Counter Interrupt Status"]
pub type TX256T511OCTGBFIS_R = crate::BitReader;
#[doc = "Field `TX512T1023OCTGBFIS` reader - MMC Transmit 512 to 1023 Octet Good Bad Frame Counter Interrupt Status"]
pub type TX512T1023OCTGBFIS_R = crate::BitReader;
#[doc = "Field `TX1024TMAXOCTGBFIS` reader - MMC Transmit 1024 to Maximum Octet Good Bad Frame Counter Interrupt Status"]
pub type TX1024TMAXOCTGBFIS_R = crate::BitReader;
#[doc = "Field `TXUCGBFIS` reader - MMC Transmit Unicast Good Bad Frame Counter Interrupt Status"]
pub type TXUCGBFIS_R = crate::BitReader;
#[doc = "Field `TXMCGBFIS` reader - MMC Transmit Multicast Good Bad Frame Counter Interrupt Status"]
pub type TXMCGBFIS_R = crate::BitReader;
#[doc = "Field `TXBCGBFIS` reader - MMC Transmit Broadcast Good Bad Frame Counter Interrupt Status"]
pub type TXBCGBFIS_R = crate::BitReader;
#[doc = "Field `TXUFLOWERFIS` reader - MMC Transmit Underflow Error Frame Counter Interrupt Status"]
pub type TXUFLOWERFIS_R = crate::BitReader;
#[doc = "Field `TXSCOLGFIS` reader - MMC Transmit Single Collision Good Frame Counter Interrupt Status"]
pub type TXSCOLGFIS_R = crate::BitReader;
#[doc = "Field `TXMCOLGFIS` reader - MMC Transmit Multiple Collision Good Frame Counter Interrupt Status"]
pub type TXMCOLGFIS_R = crate::BitReader;
#[doc = "Field `TXDEFFIS` reader - MMC Transmit Deferred Frame Counter Interrupt Status"]
pub type TXDEFFIS_R = crate::BitReader;
#[doc = "Field `TXLATCOLFIS` reader - MMC Transmit Late Collision Frame Counter Interrupt Status"]
pub type TXLATCOLFIS_R = crate::BitReader;
#[doc = "Field `TXEXCOLFIS` reader - MMC Transmit Excessive Collision Frame Counter Interrupt Status"]
pub type TXEXCOLFIS_R = crate::BitReader;
#[doc = "Field `TXCARERFIS` reader - MMC Transmit Carrier Error Frame Counter Interrupt Status"]
pub type TXCARERFIS_R = crate::BitReader;
#[doc = "Field `TXGOCTIS` reader - MMC Transmit Good Octet Counter Interrupt Status"]
pub type TXGOCTIS_R = crate::BitReader;
#[doc = "Field `TXGFRMIS` reader - MMC Transmit Good Frame Counter Interrupt Status"]
pub type TXGFRMIS_R = crate::BitReader;
#[doc = "Field `TXEXDEFFIS` reader - MMC Transmit Excessive Deferral Frame Counter Interrupt Status"]
pub type TXEXDEFFIS_R = crate::BitReader;
#[doc = "Field `TXPAUSFIS` reader - MMC Transmit Pause Frame Counter Interrupt Status"]
pub type TXPAUSFIS_R = crate::BitReader;
#[doc = "Field `TXVLANGFIS` reader - MMC Transmit VLAN Good Frame Counter Interrupt Status"]
pub type TXVLANGFIS_R = crate::BitReader;
#[doc = "Field `TXOSIZEGFIS` reader - MMC Transmit Oversize Good Frame Counter Interrupt Status"]
pub type TXOSIZEGFIS_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - MMC Transmit Good Bad Octet Counter Interrupt Status"]
    #[inline(always)]
    pub fn txgboctis(&self) -> TXGBOCTIS_R {
        TXGBOCTIS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - MMC Transmit Good Bad Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn txgbfrmis(&self) -> TXGBFRMIS_R {
        TXGBFRMIS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - MMC Transmit Broadcast Good Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn txbcgfis(&self) -> TXBCGFIS_R {
        TXBCGFIS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - MMC Transmit Multicast Good Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn txmcgfis(&self) -> TXMCGFIS_R {
        TXMCGFIS_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - MMC Transmit 64 Octet Good Bad Frame Counter Interrupt Status."]
    #[inline(always)]
    pub fn tx64octgbfis(&self) -> TX64OCTGBFIS_R {
        TX64OCTGBFIS_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - MMC Transmit 65 to 127 Octet Good Bad Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn tx65t127octgbfis(&self) -> TX65T127OCTGBFIS_R {
        TX65T127OCTGBFIS_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - MMC Transmit 128 to 255 Octet Good Bad Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn tx128t255octgbfis(&self) -> TX128T255OCTGBFIS_R {
        TX128T255OCTGBFIS_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - MMC Transmit 256 to 511 Octet Good Bad Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn tx256t511octgbfis(&self) -> TX256T511OCTGBFIS_R {
        TX256T511OCTGBFIS_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - MMC Transmit 512 to 1023 Octet Good Bad Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn tx512t1023octgbfis(&self) -> TX512T1023OCTGBFIS_R {
        TX512T1023OCTGBFIS_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - MMC Transmit 1024 to Maximum Octet Good Bad Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn tx1024tmaxoctgbfis(&self) -> TX1024TMAXOCTGBFIS_R {
        TX1024TMAXOCTGBFIS_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - MMC Transmit Unicast Good Bad Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn txucgbfis(&self) -> TXUCGBFIS_R {
        TXUCGBFIS_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - MMC Transmit Multicast Good Bad Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn txmcgbfis(&self) -> TXMCGBFIS_R {
        TXMCGBFIS_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - MMC Transmit Broadcast Good Bad Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn txbcgbfis(&self) -> TXBCGBFIS_R {
        TXBCGBFIS_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - MMC Transmit Underflow Error Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn txuflowerfis(&self) -> TXUFLOWERFIS_R {
        TXUFLOWERFIS_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - MMC Transmit Single Collision Good Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn txscolgfis(&self) -> TXSCOLGFIS_R {
        TXSCOLGFIS_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - MMC Transmit Multiple Collision Good Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn txmcolgfis(&self) -> TXMCOLGFIS_R {
        TXMCOLGFIS_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - MMC Transmit Deferred Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn txdeffis(&self) -> TXDEFFIS_R {
        TXDEFFIS_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - MMC Transmit Late Collision Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn txlatcolfis(&self) -> TXLATCOLFIS_R {
        TXLATCOLFIS_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - MMC Transmit Excessive Collision Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn txexcolfis(&self) -> TXEXCOLFIS_R {
        TXEXCOLFIS_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - MMC Transmit Carrier Error Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn txcarerfis(&self) -> TXCARERFIS_R {
        TXCARERFIS_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - MMC Transmit Good Octet Counter Interrupt Status"]
    #[inline(always)]
    pub fn txgoctis(&self) -> TXGOCTIS_R {
        TXGOCTIS_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - MMC Transmit Good Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn txgfrmis(&self) -> TXGFRMIS_R {
        TXGFRMIS_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - MMC Transmit Excessive Deferral Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn txexdeffis(&self) -> TXEXDEFFIS_R {
        TXEXDEFFIS_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - MMC Transmit Pause Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn txpausfis(&self) -> TXPAUSFIS_R {
        TXPAUSFIS_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - MMC Transmit VLAN Good Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn txvlangfis(&self) -> TXVLANGFIS_R {
        TXVLANGFIS_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - MMC Transmit Oversize Good Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn txosizegfis(&self) -> TXOSIZEGFIS_R {
        TXOSIZEGFIS_R::new(((self.bits >> 25) & 1) != 0)
    }
}
#[doc = "MMC Transmit Interrupt Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmc_transmit_interrupt::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MMC_TRANSMIT_INTERRUPT_SPEC;
impl crate::RegisterSpec for MMC_TRANSMIT_INTERRUPT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mmc_transmit_interrupt::R`](R) reader structure"]
impl crate::Readable for MMC_TRANSMIT_INTERRUPT_SPEC {}
#[doc = "`reset()` method sets MMC_TRANSMIT_INTERRUPT to value 0"]
impl crate::Resettable for MMC_TRANSMIT_INTERRUPT_SPEC {
    const RESET_VALUE: u32 = 0;
}
