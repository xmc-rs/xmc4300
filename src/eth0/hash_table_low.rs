#[doc = "Register `HASH_TABLE_LOW` reader"]
pub type R = crate::R<HashTableLowSpec>;
#[doc = "Register `HASH_TABLE_LOW` writer"]
pub type W = crate::W<HashTableLowSpec>;
#[doc = "Field `HTL` reader - Hash Table Low"]
pub type HtlR = crate::FieldReader<u32>;
#[doc = "Field `HTL` writer - Hash Table Low"]
pub type HtlW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Hash Table Low"]
    #[inline(always)]
    pub fn htl(&self) -> HtlR {
        HtlR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Hash Table Low"]
    #[inline(always)]
    #[must_use]
    pub fn htl(&mut self) -> HtlW<HashTableLowSpec> {
        HtlW::new(self, 0)
    }
}
#[doc = "Hash Table Low Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hash_table_low::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hash_table_low::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HashTableLowSpec;
impl crate::RegisterSpec for HashTableLowSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hash_table_low::R`](R) reader structure"]
impl crate::Readable for HashTableLowSpec {}
#[doc = "`write(|w| ..)` method takes [`hash_table_low::W`](W) writer structure"]
impl crate::Writable for HashTableLowSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HASH_TABLE_LOW to value 0"]
impl crate::Resettable for HashTableLowSpec {
    const RESET_VALUE: u32 = 0;
}
