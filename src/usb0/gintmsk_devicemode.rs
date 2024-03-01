#[doc = "Register `GINTMSK_DEVICEMODE` reader"]
pub type R = crate::R<GintmskDevicemodeSpec>;
#[doc = "Register `GINTMSK_DEVICEMODE` writer"]
pub type W = crate::W<GintmskDevicemodeSpec>;
#[doc = "Field `ModeMisMsk` reader - Mode Mismatch Interrupt Mask"]
pub type ModeMisMskR = crate::BitReader;
#[doc = "Field `ModeMisMsk` writer - Mode Mismatch Interrupt Mask"]
pub type ModeMisMskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OTGIntMsk` reader - OTG Interrupt Mask"]
pub type OtgintMskR = crate::BitReader;
#[doc = "Field `OTGIntMsk` writer - OTG Interrupt Mask"]
pub type OtgintMskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SofMsk` reader - Start of Frame Mask"]
pub type SofMskR = crate::BitReader;
#[doc = "Field `SofMsk` writer - Start of Frame Mask"]
pub type SofMskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RxFLvlMsk` reader - Receive FIFO Non-Empty Mask"]
pub type RxFlvlMskR = crate::BitReader;
#[doc = "Field `RxFLvlMsk` writer - Receive FIFO Non-Empty Mask"]
pub type RxFlvlMskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GINNakEffMsk` reader - Global Non-periodic IN NAK Effective Mask"]
pub type GinnakEffMskR = crate::BitReader;
#[doc = "Field `GINNakEffMsk` writer - Global Non-periodic IN NAK Effective Mask"]
pub type GinnakEffMskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GOUTNakEffMsk` reader - Global OUT NAK Effective Mask"]
pub type GoutnakEffMskR = crate::BitReader;
#[doc = "Field `GOUTNakEffMsk` writer - Global OUT NAK Effective Mask"]
pub type GoutnakEffMskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ErlySuspMsk` reader - Early Suspend Mask"]
pub type ErlySuspMskR = crate::BitReader;
#[doc = "Field `ErlySuspMsk` writer - Early Suspend Mask"]
pub type ErlySuspMskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USBSuspMsk` reader - USB Suspend Mask"]
pub type UsbsuspMskR = crate::BitReader;
#[doc = "Field `USBSuspMsk` writer - USB Suspend Mask"]
pub type UsbsuspMskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USBRstMsk` reader - USB Reset Mask"]
pub type UsbrstMskR = crate::BitReader;
#[doc = "Field `USBRstMsk` writer - USB Reset Mask"]
pub type UsbrstMskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnumDoneMsk` reader - Enumeration Done Mask"]
pub type EnumDoneMskR = crate::BitReader;
#[doc = "Field `EnumDoneMsk` writer - Enumeration Done Mask"]
pub type EnumDoneMskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ISOOutDropMsk` reader - Isochronous OUT Packet Dropped Interrupt Mask"]
pub type IsooutDropMskR = crate::BitReader;
#[doc = "Field `ISOOutDropMsk` writer - Isochronous OUT Packet Dropped Interrupt Mask"]
pub type IsooutDropMskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EOPFMsk` reader - End of Periodic Frame Interrupt Mask"]
pub type EopfmskR = crate::BitReader;
#[doc = "Field `EOPFMsk` writer - End of Periodic Frame Interrupt Mask"]
pub type EopfmskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IEPIntMsk` reader - IN Endpoints Interrupt Mask"]
pub type IepintMskR = crate::BitReader;
#[doc = "Field `IEPIntMsk` writer - IN Endpoints Interrupt Mask"]
pub type IepintMskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OEPIntMsk` reader - OUT Endpoints Interrupt Mask"]
pub type OepintMskR = crate::BitReader;
#[doc = "Field `OEPIntMsk` writer - OUT Endpoints Interrupt Mask"]
pub type OepintMskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `incompISOINMsk` reader - Incomplete Isochronous IN Transfer Mask"]
pub type IncompIsoinmskR = crate::BitReader;
#[doc = "Field `incompISOINMsk` writer - Incomplete Isochronous IN Transfer Mask"]
pub type IncompIsoinmskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `incomplSOOUTMsk` reader - Incomplete Isochronous OUT Transfer Mask"]
pub type IncomplSooutmskR = crate::BitReader;
#[doc = "Field `incomplSOOUTMsk` writer - Incomplete Isochronous OUT Transfer Mask"]
pub type IncomplSooutmskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ConIDStsChngMsk` reader - Connector ID Status Change Mask"]
pub type ConIdstsChngMskR = crate::BitReader;
#[doc = "Field `ConIDStsChngMsk` writer - Connector ID Status Change Mask"]
pub type ConIdstsChngMskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DisconnIntMsk` reader - Disconnect Detected Interrupt Mask"]
pub type DisconnIntMskR = crate::BitReader;
#[doc = "Field `DisconnIntMsk` writer - Disconnect Detected Interrupt Mask"]
pub type DisconnIntMskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SessReqIntMsk` reader - Session Request/New Session Detected Interrupt Mask"]
pub type SessReqIntMskR = crate::BitReader;
#[doc = "Field `SessReqIntMsk` writer - Session Request/New Session Detected Interrupt Mask"]
pub type SessReqIntMskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WkUpIntMsk` reader - Resume/Remote Wakeup Detected Interrupt Mask"]
pub type WkUpIntMskR = crate::BitReader;
#[doc = "Field `WkUpIntMsk` writer - Resume/Remote Wakeup Detected Interrupt Mask"]
pub type WkUpIntMskW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1 - Mode Mismatch Interrupt Mask"]
    #[inline(always)]
    pub fn mode_mis_msk(&self) -> ModeMisMskR {
        ModeMisMskR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - OTG Interrupt Mask"]
    #[inline(always)]
    pub fn otgint_msk(&self) -> OtgintMskR {
        OtgintMskR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Start of Frame Mask"]
    #[inline(always)]
    pub fn sof_msk(&self) -> SofMskR {
        SofMskR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Receive FIFO Non-Empty Mask"]
    #[inline(always)]
    pub fn rx_flvl_msk(&self) -> RxFlvlMskR {
        RxFlvlMskR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - Global Non-periodic IN NAK Effective Mask"]
    #[inline(always)]
    pub fn ginnak_eff_msk(&self) -> GinnakEffMskR {
        GinnakEffMskR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Global OUT NAK Effective Mask"]
    #[inline(always)]
    pub fn goutnak_eff_msk(&self) -> GoutnakEffMskR {
        GoutnakEffMskR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 10 - Early Suspend Mask"]
    #[inline(always)]
    pub fn erly_susp_msk(&self) -> ErlySuspMskR {
        ErlySuspMskR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - USB Suspend Mask"]
    #[inline(always)]
    pub fn usbsusp_msk(&self) -> UsbsuspMskR {
        UsbsuspMskR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - USB Reset Mask"]
    #[inline(always)]
    pub fn usbrst_msk(&self) -> UsbrstMskR {
        UsbrstMskR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Enumeration Done Mask"]
    #[inline(always)]
    pub fn enum_done_msk(&self) -> EnumDoneMskR {
        EnumDoneMskR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Isochronous OUT Packet Dropped Interrupt Mask"]
    #[inline(always)]
    pub fn isoout_drop_msk(&self) -> IsooutDropMskR {
        IsooutDropMskR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - End of Periodic Frame Interrupt Mask"]
    #[inline(always)]
    pub fn eopfmsk(&self) -> EopfmskR {
        EopfmskR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 18 - IN Endpoints Interrupt Mask"]
    #[inline(always)]
    pub fn iepint_msk(&self) -> IepintMskR {
        IepintMskR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - OUT Endpoints Interrupt Mask"]
    #[inline(always)]
    pub fn oepint_msk(&self) -> OepintMskR {
        OepintMskR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Incomplete Isochronous IN Transfer Mask"]
    #[inline(always)]
    pub fn incomp_isoinmsk(&self) -> IncompIsoinmskR {
        IncompIsoinmskR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Incomplete Isochronous OUT Transfer Mask"]
    #[inline(always)]
    pub fn incompl_sooutmsk(&self) -> IncomplSooutmskR {
        IncomplSooutmskR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 28 - Connector ID Status Change Mask"]
    #[inline(always)]
    pub fn con_idsts_chng_msk(&self) -> ConIdstsChngMskR {
        ConIdstsChngMskR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Disconnect Detected Interrupt Mask"]
    #[inline(always)]
    pub fn disconn_int_msk(&self) -> DisconnIntMskR {
        DisconnIntMskR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Session Request/New Session Detected Interrupt Mask"]
    #[inline(always)]
    pub fn sess_req_int_msk(&self) -> SessReqIntMskR {
        SessReqIntMskR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Resume/Remote Wakeup Detected Interrupt Mask"]
    #[inline(always)]
    pub fn wk_up_int_msk(&self) -> WkUpIntMskR {
        WkUpIntMskR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Mode Mismatch Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn mode_mis_msk(&mut self) -> ModeMisMskW<GintmskDevicemodeSpec> {
        ModeMisMskW::new(self, 1)
    }
    #[doc = "Bit 2 - OTG Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn otgint_msk(&mut self) -> OtgintMskW<GintmskDevicemodeSpec> {
        OtgintMskW::new(self, 2)
    }
    #[doc = "Bit 3 - Start of Frame Mask"]
    #[inline(always)]
    #[must_use]
    pub fn sof_msk(&mut self) -> SofMskW<GintmskDevicemodeSpec> {
        SofMskW::new(self, 3)
    }
    #[doc = "Bit 4 - Receive FIFO Non-Empty Mask"]
    #[inline(always)]
    #[must_use]
    pub fn rx_flvl_msk(&mut self) -> RxFlvlMskW<GintmskDevicemodeSpec> {
        RxFlvlMskW::new(self, 4)
    }
    #[doc = "Bit 6 - Global Non-periodic IN NAK Effective Mask"]
    #[inline(always)]
    #[must_use]
    pub fn ginnak_eff_msk(&mut self) -> GinnakEffMskW<GintmskDevicemodeSpec> {
        GinnakEffMskW::new(self, 6)
    }
    #[doc = "Bit 7 - Global OUT NAK Effective Mask"]
    #[inline(always)]
    #[must_use]
    pub fn goutnak_eff_msk(&mut self) -> GoutnakEffMskW<GintmskDevicemodeSpec> {
        GoutnakEffMskW::new(self, 7)
    }
    #[doc = "Bit 10 - Early Suspend Mask"]
    #[inline(always)]
    #[must_use]
    pub fn erly_susp_msk(&mut self) -> ErlySuspMskW<GintmskDevicemodeSpec> {
        ErlySuspMskW::new(self, 10)
    }
    #[doc = "Bit 11 - USB Suspend Mask"]
    #[inline(always)]
    #[must_use]
    pub fn usbsusp_msk(&mut self) -> UsbsuspMskW<GintmskDevicemodeSpec> {
        UsbsuspMskW::new(self, 11)
    }
    #[doc = "Bit 12 - USB Reset Mask"]
    #[inline(always)]
    #[must_use]
    pub fn usbrst_msk(&mut self) -> UsbrstMskW<GintmskDevicemodeSpec> {
        UsbrstMskW::new(self, 12)
    }
    #[doc = "Bit 13 - Enumeration Done Mask"]
    #[inline(always)]
    #[must_use]
    pub fn enum_done_msk(&mut self) -> EnumDoneMskW<GintmskDevicemodeSpec> {
        EnumDoneMskW::new(self, 13)
    }
    #[doc = "Bit 14 - Isochronous OUT Packet Dropped Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn isoout_drop_msk(&mut self) -> IsooutDropMskW<GintmskDevicemodeSpec> {
        IsooutDropMskW::new(self, 14)
    }
    #[doc = "Bit 15 - End of Periodic Frame Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn eopfmsk(&mut self) -> EopfmskW<GintmskDevicemodeSpec> {
        EopfmskW::new(self, 15)
    }
    #[doc = "Bit 18 - IN Endpoints Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn iepint_msk(&mut self) -> IepintMskW<GintmskDevicemodeSpec> {
        IepintMskW::new(self, 18)
    }
    #[doc = "Bit 19 - OUT Endpoints Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn oepint_msk(&mut self) -> OepintMskW<GintmskDevicemodeSpec> {
        OepintMskW::new(self, 19)
    }
    #[doc = "Bit 20 - Incomplete Isochronous IN Transfer Mask"]
    #[inline(always)]
    #[must_use]
    pub fn incomp_isoinmsk(&mut self) -> IncompIsoinmskW<GintmskDevicemodeSpec> {
        IncompIsoinmskW::new(self, 20)
    }
    #[doc = "Bit 21 - Incomplete Isochronous OUT Transfer Mask"]
    #[inline(always)]
    #[must_use]
    pub fn incompl_sooutmsk(&mut self) -> IncomplSooutmskW<GintmskDevicemodeSpec> {
        IncomplSooutmskW::new(self, 21)
    }
    #[doc = "Bit 28 - Connector ID Status Change Mask"]
    #[inline(always)]
    #[must_use]
    pub fn con_idsts_chng_msk(&mut self) -> ConIdstsChngMskW<GintmskDevicemodeSpec> {
        ConIdstsChngMskW::new(self, 28)
    }
    #[doc = "Bit 29 - Disconnect Detected Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn disconn_int_msk(&mut self) -> DisconnIntMskW<GintmskDevicemodeSpec> {
        DisconnIntMskW::new(self, 29)
    }
    #[doc = "Bit 30 - Session Request/New Session Detected Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn sess_req_int_msk(&mut self) -> SessReqIntMskW<GintmskDevicemodeSpec> {
        SessReqIntMskW::new(self, 30)
    }
    #[doc = "Bit 31 - Resume/Remote Wakeup Detected Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn wk_up_int_msk(&mut self) -> WkUpIntMskW<GintmskDevicemodeSpec> {
        WkUpIntMskW::new(self, 31)
    }
}
#[doc = "Interrupt Mask Register \\[DEVICEMODE\\]\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gintmsk_devicemode::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gintmsk_devicemode::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GintmskDevicemodeSpec;
impl crate::RegisterSpec for GintmskDevicemodeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gintmsk_devicemode::R`](R) reader structure"]
impl crate::Readable for GintmskDevicemodeSpec {}
#[doc = "`write(|w| ..)` method takes [`gintmsk_devicemode::W`](W) writer structure"]
impl crate::Writable for GintmskDevicemodeSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GINTMSK_DEVICEMODE to value 0"]
impl crate::Resettable for GintmskDevicemodeSpec {
    const RESET_VALUE: u32 = 0;
}
