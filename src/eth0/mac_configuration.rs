#[doc = "Register `MAC_CONFIGURATION` reader"]
pub struct R(crate::R<MAC_CONFIGURATION_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MAC_CONFIGURATION_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MAC_CONFIGURATION_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MAC_CONFIGURATION_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MAC_CONFIGURATION` writer"]
pub struct W(crate::W<MAC_CONFIGURATION_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MAC_CONFIGURATION_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<MAC_CONFIGURATION_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MAC_CONFIGURATION_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PRELEN` reader - Preamble Length for Transmit Frames"]
pub type PRELEN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PRELEN` writer - Preamble Length for Transmit Frames"]
pub type PRELEN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MAC_CONFIGURATION_SPEC, u8, u8, 2, O>;
#[doc = "Field `RE` reader - Receiver Enable"]
pub type RE_R = crate::BitReader<bool>;
#[doc = "Field `RE` writer - Receiver Enable"]
pub type RE_W<'a, const O: u8> = crate::BitWriter<'a, u32, MAC_CONFIGURATION_SPEC, bool, O>;
#[doc = "Field `TE` reader - Transmitter Enable"]
pub type TE_R = crate::BitReader<bool>;
#[doc = "Field `TE` writer - Transmitter Enable"]
pub type TE_W<'a, const O: u8> = crate::BitWriter<'a, u32, MAC_CONFIGURATION_SPEC, bool, O>;
#[doc = "Field `DC` reader - Deferral Check"]
pub type DC_R = crate::BitReader<bool>;
#[doc = "Field `DC` writer - Deferral Check"]
pub type DC_W<'a, const O: u8> = crate::BitWriter<'a, u32, MAC_CONFIGURATION_SPEC, bool, O>;
#[doc = "Field `BL` reader - Back-Off Limit"]
pub type BL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BL` writer - Back-Off Limit"]
pub type BL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MAC_CONFIGURATION_SPEC, u8, u8, 2, O>;
#[doc = "Field `ACS` reader - Automatic Pad or CRC Stripping"]
pub type ACS_R = crate::BitReader<bool>;
#[doc = "Field `ACS` writer - Automatic Pad or CRC Stripping"]
pub type ACS_W<'a, const O: u8> = crate::BitWriter<'a, u32, MAC_CONFIGURATION_SPEC, bool, O>;
#[doc = "Field `DR` reader - Disable Retry"]
pub type DR_R = crate::BitReader<bool>;
#[doc = "Field `DR` writer - Disable Retry"]
pub type DR_W<'a, const O: u8> = crate::BitWriter<'a, u32, MAC_CONFIGURATION_SPEC, bool, O>;
#[doc = "Field `IPC` reader - Checksum Offload"]
pub type IPC_R = crate::BitReader<bool>;
#[doc = "Field `IPC` writer - Checksum Offload"]
pub type IPC_W<'a, const O: u8> = crate::BitWriter<'a, u32, MAC_CONFIGURATION_SPEC, bool, O>;
#[doc = "Field `DM` reader - Duplex Mode"]
pub type DM_R = crate::BitReader<bool>;
#[doc = "Field `DM` writer - Duplex Mode"]
pub type DM_W<'a, const O: u8> = crate::BitWriter<'a, u32, MAC_CONFIGURATION_SPEC, bool, O>;
#[doc = "Field `LM` reader - Loopback Mode"]
pub type LM_R = crate::BitReader<bool>;
#[doc = "Field `LM` writer - Loopback Mode"]
pub type LM_W<'a, const O: u8> = crate::BitWriter<'a, u32, MAC_CONFIGURATION_SPEC, bool, O>;
#[doc = "Field `DO` reader - Disable Receive Own"]
pub type DO_R = crate::BitReader<bool>;
#[doc = "Field `DO` writer - Disable Receive Own"]
pub type DO_W<'a, const O: u8> = crate::BitWriter<'a, u32, MAC_CONFIGURATION_SPEC, bool, O>;
#[doc = "Field `FES` reader - Speed"]
pub type FES_R = crate::BitReader<bool>;
#[doc = "Field `FES` writer - Speed"]
pub type FES_W<'a, const O: u8> = crate::BitWriter<'a, u32, MAC_CONFIGURATION_SPEC, bool, O>;
#[doc = "Field `DCRS` reader - Disable Carrier Sense During Transmission"]
pub type DCRS_R = crate::BitReader<bool>;
#[doc = "Field `DCRS` writer - Disable Carrier Sense During Transmission"]
pub type DCRS_W<'a, const O: u8> = crate::BitWriter<'a, u32, MAC_CONFIGURATION_SPEC, bool, O>;
#[doc = "Field `IFG` reader - Inter-Frame Gap"]
pub type IFG_R = crate::FieldReader<u8, u8>;
#[doc = "Field `IFG` writer - Inter-Frame Gap"]
pub type IFG_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MAC_CONFIGURATION_SPEC, u8, u8, 3, O>;
#[doc = "Field `JE` reader - Jumbo Frame Enable"]
pub type JE_R = crate::BitReader<bool>;
#[doc = "Field `JE` writer - Jumbo Frame Enable"]
pub type JE_W<'a, const O: u8> = crate::BitWriter<'a, u32, MAC_CONFIGURATION_SPEC, bool, O>;
#[doc = "Field `BE` reader - Frame Burst Enable"]
pub type BE_R = crate::BitReader<bool>;
#[doc = "Field `JD` reader - Jabber Disable"]
pub type JD_R = crate::BitReader<bool>;
#[doc = "Field `JD` writer - Jabber Disable"]
pub type JD_W<'a, const O: u8> = crate::BitWriter<'a, u32, MAC_CONFIGURATION_SPEC, bool, O>;
#[doc = "Field `WD` reader - Watchdog Disable"]
pub type WD_R = crate::BitReader<bool>;
#[doc = "Field `WD` writer - Watchdog Disable"]
pub type WD_W<'a, const O: u8> = crate::BitWriter<'a, u32, MAC_CONFIGURATION_SPEC, bool, O>;
#[doc = "Field `TC` reader - Transmit Configuration in RMII"]
pub type TC_R = crate::BitReader<bool>;
#[doc = "Field `CST` reader - CRC Stripping of Type Frames"]
pub type CST_R = crate::BitReader<bool>;
#[doc = "Field `CST` writer - CRC Stripping of Type Frames"]
pub type CST_W<'a, const O: u8> = crate::BitWriter<'a, u32, MAC_CONFIGURATION_SPEC, bool, O>;
#[doc = "Field `TWOKPE` reader - IEEE 802.3as support for 2K packets Enable"]
pub type TWOKPE_R = crate::BitReader<bool>;
#[doc = "Field `TWOKPE` writer - IEEE 802.3as support for 2K packets Enable"]
pub type TWOKPE_W<'a, const O: u8> = crate::BitWriter<'a, u32, MAC_CONFIGURATION_SPEC, bool, O>;
#[doc = "Field `SARC` reader - Source Address Insertion or Replacement Control"]
pub type SARC_R = crate::FieldReader<u8, u8>;
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
    pub fn prelen(&mut self) -> PRELEN_W<0> {
        PRELEN_W::new(self)
    }
    #[doc = "Bit 2 - Receiver Enable"]
    #[inline(always)]
    #[must_use]
    pub fn re(&mut self) -> RE_W<2> {
        RE_W::new(self)
    }
    #[doc = "Bit 3 - Transmitter Enable"]
    #[inline(always)]
    #[must_use]
    pub fn te(&mut self) -> TE_W<3> {
        TE_W::new(self)
    }
    #[doc = "Bit 4 - Deferral Check"]
    #[inline(always)]
    #[must_use]
    pub fn dc(&mut self) -> DC_W<4> {
        DC_W::new(self)
    }
    #[doc = "Bits 5:6 - Back-Off Limit"]
    #[inline(always)]
    #[must_use]
    pub fn bl(&mut self) -> BL_W<5> {
        BL_W::new(self)
    }
    #[doc = "Bit 7 - Automatic Pad or CRC Stripping"]
    #[inline(always)]
    #[must_use]
    pub fn acs(&mut self) -> ACS_W<7> {
        ACS_W::new(self)
    }
    #[doc = "Bit 9 - Disable Retry"]
    #[inline(always)]
    #[must_use]
    pub fn dr(&mut self) -> DR_W<9> {
        DR_W::new(self)
    }
    #[doc = "Bit 10 - Checksum Offload"]
    #[inline(always)]
    #[must_use]
    pub fn ipc(&mut self) -> IPC_W<10> {
        IPC_W::new(self)
    }
    #[doc = "Bit 11 - Duplex Mode"]
    #[inline(always)]
    #[must_use]
    pub fn dm(&mut self) -> DM_W<11> {
        DM_W::new(self)
    }
    #[doc = "Bit 12 - Loopback Mode"]
    #[inline(always)]
    #[must_use]
    pub fn lm(&mut self) -> LM_W<12> {
        LM_W::new(self)
    }
    #[doc = "Bit 13 - Disable Receive Own"]
    #[inline(always)]
    #[must_use]
    pub fn do_(&mut self) -> DO_W<13> {
        DO_W::new(self)
    }
    #[doc = "Bit 14 - Speed"]
    #[inline(always)]
    #[must_use]
    pub fn fes(&mut self) -> FES_W<14> {
        FES_W::new(self)
    }
    #[doc = "Bit 16 - Disable Carrier Sense During Transmission"]
    #[inline(always)]
    #[must_use]
    pub fn dcrs(&mut self) -> DCRS_W<16> {
        DCRS_W::new(self)
    }
    #[doc = "Bits 17:19 - Inter-Frame Gap"]
    #[inline(always)]
    #[must_use]
    pub fn ifg(&mut self) -> IFG_W<17> {
        IFG_W::new(self)
    }
    #[doc = "Bit 20 - Jumbo Frame Enable"]
    #[inline(always)]
    #[must_use]
    pub fn je(&mut self) -> JE_W<20> {
        JE_W::new(self)
    }
    #[doc = "Bit 22 - Jabber Disable"]
    #[inline(always)]
    #[must_use]
    pub fn jd(&mut self) -> JD_W<22> {
        JD_W::new(self)
    }
    #[doc = "Bit 23 - Watchdog Disable"]
    #[inline(always)]
    #[must_use]
    pub fn wd(&mut self) -> WD_W<23> {
        WD_W::new(self)
    }
    #[doc = "Bit 25 - CRC Stripping of Type Frames"]
    #[inline(always)]
    #[must_use]
    pub fn cst(&mut self) -> CST_W<25> {
        CST_W::new(self)
    }
    #[doc = "Bit 27 - IEEE 802.3as support for 2K packets Enable"]
    #[inline(always)]
    #[must_use]
    pub fn twokpe(&mut self) -> TWOKPE_W<27> {
        TWOKPE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MAC Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mac_configuration](index.html) module"]
pub struct MAC_CONFIGURATION_SPEC;
impl crate::RegisterSpec for MAC_CONFIGURATION_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mac_configuration::R](R) reader structure"]
impl crate::Readable for MAC_CONFIGURATION_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mac_configuration::W](W) writer structure"]
impl crate::Writable for MAC_CONFIGURATION_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MAC_CONFIGURATION to value 0x8000"]
impl crate::Resettable for MAC_CONFIGURATION_SPEC {
    const RESET_VALUE: Self::Ux = 0x8000;
}
