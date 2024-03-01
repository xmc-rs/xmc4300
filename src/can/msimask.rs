#[doc = "Register `MSIMASK` reader"]
pub type R = crate::R<MsimaskSpec>;
#[doc = "Register `MSIMASK` writer"]
pub type W = crate::W<MsimaskSpec>;
#[doc = "Field `IM` reader - Message Index Mask"]
pub type ImR = crate::FieldReader<u32>;
#[doc = "Field `IM` writer - Message Index Mask"]
pub type ImW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Message Index Mask"]
    #[inline(always)]
    pub fn im(&self) -> ImR {
        ImR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Message Index Mask"]
    #[inline(always)]
    #[must_use]
    pub fn im(&mut self) -> ImW<MsimaskSpec> {
        ImW::new(self, 0)
    }
}
#[doc = "Message Index Mask Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`msimask::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`msimask::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MsimaskSpec;
impl crate::RegisterSpec for MsimaskSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`msimask::R`](R) reader structure"]
impl crate::Readable for MsimaskSpec {}
#[doc = "`write(|w| ..)` method takes [`msimask::W`](W) writer structure"]
impl crate::Writable for MsimaskSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MSIMASK to value 0"]
impl crate::Resettable for MsimaskSpec {
    const RESET_VALUE: u32 = 0;
}
