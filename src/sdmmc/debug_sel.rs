#[doc = "Register `DEBUG_SEL` writer"]
pub type W = crate::W<DEBUG_SEL_SPEC>;
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
pub type DEBUG_SEL_W<'a, REG> = crate::BitWriter<'a, REG, DEBUG_SEL_AW>;
impl<'a, REG> DEBUG_SEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "receiver module and fifo_ctrl module signals are probed out"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(DEBUG_SEL_AW::VALUE1)
    }
    #[doc = "cmd register, Interrupt status, transmitter module and clk sdcard signals are probed out."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(DEBUG_SEL_AW::VALUE2)
    }
}
impl W {
    #[doc = "Bit 0 - Debug_sel"]
    #[inline(always)]
    #[must_use]
    pub fn debug_sel(&mut self) -> DEBUG_SEL_W<DEBUG_SEL_SPEC> {
        DEBUG_SEL_W::new(self, 0)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Debug Selection Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`debug_sel::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DEBUG_SEL_SPEC;
impl crate::RegisterSpec for DEBUG_SEL_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`debug_sel::W`](W) writer structure"]
impl crate::Writable for DEBUG_SEL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DEBUG_SEL to value 0"]
impl crate::Resettable for DEBUG_SEL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
