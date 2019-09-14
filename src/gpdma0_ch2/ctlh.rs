#[doc = "Reader of register CTLH"]
pub type R = crate::R<u32, super::CTLH>;
#[doc = "Writer for register CTLH"]
pub type W = crate::W<u32, super::CTLH>;
#[doc = "Register CTLH `reset()`'s with value 0x02"]
impl crate::ResetValue for super::CTLH {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x02
    }
}
#[doc = "Reader of field `DONE`"]
pub type DONE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DONE`"]
pub struct DONE_W<'a> {
    w: &'a mut W,
}
impl<'a> DONE_W<'a> {
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
#[doc = "Reader of field `BLOCK_TS`"]
pub type BLOCK_TS_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `BLOCK_TS`"]
pub struct BLOCK_TS_W<'a> {
    w: &'a mut W,
}
impl<'a> BLOCK_TS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | ((value as u32) & 0x0fff);
        self.w
    }
}
impl R {
    #[doc = "Bit 12 - Done bit"]
    #[inline(always)]
    pub fn done(&self) -> DONE_R {
        DONE_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bits 0:11 - Block Transfer Size"]
    #[inline(always)]
    pub fn block_ts(&self) -> BLOCK_TS_R {
        BLOCK_TS_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bit 12 - Done bit"]
    #[inline(always)]
    pub fn done(&mut self) -> DONE_W {
        DONE_W { w: self }
    }
    #[doc = "Bits 0:11 - Block Transfer Size"]
    #[inline(always)]
    pub fn block_ts(&mut self) -> BLOCK_TS_W {
        BLOCK_TS_W { w: self }
    }
}
