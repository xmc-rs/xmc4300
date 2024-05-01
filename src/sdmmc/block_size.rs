#[doc = "Register `BLOCK_SIZE` reader"]
pub type R = crate::R<BlockSizeSpec>;
#[doc = "Register `BLOCK_SIZE` writer"]
pub type W = crate::W<BlockSizeSpec>;
#[doc = "Field `TX_BLOCK_SIZE` reader - Transfer Block Size"]
pub type TxBlockSizeR = crate::FieldReader<u16>;
#[doc = "Field `TX_BLOCK_SIZE` writer - Transfer Block Size"]
pub type TxBlockSizeW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `TX_BLOCK_SIZE_12` reader - Transfer Block Size 12th bit."]
pub type TxBlockSize12R = crate::BitReader;
#[doc = "Field `TX_BLOCK_SIZE_12` writer - Transfer Block Size 12th bit."]
pub type TxBlockSize12W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:11 - Transfer Block Size"]
    #[inline(always)]
    pub fn tx_block_size(&self) -> TxBlockSizeR {
        TxBlockSizeR::new(self.bits & 0x0fff)
    }
    #[doc = "Bit 15 - Transfer Block Size 12th bit."]
    #[inline(always)]
    pub fn tx_block_size_12(&self) -> TxBlockSize12R {
        TxBlockSize12R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:11 - Transfer Block Size"]
    #[inline(always)]
    #[must_use]
    pub fn tx_block_size(&mut self) -> TxBlockSizeW<BlockSizeSpec> {
        TxBlockSizeW::new(self, 0)
    }
    #[doc = "Bit 15 - Transfer Block Size 12th bit."]
    #[inline(always)]
    #[must_use]
    pub fn tx_block_size_12(&mut self) -> TxBlockSize12W<BlockSizeSpec> {
        TxBlockSize12W::new(self, 15)
    }
}
#[doc = "Block Size Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`block_size::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`block_size::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BlockSizeSpec;
impl crate::RegisterSpec for BlockSizeSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`block_size::R`](R) reader structure"]
impl crate::Readable for BlockSizeSpec {}
#[doc = "`write(|w| ..)` method takes [`block_size::W`](W) writer structure"]
impl crate::Writable for BlockSizeSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets BLOCK_SIZE to value 0"]
impl crate::Resettable for BlockSizeSpec {
    const RESET_VALUE: u16 = 0;
}
