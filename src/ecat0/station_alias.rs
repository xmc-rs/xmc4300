#[doc = "Register `STATION_ALIAS` reader"]
pub type R = crate::R<StationAliasSpec>;
#[doc = "Register `STATION_ALIAS` writer"]
pub type W = crate::W<StationAliasSpec>;
#[doc = "Field `ALIAS_ADDR` reader - Alias Address used for node addressing(FPxx commands)"]
pub type AliasAddrR = crate::FieldReader<u16>;
#[doc = "Field `ALIAS_ADDR` writer - Alias Address used for node addressing(FPxx commands)"]
pub type AliasAddrW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Alias Address used for node addressing(FPxx commands)"]
    #[inline(always)]
    pub fn alias_addr(&self) -> AliasAddrR {
        AliasAddrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - Alias Address used for node addressing(FPxx commands)"]
    #[inline(always)]
    #[must_use]
    pub fn alias_addr(&mut self) -> AliasAddrW<StationAliasSpec> {
        AliasAddrW::new(self, 0)
    }
}
#[doc = "Configured Station Alias\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`station_alias::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`station_alias::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StationAliasSpec;
impl crate::RegisterSpec for StationAliasSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`station_alias::R`](R) reader structure"]
impl crate::Readable for StationAliasSpec {}
#[doc = "`write(|w| ..)` method takes [`station_alias::W`](W) writer structure"]
impl crate::Writable for StationAliasSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets STATION_ALIAS to value 0"]
impl crate::Resettable for StationAliasSpec {
    const RESET_VALUE: u16 = 0;
}
