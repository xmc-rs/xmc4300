#[doc = "Register `MMC_TRANSMIT_INTERRUPT_MASK` reader"]
pub type R = crate::R<MMC_TRANSMIT_INTERRUPT_MASK_SPEC>;
#[doc = "Register `MMC_TRANSMIT_INTERRUPT_MASK` writer"]
pub type W = crate::W<MMC_TRANSMIT_INTERRUPT_MASK_SPEC>;
#[doc = "Field `TXGBOCTIM` reader - MMC Transmit Good Bad Octet Counter Interrupt Mask"]
pub type TXGBOCTIM_R = crate::BitReader;
#[doc = "Field `TXGBOCTIM` writer - MMC Transmit Good Bad Octet Counter Interrupt Mask"]
pub type TXGBOCTIM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXGBFRMIM` reader - MMC Transmit Good Bad Frame Counter Interrupt Mask"]
pub type TXGBFRMIM_R = crate::BitReader;
#[doc = "Field `TXGBFRMIM` writer - MMC Transmit Good Bad Frame Counter Interrupt Mask"]
pub type TXGBFRMIM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXBCGFIM` reader - MMC Transmit Broadcast Good Frame Counter Interrupt Mask"]
pub type TXBCGFIM_R = crate::BitReader;
#[doc = "Field `TXBCGFIM` writer - MMC Transmit Broadcast Good Frame Counter Interrupt Mask"]
pub type TXBCGFIM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXMCGFIM` reader - MMC Transmit Multicast Good Frame Counter Interrupt Mask"]
pub type TXMCGFIM_R = crate::BitReader;
#[doc = "Field `TXMCGFIM` writer - MMC Transmit Multicast Good Frame Counter Interrupt Mask"]
pub type TXMCGFIM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX64OCTGBFIM` reader - MMC Transmit 64 Octet Good Bad Frame Counter Interrupt Mask"]
pub type TX64OCTGBFIM_R = crate::BitReader;
#[doc = "Field `TX64OCTGBFIM` writer - MMC Transmit 64 Octet Good Bad Frame Counter Interrupt Mask"]
pub type TX64OCTGBFIM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX65T127OCTGBFIM` reader - MMC Transmit 65 to 127 Octet Good Bad Frame Counter Interrupt Mask"]
pub type TX65T127OCTGBFIM_R = crate::BitReader;
#[doc = "Field `TX65T127OCTGBFIM` writer - MMC Transmit 65 to 127 Octet Good Bad Frame Counter Interrupt Mask"]
pub type TX65T127OCTGBFIM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX128T255OCTGBFIM` reader - MMC Transmit 128 to 255 Octet Good Bad Frame Counter Interrupt Mask"]
pub type TX128T255OCTGBFIM_R = crate::BitReader;
#[doc = "Field `TX128T255OCTGBFIM` writer - MMC Transmit 128 to 255 Octet Good Bad Frame Counter Interrupt Mask"]
pub type TX128T255OCTGBFIM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX256T511OCTGBFIM` reader - MMC Transmit 256 to 511 Octet Good Bad Frame Counter Interrupt Mask"]
pub type TX256T511OCTGBFIM_R = crate::BitReader;
#[doc = "Field `TX256T511OCTGBFIM` writer - MMC Transmit 256 to 511 Octet Good Bad Frame Counter Interrupt Mask"]
pub type TX256T511OCTGBFIM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX512T1023OCTGBFIM` reader - MMC Transmit 512 to 1023 Octet Good Bad Frame Counter Interrupt Mask"]
pub type TX512T1023OCTGBFIM_R = crate::BitReader;
#[doc = "Field `TX512T1023OCTGBFIM` writer - MMC Transmit 512 to 1023 Octet Good Bad Frame Counter Interrupt Mask"]
pub type TX512T1023OCTGBFIM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX1024TMAXOCTGBFIM` reader - MMC Transmit 1024 to Maximum Octet Good Bad Frame Counter Interrupt Mask"]
pub type TX1024TMAXOCTGBFIM_R = crate::BitReader;
#[doc = "Field `TX1024TMAXOCTGBFIM` writer - MMC Transmit 1024 to Maximum Octet Good Bad Frame Counter Interrupt Mask"]
pub type TX1024TMAXOCTGBFIM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXUCGBFIM` reader - MMC Transmit Unicast Good Bad Frame Counter Interrupt Mask"]
pub type TXUCGBFIM_R = crate::BitReader;
#[doc = "Field `TXUCGBFIM` writer - MMC Transmit Unicast Good Bad Frame Counter Interrupt Mask"]
pub type TXUCGBFIM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXMCGBFIM` reader - MMC Transmit Multicast Good Bad Frame Counter Interrupt Mask"]
pub type TXMCGBFIM_R = crate::BitReader;
#[doc = "Field `TXMCGBFIM` writer - MMC Transmit Multicast Good Bad Frame Counter Interrupt Mask"]
pub type TXMCGBFIM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXBCGBFIM` reader - MMC Transmit Broadcast Good Bad Frame Counter Interrupt Mask"]
pub type TXBCGBFIM_R = crate::BitReader;
#[doc = "Field `TXBCGBFIM` writer - MMC Transmit Broadcast Good Bad Frame Counter Interrupt Mask"]
pub type TXBCGBFIM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXUFLOWERFIM` reader - MMC Transmit Underflow Error Frame Counter Interrupt Mask"]
pub type TXUFLOWERFIM_R = crate::BitReader;
#[doc = "Field `TXUFLOWERFIM` writer - MMC Transmit Underflow Error Frame Counter Interrupt Mask"]
pub type TXUFLOWERFIM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXSCOLGFIM` reader - MMC Transmit Single Collision Good Frame Counter Interrupt Mask"]
pub type TXSCOLGFIM_R = crate::BitReader;
#[doc = "Field `TXSCOLGFIM` writer - MMC Transmit Single Collision Good Frame Counter Interrupt Mask"]
pub type TXSCOLGFIM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXMCOLGFIM` reader - MMC Transmit Multiple Collision Good Frame Counter Interrupt Mask"]
pub type TXMCOLGFIM_R = crate::BitReader;
#[doc = "Field `TXMCOLGFIM` writer - MMC Transmit Multiple Collision Good Frame Counter Interrupt Mask"]
pub type TXMCOLGFIM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXDEFFIM` reader - MMC Transmit Deferred Frame Counter Interrupt Mask"]
pub type TXDEFFIM_R = crate::BitReader;
#[doc = "Field `TXDEFFIM` writer - MMC Transmit Deferred Frame Counter Interrupt Mask"]
pub type TXDEFFIM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXLATCOLFIM` reader - MMC Transmit Late Collision Frame Counter Interrupt Mask"]
pub type TXLATCOLFIM_R = crate::BitReader;
#[doc = "Field `TXLATCOLFIM` writer - MMC Transmit Late Collision Frame Counter Interrupt Mask"]
pub type TXLATCOLFIM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXEXCOLFIM` reader - MMC Transmit Excessive Collision Frame Counter Interrupt Mask"]
pub type TXEXCOLFIM_R = crate::BitReader;
#[doc = "Field `TXEXCOLFIM` writer - MMC Transmit Excessive Collision Frame Counter Interrupt Mask"]
pub type TXEXCOLFIM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXCARERFIM` reader - MMC Transmit Carrier Error Frame Counter Interrupt Mask"]
pub type TXCARERFIM_R = crate::BitReader;
#[doc = "Field `TXCARERFIM` writer - MMC Transmit Carrier Error Frame Counter Interrupt Mask"]
pub type TXCARERFIM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXGOCTIM` reader - MMC Transmit Good Octet Counter Interrupt Mask"]
pub type TXGOCTIM_R = crate::BitReader;
#[doc = "Field `TXGOCTIM` writer - MMC Transmit Good Octet Counter Interrupt Mask"]
pub type TXGOCTIM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXGFRMIM` reader - MMC Transmit Good Frame Counter Interrupt Mask"]
pub type TXGFRMIM_R = crate::BitReader;
#[doc = "Field `TXGFRMIM` writer - MMC Transmit Good Frame Counter Interrupt Mask"]
pub type TXGFRMIM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXEXDEFFIM` reader - MMC Transmit Excessive Deferral Frame Counter Interrupt Mask"]
pub type TXEXDEFFIM_R = crate::BitReader;
#[doc = "Field `TXEXDEFFIM` writer - MMC Transmit Excessive Deferral Frame Counter Interrupt Mask"]
pub type TXEXDEFFIM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXPAUSFIM` reader - MMC Transmit Pause Frame Counter Interrupt Mask"]
pub type TXPAUSFIM_R = crate::BitReader;
#[doc = "Field `TXPAUSFIM` writer - MMC Transmit Pause Frame Counter Interrupt Mask"]
pub type TXPAUSFIM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXVLANGFIM` reader - MMC Transmit VLAN Good Frame Counter Interrupt Mask"]
pub type TXVLANGFIM_R = crate::BitReader;
#[doc = "Field `TXVLANGFIM` writer - MMC Transmit VLAN Good Frame Counter Interrupt Mask"]
pub type TXVLANGFIM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXOSIZEGFIM` reader - MMC Transmit Oversize Good Frame Counter Interrupt Mask"]
pub type TXOSIZEGFIM_R = crate::BitReader;
#[doc = "Field `TXOSIZEGFIM` writer - MMC Transmit Oversize Good Frame Counter Interrupt Mask"]
pub type TXOSIZEGFIM_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - MMC Transmit Good Bad Octet Counter Interrupt Mask"]
    #[inline(always)]
    pub fn txgboctim(&self) -> TXGBOCTIM_R {
        TXGBOCTIM_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - MMC Transmit Good Bad Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn txgbfrmim(&self) -> TXGBFRMIM_R {
        TXGBFRMIM_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - MMC Transmit Broadcast Good Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn txbcgfim(&self) -> TXBCGFIM_R {
        TXBCGFIM_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - MMC Transmit Multicast Good Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn txmcgfim(&self) -> TXMCGFIM_R {
        TXMCGFIM_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - MMC Transmit 64 Octet Good Bad Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn tx64octgbfim(&self) -> TX64OCTGBFIM_R {
        TX64OCTGBFIM_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - MMC Transmit 65 to 127 Octet Good Bad Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn tx65t127octgbfim(&self) -> TX65T127OCTGBFIM_R {
        TX65T127OCTGBFIM_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - MMC Transmit 128 to 255 Octet Good Bad Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn tx128t255octgbfim(&self) -> TX128T255OCTGBFIM_R {
        TX128T255OCTGBFIM_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - MMC Transmit 256 to 511 Octet Good Bad Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn tx256t511octgbfim(&self) -> TX256T511OCTGBFIM_R {
        TX256T511OCTGBFIM_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - MMC Transmit 512 to 1023 Octet Good Bad Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn tx512t1023octgbfim(&self) -> TX512T1023OCTGBFIM_R {
        TX512T1023OCTGBFIM_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - MMC Transmit 1024 to Maximum Octet Good Bad Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn tx1024tmaxoctgbfim(&self) -> TX1024TMAXOCTGBFIM_R {
        TX1024TMAXOCTGBFIM_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - MMC Transmit Unicast Good Bad Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn txucgbfim(&self) -> TXUCGBFIM_R {
        TXUCGBFIM_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - MMC Transmit Multicast Good Bad Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn txmcgbfim(&self) -> TXMCGBFIM_R {
        TXMCGBFIM_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - MMC Transmit Broadcast Good Bad Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn txbcgbfim(&self) -> TXBCGBFIM_R {
        TXBCGBFIM_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - MMC Transmit Underflow Error Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn txuflowerfim(&self) -> TXUFLOWERFIM_R {
        TXUFLOWERFIM_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - MMC Transmit Single Collision Good Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn txscolgfim(&self) -> TXSCOLGFIM_R {
        TXSCOLGFIM_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - MMC Transmit Multiple Collision Good Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn txmcolgfim(&self) -> TXMCOLGFIM_R {
        TXMCOLGFIM_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - MMC Transmit Deferred Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn txdeffim(&self) -> TXDEFFIM_R {
        TXDEFFIM_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - MMC Transmit Late Collision Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn txlatcolfim(&self) -> TXLATCOLFIM_R {
        TXLATCOLFIM_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - MMC Transmit Excessive Collision Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn txexcolfim(&self) -> TXEXCOLFIM_R {
        TXEXCOLFIM_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - MMC Transmit Carrier Error Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn txcarerfim(&self) -> TXCARERFIM_R {
        TXCARERFIM_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - MMC Transmit Good Octet Counter Interrupt Mask"]
    #[inline(always)]
    pub fn txgoctim(&self) -> TXGOCTIM_R {
        TXGOCTIM_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - MMC Transmit Good Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn txgfrmim(&self) -> TXGFRMIM_R {
        TXGFRMIM_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - MMC Transmit Excessive Deferral Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn txexdeffim(&self) -> TXEXDEFFIM_R {
        TXEXDEFFIM_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - MMC Transmit Pause Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn txpausfim(&self) -> TXPAUSFIM_R {
        TXPAUSFIM_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - MMC Transmit VLAN Good Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn txvlangfim(&self) -> TXVLANGFIM_R {
        TXVLANGFIM_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - MMC Transmit Oversize Good Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn txosizegfim(&self) -> TXOSIZEGFIM_R {
        TXOSIZEGFIM_R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - MMC Transmit Good Bad Octet Counter Interrupt Mask"]
    #[inline(always)]
    pub fn txgboctim(&mut self) -> TXGBOCTIM_W<MMC_TRANSMIT_INTERRUPT_MASK_SPEC> {
        TXGBOCTIM_W::new(self, 0)
    }
    #[doc = "Bit 1 - MMC Transmit Good Bad Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn txgbfrmim(&mut self) -> TXGBFRMIM_W<MMC_TRANSMIT_INTERRUPT_MASK_SPEC> {
        TXGBFRMIM_W::new(self, 1)
    }
    #[doc = "Bit 2 - MMC Transmit Broadcast Good Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn txbcgfim(&mut self) -> TXBCGFIM_W<MMC_TRANSMIT_INTERRUPT_MASK_SPEC> {
        TXBCGFIM_W::new(self, 2)
    }
    #[doc = "Bit 3 - MMC Transmit Multicast Good Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn txmcgfim(&mut self) -> TXMCGFIM_W<MMC_TRANSMIT_INTERRUPT_MASK_SPEC> {
        TXMCGFIM_W::new(self, 3)
    }
    #[doc = "Bit 4 - MMC Transmit 64 Octet Good Bad Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn tx64octgbfim(&mut self) -> TX64OCTGBFIM_W<MMC_TRANSMIT_INTERRUPT_MASK_SPEC> {
        TX64OCTGBFIM_W::new(self, 4)
    }
    #[doc = "Bit 5 - MMC Transmit 65 to 127 Octet Good Bad Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn tx65t127octgbfim(&mut self) -> TX65T127OCTGBFIM_W<MMC_TRANSMIT_INTERRUPT_MASK_SPEC> {
        TX65T127OCTGBFIM_W::new(self, 5)
    }
    #[doc = "Bit 6 - MMC Transmit 128 to 255 Octet Good Bad Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn tx128t255octgbfim(&mut self) -> TX128T255OCTGBFIM_W<MMC_TRANSMIT_INTERRUPT_MASK_SPEC> {
        TX128T255OCTGBFIM_W::new(self, 6)
    }
    #[doc = "Bit 7 - MMC Transmit 256 to 511 Octet Good Bad Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn tx256t511octgbfim(&mut self) -> TX256T511OCTGBFIM_W<MMC_TRANSMIT_INTERRUPT_MASK_SPEC> {
        TX256T511OCTGBFIM_W::new(self, 7)
    }
    #[doc = "Bit 8 - MMC Transmit 512 to 1023 Octet Good Bad Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn tx512t1023octgbfim(&mut self) -> TX512T1023OCTGBFIM_W<MMC_TRANSMIT_INTERRUPT_MASK_SPEC> {
        TX512T1023OCTGBFIM_W::new(self, 8)
    }
    #[doc = "Bit 9 - MMC Transmit 1024 to Maximum Octet Good Bad Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn tx1024tmaxoctgbfim(&mut self) -> TX1024TMAXOCTGBFIM_W<MMC_TRANSMIT_INTERRUPT_MASK_SPEC> {
        TX1024TMAXOCTGBFIM_W::new(self, 9)
    }
    #[doc = "Bit 10 - MMC Transmit Unicast Good Bad Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn txucgbfim(&mut self) -> TXUCGBFIM_W<MMC_TRANSMIT_INTERRUPT_MASK_SPEC> {
        TXUCGBFIM_W::new(self, 10)
    }
    #[doc = "Bit 11 - MMC Transmit Multicast Good Bad Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn txmcgbfim(&mut self) -> TXMCGBFIM_W<MMC_TRANSMIT_INTERRUPT_MASK_SPEC> {
        TXMCGBFIM_W::new(self, 11)
    }
    #[doc = "Bit 12 - MMC Transmit Broadcast Good Bad Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn txbcgbfim(&mut self) -> TXBCGBFIM_W<MMC_TRANSMIT_INTERRUPT_MASK_SPEC> {
        TXBCGBFIM_W::new(self, 12)
    }
    #[doc = "Bit 13 - MMC Transmit Underflow Error Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn txuflowerfim(&mut self) -> TXUFLOWERFIM_W<MMC_TRANSMIT_INTERRUPT_MASK_SPEC> {
        TXUFLOWERFIM_W::new(self, 13)
    }
    #[doc = "Bit 14 - MMC Transmit Single Collision Good Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn txscolgfim(&mut self) -> TXSCOLGFIM_W<MMC_TRANSMIT_INTERRUPT_MASK_SPEC> {
        TXSCOLGFIM_W::new(self, 14)
    }
    #[doc = "Bit 15 - MMC Transmit Multiple Collision Good Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn txmcolgfim(&mut self) -> TXMCOLGFIM_W<MMC_TRANSMIT_INTERRUPT_MASK_SPEC> {
        TXMCOLGFIM_W::new(self, 15)
    }
    #[doc = "Bit 16 - MMC Transmit Deferred Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn txdeffim(&mut self) -> TXDEFFIM_W<MMC_TRANSMIT_INTERRUPT_MASK_SPEC> {
        TXDEFFIM_W::new(self, 16)
    }
    #[doc = "Bit 17 - MMC Transmit Late Collision Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn txlatcolfim(&mut self) -> TXLATCOLFIM_W<MMC_TRANSMIT_INTERRUPT_MASK_SPEC> {
        TXLATCOLFIM_W::new(self, 17)
    }
    #[doc = "Bit 18 - MMC Transmit Excessive Collision Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn txexcolfim(&mut self) -> TXEXCOLFIM_W<MMC_TRANSMIT_INTERRUPT_MASK_SPEC> {
        TXEXCOLFIM_W::new(self, 18)
    }
    #[doc = "Bit 19 - MMC Transmit Carrier Error Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn txcarerfim(&mut self) -> TXCARERFIM_W<MMC_TRANSMIT_INTERRUPT_MASK_SPEC> {
        TXCARERFIM_W::new(self, 19)
    }
    #[doc = "Bit 20 - MMC Transmit Good Octet Counter Interrupt Mask"]
    #[inline(always)]
    pub fn txgoctim(&mut self) -> TXGOCTIM_W<MMC_TRANSMIT_INTERRUPT_MASK_SPEC> {
        TXGOCTIM_W::new(self, 20)
    }
    #[doc = "Bit 21 - MMC Transmit Good Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn txgfrmim(&mut self) -> TXGFRMIM_W<MMC_TRANSMIT_INTERRUPT_MASK_SPEC> {
        TXGFRMIM_W::new(self, 21)
    }
    #[doc = "Bit 22 - MMC Transmit Excessive Deferral Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn txexdeffim(&mut self) -> TXEXDEFFIM_W<MMC_TRANSMIT_INTERRUPT_MASK_SPEC> {
        TXEXDEFFIM_W::new(self, 22)
    }
    #[doc = "Bit 23 - MMC Transmit Pause Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn txpausfim(&mut self) -> TXPAUSFIM_W<MMC_TRANSMIT_INTERRUPT_MASK_SPEC> {
        TXPAUSFIM_W::new(self, 23)
    }
    #[doc = "Bit 24 - MMC Transmit VLAN Good Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn txvlangfim(&mut self) -> TXVLANGFIM_W<MMC_TRANSMIT_INTERRUPT_MASK_SPEC> {
        TXVLANGFIM_W::new(self, 24)
    }
    #[doc = "Bit 25 - MMC Transmit Oversize Good Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn txosizegfim(&mut self) -> TXOSIZEGFIM_W<MMC_TRANSMIT_INTERRUPT_MASK_SPEC> {
        TXOSIZEGFIM_W::new(self, 25)
    }
}
#[doc = "MMC Transmit Interrupt Mask Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mmc_transmit_interrupt_mask::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mmc_transmit_interrupt_mask::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MMC_TRANSMIT_INTERRUPT_MASK_SPEC;
impl crate::RegisterSpec for MMC_TRANSMIT_INTERRUPT_MASK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mmc_transmit_interrupt_mask::R`](R) reader structure"]
impl crate::Readable for MMC_TRANSMIT_INTERRUPT_MASK_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mmc_transmit_interrupt_mask::W`](W) writer structure"]
impl crate::Writable for MMC_TRANSMIT_INTERRUPT_MASK_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MMC_TRANSMIT_INTERRUPT_MASK to value 0"]
impl crate::Resettable for MMC_TRANSMIT_INTERRUPT_MASK_SPEC {
    const RESET_VALUE: u32 = 0;
}
