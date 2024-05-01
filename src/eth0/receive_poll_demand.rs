#[doc = "Register `RECEIVE_POLL_DEMAND` reader"]
pub type R = crate::R<ReceivePollDemandSpec>;
#[doc = "Register `RECEIVE_POLL_DEMAND` writer"]
pub type W = crate::W<ReceivePollDemandSpec>;
#[doc = "Field `RPD` reader - Receive Poll Demand"]
pub type RpdR = crate::FieldReader<u32>;
#[doc = "Field `RPD` writer - Receive Poll Demand"]
pub type RpdW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Receive Poll Demand"]
    #[inline(always)]
    pub fn rpd(&self) -> RpdR {
        RpdR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Receive Poll Demand"]
    #[inline(always)]
    #[must_use]
    pub fn rpd(&mut self) -> RpdW<ReceivePollDemandSpec> {
        RpdW::new(self, 0)
    }
}
#[doc = "Receive Poll Demand Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`receive_poll_demand::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`receive_poll_demand::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ReceivePollDemandSpec;
impl crate::RegisterSpec for ReceivePollDemandSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`receive_poll_demand::R`](R) reader structure"]
impl crate::Readable for ReceivePollDemandSpec {}
#[doc = "`write(|w| ..)` method takes [`receive_poll_demand::W`](W) writer structure"]
impl crate::Writable for ReceivePollDemandSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RECEIVE_POLL_DEMAND to value 0"]
impl crate::Resettable for ReceivePollDemandSpec {
    const RESET_VALUE: u32 = 0;
}
