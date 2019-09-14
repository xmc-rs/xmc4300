#[doc = "Reader of register DC_SYS_TIME_FIL_DEPTH"]
pub type R = crate::R<u8, super::DC_SYS_TIME_FIL_DEPTH>;
#[doc = "Writer for register DC_SYS_TIME_FIL_DEPTH"]
pub type W = crate::W<u8, super::DC_SYS_TIME_FIL_DEPTH>;
#[doc = "Register DC_SYS_TIME_FIL_DEPTH `reset()`'s with value 0x04"]
impl crate::ResetValue for super::DC_SYS_TIME_FIL_DEPTH {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x04
    }
}
#[doc = "Reader of field `FILTER_DEPTH`"]
pub type FILTER_DEPTH_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `FILTER_DEPTH`"]
pub struct FILTER_DEPTH_W<'a> {
    w: &'a mut W,
}
impl<'a> FILTER_DEPTH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u8) & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Filter depth for averaging the received System Time deviation"]
    #[inline(always)]
    pub fn filter_depth(&self) -> FILTER_DEPTH_R {
        FILTER_DEPTH_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Filter depth for averaging the received System Time deviation"]
    #[inline(always)]
    pub fn filter_depth(&mut self) -> FILTER_DEPTH_W {
        FILTER_DEPTH_W { w: self }
    }
}
