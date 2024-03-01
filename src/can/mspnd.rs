#[doc = "Register `MSPND[%s]` reader"]
pub type R = crate::R<MspndSpec>;
#[doc = "Register `MSPND[%s]` writer"]
pub type W = crate::W<MspndSpec>;
#[doc = "Field `PND` reader - Message Pending"]
pub type PndR = crate::FieldReader<u32>;
#[doc = "Field `PND` writer - Message Pending"]
pub type PndW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Message Pending"]
    #[inline(always)]
    pub fn pnd(&self) -> PndR {
        PndR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Message Pending"]
    #[inline(always)]
    #[must_use]
    pub fn pnd(&mut self) -> PndW<MspndSpec> {
        PndW::new(self, 0)
    }
}
#[doc = "Message Pending Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mspnd::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mspnd::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MspndSpec;
impl crate::RegisterSpec for MspndSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mspnd::R`](R) reader structure"]
impl crate::Readable for MspndSpec {}
#[doc = "`write(|w| ..)` method takes [`mspnd::W`](W) writer structure"]
impl crate::Writable for MspndSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MSPND[%s]
to value 0"]
impl crate::Resettable for MspndSpec {
    const RESET_VALUE: u32 = 0;
}
