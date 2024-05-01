#[doc = "Register `TSVAL` reader"]
pub type R = crate::R<TsvalSpec>;
#[doc = "Register `TSVAL` writer"]
pub type W = crate::W<TsvalSpec>;
#[doc = "Field `TSCTRVALR` reader - Shadow TS-Counter (Read)"]
pub type TsctrvalrR = crate::FieldReader<u16>;
#[doc = "Field `TSCTRVAL` reader - TS-Counter Value"]
pub type TsctrvalR = crate::FieldReader<u16>;
#[doc = "Field `TSCTRVAL` writer - TS-Counter Value"]
pub type TsctrvalW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Shadow TS-Counter (Read)"]
    #[inline(always)]
    pub fn tsctrvalr(&self) -> TsctrvalrR {
        TsctrvalrR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - TS-Counter Value"]
    #[inline(always)]
    pub fn tsctrval(&self) -> TsctrvalR {
        TsctrvalR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:31 - TS-Counter Value"]
    #[inline(always)]
    #[must_use]
    pub fn tsctrval(&mut self) -> TsctrvalW<TsvalSpec> {
        TsctrvalW::new(self, 16)
    }
}
#[doc = "Touch-sense TS-Counter Value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tsval::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tsval::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TsvalSpec;
impl crate::RegisterSpec for TsvalSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tsval::R`](R) reader structure"]
impl crate::Readable for TsvalSpec {}
#[doc = "`write(|w| ..)` method takes [`tsval::W`](W) writer structure"]
impl crate::Writable for TsvalSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TSVAL to value 0"]
impl crate::Resettable for TsvalSpec {
    const RESET_VALUE: u32 = 0;
}
