#[doc = "Register `GINTMSK_DEVICEMODE` reader"]
pub struct R(crate::R<GINTMSK_DEVICEMODE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GINTMSK_DEVICEMODE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GINTMSK_DEVICEMODE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GINTMSK_DEVICEMODE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GINTMSK_DEVICEMODE` writer"]
pub struct W(crate::W<GINTMSK_DEVICEMODE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GINTMSK_DEVICEMODE_SPEC>;
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
impl From<crate::W<GINTMSK_DEVICEMODE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GINTMSK_DEVICEMODE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ModeMisMsk` reader - Mode Mismatch Interrupt Mask"]
pub type MODE_MIS_MSK_R = crate::BitReader<bool>;
#[doc = "Field `ModeMisMsk` writer - Mode Mismatch Interrupt Mask"]
pub type MODE_MIS_MSK_W<'a, const O: u8> = crate::BitWriter<'a, u32, GINTMSK_DEVICEMODE_SPEC, bool, O>;
#[doc = "Field `OTGIntMsk` reader - OTG Interrupt Mask"]
pub type OTGINT_MSK_R = crate::BitReader<bool>;
#[doc = "Field `OTGIntMsk` writer - OTG Interrupt Mask"]
pub type OTGINT_MSK_W<'a, const O: u8> = crate::BitWriter<'a, u32, GINTMSK_DEVICEMODE_SPEC, bool, O>;
#[doc = "Field `SofMsk` reader - Start of Frame Mask"]
pub type SOF_MSK_R = crate::BitReader<bool>;
#[doc = "Field `SofMsk` writer - Start of Frame Mask"]
pub type SOF_MSK_W<'a, const O: u8> = crate::BitWriter<'a, u32, GINTMSK_DEVICEMODE_SPEC, bool, O>;
#[doc = "Field `RxFLvlMsk` reader - Receive FIFO Non-Empty Mask"]
pub type RX_FLVL_MSK_R = crate::BitReader<bool>;
#[doc = "Field `RxFLvlMsk` writer - Receive FIFO Non-Empty Mask"]
pub type RX_FLVL_MSK_W<'a, const O: u8> = crate::BitWriter<'a, u32, GINTMSK_DEVICEMODE_SPEC, bool, O>;
#[doc = "Field `GINNakEffMsk` reader - Global Non-periodic IN NAK Effective Mask"]
pub type GINNAK_EFF_MSK_R = crate::BitReader<bool>;
#[doc = "Field `GINNakEffMsk` writer - Global Non-periodic IN NAK Effective Mask"]
pub type GINNAK_EFF_MSK_W<'a, const O: u8> = crate::BitWriter<'a, u32, GINTMSK_DEVICEMODE_SPEC, bool, O>;
#[doc = "Field `GOUTNakEffMsk` reader - Global OUT NAK Effective Mask"]
pub type GOUTNAK_EFF_MSK_R = crate::BitReader<bool>;
#[doc = "Field `GOUTNakEffMsk` writer - Global OUT NAK Effective Mask"]
pub type GOUTNAK_EFF_MSK_W<'a, const O: u8> = crate::BitWriter<'a, u32, GINTMSK_DEVICEMODE_SPEC, bool, O>;
#[doc = "Field `ErlySuspMsk` reader - Early Suspend Mask"]
pub type ERLY_SUSP_MSK_R = crate::BitReader<bool>;
#[doc = "Field `ErlySuspMsk` writer - Early Suspend Mask"]
pub type ERLY_SUSP_MSK_W<'a, const O: u8> = crate::BitWriter<'a, u32, GINTMSK_DEVICEMODE_SPEC, bool, O>;
#[doc = "Field `USBSuspMsk` reader - USB Suspend Mask"]
pub type USBSUSP_MSK_R = crate::BitReader<bool>;
#[doc = "Field `USBSuspMsk` writer - USB Suspend Mask"]
pub type USBSUSP_MSK_W<'a, const O: u8> = crate::BitWriter<'a, u32, GINTMSK_DEVICEMODE_SPEC, bool, O>;
#[doc = "Field `USBRstMsk` reader - USB Reset Mask"]
pub type USBRST_MSK_R = crate::BitReader<bool>;
#[doc = "Field `USBRstMsk` writer - USB Reset Mask"]
pub type USBRST_MSK_W<'a, const O: u8> = crate::BitWriter<'a, u32, GINTMSK_DEVICEMODE_SPEC, bool, O>;
#[doc = "Field `EnumDoneMsk` reader - Enumeration Done Mask"]
pub type ENUM_DONE_MSK_R = crate::BitReader<bool>;
#[doc = "Field `EnumDoneMsk` writer - Enumeration Done Mask"]
pub type ENUM_DONE_MSK_W<'a, const O: u8> = crate::BitWriter<'a, u32, GINTMSK_DEVICEMODE_SPEC, bool, O>;
#[doc = "Field `ISOOutDropMsk` reader - Isochronous OUT Packet Dropped Interrupt Mask"]
pub type ISOOUT_DROP_MSK_R = crate::BitReader<bool>;
#[doc = "Field `ISOOutDropMsk` writer - Isochronous OUT Packet Dropped Interrupt Mask"]
pub type ISOOUT_DROP_MSK_W<'a, const O: u8> = crate::BitWriter<'a, u32, GINTMSK_DEVICEMODE_SPEC, bool, O>;
#[doc = "Field `EOPFMsk` reader - End of Periodic Frame Interrupt Mask"]
pub type EOPFMSK_R = crate::BitReader<bool>;
#[doc = "Field `EOPFMsk` writer - End of Periodic Frame Interrupt Mask"]
pub type EOPFMSK_W<'a, const O: u8> = crate::BitWriter<'a, u32, GINTMSK_DEVICEMODE_SPEC, bool, O>;
#[doc = "Field `IEPIntMsk` reader - IN Endpoints Interrupt Mask"]
pub type IEPINT_MSK_R = crate::BitReader<bool>;
#[doc = "Field `IEPIntMsk` writer - IN Endpoints Interrupt Mask"]
pub type IEPINT_MSK_W<'a, const O: u8> = crate::BitWriter<'a, u32, GINTMSK_DEVICEMODE_SPEC, bool, O>;
#[doc = "Field `OEPIntMsk` reader - OUT Endpoints Interrupt Mask"]
pub type OEPINT_MSK_R = crate::BitReader<bool>;
#[doc = "Field `OEPIntMsk` writer - OUT Endpoints Interrupt Mask"]
pub type OEPINT_MSK_W<'a, const O: u8> = crate::BitWriter<'a, u32, GINTMSK_DEVICEMODE_SPEC, bool, O>;
#[doc = "Field `incompISOINMsk` reader - Incomplete Isochronous IN Transfer Mask"]
pub type INCOMP_ISOINMSK_R = crate::BitReader<bool>;
#[doc = "Field `incompISOINMsk` writer - Incomplete Isochronous IN Transfer Mask"]
pub type INCOMP_ISOINMSK_W<'a, const O: u8> = crate::BitWriter<'a, u32, GINTMSK_DEVICEMODE_SPEC, bool, O>;
#[doc = "Field `incomplSOOUTMsk` reader - Incomplete Isochronous OUT Transfer Mask"]
pub type INCOMPL_SOOUTMSK_R = crate::BitReader<bool>;
#[doc = "Field `incomplSOOUTMsk` writer - Incomplete Isochronous OUT Transfer Mask"]
pub type INCOMPL_SOOUTMSK_W<'a, const O: u8> = crate::BitWriter<'a, u32, GINTMSK_DEVICEMODE_SPEC, bool, O>;
#[doc = "Field `ConIDStsChngMsk` reader - Connector ID Status Change Mask"]
pub type CON_IDSTS_CHNG_MSK_R = crate::BitReader<bool>;
#[doc = "Field `ConIDStsChngMsk` writer - Connector ID Status Change Mask"]
pub type CON_IDSTS_CHNG_MSK_W<'a, const O: u8> = crate::BitWriter<'a, u32, GINTMSK_DEVICEMODE_SPEC, bool, O>;
#[doc = "Field `DisconnIntMsk` reader - Disconnect Detected Interrupt Mask"]
pub type DISCONN_INT_MSK_R = crate::BitReader<bool>;
#[doc = "Field `DisconnIntMsk` writer - Disconnect Detected Interrupt Mask"]
pub type DISCONN_INT_MSK_W<'a, const O: u8> = crate::BitWriter<'a, u32, GINTMSK_DEVICEMODE_SPEC, bool, O>;
#[doc = "Field `SessReqIntMsk` reader - Session Request/New Session Detected Interrupt Mask"]
pub type SESS_REQ_INT_MSK_R = crate::BitReader<bool>;
#[doc = "Field `SessReqIntMsk` writer - Session Request/New Session Detected Interrupt Mask"]
pub type SESS_REQ_INT_MSK_W<'a, const O: u8> = crate::BitWriter<'a, u32, GINTMSK_DEVICEMODE_SPEC, bool, O>;
#[doc = "Field `WkUpIntMsk` reader - Resume/Remote Wakeup Detected Interrupt Mask"]
pub type WK_UP_INT_MSK_R = crate::BitReader<bool>;
#[doc = "Field `WkUpIntMsk` writer - Resume/Remote Wakeup Detected Interrupt Mask"]
pub type WK_UP_INT_MSK_W<'a, const O: u8> = crate::BitWriter<'a, u32, GINTMSK_DEVICEMODE_SPEC, bool, O>;
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
    pub fn mode_mis_msk(&mut self) -> MODE_MIS_MSK_W<1> {
        MODE_MIS_MSK_W::new(self)
    }
    #[doc = "Bit 2 - OTG Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn otgint_msk(&mut self) -> OTGINT_MSK_W<2> {
        OTGINT_MSK_W::new(self)
    }
    #[doc = "Bit 3 - Start of Frame Mask"]
    #[inline(always)]
    #[must_use]
    pub fn sof_msk(&mut self) -> SOF_MSK_W<3> {
        SOF_MSK_W::new(self)
    }
    #[doc = "Bit 4 - Receive FIFO Non-Empty Mask"]
    #[inline(always)]
    #[must_use]
    pub fn rx_flvl_msk(&mut self) -> RX_FLVL_MSK_W<4> {
        RX_FLVL_MSK_W::new(self)
    }
    #[doc = "Bit 6 - Global Non-periodic IN NAK Effective Mask"]
    #[inline(always)]
    #[must_use]
    pub fn ginnak_eff_msk(&mut self) -> GINNAK_EFF_MSK_W<6> {
        GINNAK_EFF_MSK_W::new(self)
    }
    #[doc = "Bit 7 - Global OUT NAK Effective Mask"]
    #[inline(always)]
    #[must_use]
    pub fn goutnak_eff_msk(&mut self) -> GOUTNAK_EFF_MSK_W<7> {
        GOUTNAK_EFF_MSK_W::new(self)
    }
    #[doc = "Bit 10 - Early Suspend Mask"]
    #[inline(always)]
    #[must_use]
    pub fn erly_susp_msk(&mut self) -> ERLY_SUSP_MSK_W<10> {
        ERLY_SUSP_MSK_W::new(self)
    }
    #[doc = "Bit 11 - USB Suspend Mask"]
    #[inline(always)]
    #[must_use]
    pub fn usbsusp_msk(&mut self) -> USBSUSP_MSK_W<11> {
        USBSUSP_MSK_W::new(self)
    }
    #[doc = "Bit 12 - USB Reset Mask"]
    #[inline(always)]
    #[must_use]
    pub fn usbrst_msk(&mut self) -> USBRST_MSK_W<12> {
        USBRST_MSK_W::new(self)
    }
    #[doc = "Bit 13 - Enumeration Done Mask"]
    #[inline(always)]
    #[must_use]
    pub fn enum_done_msk(&mut self) -> ENUM_DONE_MSK_W<13> {
        ENUM_DONE_MSK_W::new(self)
    }
    #[doc = "Bit 14 - Isochronous OUT Packet Dropped Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn isoout_drop_msk(&mut self) -> ISOOUT_DROP_MSK_W<14> {
        ISOOUT_DROP_MSK_W::new(self)
    }
    #[doc = "Bit 15 - End of Periodic Frame Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn eopfmsk(&mut self) -> EOPFMSK_W<15> {
        EOPFMSK_W::new(self)
    }
    #[doc = "Bit 18 - IN Endpoints Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn iepint_msk(&mut self) -> IEPINT_MSK_W<18> {
        IEPINT_MSK_W::new(self)
    }
    #[doc = "Bit 19 - OUT Endpoints Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn oepint_msk(&mut self) -> OEPINT_MSK_W<19> {
        OEPINT_MSK_W::new(self)
    }
    #[doc = "Bit 20 - Incomplete Isochronous IN Transfer Mask"]
    #[inline(always)]
    #[must_use]
    pub fn incomp_isoinmsk(&mut self) -> INCOMP_ISOINMSK_W<20> {
        INCOMP_ISOINMSK_W::new(self)
    }
    #[doc = "Bit 21 - Incomplete Isochronous OUT Transfer Mask"]
    #[inline(always)]
    #[must_use]
    pub fn incompl_sooutmsk(&mut self) -> INCOMPL_SOOUTMSK_W<21> {
        INCOMPL_SOOUTMSK_W::new(self)
    }
    #[doc = "Bit 28 - Connector ID Status Change Mask"]
    #[inline(always)]
    #[must_use]
    pub fn con_idsts_chng_msk(&mut self) -> CON_IDSTS_CHNG_MSK_W<28> {
        CON_IDSTS_CHNG_MSK_W::new(self)
    }
    #[doc = "Bit 29 - Disconnect Detected Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn disconn_int_msk(&mut self) -> DISCONN_INT_MSK_W<29> {
        DISCONN_INT_MSK_W::new(self)
    }
    #[doc = "Bit 30 - Session Request/New Session Detected Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn sess_req_int_msk(&mut self) -> SESS_REQ_INT_MSK_W<30> {
        SESS_REQ_INT_MSK_W::new(self)
    }
    #[doc = "Bit 31 - Resume/Remote Wakeup Detected Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn wk_up_int_msk(&mut self) -> WK_UP_INT_MSK_W<31> {
        WK_UP_INT_MSK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Mask Register \\[DEVICEMODE\\]\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gintmsk_devicemode](index.html) module"]
pub struct GINTMSK_DEVICEMODE_SPEC;
impl crate::RegisterSpec for GINTMSK_DEVICEMODE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gintmsk_devicemode::R](R) reader structure"]
impl crate::Readable for GINTMSK_DEVICEMODE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gintmsk_devicemode::W](W) writer structure"]
impl crate::Writable for GINTMSK_DEVICEMODE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GINTMSK_DEVICEMODE to value 0"]
impl crate::Resettable for GINTMSK_DEVICEMODE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
