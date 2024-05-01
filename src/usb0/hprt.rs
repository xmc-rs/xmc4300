#[doc = "Register `HPRT` reader"]
pub type R = crate::R<HPRT_SPEC>;
#[doc = "Register `HPRT` writer"]
pub type W = crate::W<HPRT_SPEC>;
#[doc = "Port Connect Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PRT_CONN_STS_A {
    #[doc = "0: No device is attached to the port."]
    VALUE1 = 0,
    #[doc = "1: A device is attached to the port."]
    VALUE2 = 1,
}
impl From<PRT_CONN_STS_A> for bool {
    #[inline(always)]
    fn from(variant: PRT_CONN_STS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PrtConnSts` reader - Port Connect Status"]
pub type PRT_CONN_STS_R = crate::BitReader<PRT_CONN_STS_A>;
impl PRT_CONN_STS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PRT_CONN_STS_A {
        match self.bits {
            false => PRT_CONN_STS_A::VALUE1,
            true => PRT_CONN_STS_A::VALUE2,
        }
    }
    #[doc = "No device is attached to the port."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PRT_CONN_STS_A::VALUE1
    }
    #[doc = "A device is attached to the port."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PRT_CONN_STS_A::VALUE2
    }
}
#[doc = "Field `PrtConnDet` reader - Port Connect Detected"]
pub type PRT_CONN_DET_R = crate::BitReader;
#[doc = "Field `PrtConnDet` writer - Port Connect Detected"]
pub type PRT_CONN_DET_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Port Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PRT_ENA_A {
    #[doc = "0: Port disabled"]
    VALUE1 = 0,
    #[doc = "1: Port enabled"]
    VALUE2 = 1,
}
impl From<PRT_ENA_A> for bool {
    #[inline(always)]
    fn from(variant: PRT_ENA_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PrtEna` reader - Port Enable"]
pub type PRT_ENA_R = crate::BitReader<PRT_ENA_A>;
impl PRT_ENA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PRT_ENA_A {
        match self.bits {
            false => PRT_ENA_A::VALUE1,
            true => PRT_ENA_A::VALUE2,
        }
    }
    #[doc = "Port disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PRT_ENA_A::VALUE1
    }
    #[doc = "Port enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PRT_ENA_A::VALUE2
    }
}
#[doc = "Field `PrtEna` writer - Port Enable"]
pub type PRT_ENA_W<'a, REG> = crate::BitWriter<'a, REG, PRT_ENA_A>;
impl<'a, REG> PRT_ENA_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Port disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(PRT_ENA_A::VALUE1)
    }
    #[doc = "Port enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(PRT_ENA_A::VALUE2)
    }
}
#[doc = "Field `PrtEnChng` reader - Port Enable/Disable Change"]
pub type PRT_EN_CHNG_R = crate::BitReader;
#[doc = "Field `PrtEnChng` writer - Port Enable/Disable Change"]
pub type PRT_EN_CHNG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Port Overcurrent Active\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PRT_OVR_CURR_ACT_A {
    #[doc = "0: No overcurrent condition"]
    VALUE1 = 0,
    #[doc = "1: Overcurrent condition"]
    VALUE2 = 1,
}
impl From<PRT_OVR_CURR_ACT_A> for bool {
    #[inline(always)]
    fn from(variant: PRT_OVR_CURR_ACT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PrtOvrCurrAct` reader - Port Overcurrent Active"]
pub type PRT_OVR_CURR_ACT_R = crate::BitReader<PRT_OVR_CURR_ACT_A>;
impl PRT_OVR_CURR_ACT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PRT_OVR_CURR_ACT_A {
        match self.bits {
            false => PRT_OVR_CURR_ACT_A::VALUE1,
            true => PRT_OVR_CURR_ACT_A::VALUE2,
        }
    }
    #[doc = "No overcurrent condition"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PRT_OVR_CURR_ACT_A::VALUE1
    }
    #[doc = "Overcurrent condition"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PRT_OVR_CURR_ACT_A::VALUE2
    }
}
#[doc = "Field `PrtOvrCurrChng` reader - Port Overcurrent Change"]
pub type PRT_OVR_CURR_CHNG_R = crate::BitReader;
#[doc = "Field `PrtOvrCurrChng` writer - Port Overcurrent Change"]
pub type PRT_OVR_CURR_CHNG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Port Resume\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PRT_RES_A {
    #[doc = "0: No resume driven"]
    VALUE1 = 0,
    #[doc = "1: Resume driven"]
    VALUE2 = 1,
}
impl From<PRT_RES_A> for bool {
    #[inline(always)]
    fn from(variant: PRT_RES_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PrtRes` reader - Port Resume"]
pub type PRT_RES_R = crate::BitReader<PRT_RES_A>;
impl PRT_RES_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PRT_RES_A {
        match self.bits {
            false => PRT_RES_A::VALUE1,
            true => PRT_RES_A::VALUE2,
        }
    }
    #[doc = "No resume driven"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PRT_RES_A::VALUE1
    }
    #[doc = "Resume driven"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PRT_RES_A::VALUE2
    }
}
#[doc = "Field `PrtRes` writer - Port Resume"]
pub type PRT_RES_W<'a, REG> = crate::BitWriter<'a, REG, PRT_RES_A>;
impl<'a, REG> PRT_RES_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No resume driven"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(PRT_RES_A::VALUE1)
    }
    #[doc = "Resume driven"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(PRT_RES_A::VALUE2)
    }
}
#[doc = "Port Suspend\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PRT_SUSP_A {
    #[doc = "0: Port not in Suspend mode"]
    VALUE1 = 0,
    #[doc = "1: Port in Suspend mode"]
    VALUE2 = 1,
}
impl From<PRT_SUSP_A> for bool {
    #[inline(always)]
    fn from(variant: PRT_SUSP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PrtSusp` reader - Port Suspend"]
pub type PRT_SUSP_R = crate::BitReader<PRT_SUSP_A>;
impl PRT_SUSP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PRT_SUSP_A {
        match self.bits {
            false => PRT_SUSP_A::VALUE1,
            true => PRT_SUSP_A::VALUE2,
        }
    }
    #[doc = "Port not in Suspend mode"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PRT_SUSP_A::VALUE1
    }
    #[doc = "Port in Suspend mode"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PRT_SUSP_A::VALUE2
    }
}
#[doc = "Field `PrtSusp` writer - Port Suspend"]
pub type PRT_SUSP_W<'a, REG> = crate::BitWriter<'a, REG, PRT_SUSP_A>;
impl<'a, REG> PRT_SUSP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Port not in Suspend mode"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(PRT_SUSP_A::VALUE1)
    }
    #[doc = "Port in Suspend mode"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(PRT_SUSP_A::VALUE2)
    }
}
#[doc = "Port Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PRT_RST_A {
    #[doc = "0: Port not in reset"]
    VALUE1 = 0,
    #[doc = "1: Port in reset"]
    VALUE2 = 1,
}
impl From<PRT_RST_A> for bool {
    #[inline(always)]
    fn from(variant: PRT_RST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PrtRst` reader - Port Reset"]
pub type PRT_RST_R = crate::BitReader<PRT_RST_A>;
impl PRT_RST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PRT_RST_A {
        match self.bits {
            false => PRT_RST_A::VALUE1,
            true => PRT_RST_A::VALUE2,
        }
    }
    #[doc = "Port not in reset"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PRT_RST_A::VALUE1
    }
    #[doc = "Port in reset"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PRT_RST_A::VALUE2
    }
}
#[doc = "Field `PrtRst` writer - Port Reset"]
pub type PRT_RST_W<'a, REG> = crate::BitWriter<'a, REG, PRT_RST_A>;
impl<'a, REG> PRT_RST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Port not in reset"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(PRT_RST_A::VALUE1)
    }
    #[doc = "Port in reset"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(PRT_RST_A::VALUE2)
    }
}
#[doc = "Field `PrtLnSts` reader - Port Line Status"]
pub type PRT_LN_STS_R = crate::FieldReader;
#[doc = "Port Power\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PRT_PWR_A {
    #[doc = "0: Power off"]
    VALUE1 = 0,
    #[doc = "1: Power on"]
    VALUE2 = 1,
}
impl From<PRT_PWR_A> for bool {
    #[inline(always)]
    fn from(variant: PRT_PWR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PrtPwr` reader - Port Power"]
pub type PRT_PWR_R = crate::BitReader<PRT_PWR_A>;
impl PRT_PWR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PRT_PWR_A {
        match self.bits {
            false => PRT_PWR_A::VALUE1,
            true => PRT_PWR_A::VALUE2,
        }
    }
    #[doc = "Power off"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PRT_PWR_A::VALUE1
    }
    #[doc = "Power on"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PRT_PWR_A::VALUE2
    }
}
#[doc = "Field `PrtPwr` writer - Port Power"]
pub type PRT_PWR_W<'a, REG> = crate::BitWriter<'a, REG, PRT_PWR_A>;
impl<'a, REG> PRT_PWR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Power off"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(PRT_PWR_A::VALUE1)
    }
    #[doc = "Power on"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(PRT_PWR_A::VALUE2)
    }
}
#[doc = "Port Speed\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PRT_SPD_A {
    #[doc = "1: Full speed"]
    VALUE1 = 1,
}
impl From<PRT_SPD_A> for u8 {
    #[inline(always)]
    fn from(variant: PRT_SPD_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PRT_SPD_A {
    type Ux = u8;
}
impl crate::IsEnum for PRT_SPD_A {}
#[doc = "Field `PrtSpd` reader - Port Speed"]
pub type PRT_SPD_R = crate::FieldReader<PRT_SPD_A>;
impl PRT_SPD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<PRT_SPD_A> {
        match self.bits {
            1 => Some(PRT_SPD_A::VALUE1),
            _ => None,
        }
    }
    #[doc = "Full speed"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PRT_SPD_A::VALUE1
    }
}
impl R {
    #[doc = "Bit 0 - Port Connect Status"]
    #[inline(always)]
    pub fn prt_conn_sts(&self) -> PRT_CONN_STS_R {
        PRT_CONN_STS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Port Connect Detected"]
    #[inline(always)]
    pub fn prt_conn_det(&self) -> PRT_CONN_DET_R {
        PRT_CONN_DET_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Port Enable"]
    #[inline(always)]
    pub fn prt_ena(&self) -> PRT_ENA_R {
        PRT_ENA_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Port Enable/Disable Change"]
    #[inline(always)]
    pub fn prt_en_chng(&self) -> PRT_EN_CHNG_R {
        PRT_EN_CHNG_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Port Overcurrent Active"]
    #[inline(always)]
    pub fn prt_ovr_curr_act(&self) -> PRT_OVR_CURR_ACT_R {
        PRT_OVR_CURR_ACT_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Port Overcurrent Change"]
    #[inline(always)]
    pub fn prt_ovr_curr_chng(&self) -> PRT_OVR_CURR_CHNG_R {
        PRT_OVR_CURR_CHNG_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Port Resume"]
    #[inline(always)]
    pub fn prt_res(&self) -> PRT_RES_R {
        PRT_RES_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Port Suspend"]
    #[inline(always)]
    pub fn prt_susp(&self) -> PRT_SUSP_R {
        PRT_SUSP_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Port Reset"]
    #[inline(always)]
    pub fn prt_rst(&self) -> PRT_RST_R {
        PRT_RST_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 10:11 - Port Line Status"]
    #[inline(always)]
    pub fn prt_ln_sts(&self) -> PRT_LN_STS_R {
        PRT_LN_STS_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bit 12 - Port Power"]
    #[inline(always)]
    pub fn prt_pwr(&self) -> PRT_PWR_R {
        PRT_PWR_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 17:18 - Port Speed"]
    #[inline(always)]
    pub fn prt_spd(&self) -> PRT_SPD_R {
        PRT_SPD_R::new(((self.bits >> 17) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 1 - Port Connect Detected"]
    #[inline(always)]
    #[must_use]
    pub fn prt_conn_det(&mut self) -> PRT_CONN_DET_W<HPRT_SPEC> {
        PRT_CONN_DET_W::new(self, 1)
    }
    #[doc = "Bit 2 - Port Enable"]
    #[inline(always)]
    #[must_use]
    pub fn prt_ena(&mut self) -> PRT_ENA_W<HPRT_SPEC> {
        PRT_ENA_W::new(self, 2)
    }
    #[doc = "Bit 3 - Port Enable/Disable Change"]
    #[inline(always)]
    #[must_use]
    pub fn prt_en_chng(&mut self) -> PRT_EN_CHNG_W<HPRT_SPEC> {
        PRT_EN_CHNG_W::new(self, 3)
    }
    #[doc = "Bit 5 - Port Overcurrent Change"]
    #[inline(always)]
    #[must_use]
    pub fn prt_ovr_curr_chng(&mut self) -> PRT_OVR_CURR_CHNG_W<HPRT_SPEC> {
        PRT_OVR_CURR_CHNG_W::new(self, 5)
    }
    #[doc = "Bit 6 - Port Resume"]
    #[inline(always)]
    #[must_use]
    pub fn prt_res(&mut self) -> PRT_RES_W<HPRT_SPEC> {
        PRT_RES_W::new(self, 6)
    }
    #[doc = "Bit 7 - Port Suspend"]
    #[inline(always)]
    #[must_use]
    pub fn prt_susp(&mut self) -> PRT_SUSP_W<HPRT_SPEC> {
        PRT_SUSP_W::new(self, 7)
    }
    #[doc = "Bit 8 - Port Reset"]
    #[inline(always)]
    #[must_use]
    pub fn prt_rst(&mut self) -> PRT_RST_W<HPRT_SPEC> {
        PRT_RST_W::new(self, 8)
    }
    #[doc = "Bit 12 - Port Power"]
    #[inline(always)]
    #[must_use]
    pub fn prt_pwr(&mut self) -> PRT_PWR_W<HPRT_SPEC> {
        PRT_PWR_W::new(self, 12)
    }
}
#[doc = "Host Port Control and Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hprt::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hprt::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HPRT_SPEC;
impl crate::RegisterSpec for HPRT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hprt::R`](R) reader structure"]
impl crate::Readable for HPRT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hprt::W`](W) writer structure"]
impl crate::Writable for HPRT_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HPRT to value 0"]
impl crate::Resettable for HPRT_SPEC {
    const RESET_VALUE: u32 = 0;
}
