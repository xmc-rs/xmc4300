#[doc = "Reader of register TSVAL"]
pub type R = crate::R<u32, super::TSVAL>;
#[doc = "Writer for register TSVAL"]
pub type W = crate::W<u32, super::TSVAL>;
#[doc = "Register TSVAL `reset()`'s with value 0"]
impl crate::ResetValue for super::TSVAL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TSCTRVALR`"]
pub type TSCTRVALR_R = crate::R<u16, u16>;
#[doc = "Reader of field `TSCTRVAL`"]
pub type TSCTRVAL_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `TSCTRVAL`"]
pub struct TSCTRVAL_W<'a> {
    w: &'a mut W,
}
impl<'a> TSCTRVAL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Shadow TS-Counter (Read)"]
    #[inline(always)]
    pub fn tsctrvalr(&self) -> TSCTRVALR_R {
        TSCTRVALR_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - TS-Counter Value"]
    #[inline(always)]
    pub fn tsctrval(&self) -> TSCTRVAL_R {
        TSCTRVAL_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:31 - TS-Counter Value"]
    #[inline(always)]
    pub fn tsctrval(&mut self) -> TSCTRVAL_W {
        TSCTRVAL_W { w: self }
    }
}
