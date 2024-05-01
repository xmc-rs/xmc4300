#[doc = "Register `PCGCCTL` reader"]
pub type R = crate::R<PcgcctlSpec>;
#[doc = "Register `PCGCCTL` writer"]
pub type W = crate::W<PcgcctlSpec>;
#[doc = "Field `StopPclk` reader - Stop Pclk"]
pub type StopPclkR = crate::BitReader;
#[doc = "Field `StopPclk` writer - Stop Pclk"]
pub type StopPclkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GateHclk` reader - Gate Hclk"]
pub type GateHclkR = crate::BitReader;
#[doc = "Field `GateHclk` writer - Gate Hclk"]
pub type GateHclkW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Stop Pclk"]
    #[inline(always)]
    pub fn stop_pclk(&self) -> StopPclkR {
        StopPclkR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Gate Hclk"]
    #[inline(always)]
    pub fn gate_hclk(&self) -> GateHclkR {
        GateHclkR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Stop Pclk"]
    #[inline(always)]
    #[must_use]
    pub fn stop_pclk(&mut self) -> StopPclkW<PcgcctlSpec> {
        StopPclkW::new(self, 0)
    }
    #[doc = "Bit 1 - Gate Hclk"]
    #[inline(always)]
    #[must_use]
    pub fn gate_hclk(&mut self) -> GateHclkW<PcgcctlSpec> {
        GateHclkW::new(self, 1)
    }
}
#[doc = "Power and Clock Gating Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcgcctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcgcctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PcgcctlSpec;
impl crate::RegisterSpec for PcgcctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcgcctl::R`](R) reader structure"]
impl crate::Readable for PcgcctlSpec {}
#[doc = "`write(|w| ..)` method takes [`pcgcctl::W`](W) writer structure"]
impl crate::Writable for PcgcctlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PCGCCTL to value 0x0100"]
impl crate::Resettable for PcgcctlSpec {
    const RESET_VALUE: u32 = 0x0100;
}
