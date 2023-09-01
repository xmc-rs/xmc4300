#[doc = "Register `MMC_RECEIVE_INTERRUPT` reader"]
pub type R = crate::R<MMC_RECEIVE_INTERRUPT_SPEC>;
#[doc = "Field `RXGBFRMIS` reader - MMC Receive Good Bad Frame Counter Interrupt Status"]
pub type RXGBFRMIS_R = crate::BitReader;
#[doc = "Field `RXGBOCTIS` reader - MMC Receive Good Bad Octet Counter Interrupt Status"]
pub type RXGBOCTIS_R = crate::BitReader;
#[doc = "Field `RXGOCTIS` reader - MMC Receive Good Octet Counter Interrupt Status."]
pub type RXGOCTIS_R = crate::BitReader;
#[doc = "Field `RXBCGFIS` reader - MMC Receive Broadcast Good Frame Counter Interrupt Status."]
pub type RXBCGFIS_R = crate::BitReader;
#[doc = "Field `RXMCGFIS` reader - MMC Receive Multicast Good Frame Counter Interrupt Status"]
pub type RXMCGFIS_R = crate::BitReader;
#[doc = "Field `RXCRCERFIS` reader - MMC Receive CRC Error Frame Counter Interrupt Status"]
pub type RXCRCERFIS_R = crate::BitReader;
#[doc = "Field `RXALGNERFIS` reader - MMC Receive Alignment Error Frame Counter Interrupt Status"]
pub type RXALGNERFIS_R = crate::BitReader;
#[doc = "Field `RXRUNTFIS` reader - MMC Receive Runt Frame Counter Interrupt Status"]
pub type RXRUNTFIS_R = crate::BitReader;
#[doc = "Field `RXJABERFIS` reader - MMC Receive Jabber Error Frame Counter Interrupt Status"]
pub type RXJABERFIS_R = crate::BitReader;
#[doc = "Field `RXUSIZEGFIS` reader - MMC Receive Undersize Good Frame Counter Interrupt Status"]
pub type RXUSIZEGFIS_R = crate::BitReader;
#[doc = "Field `RXOSIZEGFIS` reader - MMC Receive Oversize Good Frame Counter Interrupt Status"]
pub type RXOSIZEGFIS_R = crate::BitReader;
#[doc = "Field `RX64OCTGBFIS` reader - MMC Receive 64 Octet Good Bad Frame Counter Interrupt Status"]
pub type RX64OCTGBFIS_R = crate::BitReader;
#[doc = "Field `RX65T127OCTGBFIS` reader - MMC Receive 65 to 127 Octet Good Bad Frame Counter Interrupt Status"]
pub type RX65T127OCTGBFIS_R = crate::BitReader;
#[doc = "Field `RX128T255OCTGBFIS` reader - MMC Receive 128 to 255 Octet Good Bad Frame Counter Interrupt Status"]
pub type RX128T255OCTGBFIS_R = crate::BitReader;
#[doc = "Field `RX256T511OCTGBFIS` reader - MMC Receive 256 to 511 Octet Good Bad Frame Counter Interrupt Status"]
pub type RX256T511OCTGBFIS_R = crate::BitReader;
#[doc = "Field `RX512T1023OCTGBFIS` reader - MMC Receive 512 to 1023 Octet Good Bad Frame Counter Interrupt Status"]
pub type RX512T1023OCTGBFIS_R = crate::BitReader;
#[doc = "Field `RX1024TMAXOCTGBFIS` reader - MMC Receive 1024 to Maximum Octet Good Bad Frame Counter Interrupt Status"]
pub type RX1024TMAXOCTGBFIS_R = crate::BitReader;
#[doc = "Field `RXUCGFIS` reader - MMC Receive Unicast Good Frame Counter Interrupt Status"]
pub type RXUCGFIS_R = crate::BitReader;
#[doc = "Field `RXLENERFIS` reader - MMC Receive Length Error Frame Counter Interrupt Status"]
pub type RXLENERFIS_R = crate::BitReader;
#[doc = "Field `RXORANGEFIS` reader - MMC Receive Out Of Range Error Frame Counter Interrupt Status"]
pub type RXORANGEFIS_R = crate::BitReader;
#[doc = "Field `RXPAUSFIS` reader - MMC Receive Pause Frame Counter Interrupt Status"]
pub type RXPAUSFIS_R = crate::BitReader;
#[doc = "Field `RXFOVFIS` reader - MMC Receive FIFO Overflow Frame Counter Interrupt Status"]
pub type RXFOVFIS_R = crate::BitReader;
#[doc = "Field `RXVLANGBFIS` reader - MMC Receive VLAN Good Bad Frame Counter Interrupt Status"]
pub type RXVLANGBFIS_R = crate::BitReader;
#[doc = "Field `RXWDOGFIS` reader - MMC Receive Watchdog Error Frame Counter Interrupt Status"]
pub type RXWDOGFIS_R = crate::BitReader;
#[doc = "Field `RXRCVERRFIS` reader - MMC Receive Error Frame Counter Interrupt Status"]
pub type RXRCVERRFIS_R = crate::BitReader;
#[doc = "Field `RXCTRLFIS` reader - MMC Receive Control Frame Counter Interrupt Status"]
pub type RXCTRLFIS_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - MMC Receive Good Bad Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn rxgbfrmis(&self) -> RXGBFRMIS_R {
        RXGBFRMIS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - MMC Receive Good Bad Octet Counter Interrupt Status"]
    #[inline(always)]
    pub fn rxgboctis(&self) -> RXGBOCTIS_R {
        RXGBOCTIS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - MMC Receive Good Octet Counter Interrupt Status."]
    #[inline(always)]
    pub fn rxgoctis(&self) -> RXGOCTIS_R {
        RXGOCTIS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - MMC Receive Broadcast Good Frame Counter Interrupt Status."]
    #[inline(always)]
    pub fn rxbcgfis(&self) -> RXBCGFIS_R {
        RXBCGFIS_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - MMC Receive Multicast Good Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn rxmcgfis(&self) -> RXMCGFIS_R {
        RXMCGFIS_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - MMC Receive CRC Error Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn rxcrcerfis(&self) -> RXCRCERFIS_R {
        RXCRCERFIS_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - MMC Receive Alignment Error Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn rxalgnerfis(&self) -> RXALGNERFIS_R {
        RXALGNERFIS_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - MMC Receive Runt Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn rxruntfis(&self) -> RXRUNTFIS_R {
        RXRUNTFIS_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - MMC Receive Jabber Error Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn rxjaberfis(&self) -> RXJABERFIS_R {
        RXJABERFIS_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - MMC Receive Undersize Good Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn rxusizegfis(&self) -> RXUSIZEGFIS_R {
        RXUSIZEGFIS_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - MMC Receive Oversize Good Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn rxosizegfis(&self) -> RXOSIZEGFIS_R {
        RXOSIZEGFIS_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - MMC Receive 64 Octet Good Bad Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn rx64octgbfis(&self) -> RX64OCTGBFIS_R {
        RX64OCTGBFIS_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - MMC Receive 65 to 127 Octet Good Bad Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn rx65t127octgbfis(&self) -> RX65T127OCTGBFIS_R {
        RX65T127OCTGBFIS_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - MMC Receive 128 to 255 Octet Good Bad Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn rx128t255octgbfis(&self) -> RX128T255OCTGBFIS_R {
        RX128T255OCTGBFIS_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - MMC Receive 256 to 511 Octet Good Bad Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn rx256t511octgbfis(&self) -> RX256T511OCTGBFIS_R {
        RX256T511OCTGBFIS_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - MMC Receive 512 to 1023 Octet Good Bad Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn rx512t1023octgbfis(&self) -> RX512T1023OCTGBFIS_R {
        RX512T1023OCTGBFIS_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - MMC Receive 1024 to Maximum Octet Good Bad Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn rx1024tmaxoctgbfis(&self) -> RX1024TMAXOCTGBFIS_R {
        RX1024TMAXOCTGBFIS_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - MMC Receive Unicast Good Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn rxucgfis(&self) -> RXUCGFIS_R {
        RXUCGFIS_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - MMC Receive Length Error Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn rxlenerfis(&self) -> RXLENERFIS_R {
        RXLENERFIS_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - MMC Receive Out Of Range Error Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn rxorangefis(&self) -> RXORANGEFIS_R {
        RXORANGEFIS_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - MMC Receive Pause Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn rxpausfis(&self) -> RXPAUSFIS_R {
        RXPAUSFIS_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - MMC Receive FIFO Overflow Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn rxfovfis(&self) -> RXFOVFIS_R {
        RXFOVFIS_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - MMC Receive VLAN Good Bad Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn rxvlangbfis(&self) -> RXVLANGBFIS_R {
        RXVLANGBFIS_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - MMC Receive Watchdog Error Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn rxwdogfis(&self) -> RXWDOGFIS_R {
        RXWDOGFIS_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - MMC Receive Error Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn rxrcverrfis(&self) -> RXRCVERRFIS_R {
        RXRCVERRFIS_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - MMC Receive Control Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn rxctrlfis(&self) -> RXCTRLFIS_R {
        RXCTRLFIS_R::new(((self.bits >> 25) & 1) != 0)
    }
}
#[doc = "MMC Receive Interrupt Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmc_receive_interrupt::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MMC_RECEIVE_INTERRUPT_SPEC;
impl crate::RegisterSpec for MMC_RECEIVE_INTERRUPT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mmc_receive_interrupt::R`](R) reader structure"]
impl crate::Readable for MMC_RECEIVE_INTERRUPT_SPEC {}
#[doc = "`reset()` method sets MMC_RECEIVE_INTERRUPT to value 0"]
impl crate::Resettable for MMC_RECEIVE_INTERRUPT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
