#[doc = "Reader of register TSCMP1"]
pub type R = crate::R<u32, super::TSCMP1>;
#[doc = "Writer for register TSCMP1"]
pub type W = crate::W<u32, super::TSCMP1>;
#[doc = "Register TSCMP1 `reset()`'s with value 0"]
impl crate::ResetValue for super::TSCMP1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CMP_TS4`"]
pub type CMP_TS4_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CMP_TS4`"]
pub struct CMP_TS4_W<'a> {
    w: &'a mut W,
}
impl<'a> CMP_TS4_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `CMP_TS5`"]
pub type CMP_TS5_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CMP_TS5`"]
pub struct CMP_TS5_W<'a> {
    w: &'a mut W,
}
impl<'a> CMP_TS5_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `CMP_TS6`"]
pub type CMP_TS6_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CMP_TS6`"]
pub struct CMP_TS6_W<'a> {
    w: &'a mut W,
}
impl<'a> CMP_TS6_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "Reader of field `CMP_TS7`"]
pub type CMP_TS7_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CMP_TS7`"]
pub struct CMP_TS7_W<'a> {
    w: &'a mut W,
}
impl<'a> CMP_TS7_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | (((value as u32) & 0xff) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Compare Value for Touch-Sense TSIN\\[x\\]"]
    #[inline(always)]
    pub fn cmp_ts4(&self) -> CMP_TS4_R {
        CMP_TS4_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Compare Value for Touch-Sense TSIN\\[x\\]"]
    #[inline(always)]
    pub fn cmp_ts5(&self) -> CMP_TS5_R {
        CMP_TS5_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Compare Value for Touch-Sense TSIN\\[x\\]"]
    #[inline(always)]
    pub fn cmp_ts6(&self) -> CMP_TS6_R {
        CMP_TS6_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Compare Value for Touch-Sense TSIN\\[x\\]"]
    #[inline(always)]
    pub fn cmp_ts7(&self) -> CMP_TS7_R {
        CMP_TS7_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Compare Value for Touch-Sense TSIN\\[x\\]"]
    #[inline(always)]
    pub fn cmp_ts4(&mut self) -> CMP_TS4_W {
        CMP_TS4_W { w: self }
    }
    #[doc = "Bits 8:15 - Compare Value for Touch-Sense TSIN\\[x\\]"]
    #[inline(always)]
    pub fn cmp_ts5(&mut self) -> CMP_TS5_W {
        CMP_TS5_W { w: self }
    }
    #[doc = "Bits 16:23 - Compare Value for Touch-Sense TSIN\\[x\\]"]
    #[inline(always)]
    pub fn cmp_ts6(&mut self) -> CMP_TS6_W {
        CMP_TS6_W { w: self }
    }
    #[doc = "Bits 24:31 - Compare Value for Touch-Sense TSIN\\[x\\]"]
    #[inline(always)]
    pub fn cmp_ts7(&mut self) -> CMP_TS7_W {
        CMP_TS7_W { w: self }
    }
}
