#[doc = "Reader of register CHC"]
pub type R = crate::R<u32, super::CHC>;
#[doc = "Writer for register CHC"]
pub type W = crate::W<u32, super::CHC>;
#[doc = "Register CHC `reset()`'s with value 0"]
impl crate::ResetValue for super::CHC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Asymmetric PWM mode Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ASE_A {
    #[doc = "0: Asymmetric PWM is disabled"]
    VALUE1 = 0,
    #[doc = "1: Asymmetric PWM is enabled"]
    VALUE2 = 1,
}
impl From<ASE_A> for bool {
    #[inline(always)]
    fn from(variant: ASE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ASE`"]
pub type ASE_R = crate::R<bool, ASE_A>;
impl ASE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ASE_A {
        match self.bits {
            false => ASE_A::VALUE1,
            true => ASE_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == ASE_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == ASE_A::VALUE2
    }
}
#[doc = "Write proxy for field `ASE`"]
pub struct ASE_W<'a> {
    w: &'a mut W,
}
impl<'a> ASE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ASE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Asymmetric PWM is disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(ASE_A::VALUE1)
    }
    #[doc = "Asymmetric PWM is enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(ASE_A::VALUE2)
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
#[doc = "Output selector for CCU8x.OUTy0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OCS1_A {
    #[doc = "0: CC8yST1 signal path is connected to the CCU8x.OUTy0"]
    VALUE1 = 0,
    #[doc = "1: Inverted CC8yST1 signal path is connected to the CCU8x.OUTy0"]
    VALUE2 = 1,
}
impl From<OCS1_A> for bool {
    #[inline(always)]
    fn from(variant: OCS1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `OCS1`"]
pub type OCS1_R = crate::R<bool, OCS1_A>;
impl OCS1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OCS1_A {
        match self.bits {
            false => OCS1_A::VALUE1,
            true => OCS1_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == OCS1_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == OCS1_A::VALUE2
    }
}
#[doc = "Write proxy for field `OCS1`"]
pub struct OCS1_W<'a> {
    w: &'a mut W,
}
impl<'a> OCS1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OCS1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "CC8yST1 signal path is connected to the CCU8x.OUTy0"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(OCS1_A::VALUE1)
    }
    #[doc = "Inverted CC8yST1 signal path is connected to the CCU8x.OUTy0"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(OCS1_A::VALUE2)
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
#[doc = "Output selector for CCU8x.OUTy1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OCS2_A {
    #[doc = "0: Inverted CC8yST1 signal path is connected to the CCU8x.OUTy1"]
    VALUE1 = 0,
    #[doc = "1: CC8yST1 signal path is connected to the CCU8x.OUTy1"]
    VALUE2 = 1,
}
impl From<OCS2_A> for bool {
    #[inline(always)]
    fn from(variant: OCS2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `OCS2`"]
pub type OCS2_R = crate::R<bool, OCS2_A>;
impl OCS2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OCS2_A {
        match self.bits {
            false => OCS2_A::VALUE1,
            true => OCS2_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == OCS2_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == OCS2_A::VALUE2
    }
}
#[doc = "Write proxy for field `OCS2`"]
pub struct OCS2_W<'a> {
    w: &'a mut W,
}
impl<'a> OCS2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OCS2_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Inverted CC8yST1 signal path is connected to the CCU8x.OUTy1"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(OCS2_A::VALUE1)
    }
    #[doc = "CC8yST1 signal path is connected to the CCU8x.OUTy1"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(OCS2_A::VALUE2)
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
#[doc = "Output selector for CCU8x.OUTy2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OCS3_A {
    #[doc = "0: CC8yST2 signal path is connected to the CCU8x.OUTy2"]
    VALUE1 = 0,
    #[doc = "1: Inverted CCST2 signal path is connected to the CCU8x.OUTy2"]
    VALUE2 = 1,
}
impl From<OCS3_A> for bool {
    #[inline(always)]
    fn from(variant: OCS3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `OCS3`"]
pub type OCS3_R = crate::R<bool, OCS3_A>;
impl OCS3_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OCS3_A {
        match self.bits {
            false => OCS3_A::VALUE1,
            true => OCS3_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == OCS3_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == OCS3_A::VALUE2
    }
}
#[doc = "Write proxy for field `OCS3`"]
pub struct OCS3_W<'a> {
    w: &'a mut W,
}
impl<'a> OCS3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OCS3_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "CC8yST2 signal path is connected to the CCU8x.OUTy2"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(OCS3_A::VALUE1)
    }
    #[doc = "Inverted CCST2 signal path is connected to the CCU8x.OUTy2"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(OCS3_A::VALUE2)
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
#[doc = "Output selector for CCU8x.OUTy3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OCS4_A {
    #[doc = "0: Inverted CC8yST2 signal path is connected to the CCU8x.OUTy3"]
    VALUE1 = 0,
    #[doc = "1: CC8yST2 signal path is connected to the CCU8x.OUTy3"]
    VALUE2 = 1,
}
impl From<OCS4_A> for bool {
    #[inline(always)]
    fn from(variant: OCS4_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `OCS4`"]
pub type OCS4_R = crate::R<bool, OCS4_A>;
impl OCS4_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OCS4_A {
        match self.bits {
            false => OCS4_A::VALUE1,
            true => OCS4_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == OCS4_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == OCS4_A::VALUE2
    }
}
#[doc = "Write proxy for field `OCS4`"]
pub struct OCS4_W<'a> {
    w: &'a mut W,
}
impl<'a> OCS4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OCS4_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Inverted CC8yST2 signal path is connected to the CCU8x.OUTy3"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(OCS4_A::VALUE1)
    }
    #[doc = "CC8yST2 signal path is connected to the CCU8x.OUTy3"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(OCS4_A::VALUE2)
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
impl R {
    #[doc = "Bit 0 - Asymmetric PWM mode Enable"]
    #[inline(always)]
    pub fn ase(&self) -> ASE_R {
        ASE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Output selector for CCU8x.OUTy0"]
    #[inline(always)]
    pub fn ocs1(&self) -> OCS1_R {
        OCS1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Output selector for CCU8x.OUTy1"]
    #[inline(always)]
    pub fn ocs2(&self) -> OCS2_R {
        OCS2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Output selector for CCU8x.OUTy2"]
    #[inline(always)]
    pub fn ocs3(&self) -> OCS3_R {
        OCS3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Output selector for CCU8x.OUTy3"]
    #[inline(always)]
    pub fn ocs4(&self) -> OCS4_R {
        OCS4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Asymmetric PWM mode Enable"]
    #[inline(always)]
    pub fn ase(&mut self) -> ASE_W {
        ASE_W { w: self }
    }
    #[doc = "Bit 1 - Output selector for CCU8x.OUTy0"]
    #[inline(always)]
    pub fn ocs1(&mut self) -> OCS1_W {
        OCS1_W { w: self }
    }
    #[doc = "Bit 2 - Output selector for CCU8x.OUTy1"]
    #[inline(always)]
    pub fn ocs2(&mut self) -> OCS2_W {
        OCS2_W { w: self }
    }
    #[doc = "Bit 3 - Output selector for CCU8x.OUTy2"]
    #[inline(always)]
    pub fn ocs3(&mut self) -> OCS3_W {
        OCS3_W { w: self }
    }
    #[doc = "Bit 4 - Output selector for CCU8x.OUTy3"]
    #[inline(always)]
    pub fn ocs4(&mut self) -> OCS4_W {
        OCS4_W { w: self }
    }
}
