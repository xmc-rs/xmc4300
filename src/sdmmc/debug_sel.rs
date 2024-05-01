#[doc = "Register `DEBUG_SEL` writer"]
pub type W = crate::W<DebugSelSpec>;
#[doc = "Debug_sel\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DebugSel {
    #[doc = "0: receiver module and fifo_ctrl module signals are probed out"]
    Value1 = 0,
    #[doc = "1: cmd register, Interrupt status, transmitter module and clk sdcard signals are probed out."]
    Value2 = 1,
}
impl From<DebugSel> for bool {
    #[inline(always)]
    fn from(variant: DebugSel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DEBUG_SEL` writer - Debug_sel"]
pub type DebugSelW<'a, REG> = crate::BitWriter<'a, REG, DebugSel>;
impl<'a, REG> DebugSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "receiver module and fifo_ctrl module signals are probed out"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(DebugSel::Value1)
    }
    #[doc = "cmd register, Interrupt status, transmitter module and clk sdcard signals are probed out."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(DebugSel::Value2)
    }
}
impl W {
    #[doc = "Bit 0 - Debug_sel"]
    #[inline(always)]
    #[must_use]
    pub fn debug_sel(&mut self) -> DebugSelW<DebugSelSpec> {
        DebugSelW::new(self, 0)
    }
}
#[doc = "Debug Selection Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`debug_sel::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DebugSelSpec;
impl crate::RegisterSpec for DebugSelSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`debug_sel::W`](W) writer structure"]
impl crate::Writable for DebugSelSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DEBUG_SEL to value 0"]
impl crate::Resettable for DebugSelSpec {
    const RESET_VALUE: u32 = 0;
}
