#[doc = "Register `CHCTR[%s]` reader"]
pub struct R(crate::R<CHCTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CHCTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CHCTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CHCTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CHCTR[%s]` writer"]
pub struct W(crate::W<CHCTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CHCTR_SPEC>;
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
impl From<crate::W<CHCTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CHCTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ICLSEL` reader - Input Class Select"]
pub type ICLSEL_R = crate::FieldReader<u8, ICLSEL_A>;
#[doc = "Input Class Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ICLSEL_A {
    #[doc = "0: Use group-specific class 0"]
    VALUE1 = 0,
    #[doc = "1: Use group-specific class 1"]
    VALUE2 = 1,
    #[doc = "2: Use global class 0"]
    VALUE3 = 2,
    #[doc = "3: Use global class 1"]
    VALUE4 = 3,
}
impl From<ICLSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: ICLSEL_A) -> Self {
        variant as _
    }
}
impl ICLSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ICLSEL_A {
        match self.bits {
            0 => ICLSEL_A::VALUE1,
            1 => ICLSEL_A::VALUE2,
            2 => ICLSEL_A::VALUE3,
            3 => ICLSEL_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == ICLSEL_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == ICLSEL_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == ICLSEL_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == ICLSEL_A::VALUE4
    }
}
#[doc = "Field `ICLSEL` writer - Input Class Select"]
pub type ICLSEL_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, CHCTR_SPEC, u8, ICLSEL_A, 2, O>;
impl<'a, const O: u8> ICLSEL_W<'a, O> {
    #[doc = "Use group-specific class 0"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(ICLSEL_A::VALUE1)
    }
    #[doc = "Use group-specific class 1"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(ICLSEL_A::VALUE2)
    }
    #[doc = "Use global class 0"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(ICLSEL_A::VALUE3)
    }
    #[doc = "Use global class 1"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(ICLSEL_A::VALUE4)
    }
}
#[doc = "Field `BNDSELL` reader - Lower Boundary Select"]
pub type BNDSELL_R = crate::FieldReader<u8, BNDSELL_A>;
#[doc = "Lower Boundary Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum BNDSELL_A {
    #[doc = "0: Use group-specific boundary 0"]
    VALUE1 = 0,
    #[doc = "1: Use group-specific boundary 1"]
    VALUE2 = 1,
    #[doc = "2: Use global boundary 0"]
    VALUE3 = 2,
    #[doc = "3: Use global boundary 1"]
    VALUE4 = 3,
}
impl From<BNDSELL_A> for u8 {
    #[inline(always)]
    fn from(variant: BNDSELL_A) -> Self {
        variant as _
    }
}
impl BNDSELL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BNDSELL_A {
        match self.bits {
            0 => BNDSELL_A::VALUE1,
            1 => BNDSELL_A::VALUE2,
            2 => BNDSELL_A::VALUE3,
            3 => BNDSELL_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == BNDSELL_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == BNDSELL_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == BNDSELL_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == BNDSELL_A::VALUE4
    }
}
#[doc = "Field `BNDSELL` writer - Lower Boundary Select"]
pub type BNDSELL_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, CHCTR_SPEC, u8, BNDSELL_A, 2, O>;
impl<'a, const O: u8> BNDSELL_W<'a, O> {
    #[doc = "Use group-specific boundary 0"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(BNDSELL_A::VALUE1)
    }
    #[doc = "Use group-specific boundary 1"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(BNDSELL_A::VALUE2)
    }
    #[doc = "Use global boundary 0"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(BNDSELL_A::VALUE3)
    }
    #[doc = "Use global boundary 1"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(BNDSELL_A::VALUE4)
    }
}
#[doc = "Field `BNDSELU` reader - Upper Boundary Select"]
pub type BNDSELU_R = crate::FieldReader<u8, BNDSELU_A>;
#[doc = "Upper Boundary Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum BNDSELU_A {
    #[doc = "0: Use group-specific boundary 0"]
    VALUE1 = 0,
    #[doc = "1: Use group-specific boundary 1"]
    VALUE2 = 1,
    #[doc = "2: Use global boundary 0"]
    VALUE3 = 2,
    #[doc = "3: Use global boundary 1"]
    VALUE4 = 3,
}
impl From<BNDSELU_A> for u8 {
    #[inline(always)]
    fn from(variant: BNDSELU_A) -> Self {
        variant as _
    }
}
impl BNDSELU_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BNDSELU_A {
        match self.bits {
            0 => BNDSELU_A::VALUE1,
            1 => BNDSELU_A::VALUE2,
            2 => BNDSELU_A::VALUE3,
            3 => BNDSELU_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == BNDSELU_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == BNDSELU_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == BNDSELU_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == BNDSELU_A::VALUE4
    }
}
#[doc = "Field `BNDSELU` writer - Upper Boundary Select"]
pub type BNDSELU_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, CHCTR_SPEC, u8, BNDSELU_A, 2, O>;
impl<'a, const O: u8> BNDSELU_W<'a, O> {
    #[doc = "Use group-specific boundary 0"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(BNDSELU_A::VALUE1)
    }
    #[doc = "Use group-specific boundary 1"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(BNDSELU_A::VALUE2)
    }
    #[doc = "Use global boundary 0"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(BNDSELU_A::VALUE3)
    }
    #[doc = "Use global boundary 1"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(BNDSELU_A::VALUE4)
    }
}
#[doc = "Field `CHEVMODE` reader - Channel Event Mode"]
pub type CHEVMODE_R = crate::FieldReader<u8, CHEVMODE_A>;
#[doc = "Channel Event Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CHEVMODE_A {
    #[doc = "0: Never"]
    VALUE1 = 0,
    #[doc = "1: NCM: If result is inside the boundary band FCM: If result becomes high (above cmp. val.)"]
    VALUE2 = 1,
    #[doc = "2: NCM: If result is outside the boundary band FCM: If result becomes low (below cmp. val.)"]
    VALUE3 = 2,
    #[doc = "3: NCM: Always (ignore band) FCM: If result switches to either level"]
    VALUE4 = 3,
}
impl From<CHEVMODE_A> for u8 {
    #[inline(always)]
    fn from(variant: CHEVMODE_A) -> Self {
        variant as _
    }
}
impl CHEVMODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHEVMODE_A {
        match self.bits {
            0 => CHEVMODE_A::VALUE1,
            1 => CHEVMODE_A::VALUE2,
            2 => CHEVMODE_A::VALUE3,
            3 => CHEVMODE_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CHEVMODE_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CHEVMODE_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == CHEVMODE_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == CHEVMODE_A::VALUE4
    }
}
#[doc = "Field `CHEVMODE` writer - Channel Event Mode"]
pub type CHEVMODE_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, CHCTR_SPEC, u8, CHEVMODE_A, 2, O>;
impl<'a, const O: u8> CHEVMODE_W<'a, O> {
    #[doc = "Never"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CHEVMODE_A::VALUE1)
    }
    #[doc = "NCM: If result is inside the boundary band FCM: If result becomes high (above cmp. val.)"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CHEVMODE_A::VALUE2)
    }
    #[doc = "NCM: If result is outside the boundary band FCM: If result becomes low (below cmp. val.)"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(CHEVMODE_A::VALUE3)
    }
    #[doc = "NCM: Always (ignore band) FCM: If result switches to either level"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(CHEVMODE_A::VALUE4)
    }
}
#[doc = "Field `SYNC` reader - Synchronization Request"]
pub type SYNC_R = crate::BitReader<SYNC_A>;
#[doc = "Synchronization Request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SYNC_A {
    #[doc = "0: No synchroniz. request, standalone operation"]
    VALUE1 = 0,
    #[doc = "1: Request a synchronized conversion of this channel (only taken into account for a master)"]
    VALUE2 = 1,
}
impl From<SYNC_A> for bool {
    #[inline(always)]
    fn from(variant: SYNC_A) -> Self {
        variant as u8 != 0
    }
}
impl SYNC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SYNC_A {
        match self.bits {
            false => SYNC_A::VALUE1,
            true => SYNC_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SYNC_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SYNC_A::VALUE2
    }
}
#[doc = "Field `SYNC` writer - Synchronization Request"]
pub type SYNC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHCTR_SPEC, SYNC_A, O>;
impl<'a, const O: u8> SYNC_W<'a, O> {
    #[doc = "No synchroniz. request, standalone operation"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(SYNC_A::VALUE1)
    }
    #[doc = "Request a synchronized conversion of this channel (only taken into account for a master)"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(SYNC_A::VALUE2)
    }
}
#[doc = "Field `REFSEL` reader - Reference Input Selection"]
pub type REFSEL_R = crate::BitReader<REFSEL_A>;
#[doc = "Reference Input Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REFSEL_A {
    #[doc = "0: Standard reference input VAREF"]
    VALUE1 = 0,
    #[doc = "1: Alternate reference input from CH0"]
    VALUE2 = 1,
}
impl From<REFSEL_A> for bool {
    #[inline(always)]
    fn from(variant: REFSEL_A) -> Self {
        variant as u8 != 0
    }
}
impl REFSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REFSEL_A {
        match self.bits {
            false => REFSEL_A::VALUE1,
            true => REFSEL_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == REFSEL_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == REFSEL_A::VALUE2
    }
}
#[doc = "Field `REFSEL` writer - Reference Input Selection"]
pub type REFSEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHCTR_SPEC, REFSEL_A, O>;
impl<'a, const O: u8> REFSEL_W<'a, O> {
    #[doc = "Standard reference input VAREF"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(REFSEL_A::VALUE1)
    }
    #[doc = "Alternate reference input from CH0"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(REFSEL_A::VALUE2)
    }
}
#[doc = "Field `RESREG` reader - Result Register"]
pub type RESREG_R = crate::FieldReader<u8, RESREG_A>;
#[doc = "Result Register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RESREG_A {
    #[doc = "0: Store result in group result register GxRES0"]
    VALUE1 = 0,
    #[doc = "15: Store result in group result register GxRES15"]
    VALUE2 = 15,
}
impl From<RESREG_A> for u8 {
    #[inline(always)]
    fn from(variant: RESREG_A) -> Self {
        variant as _
    }
}
impl RESREG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<RESREG_A> {
        match self.bits {
            0 => Some(RESREG_A::VALUE1),
            15 => Some(RESREG_A::VALUE2),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == RESREG_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == RESREG_A::VALUE2
    }
}
#[doc = "Field `RESREG` writer - Result Register"]
pub type RESREG_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CHCTR_SPEC, u8, RESREG_A, 4, O>;
impl<'a, const O: u8> RESREG_W<'a, O> {
    #[doc = "Store result in group result register GxRES0"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(RESREG_A::VALUE1)
    }
    #[doc = "Store result in group result register GxRES15"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(RESREG_A::VALUE2)
    }
}
#[doc = "Field `RESTBS` reader - Result Target for Background Source"]
pub type RESTBS_R = crate::BitReader<RESTBS_A>;
#[doc = "Result Target for Background Source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RESTBS_A {
    #[doc = "0: Store results in the selected group result register"]
    VALUE1 = 0,
    #[doc = "1: Store results in the global result register"]
    VALUE2 = 1,
}
impl From<RESTBS_A> for bool {
    #[inline(always)]
    fn from(variant: RESTBS_A) -> Self {
        variant as u8 != 0
    }
}
impl RESTBS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RESTBS_A {
        match self.bits {
            false => RESTBS_A::VALUE1,
            true => RESTBS_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == RESTBS_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == RESTBS_A::VALUE2
    }
}
#[doc = "Field `RESTBS` writer - Result Target for Background Source"]
pub type RESTBS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHCTR_SPEC, RESTBS_A, O>;
impl<'a, const O: u8> RESTBS_W<'a, O> {
    #[doc = "Store results in the selected group result register"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(RESTBS_A::VALUE1)
    }
    #[doc = "Store results in the global result register"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(RESTBS_A::VALUE2)
    }
}
#[doc = "Field `RESPOS` reader - Result Position"]
pub type RESPOS_R = crate::BitReader<RESPOS_A>;
#[doc = "Result Position\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RESPOS_A {
    #[doc = "0: Store results left-aligned"]
    VALUE1 = 0,
    #[doc = "1: Store results right-aligned"]
    VALUE2 = 1,
}
impl From<RESPOS_A> for bool {
    #[inline(always)]
    fn from(variant: RESPOS_A) -> Self {
        variant as u8 != 0
    }
}
impl RESPOS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RESPOS_A {
        match self.bits {
            false => RESPOS_A::VALUE1,
            true => RESPOS_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == RESPOS_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == RESPOS_A::VALUE2
    }
}
#[doc = "Field `RESPOS` writer - Result Position"]
pub type RESPOS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHCTR_SPEC, RESPOS_A, O>;
impl<'a, const O: u8> RESPOS_W<'a, O> {
    #[doc = "Store results left-aligned"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(RESPOS_A::VALUE1)
    }
    #[doc = "Store results right-aligned"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(RESPOS_A::VALUE2)
    }
}
#[doc = "Field `BWDCH` reader - Broken Wire Detection Channel"]
pub type BWDCH_R = crate::FieldReader<u8, BWDCH_A>;
#[doc = "Broken Wire Detection Channel\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum BWDCH_A {
    #[doc = "0: Select VAGND"]
    VALUE1 = 0,
    #[doc = "1: Select VAREF"]
    VALUE2 = 1,
}
impl From<BWDCH_A> for u8 {
    #[inline(always)]
    fn from(variant: BWDCH_A) -> Self {
        variant as _
    }
}
impl BWDCH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<BWDCH_A> {
        match self.bits {
            0 => Some(BWDCH_A::VALUE1),
            1 => Some(BWDCH_A::VALUE2),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == BWDCH_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == BWDCH_A::VALUE2
    }
}
#[doc = "Field `BWDCH` writer - Broken Wire Detection Channel"]
pub type BWDCH_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CHCTR_SPEC, u8, BWDCH_A, 2, O>;
impl<'a, const O: u8> BWDCH_W<'a, O> {
    #[doc = "Select VAGND"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(BWDCH_A::VALUE1)
    }
    #[doc = "Select VAREF"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(BWDCH_A::VALUE2)
    }
}
#[doc = "Field `BWDEN` reader - Broken Wire Detection Enable"]
pub type BWDEN_R = crate::BitReader<BWDEN_A>;
#[doc = "Broken Wire Detection Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BWDEN_A {
    #[doc = "0: Normal operation"]
    VALUE1 = 0,
    #[doc = "1: Additional preparation phase is enabled"]
    VALUE2 = 1,
}
impl From<BWDEN_A> for bool {
    #[inline(always)]
    fn from(variant: BWDEN_A) -> Self {
        variant as u8 != 0
    }
}
impl BWDEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BWDEN_A {
        match self.bits {
            false => BWDEN_A::VALUE1,
            true => BWDEN_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == BWDEN_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == BWDEN_A::VALUE2
    }
}
#[doc = "Field `BWDEN` writer - Broken Wire Detection Enable"]
pub type BWDEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHCTR_SPEC, BWDEN_A, O>;
impl<'a, const O: u8> BWDEN_W<'a, O> {
    #[doc = "Normal operation"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(BWDEN_A::VALUE1)
    }
    #[doc = "Additional preparation phase is enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(BWDEN_A::VALUE2)
    }
}
impl R {
    #[doc = "Bits 0:1 - Input Class Select"]
    #[inline(always)]
    pub fn iclsel(&self) -> ICLSEL_R {
        ICLSEL_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 4:5 - Lower Boundary Select"]
    #[inline(always)]
    pub fn bndsell(&self) -> BNDSELL_R {
        BNDSELL_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Upper Boundary Select"]
    #[inline(always)]
    pub fn bndselu(&self) -> BNDSELU_R {
        BNDSELU_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Channel Event Mode"]
    #[inline(always)]
    pub fn chevmode(&self) -> CHEVMODE_R {
        CHEVMODE_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 10 - Synchronization Request"]
    #[inline(always)]
    pub fn sync(&self) -> SYNC_R {
        SYNC_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Reference Input Selection"]
    #[inline(always)]
    pub fn refsel(&self) -> REFSEL_R {
        REFSEL_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 16:19 - Result Register"]
    #[inline(always)]
    pub fn resreg(&self) -> RESREG_R {
        RESREG_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 20 - Result Target for Background Source"]
    #[inline(always)]
    pub fn restbs(&self) -> RESTBS_R {
        RESTBS_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Result Position"]
    #[inline(always)]
    pub fn respos(&self) -> RESPOS_R {
        RESPOS_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bits 28:29 - Broken Wire Detection Channel"]
    #[inline(always)]
    pub fn bwdch(&self) -> BWDCH_R {
        BWDCH_R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bit 30 - Broken Wire Detection Enable"]
    #[inline(always)]
    pub fn bwden(&self) -> BWDEN_R {
        BWDEN_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Input Class Select"]
    #[inline(always)]
    #[must_use]
    pub fn iclsel(&mut self) -> ICLSEL_W<0> {
        ICLSEL_W::new(self)
    }
    #[doc = "Bits 4:5 - Lower Boundary Select"]
    #[inline(always)]
    #[must_use]
    pub fn bndsell(&mut self) -> BNDSELL_W<4> {
        BNDSELL_W::new(self)
    }
    #[doc = "Bits 6:7 - Upper Boundary Select"]
    #[inline(always)]
    #[must_use]
    pub fn bndselu(&mut self) -> BNDSELU_W<6> {
        BNDSELU_W::new(self)
    }
    #[doc = "Bits 8:9 - Channel Event Mode"]
    #[inline(always)]
    #[must_use]
    pub fn chevmode(&mut self) -> CHEVMODE_W<8> {
        CHEVMODE_W::new(self)
    }
    #[doc = "Bit 10 - Synchronization Request"]
    #[inline(always)]
    #[must_use]
    pub fn sync(&mut self) -> SYNC_W<10> {
        SYNC_W::new(self)
    }
    #[doc = "Bit 11 - Reference Input Selection"]
    #[inline(always)]
    #[must_use]
    pub fn refsel(&mut self) -> REFSEL_W<11> {
        REFSEL_W::new(self)
    }
    #[doc = "Bits 16:19 - Result Register"]
    #[inline(always)]
    #[must_use]
    pub fn resreg(&mut self) -> RESREG_W<16> {
        RESREG_W::new(self)
    }
    #[doc = "Bit 20 - Result Target for Background Source"]
    #[inline(always)]
    #[must_use]
    pub fn restbs(&mut self) -> RESTBS_W<20> {
        RESTBS_W::new(self)
    }
    #[doc = "Bit 21 - Result Position"]
    #[inline(always)]
    #[must_use]
    pub fn respos(&mut self) -> RESPOS_W<21> {
        RESPOS_W::new(self)
    }
    #[doc = "Bits 28:29 - Broken Wire Detection Channel"]
    #[inline(always)]
    #[must_use]
    pub fn bwdch(&mut self) -> BWDCH_W<28> {
        BWDCH_W::new(self)
    }
    #[doc = "Bit 30 - Broken Wire Detection Enable"]
    #[inline(always)]
    #[must_use]
    pub fn bwden(&mut self) -> BWDEN_W<30> {
        BWDEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Channel Ctrl. Reg.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chctr](index.html) module"]
pub struct CHCTR_SPEC;
impl crate::RegisterSpec for CHCTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [chctr::R](R) reader structure"]
impl crate::Readable for CHCTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [chctr::W](W) writer structure"]
impl crate::Writable for CHCTR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CHCTR[%s]
to value 0"]
impl crate::Resettable for CHCTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
