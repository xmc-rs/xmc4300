#[doc = "Register `GINTMSK_HOSTMODE` reader"]
pub type R = crate::R<GintmskHostmodeSpec>;
#[doc = "Register `GINTMSK_HOSTMODE` writer"]
pub type W = crate::W<GintmskHostmodeSpec>;
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
#[doc = "Field `incomplPMsk` reader - Incomplete Periodic Transfer Mask"]
pub type IncomplPmskR = crate::BitReader;
#[doc = "Field `incomplPMsk` writer - Incomplete Periodic Transfer Mask"]
pub type IncomplPmskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PrtIntMsk` reader - Host Port Interrupt Mask"]
pub type PrtIntMskR = crate::BitReader;
#[doc = "Field `PrtIntMsk` writer - Host Port Interrupt Mask"]
pub type PrtIntMskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HChIntMsk` reader - Host Channels Interrupt Mask"]
pub type HchIntMskR = crate::BitReader;
#[doc = "Field `HChIntMsk` writer - Host Channels Interrupt Mask"]
pub type HchIntMskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PTxFEmpMsk` reader - Periodic TxFIFO Empty Mask"]
pub type PtxFempMskR = crate::BitReader;
#[doc = "Field `PTxFEmpMsk` writer - Periodic TxFIFO Empty Mask"]
pub type PtxFempMskW<'a, REG> = crate::BitWriter<'a, REG>;
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
    #[doc = "Bit 21 - Incomplete Periodic Transfer Mask"]
    #[inline(always)]
    pub fn incompl_pmsk(&self) -> IncomplPmskR {
        IncomplPmskR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 24 - Host Port Interrupt Mask"]
    #[inline(always)]
    pub fn prt_int_msk(&self) -> PrtIntMskR {
        PrtIntMskR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Host Channels Interrupt Mask"]
    #[inline(always)]
    pub fn hch_int_msk(&self) -> HchIntMskR {
        HchIntMskR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Periodic TxFIFO Empty Mask"]
    #[inline(always)]
    pub fn ptx_femp_msk(&self) -> PtxFempMskR {
        PtxFempMskR::new(((self.bits >> 26) & 1) != 0)
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
    pub fn mode_mis_msk(&mut self) -> ModeMisMskW<GintmskHostmodeSpec> {
        ModeMisMskW::new(self, 1)
    }
    #[doc = "Bit 2 - OTG Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn otgint_msk(&mut self) -> OtgintMskW<GintmskHostmodeSpec> {
        OtgintMskW::new(self, 2)
    }
    #[doc = "Bit 3 - Start of Frame Mask"]
    #[inline(always)]
    #[must_use]
    pub fn sof_msk(&mut self) -> SofMskW<GintmskHostmodeSpec> {
        SofMskW::new(self, 3)
    }
    #[doc = "Bit 4 - Receive FIFO Non-Empty Mask"]
    #[inline(always)]
    #[must_use]
    pub fn rx_flvl_msk(&mut self) -> RxFlvlMskW<GintmskHostmodeSpec> {
        RxFlvlMskW::new(self, 4)
    }
    #[doc = "Bit 21 - Incomplete Periodic Transfer Mask"]
    #[inline(always)]
    #[must_use]
    pub fn incompl_pmsk(&mut self) -> IncomplPmskW<GintmskHostmodeSpec> {
        IncomplPmskW::new(self, 21)
    }
    #[doc = "Bit 24 - Host Port Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn prt_int_msk(&mut self) -> PrtIntMskW<GintmskHostmodeSpec> {
        PrtIntMskW::new(self, 24)
    }
    #[doc = "Bit 25 - Host Channels Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn hch_int_msk(&mut self) -> HchIntMskW<GintmskHostmodeSpec> {
        HchIntMskW::new(self, 25)
    }
    #[doc = "Bit 26 - Periodic TxFIFO Empty Mask"]
    #[inline(always)]
    #[must_use]
    pub fn ptx_femp_msk(&mut self) -> PtxFempMskW<GintmskHostmodeSpec> {
        PtxFempMskW::new(self, 26)
    }
    #[doc = "Bit 28 - Connector ID Status Change Mask"]
    #[inline(always)]
    #[must_use]
    pub fn con_idsts_chng_msk(&mut self) -> ConIdstsChngMskW<GintmskHostmodeSpec> {
        ConIdstsChngMskW::new(self, 28)
    }
    #[doc = "Bit 29 - Disconnect Detected Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn disconn_int_msk(&mut self) -> DisconnIntMskW<GintmskHostmodeSpec> {
        DisconnIntMskW::new(self, 29)
    }
    #[doc = "Bit 30 - Session Request/New Session Detected Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn sess_req_int_msk(&mut self) -> SessReqIntMskW<GintmskHostmodeSpec> {
        SessReqIntMskW::new(self, 30)
    }
    #[doc = "Bit 31 - Resume/Remote Wakeup Detected Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn wk_up_int_msk(&mut self) -> WkUpIntMskW<GintmskHostmodeSpec> {
        WkUpIntMskW::new(self, 31)
    }
}
#[doc = "Interrupt Mask Register \\[HOSTMODE\\]\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gintmsk_hostmode::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gintmsk_hostmode::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GintmskHostmodeSpec;
impl crate::RegisterSpec for GintmskHostmodeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gintmsk_hostmode::R`](R) reader structure"]
impl crate::Readable for GintmskHostmodeSpec {}
#[doc = "`write(|w| ..)` method takes [`gintmsk_hostmode::W`](W) writer structure"]
impl crate::Writable for GintmskHostmodeSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GINTMSK_HOSTMODE to value 0"]
impl crate::Resettable for GintmskHostmodeSpec {
    const RESET_VALUE: u32 = 0;
}
