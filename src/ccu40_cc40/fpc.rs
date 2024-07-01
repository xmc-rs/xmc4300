#[doc = "Register `FPC` reader"]
pub type R = crate::R<FPC_SPEC>;
#[doc = "Register `FPC` writer"]
pub type W = crate::W<FPC_SPEC>;
#[doc = "Field `PCMP` reader - Floating Prescaler Compare Value"]
pub type PCMP_R = crate::FieldReader;
#[doc = "Field `PVAL` reader - Actual Prescaler Value"]
pub type PVAL_R = crate::FieldReader;
#[doc = "Field `PVAL` writer - Actual Prescaler Value"]
pub type PVAL_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - Floating Prescaler Compare Value"]
    #[inline(always)]
    pub fn pcmp(&self) -> PCMP_R {
        PCMP_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Actual Prescaler Value"]
    #[inline(always)]
    pub fn pval(&self) -> PVAL_R {
        PVAL_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 8:11 - Actual Prescaler Value"]
    #[inline(always)]
    #[must_use]
    pub fn pval(&mut self) -> PVAL_W<FPC_SPEC> {
        PVAL_W::new(self, 8)
    }
}
#[doc = "Floating Prescaler Control\n\nYou can [`read`](crate::Reg::read) this register and get [`fpc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fpc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FPC_SPEC;
impl crate::RegisterSpec for FPC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fpc::R`](R) reader structure"]
impl crate::Readable for FPC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`fpc::W`](W) writer structure"]
impl crate::Writable for FPC_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FPC to value 0"]
impl crate::Resettable for FPC_SPEC {
    const RESET_VALUE: u32 = 0;
}
