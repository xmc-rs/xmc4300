#[doc = "Register `GINTMSK_DEVICEMODE` reader"]
pub type R = crate::R<GINTMSK_DEVICEMODE_SPEC>;
#[doc = "Register `GINTMSK_DEVICEMODE` writer"]
pub type W = crate::W<GINTMSK_DEVICEMODE_SPEC>;
#[doc = "Field `ModeMisMsk` reader - Mode Mismatch Interrupt Mask"]
pub type MODE_MIS_MSK_R = crate::BitReader;
#[doc = "Field `ModeMisMsk` writer - Mode Mismatch Interrupt Mask"]
pub type MODE_MIS_MSK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OTGIntMsk` reader - OTG Interrupt Mask"]
pub type OTGINT_MSK_R = crate::BitReader;
#[doc = "Field `OTGIntMsk` writer - OTG Interrupt Mask"]
pub type OTGINT_MSK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SofMsk` reader - Start of Frame Mask"]
pub type SOF_MSK_R = crate::BitReader;
#[doc = "Field `SofMsk` writer - Start of Frame Mask"]
pub type SOF_MSK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RxFLvlMsk` reader - Receive FIFO Non-Empty Mask"]
pub type RX_FLVL_MSK_R = crate::BitReader;
#[doc = "Field `RxFLvlMsk` writer - Receive FIFO Non-Empty Mask"]
pub type RX_FLVL_MSK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GINNakEffMsk` reader - Global Non-periodic IN NAK Effective Mask"]
pub type GINNAK_EFF_MSK_R = crate::BitReader;
#[doc = "Field `GINNakEffMsk` writer - Global Non-periodic IN NAK Effective Mask"]
pub type GINNAK_EFF_MSK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GOUTNakEffMsk` reader - Global OUT NAK Effective Mask"]
pub type GOUTNAK_EFF_MSK_R = crate::BitReader;
#[doc = "Field `GOUTNakEffMsk` writer - Global OUT NAK Effective Mask"]
pub type GOUTNAK_EFF_MSK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ErlySuspMsk` reader - Early Suspend Mask"]
pub type ERLY_SUSP_MSK_R = crate::BitReader;
#[doc = "Field `ErlySuspMsk` writer - Early Suspend Mask"]
pub type ERLY_SUSP_MSK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USBSuspMsk` reader - USB Suspend Mask"]
pub type USBSUSP_MSK_R = crate::BitReader;
#[doc = "Field `USBSuspMsk` writer - USB Suspend Mask"]
pub type USBSUSP_MSK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USBRstMsk` reader - USB Reset Mask"]
pub type USBRST_MSK_R = crate::BitReader;
#[doc = "Field `USBRstMsk` writer - USB Reset Mask"]
pub type USBRST_MSK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnumDoneMsk` reader - Enumeration Done Mask"]
pub type ENUM_DONE_MSK_R = crate::BitReader;
#[doc = "Field `EnumDoneMsk` writer - Enumeration Done Mask"]
pub type ENUM_DONE_MSK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ISOOutDropMsk` reader - Isochronous OUT Packet Dropped Interrupt Mask"]
pub type ISOOUT_DROP_MSK_R = crate::BitReader;
#[doc = "Field `ISOOutDropMsk` writer - Isochronous OUT Packet Dropped Interrupt Mask"]
pub type ISOOUT_DROP_MSK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EOPFMsk` reader - End of Periodic Frame Interrupt Mask"]
pub type EOPFMSK_R = crate::BitReader;
#[doc = "Field `EOPFMsk` writer - End of Periodic Frame Interrupt Mask"]
pub type EOPFMSK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IEPIntMsk` reader - IN Endpoints Interrupt Mask"]
pub type IEPINT_MSK_R = crate::BitReader;
#[doc = "Field `IEPIntMsk` writer - IN Endpoints Interrupt Mask"]
pub type IEPINT_MSK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OEPIntMsk` reader - OUT Endpoints Interrupt Mask"]
pub type OEPINT_MSK_R = crate::BitReader;
#[doc = "Field `OEPIntMsk` writer - OUT Endpoints Interrupt Mask"]
pub type OEPINT_MSK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `incompISOINMsk` reader - Incomplete Isochronous IN Transfer Mask"]
pub type INCOMP_ISOINMSK_R = crate::BitReader;
#[doc = "Field `incompISOINMsk` writer - Incomplete Isochronous IN Transfer Mask"]
pub type INCOMP_ISOINMSK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `incomplSOOUTMsk` reader - Incomplete Isochronous OUT Transfer Mask"]
pub type INCOMPL_SOOUTMSK_R = crate::BitReader;
#[doc = "Field `incomplSOOUTMsk` writer - Incomplete Isochronous OUT Transfer Mask"]
pub type INCOMPL_SOOUTMSK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ConIDStsChngMsk` reader - Connector ID Status Change Mask"]
pub type CON_IDSTS_CHNG_MSK_R = crate::BitReader;
#[doc = "Field `ConIDStsChngMsk` writer - Connector ID Status Change Mask"]
pub type CON_IDSTS_CHNG_MSK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DisconnIntMsk` reader - Disconnect Detected Interrupt Mask"]
pub type DISCONN_INT_MSK_R = crate::BitReader;
#[doc = "Field `DisconnIntMsk` writer - Disconnect Detected Interrupt Mask"]
pub type DISCONN_INT_MSK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SessReqIntMsk` reader - Session Request/New Session Detected Interrupt Mask"]
pub type SESS_REQ_INT_MSK_R = crate::BitReader;
#[doc = "Field `SessReqIntMsk` writer - Session Request/New Session Detected Interrupt Mask"]
pub type SESS_REQ_INT_MSK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WkUpIntMsk` reader - Resume/Remote Wakeup Detected Interrupt Mask"]
pub type WK_UP_INT_MSK_R = crate::BitReader;
#[doc = "Field `WkUpIntMsk` writer - Resume/Remote Wakeup Detected Interrupt Mask"]
pub type WK_UP_INT_MSK_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1 - Mode Mismatch Interrupt Mask"]
    #[inline(always)]
    pub fn mode_mis_msk(&self) -> MODE_MIS_MSK_R {
        MODE_MIS_MSK_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - OTG Interrupt Mask"]
    #[inline(always)]
    pub fn otgint_msk(&self) -> OTGINT_MSK_R {
        OTGINT_MSK_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Start of Frame Mask"]
    #[inline(always)]
    pub fn sof_msk(&self) -> SOF_MSK_R {
        SOF_MSK_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Receive FIFO Non-Empty Mask"]
    #[inline(always)]
    pub fn rx_flvl_msk(&self) -> RX_FLVL_MSK_R {
        RX_FLVL_MSK_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - Global Non-periodic IN NAK Effective Mask"]
    #[inline(always)]
    pub fn ginnak_eff_msk(&self) -> GINNAK_EFF_MSK_R {
        GINNAK_EFF_MSK_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Global OUT NAK Effective Mask"]
    #[inline(always)]
    pub fn goutnak_eff_msk(&self) -> GOUTNAK_EFF_MSK_R {
        GOUTNAK_EFF_MSK_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 10 - Early Suspend Mask"]
    #[inline(always)]
    pub fn erly_susp_msk(&self) -> ERLY_SUSP_MSK_R {
        ERLY_SUSP_MSK_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - USB Suspend Mask"]
    #[inline(always)]
    pub fn usbsusp_msk(&self) -> USBSUSP_MSK_R {
        USBSUSP_MSK_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - USB Reset Mask"]
    #[inline(always)]
    pub fn usbrst_msk(&self) -> USBRST_MSK_R {
        USBRST_MSK_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Enumeration Done Mask"]
    #[inline(always)]
    pub fn enum_done_msk(&self) -> ENUM_DONE_MSK_R {
        ENUM_DONE_MSK_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Isochronous OUT Packet Dropped Interrupt Mask"]
    #[inline(always)]
    pub fn isoout_drop_msk(&self) -> ISOOUT_DROP_MSK_R {
        ISOOUT_DROP_MSK_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - End of Periodic Frame Interrupt Mask"]
    #[inline(always)]
    pub fn eopfmsk(&self) -> EOPFMSK_R {
        EOPFMSK_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 18 - IN Endpoints Interrupt Mask"]
    #[inline(always)]
    pub fn iepint_msk(&self) -> IEPINT_MSK_R {
        IEPINT_MSK_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - OUT Endpoints Interrupt Mask"]
    #[inline(always)]
    pub fn oepint_msk(&self) -> OEPINT_MSK_R {
        OEPINT_MSK_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Incomplete Isochronous IN Transfer Mask"]
    #[inline(always)]
    pub fn incomp_isoinmsk(&self) -> INCOMP_ISOINMSK_R {
        INCOMP_ISOINMSK_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Incomplete Isochronous OUT Transfer Mask"]
    #[inline(always)]
    pub fn incompl_sooutmsk(&self) -> INCOMPL_SOOUTMSK_R {
        INCOMPL_SOOUTMSK_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 28 - Connector ID Status Change Mask"]
    #[inline(always)]
    pub fn con_idsts_chng_msk(&self) -> CON_IDSTS_CHNG_MSK_R {
        CON_IDSTS_CHNG_MSK_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Disconnect Detected Interrupt Mask"]
    #[inline(always)]
    pub fn disconn_int_msk(&self) -> DISCONN_INT_MSK_R {
        DISCONN_INT_MSK_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Session Request/New Session Detected Interrupt Mask"]
    #[inline(always)]
    pub fn sess_req_int_msk(&self) -> SESS_REQ_INT_MSK_R {
        SESS_REQ_INT_MSK_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Resume/Remote Wakeup Detected Interrupt Mask"]
    #[inline(always)]
    pub fn wk_up_int_msk(&self) -> WK_UP_INT_MSK_R {
        WK_UP_INT_MSK_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Mode Mismatch Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn mode_mis_msk(&mut self) -> MODE_MIS_MSK_W<GINTMSK_DEVICEMODE_SPEC> {
        MODE_MIS_MSK_W::new(self, 1)
    }
    #[doc = "Bit 2 - OTG Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn otgint_msk(&mut self) -> OTGINT_MSK_W<GINTMSK_DEVICEMODE_SPEC> {
        OTGINT_MSK_W::new(self, 2)
    }
    #[doc = "Bit 3 - Start of Frame Mask"]
    #[inline(always)]
    #[must_use]
    pub fn sof_msk(&mut self) -> SOF_MSK_W<GINTMSK_DEVICEMODE_SPEC> {
        SOF_MSK_W::new(self, 3)
    }
    #[doc = "Bit 4 - Receive FIFO Non-Empty Mask"]
    #[inline(always)]
    #[must_use]
    pub fn rx_flvl_msk(&mut self) -> RX_FLVL_MSK_W<GINTMSK_DEVICEMODE_SPEC> {
        RX_FLVL_MSK_W::new(self, 4)
    }
    #[doc = "Bit 6 - Global Non-periodic IN NAK Effective Mask"]
    #[inline(always)]
    #[must_use]
    pub fn ginnak_eff_msk(&mut self) -> GINNAK_EFF_MSK_W<GINTMSK_DEVICEMODE_SPEC> {
        GINNAK_EFF_MSK_W::new(self, 6)
    }
    #[doc = "Bit 7 - Global OUT NAK Effective Mask"]
    #[inline(always)]
    #[must_use]
    pub fn goutnak_eff_msk(&mut self) -> GOUTNAK_EFF_MSK_W<GINTMSK_DEVICEMODE_SPEC> {
        GOUTNAK_EFF_MSK_W::new(self, 7)
    }
    #[doc = "Bit 10 - Early Suspend Mask"]
    #[inline(always)]
    #[must_use]
    pub fn erly_susp_msk(&mut self) -> ERLY_SUSP_MSK_W<GINTMSK_DEVICEMODE_SPEC> {
        ERLY_SUSP_MSK_W::new(self, 10)
    }
    #[doc = "Bit 11 - USB Suspend Mask"]
    #[inline(always)]
    #[must_use]
    pub fn usbsusp_msk(&mut self) -> USBSUSP_MSK_W<GINTMSK_DEVICEMODE_SPEC> {
        USBSUSP_MSK_W::new(self, 11)
    }
    #[doc = "Bit 12 - USB Reset Mask"]
    #[inline(always)]
    #[must_use]
    pub fn usbrst_msk(&mut self) -> USBRST_MSK_W<GINTMSK_DEVICEMODE_SPEC> {
        USBRST_MSK_W::new(self, 12)
    }
    #[doc = "Bit 13 - Enumeration Done Mask"]
    #[inline(always)]
    #[must_use]
    pub fn enum_done_msk(&mut self) -> ENUM_DONE_MSK_W<GINTMSK_DEVICEMODE_SPEC> {
        ENUM_DONE_MSK_W::new(self, 13)
    }
    #[doc = "Bit 14 - Isochronous OUT Packet Dropped Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn isoout_drop_msk(&mut self) -> ISOOUT_DROP_MSK_W<GINTMSK_DEVICEMODE_SPEC> {
        ISOOUT_DROP_MSK_W::new(self, 14)
    }
    #[doc = "Bit 15 - End of Periodic Frame Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn eopfmsk(&mut self) -> EOPFMSK_W<GINTMSK_DEVICEMODE_SPEC> {
        EOPFMSK_W::new(self, 15)
    }
    #[doc = "Bit 18 - IN Endpoints Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn iepint_msk(&mut self) -> IEPINT_MSK_W<GINTMSK_DEVICEMODE_SPEC> {
        IEPINT_MSK_W::new(self, 18)
    }
    #[doc = "Bit 19 - OUT Endpoints Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn oepint_msk(&mut self) -> OEPINT_MSK_W<GINTMSK_DEVICEMODE_SPEC> {
        OEPINT_MSK_W::new(self, 19)
    }
    #[doc = "Bit 20 - Incomplete Isochronous IN Transfer Mask"]
    #[inline(always)]
    #[must_use]
    pub fn incomp_isoinmsk(&mut self) -> INCOMP_ISOINMSK_W<GINTMSK_DEVICEMODE_SPEC> {
        INCOMP_ISOINMSK_W::new(self, 20)
    }
    #[doc = "Bit 21 - Incomplete Isochronous OUT Transfer Mask"]
    #[inline(always)]
    #[must_use]
    pub fn incompl_sooutmsk(&mut self) -> INCOMPL_SOOUTMSK_W<GINTMSK_DEVICEMODE_SPEC> {
        INCOMPL_SOOUTMSK_W::new(self, 21)
    }
    #[doc = "Bit 28 - Connector ID Status Change Mask"]
    #[inline(always)]
    #[must_use]
    pub fn con_idsts_chng_msk(&mut self) -> CON_IDSTS_CHNG_MSK_W<GINTMSK_DEVICEMODE_SPEC> {
        CON_IDSTS_CHNG_MSK_W::new(self, 28)
    }
    #[doc = "Bit 29 - Disconnect Detected Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn disconn_int_msk(&mut self) -> DISCONN_INT_MSK_W<GINTMSK_DEVICEMODE_SPEC> {
        DISCONN_INT_MSK_W::new(self, 29)
    }
    #[doc = "Bit 30 - Session Request/New Session Detected Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn sess_req_int_msk(&mut self) -> SESS_REQ_INT_MSK_W<GINTMSK_DEVICEMODE_SPEC> {
        SESS_REQ_INT_MSK_W::new(self, 30)
    }
    #[doc = "Bit 31 - Resume/Remote Wakeup Detected Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn wk_up_int_msk(&mut self) -> WK_UP_INT_MSK_W<GINTMSK_DEVICEMODE_SPEC> {
        WK_UP_INT_MSK_W::new(self, 31)
    }
}
#[doc = "Interrupt Mask Register \\[DEVICEMODE\\]\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gintmsk_devicemode::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gintmsk_devicemode::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GINTMSK_DEVICEMODE_SPEC;
impl crate::RegisterSpec for GINTMSK_DEVICEMODE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gintmsk_devicemode::R`](R) reader structure"]
impl crate::Readable for GINTMSK_DEVICEMODE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gintmsk_devicemode::W`](W) writer structure"]
impl crate::Writable for GINTMSK_DEVICEMODE_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GINTMSK_DEVICEMODE to value 0"]
impl crate::Resettable for GINTMSK_DEVICEMODE_SPEC {
    const RESET_VALUE: u32 = 0;
}
