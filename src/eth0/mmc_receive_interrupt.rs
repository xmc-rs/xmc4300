#[doc = "Reader of register MMC_RECEIVE_INTERRUPT"]
pub type R = crate::R<u32, super::MMC_RECEIVE_INTERRUPT>;
#[doc = "Reader of field `RXGBFRMIS`"]
pub type RXGBFRMIS_R = crate::R<bool, bool>;
#[doc = "Reader of field `RXGBOCTIS`"]
pub type RXGBOCTIS_R = crate::R<bool, bool>;
#[doc = "Reader of field `RXGOCTIS`"]
pub type RXGOCTIS_R = crate::R<bool, bool>;
#[doc = "Reader of field `RXBCGFIS`"]
pub type RXBCGFIS_R = crate::R<bool, bool>;
#[doc = "Reader of field `RXMCGFIS`"]
pub type RXMCGFIS_R = crate::R<bool, bool>;
#[doc = "Reader of field `RXCRCERFIS`"]
pub type RXCRCERFIS_R = crate::R<bool, bool>;
#[doc = "Reader of field `RXALGNERFIS`"]
pub type RXALGNERFIS_R = crate::R<bool, bool>;
#[doc = "Reader of field `RXRUNTFIS`"]
pub type RXRUNTFIS_R = crate::R<bool, bool>;
#[doc = "Reader of field `RXJABERFIS`"]
pub type RXJABERFIS_R = crate::R<bool, bool>;
#[doc = "Reader of field `RXUSIZEGFIS`"]
pub type RXUSIZEGFIS_R = crate::R<bool, bool>;
#[doc = "Reader of field `RXOSIZEGFIS`"]
pub type RXOSIZEGFIS_R = crate::R<bool, bool>;
#[doc = "Reader of field `RX64OCTGBFIS`"]
pub type RX64OCTGBFIS_R = crate::R<bool, bool>;
#[doc = "Reader of field `RX65T127OCTGBFIS`"]
pub type RX65T127OCTGBFIS_R = crate::R<bool, bool>;
#[doc = "Reader of field `RX128T255OCTGBFIS`"]
pub type RX128T255OCTGBFIS_R = crate::R<bool, bool>;
#[doc = "Reader of field `RX256T511OCTGBFIS`"]
pub type RX256T511OCTGBFIS_R = crate::R<bool, bool>;
#[doc = "Reader of field `RX512T1023OCTGBFIS`"]
pub type RX512T1023OCTGBFIS_R = crate::R<bool, bool>;
#[doc = "Reader of field `RX1024TMAXOCTGBFIS`"]
pub type RX1024TMAXOCTGBFIS_R = crate::R<bool, bool>;
#[doc = "Reader of field `RXUCGFIS`"]
pub type RXUCGFIS_R = crate::R<bool, bool>;
#[doc = "Reader of field `RXLENERFIS`"]
pub type RXLENERFIS_R = crate::R<bool, bool>;
#[doc = "Reader of field `RXORANGEFIS`"]
pub type RXORANGEFIS_R = crate::R<bool, bool>;
#[doc = "Reader of field `RXPAUSFIS`"]
pub type RXPAUSFIS_R = crate::R<bool, bool>;
#[doc = "Reader of field `RXFOVFIS`"]
pub type RXFOVFIS_R = crate::R<bool, bool>;
#[doc = "Reader of field `RXVLANGBFIS`"]
pub type RXVLANGBFIS_R = crate::R<bool, bool>;
#[doc = "Reader of field `RXWDOGFIS`"]
pub type RXWDOGFIS_R = crate::R<bool, bool>;
#[doc = "Reader of field `RXRCVERRFIS`"]
pub type RXRCVERRFIS_R = crate::R<bool, bool>;
#[doc = "Reader of field `RXCTRLFIS`"]
pub type RXCTRLFIS_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - MMC Receive Good Bad Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn rxgbfrmis(&self) -> RXGBFRMIS_R {
        RXGBFRMIS_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - MMC Receive Good Bad Octet Counter Interrupt Status"]
    #[inline(always)]
    pub fn rxgboctis(&self) -> RXGBOCTIS_R {
        RXGBOCTIS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - MMC Receive Good Octet Counter Interrupt Status."]
    #[inline(always)]
    pub fn rxgoctis(&self) -> RXGOCTIS_R {
        RXGOCTIS_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - MMC Receive Broadcast Good Frame Counter Interrupt Status."]
    #[inline(always)]
    pub fn rxbcgfis(&self) -> RXBCGFIS_R {
        RXBCGFIS_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - MMC Receive Multicast Good Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn rxmcgfis(&self) -> RXMCGFIS_R {
        RXMCGFIS_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - MMC Receive CRC Error Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn rxcrcerfis(&self) -> RXCRCERFIS_R {
        RXCRCERFIS_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - MMC Receive Alignment Error Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn rxalgnerfis(&self) -> RXALGNERFIS_R {
        RXALGNERFIS_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - MMC Receive Runt Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn rxruntfis(&self) -> RXRUNTFIS_R {
        RXRUNTFIS_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - MMC Receive Jabber Error Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn rxjaberfis(&self) -> RXJABERFIS_R {
        RXJABERFIS_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - MMC Receive Undersize Good Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn rxusizegfis(&self) -> RXUSIZEGFIS_R {
        RXUSIZEGFIS_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - MMC Receive Oversize Good Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn rxosizegfis(&self) -> RXOSIZEGFIS_R {
        RXOSIZEGFIS_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - MMC Receive 64 Octet Good Bad Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn rx64octgbfis(&self) -> RX64OCTGBFIS_R {
        RX64OCTGBFIS_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - MMC Receive 65 to 127 Octet Good Bad Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn rx65t127octgbfis(&self) -> RX65T127OCTGBFIS_R {
        RX65T127OCTGBFIS_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - MMC Receive 128 to 255 Octet Good Bad Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn rx128t255octgbfis(&self) -> RX128T255OCTGBFIS_R {
        RX128T255OCTGBFIS_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - MMC Receive 256 to 511 Octet Good Bad Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn rx256t511octgbfis(&self) -> RX256T511OCTGBFIS_R {
        RX256T511OCTGBFIS_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - MMC Receive 512 to 1023 Octet Good Bad Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn rx512t1023octgbfis(&self) -> RX512T1023OCTGBFIS_R {
        RX512T1023OCTGBFIS_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - MMC Receive 1024 to Maximum Octet Good Bad Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn rx1024tmaxoctgbfis(&self) -> RX1024TMAXOCTGBFIS_R {
        RX1024TMAXOCTGBFIS_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - MMC Receive Unicast Good Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn rxucgfis(&self) -> RXUCGFIS_R {
        RXUCGFIS_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - MMC Receive Length Error Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn rxlenerfis(&self) -> RXLENERFIS_R {
        RXLENERFIS_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - MMC Receive Out Of Range Error Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn rxorangefis(&self) -> RXORANGEFIS_R {
        RXORANGEFIS_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - MMC Receive Pause Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn rxpausfis(&self) -> RXPAUSFIS_R {
        RXPAUSFIS_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - MMC Receive FIFO Overflow Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn rxfovfis(&self) -> RXFOVFIS_R {
        RXFOVFIS_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - MMC Receive VLAN Good Bad Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn rxvlangbfis(&self) -> RXVLANGBFIS_R {
        RXVLANGBFIS_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - MMC Receive Watchdog Error Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn rxwdogfis(&self) -> RXWDOGFIS_R {
        RXWDOGFIS_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - MMC Receive Error Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn rxrcverrfis(&self) -> RXRCVERRFIS_R {
        RXRCVERRFIS_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - MMC Receive Control Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn rxctrlfis(&self) -> RXCTRLFIS_R {
        RXCTRLFIS_R::new(((self.bits >> 25) & 0x01) != 0)
    }
}
