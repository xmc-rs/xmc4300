#[doc = "Reader of register EXOCON[%s]"]
pub type R = crate::R<u32, super::EXOCON>;
#[doc = "Writer for register EXOCON[%s]"]
pub type W = crate::W<u32, super::EXOCON>;
#[doc = "Register EXOCON[%s] `reset()`'s with value 0x08"]
impl crate::ResetValue for super::EXOCON {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x08
    }
}
#[doc = "Internal Trigger Source Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ISS_A {
    #[doc = "0: The peripheral trigger function is disabled"]
    VALUE1,
    #[doc = "1: Input ERU_OGUy1 is selected"]
    VALUE2,
    #[doc = "2: Input ERU_OGUy2 is selected"]
    VALUE3,
    #[doc = "3: Input ERU_OGUy3 is selected"]
    VALUE4,
}
impl From<ISS_A> for u8 {
    #[inline(always)]
    fn from(variant: ISS_A) -> Self {
        match variant {
            ISS_A::VALUE1 => 0,
            ISS_A::VALUE2 => 1,
            ISS_A::VALUE3 => 2,
            ISS_A::VALUE4 => 3,
        }
    }
}
#[doc = "Reader of field `ISS`"]
pub type ISS_R = crate::R<u8, ISS_A>;
impl ISS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ISS_A {
        match self.bits {
            0 => ISS_A::VALUE1,
            1 => ISS_A::VALUE2,
            2 => ISS_A::VALUE3,
            3 => ISS_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == ISS_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == ISS_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == ISS_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == ISS_A::VALUE4
    }
}
#[doc = "Write proxy for field `ISS`"]
pub struct ISS_W<'a> {
    w: &'a mut W,
}
impl<'a> ISS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ISS_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "The peripheral trigger function is disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(ISS_A::VALUE1)
    }
    #[doc = "Input ERU_OGUy1 is selected"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(ISS_A::VALUE2)
    }
    #[doc = "Input ERU_OGUy2 is selected"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(ISS_A::VALUE3)
    }
    #[doc = "Input ERU_OGUy3 is selected"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(ISS_A::VALUE4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Gating Event Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GEEN_A {
    #[doc = "0: The event detection is disabled"]
    VALUE1,
    #[doc = "1: The event detection is enabled"]
    VALUE2,
}
impl From<GEEN_A> for bool {
    #[inline(always)]
    fn from(variant: GEEN_A) -> Self {
        match variant {
            GEEN_A::VALUE1 => false,
            GEEN_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `GEEN`"]
pub type GEEN_R = crate::R<bool, GEEN_A>;
impl GEEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GEEN_A {
        match self.bits {
            false => GEEN_A::VALUE1,
            true => GEEN_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == GEEN_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == GEEN_A::VALUE2
    }
}
#[doc = "Write proxy for field `GEEN`"]
pub struct GEEN_W<'a> {
    w: &'a mut W,
}
impl<'a> GEEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GEEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The event detection is disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(GEEN_A::VALUE1)
    }
    #[doc = "The event detection is enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(GEEN_A::VALUE2)
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
#[doc = "Pattern Detection Result Flag\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDR_A {
    #[doc = "0: A pattern miss is detected"]
    VALUE1,
    #[doc = "1: A pattern match is detected"]
    VALUE2,
}
impl From<PDR_A> for bool {
    #[inline(always)]
    fn from(variant: PDR_A) -> Self {
        match variant {
            PDR_A::VALUE1 => false,
            PDR_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `PDR`"]
pub type PDR_R = crate::R<bool, PDR_A>;
impl PDR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDR_A {
        match self.bits {
            false => PDR_A::VALUE1,
            true => PDR_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PDR_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PDR_A::VALUE2
    }
}
#[doc = "Gating Selection for Pattern Detection Result\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GP_A {
    #[doc = "0: ERU_GOUTy is always disabled and ERU_IOUTy can not be activated"]
    VALUE1,
    #[doc = "1: ERU_GOUTy is always enabled and ERU_IOUTy becomes activated with each activation of ERU_TOUTy"]
    VALUE2,
    #[doc = "2: ERU_GOUTy is equal to ERU_PDOUTy and ERU_IOUTy becomes activated with an activation of ERU_TOUTy while the desired pattern is detected (pattern match PDR = 1)"]
    VALUE3,
    #[doc = "3: ERU_GOUTy is inverted to ERU_PDOUTy and ERU_IOUTy becomes activated with an activation of ERU_TOUTy while the desired pattern is not detected (pattern miss PDR = 0)"]
    VALUE4,
}
impl From<GP_A> for u8 {
    #[inline(always)]
    fn from(variant: GP_A) -> Self {
        match variant {
            GP_A::VALUE1 => 0,
            GP_A::VALUE2 => 1,
            GP_A::VALUE3 => 2,
            GP_A::VALUE4 => 3,
        }
    }
}
#[doc = "Reader of field `GP`"]
pub type GP_R = crate::R<u8, GP_A>;
impl GP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GP_A {
        match self.bits {
            0 => GP_A::VALUE1,
            1 => GP_A::VALUE2,
            2 => GP_A::VALUE3,
            3 => GP_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == GP_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == GP_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == GP_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == GP_A::VALUE4
    }
}
#[doc = "Write proxy for field `GP`"]
pub struct GP_W<'a> {
    w: &'a mut W,
}
impl<'a> GP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GP_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "ERU_GOUTy is always disabled and ERU_IOUTy can not be activated"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(GP_A::VALUE1)
    }
    #[doc = "ERU_GOUTy is always enabled and ERU_IOUTy becomes activated with each activation of ERU_TOUTy"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(GP_A::VALUE2)
    }
    #[doc = "ERU_GOUTy is equal to ERU_PDOUTy and ERU_IOUTy becomes activated with an activation of ERU_TOUTy while the desired pattern is detected (pattern match PDR = 1)"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(GP_A::VALUE3)
    }
    #[doc = "ERU_GOUTy is inverted to ERU_PDOUTy and ERU_IOUTy becomes activated with an activation of ERU_TOUTy while the desired pattern is not detected (pattern miss PDR = 0)"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(GP_A::VALUE4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "Pattern Detection Enable for ETL0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IPEN0_A {
    #[doc = "0: Flag EXICONx.FL is excluded from the pattern detection"]
    VALUE1,
    #[doc = "1: Flag EXICONx.FL is included in the pattern detection"]
    VALUE2,
}
impl From<IPEN0_A> for bool {
    #[inline(always)]
    fn from(variant: IPEN0_A) -> Self {
        match variant {
            IPEN0_A::VALUE1 => false,
            IPEN0_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `IPEN0`"]
pub type IPEN0_R = crate::R<bool, IPEN0_A>;
impl IPEN0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IPEN0_A {
        match self.bits {
            false => IPEN0_A::VALUE1,
            true => IPEN0_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == IPEN0_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == IPEN0_A::VALUE2
    }
}
#[doc = "Write proxy for field `IPEN0`"]
pub struct IPEN0_W<'a> {
    w: &'a mut W,
}
impl<'a> IPEN0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IPEN0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Flag EXICONx.FL is excluded from the pattern detection"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(IPEN0_A::VALUE1)
    }
    #[doc = "Flag EXICONx.FL is included in the pattern detection"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(IPEN0_A::VALUE2)
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
#[doc = "Pattern Detection Enable for ETL1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IPEN1_A {
    #[doc = "0: Flag EXICONx.FL is excluded from the pattern detection"]
    VALUE1,
    #[doc = "1: Flag EXICONx.FL is included in the pattern detection"]
    VALUE2,
}
impl From<IPEN1_A> for bool {
    #[inline(always)]
    fn from(variant: IPEN1_A) -> Self {
        match variant {
            IPEN1_A::VALUE1 => false,
            IPEN1_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `IPEN1`"]
pub type IPEN1_R = crate::R<bool, IPEN1_A>;
impl IPEN1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IPEN1_A {
        match self.bits {
            false => IPEN1_A::VALUE1,
            true => IPEN1_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == IPEN1_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == IPEN1_A::VALUE2
    }
}
#[doc = "Write proxy for field `IPEN1`"]
pub struct IPEN1_W<'a> {
    w: &'a mut W,
}
impl<'a> IPEN1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IPEN1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Flag EXICONx.FL is excluded from the pattern detection"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(IPEN1_A::VALUE1)
    }
    #[doc = "Flag EXICONx.FL is included in the pattern detection"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(IPEN1_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "Pattern Detection Enable for ETL2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IPEN2_A {
    #[doc = "0: Flag EXICONx.FL is excluded from the pattern detection"]
    VALUE1,
    #[doc = "1: Flag EXICONx.FL is included in the pattern detection"]
    VALUE2,
}
impl From<IPEN2_A> for bool {
    #[inline(always)]
    fn from(variant: IPEN2_A) -> Self {
        match variant {
            IPEN2_A::VALUE1 => false,
            IPEN2_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `IPEN2`"]
pub type IPEN2_R = crate::R<bool, IPEN2_A>;
impl IPEN2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IPEN2_A {
        match self.bits {
            false => IPEN2_A::VALUE1,
            true => IPEN2_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == IPEN2_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == IPEN2_A::VALUE2
    }
}
#[doc = "Write proxy for field `IPEN2`"]
pub struct IPEN2_W<'a> {
    w: &'a mut W,
}
impl<'a> IPEN2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IPEN2_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Flag EXICONx.FL is excluded from the pattern detection"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(IPEN2_A::VALUE1)
    }
    #[doc = "Flag EXICONx.FL is included in the pattern detection"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(IPEN2_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "Pattern Detection Enable for ETL3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IPEN3_A {
    #[doc = "0: Flag EXICONx.FL is excluded from the pattern detection"]
    VALUE1,
    #[doc = "1: Flag EXICONx.FL is included in the pattern detection"]
    VALUE2,
}
impl From<IPEN3_A> for bool {
    #[inline(always)]
    fn from(variant: IPEN3_A) -> Self {
        match variant {
            IPEN3_A::VALUE1 => false,
            IPEN3_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `IPEN3`"]
pub type IPEN3_R = crate::R<bool, IPEN3_A>;
impl IPEN3_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IPEN3_A {
        match self.bits {
            false => IPEN3_A::VALUE1,
            true => IPEN3_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == IPEN3_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == IPEN3_A::VALUE2
    }
}
#[doc = "Write proxy for field `IPEN3`"]
pub struct IPEN3_W<'a> {
    w: &'a mut W,
}
impl<'a> IPEN3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IPEN3_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Flag EXICONx.FL is excluded from the pattern detection"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(IPEN3_A::VALUE1)
    }
    #[doc = "Flag EXICONx.FL is included in the pattern detection"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(IPEN3_A::VALUE2)
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
impl R {
    #[doc = "Bits 0:1 - Internal Trigger Source Selection"]
    #[inline(always)]
    pub fn iss(&self) -> ISS_R {
        ISS_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bit 2 - Gating Event Enable"]
    #[inline(always)]
    pub fn geen(&self) -> GEEN_R {
        GEEN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Pattern Detection Result Flag"]
    #[inline(always)]
    pub fn pdr(&self) -> PDR_R {
        PDR_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 4:5 - Gating Selection for Pattern Detection Result"]
    #[inline(always)]
    pub fn gp(&self) -> GP_R {
        GP_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bit 12 - Pattern Detection Enable for ETL0"]
    #[inline(always)]
    pub fn ipen0(&self) -> IPEN0_R {
        IPEN0_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Pattern Detection Enable for ETL1"]
    #[inline(always)]
    pub fn ipen1(&self) -> IPEN1_R {
        IPEN1_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Pattern Detection Enable for ETL2"]
    #[inline(always)]
    pub fn ipen2(&self) -> IPEN2_R {
        IPEN2_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Pattern Detection Enable for ETL3"]
    #[inline(always)]
    pub fn ipen3(&self) -> IPEN3_R {
        IPEN3_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Internal Trigger Source Selection"]
    #[inline(always)]
    pub fn iss(&mut self) -> ISS_W {
        ISS_W { w: self }
    }
    #[doc = "Bit 2 - Gating Event Enable"]
    #[inline(always)]
    pub fn geen(&mut self) -> GEEN_W {
        GEEN_W { w: self }
    }
    #[doc = "Bits 4:5 - Gating Selection for Pattern Detection Result"]
    #[inline(always)]
    pub fn gp(&mut self) -> GP_W {
        GP_W { w: self }
    }
    #[doc = "Bit 12 - Pattern Detection Enable for ETL0"]
    #[inline(always)]
    pub fn ipen0(&mut self) -> IPEN0_W {
        IPEN0_W { w: self }
    }
    #[doc = "Bit 13 - Pattern Detection Enable for ETL1"]
    #[inline(always)]
    pub fn ipen1(&mut self) -> IPEN1_W {
        IPEN1_W { w: self }
    }
    #[doc = "Bit 14 - Pattern Detection Enable for ETL2"]
    #[inline(always)]
    pub fn ipen2(&mut self) -> IPEN2_W {
        IPEN2_W { w: self }
    }
    #[doc = "Bit 15 - Pattern Detection Enable for ETL3"]
    #[inline(always)]
    pub fn ipen3(&mut self) -> IPEN3_W {
        IPEN3_W { w: self }
    }
}
