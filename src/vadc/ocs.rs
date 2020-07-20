#[doc = "Reader of register OCS"]
pub type R = crate::R<u32, super::OCS>;
#[doc = "Writer for register OCS"]
pub type W = crate::W<u32, super::OCS>;
#[doc = "Register OCS `reset()`'s with value 0"]
impl crate::ResetValue for super::OCS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Trigger Set for OTGB0/1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TGS_A {
    #[doc = "0: No Trigger Set output"]
    VALUE1 = 0,
    #[doc = "1: Trigger Set 1: TS16_SSIG, input sample signals"]
    VALUE2 = 1,
}
impl From<TGS_A> for u8 {
    #[inline(always)]
    fn from(variant: TGS_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `TGS`"]
pub type TGS_R = crate::R<u8, TGS_A>;
impl TGS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, TGS_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(TGS_A::VALUE1),
            1 => Val(TGS_A::VALUE2),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == TGS_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == TGS_A::VALUE2
    }
}
#[doc = "Write proxy for field `TGS`"]
pub struct TGS_W<'a> {
    w: &'a mut W,
}
impl<'a> TGS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TGS_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "No Trigger Set output"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(TGS_A::VALUE1)
    }
    #[doc = "Trigger Set 1: TS16_SSIG, input sample signals"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(TGS_A::VALUE2)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "OTGB0/1 Bus Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TGB_A {
    #[doc = "0: Trigger Set is output on OTGB0"]
    VALUE1 = 0,
    #[doc = "1: Trigger Set is output on OTGB1"]
    VALUE2 = 1,
}
impl From<TGB_A> for bool {
    #[inline(always)]
    fn from(variant: TGB_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TGB`"]
pub type TGB_R = crate::R<bool, TGB_A>;
impl TGB_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TGB_A {
        match self.bits {
            false => TGB_A::VALUE1,
            true => TGB_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == TGB_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == TGB_A::VALUE2
    }
}
#[doc = "Write proxy for field `TGB`"]
pub struct TGB_W<'a> {
    w: &'a mut W,
}
impl<'a> TGB_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TGB_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Trigger Set is output on OTGB0"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(TGB_A::VALUE1)
    }
    #[doc = "Trigger Set is output on OTGB1"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(TGB_A::VALUE2)
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
#[doc = "Write proxy for field `TG_P`"]
pub struct TG_P_W<'a> {
    w: &'a mut W,
}
impl<'a> TG_P_W<'a> {
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
#[doc = "OCDS Suspend Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SUS_A {
    #[doc = "0: Will not suspend"]
    VALUE1 = 0,
    #[doc = "1: Hard suspend: Clock is switched off immediately."]
    VALUE2 = 1,
    #[doc = "2: Soft suspend mode 0: Stop conversions after the currently running one is completed and its result has been stored. No change for the arbiter."]
    VALUE3 = 2,
    #[doc = "3: Soft suspend mode 1: Stop conversions after the currently running one is completed and its result has been stored. Stop arbiter after the current arbitration round."]
    VALUE4 = 3,
}
impl From<SUS_A> for u8 {
    #[inline(always)]
    fn from(variant: SUS_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SUS`"]
pub type SUS_R = crate::R<u8, SUS_A>;
impl SUS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SUS_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(SUS_A::VALUE1),
            1 => Val(SUS_A::VALUE2),
            2 => Val(SUS_A::VALUE3),
            3 => Val(SUS_A::VALUE4),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SUS_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SUS_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == SUS_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == SUS_A::VALUE4
    }
}
#[doc = "Write proxy for field `SUS`"]
pub struct SUS_W<'a> {
    w: &'a mut W,
}
impl<'a> SUS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SUS_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Will not suspend"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(SUS_A::VALUE1)
    }
    #[doc = "Hard suspend: Clock is switched off immediately."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(SUS_A::VALUE2)
    }
    #[doc = "Soft suspend mode 0: Stop conversions after the currently running one is completed and its result has been stored. No change for the arbiter."]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(SUS_A::VALUE3)
    }
    #[doc = "Soft suspend mode 1: Stop conversions after the currently running one is completed and its result has been stored. Stop arbiter after the current arbitration round."]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(SUS_A::VALUE4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | (((value as u32) & 0x0f) << 24);
        self.w
    }
}
#[doc = "Write proxy for field `SUS_P`"]
pub struct SUS_P_W<'a> {
    w: &'a mut W,
}
impl<'a> SUS_P_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | (((value as u32) & 0x01) << 28);
        self.w
    }
}
#[doc = "Suspend State\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SUSSTA_A {
    #[doc = "0: Module is not (yet) suspended"]
    VALUE1 = 0,
    #[doc = "1: Module is suspended"]
    VALUE2 = 1,
}
impl From<SUSSTA_A> for bool {
    #[inline(always)]
    fn from(variant: SUSSTA_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SUSSTA`"]
pub type SUSSTA_R = crate::R<bool, SUSSTA_A>;
impl SUSSTA_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SUSSTA_A {
        match self.bits {
            false => SUSSTA_A::VALUE1,
            true => SUSSTA_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SUSSTA_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SUSSTA_A::VALUE2
    }
}
impl R {
    #[doc = "Bits 0:1 - Trigger Set for OTGB0/1"]
    #[inline(always)]
    pub fn tgs(&self) -> TGS_R {
        TGS_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bit 2 - OTGB0/1 Bus Select"]
    #[inline(always)]
    pub fn tgb(&self) -> TGB_R {
        TGB_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bits 24:27 - OCDS Suspend Control"]
    #[inline(always)]
    pub fn sus(&self) -> SUS_R {
        SUS_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bit 29 - Suspend State"]
    #[inline(always)]
    pub fn sussta(&self) -> SUSSTA_R {
        SUSSTA_R::new(((self.bits >> 29) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Trigger Set for OTGB0/1"]
    #[inline(always)]
    pub fn tgs(&mut self) -> TGS_W {
        TGS_W { w: self }
    }
    #[doc = "Bit 2 - OTGB0/1 Bus Select"]
    #[inline(always)]
    pub fn tgb(&mut self) -> TGB_W {
        TGB_W { w: self }
    }
    #[doc = "Bit 3 - TGS, TGB Write Protection"]
    #[inline(always)]
    pub fn tg_p(&mut self) -> TG_P_W {
        TG_P_W { w: self }
    }
    #[doc = "Bits 24:27 - OCDS Suspend Control"]
    #[inline(always)]
    pub fn sus(&mut self) -> SUS_W {
        SUS_W { w: self }
    }
    #[doc = "Bit 28 - SUS Write Protection"]
    #[inline(always)]
    pub fn sus_p(&mut self) -> SUS_P_W {
        SUS_P_W { w: self }
    }
}
