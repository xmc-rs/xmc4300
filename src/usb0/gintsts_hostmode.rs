#[doc = "Register `GINTSTS_HOSTMODE` reader"]
pub type R = crate::R<GintstsHostmodeSpec>;
#[doc = "Register `GINTSTS_HOSTMODE` writer"]
pub type W = crate::W<GintstsHostmodeSpec>;
#[doc = "Current Mode of Operation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CurMod {
    #[doc = "0: Device mode"]
    Value1 = 0,
    #[doc = "1: Host mode"]
    Value2 = 1,
}
impl From<CurMod> for bool {
    #[inline(always)]
    fn from(variant: CurMod) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CurMod` reader - Current Mode of Operation"]
pub type CurModR = crate::BitReader<CurMod>;
impl CurModR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CurMod {
        match self.bits {
            false => CurMod::Value1,
            true => CurMod::Value2,
        }
    }
    #[doc = "Device mode"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CurMod::Value1
    }
    #[doc = "Host mode"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CurMod::Value2
    }
}
#[doc = "Field `ModeMis` reader - Mode Mismatch Interrupt"]
pub type ModeMisR = crate::BitReader;
#[doc = "Field `ModeMis` writer - Mode Mismatch Interrupt"]
pub type ModeMisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OTGInt` reader - OTG Interrupt"]
pub type OtgintR = crate::BitReader;
#[doc = "Field `Sof` reader - Start of Frame"]
pub type SofR = crate::BitReader;
#[doc = "Field `Sof` writer - Start of Frame"]
pub type SofW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RxFLvl` reader - RxFIFO Non-Empty"]
pub type RxFlvlR = crate::BitReader;
#[doc = "Field `incomplP` reader - Incomplete Periodic Transfer"]
pub type IncomplPR = crate::BitReader;
#[doc = "Field `incomplP` writer - Incomplete Periodic Transfer"]
pub type IncomplPW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PrtInt` reader - Host Port Interrupt"]
pub type PrtIntR = crate::BitReader;
#[doc = "Field `HChInt` reader - Host Channels Interrupt"]
pub type HchIntR = crate::BitReader;
#[doc = "Field `PTxFEmp` reader - Periodic TxFIFO Empty"]
pub type PtxFempR = crate::BitReader;
#[doc = "Field `ConIDStsChng` reader - Connector ID Status Change"]
pub type ConIdstsChngR = crate::BitReader;
#[doc = "Field `ConIDStsChng` writer - Connector ID Status Change"]
pub type ConIdstsChngW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DisconnInt` reader - Disconnect Detected Interrupt"]
pub type DisconnIntR = crate::BitReader;
#[doc = "Field `DisconnInt` writer - Disconnect Detected Interrupt"]
pub type DisconnIntW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SessReqInt` reader - Session Request/New Session Detected Interrupt"]
pub type SessReqIntR = crate::BitReader;
#[doc = "Field `SessReqInt` writer - Session Request/New Session Detected Interrupt"]
pub type SessReqIntW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WkUpInt` reader - Resume/Remote Wakeup Detected Interrupt"]
pub type WkUpIntR = crate::BitReader;
#[doc = "Field `WkUpInt` writer - Resume/Remote Wakeup Detected Interrupt"]
pub type WkUpIntW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Current Mode of Operation"]
    #[inline(always)]
    pub fn cur_mod(&self) -> CurModR {
        CurModR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Mode Mismatch Interrupt"]
    #[inline(always)]
    pub fn mode_mis(&self) -> ModeMisR {
        ModeMisR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - OTG Interrupt"]
    #[inline(always)]
    pub fn otgint(&self) -> OtgintR {
        OtgintR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Start of Frame"]
    #[inline(always)]
    pub fn sof(&self) -> SofR {
        SofR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - RxFIFO Non-Empty"]
    #[inline(always)]
    pub fn rx_flvl(&self) -> RxFlvlR {
        RxFlvlR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 21 - Incomplete Periodic Transfer"]
    #[inline(always)]
    pub fn incompl_p(&self) -> IncomplPR {
        IncomplPR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 24 - Host Port Interrupt"]
    #[inline(always)]
    pub fn prt_int(&self) -> PrtIntR {
        PrtIntR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Host Channels Interrupt"]
    #[inline(always)]
    pub fn hch_int(&self) -> HchIntR {
        HchIntR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Periodic TxFIFO Empty"]
    #[inline(always)]
    pub fn ptx_femp(&self) -> PtxFempR {
        PtxFempR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 28 - Connector ID Status Change"]
    #[inline(always)]
    pub fn con_idsts_chng(&self) -> ConIdstsChngR {
        ConIdstsChngR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Disconnect Detected Interrupt"]
    #[inline(always)]
    pub fn disconn_int(&self) -> DisconnIntR {
        DisconnIntR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Session Request/New Session Detected Interrupt"]
    #[inline(always)]
    pub fn sess_req_int(&self) -> SessReqIntR {
        SessReqIntR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Resume/Remote Wakeup Detected Interrupt"]
    #[inline(always)]
    pub fn wk_up_int(&self) -> WkUpIntR {
        WkUpIntR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Mode Mismatch Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn mode_mis(&mut self) -> ModeMisW<GintstsHostmodeSpec> {
        ModeMisW::new(self, 1)
    }
    #[doc = "Bit 3 - Start of Frame"]
    #[inline(always)]
    #[must_use]
    pub fn sof(&mut self) -> SofW<GintstsHostmodeSpec> {
        SofW::new(self, 3)
    }
    #[doc = "Bit 21 - Incomplete Periodic Transfer"]
    #[inline(always)]
    #[must_use]
    pub fn incompl_p(&mut self) -> IncomplPW<GintstsHostmodeSpec> {
        IncomplPW::new(self, 21)
    }
    #[doc = "Bit 28 - Connector ID Status Change"]
    #[inline(always)]
    #[must_use]
    pub fn con_idsts_chng(&mut self) -> ConIdstsChngW<GintstsHostmodeSpec> {
        ConIdstsChngW::new(self, 28)
    }
    #[doc = "Bit 29 - Disconnect Detected Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn disconn_int(&mut self) -> DisconnIntW<GintstsHostmodeSpec> {
        DisconnIntW::new(self, 29)
    }
    #[doc = "Bit 30 - Session Request/New Session Detected Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn sess_req_int(&mut self) -> SessReqIntW<GintstsHostmodeSpec> {
        SessReqIntW::new(self, 30)
    }
    #[doc = "Bit 31 - Resume/Remote Wakeup Detected Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn wk_up_int(&mut self) -> WkUpIntW<GintstsHostmodeSpec> {
        WkUpIntW::new(self, 31)
    }
}
#[doc = "Interrupt Register \\[HOSTMODE\\]\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gintsts_hostmode::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gintsts_hostmode::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GintstsHostmodeSpec;
impl crate::RegisterSpec for GintstsHostmodeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gintsts_hostmode::R`](R) reader structure"]
impl crate::Readable for GintstsHostmodeSpec {}
#[doc = "`write(|w| ..)` method takes [`gintsts_hostmode::W`](W) writer structure"]
impl crate::Writable for GintstsHostmodeSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GINTSTS_HOSTMODE to value 0x1400_0020"]
impl crate::Resettable for GintstsHostmodeSpec {
    const RESET_VALUE: u32 = 0x1400_0020;
}
