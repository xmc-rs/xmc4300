#[doc = "Reader of register ATIM0"]
pub type R = crate::R<u32, super::ATIM0>;
#[doc = "Writer for register ATIM0"]
pub type W = crate::W<u32, super::ATIM0>;
#[doc = "Register ATIM0 `reset()`'s with value 0"]
impl crate::ResetValue for super::ATIM0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ASE`"]
pub type ASE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ASE`"]
pub struct ASE_W<'a> {
    w: &'a mut W,
}
impl<'a> ASE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | ((value as u32) & 0x3f);
        self.w
    }
}
#[doc = "Reader of field `AMI`"]
pub type AMI_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `AMI`"]
pub struct AMI_W<'a> {
    w: &'a mut W,
}
impl<'a> AMI_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 8)) | (((value as u32) & 0x3f) << 8);
        self.w
    }
}
#[doc = "Reader of field `AHO`"]
pub type AHO_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `AHO`"]
pub struct AHO_W<'a> {
    w: &'a mut W,
}
impl<'a> AHO_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 16)) | (((value as u32) & 0x1f) << 16);
        self.w
    }
}
#[doc = "Reader of field `ADA`"]
pub type ADA_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ADA`"]
pub struct ADA_W<'a> {
    w: &'a mut W,
}
impl<'a> ADA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 24)) | (((value as u32) & 0x1f) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5 - Alarm Seconds Compare Value"]
    #[inline(always)]
    pub fn ase(&self) -> ASE_R {
        ASE_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:13 - Alarm Minutes Compare Value"]
    #[inline(always)]
    pub fn ami(&self) -> AMI_R {
        AMI_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:20 - Alarm Hours Compare Value"]
    #[inline(always)]
    pub fn aho(&self) -> AHO_R {
        AHO_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28 - Alarm Days Compare Value"]
    #[inline(always)]
    pub fn ada(&self) -> ADA_R {
        ADA_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Alarm Seconds Compare Value"]
    #[inline(always)]
    pub fn ase(&mut self) -> ASE_W {
        ASE_W { w: self }
    }
    #[doc = "Bits 8:13 - Alarm Minutes Compare Value"]
    #[inline(always)]
    pub fn ami(&mut self) -> AMI_W {
        AMI_W { w: self }
    }
    #[doc = "Bits 16:20 - Alarm Hours Compare Value"]
    #[inline(always)]
    pub fn aho(&mut self) -> AHO_W {
        AHO_W { w: self }
    }
    #[doc = "Bits 24:28 - Alarm Days Compare Value"]
    #[inline(always)]
    pub fn ada(&mut self) -> ADA_W {
        ADA_W { w: self }
    }
}
