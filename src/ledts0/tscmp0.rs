#[doc = "Reader of register TSCMP0"]
pub type R = crate::R<u32, super::TSCMP0>;
#[doc = "Writer for register TSCMP0"]
pub type W = crate::W<u32, super::TSCMP0>;
#[doc = "Register TSCMP0 `reset()`'s with value 0"]
impl crate::ResetValue for super::TSCMP0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CMP_TS0`"]
pub type CMP_TS0_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CMP_TS0`"]
pub struct CMP_TS0_W<'a> {
    w: &'a mut W,
}
impl<'a> CMP_TS0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `CMP_TS1`"]
pub type CMP_TS1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CMP_TS1`"]
pub struct CMP_TS1_W<'a> {
    w: &'a mut W,
}
impl<'a> CMP_TS1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `CMP_TS2`"]
pub type CMP_TS2_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CMP_TS2`"]
pub struct CMP_TS2_W<'a> {
    w: &'a mut W,
}
impl<'a> CMP_TS2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "Reader of field `CMP_TS3`"]
pub type CMP_TS3_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CMP_TS3`"]
pub struct CMP_TS3_W<'a> {
    w: &'a mut W,
}
impl<'a> CMP_TS3_W<'a> {
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
    pub fn cmp_ts0(&self) -> CMP_TS0_R {
        CMP_TS0_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Compare Value for Touch-Sense TSIN\\[x\\]"]
    #[inline(always)]
    pub fn cmp_ts1(&self) -> CMP_TS1_R {
        CMP_TS1_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Compare Value for Touch-Sense TSIN\\[x\\]"]
    #[inline(always)]
    pub fn cmp_ts2(&self) -> CMP_TS2_R {
        CMP_TS2_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Compare Value for Touch-Sense TSIN\\[x\\]"]
    #[inline(always)]
    pub fn cmp_ts3(&self) -> CMP_TS3_R {
        CMP_TS3_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Compare Value for Touch-Sense TSIN\\[x\\]"]
    #[inline(always)]
    pub fn cmp_ts0(&mut self) -> CMP_TS0_W {
        CMP_TS0_W { w: self }
    }
    #[doc = "Bits 8:15 - Compare Value for Touch-Sense TSIN\\[x\\]"]
    #[inline(always)]
    pub fn cmp_ts1(&mut self) -> CMP_TS1_W {
        CMP_TS1_W { w: self }
    }
    #[doc = "Bits 16:23 - Compare Value for Touch-Sense TSIN\\[x\\]"]
    #[inline(always)]
    pub fn cmp_ts2(&mut self) -> CMP_TS2_W {
        CMP_TS2_W { w: self }
    }
    #[doc = "Bits 24:31 - Compare Value for Touch-Sense TSIN\\[x\\]"]
    #[inline(always)]
    pub fn cmp_ts3(&mut self) -> CMP_TS3_W {
        CMP_TS3_W { w: self }
    }
}
