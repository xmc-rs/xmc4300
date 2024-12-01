#[doc = "Register `GINTMSK_HOSTMODE` reader"]
pub type R = crate::R<GINTMSK_HOSTMODE_SPEC>;
#[doc = "Register `GINTMSK_HOSTMODE` writer"]
pub type W = crate::W<GINTMSK_HOSTMODE_SPEC>;
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
#[doc = "Field `incomplPMsk` reader - Incomplete Periodic Transfer Mask"]
pub type INCOMPL_PMSK_R = crate::BitReader;
#[doc = "Field `incomplPMsk` writer - Incomplete Periodic Transfer Mask"]
pub type INCOMPL_PMSK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PrtIntMsk` reader - Host Port Interrupt Mask"]
pub type PRT_INT_MSK_R = crate::BitReader;
#[doc = "Field `PrtIntMsk` writer - Host Port Interrupt Mask"]
pub type PRT_INT_MSK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HChIntMsk` reader - Host Channels Interrupt Mask"]
pub type HCH_INT_MSK_R = crate::BitReader;
#[doc = "Field `HChIntMsk` writer - Host Channels Interrupt Mask"]
pub type HCH_INT_MSK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PTxFEmpMsk` reader - Periodic TxFIFO Empty Mask"]
pub type PTX_FEMP_MSK_R = crate::BitReader;
#[doc = "Field `PTxFEmpMsk` writer - Periodic TxFIFO Empty Mask"]
pub type PTX_FEMP_MSK_W<'a, REG> = crate::BitWriter<'a, REG>;
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
    #[doc = "Bit 21 - Incomplete Periodic Transfer Mask"]
    #[inline(always)]
    pub fn incompl_pmsk(&self) -> INCOMPL_PMSK_R {
        INCOMPL_PMSK_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 24 - Host Port Interrupt Mask"]
    #[inline(always)]
    pub fn prt_int_msk(&self) -> PRT_INT_MSK_R {
        PRT_INT_MSK_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Host Channels Interrupt Mask"]
    #[inline(always)]
    pub fn hch_int_msk(&self) -> HCH_INT_MSK_R {
        HCH_INT_MSK_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Periodic TxFIFO Empty Mask"]
    #[inline(always)]
    pub fn ptx_femp_msk(&self) -> PTX_FEMP_MSK_R {
        PTX_FEMP_MSK_R::new(((self.bits >> 26) & 1) != 0)
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
    pub fn mode_mis_msk(&mut self) -> MODE_MIS_MSK_W<GINTMSK_HOSTMODE_SPEC> {
        MODE_MIS_MSK_W::new(self, 1)
    }
    #[doc = "Bit 2 - OTG Interrupt Mask"]
    #[inline(always)]
    pub fn otgint_msk(&mut self) -> OTGINT_MSK_W<GINTMSK_HOSTMODE_SPEC> {
        OTGINT_MSK_W::new(self, 2)
    }
    #[doc = "Bit 3 - Start of Frame Mask"]
    #[inline(always)]
    pub fn sof_msk(&mut self) -> SOF_MSK_W<GINTMSK_HOSTMODE_SPEC> {
        SOF_MSK_W::new(self, 3)
    }
    #[doc = "Bit 4 - Receive FIFO Non-Empty Mask"]
    #[inline(always)]
    pub fn rx_flvl_msk(&mut self) -> RX_FLVL_MSK_W<GINTMSK_HOSTMODE_SPEC> {
        RX_FLVL_MSK_W::new(self, 4)
    }
    #[doc = "Bit 21 - Incomplete Periodic Transfer Mask"]
    #[inline(always)]
    pub fn incompl_pmsk(&mut self) -> INCOMPL_PMSK_W<GINTMSK_HOSTMODE_SPEC> {
        INCOMPL_PMSK_W::new(self, 21)
    }
    #[doc = "Bit 24 - Host Port Interrupt Mask"]
    #[inline(always)]
    pub fn prt_int_msk(&mut self) -> PRT_INT_MSK_W<GINTMSK_HOSTMODE_SPEC> {
        PRT_INT_MSK_W::new(self, 24)
    }
    #[doc = "Bit 25 - Host Channels Interrupt Mask"]
    #[inline(always)]
    pub fn hch_int_msk(&mut self) -> HCH_INT_MSK_W<GINTMSK_HOSTMODE_SPEC> {
        HCH_INT_MSK_W::new(self, 25)
    }
    #[doc = "Bit 26 - Periodic TxFIFO Empty Mask"]
    #[inline(always)]
    pub fn ptx_femp_msk(&mut self) -> PTX_FEMP_MSK_W<GINTMSK_HOSTMODE_SPEC> {
        PTX_FEMP_MSK_W::new(self, 26)
    }
    #[doc = "Bit 28 - Connector ID Status Change Mask"]
    #[inline(always)]
    pub fn con_idsts_chng_msk(&mut self) -> CON_IDSTS_CHNG_MSK_W<GINTMSK_HOSTMODE_SPEC> {
        CON_IDSTS_CHNG_MSK_W::new(self, 28)
    }
    #[doc = "Bit 29 - Disconnect Detected Interrupt Mask"]
    #[inline(always)]
    pub fn disconn_int_msk(&mut self) -> DISCONN_INT_MSK_W<GINTMSK_HOSTMODE_SPEC> {
        DISCONN_INT_MSK_W::new(self, 29)
    }
    #[doc = "Bit 30 - Session Request/New Session Detected Interrupt Mask"]
    #[inline(always)]
    pub fn sess_req_int_msk(&mut self) -> SESS_REQ_INT_MSK_W<GINTMSK_HOSTMODE_SPEC> {
        SESS_REQ_INT_MSK_W::new(self, 30)
    }
    #[doc = "Bit 31 - Resume/Remote Wakeup Detected Interrupt Mask"]
    #[inline(always)]
    pub fn wk_up_int_msk(&mut self) -> WK_UP_INT_MSK_W<GINTMSK_HOSTMODE_SPEC> {
        WK_UP_INT_MSK_W::new(self, 31)
    }
}
#[doc = "Interrupt Mask Register \\[HOSTMODE\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`gintmsk_hostmode::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gintmsk_hostmode::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GINTMSK_HOSTMODE_SPEC;
impl crate::RegisterSpec for GINTMSK_HOSTMODE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gintmsk_hostmode::R`](R) reader structure"]
impl crate::Readable for GINTMSK_HOSTMODE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gintmsk_hostmode::W`](W) writer structure"]
impl crate::Writable for GINTMSK_HOSTMODE_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GINTMSK_HOSTMODE to value 0"]
impl crate::Resettable for GINTMSK_HOSTMODE_SPEC {
    const RESET_VALUE: u32 = 0;
}
