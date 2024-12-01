#[doc = "Register `TRANSMIT_POLL_DEMAND` reader"]
pub type R = crate::R<TRANSMIT_POLL_DEMAND_SPEC>;
#[doc = "Register `TRANSMIT_POLL_DEMAND` writer"]
pub type W = crate::W<TRANSMIT_POLL_DEMAND_SPEC>;
#[doc = "Field `TPD` reader - Transmit Poll Demand"]
pub type TPD_R = crate::FieldReader<u32>;
#[doc = "Field `TPD` writer - Transmit Poll Demand"]
pub type TPD_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Transmit Poll Demand"]
    #[inline(always)]
    pub fn tpd(&self) -> TPD_R {
        TPD_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Transmit Poll Demand"]
    #[inline(always)]
    pub fn tpd(&mut self) -> TPD_W<TRANSMIT_POLL_DEMAND_SPEC> {
        TPD_W::new(self, 0)
    }
}
#[doc = "Transmit Poll Demand Register\n\nYou can [`read`](crate::Reg::read) this register and get [`transmit_poll_demand::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`transmit_poll_demand::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TRANSMIT_POLL_DEMAND_SPEC;
impl crate::RegisterSpec for TRANSMIT_POLL_DEMAND_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`transmit_poll_demand::R`](R) reader structure"]
impl crate::Readable for TRANSMIT_POLL_DEMAND_SPEC {}
#[doc = "`write(|w| ..)` method takes [`transmit_poll_demand::W`](W) writer structure"]
impl crate::Writable for TRANSMIT_POLL_DEMAND_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TRANSMIT_POLL_DEMAND to value 0"]
impl crate::Resettable for TRANSMIT_POLL_DEMAND_SPEC {
    const RESET_VALUE: u32 = 0;
}
