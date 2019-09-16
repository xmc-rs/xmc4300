#[doc = "Writer for register DEBUG_SEL"]
pub type W = crate::W<u32, super::DEBUG_SEL>;
#[doc = "Register DEBUG_SEL `reset()`'s with value 0"]
impl crate::ResetValue for super::DEBUG_SEL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Debug_sel\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DEBUG_SEL_AW {
    #[doc = "0: receiver module and fifo_ctrl module signals are probed out"]
    VALUE1,
    #[doc = "1: cmd register, Interrupt status, transmitter module and clk sdcard signals are probed out."]
    VALUE2,
}
impl From<DEBUG_SEL_AW> for bool {
    #[inline(always)]
    fn from(variant: DEBUG_SEL_AW) -> Self {
        match variant {
            DEBUG_SEL_AW::VALUE1 => false,
            DEBUG_SEL_AW::VALUE2 => true,
        }
    }
}
#[doc = "Write proxy for field `DEBUG_SEL`"]
pub struct DEBUG_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> DEBUG_SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DEBUG_SEL_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "receiver module and fifo_ctrl module signals are probed out"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(DEBUG_SEL_AW::VALUE1)
    }
    #[doc = "cmd register, Interrupt status, transmitter module and clk sdcard signals are probed out."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(DEBUG_SEL_AW::VALUE2)
    }
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
impl W {
    #[doc = "Bit 0 - Debug_sel"]
    #[inline(always)]
    pub fn debug_sel(&mut self) -> DEBUG_SEL_W {
        DEBUG_SEL_W { w: self }
    }
}
