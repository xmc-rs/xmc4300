#[doc = "Register `WD_DIVIDE` reader"]
pub type R = crate::R<WD_DIVIDE_SPEC>;
#[doc = "Register `WD_DIVIDE` writer"]
pub type W = crate::W<WD_DIVIDE_SPEC>;
#[doc = "Field `WD_DIV` reader - Watchdog divider"]
pub type WD_DIV_R = crate::FieldReader<u16>;
#[doc = "Field `WD_DIV` writer - Watchdog divider"]
pub type WD_DIV_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Watchdog divider"]
    #[inline(always)]
    pub fn wd_div(&self) -> WD_DIV_R {
        WD_DIV_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - Watchdog divider"]
    #[inline(always)]
    #[must_use]
    pub fn wd_div(&mut self) -> WD_DIV_W<WD_DIVIDE_SPEC> {
        WD_DIV_W::new(self, 0)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Watchdog Divider\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wd_divide::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wd_divide::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WD_DIVIDE_SPEC;
impl crate::RegisterSpec for WD_DIVIDE_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`wd_divide::R`](R) reader structure"]
impl crate::Readable for WD_DIVIDE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`wd_divide::W`](W) writer structure"]
impl crate::Writable for WD_DIVIDE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets WD_DIVIDE to value 0x09c2"]
impl crate::Resettable for WD_DIVIDE_SPEC {
    const RESET_VALUE: u16 = 0x09c2;
}
