#[doc = "Reader of register AL_CONTROL"]
pub type R = crate::R<u16, super::AL_CONTROL>;
#[doc = "Initiate State Transition of the Device StateMachine\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IST_A {
    #[doc = "1: Request Init State"]
    VALUE1,
    #[doc = "2: Request Pre-Operational State"]
    VALUE2,
    #[doc = "3: Request Bootstrap State"]
    VALUE3,
    #[doc = "4: Request Safe-Operational State"]
    VALUE4,
    #[doc = "8: Request Operational State"]
    VALUE5,
}
impl From<IST_A> for u8 {
    #[inline(always)]
    fn from(variant: IST_A) -> Self {
        match variant {
            IST_A::VALUE1 => 1,
            IST_A::VALUE2 => 2,
            IST_A::VALUE3 => 3,
            IST_A::VALUE4 => 4,
            IST_A::VALUE5 => 8,
        }
    }
}
#[doc = "Reader of field `IST`"]
pub type IST_R = crate::R<u8, IST_A>;
impl IST_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, IST_A> {
        use crate::Variant::*;
        match self.bits {
            1 => Val(IST_A::VALUE1),
            2 => Val(IST_A::VALUE2),
            3 => Val(IST_A::VALUE3),
            4 => Val(IST_A::VALUE4),
            8 => Val(IST_A::VALUE5),
            i => Res(i),
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
#[doc = "Error Ind Ack\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EIA_A {
    #[doc = "0: No Ack of Error Ind in AL status register"]
    VALUE1,
    #[doc = "1: Ack of Error Ind in AL status register"]
    VALUE2,
}
impl From<EIA_A> for bool {
    #[inline(always)]
    fn from(variant: EIA_A) -> Self {
        match variant {
            EIA_A::VALUE1 => false,
            EIA_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `EIA`"]
pub type EIA_R = crate::R<bool, EIA_A>;
impl EIA_R {
    #[doc = r"Get enumerated values variant"]
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
#[doc = "Device Identification\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DID_A {
    #[doc = "0: No request"]
    VALUE1,
    #[doc = "1: Device Identification request"]
    VALUE2,
}
impl From<DID_A> for bool {
    #[inline(always)]
    fn from(variant: DID_A) -> Self {
        match variant {
            DID_A::VALUE1 => false,
            DID_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `DID`"]
pub type DID_R = crate::R<bool, DID_A>;
impl DID_R {
    #[doc = r"Get enumerated values variant"]
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
        EIA_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Device Identification"]
    #[inline(always)]
    pub fn did(&self) -> DID_R {
        DID_R::new(((self.bits >> 5) & 0x01) != 0)
    }
}
