#[doc = "Register `FNCTL` reader"]
pub struct R(crate::R<FNCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FNCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FNCTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FNCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FNCTL` writer"]
pub struct W(crate::W<FNCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FNCTL_SPEC>;
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
impl From<crate::W<FNCTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FNCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Touch-Sense TSIN Pad Turn\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PADT_A {
    #[doc = "0: TSIN0"]
    VALUE1 = 0,
    #[doc = "7: TSIN7"]
    VALUE2 = 7,
}
impl From<PADT_A> for u8 {
    #[inline(always)]
    fn from(variant: PADT_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PADT` reader - Touch-Sense TSIN Pad Turn"]
pub struct PADT_R(crate::FieldReader<u8, PADT_A>);
impl PADT_R {
    pub(crate) fn new(bits: u8) -> Self {
        PADT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PADT_A> {
        match self.bits {
            0 => Some(PADT_A::VALUE1),
            7 => Some(PADT_A::VALUE2),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == PADT_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == PADT_A::VALUE2
    }
}
impl core::ops::Deref for PADT_R {
    type Target = crate::FieldReader<u8, PADT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PADT` writer - Touch-Sense TSIN Pad Turn"]
pub struct PADT_W<'a> {
    w: &'a mut W,
}
impl<'a> PADT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PADT_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "TSIN0"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(PADT_A::VALUE1)
    }
    #[doc = "TSIN7"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(PADT_A::VALUE2)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u32 & 0x07);
        self.w
    }
}
#[doc = "Software Control for Touch-Sense Pad Turn\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PADTSW_A {
    #[doc = "0: The hardware automatically enables the touch-sense inputs in sequence round-robin, starting from TSIN0."]
    VALUE1 = 0,
    #[doc = "1: Disable hardware control for software control only. The touch-sense input is configured in bit PADT."]
    VALUE2 = 1,
}
impl From<PADTSW_A> for bool {
    #[inline(always)]
    fn from(variant: PADTSW_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PADTSW` reader - Software Control for Touch-Sense Pad Turn"]
pub struct PADTSW_R(crate::FieldReader<bool, PADTSW_A>);
impl PADTSW_R {
    pub(crate) fn new(bits: bool) -> Self {
        PADTSW_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PADTSW_A {
        match self.bits {
            false => PADTSW_A::VALUE1,
            true => PADTSW_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == PADTSW_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == PADTSW_A::VALUE2
    }
}
impl core::ops::Deref for PADTSW_R {
    type Target = crate::FieldReader<bool, PADTSW_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PADTSW` writer - Software Control for Touch-Sense Pad Turn"]
pub struct PADTSW_W<'a> {
    w: &'a mut W,
}
impl<'a> PADTSW_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PADTSW_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The hardware automatically enables the touch-sense inputs in sequence round-robin, starting from TSIN0."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(PADTSW_A::VALUE1)
    }
    #[doc = "Disable hardware control for software control only. The touch-sense input is configured in bit PADT."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(PADTSW_A::VALUE2)
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
#[doc = "Enable External Pull-up Configuration on Pin COLA\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EPULL_A {
    #[doc = "0: HW over-rule to enable internal pull-up is active on TSIN\\[x\\]
for set duration in touch-sense time slice. With this setting, it is not specified to assign the COLA to any pin."]
    VALUE1 = 0,
    #[doc = "1: Enable external pull-up: Output 1 on pin COLA for whole duration of touch-sense time slice."]
    VALUE2 = 1,
}
impl From<EPULL_A> for bool {
    #[inline(always)]
    fn from(variant: EPULL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EPULL` reader - Enable External Pull-up Configuration on Pin COLA"]
pub struct EPULL_R(crate::FieldReader<bool, EPULL_A>);
impl EPULL_R {
    pub(crate) fn new(bits: bool) -> Self {
        EPULL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EPULL_A {
        match self.bits {
            false => EPULL_A::VALUE1,
            true => EPULL_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == EPULL_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == EPULL_A::VALUE2
    }
}
impl core::ops::Deref for EPULL_R {
    type Target = crate::FieldReader<bool, EPULL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPULL` writer - Enable External Pull-up Configuration on Pin COLA"]
pub struct EPULL_W<'a> {
    w: &'a mut W,
}
impl<'a> EPULL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EPULL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "HW over-rule to enable internal pull-up is active on TSIN\\[x\\]
for set duration in touch-sense time slice. With this setting, it is not specified to assign the COLA to any pin."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(EPULL_A::VALUE1)
    }
    #[doc = "Enable external pull-up: Output 1 on pin COLA for whole duration of touch-sense time slice."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(EPULL_A::VALUE2)
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
#[doc = "Field `FNCOL` reader - Previous Active Function/LED Column Status"]
pub struct FNCOL_R(crate::FieldReader<u8, u8>);
impl FNCOL_R {
    pub(crate) fn new(bits: u8) -> Self {
        FNCOL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FNCOL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Accumulate Count on Touch-Sense Input\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ACCCNT_A {
    #[doc = "0: 1 time"]
    VALUE1 = 0,
    #[doc = "1: 2 times"]
    VALUE2 = 1,
    #[doc = "15: 16 times"]
    VALUE3 = 15,
}
impl From<ACCCNT_A> for u8 {
    #[inline(always)]
    fn from(variant: ACCCNT_A) -> Self {
        variant as _
    }
}
#[doc = "Field `ACCCNT` reader - Accumulate Count on Touch-Sense Input"]
pub struct ACCCNT_R(crate::FieldReader<u8, ACCCNT_A>);
impl ACCCNT_R {
    pub(crate) fn new(bits: u8) -> Self {
        ACCCNT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<ACCCNT_A> {
        match self.bits {
            0 => Some(ACCCNT_A::VALUE1),
            1 => Some(ACCCNT_A::VALUE2),
            15 => Some(ACCCNT_A::VALUE3),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == ACCCNT_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == ACCCNT_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        **self == ACCCNT_A::VALUE3
    }
}
impl core::ops::Deref for ACCCNT_R {
    type Target = crate::FieldReader<u8, ACCCNT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ACCCNT` writer - Accumulate Count on Touch-Sense Input"]
pub struct ACCCNT_W<'a> {
    w: &'a mut W,
}
impl<'a> ACCCNT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ACCCNT_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "1 time"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(ACCCNT_A::VALUE1)
    }
    #[doc = "2 times"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(ACCCNT_A::VALUE2)
    }
    #[doc = "16 times"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(ACCCNT_A::VALUE3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | ((value as u32 & 0x0f) << 16);
        self.w
    }
}
#[doc = "Common Compare Enable for Touch-Sense\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TSCCMP_A {
    #[doc = "0: Disable common compare for touch-sense"]
    VALUE1 = 0,
    #[doc = "1: Enable common compare for touch-sense"]
    VALUE2 = 1,
}
impl From<TSCCMP_A> for bool {
    #[inline(always)]
    fn from(variant: TSCCMP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TSCCMP` reader - Common Compare Enable for Touch-Sense"]
pub struct TSCCMP_R(crate::FieldReader<bool, TSCCMP_A>);
impl TSCCMP_R {
    pub(crate) fn new(bits: bool) -> Self {
        TSCCMP_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TSCCMP_A {
        match self.bits {
            false => TSCCMP_A::VALUE1,
            true => TSCCMP_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == TSCCMP_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == TSCCMP_A::VALUE2
    }
}
impl core::ops::Deref for TSCCMP_R {
    type Target = crate::FieldReader<bool, TSCCMP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TSCCMP` writer - Common Compare Enable for Touch-Sense"]
pub struct TSCCMP_W<'a> {
    w: &'a mut W,
}
impl<'a> TSCCMP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TSCCMP_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable common compare for touch-sense"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(TSCCMP_A::VALUE1)
    }
    #[doc = "Enable common compare for touch-sense"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(TSCCMP_A::VALUE2)
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
#[doc = "Extension for Touch-Sense Output for Pin-Low-Level\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TSOEXT_A {
    #[doc = "0: Extend by 1 ledts_clk"]
    VALUE1 = 0,
    #[doc = "1: Extend by 4 ledts_clk"]
    VALUE2 = 1,
    #[doc = "2: Extend by 8 ledts_clk"]
    VALUE3 = 2,
    #[doc = "3: Extend by 16 ledts_clk"]
    VALUE4 = 3,
}
impl From<TSOEXT_A> for u8 {
    #[inline(always)]
    fn from(variant: TSOEXT_A) -> Self {
        variant as _
    }
}
#[doc = "Field `TSOEXT` reader - Extension for Touch-Sense Output for Pin-Low-Level"]
pub struct TSOEXT_R(crate::FieldReader<u8, TSOEXT_A>);
impl TSOEXT_R {
    pub(crate) fn new(bits: u8) -> Self {
        TSOEXT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TSOEXT_A {
        match self.bits {
            0 => TSOEXT_A::VALUE1,
            1 => TSOEXT_A::VALUE2,
            2 => TSOEXT_A::VALUE3,
            3 => TSOEXT_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == TSOEXT_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == TSOEXT_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        **self == TSOEXT_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        **self == TSOEXT_A::VALUE4
    }
}
impl core::ops::Deref for TSOEXT_R {
    type Target = crate::FieldReader<u8, TSOEXT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TSOEXT` writer - Extension for Touch-Sense Output for Pin-Low-Level"]
pub struct TSOEXT_W<'a> {
    w: &'a mut W,
}
impl<'a> TSOEXT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TSOEXT_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Extend by 1 ledts_clk"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(TSOEXT_A::VALUE1)
    }
    #[doc = "Extend by 4 ledts_clk"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(TSOEXT_A::VALUE2)
    }
    #[doc = "Extend by 8 ledts_clk"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(TSOEXT_A::VALUE3)
    }
    #[doc = "Extend by 16 ledts_clk"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(TSOEXT_A::VALUE4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 21)) | ((value as u32 & 0x03) << 21);
        self.w
    }
}
#[doc = "TS-Counter Auto Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TSCTRR_A {
    #[doc = "0: Disable TS-counter automatic reset"]
    VALUE1 = 0,
    #[doc = "1: Enable TS-counter automatic reset to 00H on the first pad turn of a new TSIN\\[x\\]. Triggered on compare match in time slice."]
    VALUE2 = 1,
}
impl From<TSCTRR_A> for bool {
    #[inline(always)]
    fn from(variant: TSCTRR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TSCTRR` reader - TS-Counter Auto Reset"]
pub struct TSCTRR_R(crate::FieldReader<bool, TSCTRR_A>);
impl TSCTRR_R {
    pub(crate) fn new(bits: bool) -> Self {
        TSCTRR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TSCTRR_A {
        match self.bits {
            false => TSCTRR_A::VALUE1,
            true => TSCTRR_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == TSCTRR_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == TSCTRR_A::VALUE2
    }
}
impl core::ops::Deref for TSCTRR_R {
    type Target = crate::FieldReader<bool, TSCTRR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TSCTRR` writer - TS-Counter Auto Reset"]
pub struct TSCTRR_W<'a> {
    w: &'a mut W,
}
impl<'a> TSCTRR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TSCTRR_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable TS-counter automatic reset"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(TSCTRR_A::VALUE1)
    }
    #[doc = "Enable TS-counter automatic reset to 00H on the first pad turn of a new TSIN\\[x\\]. Triggered on compare match in time slice."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(TSCTRR_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | ((value as u32 & 0x01) << 23);
        self.w
    }
}
#[doc = "Saturation of TS-Counter\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TSCTRSAT_A {
    #[doc = "0: Disable"]
    VALUE1 = 0,
    #[doc = "1: Enable. TS-counter stops counting in the touch-sense time slice(s) of the same (extended) frame when it reaches FFH. Counter starts to count again on the first pad turn of a new TSIN\\[x\\], triggered on compare match."]
    VALUE2 = 1,
}
impl From<TSCTRSAT_A> for bool {
    #[inline(always)]
    fn from(variant: TSCTRSAT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TSCTRSAT` reader - Saturation of TS-Counter"]
pub struct TSCTRSAT_R(crate::FieldReader<bool, TSCTRSAT_A>);
impl TSCTRSAT_R {
    pub(crate) fn new(bits: bool) -> Self {
        TSCTRSAT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TSCTRSAT_A {
        match self.bits {
            false => TSCTRSAT_A::VALUE1,
            true => TSCTRSAT_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == TSCTRSAT_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == TSCTRSAT_A::VALUE2
    }
}
impl core::ops::Deref for TSCTRSAT_R {
    type Target = crate::FieldReader<bool, TSCTRSAT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TSCTRSAT` writer - Saturation of TS-Counter"]
pub struct TSCTRSAT_W<'a> {
    w: &'a mut W,
}
impl<'a> TSCTRSAT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TSCTRSAT_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(TSCTRSAT_A::VALUE1)
    }
    #[doc = "Enable. TS-counter stops counting in the touch-sense time slice(s) of the same (extended) frame when it reaches FFH. Counter starts to count again on the first pad turn of a new TSIN\\[x\\], triggered on compare match."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(TSCTRSAT_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | ((value as u32 & 0x01) << 24);
        self.w
    }
}
#[doc = "Number of Touch-Sense Input\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum NR_TSIN_A {
    #[doc = "0: 1"]
    VALUE1 = 0,
    #[doc = "7: 8"]
    VALUE2 = 7,
}
impl From<NR_TSIN_A> for u8 {
    #[inline(always)]
    fn from(variant: NR_TSIN_A) -> Self {
        variant as _
    }
}
#[doc = "Field `NR_TSIN` reader - Number of Touch-Sense Input"]
pub struct NR_TSIN_R(crate::FieldReader<u8, NR_TSIN_A>);
impl NR_TSIN_R {
    pub(crate) fn new(bits: u8) -> Self {
        NR_TSIN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<NR_TSIN_A> {
        match self.bits {
            0 => Some(NR_TSIN_A::VALUE1),
            7 => Some(NR_TSIN_A::VALUE2),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == NR_TSIN_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == NR_TSIN_A::VALUE2
    }
}
impl core::ops::Deref for NR_TSIN_R {
    type Target = crate::FieldReader<u8, NR_TSIN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NR_TSIN` writer - Number of Touch-Sense Input"]
pub struct NR_TSIN_W<'a> {
    w: &'a mut W,
}
impl<'a> NR_TSIN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: NR_TSIN_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "1"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(NR_TSIN_A::VALUE1)
    }
    #[doc = "8"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(NR_TSIN_A::VALUE2)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 25)) | ((value as u32 & 0x07) << 25);
        self.w
    }
}
#[doc = "Active Level of LED Column\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COLLEV_A {
    #[doc = "0: Active low"]
    VALUE1 = 0,
    #[doc = "1: Active high"]
    VALUE2 = 1,
}
impl From<COLLEV_A> for bool {
    #[inline(always)]
    fn from(variant: COLLEV_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COLLEV` reader - Active Level of LED Column"]
pub struct COLLEV_R(crate::FieldReader<bool, COLLEV_A>);
impl COLLEV_R {
    pub(crate) fn new(bits: bool) -> Self {
        COLLEV_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COLLEV_A {
        match self.bits {
            false => COLLEV_A::VALUE1,
            true => COLLEV_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == COLLEV_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == COLLEV_A::VALUE2
    }
}
impl core::ops::Deref for COLLEV_R {
    type Target = crate::FieldReader<bool, COLLEV_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COLLEV` writer - Active Level of LED Column"]
pub struct COLLEV_W<'a> {
    w: &'a mut W,
}
impl<'a> COLLEV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: COLLEV_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Active low"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(COLLEV_A::VALUE1)
    }
    #[doc = "Active high"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(COLLEV_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | ((value as u32 & 0x01) << 28);
        self.w
    }
}
#[doc = "Number of LED Columns\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum NR_LEDCOL_A {
    #[doc = "0: 1 LED column"]
    VALUE1 = 0,
    #[doc = "1: 2 LED columns"]
    VALUE2 = 1,
    #[doc = "2: 3 LED columns"]
    VALUE3 = 2,
    #[doc = "3: 4 LED columns"]
    VALUE4 = 3,
    #[doc = "4: 5 LED columns"]
    VALUE5 = 4,
    #[doc = "5: 6 LED columns"]
    VALUE6 = 5,
    #[doc = "6: 7 LED columns"]
    VALUE7 = 6,
    #[doc = "7: 8 LED columns (max. LED columns = 7 if bit TS_EN = 1)"]
    VALUE8 = 7,
}
impl From<NR_LEDCOL_A> for u8 {
    #[inline(always)]
    fn from(variant: NR_LEDCOL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `NR_LEDCOL` reader - Number of LED Columns"]
pub struct NR_LEDCOL_R(crate::FieldReader<u8, NR_LEDCOL_A>);
impl NR_LEDCOL_R {
    pub(crate) fn new(bits: u8) -> Self {
        NR_LEDCOL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NR_LEDCOL_A {
        match self.bits {
            0 => NR_LEDCOL_A::VALUE1,
            1 => NR_LEDCOL_A::VALUE2,
            2 => NR_LEDCOL_A::VALUE3,
            3 => NR_LEDCOL_A::VALUE4,
            4 => NR_LEDCOL_A::VALUE5,
            5 => NR_LEDCOL_A::VALUE6,
            6 => NR_LEDCOL_A::VALUE7,
            7 => NR_LEDCOL_A::VALUE8,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == NR_LEDCOL_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == NR_LEDCOL_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        **self == NR_LEDCOL_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        **self == NR_LEDCOL_A::VALUE4
    }
    #[doc = "Checks if the value of the field is `VALUE5`"]
    #[inline(always)]
    pub fn is_value5(&self) -> bool {
        **self == NR_LEDCOL_A::VALUE5
    }
    #[doc = "Checks if the value of the field is `VALUE6`"]
    #[inline(always)]
    pub fn is_value6(&self) -> bool {
        **self == NR_LEDCOL_A::VALUE6
    }
    #[doc = "Checks if the value of the field is `VALUE7`"]
    #[inline(always)]
    pub fn is_value7(&self) -> bool {
        **self == NR_LEDCOL_A::VALUE7
    }
    #[doc = "Checks if the value of the field is `VALUE8`"]
    #[inline(always)]
    pub fn is_value8(&self) -> bool {
        **self == NR_LEDCOL_A::VALUE8
    }
}
impl core::ops::Deref for NR_LEDCOL_R {
    type Target = crate::FieldReader<u8, NR_LEDCOL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NR_LEDCOL` writer - Number of LED Columns"]
pub struct NR_LEDCOL_W<'a> {
    w: &'a mut W,
}
impl<'a> NR_LEDCOL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: NR_LEDCOL_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "1 LED column"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(NR_LEDCOL_A::VALUE1)
    }
    #[doc = "2 LED columns"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(NR_LEDCOL_A::VALUE2)
    }
    #[doc = "3 LED columns"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(NR_LEDCOL_A::VALUE3)
    }
    #[doc = "4 LED columns"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(NR_LEDCOL_A::VALUE4)
    }
    #[doc = "5 LED columns"]
    #[inline(always)]
    pub fn value5(self) -> &'a mut W {
        self.variant(NR_LEDCOL_A::VALUE5)
    }
    #[doc = "6 LED columns"]
    #[inline(always)]
    pub fn value6(self) -> &'a mut W {
        self.variant(NR_LEDCOL_A::VALUE6)
    }
    #[doc = "7 LED columns"]
    #[inline(always)]
    pub fn value7(self) -> &'a mut W {
        self.variant(NR_LEDCOL_A::VALUE7)
    }
    #[doc = "8 LED columns (max. LED columns = 7 if bit TS_EN = 1)"]
    #[inline(always)]
    pub fn value8(self) -> &'a mut W {
        self.variant(NR_LEDCOL_A::VALUE8)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 29)) | ((value as u32 & 0x07) << 29);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Touch-Sense TSIN Pad Turn"]
    #[inline(always)]
    pub fn padt(&self) -> PADT_R {
        PADT_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bit 3 - Software Control for Touch-Sense Pad Turn"]
    #[inline(always)]
    pub fn padtsw(&self) -> PADTSW_R {
        PADTSW_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Enable External Pull-up Configuration on Pin COLA"]
    #[inline(always)]
    pub fn epull(&self) -> EPULL_R {
        EPULL_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 5:7 - Previous Active Function/LED Column Status"]
    #[inline(always)]
    pub fn fncol(&self) -> FNCOL_R {
        FNCOL_R::new(((self.bits >> 5) & 0x07) as u8)
    }
    #[doc = "Bits 16:19 - Accumulate Count on Touch-Sense Input"]
    #[inline(always)]
    pub fn acccnt(&self) -> ACCCNT_R {
        ACCCNT_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 20 - Common Compare Enable for Touch-Sense"]
    #[inline(always)]
    pub fn tsccmp(&self) -> TSCCMP_R {
        TSCCMP_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bits 21:22 - Extension for Touch-Sense Output for Pin-Low-Level"]
    #[inline(always)]
    pub fn tsoext(&self) -> TSOEXT_R {
        TSOEXT_R::new(((self.bits >> 21) & 0x03) as u8)
    }
    #[doc = "Bit 23 - TS-Counter Auto Reset"]
    #[inline(always)]
    pub fn tsctrr(&self) -> TSCTRR_R {
        TSCTRR_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Saturation of TS-Counter"]
    #[inline(always)]
    pub fn tsctrsat(&self) -> TSCTRSAT_R {
        TSCTRSAT_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bits 25:27 - Number of Touch-Sense Input"]
    #[inline(always)]
    pub fn nr_tsin(&self) -> NR_TSIN_R {
        NR_TSIN_R::new(((self.bits >> 25) & 0x07) as u8)
    }
    #[doc = "Bit 28 - Active Level of LED Column"]
    #[inline(always)]
    pub fn collev(&self) -> COLLEV_R {
        COLLEV_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bits 29:31 - Number of LED Columns"]
    #[inline(always)]
    pub fn nr_ledcol(&self) -> NR_LEDCOL_R {
        NR_LEDCOL_R::new(((self.bits >> 29) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Touch-Sense TSIN Pad Turn"]
    #[inline(always)]
    pub fn padt(&mut self) -> PADT_W {
        PADT_W { w: self }
    }
    #[doc = "Bit 3 - Software Control for Touch-Sense Pad Turn"]
    #[inline(always)]
    pub fn padtsw(&mut self) -> PADTSW_W {
        PADTSW_W { w: self }
    }
    #[doc = "Bit 4 - Enable External Pull-up Configuration on Pin COLA"]
    #[inline(always)]
    pub fn epull(&mut self) -> EPULL_W {
        EPULL_W { w: self }
    }
    #[doc = "Bits 16:19 - Accumulate Count on Touch-Sense Input"]
    #[inline(always)]
    pub fn acccnt(&mut self) -> ACCCNT_W {
        ACCCNT_W { w: self }
    }
    #[doc = "Bit 20 - Common Compare Enable for Touch-Sense"]
    #[inline(always)]
    pub fn tsccmp(&mut self) -> TSCCMP_W {
        TSCCMP_W { w: self }
    }
    #[doc = "Bits 21:22 - Extension for Touch-Sense Output for Pin-Low-Level"]
    #[inline(always)]
    pub fn tsoext(&mut self) -> TSOEXT_W {
        TSOEXT_W { w: self }
    }
    #[doc = "Bit 23 - TS-Counter Auto Reset"]
    #[inline(always)]
    pub fn tsctrr(&mut self) -> TSCTRR_W {
        TSCTRR_W { w: self }
    }
    #[doc = "Bit 24 - Saturation of TS-Counter"]
    #[inline(always)]
    pub fn tsctrsat(&mut self) -> TSCTRSAT_W {
        TSCTRSAT_W { w: self }
    }
    #[doc = "Bits 25:27 - Number of Touch-Sense Input"]
    #[inline(always)]
    pub fn nr_tsin(&mut self) -> NR_TSIN_W {
        NR_TSIN_W { w: self }
    }
    #[doc = "Bit 28 - Active Level of LED Column"]
    #[inline(always)]
    pub fn collev(&mut self) -> COLLEV_W {
        COLLEV_W { w: self }
    }
    #[doc = "Bits 29:31 - Number of LED Columns"]
    #[inline(always)]
    pub fn nr_ledcol(&mut self) -> NR_LEDCOL_W {
        NR_LEDCOL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Function Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fnctl](index.html) module"]
pub struct FNCTL_SPEC;
impl crate::RegisterSpec for FNCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fnctl::R](R) reader structure"]
impl crate::Readable for FNCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fnctl::W](W) writer structure"]
impl crate::Writable for FNCTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FNCTL to value 0"]
impl crate::Resettable for FNCTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
