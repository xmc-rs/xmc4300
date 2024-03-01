#[doc = "Register `MMC_TRANSMIT_INTERRUPT` reader"]
pub type R = crate::R<MmcTransmitInterruptSpec>;
#[doc = "Field `TXGBOCTIS` reader - MMC Transmit Good Bad Octet Counter Interrupt Status"]
pub type TxgboctisR = crate::BitReader;
#[doc = "Field `TXGBFRMIS` reader - MMC Transmit Good Bad Frame Counter Interrupt Status"]
pub type TxgbfrmisR = crate::BitReader;
#[doc = "Field `TXBCGFIS` reader - MMC Transmit Broadcast Good Frame Counter Interrupt Status"]
pub type TxbcgfisR = crate::BitReader;
#[doc = "Field `TXMCGFIS` reader - MMC Transmit Multicast Good Frame Counter Interrupt Status"]
pub type TxmcgfisR = crate::BitReader;
#[doc = "Field `TX64OCTGBFIS` reader - MMC Transmit 64 Octet Good Bad Frame Counter Interrupt Status."]
pub type Tx64octgbfisR = crate::BitReader;
#[doc = "Field `TX65T127OCTGBFIS` reader - MMC Transmit 65 to 127 Octet Good Bad Frame Counter Interrupt Status"]
pub type Tx65t127octgbfisR = crate::BitReader;
#[doc = "Field `TX128T255OCTGBFIS` reader - MMC Transmit 128 to 255 Octet Good Bad Frame Counter Interrupt Status"]
pub type Tx128t255octgbfisR = crate::BitReader;
#[doc = "Field `TX256T511OCTGBFIS` reader - MMC Transmit 256 to 511 Octet Good Bad Frame Counter Interrupt Status"]
pub type Tx256t511octgbfisR = crate::BitReader;
#[doc = "Field `TX512T1023OCTGBFIS` reader - MMC Transmit 512 to 1023 Octet Good Bad Frame Counter Interrupt Status"]
pub type Tx512t1023octgbfisR = crate::BitReader;
#[doc = "Field `TX1024TMAXOCTGBFIS` reader - MMC Transmit 1024 to Maximum Octet Good Bad Frame Counter Interrupt Status"]
pub type Tx1024tmaxoctgbfisR = crate::BitReader;
#[doc = "Field `TXUCGBFIS` reader - MMC Transmit Unicast Good Bad Frame Counter Interrupt Status"]
pub type TxucgbfisR = crate::BitReader;
#[doc = "Field `TXMCGBFIS` reader - MMC Transmit Multicast Good Bad Frame Counter Interrupt Status"]
pub type TxmcgbfisR = crate::BitReader;
#[doc = "Field `TXBCGBFIS` reader - MMC Transmit Broadcast Good Bad Frame Counter Interrupt Status"]
pub type TxbcgbfisR = crate::BitReader;
#[doc = "Field `TXUFLOWERFIS` reader - MMC Transmit Underflow Error Frame Counter Interrupt Status"]
pub type TxuflowerfisR = crate::BitReader;
#[doc = "Field `TXSCOLGFIS` reader - MMC Transmit Single Collision Good Frame Counter Interrupt Status"]
pub type TxscolgfisR = crate::BitReader;
#[doc = "Field `TXMCOLGFIS` reader - MMC Transmit Multiple Collision Good Frame Counter Interrupt Status"]
pub type TxmcolgfisR = crate::BitReader;
#[doc = "Field `TXDEFFIS` reader - MMC Transmit Deferred Frame Counter Interrupt Status"]
pub type TxdeffisR = crate::BitReader;
#[doc = "Field `TXLATCOLFIS` reader - MMC Transmit Late Collision Frame Counter Interrupt Status"]
pub type TxlatcolfisR = crate::BitReader;
#[doc = "Field `TXEXCOLFIS` reader - MMC Transmit Excessive Collision Frame Counter Interrupt Status"]
pub type TxexcolfisR = crate::BitReader;
#[doc = "Field `TXCARERFIS` reader - MMC Transmit Carrier Error Frame Counter Interrupt Status"]
pub type TxcarerfisR = crate::BitReader;
#[doc = "Field `TXGOCTIS` reader - MMC Transmit Good Octet Counter Interrupt Status"]
pub type TxgoctisR = crate::BitReader;
#[doc = "Field `TXGFRMIS` reader - MMC Transmit Good Frame Counter Interrupt Status"]
pub type TxgfrmisR = crate::BitReader;
#[doc = "Field `TXEXDEFFIS` reader - MMC Transmit Excessive Deferral Frame Counter Interrupt Status"]
pub type TxexdeffisR = crate::BitReader;
#[doc = "Field `TXPAUSFIS` reader - MMC Transmit Pause Frame Counter Interrupt Status"]
pub type TxpausfisR = crate::BitReader;
#[doc = "Field `TXVLANGFIS` reader - MMC Transmit VLAN Good Frame Counter Interrupt Status"]
pub type TxvlangfisR = crate::BitReader;
#[doc = "Field `TXOSIZEGFIS` reader - MMC Transmit Oversize Good Frame Counter Interrupt Status"]
pub type TxosizegfisR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - MMC Transmit Good Bad Octet Counter Interrupt Status"]
    #[inline(always)]
    pub fn txgboctis(&self) -> TxgboctisR {
        TxgboctisR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - MMC Transmit Good Bad Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn txgbfrmis(&self) -> TxgbfrmisR {
        TxgbfrmisR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - MMC Transmit Broadcast Good Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn txbcgfis(&self) -> TxbcgfisR {
        TxbcgfisR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - MMC Transmit Multicast Good Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn txmcgfis(&self) -> TxmcgfisR {
        TxmcgfisR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - MMC Transmit 64 Octet Good Bad Frame Counter Interrupt Status."]
    #[inline(always)]
    pub fn tx64octgbfis(&self) -> Tx64octgbfisR {
        Tx64octgbfisR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - MMC Transmit 65 to 127 Octet Good Bad Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn tx65t127octgbfis(&self) -> Tx65t127octgbfisR {
        Tx65t127octgbfisR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - MMC Transmit 128 to 255 Octet Good Bad Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn tx128t255octgbfis(&self) -> Tx128t255octgbfisR {
        Tx128t255octgbfisR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - MMC Transmit 256 to 511 Octet Good Bad Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn tx256t511octgbfis(&self) -> Tx256t511octgbfisR {
        Tx256t511octgbfisR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - MMC Transmit 512 to 1023 Octet Good Bad Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn tx512t1023octgbfis(&self) -> Tx512t1023octgbfisR {
        Tx512t1023octgbfisR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - MMC Transmit 1024 to Maximum Octet Good Bad Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn tx1024tmaxoctgbfis(&self) -> Tx1024tmaxoctgbfisR {
        Tx1024tmaxoctgbfisR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - MMC Transmit Unicast Good Bad Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn txucgbfis(&self) -> TxucgbfisR {
        TxucgbfisR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - MMC Transmit Multicast Good Bad Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn txmcgbfis(&self) -> TxmcgbfisR {
        TxmcgbfisR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - MMC Transmit Broadcast Good Bad Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn txbcgbfis(&self) -> TxbcgbfisR {
        TxbcgbfisR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - MMC Transmit Underflow Error Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn txuflowerfis(&self) -> TxuflowerfisR {
        TxuflowerfisR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - MMC Transmit Single Collision Good Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn txscolgfis(&self) -> TxscolgfisR {
        TxscolgfisR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - MMC Transmit Multiple Collision Good Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn txmcolgfis(&self) -> TxmcolgfisR {
        TxmcolgfisR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - MMC Transmit Deferred Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn txdeffis(&self) -> TxdeffisR {
        TxdeffisR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - MMC Transmit Late Collision Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn txlatcolfis(&self) -> TxlatcolfisR {
        TxlatcolfisR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - MMC Transmit Excessive Collision Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn txexcolfis(&self) -> TxexcolfisR {
        TxexcolfisR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - MMC Transmit Carrier Error Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn txcarerfis(&self) -> TxcarerfisR {
        TxcarerfisR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - MMC Transmit Good Octet Counter Interrupt Status"]
    #[inline(always)]
    pub fn txgoctis(&self) -> TxgoctisR {
        TxgoctisR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - MMC Transmit Good Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn txgfrmis(&self) -> TxgfrmisR {
        TxgfrmisR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - MMC Transmit Excessive Deferral Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn txexdeffis(&self) -> TxexdeffisR {
        TxexdeffisR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - MMC Transmit Pause Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn txpausfis(&self) -> TxpausfisR {
        TxpausfisR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - MMC Transmit VLAN Good Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn txvlangfis(&self) -> TxvlangfisR {
        TxvlangfisR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - MMC Transmit Oversize Good Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn txosizegfis(&self) -> TxosizegfisR {
        TxosizegfisR::new(((self.bits >> 25) & 1) != 0)
    }
}
#[doc = "MMC Transmit Interrupt Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmc_transmit_interrupt::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MmcTransmitInterruptSpec;
impl crate::RegisterSpec for MmcTransmitInterruptSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mmc_transmit_interrupt::R`](R) reader structure"]
impl crate::Readable for MmcTransmitInterruptSpec {}
#[doc = "`reset()` method sets MMC_TRANSMIT_INTERRUPT to value 0"]
impl crate::Resettable for MmcTransmitInterruptSpec {
    const RESET_VALUE: u32 = 0;
}
