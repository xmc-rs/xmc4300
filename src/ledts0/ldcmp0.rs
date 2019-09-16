#[doc = "Reader of register LDCMP0"]
pub type R = crate::R<u32, super::LDCMP0>;
#[doc = "Writer for register LDCMP0"]
pub type W = crate::W<u32, super::LDCMP0>;
#[doc = "Register LDCMP0 `reset()`'s with value 0"]
impl crate::ResetValue for super::LDCMP0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CMP_LD0`"]
pub type CMP_LD0_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CMP_LD0`"]
pub struct CMP_LD0_W<'a> {
    w: &'a mut W,
}
impl<'a> CMP_LD0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `CMP_LD1`"]
pub type CMP_LD1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CMP_LD1`"]
pub struct CMP_LD1_W<'a> {
    w: &'a mut W,
}
impl<'a> CMP_LD1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `CMP_LD2`"]
pub type CMP_LD2_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CMP_LD2`"]
pub struct CMP_LD2_W<'a> {
    w: &'a mut W,
}
impl<'a> CMP_LD2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "Reader of field `CMP_LD3`"]
pub type CMP_LD3_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CMP_LD3`"]
pub struct CMP_LD3_W<'a> {
    w: &'a mut W,
}
impl<'a> CMP_LD3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | (((value as u32) & 0xff) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Compare Value for LED COL\\[x\\]"]
    #[inline(always)]
    pub fn cmp_ld0(&self) -> CMP_LD0_R {
        CMP_LD0_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Compare Value for LED COL\\[x\\]"]
    #[inline(always)]
    pub fn cmp_ld1(&self) -> CMP_LD1_R {
        CMP_LD1_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Compare Value for LED COL\\[x\\]"]
    #[inline(always)]
    pub fn cmp_ld2(&self) -> CMP_LD2_R {
        CMP_LD2_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Compare Value for LED COL\\[x\\]"]
    #[inline(always)]
    pub fn cmp_ld3(&self) -> CMP_LD3_R {
        CMP_LD3_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Compare Value for LED COL\\[x\\]"]
    #[inline(always)]
    pub fn cmp_ld0(&mut self) -> CMP_LD0_W {
        CMP_LD0_W { w: self }
    }
    #[doc = "Bits 8:15 - Compare Value for LED COL\\[x\\]"]
    #[inline(always)]
    pub fn cmp_ld1(&mut self) -> CMP_LD1_W {
        CMP_LD1_W { w: self }
    }
    #[doc = "Bits 16:23 - Compare Value for LED COL\\[x\\]"]
    #[inline(always)]
    pub fn cmp_ld2(&mut self) -> CMP_LD2_W {
        CMP_LD2_W { w: self }
    }
    #[doc = "Bits 24:31 - Compare Value for LED COL\\[x\\]"]
    #[inline(always)]
    pub fn cmp_ld3(&mut self) -> CMP_LD3_W {
        CMP_LD3_W { w: self }
    }
}
