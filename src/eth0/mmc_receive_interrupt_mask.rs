#[doc = "Register `MMC_RECEIVE_INTERRUPT_MASK` reader"]
pub type R = crate::R<MMC_RECEIVE_INTERRUPT_MASK_SPEC>;
#[doc = "Register `MMC_RECEIVE_INTERRUPT_MASK` writer"]
pub type W = crate::W<MMC_RECEIVE_INTERRUPT_MASK_SPEC>;
#[doc = "Field `RXGBFRMIM` reader - MMC Receive Good Bad Frame Counter Interrupt Mask"]
pub type RXGBFRMIM_R = crate::BitReader;
#[doc = "Field `RXGBFRMIM` writer - MMC Receive Good Bad Frame Counter Interrupt Mask"]
pub type RXGBFRMIM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXGBOCTIM` reader - MMC Receive Good Bad Octet Counter Interrupt Mask"]
pub type RXGBOCTIM_R = crate::BitReader;
#[doc = "Field `RXGBOCTIM` writer - MMC Receive Good Bad Octet Counter Interrupt Mask"]
pub type RXGBOCTIM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXGOCTIM` reader - MMC Receive Good Octet Counter Interrupt Mask"]
pub type RXGOCTIM_R = crate::BitReader;
#[doc = "Field `RXGOCTIM` writer - MMC Receive Good Octet Counter Interrupt Mask"]
pub type RXGOCTIM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXBCGFIM` reader - MMC Receive Broadcast Good Frame Counter Interrupt Mask"]
pub type RXBCGFIM_R = crate::BitReader;
#[doc = "Field `RXBCGFIM` writer - MMC Receive Broadcast Good Frame Counter Interrupt Mask"]
pub type RXBCGFIM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXMCGFIM` reader - MMC Receive Multicast Good Frame Counter Interrupt Mask"]
pub type RXMCGFIM_R = crate::BitReader;
#[doc = "Field `RXMCGFIM` writer - MMC Receive Multicast Good Frame Counter Interrupt Mask"]
pub type RXMCGFIM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXCRCERFIM` reader - MMC Receive CRC Error Frame Counter Interrupt Mask"]
pub type RXCRCERFIM_R = crate::BitReader;
#[doc = "Field `RXCRCERFIM` writer - MMC Receive CRC Error Frame Counter Interrupt Mask"]
pub type RXCRCERFIM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXALGNERFIM` reader - MMC Receive Alignment Error Frame Counter Interrupt Mask"]
pub type RXALGNERFIM_R = crate::BitReader;
#[doc = "Field `RXALGNERFIM` writer - MMC Receive Alignment Error Frame Counter Interrupt Mask"]
pub type RXALGNERFIM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXRUNTFIM` reader - MMC Receive Runt Frame Counter Interrupt Mask"]
pub type RXRUNTFIM_R = crate::BitReader;
#[doc = "Field `RXRUNTFIM` writer - MMC Receive Runt Frame Counter Interrupt Mask"]
pub type RXRUNTFIM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXJABERFIM` reader - MMC Receive Jabber Error Frame Counter Interrupt Mask"]
pub type RXJABERFIM_R = crate::BitReader;
#[doc = "Field `RXJABERFIM` writer - MMC Receive Jabber Error Frame Counter Interrupt Mask"]
pub type RXJABERFIM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXUSIZEGFIM` reader - MMC Receive Undersize Good Frame Counter Interrupt Mask"]
pub type RXUSIZEGFIM_R = crate::BitReader;
#[doc = "Field `RXUSIZEGFIM` writer - MMC Receive Undersize Good Frame Counter Interrupt Mask"]
pub type RXUSIZEGFIM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXOSIZEGFIM` reader - MMC Receive Oversize Good Frame Counter Interrupt Mask"]
pub type RXOSIZEGFIM_R = crate::BitReader;
#[doc = "Field `RXOSIZEGFIM` writer - MMC Receive Oversize Good Frame Counter Interrupt Mask"]
pub type RXOSIZEGFIM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX64OCTGBFIM` reader - MMC Receive 64 Octet Good Bad Frame Counter Interrupt Mask"]
pub type RX64OCTGBFIM_R = crate::BitReader;
#[doc = "Field `RX64OCTGBFIM` writer - MMC Receive 64 Octet Good Bad Frame Counter Interrupt Mask"]
pub type RX64OCTGBFIM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX65T127OCTGBFIM` reader - MMC Receive 65 to 127 Octet Good Bad Frame Counter Interrupt Mask"]
pub type RX65T127OCTGBFIM_R = crate::BitReader;
#[doc = "Field `RX65T127OCTGBFIM` writer - MMC Receive 65 to 127 Octet Good Bad Frame Counter Interrupt Mask"]
pub type RX65T127OCTGBFIM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX128T255OCTGBFIM` reader - MMC Receive 128 to 255 Octet Good Bad Frame Counter Interrupt Mask"]
pub type RX128T255OCTGBFIM_R = crate::BitReader;
#[doc = "Field `RX128T255OCTGBFIM` writer - MMC Receive 128 to 255 Octet Good Bad Frame Counter Interrupt Mask"]
pub type RX128T255OCTGBFIM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX256T511OCTGBFIM` reader - MMC Receive 256 to 511 Octet Good Bad Frame Counter Interrupt Mask"]
pub type RX256T511OCTGBFIM_R = crate::BitReader;
#[doc = "Field `RX256T511OCTGBFIM` writer - MMC Receive 256 to 511 Octet Good Bad Frame Counter Interrupt Mask"]
pub type RX256T511OCTGBFIM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX512T1023OCTGBFIM` reader - MMC Receive 512 to 1023 Octet Good Bad Frame Counter Interrupt Mask"]
pub type RX512T1023OCTGBFIM_R = crate::BitReader;
#[doc = "Field `RX512T1023OCTGBFIM` writer - MMC Receive 512 to 1023 Octet Good Bad Frame Counter Interrupt Mask"]
pub type RX512T1023OCTGBFIM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX1024TMAXOCTGBFIM` reader - MMC Receive 1024 to Maximum Octet Good Bad Frame Counter Interrupt Mask"]
pub type RX1024TMAXOCTGBFIM_R = crate::BitReader;
#[doc = "Field `RX1024TMAXOCTGBFIM` writer - MMC Receive 1024 to Maximum Octet Good Bad Frame Counter Interrupt Mask"]
pub type RX1024TMAXOCTGBFIM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXUCGFIM` reader - MMC Receive Unicast Good Frame Counter Interrupt Mask"]
pub type RXUCGFIM_R = crate::BitReader;
#[doc = "Field `RXUCGFIM` writer - MMC Receive Unicast Good Frame Counter Interrupt Mask"]
pub type RXUCGFIM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXLENERFIM` reader - MMC Receive Length Error Frame Counter Interrupt Mask"]
pub type RXLENERFIM_R = crate::BitReader;
#[doc = "Field `RXLENERFIM` writer - MMC Receive Length Error Frame Counter Interrupt Mask"]
pub type RXLENERFIM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXORANGEFIM` reader - MMC Receive Out Of Range Error Frame Counter Interrupt Mask"]
pub type RXORANGEFIM_R = crate::BitReader;
#[doc = "Field `RXORANGEFIM` writer - MMC Receive Out Of Range Error Frame Counter Interrupt Mask"]
pub type RXORANGEFIM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXPAUSFIM` reader - MMC Receive Pause Frame Counter Interrupt Mask"]
pub type RXPAUSFIM_R = crate::BitReader;
#[doc = "Field `RXPAUSFIM` writer - MMC Receive Pause Frame Counter Interrupt Mask"]
pub type RXPAUSFIM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXFOVFIM` reader - MMC Receive FIFO Overflow Frame Counter Interrupt Mask"]
pub type RXFOVFIM_R = crate::BitReader;
#[doc = "Field `RXFOVFIM` writer - MMC Receive FIFO Overflow Frame Counter Interrupt Mask"]
pub type RXFOVFIM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXVLANGBFIM` reader - MMC Receive VLAN Good Bad Frame Counter Interrupt Mask"]
pub type RXVLANGBFIM_R = crate::BitReader;
#[doc = "Field `RXVLANGBFIM` writer - MMC Receive VLAN Good Bad Frame Counter Interrupt Mask"]
pub type RXVLANGBFIM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXWDOGFIM` reader - MMC Receive Watchdog Error Frame Counter Interrupt Mask"]
pub type RXWDOGFIM_R = crate::BitReader;
#[doc = "Field `RXWDOGFIM` writer - MMC Receive Watchdog Error Frame Counter Interrupt Mask"]
pub type RXWDOGFIM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXRCVERRFIM` reader - MMC Receive Error Frame Counter Interrupt Mask"]
pub type RXRCVERRFIM_R = crate::BitReader;
#[doc = "Field `RXRCVERRFIM` writer - MMC Receive Error Frame Counter Interrupt Mask"]
pub type RXRCVERRFIM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXCTRLFIM` reader - MMC Receive Control Frame Counter Interrupt Mask"]
pub type RXCTRLFIM_R = crate::BitReader;
#[doc = "Field `RXCTRLFIM` writer - MMC Receive Control Frame Counter Interrupt Mask"]
pub type RXCTRLFIM_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - MMC Receive Good Bad Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxgbfrmim(&self) -> RXGBFRMIM_R {
        RXGBFRMIM_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - MMC Receive Good Bad Octet Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxgboctim(&self) -> RXGBOCTIM_R {
        RXGBOCTIM_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - MMC Receive Good Octet Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxgoctim(&self) -> RXGOCTIM_R {
        RXGOCTIM_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - MMC Receive Broadcast Good Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxbcgfim(&self) -> RXBCGFIM_R {
        RXBCGFIM_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - MMC Receive Multicast Good Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxmcgfim(&self) -> RXMCGFIM_R {
        RXMCGFIM_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - MMC Receive CRC Error Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxcrcerfim(&self) -> RXCRCERFIM_R {
        RXCRCERFIM_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - MMC Receive Alignment Error Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxalgnerfim(&self) -> RXALGNERFIM_R {
        RXALGNERFIM_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - MMC Receive Runt Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxruntfim(&self) -> RXRUNTFIM_R {
        RXRUNTFIM_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - MMC Receive Jabber Error Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxjaberfim(&self) -> RXJABERFIM_R {
        RXJABERFIM_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - MMC Receive Undersize Good Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxusizegfim(&self) -> RXUSIZEGFIM_R {
        RXUSIZEGFIM_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - MMC Receive Oversize Good Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxosizegfim(&self) -> RXOSIZEGFIM_R {
        RXOSIZEGFIM_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - MMC Receive 64 Octet Good Bad Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rx64octgbfim(&self) -> RX64OCTGBFIM_R {
        RX64OCTGBFIM_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - MMC Receive 65 to 127 Octet Good Bad Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rx65t127octgbfim(&self) -> RX65T127OCTGBFIM_R {
        RX65T127OCTGBFIM_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - MMC Receive 128 to 255 Octet Good Bad Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rx128t255octgbfim(&self) -> RX128T255OCTGBFIM_R {
        RX128T255OCTGBFIM_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - MMC Receive 256 to 511 Octet Good Bad Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rx256t511octgbfim(&self) -> RX256T511OCTGBFIM_R {
        RX256T511OCTGBFIM_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - MMC Receive 512 to 1023 Octet Good Bad Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rx512t1023octgbfim(&self) -> RX512T1023OCTGBFIM_R {
        RX512T1023OCTGBFIM_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - MMC Receive 1024 to Maximum Octet Good Bad Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rx1024tmaxoctgbfim(&self) -> RX1024TMAXOCTGBFIM_R {
        RX1024TMAXOCTGBFIM_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - MMC Receive Unicast Good Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxucgfim(&self) -> RXUCGFIM_R {
        RXUCGFIM_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - MMC Receive Length Error Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxlenerfim(&self) -> RXLENERFIM_R {
        RXLENERFIM_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - MMC Receive Out Of Range Error Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxorangefim(&self) -> RXORANGEFIM_R {
        RXORANGEFIM_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - MMC Receive Pause Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxpausfim(&self) -> RXPAUSFIM_R {
        RXPAUSFIM_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - MMC Receive FIFO Overflow Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxfovfim(&self) -> RXFOVFIM_R {
        RXFOVFIM_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - MMC Receive VLAN Good Bad Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxvlangbfim(&self) -> RXVLANGBFIM_R {
        RXVLANGBFIM_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - MMC Receive Watchdog Error Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxwdogfim(&self) -> RXWDOGFIM_R {
        RXWDOGFIM_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - MMC Receive Error Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxrcverrfim(&self) -> RXRCVERRFIM_R {
        RXRCVERRFIM_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - MMC Receive Control Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxctrlfim(&self) -> RXCTRLFIM_R {
        RXCTRLFIM_R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - MMC Receive Good Bad Frame Counter Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn rxgbfrmim(&mut self) -> RXGBFRMIM_W<MMC_RECEIVE_INTERRUPT_MASK_SPEC> {
        RXGBFRMIM_W::new(self, 0)
    }
    #[doc = "Bit 1 - MMC Receive Good Bad Octet Counter Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn rxgboctim(&mut self) -> RXGBOCTIM_W<MMC_RECEIVE_INTERRUPT_MASK_SPEC> {
        RXGBOCTIM_W::new(self, 1)
    }
    #[doc = "Bit 2 - MMC Receive Good Octet Counter Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn rxgoctim(&mut self) -> RXGOCTIM_W<MMC_RECEIVE_INTERRUPT_MASK_SPEC> {
        RXGOCTIM_W::new(self, 2)
    }
    #[doc = "Bit 3 - MMC Receive Broadcast Good Frame Counter Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn rxbcgfim(&mut self) -> RXBCGFIM_W<MMC_RECEIVE_INTERRUPT_MASK_SPEC> {
        RXBCGFIM_W::new(self, 3)
    }
    #[doc = "Bit 4 - MMC Receive Multicast Good Frame Counter Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn rxmcgfim(&mut self) -> RXMCGFIM_W<MMC_RECEIVE_INTERRUPT_MASK_SPEC> {
        RXMCGFIM_W::new(self, 4)
    }
    #[doc = "Bit 5 - MMC Receive CRC Error Frame Counter Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn rxcrcerfim(&mut self) -> RXCRCERFIM_W<MMC_RECEIVE_INTERRUPT_MASK_SPEC> {
        RXCRCERFIM_W::new(self, 5)
    }
    #[doc = "Bit 6 - MMC Receive Alignment Error Frame Counter Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn rxalgnerfim(&mut self) -> RXALGNERFIM_W<MMC_RECEIVE_INTERRUPT_MASK_SPEC> {
        RXALGNERFIM_W::new(self, 6)
    }
    #[doc = "Bit 7 - MMC Receive Runt Frame Counter Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn rxruntfim(&mut self) -> RXRUNTFIM_W<MMC_RECEIVE_INTERRUPT_MASK_SPEC> {
        RXRUNTFIM_W::new(self, 7)
    }
    #[doc = "Bit 8 - MMC Receive Jabber Error Frame Counter Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn rxjaberfim(&mut self) -> RXJABERFIM_W<MMC_RECEIVE_INTERRUPT_MASK_SPEC> {
        RXJABERFIM_W::new(self, 8)
    }
    #[doc = "Bit 9 - MMC Receive Undersize Good Frame Counter Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn rxusizegfim(&mut self) -> RXUSIZEGFIM_W<MMC_RECEIVE_INTERRUPT_MASK_SPEC> {
        RXUSIZEGFIM_W::new(self, 9)
    }
    #[doc = "Bit 10 - MMC Receive Oversize Good Frame Counter Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn rxosizegfim(&mut self) -> RXOSIZEGFIM_W<MMC_RECEIVE_INTERRUPT_MASK_SPEC> {
        RXOSIZEGFIM_W::new(self, 10)
    }
    #[doc = "Bit 11 - MMC Receive 64 Octet Good Bad Frame Counter Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn rx64octgbfim(&mut self) -> RX64OCTGBFIM_W<MMC_RECEIVE_INTERRUPT_MASK_SPEC> {
        RX64OCTGBFIM_W::new(self, 11)
    }
    #[doc = "Bit 12 - MMC Receive 65 to 127 Octet Good Bad Frame Counter Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn rx65t127octgbfim(&mut self) -> RX65T127OCTGBFIM_W<MMC_RECEIVE_INTERRUPT_MASK_SPEC> {
        RX65T127OCTGBFIM_W::new(self, 12)
    }
    #[doc = "Bit 13 - MMC Receive 128 to 255 Octet Good Bad Frame Counter Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn rx128t255octgbfim(&mut self) -> RX128T255OCTGBFIM_W<MMC_RECEIVE_INTERRUPT_MASK_SPEC> {
        RX128T255OCTGBFIM_W::new(self, 13)
    }
    #[doc = "Bit 14 - MMC Receive 256 to 511 Octet Good Bad Frame Counter Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn rx256t511octgbfim(&mut self) -> RX256T511OCTGBFIM_W<MMC_RECEIVE_INTERRUPT_MASK_SPEC> {
        RX256T511OCTGBFIM_W::new(self, 14)
    }
    #[doc = "Bit 15 - MMC Receive 512 to 1023 Octet Good Bad Frame Counter Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn rx512t1023octgbfim(&mut self) -> RX512T1023OCTGBFIM_W<MMC_RECEIVE_INTERRUPT_MASK_SPEC> {
        RX512T1023OCTGBFIM_W::new(self, 15)
    }
    #[doc = "Bit 16 - MMC Receive 1024 to Maximum Octet Good Bad Frame Counter Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn rx1024tmaxoctgbfim(&mut self) -> RX1024TMAXOCTGBFIM_W<MMC_RECEIVE_INTERRUPT_MASK_SPEC> {
        RX1024TMAXOCTGBFIM_W::new(self, 16)
    }
    #[doc = "Bit 17 - MMC Receive Unicast Good Frame Counter Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn rxucgfim(&mut self) -> RXUCGFIM_W<MMC_RECEIVE_INTERRUPT_MASK_SPEC> {
        RXUCGFIM_W::new(self, 17)
    }
    #[doc = "Bit 18 - MMC Receive Length Error Frame Counter Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn rxlenerfim(&mut self) -> RXLENERFIM_W<MMC_RECEIVE_INTERRUPT_MASK_SPEC> {
        RXLENERFIM_W::new(self, 18)
    }
    #[doc = "Bit 19 - MMC Receive Out Of Range Error Frame Counter Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn rxorangefim(&mut self) -> RXORANGEFIM_W<MMC_RECEIVE_INTERRUPT_MASK_SPEC> {
        RXORANGEFIM_W::new(self, 19)
    }
    #[doc = "Bit 20 - MMC Receive Pause Frame Counter Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn rxpausfim(&mut self) -> RXPAUSFIM_W<MMC_RECEIVE_INTERRUPT_MASK_SPEC> {
        RXPAUSFIM_W::new(self, 20)
    }
    #[doc = "Bit 21 - MMC Receive FIFO Overflow Frame Counter Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn rxfovfim(&mut self) -> RXFOVFIM_W<MMC_RECEIVE_INTERRUPT_MASK_SPEC> {
        RXFOVFIM_W::new(self, 21)
    }
    #[doc = "Bit 22 - MMC Receive VLAN Good Bad Frame Counter Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn rxvlangbfim(&mut self) -> RXVLANGBFIM_W<MMC_RECEIVE_INTERRUPT_MASK_SPEC> {
        RXVLANGBFIM_W::new(self, 22)
    }
    #[doc = "Bit 23 - MMC Receive Watchdog Error Frame Counter Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn rxwdogfim(&mut self) -> RXWDOGFIM_W<MMC_RECEIVE_INTERRUPT_MASK_SPEC> {
        RXWDOGFIM_W::new(self, 23)
    }
    #[doc = "Bit 24 - MMC Receive Error Frame Counter Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn rxrcverrfim(&mut self) -> RXRCVERRFIM_W<MMC_RECEIVE_INTERRUPT_MASK_SPEC> {
        RXRCVERRFIM_W::new(self, 24)
    }
    #[doc = "Bit 25 - MMC Receive Control Frame Counter Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn rxctrlfim(&mut self) -> RXCTRLFIM_W<MMC_RECEIVE_INTERRUPT_MASK_SPEC> {
        RXCTRLFIM_W::new(self, 25)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "MMC Reveive Interrupt Mask Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmc_receive_interrupt_mask::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mmc_receive_interrupt_mask::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MMC_RECEIVE_INTERRUPT_MASK_SPEC;
impl crate::RegisterSpec for MMC_RECEIVE_INTERRUPT_MASK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mmc_receive_interrupt_mask::R`](R) reader structure"]
impl crate::Readable for MMC_RECEIVE_INTERRUPT_MASK_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mmc_receive_interrupt_mask::W`](W) writer structure"]
impl crate::Writable for MMC_RECEIVE_INTERRUPT_MASK_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MMC_RECEIVE_INTERRUPT_MASK to value 0"]
impl crate::Resettable for MMC_RECEIVE_INTERRUPT_MASK_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
