#[doc = "Register `HASH_TABLE_HIGH` reader"]
pub type R = crate::R<HASH_TABLE_HIGH_SPEC>;
#[doc = "Register `HASH_TABLE_HIGH` writer"]
pub type W = crate::W<HASH_TABLE_HIGH_SPEC>;
#[doc = "Field `HTH` reader - Hash Table High"]
pub type HTH_R = crate::FieldReader<u32>;
#[doc = "Field `HTH` writer - Hash Table High"]
pub type HTH_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Hash Table High"]
    #[inline(always)]
    pub fn hth(&self) -> HTH_R {
        HTH_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Hash Table High"]
    #[inline(always)]
    #[must_use]
    pub fn hth(&mut self) -> HTH_W<HASH_TABLE_HIGH_SPEC> {
        HTH_W::new(self, 0)
    }
}
#[doc = "Hash Table High Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hash_table_high::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hash_table_high::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HASH_TABLE_HIGH_SPEC;
impl crate::RegisterSpec for HASH_TABLE_HIGH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hash_table_high::R`](R) reader structure"]
impl crate::Readable for HASH_TABLE_HIGH_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hash_table_high::W`](W) writer structure"]
impl crate::Writable for HASH_TABLE_HIGH_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HASH_TABLE_HIGH to value 0"]
impl crate::Resettable for HASH_TABLE_HIGH_SPEC {
    const RESET_VALUE: u32 = 0;
}
