#[doc = "Register `PMTPR` reader"]
pub type R = crate::R<PmtprSpec>;
#[doc = "Register `PMTPR` writer"]
pub type W = crate::W<PmtprSpec>;
#[doc = "Field `PWR` reader - Parity Write Values for Memory Test"]
pub type PwrR = crate::FieldReader;
#[doc = "Field `PWR` writer - Parity Write Values for Memory Test"]
pub type PwrW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PRD` reader - Parity Read Values for Memory Test"]
pub type PrdR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Parity Write Values for Memory Test"]
    #[inline(always)]
    pub fn pwr(&self) -> PwrR {
        PwrR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Parity Read Values for Memory Test"]
    #[inline(always)]
    pub fn prd(&self) -> PrdR {
        PrdR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Parity Write Values for Memory Test"]
    #[inline(always)]
    #[must_use]
    pub fn pwr(&mut self) -> PwrW<PmtprSpec> {
        PwrW::new(self, 0)
    }
}
#[doc = "Parity Memory Test Pattern Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmtpr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmtpr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PmtprSpec;
impl crate::RegisterSpec for PmtprSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pmtpr::R`](R) reader structure"]
impl crate::Readable for PmtprSpec {}
#[doc = "`write(|w| ..)` method takes [`pmtpr::W`](W) writer structure"]
impl crate::Writable for PmtprSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PMTPR to value 0"]
impl crate::Resettable for PmtprSpec {
    const RESET_VALUE: u32 = 0;
}
