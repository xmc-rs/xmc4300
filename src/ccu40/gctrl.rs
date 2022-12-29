#[doc = "Register `GCTRL` reader"]
pub struct R(crate::R<GCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GCTRL` writer"]
pub struct W(crate::W<GCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GCTRL_SPEC>;
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
impl From<crate::W<GCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PRBC` reader - Prescaler Clear Configuration"]
pub type PRBC_R = crate::FieldReader<u8, PRBC_A>;
#[doc = "Prescaler Clear Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PRBC_A {
    #[doc = "0: SW only"]
    VALUE1 = 0,
    #[doc = "1: GSTATThe register contains the status of the prescaler and each timer slice (idle mode or running)..PRB and prescaler registers are cleared when the Run Bit of CC40 is cleared."]
    VALUE2 = 1,
    #[doc = "2: GSTATThe register contains the status of the prescaler and each timer slice (idle mode or running)..PRB and prescaler registers are cleared when the Run Bit of CC41 is cleared."]
    VALUE3 = 2,
    #[doc = "3: GSTATThe register contains the status of the prescaler and each timer slice (idle mode or running)..PRB and prescaler registers are cleared when the Run Bit of CC42 is cleared."]
    VALUE4 = 3,
    #[doc = "4: GSTATThe register contains the status of the prescaler and each timer slice (idle mode or running)..PRB and prescaler registers are cleared when the Run Bit of CC43 is cleared."]
    VALUE5 = 4,
}
impl From<PRBC_A> for u8 {
    #[inline(always)]
    fn from(variant: PRBC_A) -> Self {
        variant as _
    }
}
impl PRBC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PRBC_A> {
        match self.bits {
            0 => Some(PRBC_A::VALUE1),
            1 => Some(PRBC_A::VALUE2),
            2 => Some(PRBC_A::VALUE3),
            3 => Some(PRBC_A::VALUE4),
            4 => Some(PRBC_A::VALUE5),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PRBC_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PRBC_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == PRBC_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == PRBC_A::VALUE4
    }
    #[doc = "Checks if the value of the field is `VALUE5`"]
    #[inline(always)]
    pub fn is_value5(&self) -> bool {
        *self == PRBC_A::VALUE5
    }
}
#[doc = "Field `PRBC` writer - Prescaler Clear Configuration"]
pub type PRBC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GCTRL_SPEC, u8, PRBC_A, 3, O>;
impl<'a, const O: u8> PRBC_W<'a, O> {
    #[doc = "SW only"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(PRBC_A::VALUE1)
    }
    #[doc = "GSTATThe register contains the status of the prescaler and each timer slice (idle mode or running)..PRB and prescaler registers are cleared when the Run Bit of CC40 is cleared."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(PRBC_A::VALUE2)
    }
    #[doc = "GSTATThe register contains the status of the prescaler and each timer slice (idle mode or running)..PRB and prescaler registers are cleared when the Run Bit of CC41 is cleared."]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(PRBC_A::VALUE3)
    }
    #[doc = "GSTATThe register contains the status of the prescaler and each timer slice (idle mode or running)..PRB and prescaler registers are cleared when the Run Bit of CC42 is cleared."]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(PRBC_A::VALUE4)
    }
    #[doc = "GSTATThe register contains the status of the prescaler and each timer slice (idle mode or running)..PRB and prescaler registers are cleared when the Run Bit of CC43 is cleared."]
    #[inline(always)]
    pub fn value5(self) -> &'a mut W {
        self.variant(PRBC_A::VALUE5)
    }
}
#[doc = "Field `PCIS` reader - Prescaler Input Clock Selection"]
pub type PCIS_R = crate::FieldReader<u8, PCIS_A>;
#[doc = "Prescaler Input Clock Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PCIS_A {
    #[doc = "0: Module clock"]
    VALUE1 = 0,
    #[doc = "1: CCU4x.ECLKA"]
    VALUE2 = 1,
    #[doc = "2: CCU4x.ECLKB"]
    VALUE3 = 2,
    #[doc = "3: CCU4x.ECLKC"]
    VALUE4 = 3,
}
impl From<PCIS_A> for u8 {
    #[inline(always)]
    fn from(variant: PCIS_A) -> Self {
        variant as _
    }
}
impl PCIS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PCIS_A {
        match self.bits {
            0 => PCIS_A::VALUE1,
            1 => PCIS_A::VALUE2,
            2 => PCIS_A::VALUE3,
            3 => PCIS_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PCIS_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PCIS_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == PCIS_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == PCIS_A::VALUE4
    }
}
#[doc = "Field `PCIS` writer - Prescaler Input Clock Selection"]
pub type PCIS_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, GCTRL_SPEC, u8, PCIS_A, 2, O>;
impl<'a, const O: u8> PCIS_W<'a, O> {
    #[doc = "Module clock"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(PCIS_A::VALUE1)
    }
    #[doc = "CCU4x.ECLKA"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(PCIS_A::VALUE2)
    }
    #[doc = "CCU4x.ECLKB"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(PCIS_A::VALUE3)
    }
    #[doc = "CCU4x.ECLKC"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(PCIS_A::VALUE4)
    }
}
#[doc = "Field `SUSCFG` reader - Suspend Mode Configuration"]
pub type SUSCFG_R = crate::FieldReader<u8, SUSCFG_A>;
#[doc = "Suspend Mode Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SUSCFG_A {
    #[doc = "0: Suspend request ignored. The module never enters in suspend"]
    VALUE1 = 0,
    #[doc = "1: Stops all the running slices immediately. Safe stop is not applied."]
    VALUE2 = 1,
    #[doc = "2: Stops the block immediately and clamps all the outputs to PASSIVE state. Safe stop is applied."]
    VALUE3 = 2,
    #[doc = "3: Waits for the roll over of each slice to stop and clamp the slices outputs. Safe stop is applied."]
    VALUE4 = 3,
}
impl From<SUSCFG_A> for u8 {
    #[inline(always)]
    fn from(variant: SUSCFG_A) -> Self {
        variant as _
    }
}
impl SUSCFG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SUSCFG_A {
        match self.bits {
            0 => SUSCFG_A::VALUE1,
            1 => SUSCFG_A::VALUE2,
            2 => SUSCFG_A::VALUE3,
            3 => SUSCFG_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SUSCFG_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SUSCFG_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == SUSCFG_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == SUSCFG_A::VALUE4
    }
}
#[doc = "Field `SUSCFG` writer - Suspend Mode Configuration"]
pub type SUSCFG_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, GCTRL_SPEC, u8, SUSCFG_A, 2, O>;
impl<'a, const O: u8> SUSCFG_W<'a, O> {
    #[doc = "Suspend request ignored. The module never enters in suspend"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(SUSCFG_A::VALUE1)
    }
    #[doc = "Stops all the running slices immediately. Safe stop is not applied."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(SUSCFG_A::VALUE2)
    }
    #[doc = "Stops the block immediately and clamps all the outputs to PASSIVE state. Safe stop is applied."]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(SUSCFG_A::VALUE3)
    }
    #[doc = "Waits for the roll over of each slice to stop and clamp the slices outputs. Safe stop is applied."]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(SUSCFG_A::VALUE4)
    }
}
#[doc = "Field `MSE0` reader - Slice 0 Multi Channel shadow transfer enable"]
pub type MSE0_R = crate::BitReader<MSE0_A>;
#[doc = "Slice 0 Multi Channel shadow transfer enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSE0_A {
    #[doc = "0: Shadow transfer can only be requested by SW"]
    VALUE1 = 0,
    #[doc = "1: Shadow transfer can be requested via SW and via the CCU4x.MCSS input."]
    VALUE2 = 1,
}
impl From<MSE0_A> for bool {
    #[inline(always)]
    fn from(variant: MSE0_A) -> Self {
        variant as u8 != 0
    }
}
impl MSE0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MSE0_A {
        match self.bits {
            false => MSE0_A::VALUE1,
            true => MSE0_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == MSE0_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == MSE0_A::VALUE2
    }
}
#[doc = "Field `MSE0` writer - Slice 0 Multi Channel shadow transfer enable"]
pub type MSE0_W<'a, const O: u8> = crate::BitWriter<'a, u32, GCTRL_SPEC, MSE0_A, O>;
impl<'a, const O: u8> MSE0_W<'a, O> {
    #[doc = "Shadow transfer can only be requested by SW"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(MSE0_A::VALUE1)
    }
    #[doc = "Shadow transfer can be requested via SW and via the CCU4x.MCSS input."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(MSE0_A::VALUE2)
    }
}
#[doc = "Field `MSE1` reader - Slice 1 Multi Channel shadow transfer enable"]
pub type MSE1_R = crate::BitReader<MSE1_A>;
#[doc = "Slice 1 Multi Channel shadow transfer enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSE1_A {
    #[doc = "0: Shadow transfer can only be requested by SW"]
    VALUE1 = 0,
    #[doc = "1: Shadow transfer can be requested via SW and via the CCU4x.MCSS input."]
    VALUE2 = 1,
}
impl From<MSE1_A> for bool {
    #[inline(always)]
    fn from(variant: MSE1_A) -> Self {
        variant as u8 != 0
    }
}
impl MSE1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MSE1_A {
        match self.bits {
            false => MSE1_A::VALUE1,
            true => MSE1_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == MSE1_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == MSE1_A::VALUE2
    }
}
#[doc = "Field `MSE1` writer - Slice 1 Multi Channel shadow transfer enable"]
pub type MSE1_W<'a, const O: u8> = crate::BitWriter<'a, u32, GCTRL_SPEC, MSE1_A, O>;
impl<'a, const O: u8> MSE1_W<'a, O> {
    #[doc = "Shadow transfer can only be requested by SW"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(MSE1_A::VALUE1)
    }
    #[doc = "Shadow transfer can be requested via SW and via the CCU4x.MCSS input."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(MSE1_A::VALUE2)
    }
}
#[doc = "Field `MSE2` reader - Slice 2 Multi Channel shadow transfer enable"]
pub type MSE2_R = crate::BitReader<MSE2_A>;
#[doc = "Slice 2 Multi Channel shadow transfer enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSE2_A {
    #[doc = "0: Shadow transfer can only be requested by SW"]
    VALUE1 = 0,
    #[doc = "1: Shadow transfer can be requested via SW and via the CCU4x.MCSS input."]
    VALUE2 = 1,
}
impl From<MSE2_A> for bool {
    #[inline(always)]
    fn from(variant: MSE2_A) -> Self {
        variant as u8 != 0
    }
}
impl MSE2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MSE2_A {
        match self.bits {
            false => MSE2_A::VALUE1,
            true => MSE2_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == MSE2_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == MSE2_A::VALUE2
    }
}
#[doc = "Field `MSE2` writer - Slice 2 Multi Channel shadow transfer enable"]
pub type MSE2_W<'a, const O: u8> = crate::BitWriter<'a, u32, GCTRL_SPEC, MSE2_A, O>;
impl<'a, const O: u8> MSE2_W<'a, O> {
    #[doc = "Shadow transfer can only be requested by SW"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(MSE2_A::VALUE1)
    }
    #[doc = "Shadow transfer can be requested via SW and via the CCU4x.MCSS input."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(MSE2_A::VALUE2)
    }
}
#[doc = "Field `MSE3` reader - Slice 3 Multi Channel shadow transfer enable"]
pub type MSE3_R = crate::BitReader<MSE3_A>;
#[doc = "Slice 3 Multi Channel shadow transfer enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSE3_A {
    #[doc = "0: Shadow transfer can only be requested by SW"]
    VALUE1 = 0,
    #[doc = "1: Shadow transfer can be requested via SW and via the CCU4x.MCSS input."]
    VALUE2 = 1,
}
impl From<MSE3_A> for bool {
    #[inline(always)]
    fn from(variant: MSE3_A) -> Self {
        variant as u8 != 0
    }
}
impl MSE3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MSE3_A {
        match self.bits {
            false => MSE3_A::VALUE1,
            true => MSE3_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == MSE3_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == MSE3_A::VALUE2
    }
}
#[doc = "Field `MSE3` writer - Slice 3 Multi Channel shadow transfer enable"]
pub type MSE3_W<'a, const O: u8> = crate::BitWriter<'a, u32, GCTRL_SPEC, MSE3_A, O>;
impl<'a, const O: u8> MSE3_W<'a, O> {
    #[doc = "Shadow transfer can only be requested by SW"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(MSE3_A::VALUE1)
    }
    #[doc = "Shadow transfer can be requested via SW and via the CCU4x.MCSS input."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(MSE3_A::VALUE2)
    }
}
#[doc = "Field `MSDE` reader - Multi Channel shadow transfer request configuration"]
pub type MSDE_R = crate::FieldReader<u8, MSDE_A>;
#[doc = "Multi Channel shadow transfer request configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MSDE_A {
    #[doc = "0: Only the shadow transfer for period and compare values is requested"]
    VALUE1 = 0,
    #[doc = "1: Shadow transfer for the compare, period and prescaler compare values is requested"]
    VALUE2 = 1,
    #[doc = "3: Shadow transfer for the compare, period, prescaler and dither compare values is requested"]
    VALUE4 = 3,
}
impl From<MSDE_A> for u8 {
    #[inline(always)]
    fn from(variant: MSDE_A) -> Self {
        variant as _
    }
}
impl MSDE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<MSDE_A> {
        match self.bits {
            0 => Some(MSDE_A::VALUE1),
            1 => Some(MSDE_A::VALUE2),
            3 => Some(MSDE_A::VALUE4),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == MSDE_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == MSDE_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == MSDE_A::VALUE4
    }
}
#[doc = "Field `MSDE` writer - Multi Channel shadow transfer request configuration"]
pub type MSDE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GCTRL_SPEC, u8, MSDE_A, 2, O>;
impl<'a, const O: u8> MSDE_W<'a, O> {
    #[doc = "Only the shadow transfer for period and compare values is requested"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(MSDE_A::VALUE1)
    }
    #[doc = "Shadow transfer for the compare, period and prescaler compare values is requested"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(MSDE_A::VALUE2)
    }
    #[doc = "Shadow transfer for the compare, period, prescaler and dither compare values is requested"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(MSDE_A::VALUE4)
    }
}
impl R {
    #[doc = "Bits 0:2 - Prescaler Clear Configuration"]
    #[inline(always)]
    pub fn prbc(&self) -> PRBC_R {
        PRBC_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:5 - Prescaler Input Clock Selection"]
    #[inline(always)]
    pub fn pcis(&self) -> PCIS_R {
        PCIS_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Suspend Mode Configuration"]
    #[inline(always)]
    pub fn suscfg(&self) -> SUSCFG_R {
        SUSCFG_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 10 - Slice 0 Multi Channel shadow transfer enable"]
    #[inline(always)]
    pub fn mse0(&self) -> MSE0_R {
        MSE0_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Slice 1 Multi Channel shadow transfer enable"]
    #[inline(always)]
    pub fn mse1(&self) -> MSE1_R {
        MSE1_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Slice 2 Multi Channel shadow transfer enable"]
    #[inline(always)]
    pub fn mse2(&self) -> MSE2_R {
        MSE2_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Slice 3 Multi Channel shadow transfer enable"]
    #[inline(always)]
    pub fn mse3(&self) -> MSE3_R {
        MSE3_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 14:15 - Multi Channel shadow transfer request configuration"]
    #[inline(always)]
    pub fn msde(&self) -> MSDE_R {
        MSDE_R::new(((self.bits >> 14) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Prescaler Clear Configuration"]
    #[inline(always)]
    #[must_use]
    pub fn prbc(&mut self) -> PRBC_W<0> {
        PRBC_W::new(self)
    }
    #[doc = "Bits 4:5 - Prescaler Input Clock Selection"]
    #[inline(always)]
    #[must_use]
    pub fn pcis(&mut self) -> PCIS_W<4> {
        PCIS_W::new(self)
    }
    #[doc = "Bits 8:9 - Suspend Mode Configuration"]
    #[inline(always)]
    #[must_use]
    pub fn suscfg(&mut self) -> SUSCFG_W<8> {
        SUSCFG_W::new(self)
    }
    #[doc = "Bit 10 - Slice 0 Multi Channel shadow transfer enable"]
    #[inline(always)]
    #[must_use]
    pub fn mse0(&mut self) -> MSE0_W<10> {
        MSE0_W::new(self)
    }
    #[doc = "Bit 11 - Slice 1 Multi Channel shadow transfer enable"]
    #[inline(always)]
    #[must_use]
    pub fn mse1(&mut self) -> MSE1_W<11> {
        MSE1_W::new(self)
    }
    #[doc = "Bit 12 - Slice 2 Multi Channel shadow transfer enable"]
    #[inline(always)]
    #[must_use]
    pub fn mse2(&mut self) -> MSE2_W<12> {
        MSE2_W::new(self)
    }
    #[doc = "Bit 13 - Slice 3 Multi Channel shadow transfer enable"]
    #[inline(always)]
    #[must_use]
    pub fn mse3(&mut self) -> MSE3_W<13> {
        MSE3_W::new(self)
    }
    #[doc = "Bits 14:15 - Multi Channel shadow transfer request configuration"]
    #[inline(always)]
    #[must_use]
    pub fn msde(&mut self) -> MSDE_W<14> {
        MSDE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Global Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gctrl](index.html) module"]
pub struct GCTRL_SPEC;
impl crate::RegisterSpec for GCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gctrl::R](R) reader structure"]
impl crate::Readable for GCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gctrl::W](W) writer structure"]
impl crate::Writable for GCTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GCTRL to value 0"]
impl crate::Resettable for GCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
