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
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Retention Memory Access Data Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rmdata::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rmdata::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RMDATA_SPEC;
impl crate::RegisterSpec for RMDATA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rmdata::R`](R) reader structure"]
impl crate::Readable for RMDATA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rmdata::W`](W) writer structure"]
impl crate::Writable for RMDATA_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RMDATA to value 0"]
impl crate::Resettable for RMDATA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
