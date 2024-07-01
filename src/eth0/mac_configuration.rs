#[doc = "Register `MAC_CONFIGURATION` reader"]
pub type R = crate::R<MAC_CONFIGURATION_SPEC>;
#[doc = "Register `MAC_CONFIGURATION` writer"]
pub type W = crate::W<MAC_CONFIGURATION_SPEC>;
#[doc = "Field `PRELEN` reader - Preamble Length for Transmit Frames"]
pub type PRELEN_R = crate::FieldReader;
#[doc = "Field `PRELEN` writer - Preamble Length for Transmit Frames"]
pub type PRELEN_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `RE` reader - Receiver Enable"]
pub type RE_R = crate::BitReader;
#[doc = "Field `RE` writer - Receiver Enable"]
pub type RE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TE` reader - Transmitter Enable"]
pub type TE_R = crate::BitReader;
#[doc = "Field `TE` writer - Transmitter Enable"]
pub type TE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DC` reader - Deferral Check"]
pub type DC_R = crate::BitReader;
#[doc = "Field `DC` writer - Deferral Check"]
pub type DC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BL` reader - Back-Off Limit"]
pub type BL_R = crate::FieldReader;
#[doc = "Field `BL` writer - Back-Off Limit"]
pub type BL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ACS` reader - Automatic Pad or CRC Stripping"]
pub type ACS_R = crate::BitReader;
#[doc = "Field `ACS` writer - Automatic Pad or CRC Stripping"]
pub type ACS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DR` reader - Disable Retry"]
pub type DR_R = crate::BitReader;
#[doc = "Field `DR` writer - Disable Retry"]
pub type DR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IPC` reader - Checksum Offload"]
pub type IPC_R = crate::BitReader;
#[doc = "Field `IPC` writer - Checksum Offload"]
pub type IPC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DM` reader - Duplex Mode"]
pub type DM_R = crate::BitReader;
#[doc = "Field `DM` writer - Duplex Mode"]
pub type DM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LM` reader - Loopback Mode"]
pub type LM_R = crate::BitReader;
#[doc = "Field `LM` writer - Loopback Mode"]
pub type LM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DO` reader - Disable Receive Own"]
pub type DO_R = crate::BitReader;
#[doc = "Field `DO` writer - Disable Receive Own"]
pub type DO_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FES` reader - Speed"]
pub type FES_R = crate::BitReader;
#[doc = "Field `FES` writer - Speed"]
pub type FES_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCRS` reader - Disable Carrier Sense During Transmission"]
pub type DCRS_R = crate::BitReader;
#[doc = "Field `DCRS` writer - Disable Carrier Sense During Transmission"]
pub type DCRS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IFG` reader - Inter-Frame Gap"]
pub type IFG_R = crate::FieldReader;
#[doc = "Field `IFG` writer - Inter-Frame Gap"]
pub type IFG_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `JE` reader - Jumbo Frame Enable"]
pub type JE_R = crate::BitReader;
#[doc = "Field `JE` writer - Jumbo Frame Enable"]
pub type JE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BE` reader - Frame Burst Enable"]
pub type BE_R = crate::BitReader;
#[doc = "Field `JD` reader - Jabber Disable"]
pub type JD_R = crate::BitReader;
#[doc = "Field `JD` writer - Jabber Disable"]
pub type JD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WD` reader - Watchdog Disable"]
pub type WD_R = crate::BitReader;
#[doc = "Field `WD` writer - Watchdog Disable"]
pub type WD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TC` reader - Transmit Configuration in RMII"]
pub type TC_R = crate::BitReader;
#[doc = "Field `CST` reader - CRC Stripping of Type Frames"]
pub type CST_R = crate::BitReader;
#[doc = "Field `CST` writer - CRC Stripping of Type Frames"]
pub type CST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TWOKPE` reader - IEEE 802.3as support for 2K packets Enable"]
pub type TWOKPE_R = crate::BitReader;
#[doc = "Field `TWOKPE` writer - IEEE 802.3as support for 2K packets Enable"]
pub type TWOKPE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SARC` reader - Source Address Insertion or Replacement Control"]
pub type SARC_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:1 - Preamble Length for Transmit Frames"]
    #[inline(always)]
    pub fn prelen(&self) -> PRELEN_R {
        PRELEN_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - Receiver Enable"]
    #[inline(always)]
    pub fn re(&self) -> RE_R {
        RE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Transmitter Enable"]
    #[inline(always)]
    pub fn te(&self) -> TE_R {
        TE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Deferral Check"]
    #[inline(always)]
    pub fn dc(&self) -> DC_R {
        DC_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:6 - Back-Off Limit"]
    #[inline(always)]
    pub fn bl(&self) -> BL_R {
        BL_R::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 7 - Automatic Pad or CRC Stripping"]
    #[inline(always)]
    pub fn acs(&self) -> ACS_R {
        ACS_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 9 - Disable Retry"]
    #[inline(always)]
    pub fn dr(&self) -> DR_R {
        DR_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Checksum Offload"]
    #[inline(always)]
    pub fn ipc(&self) -> IPC_R {
        IPC_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Duplex Mode"]
    #[inline(always)]
    pub fn dm(&self) -> DM_R {
        DM_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Loopback Mode"]
    #[inline(always)]
    pub fn lm(&self) -> LM_R {
        LM_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Disable Receive Own"]
    #[inline(always)]
    pub fn do_(&self) -> DO_R {
        DO_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Speed"]
    #[inline(always)]
    pub fn fes(&self) -> FES_R {
        FES_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - Disable Carrier Sense During Transmission"]
    #[inline(always)]
    pub fn dcrs(&self) -> DCRS_R {
        DCRS_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:19 - Inter-Frame Gap"]
    #[inline(always)]
    pub fn ifg(&self) -> IFG_R {
        IFG_R::new(((self.bits >> 17) & 7) as u8)
    }
    #[doc = "Bit 20 - Jumbo Frame Enable"]
    #[inline(always)]
    pub fn je(&self) -> JE_R {
        JE_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Frame Burst Enable"]
    #[inline(always)]
    pub fn be(&self) -> BE_R {
        BE_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Jabber Disable"]
    #[inline(always)]
    pub fn jd(&self) -> JD_R {
        JD_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Watchdog Disable"]
    #[inline(always)]
    pub fn wd(&self) -> WD_R {
        WD_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Transmit Configuration in RMII"]
    #[inline(always)]
    pub fn tc(&self) -> TC_R {
        TC_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - CRC Stripping of Type Frames"]
    #[inline(always)]
    pub fn cst(&self) -> CST_R {
        CST_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 27 - IEEE 802.3as support for 2K packets Enable"]
    #[inline(always)]
    pub fn twokpe(&self) -> TWOKPE_R {
        TWOKPE_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bits 28:30 - Source Address Insertion or Replacement Control"]
    #[inline(always)]
    pub fn sarc(&self) -> SARC_R {
        SARC_R::new(((self.bits >> 28) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Preamble Length for Transmit Frames"]
    #[inline(always)]
    #[must_use]
    pub fn prelen(&mut self) -> PRELEN_W<MAC_CONFIGURATION_SPEC> {
        PRELEN_W::new(self, 0)
    }
    #[doc = "Bit 2 - Receiver Enable"]
    #[inline(always)]
    #[must_use]
    pub fn re(&mut self) -> RE_W<MAC_CONFIGURATION_SPEC> {
        RE_W::new(self, 2)
    }
    #[doc = "Bit 3 - Transmitter Enable"]
    #[inline(always)]
    #[must_use]
    pub fn te(&mut self) -> TE_W<MAC_CONFIGURATION_SPEC> {
        TE_W::new(self, 3)
    }
    #[doc = "Bit 4 - Deferral Check"]
    #[inline(always)]
    #[must_use]
    pub fn dc(&mut self) -> DC_W<MAC_CONFIGURATION_SPEC> {
        DC_W::new(self, 4)
    }
    #[doc = "Bits 5:6 - Back-Off Limit"]
    #[inline(always)]
    #[must_use]
    pub fn bl(&mut self) -> BL_W<MAC_CONFIGURATION_SPEC> {
        BL_W::new(self, 5)
    }
    #[doc = "Bit 7 - Automatic Pad or CRC Stripping"]
    #[inline(always)]
    #[must_use]
    pub fn acs(&mut self) -> ACS_W<MAC_CONFIGURATION_SPEC> {
        ACS_W::new(self, 7)
    }
    #[doc = "Bit 9 - Disable Retry"]
    #[inline(always)]
    #[must_use]
    pub fn dr(&mut self) -> DR_W<MAC_CONFIGURATION_SPEC> {
        DR_W::new(self, 9)
    }
    #[doc = "Bit 10 - Checksum Offload"]
    #[inline(always)]
    #[must_use]
    pub fn ipc(&mut self) -> IPC_W<MAC_CONFIGURATION_SPEC> {
        IPC_W::new(self, 10)
    }
    #[doc = "Bit 11 - Duplex Mode"]
    #[inline(always)]
    #[must_use]
    pub fn dm(&mut self) -> DM_W<MAC_CONFIGURATION_SPEC> {
        DM_W::new(self, 11)
    }
    #[doc = "Bit 12 - Loopback Mode"]
    #[inline(always)]
    #[must_use]
    pub fn lm(&mut self) -> LM_W<MAC_CONFIGURATION_SPEC> {
        LM_W::new(self, 12)
    }
    #[doc = "Bit 13 - Disable Receive Own"]
    #[inline(always)]
    #[must_use]
    pub fn do_(&mut self) -> DO_W<MAC_CONFIGURATION_SPEC> {
        DO_W::new(self, 13)
    }
    #[doc = "Bit 14 - Speed"]
    #[inline(always)]
    #[must_use]
    pub fn fes(&mut self) -> FES_W<MAC_CONFIGURATION_SPEC> {
        FES_W::new(self, 14)
    }
    #[doc = "Bit 16 - Disable Carrier Sense During Transmission"]
    #[inline(always)]
    #[must_use]
    pub fn dcrs(&mut self) -> DCRS_W<MAC_CONFIGURATION_SPEC> {
        DCRS_W::new(self, 16)
    }
    #[doc = "Bits 17:19 - Inter-Frame Gap"]
    #[inline(always)]
    #[must_use]
    pub fn ifg(&mut self) -> IFG_W<MAC_CONFIGURATION_SPEC> {
        IFG_W::new(self, 17)
    }
    #[doc = "Bit 20 - Jumbo Frame Enable"]
    #[inline(always)]
    #[must_use]
    pub fn je(&mut self) -> JE_W<MAC_CONFIGURATION_SPEC> {
        JE_W::new(self, 20)
    }
    #[doc = "Bit 22 - Jabber Disable"]
    #[inline(always)]
    #[must_use]
    pub fn jd(&mut self) -> JD_W<MAC_CONFIGURATION_SPEC> {
        JD_W::new(self, 22)
    }
    #[doc = "Bit 23 - Watchdog Disable"]
    #[inline(always)]
    #[must_use]
    pub fn wd(&mut self) -> WD_W<MAC_CONFIGURATION_SPEC> {
        WD_W::new(self, 23)
    }
    #[doc = "Bit 25 - CRC Stripping of Type Frames"]
    #[inline(always)]
    #[must_use]
    pub fn cst(&mut self) -> CST_W<MAC_CONFIGURATION_SPEC> {
        CST_W::new(self, 25)
    }
    #[doc = "Bit 27 - IEEE 802.3as support for 2K packets Enable"]
    #[inline(always)]
    #[must_use]
    pub fn twokpe(&mut self) -> TWOKPE_W<MAC_CONFIGURATION_SPEC> {
        TWOKPE_W::new(self, 27)
    }
}
#[doc = "MAC Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mac_configuration::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mac_configuration::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MAC_CONFIGURATION_SPEC;
impl crate::RegisterSpec for MAC_CONFIGURATION_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mac_configuration::R`](R) reader structure"]
impl crate::Readable for MAC_CONFIGURATION_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mac_configuration::W`](W) writer structure"]
impl crate::Writable for MAC_CONFIGURATION_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MAC_CONFIGURATION to value 0x8000"]
impl crate::Resettable for MAC_CONFIGURATION_SPEC {
    const RESET_VALUE: u32 = 0x8000;
}
