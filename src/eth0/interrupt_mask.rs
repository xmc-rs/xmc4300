#[doc = "Register `INTERRUPT_MASK` reader"]
pub type R = crate::R<InterruptMaskSpec>;
#[doc = "Register `INTERRUPT_MASK` writer"]
pub type W = crate::W<InterruptMaskSpec>;
#[doc = "Field `PMTIM` reader - PMT Interrupt Mask"]
pub type PmtimR = crate::BitReader;
#[doc = "Field `PMTIM` writer - PMT Interrupt Mask"]
pub type PmtimW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSIM` reader - Timestamp Interrupt Mask"]
pub type TsimR = crate::BitReader;
#[doc = "Field `TSIM` writer - Timestamp Interrupt Mask"]
pub type TsimW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 3 - PMT Interrupt Mask"]
    #[inline(always)]
    pub fn pmtim(&self) -> PmtimR {
        PmtimR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 9 - Timestamp Interrupt Mask"]
    #[inline(always)]
    pub fn tsim(&self) -> TsimR {
        TsimR::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - PMT Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn pmtim(&mut self) -> PmtimW<InterruptMaskSpec> {
        PmtimW::new(self, 3)
    }
    #[doc = "Bit 9 - Timestamp Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn tsim(&mut self) -> TsimW<InterruptMaskSpec> {
        TsimW::new(self, 9)
    }
}
#[doc = "Interrupt Mask Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`interrupt_mask::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`interrupt_mask::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct InterruptMaskSpec;
impl crate::RegisterSpec for InterruptMaskSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`interrupt_mask::R`](R) reader structure"]
impl crate::Readable for InterruptMaskSpec {}
#[doc = "`write(|w| ..)` method takes [`interrupt_mask::W`](W) writer structure"]
impl crate::Writable for InterruptMaskSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTERRUPT_MASK to value 0"]
impl crate::Resettable for InterruptMaskSpec {
    const RESET_VALUE: u32 = 0;
}
