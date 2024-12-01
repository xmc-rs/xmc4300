#[doc = "Register `RECEIVE_POLL_DEMAND` reader"]
pub type R = crate::R<RECEIVE_POLL_DEMAND_SPEC>;
#[doc = "Register `RECEIVE_POLL_DEMAND` writer"]
pub type W = crate::W<RECEIVE_POLL_DEMAND_SPEC>;
#[doc = "Field `RPD` reader - Receive Poll Demand"]
pub type RPD_R = crate::FieldReader<u32>;
#[doc = "Field `RPD` writer - Receive Poll Demand"]
pub type RPD_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Receive Poll Demand"]
    #[inline(always)]
    pub fn rpd(&self) -> RPD_R {
        RPD_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Receive Poll Demand"]
    #[inline(always)]
    pub fn rpd(&mut self) -> RPD_W<RECEIVE_POLL_DEMAND_SPEC> {
        RPD_W::new(self, 0)
    }
}
#[doc = "Receive Poll Demand Register\n\nYou can [`read`](crate::Reg::read) this register and get [`receive_poll_demand::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`receive_poll_demand::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RECEIVE_POLL_DEMAND_SPEC;
impl crate::RegisterSpec for RECEIVE_POLL_DEMAND_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`receive_poll_demand::R`](R) reader structure"]
impl crate::Readable for RECEIVE_POLL_DEMAND_SPEC {}
#[doc = "`write(|w| ..)` method takes [`receive_poll_demand::W`](W) writer structure"]
impl crate::Writable for RECEIVE_POLL_DEMAND_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RECEIVE_POLL_DEMAND to value 0"]
impl crate::Resettable for RECEIVE_POLL_DEMAND_SPEC {
    const RESET_VALUE: u32 = 0;
}
