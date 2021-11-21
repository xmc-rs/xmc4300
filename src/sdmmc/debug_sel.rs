#[doc = "Register `DEBUG_SEL` writer"]
pub struct W(crate::W<DEBUG_SEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DEBUG_SEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<DEBUG_SEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DEBUG_SEL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Debug_sel\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DEBUG_SEL_AW {
    #[doc = "0: receiver module and fifo_ctrl module signals are probed out"]
    VALUE1 = 0,
    #[doc = "1: cmd register, Interrupt status, transmitter module and clk sdcard signals are probed out."]
    VALUE2 = 1,
}
impl From<DEBUG_SEL_AW> for bool {
    #[inline(always)]
    fn from(variant: DEBUG_SEL_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DEBUG_SEL` writer - Debug_sel"]
pub struct DEBUG_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> DEBUG_SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DEBUG_SEL_AW) -> &'a mut W {
        self.bit(variant.into())
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
impl W {
    #[doc = "Bit 0 - Debug_sel"]
    #[inline(always)]
    pub fn debug_sel(&mut self) -> DEBUG_SEL_W {
        DEBUG_SEL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Debug Selection Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [debug_sel](index.html) module"]
pub struct DEBUG_SEL_SPEC;
impl crate::RegisterSpec for DEBUG_SEL_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [debug_sel::W](W) writer structure"]
impl crate::Writable for DEBUG_SEL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DEBUG_SEL to value 0"]
impl crate::Resettable for DEBUG_SEL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
