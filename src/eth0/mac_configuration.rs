#[doc = "Register `MAC_CONFIGURATION` reader"]
pub type R = crate::R<MacConfigurationSpec>;
#[doc = "Register `MAC_CONFIGURATION` writer"]
pub type W = crate::W<MacConfigurationSpec>;
#[doc = "Field `PRELEN` reader - Preamble Length for Transmit Frames"]
pub type PrelenR = crate::FieldReader;
#[doc = "Field `PRELEN` writer - Preamble Length for Transmit Frames"]
pub type PrelenW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `RE` reader - Receiver Enable"]
pub type ReR = crate::BitReader;
#[doc = "Field `RE` writer - Receiver Enable"]
pub type ReW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TE` reader - Transmitter Enable"]
pub type TeR = crate::BitReader;
#[doc = "Field `TE` writer - Transmitter Enable"]
pub type TeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DC` reader - Deferral Check"]
pub type DcR = crate::BitReader;
#[doc = "Field `DC` writer - Deferral Check"]
pub type DcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BL` reader - Back-Off Limit"]
pub type BlR = crate::FieldReader;
#[doc = "Field `BL` writer - Back-Off Limit"]
pub type BlW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ACS` reader - Automatic Pad or CRC Stripping"]
pub type AcsR = crate::BitReader;
#[doc = "Field `ACS` writer - Automatic Pad or CRC Stripping"]
pub type AcsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DR` reader - Disable Retry"]
pub type DrR = crate::BitReader;
#[doc = "Field `DR` writer - Disable Retry"]
pub type DrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IPC` reader - Checksum Offload"]
pub type IpcR = crate::BitReader;
#[doc = "Field `IPC` writer - Checksum Offload"]
pub type IpcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DM` reader - Duplex Mode"]
pub type DmR = crate::BitReader;
#[doc = "Field `DM` writer - Duplex Mode"]
pub type DmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LM` reader - Loopback Mode"]
pub type LmR = crate::BitReader;
#[doc = "Field `LM` writer - Loopback Mode"]
pub type LmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DO` reader - Disable Receive Own"]
pub type DoR = crate::BitReader;
#[doc = "Field `DO` writer - Disable Receive Own"]
pub type DoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FES` reader - Speed"]
pub type FesR = crate::BitReader;
#[doc = "Field `FES` writer - Speed"]
pub type FesW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCRS` reader - Disable Carrier Sense During Transmission"]
pub type DcrsR = crate::BitReader;
#[doc = "Field `DCRS` writer - Disable Carrier Sense During Transmission"]
pub type DcrsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IFG` reader - Inter-Frame Gap"]
pub type IfgR = crate::FieldReader;
#[doc = "Field `IFG` writer - Inter-Frame Gap"]
pub type IfgW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `JE` reader - Jumbo Frame Enable"]
pub type JeR = crate::BitReader;
#[doc = "Field `JE` writer - Jumbo Frame Enable"]
pub type JeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BE` reader - Frame Burst Enable"]
pub type BeR = crate::BitReader;
#[doc = "Field `JD` reader - Jabber Disable"]
pub type JdR = crate::BitReader;
#[doc = "Field `JD` writer - Jabber Disable"]
pub type JdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WD` reader - Watchdog Disable"]
pub type WdR = crate::BitReader;
#[doc = "Field `WD` writer - Watchdog Disable"]
pub type WdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TC` reader - Transmit Configuration in RMII"]
pub type TcR = crate::BitReader;
#[doc = "Field `CST` reader - CRC Stripping of Type Frames"]
pub type CstR = crate::BitReader;
#[doc = "Field `CST` writer - CRC Stripping of Type Frames"]
pub type CstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TWOKPE` reader - IEEE 802.3as support for 2K packets Enable"]
pub type TwokpeR = crate::BitReader;
#[doc = "Field `TWOKPE` writer - IEEE 802.3as support for 2K packets Enable"]
pub type TwokpeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SARC` reader - Source Address Insertion or Replacement Control"]
pub type SarcR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:1 - Preamble Length for Transmit Frames"]
    #[inline(always)]
    pub fn prelen(&self) -> PrelenR {
        PrelenR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - Receiver Enable"]
    #[inline(always)]
    pub fn re(&self) -> ReR {
        ReR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Transmitter Enable"]
    #[inline(always)]
    pub fn te(&self) -> TeR {
        TeR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Deferral Check"]
    #[inline(always)]
    pub fn dc(&self) -> DcR {
        DcR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:6 - Back-Off Limit"]
    #[inline(always)]
    pub fn bl(&self) -> BlR {
        BlR::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 7 - Automatic Pad or CRC Stripping"]
    #[inline(always)]
    pub fn acs(&self) -> AcsR {
        AcsR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 9 - Disable Retry"]
    #[inline(always)]
    pub fn dr(&self) -> DrR {
        DrR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Checksum Offload"]
    #[inline(always)]
    pub fn ipc(&self) -> IpcR {
        IpcR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Duplex Mode"]
    #[inline(always)]
    pub fn dm(&self) -> DmR {
        DmR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Loopback Mode"]
    #[inline(always)]
    pub fn lm(&self) -> LmR {
        LmR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Disable Receive Own"]
    #[inline(always)]
    pub fn do_(&self) -> DoR {
        DoR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Speed"]
    #[inline(always)]
    pub fn fes(&self) -> FesR {
        FesR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - Disable Carrier Sense During Transmission"]
    #[inline(always)]
    pub fn dcrs(&self) -> DcrsR {
        DcrsR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:19 - Inter-Frame Gap"]
    #[inline(always)]
    pub fn ifg(&self) -> IfgR {
        IfgR::new(((self.bits >> 17) & 7) as u8)
    }
    #[doc = "Bit 20 - Jumbo Frame Enable"]
    #[inline(always)]
    pub fn je(&self) -> JeR {
        JeR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Frame Burst Enable"]
    #[inline(always)]
    pub fn be(&self) -> BeR {
        BeR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Jabber Disable"]
    #[inline(always)]
    pub fn jd(&self) -> JdR {
        JdR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Watchdog Disable"]
    #[inline(always)]
    pub fn wd(&self) -> WdR {
        WdR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Transmit Configuration in RMII"]
    #[inline(always)]
    pub fn tc(&self) -> TcR {
        TcR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - CRC Stripping of Type Frames"]
    #[inline(always)]
    pub fn cst(&self) -> CstR {
        CstR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 27 - IEEE 802.3as support for 2K packets Enable"]
    #[inline(always)]
    pub fn twokpe(&self) -> TwokpeR {
        TwokpeR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bits 28:30 - Source Address Insertion or Replacement Control"]
    #[inline(always)]
    pub fn sarc(&self) -> SarcR {
        SarcR::new(((self.bits >> 28) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Preamble Length for Transmit Frames"]
    #[inline(always)]
    #[must_use]
    pub fn prelen(&mut self) -> PrelenW<MacConfigurationSpec> {
        PrelenW::new(self, 0)
    }
    #[doc = "Bit 2 - Receiver Enable"]
    #[inline(always)]
    #[must_use]
    pub fn re(&mut self) -> ReW<MacConfigurationSpec> {
        ReW::new(self, 2)
    }
    #[doc = "Bit 3 - Transmitter Enable"]
    #[inline(always)]
    #[must_use]
    pub fn te(&mut self) -> TeW<MacConfigurationSpec> {
        TeW::new(self, 3)
    }
    #[doc = "Bit 4 - Deferral Check"]
    #[inline(always)]
    #[must_use]
    pub fn dc(&mut self) -> DcW<MacConfigurationSpec> {
        DcW::new(self, 4)
    }
    #[doc = "Bits 5:6 - Back-Off Limit"]
    #[inline(always)]
    #[must_use]
    pub fn bl(&mut self) -> BlW<MacConfigurationSpec> {
        BlW::new(self, 5)
    }
    #[doc = "Bit 7 - Automatic Pad or CRC Stripping"]
    #[inline(always)]
    #[must_use]
    pub fn acs(&mut self) -> AcsW<MacConfigurationSpec> {
        AcsW::new(self, 7)
    }
    #[doc = "Bit 9 - Disable Retry"]
    #[inline(always)]
    #[must_use]
    pub fn dr(&mut self) -> DrW<MacConfigurationSpec> {
        DrW::new(self, 9)
    }
    #[doc = "Bit 10 - Checksum Offload"]
    #[inline(always)]
    #[must_use]
    pub fn ipc(&mut self) -> IpcW<MacConfigurationSpec> {
        IpcW::new(self, 10)
    }
    #[doc = "Bit 11 - Duplex Mode"]
    #[inline(always)]
    #[must_use]
    pub fn dm(&mut self) -> DmW<MacConfigurationSpec> {
        DmW::new(self, 11)
    }
    #[doc = "Bit 12 - Loopback Mode"]
    #[inline(always)]
    #[must_use]
    pub fn lm(&mut self) -> LmW<MacConfigurationSpec> {
        LmW::new(self, 12)
    }
    #[doc = "Bit 13 - Disable Receive Own"]
    #[inline(always)]
    #[must_use]
    pub fn do_(&mut self) -> DoW<MacConfigurationSpec> {
        DoW::new(self, 13)
    }
    #[doc = "Bit 14 - Speed"]
    #[inline(always)]
    #[must_use]
    pub fn fes(&mut self) -> FesW<MacConfigurationSpec> {
        FesW::new(self, 14)
    }
    #[doc = "Bit 16 - Disable Carrier Sense During Transmission"]
    #[inline(always)]
    #[must_use]
    pub fn dcrs(&mut self) -> DcrsW<MacConfigurationSpec> {
        DcrsW::new(self, 16)
    }
    #[doc = "Bits 17:19 - Inter-Frame Gap"]
    #[inline(always)]
    #[must_use]
    pub fn ifg(&mut self) -> IfgW<MacConfigurationSpec> {
        IfgW::new(self, 17)
    }
    #[doc = "Bit 20 - Jumbo Frame Enable"]
    #[inline(always)]
    #[must_use]
    pub fn je(&mut self) -> JeW<MacConfigurationSpec> {
        JeW::new(self, 20)
    }
    #[doc = "Bit 22 - Jabber Disable"]
    #[inline(always)]
    #[must_use]
    pub fn jd(&mut self) -> JdW<MacConfigurationSpec> {
        JdW::new(self, 22)
    }
    #[doc = "Bit 23 - Watchdog Disable"]
    #[inline(always)]
    #[must_use]
    pub fn wd(&mut self) -> WdW<MacConfigurationSpec> {
        WdW::new(self, 23)
    }
    #[doc = "Bit 25 - CRC Stripping of Type Frames"]
    #[inline(always)]
    #[must_use]
    pub fn cst(&mut self) -> CstW<MacConfigurationSpec> {
        CstW::new(self, 25)
    }
    #[doc = "Bit 27 - IEEE 802.3as support for 2K packets Enable"]
    #[inline(always)]
    #[must_use]
    pub fn twokpe(&mut self) -> TwokpeW<MacConfigurationSpec> {
        TwokpeW::new(self, 27)
    }
}
#[doc = "MAC Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mac_configuration::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mac_configuration::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MacConfigurationSpec;
impl crate::RegisterSpec for MacConfigurationSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mac_configuration::R`](R) reader structure"]
impl crate::Readable for MacConfigurationSpec {}
#[doc = "`write(|w| ..)` method takes [`mac_configuration::W`](W) writer structure"]
impl crate::Writable for MacConfigurationSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MAC_CONFIGURATION to value 0x8000"]
impl crate::Resettable for MacConfigurationSpec {
    const RESET_VALUE: u32 = 0x8000;
}
