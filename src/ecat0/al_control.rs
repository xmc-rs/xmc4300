#[doc = "Register `AL_CONTROL` reader"]
pub type R = crate::R<AlControlSpec>;
#[doc = "Initiate State Transition of the Device StateMachine\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ist {
    #[doc = "1: Request Init State"]
    Value1 = 1,
    #[doc = "2: Request Pre-Operational State"]
    Value2 = 2,
    #[doc = "3: Request Bootstrap State"]
    Value3 = 3,
    #[doc = "4: Request Safe-Operational State"]
    Value4 = 4,
    #[doc = "8: Request Operational State"]
    Value5 = 8,
}
impl From<Ist> for u8 {
    #[inline(always)]
    fn from(variant: Ist) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ist {
    type Ux = u8;
}
impl crate::IsEnum for Ist {}
#[doc = "Field `IST` reader - Initiate State Transition of the Device StateMachine"]
pub type IstR = crate::FieldReader<Ist>;
impl IstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Ist> {
        match self.bits {
            1 => Some(Ist::Value1),
            2 => Some(Ist::Value2),
            3 => Some(Ist::Value3),
            4 => Some(Ist::Value4),
            8 => Some(Ist::Value5),
            _ => None,
        }
    }
    #[doc = "Request Init State"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Ist::Value1
    }
    #[doc = "Request Pre-Operational State"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Ist::Value2
    }
    #[doc = "Request Bootstrap State"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == Ist::Value3
    }
    #[doc = "Request Safe-Operational State"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == Ist::Value4
    }
    #[doc = "Request Operational State"]
    #[inline(always)]
    pub fn is_value5(&self) -> bool {
        *self == Ist::Value5
    }
}
#[doc = "Error Ind Ack\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Eia {
    #[doc = "0: No Ack of Error Ind in AL status register"]
    Value1 = 0,
    #[doc = "1: Ack of Error Ind in AL status register"]
    Value2 = 1,
}
impl From<Eia> for bool {
    #[inline(always)]
    fn from(variant: Eia) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EIA` reader - Error Ind Ack"]
pub type EiaR = crate::BitReader<Eia>;
impl EiaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Eia {
        match self.bits {
            false => Eia::Value1,
            true => Eia::Value2,
        }
    }
    #[doc = "No Ack of Error Ind in AL status register"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Eia::Value1
    }
    #[doc = "Ack of Error Ind in AL status register"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Eia::Value2
    }
}
#[doc = "Device Identification\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Did {
    #[doc = "0: No request"]
    Value1 = 0,
    #[doc = "1: Device Identification request"]
    Value2 = 1,
}
impl From<Did> for bool {
    #[inline(always)]
    fn from(variant: Did) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DID` reader - Device Identification"]
pub type DidR = crate::BitReader<Did>;
impl DidR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Did {
        match self.bits {
            false => Did::Value1,
            true => Did::Value2,
        }
    }
    #[doc = "No request"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Did::Value1
    }
    #[doc = "Device Identification request"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Did::Value2
    }
}
impl R {
    #[doc = "Bits 0:3 - Initiate State Transition of the Device StateMachine"]
    #[inline(always)]
    pub fn ist(&self) -> IstR {
        IstR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - Error Ind Ack"]
    #[inline(always)]
    pub fn eia(&self) -> EiaR {
        EiaR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Device Identification"]
    #[inline(always)]
    pub fn did(&self) -> DidR {
        DidR::new(((self.bits >> 5) & 1) != 0)
    }
}
#[doc = "AL Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`al_control::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AlControlSpec;
impl crate::RegisterSpec for AlControlSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`al_control::R`](R) reader structure"]
impl crate::Readable for AlControlSpec {}
#[doc = "`reset()` method sets AL_CONTROL to value 0x01"]
impl crate::Resettable for AlControlSpec {
    const RESET_VALUE: u16 = 0x01;
}
