#[doc = "Reader of register BLOCK_COUNT"]
pub type R = crate::R<u16, super::BLOCK_COUNT>;
#[doc = "Writer for register BLOCK_COUNT"]
pub type W = crate::W<u16, super::BLOCK_COUNT>;
#[doc = "Register BLOCK_COUNT `reset()`'s with value 0"]
impl crate::ResetValue for super::BLOCK_COUNT {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `BLOCK_COUNT`"]
pub type BLOCK_COUNT_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `BLOCK_COUNT`"]
pub struct BLOCK_COUNT_W<'a> {
    w: &'a mut W,
}
impl<'a> BLOCK_COUNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u16) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Blocks Count for Current Transfer"]
    #[inline(always)]
    pub fn block_count(&self) -> BLOCK_COUNT_R {
        BLOCK_COUNT_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Blocks Count for Current Transfer"]
    #[inline(always)]
    pub fn block_count(&mut self) -> BLOCK_COUNT_W {
        BLOCK_COUNT_W { w: self }
    }
}
