#[doc = "Register `PCGCCTL` reader"]
pub type R = crate::R<PCGCCTL_SPEC>;
#[doc = "Register `PCGCCTL` writer"]
pub type W = crate::W<PCGCCTL_SPEC>;
#[doc = "Field `StopPclk` reader - Stop Pclk"]
pub type STOP_PCLK_R = crate::BitReader;
#[doc = "Field `StopPclk` writer - Stop Pclk"]
pub type STOP_PCLK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GateHclk` reader - Gate Hclk"]
pub type GATE_HCLK_R = crate::BitReader;
#[doc = "Field `GateHclk` writer - Gate Hclk"]
pub type GATE_HCLK_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Stop Pclk"]
    #[inline(always)]
    pub fn stop_pclk(&self) -> STOP_PCLK_R {
        STOP_PCLK_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Gate Hclk"]
    #[inline(always)]
    pub fn gate_hclk(&self) -> GATE_HCLK_R {
        GATE_HCLK_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Stop Pclk"]
    #[inline(always)]
    pub fn stop_pclk(&mut self) -> STOP_PCLK_W<PCGCCTL_SPEC> {
        STOP_PCLK_W::new(self, 0)
    }
    #[doc = "Bit 1 - Gate Hclk"]
    #[inline(always)]
    pub fn gate_hclk(&mut self) -> GATE_HCLK_W<PCGCCTL_SPEC> {
        GATE_HCLK_W::new(self, 1)
    }
}
#[doc = "Power and Clock Gating Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pcgcctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcgcctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PCGCCTL_SPEC;
impl crate::RegisterSpec for PCGCCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcgcctl::R`](R) reader structure"]
impl crate::Readable for PCGCCTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pcgcctl::W`](W) writer structure"]
impl crate::Writable for PCGCCTL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PCGCCTL to value 0x0100"]
impl crate::Resettable for PCGCCTL_SPEC {
    const RESET_VALUE: u32 = 0x0100;
}
