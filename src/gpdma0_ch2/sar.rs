#[doc = "Register `SAR` reader"]
pub type R = crate::R<SarSpec>;
#[doc = "Register `SAR` writer"]
pub type W = crate::W<SarSpec>;
#[doc = "Field `SAR` reader - Current Source Address of DMA transfer"]
pub type SarR = crate::FieldReader<u32>;
#[doc = "Field `SAR` writer - Current Source Address of DMA transfer"]
pub type SarW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Current Source Address of DMA transfer"]
    #[inline(always)]
    pub fn sar(&self) -> SarR {
        SarR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Current Source Address of DMA transfer"]
    #[inline(always)]
    #[must_use]
    pub fn sar(&mut self) -> SarW<SarSpec> {
        SarW::new(self, 0)
    }
}
#[doc = "Source Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sar::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sar::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SarSpec;
impl crate::RegisterSpec for SarSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sar::R`](R) reader structure"]
impl crate::Readable for SarSpec {}
#[doc = "`write(|w| ..)` method takes [`sar::W`](W) writer structure"]
impl crate::Writable for SarSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SAR to value 0"]
impl crate::Resettable for SarSpec {
    const RESET_VALUE: u32 = 0;
}
