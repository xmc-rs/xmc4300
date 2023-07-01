#[doc = "Register `AL_CONTROL` reader"]
pub struct R(crate::R<AL_CONTROL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AL_CONTROL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AL_CONTROL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AL_CONTROL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `IST` reader - Initiate State Transition of the Device StateMachine"]
pub type IST_R = crate::FieldReader<IST_A>;
#[doc = "Initiate State Transition of the Device StateMachine\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum IST_A {
    #[doc = "1: Request Init State"]
    VALUE1 = 1,
    #[doc = "2: Request Pre-Operational State"]
    VALUE2 = 2,
    #[doc = "3: Request Bootstrap State"]
    VALUE3 = 3,
    #[doc = "4: Request Safe-Operational State"]
    VALUE4 = 4,
    #[doc = "8: Request Operational State"]
    VALUE5 = 8,
}
impl From<IST_A> for u8 {
    #[inline(always)]
    fn from(variant: IST_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for IST_A {
    type Ux = u8;
}
impl IST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<IST_A> {
        match self.bits {
            1 => Some(IST_A::VALUE1),
            2 => Some(IST_A::VALUE2),
            3 => Some(IST_A::VALUE3),
            4 => Some(IST_A::VALUE4),
            8 => Some(IST_A::VALUE5),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == IST_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == IST_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == IST_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == IST_A::VALUE4
    }
    #[doc = "Checks if the value of the field is `VALUE5`"]
    #[inline(always)]
    pub fn is_value5(&self) -> bool {
        *self == IST_A::VALUE5
    }
}
#[doc = "Field `EIA` reader - Error Ind Ack"]
pub type EIA_R = crate::BitReader<EIA_A>;
#[doc = "Error Ind Ack\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EIA_A {
    #[doc = "0: No Ack of Error Ind in AL status register"]
    VALUE1 = 0,
    #[doc = "1: Ack of Error Ind in AL status register"]
    VALUE2 = 1,
}
impl From<EIA_A> for bool {
    #[inline(always)]
    fn from(variant: EIA_A) -> Self {
        variant as u8 != 0
    }
}
impl EIA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EIA_A {
        match self.bits {
            false => EIA_A::VALUE1,
            true => EIA_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == EIA_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == EIA_A::VALUE2
    }
}
#[doc = "Field `DID` reader - Device Identification"]
pub type DID_R = crate::BitReader<DID_A>;
#[doc = "Device Identification\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DID_A {
    #[doc = "0: No request"]
    VALUE1 = 0,
    #[doc = "1: Device Identification request"]
    VALUE2 = 1,
}
impl From<DID_A> for bool {
    #[inline(always)]
    fn from(variant: DID_A) -> Self {
        variant as u8 != 0
    }
}
impl DID_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DID_A {
        match self.bits {
            false => DID_A::VALUE1,
            true => DID_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == DID_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == DID_A::VALUE2
    }
}
impl R {
    #[doc = "Bits 0:3 - Initiate State Transition of the Device StateMachine"]
    #[inline(always)]
    pub fn ist(&self) -> IST_R {
        IST_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - Error Ind Ack"]
    #[inline(always)]
    pub fn eia(&self) -> EIA_R {
        EIA_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Device Identification"]
    #[inline(always)]
    pub fn did(&self) -> DID_R {
        DID_R::new(((self.bits >> 5) & 1) != 0)
    }
}
#[doc = "AL Control\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [al_control](index.html) module"]
pub struct AL_CONTROL_SPEC;
impl crate::RegisterSpec for AL_CONTROL_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [al_control::R](R) reader structure"]
impl crate::Readable for AL_CONTROL_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets AL_CONTROL to value 0x01"]
impl crate::Resettable for AL_CONTROL_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
