#[doc = "Reader of register AL_STATUS"]
pub type R = crate::R<u16, super::AL_STATUS>;
#[doc = "Writer for register AL_STATUS"]
pub type W = crate::W<u16, super::AL_STATUS>;
#[doc = "Register AL_STATUS `reset()`'s with value 0x01"]
impl crate::ResetValue for super::AL_STATUS {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x01
    }
}
#[doc = "Actual State of the Device State Machine\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum STATE_A {
    #[doc = "1: Init State"]
    VALUE1 = 1,
    #[doc = "2: Pre-Operational State"]
    VALUE2 = 2,
    #[doc = "3: Bootstrap State"]
    VALUE3 = 3,
    #[doc = "4: Safe-Operational State"]
    VALUE4 = 4,
    #[doc = "8: Operational State"]
    VALUE5 = 8,
}
impl From<STATE_A> for u8 {
    #[inline(always)]
    fn from(variant: STATE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `STATE`"]
pub type STATE_R = crate::R<u8, STATE_A>;
impl STATE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, STATE_A> {
        use crate::Variant::*;
        match self.bits {
            1 => Val(STATE_A::VALUE1),
            2 => Val(STATE_A::VALUE2),
            3 => Val(STATE_A::VALUE3),
            4 => Val(STATE_A::VALUE4),
            8 => Val(STATE_A::VALUE5),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == STATE_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == STATE_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == STATE_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == STATE_A::VALUE4
    }
    #[doc = "Checks if the value of the field is `VALUE5`"]
    #[inline(always)]
    pub fn is_value5(&self) -> bool {
        *self == STATE_A::VALUE5
    }
}
#[doc = "Write proxy for field `STATE`"]
pub struct STATE_W<'a> {
    w: &'a mut W,
}
impl<'a> STATE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: STATE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Init State"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(STATE_A::VALUE1)
    }
    #[doc = "Pre-Operational State"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(STATE_A::VALUE2)
    }
    #[doc = "Bootstrap State"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(STATE_A::VALUE3)
    }
    #[doc = "Safe-Operational State"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(STATE_A::VALUE4)
    }
    #[doc = "Operational State"]
    #[inline(always)]
    pub fn value5(self) -> &'a mut W {
        self.variant(STATE_A::VALUE5)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u16) & 0x0f);
        self.w
    }
}
#[doc = "Error Ind\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERRI_A {
    #[doc = "0: Device is in State as requested or Flag cleared by command"]
    VALUE1 = 0,
    #[doc = "1: Device has not entered requested State or changed State as result of a local action"]
    VALUE2 = 1,
}
impl From<ERRI_A> for bool {
    #[inline(always)]
    fn from(variant: ERRI_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ERRI`"]
pub type ERRI_R = crate::R<bool, ERRI_A>;
impl ERRI_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERRI_A {
        match self.bits {
            false => ERRI_A::VALUE1,
            true => ERRI_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == ERRI_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == ERRI_A::VALUE2
    }
}
#[doc = "Write proxy for field `ERRI`"]
pub struct ERRI_W<'a> {
    w: &'a mut W,
}
impl<'a> ERRI_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ERRI_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Device is in State as requested or Flag cleared by command"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(ERRI_A::VALUE1)
    }
    #[doc = "Device has not entered requested State or changed State as result of a local action"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(ERRI_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u16) & 0x01) << 4);
        self.w
    }
}
#[doc = "Device Identification\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DID_A {
    #[doc = "0: Device Identification not valid"]
    VALUE1 = 0,
    #[doc = "1: Device Identification loaded"]
    VALUE2 = 1,
}
impl From<DID_A> for bool {
    #[inline(always)]
    fn from(variant: DID_A) -> Self {
        variant as u8 != 0
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
#[doc = "Write proxy for field `DID`"]
pub struct DID_W<'a> {
    w: &'a mut W,
}
impl<'a> DID_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DID_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Device Identification not valid"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(DID_A::VALUE1)
    }
    #[doc = "Device Identification loaded"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(DID_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u16) & 0x01) << 5);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Actual State of the Device State Machine"]
    #[inline(always)]
    pub fn state(&self) -> STATE_R {
        STATE_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - Error Ind"]
    #[inline(always)]
    pub fn erri(&self) -> ERRI_R {
        ERRI_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Device Identification"]
    #[inline(always)]
    pub fn did(&self) -> DID_R {
        DID_R::new(((self.bits >> 5) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Actual State of the Device State Machine"]
    #[inline(always)]
    pub fn state(&mut self) -> STATE_W {
        STATE_W { w: self }
    }
    #[doc = "Bit 4 - Error Ind"]
    #[inline(always)]
    pub fn erri(&mut self) -> ERRI_W {
        ERRI_W { w: self }
    }
    #[doc = "Bit 5 - Device Identification"]
    #[inline(always)]
    pub fn did(&mut self) -> DID_W {
        DID_W { w: self }
    }
}
