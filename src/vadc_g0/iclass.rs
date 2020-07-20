#[doc = "Reader of register ICLASS[%s]"]
pub type R = crate::R<u32, super::ICLASS>;
#[doc = "Writer for register ICLASS[%s]"]
pub type W = crate::W<u32, super::ICLASS>;
#[doc = "Register ICLASS[%s]
`reset()`'s with value 0"]
impl crate::ResetValue for super::ICLASS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `STCS`"]
pub type STCS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `STCS`"]
pub struct STCS_W<'a> {
    w: &'a mut W,
}
impl<'a> STCS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u32) & 0x1f);
        self.w
    }
}
#[doc = "Conversion Mode for Standard Conversions\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CMS_A {
    #[doc = "0: 12-bit conversion"]
    VALUE1 = 0,
    #[doc = "1: 10-bit conversion"]
    VALUE2 = 1,
    #[doc = "2: 8-bit conversion"]
    VALUE3 = 2,
    #[doc = "5: 10-bit fast compare mode"]
    VALUE6 = 5,
}
impl From<CMS_A> for u8 {
    #[inline(always)]
    fn from(variant: CMS_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `CMS`"]
pub type CMS_R = crate::R<u8, CMS_A>;
impl CMS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, CMS_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(CMS_A::VALUE1),
            1 => Val(CMS_A::VALUE2),
            2 => Val(CMS_A::VALUE3),
            5 => Val(CMS_A::VALUE6),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CMS_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CMS_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == CMS_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE6`"]
    #[inline(always)]
    pub fn is_value6(&self) -> bool {
        *self == CMS_A::VALUE6
    }
}
#[doc = "Write proxy for field `CMS`"]
pub struct CMS_W<'a> {
    w: &'a mut W,
}
impl<'a> CMS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CMS_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "12-bit conversion"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CMS_A::VALUE1)
    }
    #[doc = "10-bit conversion"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CMS_A::VALUE2)
    }
    #[doc = "8-bit conversion"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(CMS_A::VALUE3)
    }
    #[doc = "10-bit fast compare mode"]
    #[inline(always)]
    pub fn value6(self) -> &'a mut W {
        self.variant(CMS_A::VALUE6)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | (((value as u32) & 0x07) << 8);
        self.w
    }
}
#[doc = "Reader of field `STCE`"]
pub type STCE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `STCE`"]
pub struct STCE_W<'a> {
    w: &'a mut W,
}
impl<'a> STCE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 16)) | (((value as u32) & 0x1f) << 16);
        self.w
    }
}
#[doc = "Conversion Mode for EMUX Conversions\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CME_A {
    #[doc = "0: 12-bit conversion"]
    VALUE1 = 0,
    #[doc = "1: 10-bit conversion"]
    VALUE2 = 1,
    #[doc = "2: 8-bit conversion"]
    VALUE3 = 2,
    #[doc = "5: 10-bit fast compare mode"]
    VALUE6 = 5,
}
impl From<CME_A> for u8 {
    #[inline(always)]
    fn from(variant: CME_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `CME`"]
pub type CME_R = crate::R<u8, CME_A>;
impl CME_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, CME_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(CME_A::VALUE1),
            1 => Val(CME_A::VALUE2),
            2 => Val(CME_A::VALUE3),
            5 => Val(CME_A::VALUE6),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CME_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CME_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == CME_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE6`"]
    #[inline(always)]
    pub fn is_value6(&self) -> bool {
        *self == CME_A::VALUE6
    }
}
#[doc = "Write proxy for field `CME`"]
pub struct CME_W<'a> {
    w: &'a mut W,
}
impl<'a> CME_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CME_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "12-bit conversion"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CME_A::VALUE1)
    }
    #[doc = "10-bit conversion"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CME_A::VALUE2)
    }
    #[doc = "8-bit conversion"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(CME_A::VALUE3)
    }
    #[doc = "10-bit fast compare mode"]
    #[inline(always)]
    pub fn value6(self) -> &'a mut W {
        self.variant(CME_A::VALUE6)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 24)) | (((value as u32) & 0x07) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:4 - Sample Time Control for Standard Conversions"]
    #[inline(always)]
    pub fn stcs(&self) -> STCS_R {
        STCS_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:10 - Conversion Mode for Standard Conversions"]
    #[inline(always)]
    pub fn cms(&self) -> CMS_R {
        CMS_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bits 16:20 - Sample Time Control for EMUX Conversions"]
    #[inline(always)]
    pub fn stce(&self) -> STCE_R {
        STCE_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:26 - Conversion Mode for EMUX Conversions"]
    #[inline(always)]
    pub fn cme(&self) -> CME_R {
        CME_R::new(((self.bits >> 24) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Sample Time Control for Standard Conversions"]
    #[inline(always)]
    pub fn stcs(&mut self) -> STCS_W {
        STCS_W { w: self }
    }
    #[doc = "Bits 8:10 - Conversion Mode for Standard Conversions"]
    #[inline(always)]
    pub fn cms(&mut self) -> CMS_W {
        CMS_W { w: self }
    }
    #[doc = "Bits 16:20 - Sample Time Control for EMUX Conversions"]
    #[inline(always)]
    pub fn stce(&mut self) -> STCE_W {
        STCE_W { w: self }
    }
    #[doc = "Bits 24:26 - Conversion Mode for EMUX Conversions"]
    #[inline(always)]
    pub fn cme(&mut self) -> CME_W {
        CME_W { w: self }
    }
}
