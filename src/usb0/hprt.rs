#[doc = "Register `HPRT` reader"]
pub type R = crate::R<HprtSpec>;
#[doc = "Register `HPRT` writer"]
pub type W = crate::W<HprtSpec>;
#[doc = "Port Connect Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PrtConnSts {
    #[doc = "0: No device is attached to the port."]
    Value1 = 0,
    #[doc = "1: A device is attached to the port."]
    Value2 = 1,
}
impl From<PrtConnSts> for bool {
    #[inline(always)]
    fn from(variant: PrtConnSts) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PrtConnSts` reader - Port Connect Status"]
pub type PrtConnStsR = crate::BitReader<PrtConnSts>;
impl PrtConnStsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PrtConnSts {
        match self.bits {
            false => PrtConnSts::Value1,
            true => PrtConnSts::Value2,
        }
    }
    #[doc = "No device is attached to the port."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PrtConnSts::Value1
    }
    #[doc = "A device is attached to the port."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PrtConnSts::Value2
    }
}
#[doc = "Field `PrtConnDet` reader - Port Connect Detected"]
pub type PrtConnDetR = crate::BitReader;
#[doc = "Field `PrtConnDet` writer - Port Connect Detected"]
pub type PrtConnDetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Port Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PrtEna {
    #[doc = "0: Port disabled"]
    Value1 = 0,
    #[doc = "1: Port enabled"]
    Value2 = 1,
}
impl From<PrtEna> for bool {
    #[inline(always)]
    fn from(variant: PrtEna) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PrtEna` reader - Port Enable"]
pub type PrtEnaR = crate::BitReader<PrtEna>;
impl PrtEnaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PrtEna {
        match self.bits {
            false => PrtEna::Value1,
            true => PrtEna::Value2,
        }
    }
    #[doc = "Port disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PrtEna::Value1
    }
    #[doc = "Port enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PrtEna::Value2
    }
}
#[doc = "Field `PrtEna` writer - Port Enable"]
pub type PrtEnaW<'a, REG> = crate::BitWriter<'a, REG, PrtEna>;
impl<'a, REG> PrtEnaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Port disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(PrtEna::Value1)
    }
    #[doc = "Port enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(PrtEna::Value2)
    }
}
#[doc = "Field `PrtEnChng` reader - Port Enable/Disable Change"]
pub type PrtEnChngR = crate::BitReader;
#[doc = "Field `PrtEnChng` writer - Port Enable/Disable Change"]
pub type PrtEnChngW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Port Overcurrent Active\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PrtOvrCurrAct {
    #[doc = "0: No overcurrent condition"]
    Value1 = 0,
    #[doc = "1: Overcurrent condition"]
    Value2 = 1,
}
impl From<PrtOvrCurrAct> for bool {
    #[inline(always)]
    fn from(variant: PrtOvrCurrAct) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PrtOvrCurrAct` reader - Port Overcurrent Active"]
pub type PrtOvrCurrActR = crate::BitReader<PrtOvrCurrAct>;
impl PrtOvrCurrActR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PrtOvrCurrAct {
        match self.bits {
            false => PrtOvrCurrAct::Value1,
            true => PrtOvrCurrAct::Value2,
        }
    }
    #[doc = "No overcurrent condition"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PrtOvrCurrAct::Value1
    }
    #[doc = "Overcurrent condition"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PrtOvrCurrAct::Value2
    }
}
#[doc = "Field `PrtOvrCurrChng` reader - Port Overcurrent Change"]
pub type PrtOvrCurrChngR = crate::BitReader;
#[doc = "Field `PrtOvrCurrChng` writer - Port Overcurrent Change"]
pub type PrtOvrCurrChngW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Port Resume\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PrtRes {
    #[doc = "0: No resume driven"]
    Value1 = 0,
    #[doc = "1: Resume driven"]
    Value2 = 1,
}
impl From<PrtRes> for bool {
    #[inline(always)]
    fn from(variant: PrtRes) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PrtRes` reader - Port Resume"]
pub type PrtResR = crate::BitReader<PrtRes>;
impl PrtResR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PrtRes {
        match self.bits {
            false => PrtRes::Value1,
            true => PrtRes::Value2,
        }
    }
    #[doc = "No resume driven"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PrtRes::Value1
    }
    #[doc = "Resume driven"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PrtRes::Value2
    }
}
#[doc = "Field `PrtRes` writer - Port Resume"]
pub type PrtResW<'a, REG> = crate::BitWriter<'a, REG, PrtRes>;
impl<'a, REG> PrtResW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No resume driven"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(PrtRes::Value1)
    }
    #[doc = "Resume driven"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(PrtRes::Value2)
    }
}
#[doc = "Port Suspend\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PrtSusp {
    #[doc = "0: Port not in Suspend mode"]
    Value1 = 0,
    #[doc = "1: Port in Suspend mode"]
    Value2 = 1,
}
impl From<PrtSusp> for bool {
    #[inline(always)]
    fn from(variant: PrtSusp) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PrtSusp` reader - Port Suspend"]
pub type PrtSuspR = crate::BitReader<PrtSusp>;
impl PrtSuspR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PrtSusp {
        match self.bits {
            false => PrtSusp::Value1,
            true => PrtSusp::Value2,
        }
    }
    #[doc = "Port not in Suspend mode"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PrtSusp::Value1
    }
    #[doc = "Port in Suspend mode"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PrtSusp::Value2
    }
}
#[doc = "Field `PrtSusp` writer - Port Suspend"]
pub type PrtSuspW<'a, REG> = crate::BitWriter<'a, REG, PrtSusp>;
impl<'a, REG> PrtSuspW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Port not in Suspend mode"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(PrtSusp::Value1)
    }
    #[doc = "Port in Suspend mode"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(PrtSusp::Value2)
    }
}
#[doc = "Port Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PrtRst {
    #[doc = "0: Port not in reset"]
    Value1 = 0,
    #[doc = "1: Port in reset"]
    Value2 = 1,
}
impl From<PrtRst> for bool {
    #[inline(always)]
    fn from(variant: PrtRst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PrtRst` reader - Port Reset"]
pub type PrtRstR = crate::BitReader<PrtRst>;
impl PrtRstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PrtRst {
        match self.bits {
            false => PrtRst::Value1,
            true => PrtRst::Value2,
        }
    }
    #[doc = "Port not in reset"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PrtRst::Value1
    }
    #[doc = "Port in reset"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PrtRst::Value2
    }
}
#[doc = "Field `PrtRst` writer - Port Reset"]
pub type PrtRstW<'a, REG> = crate::BitWriter<'a, REG, PrtRst>;
impl<'a, REG> PrtRstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Port not in reset"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(PrtRst::Value1)
    }
    #[doc = "Port in reset"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(PrtRst::Value2)
    }
}
#[doc = "Field `PrtLnSts` reader - Port Line Status"]
pub type PrtLnStsR = crate::FieldReader;
#[doc = "Port Power\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PrtPwr {
    #[doc = "0: Power off"]
    Value1 = 0,
    #[doc = "1: Power on"]
    Value2 = 1,
}
impl From<PrtPwr> for bool {
    #[inline(always)]
    fn from(variant: PrtPwr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PrtPwr` reader - Port Power"]
pub type PrtPwrR = crate::BitReader<PrtPwr>;
impl PrtPwrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PrtPwr {
        match self.bits {
            false => PrtPwr::Value1,
            true => PrtPwr::Value2,
        }
    }
    #[doc = "Power off"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PrtPwr::Value1
    }
    #[doc = "Power on"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PrtPwr::Value2
    }
}
#[doc = "Field `PrtPwr` writer - Port Power"]
pub type PrtPwrW<'a, REG> = crate::BitWriter<'a, REG, PrtPwr>;
impl<'a, REG> PrtPwrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Power off"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(PrtPwr::Value1)
    }
    #[doc = "Power on"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(PrtPwr::Value2)
    }
}
#[doc = "Port Speed\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PrtSpd {
    #[doc = "1: Full speed"]
    Value1 = 1,
}
impl From<PrtSpd> for u8 {
    #[inline(always)]
    fn from(variant: PrtSpd) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PrtSpd {
    type Ux = u8;
}
impl crate::IsEnum for PrtSpd {}
#[doc = "Field `PrtSpd` reader - Port Speed"]
pub type PrtSpdR = crate::FieldReader<PrtSpd>;
impl PrtSpdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<PrtSpd> {
        match self.bits {
            1 => Some(PrtSpd::Value1),
            _ => None,
        }
    }
    #[doc = "Full speed"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PrtSpd::Value1
    }
}
impl R {
    #[doc = "Bit 0 - Port Connect Status"]
    #[inline(always)]
    pub fn prt_conn_sts(&self) -> PrtConnStsR {
        PrtConnStsR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Port Connect Detected"]
    #[inline(always)]
    pub fn prt_conn_det(&self) -> PrtConnDetR {
        PrtConnDetR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Port Enable"]
    #[inline(always)]
    pub fn prt_ena(&self) -> PrtEnaR {
        PrtEnaR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Port Enable/Disable Change"]
    #[inline(always)]
    pub fn prt_en_chng(&self) -> PrtEnChngR {
        PrtEnChngR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Port Overcurrent Active"]
    #[inline(always)]
    pub fn prt_ovr_curr_act(&self) -> PrtOvrCurrActR {
        PrtOvrCurrActR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Port Overcurrent Change"]
    #[inline(always)]
    pub fn prt_ovr_curr_chng(&self) -> PrtOvrCurrChngR {
        PrtOvrCurrChngR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Port Resume"]
    #[inline(always)]
    pub fn prt_res(&self) -> PrtResR {
        PrtResR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Port Suspend"]
    #[inline(always)]
    pub fn prt_susp(&self) -> PrtSuspR {
        PrtSuspR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Port Reset"]
    #[inline(always)]
    pub fn prt_rst(&self) -> PrtRstR {
        PrtRstR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 10:11 - Port Line Status"]
    #[inline(always)]
    pub fn prt_ln_sts(&self) -> PrtLnStsR {
        PrtLnStsR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bit 12 - Port Power"]
    #[inline(always)]
    pub fn prt_pwr(&self) -> PrtPwrR {
        PrtPwrR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 17:18 - Port Speed"]
    #[inline(always)]
    pub fn prt_spd(&self) -> PrtSpdR {
        PrtSpdR::new(((self.bits >> 17) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 1 - Port Connect Detected"]
    #[inline(always)]
    #[must_use]
    pub fn prt_conn_det(&mut self) -> PrtConnDetW<HprtSpec> {
        PrtConnDetW::new(self, 1)
    }
    #[doc = "Bit 2 - Port Enable"]
    #[inline(always)]
    #[must_use]
    pub fn prt_ena(&mut self) -> PrtEnaW<HprtSpec> {
        PrtEnaW::new(self, 2)
    }
    #[doc = "Bit 3 - Port Enable/Disable Change"]
    #[inline(always)]
    #[must_use]
    pub fn prt_en_chng(&mut self) -> PrtEnChngW<HprtSpec> {
        PrtEnChngW::new(self, 3)
    }
    #[doc = "Bit 5 - Port Overcurrent Change"]
    #[inline(always)]
    #[must_use]
    pub fn prt_ovr_curr_chng(&mut self) -> PrtOvrCurrChngW<HprtSpec> {
        PrtOvrCurrChngW::new(self, 5)
    }
    #[doc = "Bit 6 - Port Resume"]
    #[inline(always)]
    #[must_use]
    pub fn prt_res(&mut self) -> PrtResW<HprtSpec> {
        PrtResW::new(self, 6)
    }
    #[doc = "Bit 7 - Port Suspend"]
    #[inline(always)]
    #[must_use]
    pub fn prt_susp(&mut self) -> PrtSuspW<HprtSpec> {
        PrtSuspW::new(self, 7)
    }
    #[doc = "Bit 8 - Port Reset"]
    #[inline(always)]
    #[must_use]
    pub fn prt_rst(&mut self) -> PrtRstW<HprtSpec> {
        PrtRstW::new(self, 8)
    }
    #[doc = "Bit 12 - Port Power"]
    #[inline(always)]
    #[must_use]
    pub fn prt_pwr(&mut self) -> PrtPwrW<HprtSpec> {
        PrtPwrW::new(self, 12)
    }
}
#[doc = "Host Port Control and Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hprt::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hprt::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HprtSpec;
impl crate::RegisterSpec for HprtSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hprt::R`](R) reader structure"]
impl crate::Readable for HprtSpec {}
#[doc = "`write(|w| ..)` method takes [`hprt::W`](W) writer structure"]
impl crate::Writable for HprtSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HPRT to value 0"]
impl crate::Resettable for HprtSpec {
    const RESET_VALUE: u32 = 0;
}
