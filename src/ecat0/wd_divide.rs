#[doc = "Register `WD_DIVIDE` reader"]
pub type R = crate::R<WdDivideSpec>;
#[doc = "Register `WD_DIVIDE` writer"]
pub type W = crate::W<WdDivideSpec>;
#[doc = "Field `WD_DIV` reader - Watchdog divider"]
pub type WdDivR = crate::FieldReader<u16>;
#[doc = "Field `WD_DIV` writer - Watchdog divider"]
pub type WdDivW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Watchdog divider"]
    #[inline(always)]
    pub fn wd_div(&self) -> WdDivR {
        WdDivR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - Watchdog divider"]
    #[inline(always)]
    #[must_use]
    pub fn wd_div(&mut self) -> WdDivW<WdDivideSpec> {
        WdDivW::new(self, 0)
    }
}
#[doc = "Watchdog Divider\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wd_divide::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wd_divide::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WdDivideSpec;
impl crate::RegisterSpec for WdDivideSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`wd_divide::R`](R) reader structure"]
impl crate::Readable for WdDivideSpec {}
#[doc = "`write(|w| ..)` method takes [`wd_divide::W`](W) writer structure"]
impl crate::Writable for WdDivideSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets WD_DIVIDE to value 0x09c2"]
impl crate::Resettable for WdDivideSpec {
    const RESET_VALUE: u16 = 0x09c2;
}
