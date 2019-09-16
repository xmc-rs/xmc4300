#[doc = "Reader of register TC"]
pub type R = crate::R<u32, super::TC>;
#[doc = "Writer for register TC"]
pub type W = crate::W<u32, super::TC>;
#[doc = "Register TC `reset()`'s with value 0"]
impl crate::ResetValue for super::TC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Timer Counting Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TCM_A {
    #[doc = "0: Edge aligned mode"]
    VALUE1,
    #[doc = "1: Center aligned mode"]
    VALUE2,
}
impl From<TCM_A> for bool {
    #[inline(always)]
    fn from(variant: TCM_A) -> Self {
        match variant {
            TCM_A::VALUE1 => false,
            TCM_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `TCM`"]
pub type TCM_R = crate::R<bool, TCM_A>;
impl TCM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TCM_A {
        match self.bits {
            false => TCM_A::VALUE1,
            true => TCM_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == TCM_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == TCM_A::VALUE2
    }
}
#[doc = "Write proxy for field `TCM`"]
pub struct TCM_W<'a> {
    w: &'a mut W,
}
impl<'a> TCM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TCM_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Edge aligned mode"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(TCM_A::VALUE1)
    }
    #[doc = "Center aligned mode"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(TCM_A::VALUE2)
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Timer Single Shot Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TSSM_A {
    #[doc = "0: Single shot mode is disabled"]
    VALUE1,
    #[doc = "1: Single shot mode is enabled"]
    VALUE2,
}
impl From<TSSM_A> for bool {
    #[inline(always)]
    fn from(variant: TSSM_A) -> Self {
        match variant {
            TSSM_A::VALUE1 => false,
            TSSM_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `TSSM`"]
pub type TSSM_R = crate::R<bool, TSSM_A>;
impl TSSM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TSSM_A {
        match self.bits {
            false => TSSM_A::VALUE1,
            true => TSSM_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == TSSM_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == TSSM_A::VALUE2
    }
}
#[doc = "Write proxy for field `TSSM`"]
pub struct TSSM_W<'a> {
    w: &'a mut W,
}
impl<'a> TSSM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TSSM_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Single shot mode is disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(TSSM_A::VALUE1)
    }
    #[doc = "Single shot mode is enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(TSSM_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `CLST`"]
pub type CLST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CLST`"]
pub struct CLST_W<'a> {
    w: &'a mut W,
}
impl<'a> CLST_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Capture Compare Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CMOD_A {
    #[doc = "0: Compare Mode"]
    VALUE1,
    #[doc = "1: Capture Mode"]
    VALUE2,
}
impl From<CMOD_A> for bool {
    #[inline(always)]
    fn from(variant: CMOD_A) -> Self {
        match variant {
            CMOD_A::VALUE1 => false,
            CMOD_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `CMOD`"]
pub type CMOD_R = crate::R<bool, CMOD_A>;
impl CMOD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMOD_A {
        match self.bits {
            false => CMOD_A::VALUE1,
            true => CMOD_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CMOD_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CMOD_A::VALUE2
    }
}
#[doc = "Extended Capture Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ECM_A {
    #[doc = "0: Normal Capture Mode. Clear of the Full Flag of each capture register is done by accessing the registers individually only."]
    VALUE1,
    #[doc = "1: Extended Capture Mode. Clear of the Full Flag of each capture register is done not only by accessing the individual registers but also by accessing the ECRD register. When reading the ECRD register, only the capture register register full flag pointed by the ECRD.VPTR is cleared."]
    VALUE2,
}
impl From<ECM_A> for bool {
    #[inline(always)]
    fn from(variant: ECM_A) -> Self {
        match variant {
            ECM_A::VALUE1 => false,
            ECM_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `ECM`"]
pub type ECM_R = crate::R<bool, ECM_A>;
impl ECM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ECM_A {
        match self.bits {
            false => ECM_A::VALUE1,
            true => ECM_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == ECM_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == ECM_A::VALUE2
    }
}
#[doc = "Write proxy for field `ECM`"]
pub struct ECM_W<'a> {
    w: &'a mut W,
}
impl<'a> ECM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ECM_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Normal Capture Mode. Clear of the Full Flag of each capture register is done by accessing the registers individually only."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(ECM_A::VALUE1)
    }
    #[doc = "Extended Capture Mode. Clear of the Full Flag of each capture register is done not only by accessing the individual registers but also by accessing the ECRD register. When reading the ECRD register, only the capture register register full flag pointed by the ECRD.VPTR is cleared."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(ECM_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Clear on Capture Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CAPC_A {
    #[doc = "0: Timer is never cleared on a capture event"]
    VALUE1,
    #[doc = "1: Timer is cleared on a capture event into capture registers 2 and 3. (When SCE = 1#, Timer is always cleared in a capture event)"]
    VALUE2,
    #[doc = "2: Timer is cleared on a capture event into capture registers 0 and 1. (When SCE = 1#, Timer is always cleared in a capture event)"]
    VALUE3,
    #[doc = "3: Timer is always cleared in a capture event."]
    VALUE4,
}
impl From<CAPC_A> for u8 {
    #[inline(always)]
    fn from(variant: CAPC_A) -> Self {
        match variant {
            CAPC_A::VALUE1 => 0,
            CAPC_A::VALUE2 => 1,
            CAPC_A::VALUE3 => 2,
            CAPC_A::VALUE4 => 3,
        }
    }
}
#[doc = "Reader of field `CAPC`"]
pub type CAPC_R = crate::R<u8, CAPC_A>;
impl CAPC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CAPC_A {
        match self.bits {
            0 => CAPC_A::VALUE1,
            1 => CAPC_A::VALUE2,
            2 => CAPC_A::VALUE3,
            3 => CAPC_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CAPC_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CAPC_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == CAPC_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == CAPC_A::VALUE4
    }
}
#[doc = "Write proxy for field `CAPC`"]
pub struct CAPC_W<'a> {
    w: &'a mut W,
}
impl<'a> CAPC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CAPC_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Timer is never cleared on a capture event"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CAPC_A::VALUE1)
    }
    #[doc = "Timer is cleared on a capture event into capture registers 2 and 3. (When SCE = 1#, Timer is always cleared in a capture event)"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CAPC_A::VALUE2)
    }
    #[doc = "Timer is cleared on a capture event into capture registers 0 and 1. (When SCE = 1#, Timer is always cleared in a capture event)"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(CAPC_A::VALUE3)
    }
    #[doc = "Timer is always cleared in a capture event."]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(CAPC_A::VALUE4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 5)) | (((value as u32) & 0x03) << 5);
        self.w
    }
}
#[doc = "Extended Stop Function Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENDM_A {
    #[doc = "0: Clears the timer run bit only (default stop)"]
    VALUE1,
    #[doc = "1: Clears the timer only (flush)"]
    VALUE2,
    #[doc = "2: Clears the timer and run bit (flush/stop)"]
    VALUE3,
}
impl From<ENDM_A> for u8 {
    #[inline(always)]
    fn from(variant: ENDM_A) -> Self {
        match variant {
            ENDM_A::VALUE1 => 0,
            ENDM_A::VALUE2 => 1,
            ENDM_A::VALUE3 => 2,
        }
    }
}
#[doc = "Reader of field `ENDM`"]
pub type ENDM_R = crate::R<u8, ENDM_A>;
impl ENDM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, ENDM_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(ENDM_A::VALUE1),
            1 => Val(ENDM_A::VALUE2),
            2 => Val(ENDM_A::VALUE3),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == ENDM_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == ENDM_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == ENDM_A::VALUE3
    }
}
#[doc = "Write proxy for field `ENDM`"]
pub struct ENDM_W<'a> {
    w: &'a mut W,
}
impl<'a> ENDM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ENDM_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Clears the timer run bit only (default stop)"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(ENDM_A::VALUE1)
    }
    #[doc = "Clears the timer only (flush)"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(ENDM_A::VALUE2)
    }
    #[doc = "Clears the timer and run bit (flush/stop)"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(ENDM_A::VALUE3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "Extended Start Function Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STRM_A {
    #[doc = "0: Sets run bit only (default start)"]
    VALUE1,
    #[doc = "1: Clears the timer and sets run bit (flush/start)"]
    VALUE2,
}
impl From<STRM_A> for bool {
    #[inline(always)]
    fn from(variant: STRM_A) -> Self {
        match variant {
            STRM_A::VALUE1 => false,
            STRM_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `STRM`"]
pub type STRM_R = crate::R<bool, STRM_A>;
impl STRM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STRM_A {
        match self.bits {
            false => STRM_A::VALUE1,
            true => STRM_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == STRM_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == STRM_A::VALUE2
    }
}
#[doc = "Write proxy for field `STRM`"]
pub struct STRM_W<'a> {
    w: &'a mut W,
}
impl<'a> STRM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: STRM_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Sets run bit only (default start)"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(STRM_A::VALUE1)
    }
    #[doc = "Clears the timer and sets run bit (flush/start)"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(STRM_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Equal Capture Event enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SCE_A {
    #[doc = "0: Capture into CC4yC0VThis register contains the values associated with the Capture 0 field./CC4yC1VThis register contains the values associated with the Capture 1 field. registers control by CCycapt0 and capture into CC4yC3VThis register contains the values associated with the Capture 3 field./CC4yC2VThis register contains the values associated with the Capture 2 field. control by CCycapt1"]
    VALUE1,
    #[doc = "1: Capture into CC4yC0VThis register contains the values associated with the Capture 0 field./CC4yC1VThis register contains the values associated with the Capture 1 field. and CC4yC3VThis register contains the values associated with the Capture 3 field./CC4yC2VThis register contains the values associated with the Capture 2 field. control by CCycapt1"]
    VALUE2,
}
impl From<SCE_A> for bool {
    #[inline(always)]
    fn from(variant: SCE_A) -> Self {
        match variant {
            SCE_A::VALUE1 => false,
            SCE_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `SCE`"]
pub type SCE_R = crate::R<bool, SCE_A>;
impl SCE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SCE_A {
        match self.bits {
            false => SCE_A::VALUE1,
            true => SCE_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SCE_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SCE_A::VALUE2
    }
}
#[doc = "Write proxy for field `SCE`"]
pub struct SCE_W<'a> {
    w: &'a mut W,
}
impl<'a> SCE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SCE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Capture into CC4yC0VThis register contains the values associated with the Capture 0 field./CC4yC1VThis register contains the values associated with the Capture 1 field. registers control by CCycapt0 and capture into CC4yC3VThis register contains the values associated with the Capture 3 field./CC4yC2VThis register contains the values associated with the Capture 2 field. control by CCycapt1"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(SCE_A::VALUE1)
    }
    #[doc = "Capture into CC4yC0VThis register contains the values associated with the Capture 0 field./CC4yC1VThis register contains the values associated with the Capture 1 field. and CC4yC3VThis register contains the values associated with the Capture 3 field./CC4yC2VThis register contains the values associated with the Capture 2 field. control by CCycapt1"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(SCE_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Continuous Capture Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CCS_A {
    #[doc = "0: The capture into a specific capture register is done with the rules linked with the full flags, described at ."]
    VALUE1,
    #[doc = "1: The capture into the capture registers is always done regardless of the full flag status (even if the register has not been read back)."]
    VALUE2,
}
impl From<CCS_A> for bool {
    #[inline(always)]
    fn from(variant: CCS_A) -> Self {
        match variant {
            CCS_A::VALUE1 => false,
            CCS_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `CCS`"]
pub type CCS_R = crate::R<bool, CCS_A>;
impl CCS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CCS_A {
        match self.bits {
            false => CCS_A::VALUE1,
            true => CCS_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CCS_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CCS_A::VALUE2
    }
}
#[doc = "Write proxy for field `CCS`"]
pub struct CCS_W<'a> {
    w: &'a mut W,
}
impl<'a> CCS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CCS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The capture into a specific capture register is done with the rules linked with the full flags, described at ."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CCS_A::VALUE1)
    }
    #[doc = "The capture into the capture registers is always done regardless of the full flag status (even if the register has not been read back)."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CCS_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Dither Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DITHE_A {
    #[doc = "0: Dither is disabled"]
    VALUE1,
    #[doc = "1: Dither is applied to the Period"]
    VALUE2,
    #[doc = "2: Dither is applied to the Compare"]
    VALUE3,
    #[doc = "3: Dither is applied to the Period and Compare"]
    VALUE4,
}
impl From<DITHE_A> for u8 {
    #[inline(always)]
    fn from(variant: DITHE_A) -> Self {
        match variant {
            DITHE_A::VALUE1 => 0,
            DITHE_A::VALUE2 => 1,
            DITHE_A::VALUE3 => 2,
            DITHE_A::VALUE4 => 3,
        }
    }
}
#[doc = "Reader of field `DITHE`"]
pub type DITHE_R = crate::R<u8, DITHE_A>;
impl DITHE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DITHE_A {
        match self.bits {
            0 => DITHE_A::VALUE1,
            1 => DITHE_A::VALUE2,
            2 => DITHE_A::VALUE3,
            3 => DITHE_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == DITHE_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == DITHE_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == DITHE_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == DITHE_A::VALUE4
    }
}
#[doc = "Write proxy for field `DITHE`"]
pub struct DITHE_W<'a> {
    w: &'a mut W,
}
impl<'a> DITHE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DITHE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Dither is disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(DITHE_A::VALUE1)
    }
    #[doc = "Dither is applied to the Period"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(DITHE_A::VALUE2)
    }
    #[doc = "Dither is applied to the Compare"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(DITHE_A::VALUE3)
    }
    #[doc = "Dither is applied to the Period and Compare"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(DITHE_A::VALUE4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 13)) | (((value as u32) & 0x03) << 13);
        self.w
    }
}
#[doc = "Dither input selector\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIM_A {
    #[doc = "0: Slice is using its own dither unit"]
    VALUE1,
    #[doc = "1: Slice is connected to the dither unit of slice 0."]
    VALUE2,
}
impl From<DIM_A> for bool {
    #[inline(always)]
    fn from(variant: DIM_A) -> Self {
        match variant {
            DIM_A::VALUE1 => false,
            DIM_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `DIM`"]
pub type DIM_R = crate::R<bool, DIM_A>;
impl DIM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIM_A {
        match self.bits {
            false => DIM_A::VALUE1,
            true => DIM_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == DIM_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == DIM_A::VALUE2
    }
}
#[doc = "Write proxy for field `DIM`"]
pub struct DIM_W<'a> {
    w: &'a mut W,
}
impl<'a> DIM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIM_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Slice is using its own dither unit"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(DIM_A::VALUE1)
    }
    #[doc = "Slice is connected to the dither unit of slice 0."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(DIM_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = "Floating Prescaler enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FPE_A {
    #[doc = "0: Floating prescaler mode is disabled"]
    VALUE1,
    #[doc = "1: Floating prescaler mode is enabled"]
    VALUE2,
}
impl From<FPE_A> for bool {
    #[inline(always)]
    fn from(variant: FPE_A) -> Self {
        match variant {
            FPE_A::VALUE1 => false,
            FPE_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `FPE`"]
pub type FPE_R = crate::R<bool, FPE_A>;
impl FPE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FPE_A {
        match self.bits {
            false => FPE_A::VALUE1,
            true => FPE_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == FPE_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == FPE_A::VALUE2
    }
}
#[doc = "Write proxy for field `FPE`"]
pub struct FPE_W<'a> {
    w: &'a mut W,
}
impl<'a> FPE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FPE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Floating prescaler mode is disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(FPE_A::VALUE1)
    }
    #[doc = "Floating prescaler mode is enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(FPE_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "TRAP enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRAPE_A {
    #[doc = "0: TRAP functionality has no effect on the output"]
    VALUE1,
    #[doc = "1: TRAP functionality affects the output"]
    VALUE2,
}
impl From<TRAPE_A> for bool {
    #[inline(always)]
    fn from(variant: TRAPE_A) -> Self {
        match variant {
            TRAPE_A::VALUE1 => false,
            TRAPE_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `TRAPE`"]
pub type TRAPE_R = crate::R<bool, TRAPE_A>;
impl TRAPE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRAPE_A {
        match self.bits {
            false => TRAPE_A::VALUE1,
            true => TRAPE_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == TRAPE_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == TRAPE_A::VALUE2
    }
}
#[doc = "Write proxy for field `TRAPE`"]
pub struct TRAPE_W<'a> {
    w: &'a mut W,
}
impl<'a> TRAPE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TRAPE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "TRAP functionality has no effect on the output"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(TRAPE_A::VALUE1)
    }
    #[doc = "TRAP functionality affects the output"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(TRAPE_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "TRAP Synchronization Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRPSE_A {
    #[doc = "0: Exiting from TRAP state isn't synchronized with the PWM signal"]
    VALUE1,
    #[doc = "1: Exiting from TRAP state is synchronized with the PWM signal"]
    VALUE2,
}
impl From<TRPSE_A> for bool {
    #[inline(always)]
    fn from(variant: TRPSE_A) -> Self {
        match variant {
            TRPSE_A::VALUE1 => false,
            TRPSE_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `TRPSE`"]
pub type TRPSE_R = crate::R<bool, TRPSE_A>;
impl TRPSE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRPSE_A {
        match self.bits {
            false => TRPSE_A::VALUE1,
            true => TRPSE_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == TRPSE_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == TRPSE_A::VALUE2
    }
}
#[doc = "Write proxy for field `TRPSE`"]
pub struct TRPSE_W<'a> {
    w: &'a mut W,
}
impl<'a> TRPSE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TRPSE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Exiting from TRAP state isn't synchronized with the PWM signal"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(TRPSE_A::VALUE1)
    }
    #[doc = "Exiting from TRAP state is synchronized with the PWM signal"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(TRPSE_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
        self.w
    }
}
#[doc = "TRAP State Clear Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRPSW_A {
    #[doc = "0: The slice exits the TRAP state automatically when the TRAP condition is not present"]
    VALUE1,
    #[doc = "1: The TRAP state can only be exited by a SW request."]
    VALUE2,
}
impl From<TRPSW_A> for bool {
    #[inline(always)]
    fn from(variant: TRPSW_A) -> Self {
        match variant {
            TRPSW_A::VALUE1 => false,
            TRPSW_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `TRPSW`"]
pub type TRPSW_R = crate::R<bool, TRPSW_A>;
impl TRPSW_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRPSW_A {
        match self.bits {
            false => TRPSW_A::VALUE1,
            true => TRPSW_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == TRPSW_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == TRPSW_A::VALUE2
    }
}
#[doc = "Write proxy for field `TRPSW`"]
pub struct TRPSW_W<'a> {
    w: &'a mut W,
}
impl<'a> TRPSW_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TRPSW_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The slice exits the TRAP state automatically when the TRAP condition is not present"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(TRPSW_A::VALUE1)
    }
    #[doc = "The TRAP state can only be exited by a SW request."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(TRPSW_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
        self.w
    }
}
#[doc = "External Modulation Synchronization\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EMS_A {
    #[doc = "0: External Modulation functionality is not synchronized with the PWM signal"]
    VALUE1,
    #[doc = "1: External Modulation functionality is synchronized with the PWM signal"]
    VALUE2,
}
impl From<EMS_A> for bool {
    #[inline(always)]
    fn from(variant: EMS_A) -> Self {
        match variant {
            EMS_A::VALUE1 => false,
            EMS_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `EMS`"]
pub type EMS_R = crate::R<bool, EMS_A>;
impl EMS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EMS_A {
        match self.bits {
            false => EMS_A::VALUE1,
            true => EMS_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == EMS_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == EMS_A::VALUE2
    }
}
#[doc = "Write proxy for field `EMS`"]
pub struct EMS_W<'a> {
    w: &'a mut W,
}
impl<'a> EMS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EMS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "External Modulation functionality is not synchronized with the PWM signal"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(EMS_A::VALUE1)
    }
    #[doc = "External Modulation functionality is synchronized with the PWM signal"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(EMS_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
        self.w
    }
}
#[doc = "External Modulation Type\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EMT_A {
    #[doc = "0: External Modulation functionality is clearing the CC4yST bit."]
    VALUE1,
    #[doc = "1: External Modulation functionality is gating the outputs."]
    VALUE2,
}
impl From<EMT_A> for bool {
    #[inline(always)]
    fn from(variant: EMT_A) -> Self {
        match variant {
            EMT_A::VALUE1 => false,
            EMT_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `EMT`"]
pub type EMT_R = crate::R<bool, EMT_A>;
impl EMT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EMT_A {
        match self.bits {
            false => EMT_A::VALUE1,
            true => EMT_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == EMT_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == EMT_A::VALUE2
    }
}
#[doc = "Write proxy for field `EMT`"]
pub struct EMT_W<'a> {
    w: &'a mut W,
}
impl<'a> EMT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EMT_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "External Modulation functionality is clearing the CC4yST bit."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(EMT_A::VALUE1)
    }
    #[doc = "External Modulation functionality is gating the outputs."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(EMT_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
#[doc = "Multi Channel Mode Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MCME_A {
    #[doc = "0: Multi Channel Mode is disabled"]
    VALUE1,
    #[doc = "1: Multi Channel Mode is enabled"]
    VALUE2,
}
impl From<MCME_A> for bool {
    #[inline(always)]
    fn from(variant: MCME_A) -> Self {
        match variant {
            MCME_A::VALUE1 => false,
            MCME_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `MCME`"]
pub type MCME_R = crate::R<bool, MCME_A>;
impl MCME_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MCME_A {
        match self.bits {
            false => MCME_A::VALUE1,
            true => MCME_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == MCME_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == MCME_A::VALUE2
    }
}
#[doc = "Write proxy for field `MCME`"]
pub struct MCME_W<'a> {
    w: &'a mut W,
}
impl<'a> MCME_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MCME_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Multi Channel Mode is disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(MCME_A::VALUE1)
    }
    #[doc = "Multi Channel Mode is enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(MCME_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Timer Counting Mode"]
    #[inline(always)]
    pub fn tcm(&self) -> TCM_R {
        TCM_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Timer Single Shot Mode"]
    #[inline(always)]
    pub fn tssm(&self) -> TSSM_R {
        TSSM_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Shadow Transfer on Clear"]
    #[inline(always)]
    pub fn clst(&self) -> CLST_R {
        CLST_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Capture Compare Mode"]
    #[inline(always)]
    pub fn cmod(&self) -> CMOD_R {
        CMOD_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Extended Capture Mode"]
    #[inline(always)]
    pub fn ecm(&self) -> ECM_R {
        ECM_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 5:6 - Clear on Capture Control"]
    #[inline(always)]
    pub fn capc(&self) -> CAPC_R {
        CAPC_R::new(((self.bits >> 5) & 0x03) as u8)
    }
    #[doc = "Bits 8:9 - Extended Stop Function Control"]
    #[inline(always)]
    pub fn endm(&self) -> ENDM_R {
        ENDM_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bit 10 - Extended Start Function Control"]
    #[inline(always)]
    pub fn strm(&self) -> STRM_R {
        STRM_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Equal Capture Event enable"]
    #[inline(always)]
    pub fn sce(&self) -> SCE_R {
        SCE_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Continuous Capture Enable"]
    #[inline(always)]
    pub fn ccs(&self) -> CCS_R {
        CCS_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bits 13:14 - Dither Enable"]
    #[inline(always)]
    pub fn dithe(&self) -> DITHE_R {
        DITHE_R::new(((self.bits >> 13) & 0x03) as u8)
    }
    #[doc = "Bit 15 - Dither input selector"]
    #[inline(always)]
    pub fn dim(&self) -> DIM_R {
        DIM_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Floating Prescaler enable"]
    #[inline(always)]
    pub fn fpe(&self) -> FPE_R {
        FPE_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - TRAP enable"]
    #[inline(always)]
    pub fn trape(&self) -> TRAPE_R {
        TRAPE_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 21 - TRAP Synchronization Enable"]
    #[inline(always)]
    pub fn trpse(&self) -> TRPSE_R {
        TRPSE_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - TRAP State Clear Control"]
    #[inline(always)]
    pub fn trpsw(&self) -> TRPSW_R {
        TRPSW_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - External Modulation Synchronization"]
    #[inline(always)]
    pub fn ems(&self) -> EMS_R {
        EMS_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - External Modulation Type"]
    #[inline(always)]
    pub fn emt(&self) -> EMT_R {
        EMT_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Multi Channel Mode Enable"]
    #[inline(always)]
    pub fn mcme(&self) -> MCME_R {
        MCME_R::new(((self.bits >> 25) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Timer Counting Mode"]
    #[inline(always)]
    pub fn tcm(&mut self) -> TCM_W {
        TCM_W { w: self }
    }
    #[doc = "Bit 1 - Timer Single Shot Mode"]
    #[inline(always)]
    pub fn tssm(&mut self) -> TSSM_W {
        TSSM_W { w: self }
    }
    #[doc = "Bit 2 - Shadow Transfer on Clear"]
    #[inline(always)]
    pub fn clst(&mut self) -> CLST_W {
        CLST_W { w: self }
    }
    #[doc = "Bit 4 - Extended Capture Mode"]
    #[inline(always)]
    pub fn ecm(&mut self) -> ECM_W {
        ECM_W { w: self }
    }
    #[doc = "Bits 5:6 - Clear on Capture Control"]
    #[inline(always)]
    pub fn capc(&mut self) -> CAPC_W {
        CAPC_W { w: self }
    }
    #[doc = "Bits 8:9 - Extended Stop Function Control"]
    #[inline(always)]
    pub fn endm(&mut self) -> ENDM_W {
        ENDM_W { w: self }
    }
    #[doc = "Bit 10 - Extended Start Function Control"]
    #[inline(always)]
    pub fn strm(&mut self) -> STRM_W {
        STRM_W { w: self }
    }
    #[doc = "Bit 11 - Equal Capture Event enable"]
    #[inline(always)]
    pub fn sce(&mut self) -> SCE_W {
        SCE_W { w: self }
    }
    #[doc = "Bit 12 - Continuous Capture Enable"]
    #[inline(always)]
    pub fn ccs(&mut self) -> CCS_W {
        CCS_W { w: self }
    }
    #[doc = "Bits 13:14 - Dither Enable"]
    #[inline(always)]
    pub fn dithe(&mut self) -> DITHE_W {
        DITHE_W { w: self }
    }
    #[doc = "Bit 15 - Dither input selector"]
    #[inline(always)]
    pub fn dim(&mut self) -> DIM_W {
        DIM_W { w: self }
    }
    #[doc = "Bit 16 - Floating Prescaler enable"]
    #[inline(always)]
    pub fn fpe(&mut self) -> FPE_W {
        FPE_W { w: self }
    }
    #[doc = "Bit 17 - TRAP enable"]
    #[inline(always)]
    pub fn trape(&mut self) -> TRAPE_W {
        TRAPE_W { w: self }
    }
    #[doc = "Bit 21 - TRAP Synchronization Enable"]
    #[inline(always)]
    pub fn trpse(&mut self) -> TRPSE_W {
        TRPSE_W { w: self }
    }
    #[doc = "Bit 22 - TRAP State Clear Control"]
    #[inline(always)]
    pub fn trpsw(&mut self) -> TRPSW_W {
        TRPSW_W { w: self }
    }
    #[doc = "Bit 23 - External Modulation Synchronization"]
    #[inline(always)]
    pub fn ems(&mut self) -> EMS_W {
        EMS_W { w: self }
    }
    #[doc = "Bit 24 - External Modulation Type"]
    #[inline(always)]
    pub fn emt(&mut self) -> EMT_W {
        EMT_W { w: self }
    }
    #[doc = "Bit 25 - Multi Channel Mode Enable"]
    #[inline(always)]
    pub fn mcme(&mut self) -> MCME_W {
        MCME_W { w: self }
    }
}
