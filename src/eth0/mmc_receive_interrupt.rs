#[doc = "Register `MMC_RECEIVE_INTERRUPT` reader"]
pub type R = crate::R<MmcReceiveInterruptSpec>;
#[doc = "Field `RXGBFRMIS` reader - MMC Receive Good Bad Frame Counter Interrupt Status"]
pub type RxgbfrmisR = crate::BitReader;
#[doc = "Field `RXGBOCTIS` reader - MMC Receive Good Bad Octet Counter Interrupt Status"]
pub type RxgboctisR = crate::BitReader;
#[doc = "Field `RXGOCTIS` reader - MMC Receive Good Octet Counter Interrupt Status."]
pub type RxgoctisR = crate::BitReader;
#[doc = "Field `RXBCGFIS` reader - MMC Receive Broadcast Good Frame Counter Interrupt Status."]
pub type RxbcgfisR = crate::BitReader;
#[doc = "Field `RXMCGFIS` reader - MMC Receive Multicast Good Frame Counter Interrupt Status"]
pub type RxmcgfisR = crate::BitReader;
#[doc = "Field `RXCRCERFIS` reader - MMC Receive CRC Error Frame Counter Interrupt Status"]
pub type RxcrcerfisR = crate::BitReader;
#[doc = "Field `RXALGNERFIS` reader - MMC Receive Alignment Error Frame Counter Interrupt Status"]
pub type RxalgnerfisR = crate::BitReader;
#[doc = "Field `RXRUNTFIS` reader - MMC Receive Runt Frame Counter Interrupt Status"]
pub type RxruntfisR = crate::BitReader;
#[doc = "Field `RXJABERFIS` reader - MMC Receive Jabber Error Frame Counter Interrupt Status"]
pub type RxjaberfisR = crate::BitReader;
#[doc = "Field `RXUSIZEGFIS` reader - MMC Receive Undersize Good Frame Counter Interrupt Status"]
pub type RxusizegfisR = crate::BitReader;
#[doc = "Field `RXOSIZEGFIS` reader - MMC Receive Oversize Good Frame Counter Interrupt Status"]
pub type RxosizegfisR = crate::BitReader;
#[doc = "Field `RX64OCTGBFIS` reader - MMC Receive 64 Octet Good Bad Frame Counter Interrupt Status"]
pub type Rx64octgbfisR = crate::BitReader;
#[doc = "Field `RX65T127OCTGBFIS` reader - MMC Receive 65 to 127 Octet Good Bad Frame Counter Interrupt Status"]
pub type Rx65t127octgbfisR = crate::BitReader;
#[doc = "Field `RX128T255OCTGBFIS` reader - MMC Receive 128 to 255 Octet Good Bad Frame Counter Interrupt Status"]
pub type Rx128t255octgbfisR = crate::BitReader;
#[doc = "Field `RX256T511OCTGBFIS` reader - MMC Receive 256 to 511 Octet Good Bad Frame Counter Interrupt Status"]
pub type Rx256t511octgbfisR = crate::BitReader;
#[doc = "Field `RX512T1023OCTGBFIS` reader - MMC Receive 512 to 1023 Octet Good Bad Frame Counter Interrupt Status"]
pub type Rx512t1023octgbfisR = crate::BitReader;
#[doc = "Field `RX1024TMAXOCTGBFIS` reader - MMC Receive 1024 to Maximum Octet Good Bad Frame Counter Interrupt Status"]
pub type Rx1024tmaxoctgbfisR = crate::BitReader;
#[doc = "Field `RXUCGFIS` reader - MMC Receive Unicast Good Frame Counter Interrupt Status"]
pub type RxucgfisR = crate::BitReader;
#[doc = "Field `RXLENERFIS` reader - MMC Receive Length Error Frame Counter Interrupt Status"]
pub type RxlenerfisR = crate::BitReader;
#[doc = "Field `RXORANGEFIS` reader - MMC Receive Out Of Range Error Frame Counter Interrupt Status"]
pub type RxorangefisR = crate::BitReader;
#[doc = "Field `RXPAUSFIS` reader - MMC Receive Pause Frame Counter Interrupt Status"]
pub type RxpausfisR = crate::BitReader;
#[doc = "Field `RXFOVFIS` reader - MMC Receive FIFO Overflow Frame Counter Interrupt Status"]
pub type RxfovfisR = crate::BitReader;
#[doc = "Field `RXVLANGBFIS` reader - MMC Receive VLAN Good Bad Frame Counter Interrupt Status"]
pub type RxvlangbfisR = crate::BitReader;
#[doc = "Field `RXWDOGFIS` reader - MMC Receive Watchdog Error Frame Counter Interrupt Status"]
pub type RxwdogfisR = crate::BitReader;
#[doc = "Field `RXRCVERRFIS` reader - MMC Receive Error Frame Counter Interrupt Status"]
pub type RxrcverrfisR = crate::BitReader;
#[doc = "Field `RXCTRLFIS` reader - MMC Receive Control Frame Counter Interrupt Status"]
pub type RxctrlfisR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - MMC Receive Good Bad Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn rxgbfrmis(&self) -> RxgbfrmisR {
        RxgbfrmisR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - MMC Receive Good Bad Octet Counter Interrupt Status"]
    #[inline(always)]
    pub fn rxgboctis(&self) -> RxgboctisR {
        RxgboctisR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - MMC Receive Good Octet Counter Interrupt Status."]
    #[inline(always)]
    pub fn rxgoctis(&self) -> RxgoctisR {
        RxgoctisR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - MMC Receive Broadcast Good Frame Counter Interrupt Status."]
    #[inline(always)]
    pub fn rxbcgfis(&self) -> RxbcgfisR {
        RxbcgfisR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - MMC Receive Multicast Good Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn rxmcgfis(&self) -> RxmcgfisR {
        RxmcgfisR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - MMC Receive CRC Error Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn rxcrcerfis(&self) -> RxcrcerfisR {
        RxcrcerfisR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - MMC Receive Alignment Error Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn rxalgnerfis(&self) -> RxalgnerfisR {
        RxalgnerfisR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - MMC Receive Runt Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn rxruntfis(&self) -> RxruntfisR {
        RxruntfisR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - MMC Receive Jabber Error Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn rxjaberfis(&self) -> RxjaberfisR {
        RxjaberfisR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - MMC Receive Undersize Good Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn rxusizegfis(&self) -> RxusizegfisR {
        RxusizegfisR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - MMC Receive Oversize Good Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn rxosizegfis(&self) -> RxosizegfisR {
        RxosizegfisR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - MMC Receive 64 Octet Good Bad Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn rx64octgbfis(&self) -> Rx64octgbfisR {
        Rx64octgbfisR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - MMC Receive 65 to 127 Octet Good Bad Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn rx65t127octgbfis(&self) -> Rx65t127octgbfisR {
        Rx65t127octgbfisR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - MMC Receive 128 to 255 Octet Good Bad Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn rx128t255octgbfis(&self) -> Rx128t255octgbfisR {
        Rx128t255octgbfisR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - MMC Receive 256 to 511 Octet Good Bad Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn rx256t511octgbfis(&self) -> Rx256t511octgbfisR {
        Rx256t511octgbfisR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - MMC Receive 512 to 1023 Octet Good Bad Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn rx512t1023octgbfis(&self) -> Rx512t1023octgbfisR {
        Rx512t1023octgbfisR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - MMC Receive 1024 to Maximum Octet Good Bad Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn rx1024tmaxoctgbfis(&self) -> Rx1024tmaxoctgbfisR {
        Rx1024tmaxoctgbfisR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - MMC Receive Unicast Good Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn rxucgfis(&self) -> RxucgfisR {
        RxucgfisR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - MMC Receive Length Error Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn rxlenerfis(&self) -> RxlenerfisR {
        RxlenerfisR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - MMC Receive Out Of Range Error Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn rxorangefis(&self) -> RxorangefisR {
        RxorangefisR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - MMC Receive Pause Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn rxpausfis(&self) -> RxpausfisR {
        RxpausfisR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - MMC Receive FIFO Overflow Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn rxfovfis(&self) -> RxfovfisR {
        RxfovfisR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - MMC Receive VLAN Good Bad Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn rxvlangbfis(&self) -> RxvlangbfisR {
        RxvlangbfisR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - MMC Receive Watchdog Error Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn rxwdogfis(&self) -> RxwdogfisR {
        RxwdogfisR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - MMC Receive Error Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn rxrcverrfis(&self) -> RxrcverrfisR {
        RxrcverrfisR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - MMC Receive Control Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn rxctrlfis(&self) -> RxctrlfisR {
        RxctrlfisR::new(((self.bits >> 25) & 1) != 0)
    }
}
#[doc = "MMC Receive Interrupt Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmc_receive_interrupt::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MmcReceiveInterruptSpec;
impl crate::RegisterSpec for MmcReceiveInterruptSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mmc_receive_interrupt::R`](R) reader structure"]
impl crate::Readable for MmcReceiveInterruptSpec {}
#[doc = "`reset()` method sets MMC_RECEIVE_INTERRUPT to value 0"]
impl crate::Resettable for MmcReceiveInterruptSpec {
    const RESET_VALUE: u32 = 0;
}
