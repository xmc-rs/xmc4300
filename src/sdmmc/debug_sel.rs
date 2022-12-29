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
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
pub type DEBUG_SEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, DEBUG_SEL_SPEC, DEBUG_SEL_AW, O>;
impl<'a, const O: u8> DEBUG_SEL_W<'a, O> {
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
}
impl W {
    #[doc = "Bit 0 - Debug_sel"]
    #[inline(always)]
    #[must_use]
    pub fn debug_sel(&mut self) -> DEBUG_SEL_W<0> {
        DEBUG_SEL_W::new(self)
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
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DEBUG_SEL to value 0"]
impl crate::Resettable for DEBUG_SEL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
