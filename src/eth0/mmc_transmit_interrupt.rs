#[doc = "Reader of register MMC_TRANSMIT_INTERRUPT"]
pub type R = crate::R<u32, super::MMC_TRANSMIT_INTERRUPT>;
#[doc = "Reader of field `TXGBOCTIS`"]
pub type TXGBOCTIS_R = crate::R<bool, bool>;
#[doc = "Reader of field `TXGBFRMIS`"]
pub type TXGBFRMIS_R = crate::R<bool, bool>;
#[doc = "Reader of field `TXBCGFIS`"]
pub type TXBCGFIS_R = crate::R<bool, bool>;
#[doc = "Reader of field `TXMCGFIS`"]
pub type TXMCGFIS_R = crate::R<bool, bool>;
#[doc = "Reader of field `TX64OCTGBFIS`"]
pub type TX64OCTGBFIS_R = crate::R<bool, bool>;
#[doc = "Reader of field `TX65T127OCTGBFIS`"]
pub type TX65T127OCTGBFIS_R = crate::R<bool, bool>;
#[doc = "Reader of field `TX128T255OCTGBFIS`"]
pub type TX128T255OCTGBFIS_R = crate::R<bool, bool>;
#[doc = "Reader of field `TX256T511OCTGBFIS`"]
pub type TX256T511OCTGBFIS_R = crate::R<bool, bool>;
#[doc = "Reader of field `TX512T1023OCTGBFIS`"]
pub type TX512T1023OCTGBFIS_R = crate::R<bool, bool>;
#[doc = "Reader of field `TX1024TMAXOCTGBFIS`"]
pub type TX1024TMAXOCTGBFIS_R = crate::R<bool, bool>;
#[doc = "Reader of field `TXUCGBFIS`"]
pub type TXUCGBFIS_R = crate::R<bool, bool>;
#[doc = "Reader of field `TXMCGBFIS`"]
pub type TXMCGBFIS_R = crate::R<bool, bool>;
#[doc = "Reader of field `TXBCGBFIS`"]
pub type TXBCGBFIS_R = crate::R<bool, bool>;
#[doc = "Reader of field `TXUFLOWERFIS`"]
pub type TXUFLOWERFIS_R = crate::R<bool, bool>;
#[doc = "Reader of field `TXSCOLGFIS`"]
pub type TXSCOLGFIS_R = crate::R<bool, bool>;
#[doc = "Reader of field `TXMCOLGFIS`"]
pub type TXMCOLGFIS_R = crate::R<bool, bool>;
#[doc = "Reader of field `TXDEFFIS`"]
pub type TXDEFFIS_R = crate::R<bool, bool>;
#[doc = "Reader of field `TXLATCOLFIS`"]
pub type TXLATCOLFIS_R = crate::R<bool, bool>;
#[doc = "Reader of field `TXEXCOLFIS`"]
pub type TXEXCOLFIS_R = crate::R<bool, bool>;
#[doc = "Reader of field `TXCARERFIS`"]
pub type TXCARERFIS_R = crate::R<bool, bool>;
#[doc = "Reader of field `TXGOCTIS`"]
pub type TXGOCTIS_R = crate::R<bool, bool>;
#[doc = "Reader of field `TXGFRMIS`"]
pub type TXGFRMIS_R = crate::R<bool, bool>;
#[doc = "Reader of field `TXEXDEFFIS`"]
pub type TXEXDEFFIS_R = crate::R<bool, bool>;
#[doc = "Reader of field `TXPAUSFIS`"]
pub type TXPAUSFIS_R = crate::R<bool, bool>;
#[doc = "Reader of field `TXVLANGFIS`"]
pub type TXVLANGFIS_R = crate::R<bool, bool>;
#[doc = "Reader of field `TXOSIZEGFIS`"]
pub type TXOSIZEGFIS_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - MMC Transmit Good Bad Octet Counter Interrupt Status"]
    #[inline(always)]
    pub fn txgboctis(&self) -> TXGBOCTIS_R {
        TXGBOCTIS_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - MMC Transmit Good Bad Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn txgbfrmis(&self) -> TXGBFRMIS_R {
        TXGBFRMIS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - MMC Transmit Broadcast Good Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn txbcgfis(&self) -> TXBCGFIS_R {
        TXBCGFIS_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - MMC Transmit Multicast Good Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn txmcgfis(&self) -> TXMCGFIS_R {
        TXMCGFIS_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - MMC Transmit 64 Octet Good Bad Frame Counter Interrupt Status."]
    #[inline(always)]
    pub fn tx64octgbfis(&self) -> TX64OCTGBFIS_R {
        TX64OCTGBFIS_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - MMC Transmit 65 to 127 Octet Good Bad Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn tx65t127octgbfis(&self) -> TX65T127OCTGBFIS_R {
        TX65T127OCTGBFIS_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - MMC Transmit 128 to 255 Octet Good Bad Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn tx128t255octgbfis(&self) -> TX128T255OCTGBFIS_R {
        TX128T255OCTGBFIS_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - MMC Transmit 256 to 511 Octet Good Bad Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn tx256t511octgbfis(&self) -> TX256T511OCTGBFIS_R {
        TX256T511OCTGBFIS_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - MMC Transmit 512 to 1023 Octet Good Bad Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn tx512t1023octgbfis(&self) -> TX512T1023OCTGBFIS_R {
        TX512T1023OCTGBFIS_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - MMC Transmit 1024 to Maximum Octet Good Bad Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn tx1024tmaxoctgbfis(&self) -> TX1024TMAXOCTGBFIS_R {
        TX1024TMAXOCTGBFIS_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - MMC Transmit Unicast Good Bad Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn txucgbfis(&self) -> TXUCGBFIS_R {
        TXUCGBFIS_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - MMC Transmit Multicast Good Bad Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn txmcgbfis(&self) -> TXMCGBFIS_R {
        TXMCGBFIS_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - MMC Transmit Broadcast Good Bad Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn txbcgbfis(&self) -> TXBCGBFIS_R {
        TXBCGBFIS_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - MMC Transmit Underflow Error Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn txuflowerfis(&self) -> TXUFLOWERFIS_R {
        TXUFLOWERFIS_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - MMC Transmit Single Collision Good Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn txscolgfis(&self) -> TXSCOLGFIS_R {
        TXSCOLGFIS_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - MMC Transmit Multiple Collision Good Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn txmcolgfis(&self) -> TXMCOLGFIS_R {
        TXMCOLGFIS_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - MMC Transmit Deferred Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn txdeffis(&self) -> TXDEFFIS_R {
        TXDEFFIS_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - MMC Transmit Late Collision Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn txlatcolfis(&self) -> TXLATCOLFIS_R {
        TXLATCOLFIS_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - MMC Transmit Excessive Collision Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn txexcolfis(&self) -> TXEXCOLFIS_R {
        TXEXCOLFIS_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - MMC Transmit Carrier Error Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn txcarerfis(&self) -> TXCARERFIS_R {
        TXCARERFIS_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - MMC Transmit Good Octet Counter Interrupt Status"]
    #[inline(always)]
    pub fn txgoctis(&self) -> TXGOCTIS_R {
        TXGOCTIS_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - MMC Transmit Good Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn txgfrmis(&self) -> TXGFRMIS_R {
        TXGFRMIS_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - MMC Transmit Excessive Deferral Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn txexdeffis(&self) -> TXEXDEFFIS_R {
        TXEXDEFFIS_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - MMC Transmit Pause Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn txpausfis(&self) -> TXPAUSFIS_R {
        TXPAUSFIS_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - MMC Transmit VLAN Good Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn txvlangfis(&self) -> TXVLANGFIS_R {
        TXVLANGFIS_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - MMC Transmit Oversize Good Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn txosizegfis(&self) -> TXOSIZEGFIS_R {
        TXOSIZEGFIS_R::new(((self.bits >> 25) & 0x01) != 0)
    }
}
