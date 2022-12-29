#[doc = "Register `HPRT` reader"]
pub struct R(crate::R<HPRT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HPRT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HPRT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HPRT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HPRT` writer"]
pub struct W(crate::W<HPRT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HPRT_SPEC>;
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
impl From<crate::W<HPRT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HPRT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PrtConnSts` reader - Port Connect Status"]
pub type PRT_CONN_STS_R = crate::BitReader<PRT_CONN_STS_A>;
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
impl PRT_CONN_STS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PRT_CONN_STS_A {
        match self.bits {
            false => PRT_CONN_STS_A::VALUE1,
            true => PRT_CONN_STS_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PRT_CONN_STS_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PRT_CONN_STS_A::VALUE2
    }
}
#[doc = "Field `PrtConnDet` reader - Port Connect Detected"]
pub type PRT_CONN_DET_R = crate::BitReader<bool>;
#[doc = "Field `PrtConnDet` writer - Port Connect Detected"]
pub type PRT_CONN_DET_W<'a, const O: u8> = crate::BitWriter<'a, u32, HPRT_SPEC, bool, O>;
#[doc = "Field `PrtEna` reader - Port Enable"]
pub type PRT_ENA_R = crate::BitReader<PRT_ENA_A>;
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
impl PRT_ENA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PRT_ENA_A {
        match self.bits {
            false => PRT_ENA_A::VALUE1,
            true => PRT_ENA_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PRT_ENA_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PRT_ENA_A::VALUE2
    }
}
#[doc = "Field `PrtEna` writer - Port Enable"]
pub type PRT_ENA_W<'a, const O: u8> = crate::BitWriter<'a, u32, HPRT_SPEC, PRT_ENA_A, O>;
impl<'a, const O: u8> PRT_ENA_W<'a, O> {
    #[doc = "Port disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(PRT_ENA_A::VALUE1)
    }
    #[doc = "Port enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(PRT_ENA_A::VALUE2)
    }
}
#[doc = "Field `PrtEnChng` reader - Port Enable/Disable Change"]
pub type PRT_EN_CHNG_R = crate::BitReader<bool>;
#[doc = "Field `PrtEnChng` writer - Port Enable/Disable Change"]
pub type PRT_EN_CHNG_W<'a, const O: u8> = crate::BitWriter<'a, u32, HPRT_SPEC, bool, O>;
#[doc = "Field `PrtOvrCurrAct` reader - Port Overcurrent Active"]
pub type PRT_OVR_CURR_ACT_R = crate::BitReader<PRT_OVR_CURR_ACT_A>;
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
impl PRT_OVR_CURR_ACT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PRT_OVR_CURR_ACT_A {
        match self.bits {
            false => PRT_OVR_CURR_ACT_A::VALUE1,
            true => PRT_OVR_CURR_ACT_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PRT_OVR_CURR_ACT_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PRT_OVR_CURR_ACT_A::VALUE2
    }
}
#[doc = "Field `PrtOvrCurrChng` reader - Port Overcurrent Change"]
pub type PRT_OVR_CURR_CHNG_R = crate::BitReader<bool>;
#[doc = "Field `PrtOvrCurrChng` writer - Port Overcurrent Change"]
pub type PRT_OVR_CURR_CHNG_W<'a, const O: u8> = crate::BitWriter<'a, u32, HPRT_SPEC, bool, O>;
#[doc = "Field `PrtRes` reader - Port Resume"]
pub type PRT_RES_R = crate::BitReader<PRT_RES_A>;
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
impl PRT_RES_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PRT_RES_A {
        match self.bits {
            false => PRT_RES_A::VALUE1,
            true => PRT_RES_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PRT_RES_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PRT_RES_A::VALUE2
    }
}
#[doc = "Field `PrtRes` writer - Port Resume"]
pub type PRT_RES_W<'a, const O: u8> = crate::BitWriter<'a, u32, HPRT_SPEC, PRT_RES_A, O>;
impl<'a, const O: u8> PRT_RES_W<'a, O> {
    #[doc = "No resume driven"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(PRT_RES_A::VALUE1)
    }
    #[doc = "Resume driven"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(PRT_RES_A::VALUE2)
    }
}
#[doc = "Field `PrtSusp` reader - Port Suspend"]
pub type PRT_SUSP_R = crate::BitReader<PRT_SUSP_A>;
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
impl PRT_SUSP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PRT_SUSP_A {
        match self.bits {
            false => PRT_SUSP_A::VALUE1,
            true => PRT_SUSP_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PRT_SUSP_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PRT_SUSP_A::VALUE2
    }
}
#[doc = "Field `PrtSusp` writer - Port Suspend"]
pub type PRT_SUSP_W<'a, const O: u8> = crate::BitWriter<'a, u32, HPRT_SPEC, PRT_SUSP_A, O>;
impl<'a, const O: u8> PRT_SUSP_W<'a, O> {
    #[doc = "Port not in Suspend mode"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(PRT_SUSP_A::VALUE1)
    }
    #[doc = "Port in Suspend mode"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(PRT_SUSP_A::VALUE2)
    }
}
#[doc = "Field `PrtRst` reader - Port Reset"]
pub type PRT_RST_R = crate::BitReader<PRT_RST_A>;
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
impl PRT_RST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PRT_RST_A {
        match self.bits {
            false => PRT_RST_A::VALUE1,
            true => PRT_RST_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PRT_RST_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PRT_RST_A::VALUE2
    }
}
#[doc = "Field `PrtRst` writer - Port Reset"]
pub type PRT_RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, HPRT_SPEC, PRT_RST_A, O>;
impl<'a, const O: u8> PRT_RST_W<'a, O> {
    #[doc = "Port not in reset"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(PRT_RST_A::VALUE1)
    }
    #[doc = "Port in reset"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(PRT_RST_A::VALUE2)
    }
}
#[doc = "Field `PrtLnSts` reader - Port Line Status"]
pub type PRT_LN_STS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PrtPwr` reader - Port Power"]
pub type PRT_PWR_R = crate::BitReader<PRT_PWR_A>;
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
impl PRT_PWR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PRT_PWR_A {
        match self.bits {
            false => PRT_PWR_A::VALUE1,
            true => PRT_PWR_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PRT_PWR_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PRT_PWR_A::VALUE2
    }
}
#[doc = "Field `PrtPwr` writer - Port Power"]
pub type PRT_PWR_W<'a, const O: u8> = crate::BitWriter<'a, u32, HPRT_SPEC, PRT_PWR_A, O>;
impl<'a, const O: u8> PRT_PWR_W<'a, O> {
    #[doc = "Power off"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(PRT_PWR_A::VALUE1)
    }
    #[doc = "Power on"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(PRT_PWR_A::VALUE2)
    }
}
#[doc = "Field `PrtSpd` reader - Port Speed"]
pub type PRT_SPD_R = crate::FieldReader<u8, PRT_SPD_A>;
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
impl PRT_SPD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PRT_SPD_A> {
        match self.bits {
            1 => Some(PRT_SPD_A::VALUE1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
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
    pub fn prt_conn_det(&mut self) -> PRT_CONN_DET_W<1> {
        PRT_CONN_DET_W::new(self)
    }
    #[doc = "Bit 2 - Port Enable"]
    #[inline(always)]
    #[must_use]
    pub fn prt_ena(&mut self) -> PRT_ENA_W<2> {
        PRT_ENA_W::new(self)
    }
    #[doc = "Bit 3 - Port Enable/Disable Change"]
    #[inline(always)]
    #[must_use]
    pub fn prt_en_chng(&mut self) -> PRT_EN_CHNG_W<3> {
        PRT_EN_CHNG_W::new(self)
    }
    #[doc = "Bit 5 - Port Overcurrent Change"]
    #[inline(always)]
    #[must_use]
    pub fn prt_ovr_curr_chng(&mut self) -> PRT_OVR_CURR_CHNG_W<5> {
        PRT_OVR_CURR_CHNG_W::new(self)
    }
    #[doc = "Bit 6 - Port Resume"]
    #[inline(always)]
    #[must_use]
    pub fn prt_res(&mut self) -> PRT_RES_W<6> {
        PRT_RES_W::new(self)
    }
    #[doc = "Bit 7 - Port Suspend"]
    #[inline(always)]
    #[must_use]
    pub fn prt_susp(&mut self) -> PRT_SUSP_W<7> {
        PRT_SUSP_W::new(self)
    }
    #[doc = "Bit 8 - Port Reset"]
    #[inline(always)]
    #[must_use]
    pub fn prt_rst(&mut self) -> PRT_RST_W<8> {
        PRT_RST_W::new(self)
    }
    #[doc = "Bit 12 - Port Power"]
    #[inline(always)]
    #[must_use]
    pub fn prt_pwr(&mut self) -> PRT_PWR_W<12> {
        PRT_PWR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Host Port Control and Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hprt](index.html) module"]
pub struct HPRT_SPEC;
impl crate::RegisterSpec for HPRT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hprt::R](R) reader structure"]
impl crate::Readable for HPRT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hprt::W](W) writer structure"]
impl crate::Writable for HPRT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HPRT to value 0"]
impl crate::Resettable for HPRT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
