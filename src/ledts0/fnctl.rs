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
#[doc = "Field `PADT` reader - Touch-Sense TSIN Pad Turn"]
pub type PADT_R = crate::FieldReader<u8, PADT_A>;
#[doc = "Touch-Sense TSIN Pad Turn\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl PADT_R {
    #[doc = "Get enumerated values variant"]
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
        *self == PADT_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PADT_A::VALUE2
    }
}
#[doc = "Field `PADT` writer - Touch-Sense TSIN Pad Turn"]
pub type PADT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FNCTL_SPEC, u8, PADT_A, 3, O>;
impl<'a, const O: u8> PADT_W<'a, O> {
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
}
#[doc = "Field `PADTSW` reader - Software Control for Touch-Sense Pad Turn"]
pub type PADTSW_R = crate::BitReader<PADTSW_A>;
#[doc = "Software Control for Touch-Sense Pad Turn\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl PADTSW_R {
    #[doc = "Get enumerated values variant"]
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
        *self == PADTSW_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PADTSW_A::VALUE2
    }
}
#[doc = "Field `PADTSW` writer - Software Control for Touch-Sense Pad Turn"]
pub type PADTSW_W<'a, const O: u8> = crate::BitWriter<'a, u32, FNCTL_SPEC, PADTSW_A, O>;
impl<'a, const O: u8> PADTSW_W<'a, O> {
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
}
#[doc = "Field `EPULL` reader - Enable External Pull-up Configuration on Pin COLA"]
pub type EPULL_R = crate::BitReader<EPULL_A>;
#[doc = "Enable External Pull-up Configuration on Pin COLA\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl EPULL_R {
    #[doc = "Get enumerated values variant"]
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
        *self == EPULL_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == EPULL_A::VALUE2
    }
}
#[doc = "Field `EPULL` writer - Enable External Pull-up Configuration on Pin COLA"]
pub type EPULL_W<'a, const O: u8> = crate::BitWriter<'a, u32, FNCTL_SPEC, EPULL_A, O>;
impl<'a, const O: u8> EPULL_W<'a, O> {
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
}
#[doc = "Field `FNCOL` reader - Previous Active Function/LED Column Status"]
pub type FNCOL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ACCCNT` reader - Accumulate Count on Touch-Sense Input"]
pub type ACCCNT_R = crate::FieldReader<u8, ACCCNT_A>;
#[doc = "Accumulate Count on Touch-Sense Input\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl ACCCNT_R {
    #[doc = "Get enumerated values variant"]
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
        *self == ACCCNT_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == ACCCNT_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == ACCCNT_A::VALUE3
    }
}
#[doc = "Field `ACCCNT` writer - Accumulate Count on Touch-Sense Input"]
pub type ACCCNT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FNCTL_SPEC, u8, ACCCNT_A, 4, O>;
impl<'a, const O: u8> ACCCNT_W<'a, O> {
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
}
#[doc = "Field `TSCCMP` reader - Common Compare Enable for Touch-Sense"]
pub type TSCCMP_R = crate::BitReader<TSCCMP_A>;
#[doc = "Common Compare Enable for Touch-Sense\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl TSCCMP_R {
    #[doc = "Get enumerated values variant"]
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
        *self == TSCCMP_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == TSCCMP_A::VALUE2
    }
}
#[doc = "Field `TSCCMP` writer - Common Compare Enable for Touch-Sense"]
pub type TSCCMP_W<'a, const O: u8> = crate::BitWriter<'a, u32, FNCTL_SPEC, TSCCMP_A, O>;
impl<'a, const O: u8> TSCCMP_W<'a, O> {
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
}
#[doc = "Field `TSOEXT` reader - Extension for Touch-Sense Output for Pin-Low-Level"]
pub type TSOEXT_R = crate::FieldReader<u8, TSOEXT_A>;
#[doc = "Extension for Touch-Sense Output for Pin-Low-Level\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl TSOEXT_R {
    #[doc = "Get enumerated values variant"]
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
        *self == TSOEXT_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == TSOEXT_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == TSOEXT_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == TSOEXT_A::VALUE4
    }
}
#[doc = "Field `TSOEXT` writer - Extension for Touch-Sense Output for Pin-Low-Level"]
pub type TSOEXT_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, FNCTL_SPEC, u8, TSOEXT_A, 2, O>;
impl<'a, const O: u8> TSOEXT_W<'a, O> {
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
}
#[doc = "Field `TSCTRR` reader - TS-Counter Auto Reset"]
pub type TSCTRR_R = crate::BitReader<TSCTRR_A>;
#[doc = "TS-Counter Auto Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl TSCTRR_R {
    #[doc = "Get enumerated values variant"]
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
        *self == TSCTRR_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == TSCTRR_A::VALUE2
    }
}
#[doc = "Field `TSCTRR` writer - TS-Counter Auto Reset"]
pub type TSCTRR_W<'a, const O: u8> = crate::BitWriter<'a, u32, FNCTL_SPEC, TSCTRR_A, O>;
impl<'a, const O: u8> TSCTRR_W<'a, O> {
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
}
#[doc = "Field `TSCTRSAT` reader - Saturation of TS-Counter"]
pub type TSCTRSAT_R = crate::BitReader<TSCTRSAT_A>;
#[doc = "Saturation of TS-Counter\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl TSCTRSAT_R {
    #[doc = "Get enumerated values variant"]
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
        *self == TSCTRSAT_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == TSCTRSAT_A::VALUE2
    }
}
#[doc = "Field `TSCTRSAT` writer - Saturation of TS-Counter"]
pub type TSCTRSAT_W<'a, const O: u8> = crate::BitWriter<'a, u32, FNCTL_SPEC, TSCTRSAT_A, O>;
impl<'a, const O: u8> TSCTRSAT_W<'a, O> {
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
}
#[doc = "Field `NR_TSIN` reader - Number of Touch-Sense Input"]
pub type NR_TSIN_R = crate::FieldReader<u8, NR_TSIN_A>;
#[doc = "Number of Touch-Sense Input\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl NR_TSIN_R {
    #[doc = "Get enumerated values variant"]
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
        *self == NR_TSIN_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == NR_TSIN_A::VALUE2
    }
}
#[doc = "Field `NR_TSIN` writer - Number of Touch-Sense Input"]
pub type NR_TSIN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FNCTL_SPEC, u8, NR_TSIN_A, 3, O>;
impl<'a, const O: u8> NR_TSIN_W<'a, O> {
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
}
#[doc = "Field `COLLEV` reader - Active Level of LED Column"]
pub type COLLEV_R = crate::BitReader<COLLEV_A>;
#[doc = "Active Level of LED Column\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl COLLEV_R {
    #[doc = "Get enumerated values variant"]
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
        *self == COLLEV_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == COLLEV_A::VALUE2
    }
}
#[doc = "Field `COLLEV` writer - Active Level of LED Column"]
pub type COLLEV_W<'a, const O: u8> = crate::BitWriter<'a, u32, FNCTL_SPEC, COLLEV_A, O>;
impl<'a, const O: u8> COLLEV_W<'a, O> {
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
}
#[doc = "Field `NR_LEDCOL` reader - Number of LED Columns"]
pub type NR_LEDCOL_R = crate::FieldReader<u8, NR_LEDCOL_A>;
#[doc = "Number of LED Columns\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl NR_LEDCOL_R {
    #[doc = "Get enumerated values variant"]
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
        *self == NR_LEDCOL_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == NR_LEDCOL_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == NR_LEDCOL_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == NR_LEDCOL_A::VALUE4
    }
    #[doc = "Checks if the value of the field is `VALUE5`"]
    #[inline(always)]
    pub fn is_value5(&self) -> bool {
        *self == NR_LEDCOL_A::VALUE5
    }
    #[doc = "Checks if the value of the field is `VALUE6`"]
    #[inline(always)]
    pub fn is_value6(&self) -> bool {
        *self == NR_LEDCOL_A::VALUE6
    }
    #[doc = "Checks if the value of the field is `VALUE7`"]
    #[inline(always)]
    pub fn is_value7(&self) -> bool {
        *self == NR_LEDCOL_A::VALUE7
    }
    #[doc = "Checks if the value of the field is `VALUE8`"]
    #[inline(always)]
    pub fn is_value8(&self) -> bool {
        *self == NR_LEDCOL_A::VALUE8
    }
}
#[doc = "Field `NR_LEDCOL` writer - Number of LED Columns"]
pub type NR_LEDCOL_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, FNCTL_SPEC, u8, NR_LEDCOL_A, 3, O>;
impl<'a, const O: u8> NR_LEDCOL_W<'a, O> {
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
}
impl R {
    #[doc = "Bits 0:2 - Touch-Sense TSIN Pad Turn"]
    #[inline(always)]
    pub fn padt(&self) -> PADT_R {
        PADT_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - Software Control for Touch-Sense Pad Turn"]
    #[inline(always)]
    pub fn padtsw(&self) -> PADTSW_R {
        PADTSW_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable External Pull-up Configuration on Pin COLA"]
    #[inline(always)]
    pub fn epull(&self) -> EPULL_R {
        EPULL_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:7 - Previous Active Function/LED Column Status"]
    #[inline(always)]
    pub fn fncol(&self) -> FNCOL_R {
        FNCOL_R::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bits 16:19 - Accumulate Count on Touch-Sense Input"]
    #[inline(always)]
    pub fn acccnt(&self) -> ACCCNT_R {
        ACCCNT_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 20 - Common Compare Enable for Touch-Sense"]
    #[inline(always)]
    pub fn tsccmp(&self) -> TSCCMP_R {
        TSCCMP_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bits 21:22 - Extension for Touch-Sense Output for Pin-Low-Level"]
    #[inline(always)]
    pub fn tsoext(&self) -> TSOEXT_R {
        TSOEXT_R::new(((self.bits >> 21) & 3) as u8)
    }
    #[doc = "Bit 23 - TS-Counter Auto Reset"]
    #[inline(always)]
    pub fn tsctrr(&self) -> TSCTRR_R {
        TSCTRR_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Saturation of TS-Counter"]
    #[inline(always)]
    pub fn tsctrsat(&self) -> TSCTRSAT_R {
        TSCTRSAT_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 25:27 - Number of Touch-Sense Input"]
    #[inline(always)]
    pub fn nr_tsin(&self) -> NR_TSIN_R {
        NR_TSIN_R::new(((self.bits >> 25) & 7) as u8)
    }
    #[doc = "Bit 28 - Active Level of LED Column"]
    #[inline(always)]
    pub fn collev(&self) -> COLLEV_R {
        COLLEV_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bits 29:31 - Number of LED Columns"]
    #[inline(always)]
    pub fn nr_ledcol(&self) -> NR_LEDCOL_R {
        NR_LEDCOL_R::new(((self.bits >> 29) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Touch-Sense TSIN Pad Turn"]
    #[inline(always)]
    #[must_use]
    pub fn padt(&mut self) -> PADT_W<0> {
        PADT_W::new(self)
    }
    #[doc = "Bit 3 - Software Control for Touch-Sense Pad Turn"]
    #[inline(always)]
    #[must_use]
    pub fn padtsw(&mut self) -> PADTSW_W<3> {
        PADTSW_W::new(self)
    }
    #[doc = "Bit 4 - Enable External Pull-up Configuration on Pin COLA"]
    #[inline(always)]
    #[must_use]
    pub fn epull(&mut self) -> EPULL_W<4> {
        EPULL_W::new(self)
    }
    #[doc = "Bits 16:19 - Accumulate Count on Touch-Sense Input"]
    #[inline(always)]
    #[must_use]
    pub fn acccnt(&mut self) -> ACCCNT_W<16> {
        ACCCNT_W::new(self)
    }
    #[doc = "Bit 20 - Common Compare Enable for Touch-Sense"]
    #[inline(always)]
    #[must_use]
    pub fn tsccmp(&mut self) -> TSCCMP_W<20> {
        TSCCMP_W::new(self)
    }
    #[doc = "Bits 21:22 - Extension for Touch-Sense Output for Pin-Low-Level"]
    #[inline(always)]
    #[must_use]
    pub fn tsoext(&mut self) -> TSOEXT_W<21> {
        TSOEXT_W::new(self)
    }
    #[doc = "Bit 23 - TS-Counter Auto Reset"]
    #[inline(always)]
    #[must_use]
    pub fn tsctrr(&mut self) -> TSCTRR_W<23> {
        TSCTRR_W::new(self)
    }
    #[doc = "Bit 24 - Saturation of TS-Counter"]
    #[inline(always)]
    #[must_use]
    pub fn tsctrsat(&mut self) -> TSCTRSAT_W<24> {
        TSCTRSAT_W::new(self)
    }
    #[doc = "Bits 25:27 - Number of Touch-Sense Input"]
    #[inline(always)]
    #[must_use]
    pub fn nr_tsin(&mut self) -> NR_TSIN_W<25> {
        NR_TSIN_W::new(self)
    }
    #[doc = "Bit 28 - Active Level of LED Column"]
    #[inline(always)]
    #[must_use]
    pub fn collev(&mut self) -> COLLEV_W<28> {
        COLLEV_W::new(self)
    }
    #[doc = "Bits 29:31 - Number of LED Columns"]
    #[inline(always)]
    #[must_use]
    pub fn nr_ledcol(&mut self) -> NR_LEDCOL_W<29> {
        NR_LEDCOL_W::new(self)
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
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FNCTL to value 0"]
impl crate::Resettable for FNCTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
