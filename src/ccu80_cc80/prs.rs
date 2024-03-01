#[doc = "Register `PRS` reader"]
pub type R = crate::R<PrsSpec>;
#[doc = "Register `PRS` writer"]
pub type W = crate::W<PrsSpec>;
#[doc = "Field `PRS` reader - Period Register"]
pub type PrsR = crate::FieldReader<u16>;
#[doc = "Field `PRS` writer - Period Register"]
pub type PrsW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Period Register"]
    #[inline(always)]
    pub fn prs(&self) -> PrsR {
        PrsR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Period Register"]
    #[inline(always)]
    #[must_use]
    pub fn prs(&mut self) -> PrsW<PrsSpec> {
        PrsW::new(self, 0)
    }
}
#[doc = "Timer Shadow Period Value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`prs::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`prs::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PrsSpec;
impl crate::RegisterSpec for PrsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`prs::R`](R) reader structure"]
impl crate::Readable for PrsSpec {}
#[doc = "`write(|w| ..)` method takes [`prs::W`](W) writer structure"]
impl crate::Writable for PrsSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PRS to value 0"]
impl crate::Resettable for PrsSpec {
    const RESET_VALUE: u32 = 0;
}
