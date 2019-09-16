#[doc = "Reader of register STS"]
pub type R = crate::R<u32, super::STS>;
#[doc = "Writer for register STS"]
pub type W = crate::W<u32, super::STS>;
#[doc = "Register STS `reset()`'s with value 0"]
impl crate::ResetValue for super::STS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CMF`"]
pub type CMF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CMF`"]
pub struct CMF_W<'a> {
    w: &'a mut W,
}
impl<'a> CMF_W<'a> {
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
#[doc = "Reader of field `CEF`"]
pub type CEF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CEF`"]
pub struct CEF_W<'a> {
    w: &'a mut W,
}
impl<'a> CEF_W<'a> {
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
#[doc = "Reader of field `LEF`"]
pub type LEF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LEF`"]
pub struct LEF_W<'a> {
    w: &'a mut W,
}
impl<'a> LEF_W<'a> {
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
#[doc = "Reader of field `BEF`"]
pub type BEF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BEF`"]
pub struct BEF_W<'a> {
    w: &'a mut W,
}
impl<'a> BEF_W<'a> {
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
impl R {
    #[doc = "Bit 0 - CRC Mismatch Flag"]
    #[inline(always)]
    pub fn cmf(&self) -> CMF_R {
        CMF_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Configuration Error Flag"]
    #[inline(always)]
    pub fn cef(&self) -> CEF_R {
        CEF_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Length Error Flag"]
    #[inline(always)]
    pub fn lef(&self) -> LEF_R {
        LEF_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Bus Error Flag"]
    #[inline(always)]
    pub fn bef(&self) -> BEF_R {
        BEF_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CRC Mismatch Flag"]
    #[inline(always)]
    pub fn cmf(&mut self) -> CMF_W {
        CMF_W { w: self }
    }
    #[doc = "Bit 1 - Configuration Error Flag"]
    #[inline(always)]
    pub fn cef(&mut self) -> CEF_W {
        CEF_W { w: self }
    }
    #[doc = "Bit 2 - Length Error Flag"]
    #[inline(always)]
    pub fn lef(&mut self) -> LEF_W {
        LEF_W { w: self }
    }
    #[doc = "Bit 3 - Bus Error Flag"]
    #[inline(always)]
    pub fn bef(&mut self) -> BEF_W {
        BEF_W { w: self }
    }
}
