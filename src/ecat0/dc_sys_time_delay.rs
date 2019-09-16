#[doc = "Reader of register DC_SYS_TIME_DELAY"]
pub type R = crate::R<u32, super::DC_SYS_TIME_DELAY>;
#[doc = "Writer for register DC_SYS_TIME_DELAY"]
pub type W = crate::W<u32, super::DC_SYS_TIME_DELAY>;
#[doc = "Register DC_SYS_TIME_DELAY `reset()`'s with value 0"]
impl crate::ResetValue for super::DC_SYS_TIME_DELAY {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CLK_DELAY`"]
pub type CLK_DELAY_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `CLK_DELAY`"]
pub struct CLK_DELAY_W<'a> {
    w: &'a mut W,
}
impl<'a> CLK_DELAY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Delay between Reference Clock and the ESC"]
    #[inline(always)]
    pub fn clk_delay(&self) -> CLK_DELAY_R {
        CLK_DELAY_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Delay between Reference Clock and the ESC"]
    #[inline(always)]
    pub fn clk_delay(&mut self) -> CLK_DELAY_W {
        CLK_DELAY_W { w: self }
    }
}
