#[doc = "Register `RMDATA` reader"]
pub type R = crate::R<RMDATA_SPEC>;
#[doc = "Register `RMDATA` writer"]
pub type W = crate::W<RMDATA_SPEC>;
#[doc = "Field `DATA` reader - Hibernate Retention Memory Data"]
pub type DATA_R = crate::FieldReader<u32>;
#[doc = "Field `DATA` writer - Hibernate Retention Memory Data"]
pub type DATA_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Hibernate Retention Memory Data"]
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Hibernate Retention Memory Data"]
    #[inline(always)]
    #[must_use]
    pub fn data(&mut self) -> DATA_W<RMDATA_SPEC> {
        DATA_W::new(self, 0)
    }
}
#[doc = "Retention Memory Access Data Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rmdata::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rmdata::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RMDATA_SPEC;
impl crate::RegisterSpec for RMDATA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rmdata::R`](R) reader structure"]
impl crate::Readable for RMDATA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rmdata::W`](W) writer structure"]
impl crate::Writable for RMDATA_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RMDATA to value 0"]
impl crate::Resettable for RMDATA_SPEC {
    const RESET_VALUE: u32 = 0;
}
