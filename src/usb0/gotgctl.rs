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
#[doc = "Session Request Success\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SESREQSCS_A {
    #[doc = "0: Session request failure"]
    VALUE1 = 0,
    #[doc = "1: Session request success"]
    VALUE2 = 1,
}
impl From<SESREQSCS_A> for bool {
    #[inline(always)]
    fn from(variant: SESREQSCS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SesReqScs` reader - Session Request Success"]
pub struct SESREQSCS_R(crate::FieldReader<bool, SESREQSCS_A>);
impl SESREQSCS_R {
    pub(crate) fn new(bits: bool) -> Self {
        SESREQSCS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SESREQSCS_A {
        match self.bits {
            false => SESREQSCS_A::VALUE1,
            true => SESREQSCS_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == SESREQSCS_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == SESREQSCS_A::VALUE2
    }
}
impl core::ops::Deref for SESREQSCS_R {
    type Target = crate::FieldReader<bool, SESREQSCS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Session Request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SESREQ_A {
    #[doc = "0: No session request"]
    VALUE1 = 0,
    #[doc = "1: Session request"]
    VALUE2 = 1,
}
impl From<SESREQ_A> for bool {
    #[inline(always)]
    fn from(variant: SESREQ_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SesReq` reader - Session Request"]
pub struct SESREQ_R(crate::FieldReader<bool, SESREQ_A>);
impl SESREQ_R {
    pub(crate) fn new(bits: bool) -> Self {
        SESREQ_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SESREQ_A {
        match self.bits {
            false => SESREQ_A::VALUE1,
            true => SESREQ_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == SESREQ_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == SESREQ_A::VALUE2
    }
}
impl core::ops::Deref for SESREQ_R {
    type Target = crate::FieldReader<bool, SESREQ_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SesReq` writer - Session Request"]
pub struct SESREQ_W<'a> {
    w: &'a mut W,
}
impl<'a> SESREQ_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SESREQ_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No session request"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(SESREQ_A::VALUE1)
    }
    #[doc = "Session request"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(SESREQ_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "VBUS Valid Override Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VBVALIDOVEN_A {
    #[doc = "0: Override is disabled and vbus valid signal from the PHY is used internally by the core."]
    VALUE1 = 0,
    #[doc = "1: Internally vbus valid received from the PHY is overridden with GOTGCTL.VbvalidOvVal."]
    VALUE2 = 1,
}
impl From<VBVALIDOVEN_A> for bool {
    #[inline(always)]
    fn from(variant: VBVALIDOVEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VbvalidOvEn` reader - VBUS Valid Override Enable"]
pub struct VBVALIDOVEN_R(crate::FieldReader<bool, VBVALIDOVEN_A>);
impl VBVALIDOVEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        VBVALIDOVEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VBVALIDOVEN_A {
        match self.bits {
            false => VBVALIDOVEN_A::VALUE1,
            true => VBVALIDOVEN_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == VBVALIDOVEN_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == VBVALIDOVEN_A::VALUE2
    }
}
impl core::ops::Deref for VBVALIDOVEN_R {
    type Target = crate::FieldReader<bool, VBVALIDOVEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VbvalidOvEn` writer - VBUS Valid Override Enable"]
pub struct VBVALIDOVEN_W<'a> {
    w: &'a mut W,
}
impl<'a> VBVALIDOVEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: VBVALIDOVEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Override is disabled and vbus valid signal from the PHY is used internally by the core."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(VBVALIDOVEN_A::VALUE1)
    }
    #[doc = "Internally vbus valid received from the PHY is overridden with GOTGCTL.VbvalidOvVal."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(VBVALIDOVEN_A::VALUE2)
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
#[doc = "VBUS Valid Override Value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VBVALIDOVVAL_A {
    #[doc = "0: vbusvalid value is 0# when GOTGCTL.VbvalidOvEn = 1"]
    VALUE1 = 0,
    #[doc = "1: vbusvalid value is 1# when GOTGCTL.VbvalidOvEn = 1"]
    VALUE2 = 1,
}
impl From<VBVALIDOVVAL_A> for bool {
    #[inline(always)]
    fn from(variant: VBVALIDOVVAL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VbvalidOvVal` reader - VBUS Valid Override Value"]
pub struct VBVALIDOVVAL_R(crate::FieldReader<bool, VBVALIDOVVAL_A>);
impl VBVALIDOVVAL_R {
    pub(crate) fn new(bits: bool) -> Self {
        VBVALIDOVVAL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VBVALIDOVVAL_A {
        match self.bits {
            false => VBVALIDOVVAL_A::VALUE1,
            true => VBVALIDOVVAL_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == VBVALIDOVVAL_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == VBVALIDOVVAL_A::VALUE2
    }
}
impl core::ops::Deref for VBVALIDOVVAL_R {
    type Target = crate::FieldReader<bool, VBVALIDOVVAL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VbvalidOvVal` writer - VBUS Valid Override Value"]
pub struct VBVALIDOVVAL_W<'a> {
    w: &'a mut W,
}
impl<'a> VBVALIDOVVAL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: VBVALIDOVVAL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "vbusvalid value is 0# when GOTGCTL.VbvalidOvEn = 1"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(VBVALIDOVVAL_A::VALUE1)
    }
    #[doc = "vbusvalid value is 1# when GOTGCTL.VbvalidOvEn = 1"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(VBVALIDOVVAL_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "A-Peripheral Session Valid Override Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AVALIDOVEN_A {
    #[doc = "0: Override is disabled and Avalid signal from the PHY is used internally by the core."]
    VALUE1 = 0,
    #[doc = "1: Internally Avalid received from the PHY is overridden with GOTGCTL.AvalidOvVal."]
    VALUE2 = 1,
}
impl From<AVALIDOVEN_A> for bool {
    #[inline(always)]
    fn from(variant: AVALIDOVEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AvalidOvEn` reader - A-Peripheral Session Valid Override Enable"]
pub struct AVALIDOVEN_R(crate::FieldReader<bool, AVALIDOVEN_A>);
impl AVALIDOVEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        AVALIDOVEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AVALIDOVEN_A {
        match self.bits {
            false => AVALIDOVEN_A::VALUE1,
            true => AVALIDOVEN_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == AVALIDOVEN_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == AVALIDOVEN_A::VALUE2
    }
}
impl core::ops::Deref for AVALIDOVEN_R {
    type Target = crate::FieldReader<bool, AVALIDOVEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AvalidOvEn` writer - A-Peripheral Session Valid Override Enable"]
pub struct AVALIDOVEN_W<'a> {
    w: &'a mut W,
}
impl<'a> AVALIDOVEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AVALIDOVEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Override is disabled and Avalid signal from the PHY is used internally by the core."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(AVALIDOVEN_A::VALUE1)
    }
    #[doc = "Internally Avalid received from the PHY is overridden with GOTGCTL.AvalidOvVal."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(AVALIDOVEN_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "A-Peripheral Session Valid Override Value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AVALIDOVVAL_A {
    #[doc = "0: Avalid value is 0# when GOTGCTL.AvalidOvEn = 1"]
    VALUE1 = 0,
    #[doc = "1: Avalid value is 1# when GOTGCTL.AvalidOvEn = 1"]
    VALUE2 = 1,
}
impl From<AVALIDOVVAL_A> for bool {
    #[inline(always)]
    fn from(variant: AVALIDOVVAL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AvalidOvVal` reader - A-Peripheral Session Valid Override Value"]
pub struct AVALIDOVVAL_R(crate::FieldReader<bool, AVALIDOVVAL_A>);
impl AVALIDOVVAL_R {
    pub(crate) fn new(bits: bool) -> Self {
        AVALIDOVVAL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AVALIDOVVAL_A {
        match self.bits {
            false => AVALIDOVVAL_A::VALUE1,
            true => AVALIDOVVAL_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == AVALIDOVVAL_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == AVALIDOVVAL_A::VALUE2
    }
}
impl core::ops::Deref for AVALIDOVVAL_R {
    type Target = crate::FieldReader<bool, AVALIDOVVAL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AvalidOvVal` writer - A-Peripheral Session Valid Override Value"]
pub struct AVALIDOVVAL_W<'a> {
    w: &'a mut W,
}
impl<'a> AVALIDOVVAL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AVALIDOVVAL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Avalid value is 0# when GOTGCTL.AvalidOvEn = 1"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(AVALIDOVVAL_A::VALUE1)
    }
    #[doc = "Avalid value is 1# when GOTGCTL.AvalidOvEn = 1"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(AVALIDOVVAL_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "B-Peripheral Session Valid Override Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BVALIDOVEN_A {
    #[doc = "0: Override is disabled and Bvalid signal from the PHY is used internally by the core."]
    VALUE1 = 0,
    #[doc = "1: Internally Bvalid received from the PHY is overridden with GOTGCTL.BvalidOvVal."]
    VALUE2 = 1,
}
impl From<BVALIDOVEN_A> for bool {
    #[inline(always)]
    fn from(variant: BVALIDOVEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BvalidOvEn` reader - B-Peripheral Session Valid Override Enable"]
pub struct BVALIDOVEN_R(crate::FieldReader<bool, BVALIDOVEN_A>);
impl BVALIDOVEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        BVALIDOVEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BVALIDOVEN_A {
        match self.bits {
            false => BVALIDOVEN_A::VALUE1,
            true => BVALIDOVEN_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == BVALIDOVEN_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == BVALIDOVEN_A::VALUE2
    }
}
impl core::ops::Deref for BVALIDOVEN_R {
    type Target = crate::FieldReader<bool, BVALIDOVEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BvalidOvEn` writer - B-Peripheral Session Valid Override Enable"]
pub struct BVALIDOVEN_W<'a> {
    w: &'a mut W,
}
impl<'a> BVALIDOVEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BVALIDOVEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Override is disabled and Bvalid signal from the PHY is used internally by the core."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(BVALIDOVEN_A::VALUE1)
    }
    #[doc = "Internally Bvalid received from the PHY is overridden with GOTGCTL.BvalidOvVal."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(BVALIDOVEN_A::VALUE2)
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
#[doc = "B-Peripheral Session Valid Override Value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BVALIDOVVAL_A {
    #[doc = "0: Bvalid value is 0# when GOTGCTL.BvalidOvEn = 1"]
    VALUE1 = 0,
    #[doc = "1: Bvalid value is 1# when GOTGCTL.BvalidOvEn = 1"]
    VALUE2 = 1,
}
impl From<BVALIDOVVAL_A> for bool {
    #[inline(always)]
    fn from(variant: BVALIDOVVAL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BvalidOvVal` reader - B-Peripheral Session Valid Override Value"]
pub struct BVALIDOVVAL_R(crate::FieldReader<bool, BVALIDOVVAL_A>);
impl BVALIDOVVAL_R {
    pub(crate) fn new(bits: bool) -> Self {
        BVALIDOVVAL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BVALIDOVVAL_A {
        match self.bits {
            false => BVALIDOVVAL_A::VALUE1,
            true => BVALIDOVVAL_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == BVALIDOVVAL_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == BVALIDOVVAL_A::VALUE2
    }
}
impl core::ops::Deref for BVALIDOVVAL_R {
    type Target = crate::FieldReader<bool, BVALIDOVVAL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BvalidOvVal` writer - B-Peripheral Session Valid Override Value"]
pub struct BVALIDOVVAL_W<'a> {
    w: &'a mut W,
}
impl<'a> BVALIDOVVAL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BVALIDOVVAL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Bvalid value is 0# when GOTGCTL.BvalidOvEn = 1"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(BVALIDOVVAL_A::VALUE1)
    }
    #[doc = "Bvalid value is 1# when GOTGCTL.BvalidOvEn = 1"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(BVALIDOVVAL_A::VALUE2)
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
#[doc = "Host Negotiation Success\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HSTNEGSCS_A {
    #[doc = "0: Host negotiation failure"]
    VALUE1 = 0,
    #[doc = "1: Host negotiation success"]
    VALUE2 = 1,
}
impl From<HSTNEGSCS_A> for bool {
    #[inline(always)]
    fn from(variant: HSTNEGSCS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HstNegScs` reader - Host Negotiation Success"]
pub struct HSTNEGSCS_R(crate::FieldReader<bool, HSTNEGSCS_A>);
impl HSTNEGSCS_R {
    pub(crate) fn new(bits: bool) -> Self {
        HSTNEGSCS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HSTNEGSCS_A {
        match self.bits {
            false => HSTNEGSCS_A::VALUE1,
            true => HSTNEGSCS_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == HSTNEGSCS_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == HSTNEGSCS_A::VALUE2
    }
}
impl core::ops::Deref for HSTNEGSCS_R {
    type Target = crate::FieldReader<bool, HSTNEGSCS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "HNP Request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Field `HNPReq` reader - HNP Request"]
pub struct HNPREQ_R(crate::FieldReader<bool, HNPREQ_A>);
impl HNPREQ_R {
    pub(crate) fn new(bits: bool) -> Self {
        HNPREQ_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
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
        **self == HNPREQ_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == HNPREQ_A::VALUE2
    }
}
impl core::ops::Deref for HNPREQ_R {
    type Target = crate::FieldReader<bool, HNPREQ_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HNPReq` writer - HNP Request"]
pub struct HNPREQ_W<'a> {
    w: &'a mut W,
}
impl<'a> HNPREQ_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HNPREQ_A) -> &'a mut W {
        self.bit(variant.into())
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
#[doc = "Host Set HNP Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HSTSETHNPEN_A {
    #[doc = "0: Host Set HNP is not enabled"]
    VALUE1 = 0,
    #[doc = "1: Host Set HNP is enabled"]
    VALUE2 = 1,
}
impl From<HSTSETHNPEN_A> for bool {
    #[inline(always)]
    fn from(variant: HSTSETHNPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HstSetHNPEn` reader - Host Set HNP Enable"]
pub struct HSTSETHNPEN_R(crate::FieldReader<bool, HSTSETHNPEN_A>);
impl HSTSETHNPEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        HSTSETHNPEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HSTSETHNPEN_A {
        match self.bits {
            false => HSTSETHNPEN_A::VALUE1,
            true => HSTSETHNPEN_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == HSTSETHNPEN_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == HSTSETHNPEN_A::VALUE2
    }
}
impl core::ops::Deref for HSTSETHNPEN_R {
    type Target = crate::FieldReader<bool, HSTSETHNPEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HstSetHNPEn` writer - Host Set HNP Enable"]
pub struct HSTSETHNPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> HSTSETHNPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HSTSETHNPEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Host Set HNP is not enabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(HSTSETHNPEN_A::VALUE1)
    }
    #[doc = "Host Set HNP is enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(HSTSETHNPEN_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
        self.w
    }
}
#[doc = "Device HNP Enabled\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DEVHNPEN_A {
    #[doc = "0: HNP is not enabled in the application"]
    VALUE1 = 0,
    #[doc = "1: HNP is enabled in the application"]
    VALUE2 = 1,
}
impl From<DEVHNPEN_A> for bool {
    #[inline(always)]
    fn from(variant: DEVHNPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DevHNPEn` reader - Device HNP Enabled"]
pub struct DEVHNPEN_R(crate::FieldReader<bool, DEVHNPEN_A>);
impl DEVHNPEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        DEVHNPEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DEVHNPEN_A {
        match self.bits {
            false => DEVHNPEN_A::VALUE1,
            true => DEVHNPEN_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == DEVHNPEN_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == DEVHNPEN_A::VALUE2
    }
}
impl core::ops::Deref for DEVHNPEN_R {
    type Target = crate::FieldReader<bool, DEVHNPEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DevHNPEn` writer - Device HNP Enabled"]
pub struct DEVHNPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DEVHNPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DEVHNPEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "HNP is not enabled in the application"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(DEVHNPEN_A::VALUE1)
    }
    #[doc = "HNP is enabled in the application"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(DEVHNPEN_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u32 & 0x01) << 11);
        self.w
    }
}
#[doc = "Connector ID Status\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CONLDSTS_A {
    #[doc = "0: The USB core is in A-Device mode"]
    VALUE1 = 0,
    #[doc = "1: The USB core is in B-Device mode"]
    VALUE2 = 1,
}
impl From<CONLDSTS_A> for bool {
    #[inline(always)]
    fn from(variant: CONLDSTS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ConlDSts` reader - Connector ID Status"]
pub struct CONLDSTS_R(crate::FieldReader<bool, CONLDSTS_A>);
impl CONLDSTS_R {
    pub(crate) fn new(bits: bool) -> Self {
        CONLDSTS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CONLDSTS_A {
        match self.bits {
            false => CONLDSTS_A::VALUE1,
            true => CONLDSTS_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == CONLDSTS_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == CONLDSTS_A::VALUE2
    }
}
impl core::ops::Deref for CONLDSTS_R {
    type Target = crate::FieldReader<bool, CONLDSTS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Long/Short Debounce Time\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DBNCTIME_A {
    #[doc = "0: Long debounce time, used for physical connections (100 ms + 2.5 us)"]
    VALUE1 = 0,
    #[doc = "1: Short debounce time, used for soft connections (2.5 us)"]
    VALUE2 = 1,
}
impl From<DBNCTIME_A> for bool {
    #[inline(always)]
    fn from(variant: DBNCTIME_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DbncTime` reader - Long/Short Debounce Time"]
pub struct DBNCTIME_R(crate::FieldReader<bool, DBNCTIME_A>);
impl DBNCTIME_R {
    pub(crate) fn new(bits: bool) -> Self {
        DBNCTIME_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DBNCTIME_A {
        match self.bits {
            false => DBNCTIME_A::VALUE1,
            true => DBNCTIME_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == DBNCTIME_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == DBNCTIME_A::VALUE2
    }
}
impl core::ops::Deref for DBNCTIME_R {
    type Target = crate::FieldReader<bool, DBNCTIME_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "A-Session Valid\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ASESVID_A {
    #[doc = "0: A-session is not valid"]
    VALUE1 = 0,
    #[doc = "1: A-session is valid"]
    VALUE2 = 1,
}
impl From<ASESVID_A> for bool {
    #[inline(always)]
    fn from(variant: ASESVID_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ASesVId` reader - A-Session Valid"]
pub struct ASESVID_R(crate::FieldReader<bool, ASESVID_A>);
impl ASESVID_R {
    pub(crate) fn new(bits: bool) -> Self {
        ASESVID_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ASESVID_A {
        match self.bits {
            false => ASESVID_A::VALUE1,
            true => ASESVID_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == ASESVID_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == ASESVID_A::VALUE2
    }
}
impl core::ops::Deref for ASESVID_R {
    type Target = crate::FieldReader<bool, ASESVID_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "B-Session Valid\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BSESVLD_A {
    #[doc = "0: B-session is not valid."]
    VALUE1 = 0,
    #[doc = "1: B-session is valid."]
    VALUE2 = 1,
}
impl From<BSESVLD_A> for bool {
    #[inline(always)]
    fn from(variant: BSESVLD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BSesVld` reader - B-Session Valid"]
pub struct BSESVLD_R(crate::FieldReader<bool, BSESVLD_A>);
impl BSESVLD_R {
    pub(crate) fn new(bits: bool) -> Self {
        BSESVLD_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BSESVLD_A {
        match self.bits {
            false => BSESVLD_A::VALUE1,
            true => BSESVLD_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == BSESVLD_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == BSESVLD_A::VALUE2
    }
}
impl core::ops::Deref for BSESVLD_R {
    type Target = crate::FieldReader<bool, BSESVLD_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "OTG Version\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Field `OTGVer` reader - OTG Version"]
pub struct OTGVER_R(crate::FieldReader<bool, OTGVER_A>);
impl OTGVER_R {
    pub(crate) fn new(bits: bool) -> Self {
        OTGVER_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
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
        **self == OTGVER_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == OTGVER_A::VALUE2
    }
}
impl core::ops::Deref for OTGVER_R {
    type Target = crate::FieldReader<bool, OTGVER_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OTGVer` writer - OTG Version"]
pub struct OTGVER_W<'a> {
    w: &'a mut W,
}
impl<'a> OTGVER_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OTGVER_A) -> &'a mut W {
        self.bit(variant.into())
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | ((value as u32 & 0x01) << 20);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Session Request Success"]
    #[inline(always)]
    pub fn ses_req_scs(&self) -> SESREQSCS_R {
        SESREQSCS_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Session Request"]
    #[inline(always)]
    pub fn ses_req(&self) -> SESREQ_R {
        SESREQ_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - VBUS Valid Override Enable"]
    #[inline(always)]
    pub fn vbvalid_ov_en(&self) -> VBVALIDOVEN_R {
        VBVALIDOVEN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - VBUS Valid Override Value"]
    #[inline(always)]
    pub fn vbvalid_ov_val(&self) -> VBVALIDOVVAL_R {
        VBVALIDOVVAL_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - A-Peripheral Session Valid Override Enable"]
    #[inline(always)]
    pub fn avalid_ov_en(&self) -> AVALIDOVEN_R {
        AVALIDOVEN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - A-Peripheral Session Valid Override Value"]
    #[inline(always)]
    pub fn avalid_ov_val(&self) -> AVALIDOVVAL_R {
        AVALIDOVVAL_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - B-Peripheral Session Valid Override Enable"]
    #[inline(always)]
    pub fn bvalid_ov_en(&self) -> BVALIDOVEN_R {
        BVALIDOVEN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - B-Peripheral Session Valid Override Value"]
    #[inline(always)]
    pub fn bvalid_ov_val(&self) -> BVALIDOVVAL_R {
        BVALIDOVVAL_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Host Negotiation Success"]
    #[inline(always)]
    pub fn hst_neg_scs(&self) -> HSTNEGSCS_R {
        HSTNEGSCS_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - HNP Request"]
    #[inline(always)]
    pub fn hnpreq(&self) -> HNPREQ_R {
        HNPREQ_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Host Set HNP Enable"]
    #[inline(always)]
    pub fn hst_set_hnpen(&self) -> HSTSETHNPEN_R {
        HSTSETHNPEN_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Device HNP Enabled"]
    #[inline(always)]
    pub fn dev_hnpen(&self) -> DEVHNPEN_R {
        DEVHNPEN_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Connector ID Status"]
    #[inline(always)]
    pub fn conl_dsts(&self) -> CONLDSTS_R {
        CONLDSTS_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Long/Short Debounce Time"]
    #[inline(always)]
    pub fn dbnc_time(&self) -> DBNCTIME_R {
        DBNCTIME_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - A-Session Valid"]
    #[inline(always)]
    pub fn ases_vid(&self) -> ASESVID_R {
        ASESVID_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - B-Session Valid"]
    #[inline(always)]
    pub fn bses_vld(&self) -> BSESVLD_R {
        BSESVLD_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - OTG Version"]
    #[inline(always)]
    pub fn otgver(&self) -> OTGVER_R {
        OTGVER_R::new(((self.bits >> 20) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Session Request"]
    #[inline(always)]
    pub fn ses_req(&mut self) -> SESREQ_W {
        SESREQ_W { w: self }
    }
    #[doc = "Bit 2 - VBUS Valid Override Enable"]
    #[inline(always)]
    pub fn vbvalid_ov_en(&mut self) -> VBVALIDOVEN_W {
        VBVALIDOVEN_W { w: self }
    }
    #[doc = "Bit 3 - VBUS Valid Override Value"]
    #[inline(always)]
    pub fn vbvalid_ov_val(&mut self) -> VBVALIDOVVAL_W {
        VBVALIDOVVAL_W { w: self }
    }
    #[doc = "Bit 4 - A-Peripheral Session Valid Override Enable"]
    #[inline(always)]
    pub fn avalid_ov_en(&mut self) -> AVALIDOVEN_W {
        AVALIDOVEN_W { w: self }
    }
    #[doc = "Bit 5 - A-Peripheral Session Valid Override Value"]
    #[inline(always)]
    pub fn avalid_ov_val(&mut self) -> AVALIDOVVAL_W {
        AVALIDOVVAL_W { w: self }
    }
    #[doc = "Bit 6 - B-Peripheral Session Valid Override Enable"]
    #[inline(always)]
    pub fn bvalid_ov_en(&mut self) -> BVALIDOVEN_W {
        BVALIDOVEN_W { w: self }
    }
    #[doc = "Bit 7 - B-Peripheral Session Valid Override Value"]
    #[inline(always)]
    pub fn bvalid_ov_val(&mut self) -> BVALIDOVVAL_W {
        BVALIDOVVAL_W { w: self }
    }
    #[doc = "Bit 9 - HNP Request"]
    #[inline(always)]
    pub fn hnpreq(&mut self) -> HNPREQ_W {
        HNPREQ_W { w: self }
    }
    #[doc = "Bit 10 - Host Set HNP Enable"]
    #[inline(always)]
    pub fn hst_set_hnpen(&mut self) -> HSTSETHNPEN_W {
        HSTSETHNPEN_W { w: self }
    }
    #[doc = "Bit 11 - Device HNP Enabled"]
    #[inline(always)]
    pub fn dev_hnpen(&mut self) -> DEVHNPEN_W {
        DEVHNPEN_W { w: self }
    }
    #[doc = "Bit 20 - OTG Version"]
    #[inline(always)]
    pub fn otgver(&mut self) -> OTGVER_W {
        OTGVER_W { w: self }
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
}
#[doc = "`reset()` method sets GOTGCTL to value 0x0001_0000"]
impl crate::Resettable for GOTGCTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0001_0000
    }
}
