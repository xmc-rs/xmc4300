#[doc = "Register `PCGCCTL` reader"]
pub type R = crate::R<PCGCCTL_SPEC>;
#[doc = "Register `PCGCCTL` writer"]
pub type W = crate::W<PCGCCTL_SPEC>;
#[doc = "Field `StopPclk` reader - Stop Pclk"]
pub type STOP_PCLK_R = crate::BitReader;
#[doc = "Field `StopPclk` writer - Stop Pclk"]
pub type STOP_PCLK_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `GateHclk` reader - Gate Hclk"]
pub type GATE_HCLK_R = crate::BitReader;
#[doc = "Field `GateHclk` writer - Gate Hclk"]
pub type GATE_HCLK_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
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
    #[must_use]
    pub fn stop_pclk(&mut self) -> STOP_PCLK_W<PCGCCTL_SPEC, 0> {
        STOP_PCLK_W::new(self)
    }
    #[doc = "Bit 1 - Gate Hclk"]
    #[inline(always)]
    #[must_use]
    pub fn gate_hclk(&mut self) -> GATE_HCLK_W<PCGCCTL_SPEC, 1> {
        GATE_HCLK_W::new(self)
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
#[doc = "Power and Clock Gating Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcgcctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcgcctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PCGCCTL_SPEC;
impl crate::RegisterSpec for PCGCCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcgcctl::R`](R) reader structure"]
impl crate::Readable for PCGCCTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pcgcctl::W`](W) writer structure"]
impl crate::Writable for PCGCCTL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PCGCCTL to value 0x0100"]
impl crate::Resettable for PCGCCTL_SPEC {
    const RESET_VALUE: Self::Ux = 0x0100;
}
