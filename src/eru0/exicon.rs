#[doc = "Reader of register EXICON[%s]"]
pub type R = crate::R<u32, super::EXICON>;
#[doc = "Writer for register EXICON[%s]"]
pub type W = crate::W<u32, super::EXICON>;
#[doc = "Register EXICON[%s]
`reset()`'s with value 0"]
impl crate::ResetValue for super::EXICON {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Output Trigger Pulse Enable for ETLx\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PE_A {
    #[doc = "0: The trigger pulse generation is disabled"]
    VALUE1 = 0,
    #[doc = "1: The trigger pulse generation is enabled"]
    VALUE2 = 1,
}
impl From<PE_A> for bool {
    #[inline(always)]
    fn from(variant: PE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PE`"]
pub type PE_R = crate::R<bool, PE_A>;
impl PE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PE_A {
        match self.bits {
            false => PE_A::VALUE1,
            true => PE_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PE_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PE_A::VALUE2
    }
}
#[doc = "Write proxy for field `PE`"]
pub struct PE_W<'a> {
    w: &'a mut W,
}
impl<'a> PE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The trigger pulse generation is disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(PE_A::VALUE1)
    }
    #[doc = "The trigger pulse generation is enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(PE_A::VALUE2)
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
#[doc = "Rebuild Level Detection for Status Flag for ETLx\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LD_A {
    #[doc = "0: The status flag FL is not cleared by hardware and is used as \"sticky\" bit. Once set, it is not influenced by any edge until it becomes cleared by software."]
    VALUE1 = 0,
    #[doc = "1: The status flag FL rebuilds a level detection of the desired event. It becomes automatically set with a rising edge if RE = 1 or with a falling edge if FE = 1. It becomes automatically cleared with a rising edge if RE = 0 or with a falling edge if FE = 0."]
    VALUE2 = 1,
}
impl From<LD_A> for bool {
    #[inline(always)]
    fn from(variant: LD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LD`"]
pub type LD_R = crate::R<bool, LD_A>;
impl LD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LD_A {
        match self.bits {
            false => LD_A::VALUE1,
            true => LD_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == LD_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == LD_A::VALUE2
    }
}
#[doc = "Write proxy for field `LD`"]
pub struct LD_W<'a> {
    w: &'a mut W,
}
impl<'a> LD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The status flag FL is not cleared by hardware and is used as \"sticky\" bit. Once set, it is not influenced by any edge until it becomes cleared by software."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(LD_A::VALUE1)
    }
    #[doc = "The status flag FL rebuilds a level detection of the desired event. It becomes automatically set with a rising edge if RE = 1 or with a falling edge if FE = 1. It becomes automatically cleared with a rising edge if RE = 0 or with a falling edge if FE = 0."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(LD_A::VALUE2)
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
#[doc = "Rising Edge Detection Enable ETLx\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RE_A {
    #[doc = "0: A rising edge is not considered as edge event"]
    VALUE1 = 0,
    #[doc = "1: A rising edge is considered as edge event"]
    VALUE2 = 1,
}
impl From<RE_A> for bool {
    #[inline(always)]
    fn from(variant: RE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RE`"]
pub type RE_R = crate::R<bool, RE_A>;
impl RE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RE_A {
        match self.bits {
            false => RE_A::VALUE1,
            true => RE_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == RE_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == RE_A::VALUE2
    }
}
#[doc = "Write proxy for field `RE`"]
pub struct RE_W<'a> {
    w: &'a mut W,
}
impl<'a> RE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "A rising edge is not considered as edge event"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(RE_A::VALUE1)
    }
    #[doc = "A rising edge is considered as edge event"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(RE_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Falling Edge Detection Enable ETLx\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FE_A {
    #[doc = "0: A falling edge is not considered as edge event"]
    VALUE1 = 0,
    #[doc = "1: A falling edge is considered as edge event"]
    VALUE2 = 1,
}
impl From<FE_A> for bool {
    #[inline(always)]
    fn from(variant: FE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FE`"]
pub type FE_R = crate::R<bool, FE_A>;
impl FE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FE_A {
        match self.bits {
            false => FE_A::VALUE1,
            true => FE_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == FE_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == FE_A::VALUE2
    }
}
#[doc = "Write proxy for field `FE`"]
pub struct FE_W<'a> {
    w: &'a mut W,
}
impl<'a> FE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "A falling edge is not considered as edge event"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(FE_A::VALUE1)
    }
    #[doc = "A falling edge is considered as edge event"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(FE_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Output Channel Select for ETLx Output Trigger Pulse\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum OCS_A {
    #[doc = "0: Trigger pulses are sent to OGU0"]
    VALUE1 = 0,
    #[doc = "1: Trigger pulses are sent to OGU1"]
    VALUE2 = 1,
    #[doc = "2: Trigger pulses are sent to OGU2"]
    VALUE3 = 2,
    #[doc = "3: Trigger pulses are sent to OGU3"]
    VALUE4 = 3,
}
impl From<OCS_A> for u8 {
    #[inline(always)]
    fn from(variant: OCS_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `OCS`"]
pub type OCS_R = crate::R<u8, OCS_A>;
impl OCS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, OCS_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(OCS_A::VALUE1),
            1 => Val(OCS_A::VALUE2),
            2 => Val(OCS_A::VALUE3),
            3 => Val(OCS_A::VALUE4),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == OCS_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == OCS_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == OCS_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == OCS_A::VALUE4
    }
}
#[doc = "Write proxy for field `OCS`"]
pub struct OCS_W<'a> {
    w: &'a mut W,
}
impl<'a> OCS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OCS_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Trigger pulses are sent to OGU0"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(OCS_A::VALUE1)
    }
    #[doc = "Trigger pulses are sent to OGU1"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(OCS_A::VALUE2)
    }
    #[doc = "Trigger pulses are sent to OGU2"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(OCS_A::VALUE3)
    }
    #[doc = "Trigger pulses are sent to OGU3"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(OCS_A::VALUE4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | (((value as u32) & 0x07) << 4);
        self.w
    }
}
#[doc = "Status Flag for ETLx\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FL_A {
    #[doc = "0: The enabled edge event has not been detected"]
    VALUE1 = 0,
    #[doc = "1: The enabled edge event has been detected"]
    VALUE2 = 1,
}
impl From<FL_A> for bool {
    #[inline(always)]
    fn from(variant: FL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FL`"]
pub type FL_R = crate::R<bool, FL_A>;
impl FL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FL_A {
        match self.bits {
            false => FL_A::VALUE1,
            true => FL_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == FL_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == FL_A::VALUE2
    }
}
#[doc = "Write proxy for field `FL`"]
pub struct FL_W<'a> {
    w: &'a mut W,
}
impl<'a> FL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The enabled edge event has not been detected"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(FL_A::VALUE1)
    }
    #[doc = "The enabled edge event has been detected"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(FL_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Input Source Select for ERSx\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SS_A {
    #[doc = "0: Input A without additional combination"]
    VALUE1 = 0,
    #[doc = "1: Input B without additional combination"]
    VALUE2 = 1,
    #[doc = "2: Input A OR input B"]
    VALUE3 = 2,
    #[doc = "3: Input A AND input B"]
    VALUE4 = 3,
}
impl From<SS_A> for u8 {
    #[inline(always)]
    fn from(variant: SS_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SS`"]
pub type SS_R = crate::R<u8, SS_A>;
impl SS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SS_A {
        match self.bits {
            0 => SS_A::VALUE1,
            1 => SS_A::VALUE2,
            2 => SS_A::VALUE3,
            3 => SS_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SS_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SS_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == SS_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == SS_A::VALUE4
    }
}
#[doc = "Write proxy for field `SS`"]
pub struct SS_W<'a> {
    w: &'a mut W,
}
impl<'a> SS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SS_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Input A without additional combination"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(SS_A::VALUE1)
    }
    #[doc = "Input B without additional combination"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(SS_A::VALUE2)
    }
    #[doc = "Input A OR input B"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(SS_A::VALUE3)
    }
    #[doc = "Input A AND input B"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(SS_A::VALUE4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "Input A Negation Select for ERSx\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NA_A {
    #[doc = "0: Input A is used directly"]
    VALUE1 = 0,
    #[doc = "1: Input A is inverted"]
    VALUE2 = 1,
}
impl From<NA_A> for bool {
    #[inline(always)]
    fn from(variant: NA_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `NA`"]
pub type NA_R = crate::R<bool, NA_A>;
impl NA_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NA_A {
        match self.bits {
            false => NA_A::VALUE1,
            true => NA_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == NA_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == NA_A::VALUE2
    }
}
#[doc = "Write proxy for field `NA`"]
pub struct NA_W<'a> {
    w: &'a mut W,
}
impl<'a> NA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: NA_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Input A is used directly"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(NA_A::VALUE1)
    }
    #[doc = "Input A is inverted"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(NA_A::VALUE2)
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
#[doc = "Input B Negation Select for ERSx\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NB_A {
    #[doc = "0: Input B is used directly"]
    VALUE1 = 0,
    #[doc = "1: Input B is inverted"]
    VALUE2 = 1,
}
impl From<NB_A> for bool {
    #[inline(always)]
    fn from(variant: NB_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `NB`"]
pub type NB_R = crate::R<bool, NB_A>;
impl NB_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NB_A {
        match self.bits {
            false => NB_A::VALUE1,
            true => NB_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == NB_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == NB_A::VALUE2
    }
}
#[doc = "Write proxy for field `NB`"]
pub struct NB_W<'a> {
    w: &'a mut W,
}
impl<'a> NB_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: NB_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Input B is used directly"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(NB_A::VALUE1)
    }
    #[doc = "Input B is inverted"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(NB_A::VALUE2)
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
impl R {
    #[doc = "Bit 0 - Output Trigger Pulse Enable for ETLx"]
    #[inline(always)]
    pub fn pe(&self) -> PE_R {
        PE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Rebuild Level Detection for Status Flag for ETLx"]
    #[inline(always)]
    pub fn ld(&self) -> LD_R {
        LD_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Rising Edge Detection Enable ETLx"]
    #[inline(always)]
    pub fn re(&self) -> RE_R {
        RE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Falling Edge Detection Enable ETLx"]
    #[inline(always)]
    pub fn fe(&self) -> FE_R {
        FE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 4:6 - Output Channel Select for ETLx Output Trigger Pulse"]
    #[inline(always)]
    pub fn ocs(&self) -> OCS_R {
        OCS_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bit 7 - Status Flag for ETLx"]
    #[inline(always)]
    pub fn fl(&self) -> FL_R {
        FL_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 8:9 - Input Source Select for ERSx"]
    #[inline(always)]
    pub fn ss(&self) -> SS_R {
        SS_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bit 10 - Input A Negation Select for ERSx"]
    #[inline(always)]
    pub fn na(&self) -> NA_R {
        NA_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Input B Negation Select for ERSx"]
    #[inline(always)]
    pub fn nb(&self) -> NB_R {
        NB_R::new(((self.bits >> 11) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Output Trigger Pulse Enable for ETLx"]
    #[inline(always)]
    pub fn pe(&mut self) -> PE_W {
        PE_W { w: self }
    }
    #[doc = "Bit 1 - Rebuild Level Detection for Status Flag for ETLx"]
    #[inline(always)]
    pub fn ld(&mut self) -> LD_W {
        LD_W { w: self }
    }
    #[doc = "Bit 2 - Rising Edge Detection Enable ETLx"]
    #[inline(always)]
    pub fn re(&mut self) -> RE_W {
        RE_W { w: self }
    }
    #[doc = "Bit 3 - Falling Edge Detection Enable ETLx"]
    #[inline(always)]
    pub fn fe(&mut self) -> FE_W {
        FE_W { w: self }
    }
    #[doc = "Bits 4:6 - Output Channel Select for ETLx Output Trigger Pulse"]
    #[inline(always)]
    pub fn ocs(&mut self) -> OCS_W {
        OCS_W { w: self }
    }
    #[doc = "Bit 7 - Status Flag for ETLx"]
    #[inline(always)]
    pub fn fl(&mut self) -> FL_W {
        FL_W { w: self }
    }
    #[doc = "Bits 8:9 - Input Source Select for ERSx"]
    #[inline(always)]
    pub fn ss(&mut self) -> SS_W {
        SS_W { w: self }
    }
    #[doc = "Bit 10 - Input A Negation Select for ERSx"]
    #[inline(always)]
    pub fn na(&mut self) -> NA_W {
        NA_W { w: self }
    }
    #[doc = "Bit 11 - Input B Negation Select for ERSx"]
    #[inline(always)]
    pub fn nb(&mut self) -> NB_W {
        NB_W { w: self }
    }
}
