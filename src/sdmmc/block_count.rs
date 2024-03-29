#[doc = "Register `BLOCK_COUNT` reader"]
pub type R = crate::R<BlockCountSpec>;
#[doc = "Register `BLOCK_COUNT` writer"]
pub type W = crate::W<BlockCountSpec>;
#[doc = "Field `BLOCK_COUNT` reader - Blocks Count for Current Transfer"]
pub type BlockCountR = crate::FieldReader<u16>;
#[doc = "Field `BLOCK_COUNT` writer - Blocks Count for Current Transfer"]
pub type BlockCountW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Blocks Count for Current Transfer"]
    #[inline(always)]
    pub fn block_count(&self) -> BlockCountR {
        BlockCountR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - Blocks Count for Current Transfer"]
    #[inline(always)]
    #[must_use]
    pub fn block_count(&mut self) -> BlockCountW<BlockCountSpec> {
        BlockCountW::new(self, 0)
    }
}
#[doc = "Block Count Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`block_count::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`block_count::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BlockCountSpec;
impl crate::RegisterSpec for BlockCountSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`block_count::R`](R) reader structure"]
impl crate::Readable for BlockCountSpec {}
#[doc = "`write(|w| ..)` method takes [`block_count::W`](W) writer structure"]
impl crate::Writable for BlockCountSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets BLOCK_COUNT to value 0"]
impl crate::Resettable for BlockCountSpec {
    const RESET_VALUE: u16 = 0;
}
