#[doc = "Register `FPC` reader"]
pub type R = crate::R<FpcSpec>;
#[doc = "Register `FPC` writer"]
pub type W = crate::W<FpcSpec>;
#[doc = "Field `PCMP` reader - Floating Prescaler Compare Value"]
pub type PcmpR = crate::FieldReader;
#[doc = "Field `PVAL` reader - Actual Prescaler Value"]
pub type PvalR = crate::FieldReader;
#[doc = "Field `PVAL` writer - Actual Prescaler Value"]
pub type PvalW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - Floating Prescaler Compare Value"]
    #[inline(always)]
    pub fn pcmp(&self) -> PcmpR {
        PcmpR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Actual Prescaler Value"]
    #[inline(always)]
    pub fn pval(&self) -> PvalR {
        PvalR::new(((self.bits >> 8) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 8:11 - Actual Prescaler Value"]
    #[inline(always)]
    #[must_use]
    pub fn pval(&mut self) -> PvalW<FpcSpec> {
        PvalW::new(self, 8)
    }
}
#[doc = "Floating Prescaler Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fpc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fpc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FpcSpec;
impl crate::RegisterSpec for FpcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fpc::R`](R) reader structure"]
impl crate::Readable for FpcSpec {}
#[doc = "`write(|w| ..)` method takes [`fpc::W`](W) writer structure"]
impl crate::Writable for FpcSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FPC to value 0"]
impl crate::Resettable for FpcSpec {
    const RESET_VALUE: u32 = 0;
}
