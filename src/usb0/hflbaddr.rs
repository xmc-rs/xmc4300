#[doc = "Register `HFLBADDR` reader"]
pub type R = crate::R<HflbaddrSpec>;
#[doc = "Register `HFLBADDR` writer"]
pub type W = crate::W<HflbaddrSpec>;
#[doc = "Field `Starting_Address` reader - Starting Address"]
pub type StartingAddressR = crate::FieldReader<u32>;
#[doc = "Field `Starting_Address` writer - Starting Address"]
pub type StartingAddressW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Starting Address"]
    #[inline(always)]
    pub fn starting_address(&self) -> StartingAddressR {
        StartingAddressR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Starting Address"]
    #[inline(always)]
    #[must_use]
    pub fn starting_address(&mut self) -> StartingAddressW<HflbaddrSpec> {
        StartingAddressW::new(self, 0)
    }
}
#[doc = "Host Frame List Base Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hflbaddr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hflbaddr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HflbaddrSpec;
impl crate::RegisterSpec for HflbaddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hflbaddr::R`](R) reader structure"]
impl crate::Readable for HflbaddrSpec {}
#[doc = "`write(|w| ..)` method takes [`hflbaddr::W`](W) writer structure"]
impl crate::Writable for HflbaddrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HFLBADDR to value 0"]
impl crate::Resettable for HflbaddrSpec {
    const RESET_VALUE: u32 = 0;
}
