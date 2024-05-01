#[doc = "Register `GINTSTS_HOSTMODE` reader"]
pub type R = crate::R<GINTSTS_HOSTMODE_SPEC>;
#[doc = "Register `GINTSTS_HOSTMODE` writer"]
pub type W = crate::W<GINTSTS_HOSTMODE_SPEC>;
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
#[doc = "Field `incomplP` reader - Incomplete Periodic Transfer"]
pub type INCOMPL_P_R = crate::BitReader;
#[doc = "Field `incomplP` writer - Incomplete Periodic Transfer"]
pub type INCOMPL_P_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PrtInt` reader - Host Port Interrupt"]
pub type PRT_INT_R = crate::BitReader;
#[doc = "Field `HChInt` reader - Host Channels Interrupt"]
pub type HCH_INT_R = crate::BitReader;
#[doc = "Field `PTxFEmp` reader - Periodic TxFIFO Empty"]
pub type PTX_FEMP_R = crate::BitReader;
#[doc = "Field `ConIDStsChng` reader - Connector ID Status Change"]
pub type CON_IDSTS_CHNG_R = crate::BitReader;
#[doc = "Field `ConIDStsChng` writer - Connector ID Status Change"]
pub type CON_IDSTS_CHNG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DisconnInt` reader - Disconnect Detected Interrupt"]
pub type DISCONN_INT_R = crate::BitReader;
#[doc = "Field `DisconnInt` writer - Disconnect Detected Interrupt"]
pub type DISCONN_INT_W<'a, REG> = crate::BitWriter<'a, REG>;
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
    #[doc = "Bit 21 - Incomplete Periodic Transfer"]
    #[inline(always)]
    pub fn incompl_p(&self) -> INCOMPL_P_R {
        INCOMPL_P_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 24 - Host Port Interrupt"]
    #[inline(always)]
    pub fn prt_int(&self) -> PRT_INT_R {
        PRT_INT_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Host Channels Interrupt"]
    #[inline(always)]
    pub fn hch_int(&self) -> HCH_INT_R {
        HCH_INT_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Periodic TxFIFO Empty"]
    #[inline(always)]
    pub fn ptx_femp(&self) -> PTX_FEMP_R {
        PTX_FEMP_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 28 - Connector ID Status Change"]
    #[inline(always)]
    pub fn con_idsts_chng(&self) -> CON_IDSTS_CHNG_R {
        CON_IDSTS_CHNG_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Disconnect Detected Interrupt"]
    #[inline(always)]
    pub fn disconn_int(&self) -> DISCONN_INT_R {
        DISCONN_INT_R::new(((self.bits >> 29) & 1) != 0)
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
    pub fn mode_mis(&mut self) -> MODE_MIS_W<GINTSTS_HOSTMODE_SPEC> {
        MODE_MIS_W::new(self, 1)
    }
    #[doc = "Bit 3 - Start of Frame"]
    #[inline(always)]
    #[must_use]
    pub fn sof(&mut self) -> SOF_W<GINTSTS_HOSTMODE_SPEC> {
        SOF_W::new(self, 3)
    }
    #[doc = "Bit 21 - Incomplete Periodic Transfer"]
    #[inline(always)]
    #[must_use]
    pub fn incompl_p(&mut self) -> INCOMPL_P_W<GINTSTS_HOSTMODE_SPEC> {
        INCOMPL_P_W::new(self, 21)
    }
    #[doc = "Bit 28 - Connector ID Status Change"]
    #[inline(always)]
    #[must_use]
    pub fn con_idsts_chng(&mut self) -> CON_IDSTS_CHNG_W<GINTSTS_HOSTMODE_SPEC> {
        CON_IDSTS_CHNG_W::new(self, 28)
    }
    #[doc = "Bit 29 - Disconnect Detected Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn disconn_int(&mut self) -> DISCONN_INT_W<GINTSTS_HOSTMODE_SPEC> {
        DISCONN_INT_W::new(self, 29)
    }
    #[doc = "Bit 30 - Session Request/New Session Detected Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn sess_req_int(&mut self) -> SESS_REQ_INT_W<GINTSTS_HOSTMODE_SPEC> {
        SESS_REQ_INT_W::new(self, 30)
    }
    #[doc = "Bit 31 - Resume/Remote Wakeup Detected Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn wk_up_int(&mut self) -> WK_UP_INT_W<GINTSTS_HOSTMODE_SPEC> {
        WK_UP_INT_W::new(self, 31)
    }
}
#[doc = "Interrupt Register \\[HOSTMODE\\]\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gintsts_hostmode::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gintsts_hostmode::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GINTSTS_HOSTMODE_SPEC;
impl crate::RegisterSpec for GINTSTS_HOSTMODE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gintsts_hostmode::R`](R) reader structure"]
impl crate::Readable for GINTSTS_HOSTMODE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gintsts_hostmode::W`](W) writer structure"]
impl crate::Writable for GINTSTS_HOSTMODE_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GINTSTS_HOSTMODE to value 0x1400_0020"]
impl crate::Resettable for GINTSTS_HOSTMODE_SPEC {
    const RESET_VALUE: u32 = 0x1400_0020;
}
