#[doc = "Register `GINTSTS_DEVICEMODE` reader"]
pub type R = crate::R<GINTSTS_DEVICEMODE_SPEC>;
#[doc = "Register `GINTSTS_DEVICEMODE` writer"]
pub type W = crate::W<GINTSTS_DEVICEMODE_SPEC>;
#[doc = "Current Mode of Operation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CUR_MOD_A {
    #[doc = "0: Device mode"]
    VALUE1 = 0,
    #[doc = "1: Host mode"]
    VALUE2 = 1,
}
impl From<CUR_MOD_A> for bool {
    #[inline(always)]
    fn from(variant: CUR_MOD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CurMod` reader - Current Mode of Operation"]
pub type CUR_MOD_R = crate::BitReader<CUR_MOD_A>;
impl CUR_MOD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CUR_MOD_A {
        match self.bits {
            false => CUR_MOD_A::VALUE1,
            true => CUR_MOD_A::VALUE2,
        }
    }
    #[doc = "Device mode"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CUR_MOD_A::VALUE1
    }
    #[doc = "Host mode"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CUR_MOD_A::VALUE2
    }
}
#[doc = "Field `ModeMis` reader - Mode Mismatch Interrupt"]
pub type MODE_MIS_R = crate::BitReader;
#[doc = "Field `ModeMis` writer - Mode Mismatch Interrupt"]
pub type MODE_MIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OTGInt` reader - OTG Interrupt"]
pub type OTGINT_R = crate::BitReader;
#[doc = "Field `Sof` reader - Start of Frame"]
pub type SOF_R = crate::BitReader;
#[doc = "Field `Sof` writer - Start of Frame"]
pub type SOF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RxFLvl` reader - RxFIFO Non-Empty"]
pub type RX_FLVL_R = crate::BitReader;
#[doc = "Field `GINNakEff` reader - Global IN Non-Periodic NAK Effective"]
pub type GINNAK_EFF_R = crate::BitReader;
#[doc = "Field `GOUTNakEff` reader - Global OUT NAK Effective"]
pub type GOUTNAK_EFF_R = crate::BitReader;
#[doc = "Field `ErlySusp` reader - Early Suspend"]
pub type ERLY_SUSP_R = crate::BitReader;
#[doc = "Field `ErlySusp` writer - Early Suspend"]
pub type ERLY_SUSP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USBSusp` reader - USB Suspend"]
pub type USBSUSP_R = crate::BitReader;
#[doc = "Field `USBSusp` writer - USB Suspend"]
pub type USBSUSP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USBRst` reader - USB Reset"]
pub type USBRST_R = crate::BitReader;
#[doc = "Field `USBRst` writer - USB Reset"]
pub type USBRST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnumDone` reader - Enumeration Done"]
pub type ENUM_DONE_R = crate::BitReader;
#[doc = "Field `EnumDone` writer - Enumeration Done"]
pub type ENUM_DONE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ISOOutDrop` reader - Isochronous OUT Packet Dropped Interrupt"]
pub type ISOOUT_DROP_R = crate::BitReader;
#[doc = "Field `ISOOutDrop` writer - Isochronous OUT Packet Dropped Interrupt"]
pub type ISOOUT_DROP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EOPF` reader - End of Periodic Frame Interrupt"]
pub type EOPF_R = crate::BitReader;
#[doc = "Field `EOPF` writer - End of Periodic Frame Interrupt"]
pub type EOPF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IEPInt` reader - IN Endpoints Interrupt"]
pub type IEPINT_R = crate::BitReader;
#[doc = "Field `OEPInt` reader - OUT Endpoints Interrupt"]
pub type OEPINT_R = crate::BitReader;
#[doc = "Field `incompISOIN` reader - Incomplete Isochronous IN Transfer"]
pub type INCOMP_ISOIN_R = crate::BitReader;
#[doc = "Field `incompISOIN` writer - Incomplete Isochronous IN Transfer"]
pub type INCOMP_ISOIN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `incomplSOOUT` reader - Incomplete Isochronous OUT Transfer"]
pub type INCOMPL_SOOUT_R = crate::BitReader;
#[doc = "Field `incomplSOOUT` writer - Incomplete Isochronous OUT Transfer"]
pub type INCOMPL_SOOUT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ConIDStsChng` reader - Connector ID Status Change"]
pub type CON_IDSTS_CHNG_R = crate::BitReader;
#[doc = "Field `ConIDStsChng` writer - Connector ID Status Change"]
pub type CON_IDSTS_CHNG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SessReqInt` reader - Session Request/New Session Detected Interrupt"]
pub type SESS_REQ_INT_R = crate::BitReader;
#[doc = "Field `SessReqInt` writer - Session Request/New Session Detected Interrupt"]
pub type SESS_REQ_INT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WkUpInt` reader - Resume/Remote Wakeup Detected Interrupt"]
pub type WK_UP_INT_R = crate::BitReader;
#[doc = "Field `WkUpInt` writer - Resume/Remote Wakeup Detected Interrupt"]
pub type WK_UP_INT_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Current Mode of Operation"]
    #[inline(always)]
    pub fn cur_mod(&self) -> CUR_MOD_R {
        CUR_MOD_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Mode Mismatch Interrupt"]
    #[inline(always)]
    pub fn mode_mis(&self) -> MODE_MIS_R {
        MODE_MIS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - OTG Interrupt"]
    #[inline(always)]
    pub fn otgint(&self) -> OTGINT_R {
        OTGINT_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Start of Frame"]
    #[inline(always)]
    pub fn sof(&self) -> SOF_R {
        SOF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - RxFIFO Non-Empty"]
    #[inline(always)]
    pub fn rx_flvl(&self) -> RX_FLVL_R {
        RX_FLVL_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - Global IN Non-Periodic NAK Effective"]
    #[inline(always)]
    pub fn ginnak_eff(&self) -> GINNAK_EFF_R {
        GINNAK_EFF_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Global OUT NAK Effective"]
    #[inline(always)]
    pub fn goutnak_eff(&self) -> GOUTNAK_EFF_R {
        GOUTNAK_EFF_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 10 - Early Suspend"]
    #[inline(always)]
    pub fn erly_susp(&self) -> ERLY_SUSP_R {
        ERLY_SUSP_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - USB Suspend"]
    #[inline(always)]
    pub fn usbsusp(&self) -> USBSUSP_R {
        USBSUSP_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - USB Reset"]
    #[inline(always)]
    pub fn usbrst(&self) -> USBRST_R {
        USBRST_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Enumeration Done"]
    #[inline(always)]
    pub fn enum_done(&self) -> ENUM_DONE_R {
        ENUM_DONE_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Isochronous OUT Packet Dropped Interrupt"]
    #[inline(always)]
    pub fn isoout_drop(&self) -> ISOOUT_DROP_R {
        ISOOUT_DROP_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - End of Periodic Frame Interrupt"]
    #[inline(always)]
    pub fn eopf(&self) -> EOPF_R {
        EOPF_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 18 - IN Endpoints Interrupt"]
    #[inline(always)]
    pub fn iepint(&self) -> IEPINT_R {
        IEPINT_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - OUT Endpoints Interrupt"]
    #[inline(always)]
    pub fn oepint(&self) -> OEPINT_R {
        OEPINT_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Incomplete Isochronous IN Transfer"]
    #[inline(always)]
    pub fn incomp_isoin(&self) -> INCOMP_ISOIN_R {
        INCOMP_ISOIN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Incomplete Isochronous OUT Transfer"]
    #[inline(always)]
    pub fn incompl_soout(&self) -> INCOMPL_SOOUT_R {
        INCOMPL_SOOUT_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 28 - Connector ID Status Change"]
    #[inline(always)]
    pub fn con_idsts_chng(&self) -> CON_IDSTS_CHNG_R {
        CON_IDSTS_CHNG_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 30 - Session Request/New Session Detected Interrupt"]
    #[inline(always)]
    pub fn sess_req_int(&self) -> SESS_REQ_INT_R {
        SESS_REQ_INT_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Resume/Remote Wakeup Detected Interrupt"]
    #[inline(always)]
    pub fn wk_up_int(&self) -> WK_UP_INT_R {
        WK_UP_INT_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Mode Mismatch Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn mode_mis(&mut self) -> MODE_MIS_W<GINTSTS_DEVICEMODE_SPEC> {
        MODE_MIS_W::new(self, 1)
    }
    #[doc = "Bit 3 - Start of Frame"]
    #[inline(always)]
    #[must_use]
    pub fn sof(&mut self) -> SOF_W<GINTSTS_DEVICEMODE_SPEC> {
        SOF_W::new(self, 3)
    }
    #[doc = "Bit 10 - Early Suspend"]
    #[inline(always)]
    #[must_use]
    pub fn erly_susp(&mut self) -> ERLY_SUSP_W<GINTSTS_DEVICEMODE_SPEC> {
        ERLY_SUSP_W::new(self, 10)
    }
    #[doc = "Bit 11 - USB Suspend"]
    #[inline(always)]
    #[must_use]
    pub fn usbsusp(&mut self) -> USBSUSP_W<GINTSTS_DEVICEMODE_SPEC> {
        USBSUSP_W::new(self, 11)
    }
    #[doc = "Bit 12 - USB Reset"]
    #[inline(always)]
    #[must_use]
    pub fn usbrst(&mut self) -> USBRST_W<GINTSTS_DEVICEMODE_SPEC> {
        USBRST_W::new(self, 12)
    }
    #[doc = "Bit 13 - Enumeration Done"]
    #[inline(always)]
    #[must_use]
    pub fn enum_done(&mut self) -> ENUM_DONE_W<GINTSTS_DEVICEMODE_SPEC> {
        ENUM_DONE_W::new(self, 13)
    }
    #[doc = "Bit 14 - Isochronous OUT Packet Dropped Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn isoout_drop(&mut self) -> ISOOUT_DROP_W<GINTSTS_DEVICEMODE_SPEC> {
        ISOOUT_DROP_W::new(self, 14)
    }
    #[doc = "Bit 15 - End of Periodic Frame Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn eopf(&mut self) -> EOPF_W<GINTSTS_DEVICEMODE_SPEC> {
        EOPF_W::new(self, 15)
    }
    #[doc = "Bit 20 - Incomplete Isochronous IN Transfer"]
    #[inline(always)]
    #[must_use]
    pub fn incomp_isoin(&mut self) -> INCOMP_ISOIN_W<GINTSTS_DEVICEMODE_SPEC> {
        INCOMP_ISOIN_W::new(self, 20)
    }
    #[doc = "Bit 21 - Incomplete Isochronous OUT Transfer"]
    #[inline(always)]
    #[must_use]
    pub fn incompl_soout(&mut self) -> INCOMPL_SOOUT_W<GINTSTS_DEVICEMODE_SPEC> {
        INCOMPL_SOOUT_W::new(self, 21)
    }
    #[doc = "Bit 28 - Connector ID Status Change"]
    #[inline(always)]
    #[must_use]
    pub fn con_idsts_chng(&mut self) -> CON_IDSTS_CHNG_W<GINTSTS_DEVICEMODE_SPEC> {
        CON_IDSTS_CHNG_W::new(self, 28)
    }
    #[doc = "Bit 30 - Session Request/New Session Detected Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn sess_req_int(&mut self) -> SESS_REQ_INT_W<GINTSTS_DEVICEMODE_SPEC> {
        SESS_REQ_INT_W::new(self, 30)
    }
    #[doc = "Bit 31 - Resume/Remote Wakeup Detected Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn wk_up_int(&mut self) -> WK_UP_INT_W<GINTSTS_DEVICEMODE_SPEC> {
        WK_UP_INT_W::new(self, 31)
    }
}
#[doc = "Interrupt Register \\[DEVICEMODE\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`gintsts_devicemode::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gintsts_devicemode::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GINTSTS_DEVICEMODE_SPEC;
impl crate::RegisterSpec for GINTSTS_DEVICEMODE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gintsts_devicemode::R`](R) reader structure"]
impl crate::Readable for GINTSTS_DEVICEMODE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gintsts_devicemode::W`](W) writer structure"]
impl crate::Writable for GINTSTS_DEVICEMODE_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GINTSTS_DEVICEMODE to value 0x1400_0020"]
impl crate::Resettable for GINTSTS_DEVICEMODE_SPEC {
    const RESET_VALUE: u32 = 0x1400_0020;
}
