#[doc = "Register `RMDATA` reader"]
pub type R = crate::R<RmdataSpec>;
#[doc = "Register `RMDATA` writer"]
pub type W = crate::W<RmdataSpec>;
#[doc = "Field `DATA` reader - Hibernate Retention Memory Data"]
pub type DataR = crate::FieldReader<u32>;
#[doc = "Field `DATA` writer - Hibernate Retention Memory Data"]
pub type DataW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Hibernate Retention Memory Data"]
    #[inline(always)]
    pub fn data(&self) -> DataR {
        DataR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Hibernate Retention Memory Data"]
    #[inline(always)]
    #[must_use]
    pub fn data(&mut self) -> DataW<RmdataSpec> {
        DataW::new(self, 0)
    }
}
#[doc = "Retention Memory Access Data Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rmdata::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rmdata::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RmdataSpec;
impl crate::RegisterSpec for RmdataSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rmdata::R`](R) reader structure"]
impl crate::Readable for RmdataSpec {}
#[doc = "`write(|w| ..)` method takes [`rmdata::W`](W) writer structure"]
impl crate::Writable for RmdataSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RMDATA to value 0"]
impl crate::Resettable for RmdataSpec {
    const RESET_VALUE: u32 = 0;
}
