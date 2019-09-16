#[doc = "Writer for register IN[%s]"]
pub type W = crate::W<u32, super::IN>;
#[doc = "Register IN[%s] `reset()`'s with value 0"]
impl crate::ResetValue for super::IN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `TDATA`"]
pub struct TDATA_W<'a> {
    w: &'a mut W,
}
impl<'a> TDATA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:15 - Transmit Data"]
    #[inline(always)]
    pub fn tdata(&mut self) -> TDATA_W {
        TDATA_W { w: self }
    }
}
