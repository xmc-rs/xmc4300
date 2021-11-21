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
#[doc = "Port Connect Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PRTCONNSTS_A {
    #[doc = "0: No device is attached to the port."]
    VALUE1 = 0,
    #[doc = "1: A device is attached to the port."]
    VALUE2 = 1,
}
impl From<PRTCONNSTS_A> for bool {
    #[inline(always)]
    fn from(variant: PRTCONNSTS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PrtConnSts` reader - Port Connect Status"]
pub struct PRTCONNSTS_R(crate::FieldReader<bool, PRTCONNSTS_A>);
impl PRTCONNSTS_R {
    pub(crate) fn new(bits: bool) -> Self {
        PRTCONNSTS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PRTCONNSTS_A {
        match self.bits {
            false => PRTCONNSTS_A::VALUE1,
            true => PRTCONNSTS_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == PRTCONNSTS_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == PRTCONNSTS_A::VALUE2
    }
}
impl core::ops::Deref for PRTCONNSTS_R {
    type Target = crate::FieldReader<bool, PRTCONNSTS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PrtConnDet` reader - Port Connect Detected"]
pub struct PRTCONNDET_R(crate::FieldReader<bool, bool>);
impl PRTCONNDET_R {
    pub(crate) fn new(bits: bool) -> Self {
        PRTCONNDET_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRTCONNDET_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PrtConnDet` writer - Port Connect Detected"]
pub struct PRTCONNDET_W<'a> {
    w: &'a mut W,
}
impl<'a> PRTCONNDET_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Port Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PRTENA_A {
    #[doc = "0: Port disabled"]
    VALUE1 = 0,
    #[doc = "1: Port enabled"]
    VALUE2 = 1,
}
impl From<PRTENA_A> for bool {
    #[inline(always)]
    fn from(variant: PRTENA_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PrtEna` reader - Port Enable"]
pub struct PRTENA_R(crate::FieldReader<bool, PRTENA_A>);
impl PRTENA_R {
    pub(crate) fn new(bits: bool) -> Self {
        PRTENA_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PRTENA_A {
        match self.bits {
            false => PRTENA_A::VALUE1,
            true => PRTENA_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == PRTENA_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == PRTENA_A::VALUE2
    }
}
impl core::ops::Deref for PRTENA_R {
    type Target = crate::FieldReader<bool, PRTENA_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PrtEna` writer - Port Enable"]
pub struct PRTENA_W<'a> {
    w: &'a mut W,
}
impl<'a> PRTENA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PRTENA_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Port disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(PRTENA_A::VALUE1)
    }
    #[doc = "Port enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(PRTENA_A::VALUE2)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `PrtEnChng` reader - Port Enable/Disable Change"]
pub struct PRTENCHNG_R(crate::FieldReader<bool, bool>);
impl PRTENCHNG_R {
    pub(crate) fn new(bits: bool) -> Self {
        PRTENCHNG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRTENCHNG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PrtEnChng` writer - Port Enable/Disable Change"]
pub struct PRTENCHNG_W<'a> {
    w: &'a mut W,
}
impl<'a> PRTENCHNG_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "Port Overcurrent Active\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PRTOVRCURRACT_A {
    #[doc = "0: No overcurrent condition"]
    VALUE1 = 0,
    #[doc = "1: Overcurrent condition"]
    VALUE2 = 1,
}
impl From<PRTOVRCURRACT_A> for bool {
    #[inline(always)]
    fn from(variant: PRTOVRCURRACT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PrtOvrCurrAct` reader - Port Overcurrent Active"]
pub struct PRTOVRCURRACT_R(crate::FieldReader<bool, PRTOVRCURRACT_A>);
impl PRTOVRCURRACT_R {
    pub(crate) fn new(bits: bool) -> Self {
        PRTOVRCURRACT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PRTOVRCURRACT_A {
        match self.bits {
            false => PRTOVRCURRACT_A::VALUE1,
            true => PRTOVRCURRACT_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == PRTOVRCURRACT_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == PRTOVRCURRACT_A::VALUE2
    }
}
impl core::ops::Deref for PRTOVRCURRACT_R {
    type Target = crate::FieldReader<bool, PRTOVRCURRACT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PrtOvrCurrChng` reader - Port Overcurrent Change"]
pub struct PRTOVRCURRCHNG_R(crate::FieldReader<bool, bool>);
impl PRTOVRCURRCHNG_R {
    pub(crate) fn new(bits: bool) -> Self {
        PRTOVRCURRCHNG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRTOVRCURRCHNG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PrtOvrCurrChng` writer - Port Overcurrent Change"]
pub struct PRTOVRCURRCHNG_W<'a> {
    w: &'a mut W,
}
impl<'a> PRTOVRCURRCHNG_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "Port Resume\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PRTRES_A {
    #[doc = "0: No resume driven"]
    VALUE1 = 0,
    #[doc = "1: Resume driven"]
    VALUE2 = 1,
}
impl From<PRTRES_A> for bool {
    #[inline(always)]
    fn from(variant: PRTRES_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PrtRes` reader - Port Resume"]
pub struct PRTRES_R(crate::FieldReader<bool, PRTRES_A>);
impl PRTRES_R {
    pub(crate) fn new(bits: bool) -> Self {
        PRTRES_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PRTRES_A {
        match self.bits {
            false => PRTRES_A::VALUE1,
            true => PRTRES_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == PRTRES_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == PRTRES_A::VALUE2
    }
}
impl core::ops::Deref for PRTRES_R {
    type Target = crate::FieldReader<bool, PRTRES_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PrtRes` writer - Port Resume"]
pub struct PRTRES_W<'a> {
    w: &'a mut W,
}
impl<'a> PRTRES_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PRTRES_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No resume driven"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(PRTRES_A::VALUE1)
    }
    #[doc = "Resume driven"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(PRTRES_A::VALUE2)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "Port Suspend\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PRTSUSP_A {
    #[doc = "0: Port not in Suspend mode"]
    VALUE1 = 0,
    #[doc = "1: Port in Suspend mode"]
    VALUE2 = 1,
}
impl From<PRTSUSP_A> for bool {
    #[inline(always)]
    fn from(variant: PRTSUSP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PrtSusp` reader - Port Suspend"]
pub struct PRTSUSP_R(crate::FieldReader<bool, PRTSUSP_A>);
impl PRTSUSP_R {
    pub(crate) fn new(bits: bool) -> Self {
        PRTSUSP_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PRTSUSP_A {
        match self.bits {
            false => PRTSUSP_A::VALUE1,
            true => PRTSUSP_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == PRTSUSP_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == PRTSUSP_A::VALUE2
    }
}
impl core::ops::Deref for PRTSUSP_R {
    type Target = crate::FieldReader<bool, PRTSUSP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PrtSusp` writer - Port Suspend"]
pub struct PRTSUSP_W<'a> {
    w: &'a mut W,
}
impl<'a> PRTSUSP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PRTSUSP_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Port not in Suspend mode"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(PRTSUSP_A::VALUE1)
    }
    #[doc = "Port in Suspend mode"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(PRTSUSP_A::VALUE2)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
#[doc = "Port Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PRTRST_A {
    #[doc = "0: Port not in reset"]
    VALUE1 = 0,
    #[doc = "1: Port in reset"]
    VALUE2 = 1,
}
impl From<PRTRST_A> for bool {
    #[inline(always)]
    fn from(variant: PRTRST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PrtRst` reader - Port Reset"]
pub struct PRTRST_R(crate::FieldReader<bool, PRTRST_A>);
impl PRTRST_R {
    pub(crate) fn new(bits: bool) -> Self {
        PRTRST_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PRTRST_A {
        match self.bits {
            false => PRTRST_A::VALUE1,
            true => PRTRST_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == PRTRST_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == PRTRST_A::VALUE2
    }
}
impl core::ops::Deref for PRTRST_R {
    type Target = crate::FieldReader<bool, PRTRST_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PrtRst` writer - Port Reset"]
pub struct PRTRST_W<'a> {
    w: &'a mut W,
}
impl<'a> PRTRST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PRTRST_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Port not in reset"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(PRTRST_A::VALUE1)
    }
    #[doc = "Port in reset"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(PRTRST_A::VALUE2)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
#[doc = "Field `PrtLnSts` reader - Port Line Status"]
pub struct PRTLNSTS_R(crate::FieldReader<u8, u8>);
impl PRTLNSTS_R {
    pub(crate) fn new(bits: u8) -> Self {
        PRTLNSTS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRTLNSTS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Port Power\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PRTPWR_A {
    #[doc = "0: Power off"]
    VALUE1 = 0,
    #[doc = "1: Power on"]
    VALUE2 = 1,
}
impl From<PRTPWR_A> for bool {
    #[inline(always)]
    fn from(variant: PRTPWR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PrtPwr` reader - Port Power"]
pub struct PRTPWR_R(crate::FieldReader<bool, PRTPWR_A>);
impl PRTPWR_R {
    pub(crate) fn new(bits: bool) -> Self {
        PRTPWR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PRTPWR_A {
        match self.bits {
            false => PRTPWR_A::VALUE1,
            true => PRTPWR_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == PRTPWR_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == PRTPWR_A::VALUE2
    }
}
impl core::ops::Deref for PRTPWR_R {
    type Target = crate::FieldReader<bool, PRTPWR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PrtPwr` writer - Port Power"]
pub struct PRTPWR_W<'a> {
    w: &'a mut W,
}
impl<'a> PRTPWR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PRTPWR_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Power off"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(PRTPWR_A::VALUE1)
    }
    #[doc = "Power on"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(PRTPWR_A::VALUE2)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u32 & 0x01) << 12);
        self.w
    }
}
#[doc = "Port Speed\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PRTSPD_A {
    #[doc = "1: Full speed"]
    VALUE1 = 1,
}
impl From<PRTSPD_A> for u8 {
    #[inline(always)]
    fn from(variant: PRTSPD_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PrtSpd` reader - Port Speed"]
pub struct PRTSPD_R(crate::FieldReader<u8, PRTSPD_A>);
impl PRTSPD_R {
    pub(crate) fn new(bits: u8) -> Self {
        PRTSPD_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PRTSPD_A> {
        match self.bits {
            1 => Some(PRTSPD_A::VALUE1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == PRTSPD_A::VALUE1
    }
}
impl core::ops::Deref for PRTSPD_R {
    type Target = crate::FieldReader<u8, PRTSPD_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Port Connect Status"]
    #[inline(always)]
    pub fn prt_conn_sts(&self) -> PRTCONNSTS_R {
        PRTCONNSTS_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Port Connect Detected"]
    #[inline(always)]
    pub fn prt_conn_det(&self) -> PRTCONNDET_R {
        PRTCONNDET_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Port Enable"]
    #[inline(always)]
    pub fn prt_ena(&self) -> PRTENA_R {
        PRTENA_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Port Enable/Disable Change"]
    #[inline(always)]
    pub fn prt_en_chng(&self) -> PRTENCHNG_R {
        PRTENCHNG_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Port Overcurrent Active"]
    #[inline(always)]
    pub fn prt_ovr_curr_act(&self) -> PRTOVRCURRACT_R {
        PRTOVRCURRACT_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Port Overcurrent Change"]
    #[inline(always)]
    pub fn prt_ovr_curr_chng(&self) -> PRTOVRCURRCHNG_R {
        PRTOVRCURRCHNG_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Port Resume"]
    #[inline(always)]
    pub fn prt_res(&self) -> PRTRES_R {
        PRTRES_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Port Suspend"]
    #[inline(always)]
    pub fn prt_susp(&self) -> PRTSUSP_R {
        PRTSUSP_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Port Reset"]
    #[inline(always)]
    pub fn prt_rst(&self) -> PRTRST_R {
        PRTRST_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 10:11 - Port Line Status"]
    #[inline(always)]
    pub fn prt_ln_sts(&self) -> PRTLNSTS_R {
        PRTLNSTS_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bit 12 - Port Power"]
    #[inline(always)]
    pub fn prt_pwr(&self) -> PRTPWR_R {
        PRTPWR_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bits 17:18 - Port Speed"]
    #[inline(always)]
    pub fn prt_spd(&self) -> PRTSPD_R {
        PRTSPD_R::new(((self.bits >> 17) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 1 - Port Connect Detected"]
    #[inline(always)]
    pub fn prt_conn_det(&mut self) -> PRTCONNDET_W {
        PRTCONNDET_W { w: self }
    }
    #[doc = "Bit 2 - Port Enable"]
    #[inline(always)]
    pub fn prt_ena(&mut self) -> PRTENA_W {
        PRTENA_W { w: self }
    }
    #[doc = "Bit 3 - Port Enable/Disable Change"]
    #[inline(always)]
    pub fn prt_en_chng(&mut self) -> PRTENCHNG_W {
        PRTENCHNG_W { w: self }
    }
    #[doc = "Bit 5 - Port Overcurrent Change"]
    #[inline(always)]
    pub fn prt_ovr_curr_chng(&mut self) -> PRTOVRCURRCHNG_W {
        PRTOVRCURRCHNG_W { w: self }
    }
    #[doc = "Bit 6 - Port Resume"]
    #[inline(always)]
    pub fn prt_res(&mut self) -> PRTRES_W {
        PRTRES_W { w: self }
    }
    #[doc = "Bit 7 - Port Suspend"]
    #[inline(always)]
    pub fn prt_susp(&mut self) -> PRTSUSP_W {
        PRTSUSP_W { w: self }
    }
    #[doc = "Bit 8 - Port Reset"]
    #[inline(always)]
    pub fn prt_rst(&mut self) -> PRTRST_W {
        PRTRST_W { w: self }
    }
    #[doc = "Bit 12 - Port Power"]
    #[inline(always)]
    pub fn prt_pwr(&mut self) -> PRTPWR_W {
        PRTPWR_W { w: self }
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
}
#[doc = "`reset()` method sets HPRT to value 0"]
impl crate::Resettable for HPRT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
