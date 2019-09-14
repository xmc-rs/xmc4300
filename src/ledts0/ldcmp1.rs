#[doc = "Reader of register LDCMP1"]
pub type R = crate::R<u32, super::LDCMP1>;
#[doc = "Writer for register LDCMP1"]
pub type W = crate::W<u32, super::LDCMP1>;
#[doc = "Register LDCMP1 `reset()`'s with value 0"]
impl crate::ResetValue for super::LDCMP1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CMP_LD4`"]
pub type CMP_LD4_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CMP_LD4`"]
pub struct CMP_LD4_W<'a> {
    w: &'a mut W,
}
impl<'a> CMP_LD4_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `CMP_LD5`"]
pub type CMP_LD5_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CMP_LD5`"]
pub struct CMP_LD5_W<'a> {
    w: &'a mut W,
}
impl<'a> CMP_LD5_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `CMP_LD6`"]
pub type CMP_LD6_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CMP_LD6`"]
pub struct CMP_LD6_W<'a> {
    w: &'a mut W,
}
impl<'a> CMP_LD6_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "Reader of field `CMP_LDA_TSCOM`"]
pub type CMP_LDA_TSCOM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CMP_LDA_TSCOM`"]
pub struct CMP_LDA_TSCOM_W<'a> {
    w: &'a mut W,
}
impl<'a> CMP_LDA_TSCOM_W<'a> {
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
    pub fn cmp_ld4(&self) -> CMP_LD4_R {
        CMP_LD4_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Compare Value for LED COL\\[x\\]"]
    #[inline(always)]
    pub fn cmp_ld5(&self) -> CMP_LD5_R {
        CMP_LD5_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Compare Value for LED COL\\[x\\]"]
    #[inline(always)]
    pub fn cmp_ld6(&self) -> CMP_LD6_R {
        CMP_LD6_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Compare Value for LED COLA / Common Compare Value for Touch-sense Pad Turns"]
    #[inline(always)]
    pub fn cmp_lda_tscom(&self) -> CMP_LDA_TSCOM_R {
        CMP_LDA_TSCOM_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Compare Value for LED COL\\[x\\]"]
    #[inline(always)]
    pub fn cmp_ld4(&mut self) -> CMP_LD4_W {
        CMP_LD4_W { w: self }
    }
    #[doc = "Bits 8:15 - Compare Value for LED COL\\[x\\]"]
    #[inline(always)]
    pub fn cmp_ld5(&mut self) -> CMP_LD5_W {
        CMP_LD5_W { w: self }
    }
    #[doc = "Bits 16:23 - Compare Value for LED COL\\[x\\]"]
    #[inline(always)]
    pub fn cmp_ld6(&mut self) -> CMP_LD6_W {
        CMP_LD6_W { w: self }
    }
    #[doc = "Bits 24:31 - Compare Value for LED COLA / Common Compare Value for Touch-sense Pad Turns"]
    #[inline(always)]
    pub fn cmp_lda_tscom(&mut self) -> CMP_LDA_TSCOM_W {
        CMP_LDA_TSCOM_W { w: self }
    }
}
