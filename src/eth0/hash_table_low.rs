#[doc = "Register `HASH_TABLE_LOW` reader"]
pub type R = crate::R<HASH_TABLE_LOW_SPEC>;
#[doc = "Register `HASH_TABLE_LOW` writer"]
pub type W = crate::W<HASH_TABLE_LOW_SPEC>;
#[doc = "Field `HTL` reader - Hash Table Low"]
pub type HTL_R = crate::FieldReader<u32>;
#[doc = "Field `HTL` writer - Hash Table Low"]
pub type HTL_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Hash Table Low"]
    #[inline(always)]
    pub fn htl(&self) -> HTL_R {
        HTL_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Hash Table Low"]
    #[inline(always)]
    #[must_use]
    pub fn htl(&mut self) -> HTL_W<HASH_TABLE_LOW_SPEC> {
        HTL_W::new(self, 0)
    }
}
#[doc = "Hash Table Low Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hash_table_low::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hash_table_low::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HASH_TABLE_LOW_SPEC;
impl crate::RegisterSpec for HASH_TABLE_LOW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hash_table_low::R`](R) reader structure"]
impl crate::Readable for HASH_TABLE_LOW_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hash_table_low::W`](W) writer structure"]
impl crate::Writable for HASH_TABLE_LOW_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HASH_TABLE_LOW to value 0"]
impl crate::Resettable for HASH_TABLE_LOW_SPEC {
    const RESET_VALUE: u32 = 0;
}
