#[doc = "Register `GOTGCTL` reader"]
pub struct R(crate::R<GOTGCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GOTGCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GOTGCTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GOTGCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GOTGCTL` writer"]
pub struct W(crate::W<GOTGCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GOTGCTL_SPEC>;
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
impl From<crate::W<GOTGCTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GOTGCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SesReqScs` reader - Session Request Success"]
pub type SES_REQ_SCS_R = crate::BitReader<SES_REQ_SCS_A>;
#[doc = "Session Request Success\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SES_REQ_SCS_A {
    #[doc = "0: Session request failure"]
    VALUE1 = 0,
    #[doc = "1: Session request success"]
    VALUE2 = 1,
}
impl From<SES_REQ_SCS_A> for bool {
    #[inline(always)]
    fn from(variant: SES_REQ_SCS_A) -> Self {
        variant as u8 != 0
    }
}
impl SES_REQ_SCS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SES_REQ_SCS_A {
        match self.bits {
            false => SES_REQ_SCS_A::VALUE1,
            true => SES_REQ_SCS_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SES_REQ_SCS_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SES_REQ_SCS_A::VALUE2
    }
}
#[doc = "Field `SesReq` reader - Session Request"]
pub type SES_REQ_R = crate::BitReader<SES_REQ_A>;
#[doc = "Session Request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SES_REQ_A {
    #[doc = "0: No session request"]
    VALUE1 = 0,
    #[doc = "1: Session request"]
    VALUE2 = 1,
}
impl From<SES_REQ_A> for bool {
    #[inline(always)]
    fn from(variant: SES_REQ_A) -> Self {
        variant as u8 != 0
    }
}
impl SES_REQ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SES_REQ_A {
        match self.bits {
            false => SES_REQ_A::VALUE1,
            true => SES_REQ_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SES_REQ_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SES_REQ_A::VALUE2
    }
}
#[doc = "Field `SesReq` writer - Session Request"]
pub type SES_REQ_W<'a, const O: u8> = crate::BitWriter<'a, u32, GOTGCTL_SPEC, SES_REQ_A, O>;
impl<'a, const O: u8> SES_REQ_W<'a, O> {
    #[doc = "No session request"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(SES_REQ_A::VALUE1)
    }
    #[doc = "Session request"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(SES_REQ_A::VALUE2)
    }
}
#[doc = "Field `VbvalidOvEn` reader - VBUS Valid Override Enable"]
pub type VBVALID_OV_EN_R = crate::BitReader<VBVALID_OV_EN_A>;
#[doc = "VBUS Valid Override Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VBVALID_OV_EN_A {
    #[doc = "0: Override is disabled and vbus valid signal from the PHY is used internally by the core."]
    VALUE1 = 0,
    #[doc = "1: Internally vbus valid received from the PHY is overridden with GOTGCTL.VbvalidOvVal."]
    VALUE2 = 1,
}
impl From<VBVALID_OV_EN_A> for bool {
    #[inline(always)]
    fn from(variant: VBVALID_OV_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl VBVALID_OV_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VBVALID_OV_EN_A {
        match self.bits {
            false => VBVALID_OV_EN_A::VALUE1,
            true => VBVALID_OV_EN_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == VBVALID_OV_EN_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == VBVALID_OV_EN_A::VALUE2
    }
}
#[doc = "Field `VbvalidOvEn` writer - VBUS Valid Override Enable"]
pub type VBVALID_OV_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, GOTGCTL_SPEC, VBVALID_OV_EN_A, O>;
impl<'a, const O: u8> VBVALID_OV_EN_W<'a, O> {
    #[doc = "Override is disabled and vbus valid signal from the PHY is used internally by the core."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(VBVALID_OV_EN_A::VALUE1)
    }
    #[doc = "Internally vbus valid received from the PHY is overridden with GOTGCTL.VbvalidOvVal."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(VBVALID_OV_EN_A::VALUE2)
    }
}
#[doc = "Field `VbvalidOvVal` reader - VBUS Valid Override Value"]
pub type VBVALID_OV_VAL_R = crate::BitReader<VBVALID_OV_VAL_A>;
#[doc = "VBUS Valid Override Value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VBVALID_OV_VAL_A {
    #[doc = "0: vbusvalid value is 0# when GOTGCTL.VbvalidOvEn = 1"]
    VALUE1 = 0,
    #[doc = "1: vbusvalid value is 1# when GOTGCTL.VbvalidOvEn = 1"]
    VALUE2 = 1,
}
impl From<VBVALID_OV_VAL_A> for bool {
    #[inline(always)]
    fn from(variant: VBVALID_OV_VAL_A) -> Self {
        variant as u8 != 0
    }
}
impl VBVALID_OV_VAL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VBVALID_OV_VAL_A {
        match self.bits {
            false => VBVALID_OV_VAL_A::VALUE1,
            true => VBVALID_OV_VAL_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == VBVALID_OV_VAL_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == VBVALID_OV_VAL_A::VALUE2
    }
}
#[doc = "Field `VbvalidOvVal` writer - VBUS Valid Override Value"]
pub type VBVALID_OV_VAL_W<'a, const O: u8> = crate::BitWriter<'a, u32, GOTGCTL_SPEC, VBVALID_OV_VAL_A, O>;
impl<'a, const O: u8> VBVALID_OV_VAL_W<'a, O> {
    #[doc = "vbusvalid value is 0# when GOTGCTL.VbvalidOvEn = 1"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(VBVALID_OV_VAL_A::VALUE1)
    }
    #[doc = "vbusvalid value is 1# when GOTGCTL.VbvalidOvEn = 1"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(VBVALID_OV_VAL_A::VALUE2)
    }
}
#[doc = "Field `AvalidOvEn` reader - A-Peripheral Session Valid Override Enable"]
pub type AVALID_OV_EN_R = crate::BitReader<AVALID_OV_EN_A>;
#[doc = "A-Peripheral Session Valid Override Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AVALID_OV_EN_A {
    #[doc = "0: Override is disabled and Avalid signal from the PHY is used internally by the core."]
    VALUE1 = 0,
    #[doc = "1: Internally Avalid received from the PHY is overridden with GOTGCTL.AvalidOvVal."]
    VALUE2 = 1,
}
impl From<AVALID_OV_EN_A> for bool {
    #[inline(always)]
    fn from(variant: AVALID_OV_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl AVALID_OV_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AVALID_OV_EN_A {
        match self.bits {
            false => AVALID_OV_EN_A::VALUE1,
            true => AVALID_OV_EN_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == AVALID_OV_EN_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == AVALID_OV_EN_A::VALUE2
    }
}
#[doc = "Field `AvalidOvEn` writer - A-Peripheral Session Valid Override Enable"]
pub type AVALID_OV_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, GOTGCTL_SPEC, AVALID_OV_EN_A, O>;
impl<'a, const O: u8> AVALID_OV_EN_W<'a, O> {
    #[doc = "Override is disabled and Avalid signal from the PHY is used internally by the core."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(AVALID_OV_EN_A::VALUE1)
    }
    #[doc = "Internally Avalid received from the PHY is overridden with GOTGCTL.AvalidOvVal."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(AVALID_OV_EN_A::VALUE2)
    }
}
#[doc = "Field `AvalidOvVal` reader - A-Peripheral Session Valid Override Value"]
pub type AVALID_OV_VAL_R = crate::BitReader<AVALID_OV_VAL_A>;
#[doc = "A-Peripheral Session Valid Override Value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AVALID_OV_VAL_A {
    #[doc = "0: Avalid value is 0# when GOTGCTL.AvalidOvEn = 1"]
    VALUE1 = 0,
    #[doc = "1: Avalid value is 1# when GOTGCTL.AvalidOvEn = 1"]
    VALUE2 = 1,
}
impl From<AVALID_OV_VAL_A> for bool {
    #[inline(always)]
    fn from(variant: AVALID_OV_VAL_A) -> Self {
        variant as u8 != 0
    }
}
impl AVALID_OV_VAL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AVALID_OV_VAL_A {
        match self.bits {
            false => AVALID_OV_VAL_A::VALUE1,
            true => AVALID_OV_VAL_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == AVALID_OV_VAL_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == AVALID_OV_VAL_A::VALUE2
    }
}
#[doc = "Field `AvalidOvVal` writer - A-Peripheral Session Valid Override Value"]
pub type AVALID_OV_VAL_W<'a, const O: u8> = crate::BitWriter<'a, u32, GOTGCTL_SPEC, AVALID_OV_VAL_A, O>;
impl<'a, const O: u8> AVALID_OV_VAL_W<'a, O> {
    #[doc = "Avalid value is 0# when GOTGCTL.AvalidOvEn = 1"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(AVALID_OV_VAL_A::VALUE1)
    }
    #[doc = "Avalid value is 1# when GOTGCTL.AvalidOvEn = 1"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(AVALID_OV_VAL_A::VALUE2)
    }
}
#[doc = "Field `BvalidOvEn` reader - B-Peripheral Session Valid Override Enable"]
pub type BVALID_OV_EN_R = crate::BitReader<BVALID_OV_EN_A>;
#[doc = "B-Peripheral Session Valid Override Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BVALID_OV_EN_A {
    #[doc = "0: Override is disabled and Bvalid signal from the PHY is used internally by the core."]
    VALUE1 = 0,
    #[doc = "1: Internally Bvalid received from the PHY is overridden with GOTGCTL.BvalidOvVal."]
    VALUE2 = 1,
}
impl From<BVALID_OV_EN_A> for bool {
    #[inline(always)]
    fn from(variant: BVALID_OV_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl BVALID_OV_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BVALID_OV_EN_A {
        match self.bits {
            false => BVALID_OV_EN_A::VALUE1,
            true => BVALID_OV_EN_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == BVALID_OV_EN_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == BVALID_OV_EN_A::VALUE2
    }
}
#[doc = "Field `BvalidOvEn` writer - B-Peripheral Session Valid Override Enable"]
pub type BVALID_OV_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, GOTGCTL_SPEC, BVALID_OV_EN_A, O>;
impl<'a, const O: u8> BVALID_OV_EN_W<'a, O> {
    #[doc = "Override is disabled and Bvalid signal from the PHY is used internally by the core."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(BVALID_OV_EN_A::VALUE1)
    }
    #[doc = "Internally Bvalid received from the PHY is overridden with GOTGCTL.BvalidOvVal."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(BVALID_OV_EN_A::VALUE2)
    }
}
#[doc = "Field `BvalidOvVal` reader - B-Peripheral Session Valid Override Value"]
pub type BVALID_OV_VAL_R = crate::BitReader<BVALID_OV_VAL_A>;
#[doc = "B-Peripheral Session Valid Override Value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BVALID_OV_VAL_A {
    #[doc = "0: Bvalid value is 0# when GOTGCTL.BvalidOvEn = 1"]
    VALUE1 = 0,
    #[doc = "1: Bvalid value is 1# when GOTGCTL.BvalidOvEn = 1"]
    VALUE2 = 1,
}
impl From<BVALID_OV_VAL_A> for bool {
    #[inline(always)]
    fn from(variant: BVALID_OV_VAL_A) -> Self {
        variant as u8 != 0
    }
}
impl BVALID_OV_VAL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BVALID_OV_VAL_A {
        match self.bits {
            false => BVALID_OV_VAL_A::VALUE1,
            true => BVALID_OV_VAL_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == BVALID_OV_VAL_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == BVALID_OV_VAL_A::VALUE2
    }
}
#[doc = "Field `BvalidOvVal` writer - B-Peripheral Session Valid Override Value"]
pub type BVALID_OV_VAL_W<'a, const O: u8> = crate::BitWriter<'a, u32, GOTGCTL_SPEC, BVALID_OV_VAL_A, O>;
impl<'a, const O: u8> BVALID_OV_VAL_W<'a, O> {
    #[doc = "Bvalid value is 0# when GOTGCTL.BvalidOvEn = 1"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(BVALID_OV_VAL_A::VALUE1)
    }
    #[doc = "Bvalid value is 1# when GOTGCTL.BvalidOvEn = 1"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(BVALID_OV_VAL_A::VALUE2)
    }
}
#[doc = "Field `HstNegScs` reader - Host Negotiation Success"]
pub type HST_NEG_SCS_R = crate::BitReader<HST_NEG_SCS_A>;
#[doc = "Host Negotiation Success\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HST_NEG_SCS_A {
    #[doc = "0: Host negotiation failure"]
    VALUE1 = 0,
    #[doc = "1: Host negotiation success"]
    VALUE2 = 1,
}
impl From<HST_NEG_SCS_A> for bool {
    #[inline(always)]
    fn from(variant: HST_NEG_SCS_A) -> Self {
        variant as u8 != 0
    }
}
impl HST_NEG_SCS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HST_NEG_SCS_A {
        match self.bits {
            false => HST_NEG_SCS_A::VALUE1,
            true => HST_NEG_SCS_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == HST_NEG_SCS_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == HST_NEG_SCS_A::VALUE2
    }
}
#[doc = "Field `HNPReq` reader - HNP Request"]
pub type HNPREQ_R = crate::BitReader<HNPREQ_A>;
#[doc = "HNP Request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HNPREQ_A {
    #[doc = "0: No HNP request"]
    VALUE1 = 0,
    #[doc = "1: HNP request"]
    VALUE2 = 1,
}
impl From<HNPREQ_A> for bool {
    #[inline(always)]
    fn from(variant: HNPREQ_A) -> Self {
        variant as u8 != 0
    }
}
impl HNPREQ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HNPREQ_A {
        match self.bits {
            false => HNPREQ_A::VALUE1,
            true => HNPREQ_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == HNPREQ_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == HNPREQ_A::VALUE2
    }
}
#[doc = "Field `HNPReq` writer - HNP Request"]
pub type HNPREQ_W<'a, const O: u8> = crate::BitWriter<'a, u32, GOTGCTL_SPEC, HNPREQ_A, O>;
impl<'a, const O: u8> HNPREQ_W<'a, O> {
    #[doc = "No HNP request"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(HNPREQ_A::VALUE1)
    }
    #[doc = "HNP request"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(HNPREQ_A::VALUE2)
    }
}
#[doc = "Field `HstSetHNPEn` reader - Host Set HNP Enable"]
pub type HST_SET_HNPEN_R = crate::BitReader<HST_SET_HNPEN_A>;
#[doc = "Host Set HNP Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HST_SET_HNPEN_A {
    #[doc = "0: Host Set HNP is not enabled"]
    VALUE1 = 0,
    #[doc = "1: Host Set HNP is enabled"]
    VALUE2 = 1,
}
impl From<HST_SET_HNPEN_A> for bool {
    #[inline(always)]
    fn from(variant: HST_SET_HNPEN_A) -> Self {
        variant as u8 != 0
    }
}
impl HST_SET_HNPEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HST_SET_HNPEN_A {
        match self.bits {
            false => HST_SET_HNPEN_A::VALUE1,
            true => HST_SET_HNPEN_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == HST_SET_HNPEN_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == HST_SET_HNPEN_A::VALUE2
    }
}
#[doc = "Field `HstSetHNPEn` writer - Host Set HNP Enable"]
pub type HST_SET_HNPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, GOTGCTL_SPEC, HST_SET_HNPEN_A, O>;
impl<'a, const O: u8> HST_SET_HNPEN_W<'a, O> {
    #[doc = "Host Set HNP is not enabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(HST_SET_HNPEN_A::VALUE1)
    }
    #[doc = "Host Set HNP is enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(HST_SET_HNPEN_A::VALUE2)
    }
}
#[doc = "Field `DevHNPEn` reader - Device HNP Enabled"]
pub type DEV_HNPEN_R = crate::BitReader<DEV_HNPEN_A>;
#[doc = "Device HNP Enabled\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DEV_HNPEN_A {
    #[doc = "0: HNP is not enabled in the application"]
    VALUE1 = 0,
    #[doc = "1: HNP is enabled in the application"]
    VALUE2 = 1,
}
impl From<DEV_HNPEN_A> for bool {
    #[inline(always)]
    fn from(variant: DEV_HNPEN_A) -> Self {
        variant as u8 != 0
    }
}
impl DEV_HNPEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DEV_HNPEN_A {
        match self.bits {
            false => DEV_HNPEN_A::VALUE1,
            true => DEV_HNPEN_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == DEV_HNPEN_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == DEV_HNPEN_A::VALUE2
    }
}
#[doc = "Field `DevHNPEn` writer - Device HNP Enabled"]
pub type DEV_HNPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, GOTGCTL_SPEC, DEV_HNPEN_A, O>;
impl<'a, const O: u8> DEV_HNPEN_W<'a, O> {
    #[doc = "HNP is not enabled in the application"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(DEV_HNPEN_A::VALUE1)
    }
    #[doc = "HNP is enabled in the application"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(DEV_HNPEN_A::VALUE2)
    }
}
#[doc = "Field `ConlDSts` reader - Connector ID Status"]
pub type CONL_DSTS_R = crate::BitReader<CONL_DSTS_A>;
#[doc = "Connector ID Status\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CONL_DSTS_A {
    #[doc = "0: The USB core is in A-Device mode"]
    VALUE1 = 0,
    #[doc = "1: The USB core is in B-Device mode"]
    VALUE2 = 1,
}
impl From<CONL_DSTS_A> for bool {
    #[inline(always)]
    fn from(variant: CONL_DSTS_A) -> Self {
        variant as u8 != 0
    }
}
impl CONL_DSTS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CONL_DSTS_A {
        match self.bits {
            false => CONL_DSTS_A::VALUE1,
            true => CONL_DSTS_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CONL_DSTS_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CONL_DSTS_A::VALUE2
    }
}
#[doc = "Field `DbncTime` reader - Long/Short Debounce Time"]
pub type DBNC_TIME_R = crate::BitReader<DBNC_TIME_A>;
#[doc = "Long/Short Debounce Time\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DBNC_TIME_A {
    #[doc = "0: Long debounce time, used for physical connections (100 ms + 2.5 us)"]
    VALUE1 = 0,
    #[doc = "1: Short debounce time, used for soft connections (2.5 us)"]
    VALUE2 = 1,
}
impl From<DBNC_TIME_A> for bool {
    #[inline(always)]
    fn from(variant: DBNC_TIME_A) -> Self {
        variant as u8 != 0
    }
}
impl DBNC_TIME_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DBNC_TIME_A {
        match self.bits {
            false => DBNC_TIME_A::VALUE1,
            true => DBNC_TIME_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == DBNC_TIME_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == DBNC_TIME_A::VALUE2
    }
}
#[doc = "Field `ASesVId` reader - A-Session Valid"]
pub type ASES_VID_R = crate::BitReader<ASES_VID_A>;
#[doc = "A-Session Valid\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ASES_VID_A {
    #[doc = "0: A-session is not valid"]
    VALUE1 = 0,
    #[doc = "1: A-session is valid"]
    VALUE2 = 1,
}
impl From<ASES_VID_A> for bool {
    #[inline(always)]
    fn from(variant: ASES_VID_A) -> Self {
        variant as u8 != 0
    }
}
impl ASES_VID_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ASES_VID_A {
        match self.bits {
            false => ASES_VID_A::VALUE1,
            true => ASES_VID_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == ASES_VID_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == ASES_VID_A::VALUE2
    }
}
#[doc = "Field `BSesVld` reader - B-Session Valid"]
pub type BSES_VLD_R = crate::BitReader<BSES_VLD_A>;
#[doc = "B-Session Valid\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BSES_VLD_A {
    #[doc = "0: B-session is not valid."]
    VALUE1 = 0,
    #[doc = "1: B-session is valid."]
    VALUE2 = 1,
}
impl From<BSES_VLD_A> for bool {
    #[inline(always)]
    fn from(variant: BSES_VLD_A) -> Self {
        variant as u8 != 0
    }
}
impl BSES_VLD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BSES_VLD_A {
        match self.bits {
            false => BSES_VLD_A::VALUE1,
            true => BSES_VLD_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == BSES_VLD_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == BSES_VLD_A::VALUE2
    }
}
#[doc = "Field `OTGVer` reader - OTG Version"]
pub type OTGVER_R = crate::BitReader<OTGVER_A>;
#[doc = "OTG Version\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OTGVER_A {
    #[doc = "0: OTG Version 1.3. In this version the core supports Data line pulsing and VBus pulsing for SRP."]
    VALUE1 = 0,
    #[doc = "1: OTG Version 2.0. In this version the core supports only Data line pulsing for SRP."]
    VALUE2 = 1,
}
impl From<OTGVER_A> for bool {
    #[inline(always)]
    fn from(variant: OTGVER_A) -> Self {
        variant as u8 != 0
    }
}
impl OTGVER_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OTGVER_A {
        match self.bits {
            false => OTGVER_A::VALUE1,
            true => OTGVER_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == OTGVER_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == OTGVER_A::VALUE2
    }
}
#[doc = "Field `OTGVer` writer - OTG Version"]
pub type OTGVER_W<'a, const O: u8> = crate::BitWriter<'a, u32, GOTGCTL_SPEC, OTGVER_A, O>;
impl<'a, const O: u8> OTGVER_W<'a, O> {
    #[doc = "OTG Version 1.3. In this version the core supports Data line pulsing and VBus pulsing for SRP."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(OTGVER_A::VALUE1)
    }
    #[doc = "OTG Version 2.0. In this version the core supports only Data line pulsing for SRP."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(OTGVER_A::VALUE2)
    }
}
impl R {
    #[doc = "Bit 0 - Session Request Success"]
    #[inline(always)]
    pub fn ses_req_scs(&self) -> SES_REQ_SCS_R {
        SES_REQ_SCS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Session Request"]
    #[inline(always)]
    pub fn ses_req(&self) -> SES_REQ_R {
        SES_REQ_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - VBUS Valid Override Enable"]
    #[inline(always)]
    pub fn vbvalid_ov_en(&self) -> VBVALID_OV_EN_R {
        VBVALID_OV_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - VBUS Valid Override Value"]
    #[inline(always)]
    pub fn vbvalid_ov_val(&self) -> VBVALID_OV_VAL_R {
        VBVALID_OV_VAL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - A-Peripheral Session Valid Override Enable"]
    #[inline(always)]
    pub fn avalid_ov_en(&self) -> AVALID_OV_EN_R {
        AVALID_OV_EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - A-Peripheral Session Valid Override Value"]
    #[inline(always)]
    pub fn avalid_ov_val(&self) -> AVALID_OV_VAL_R {
        AVALID_OV_VAL_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - B-Peripheral Session Valid Override Enable"]
    #[inline(always)]
    pub fn bvalid_ov_en(&self) -> BVALID_OV_EN_R {
        BVALID_OV_EN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - B-Peripheral Session Valid Override Value"]
    #[inline(always)]
    pub fn bvalid_ov_val(&self) -> BVALID_OV_VAL_R {
        BVALID_OV_VAL_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Host Negotiation Success"]
    #[inline(always)]
    pub fn hst_neg_scs(&self) -> HST_NEG_SCS_R {
        HST_NEG_SCS_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - HNP Request"]
    #[inline(always)]
    pub fn hnpreq(&self) -> HNPREQ_R {
        HNPREQ_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Host Set HNP Enable"]
    #[inline(always)]
    pub fn hst_set_hnpen(&self) -> HST_SET_HNPEN_R {
        HST_SET_HNPEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Device HNP Enabled"]
    #[inline(always)]
    pub fn dev_hnpen(&self) -> DEV_HNPEN_R {
        DEV_HNPEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 16 - Connector ID Status"]
    #[inline(always)]
    pub fn conl_dsts(&self) -> CONL_DSTS_R {
        CONL_DSTS_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Long/Short Debounce Time"]
    #[inline(always)]
    pub fn dbnc_time(&self) -> DBNC_TIME_R {
        DBNC_TIME_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - A-Session Valid"]
    #[inline(always)]
    pub fn ases_vid(&self) -> ASES_VID_R {
        ASES_VID_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - B-Session Valid"]
    #[inline(always)]
    pub fn bses_vld(&self) -> BSES_VLD_R {
        BSES_VLD_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - OTG Version"]
    #[inline(always)]
    pub fn otgver(&self) -> OTGVER_R {
        OTGVER_R::new(((self.bits >> 20) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Session Request"]
    #[inline(always)]
    #[must_use]
    pub fn ses_req(&mut self) -> SES_REQ_W<1> {
        SES_REQ_W::new(self)
    }
    #[doc = "Bit 2 - VBUS Valid Override Enable"]
    #[inline(always)]
    #[must_use]
    pub fn vbvalid_ov_en(&mut self) -> VBVALID_OV_EN_W<2> {
        VBVALID_OV_EN_W::new(self)
    }
    #[doc = "Bit 3 - VBUS Valid Override Value"]
    #[inline(always)]
    #[must_use]
    pub fn vbvalid_ov_val(&mut self) -> VBVALID_OV_VAL_W<3> {
        VBVALID_OV_VAL_W::new(self)
    }
    #[doc = "Bit 4 - A-Peripheral Session Valid Override Enable"]
    #[inline(always)]
    #[must_use]
    pub fn avalid_ov_en(&mut self) -> AVALID_OV_EN_W<4> {
        AVALID_OV_EN_W::new(self)
    }
    #[doc = "Bit 5 - A-Peripheral Session Valid Override Value"]
    #[inline(always)]
    #[must_use]
    pub fn avalid_ov_val(&mut self) -> AVALID_OV_VAL_W<5> {
        AVALID_OV_VAL_W::new(self)
    }
    #[doc = "Bit 6 - B-Peripheral Session Valid Override Enable"]
    #[inline(always)]
    #[must_use]
    pub fn bvalid_ov_en(&mut self) -> BVALID_OV_EN_W<6> {
        BVALID_OV_EN_W::new(self)
    }
    #[doc = "Bit 7 - B-Peripheral Session Valid Override Value"]
    #[inline(always)]
    #[must_use]
    pub fn bvalid_ov_val(&mut self) -> BVALID_OV_VAL_W<7> {
        BVALID_OV_VAL_W::new(self)
    }
    #[doc = "Bit 9 - HNP Request"]
    #[inline(always)]
    #[must_use]
    pub fn hnpreq(&mut self) -> HNPREQ_W<9> {
        HNPREQ_W::new(self)
    }
    #[doc = "Bit 10 - Host Set HNP Enable"]
    #[inline(always)]
    #[must_use]
    pub fn hst_set_hnpen(&mut self) -> HST_SET_HNPEN_W<10> {
        HST_SET_HNPEN_W::new(self)
    }
    #[doc = "Bit 11 - Device HNP Enabled"]
    #[inline(always)]
    #[must_use]
    pub fn dev_hnpen(&mut self) -> DEV_HNPEN_W<11> {
        DEV_HNPEN_W::new(self)
    }
    #[doc = "Bit 20 - OTG Version"]
    #[inline(always)]
    #[must_use]
    pub fn otgver(&mut self) -> OTGVER_W<20> {
        OTGVER_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control and Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gotgctl](index.html) module"]
pub struct GOTGCTL_SPEC;
impl crate::RegisterSpec for GOTGCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gotgctl::R](R) reader structure"]
impl crate::Readable for GOTGCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gotgctl::W](W) writer structure"]
impl crate::Writable for GOTGCTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GOTGCTL to value 0x0001_0000"]
impl crate::Resettable for GOTGCTL_SPEC {
    const RESET_VALUE: Self::Ux = 0x0001_0000;
}
