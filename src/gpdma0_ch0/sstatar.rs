#[doc = "Register `SSTATAR` reader"]
pub type R = crate::R<SstatarSpec>;
#[doc = "Register `SSTATAR` writer"]
pub type W = crate::W<SstatarSpec>;
#[doc = "Field `SSTATAR` reader - Source Status Address"]
pub type SstatarR = crate::FieldReader<u32>;
#[doc = "Field `SSTATAR` writer - Source Status Address"]
pub type SstatarW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Source Status Address"]
    #[inline(always)]
    pub fn sstatar(&self) -> SstatarR {
        SstatarR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Source Status Address"]
    #[inline(always)]
    #[must_use]
    pub fn sstatar(&mut self) -> SstatarW<SstatarSpec> {
        SstatarW::new(self, 0)
    }
}
#[doc = "Source Status Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sstatar::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sstatar::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SstatarSpec;
impl crate::RegisterSpec for SstatarSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sstatar::R`](R) reader structure"]
impl crate::Readable for SstatarSpec {}
#[doc = "`write(|w| ..)` method takes [`sstatar::W`](W) writer structure"]
impl crate::Writable for SstatarSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SSTATAR to value 0"]
impl crate::Resettable for SstatarSpec {
    const RESET_VALUE: u32 = 0;
}
