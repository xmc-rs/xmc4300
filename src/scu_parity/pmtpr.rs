#[doc = "Register `PMTPR` reader"]
pub type R = crate::R<PMTPR_SPEC>;
#[doc = "Register `PMTPR` writer"]
pub type W = crate::W<PMTPR_SPEC>;
#[doc = "Field `PWR` reader - Parity Write Values for Memory Test"]
pub type PWR_R = crate::FieldReader;
#[doc = "Field `PWR` writer - Parity Write Values for Memory Test"]
pub type PWR_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PRD` reader - Parity Read Values for Memory Test"]
pub type PRD_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Parity Write Values for Memory Test"]
    #[inline(always)]
    pub fn pwr(&self) -> PWR_R {
        PWR_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Parity Read Values for Memory Test"]
    #[inline(always)]
    pub fn prd(&self) -> PRD_R {
        PRD_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Parity Write Values for Memory Test"]
    #[inline(always)]
    #[must_use]
    pub fn pwr(&mut self) -> PWR_W<PMTPR_SPEC> {
        PWR_W::new(self, 0)
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
#[doc = "Parity Memory Test Pattern Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmtpr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmtpr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PMTPR_SPEC;
impl crate::RegisterSpec for PMTPR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pmtpr::R`](R) reader structure"]
impl crate::Readable for PMTPR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pmtpr::W`](W) writer structure"]
impl crate::Writable for PMTPR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PMTPR to value 0"]
impl crate::Resettable for PMTPR_SPEC {
    const RESET_VALUE: u32 = 0;
}
