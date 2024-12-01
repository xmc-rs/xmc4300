#[doc = "Register `BLOCK_SIZE` reader"]
pub type R = crate::R<BLOCK_SIZE_SPEC>;
#[doc = "Register `BLOCK_SIZE` writer"]
pub type W = crate::W<BLOCK_SIZE_SPEC>;
#[doc = "Field `TX_BLOCK_SIZE` reader - Transfer Block Size"]
pub type TX_BLOCK_SIZE_R = crate::FieldReader<u16>;
#[doc = "Field `TX_BLOCK_SIZE` writer - Transfer Block Size"]
pub type TX_BLOCK_SIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `TX_BLOCK_SIZE_12` reader - Transfer Block Size 12th bit."]
pub type TX_BLOCK_SIZE_12_R = crate::BitReader;
#[doc = "Field `TX_BLOCK_SIZE_12` writer - Transfer Block Size 12th bit."]
pub type TX_BLOCK_SIZE_12_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:11 - Transfer Block Size"]
    #[inline(always)]
    pub fn tx_block_size(&self) -> TX_BLOCK_SIZE_R {
        TX_BLOCK_SIZE_R::new(self.bits & 0x0fff)
    }
    #[doc = "Bit 15 - Transfer Block Size 12th bit."]
    #[inline(always)]
    pub fn tx_block_size_12(&self) -> TX_BLOCK_SIZE_12_R {
        TX_BLOCK_SIZE_12_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:11 - Transfer Block Size"]
    #[inline(always)]
    pub fn tx_block_size(&mut self) -> TX_BLOCK_SIZE_W<BLOCK_SIZE_SPEC> {
        TX_BLOCK_SIZE_W::new(self, 0)
    }
    #[doc = "Bit 15 - Transfer Block Size 12th bit."]
    #[inline(always)]
    pub fn tx_block_size_12(&mut self) -> TX_BLOCK_SIZE_12_W<BLOCK_SIZE_SPEC> {
        TX_BLOCK_SIZE_12_W::new(self, 15)
    }
}
#[doc = "Block Size Register\n\nYou can [`read`](crate::Reg::read) this register and get [`block_size::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`block_size::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BLOCK_SIZE_SPEC;
impl crate::RegisterSpec for BLOCK_SIZE_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`block_size::R`](R) reader structure"]
impl crate::Readable for BLOCK_SIZE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`block_size::W`](W) writer structure"]
impl crate::Writable for BLOCK_SIZE_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets BLOCK_SIZE to value 0"]
impl crate::Resettable for BLOCK_SIZE_SPEC {
    const RESET_VALUE: u16 = 0;
}
