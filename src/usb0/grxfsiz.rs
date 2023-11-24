#[doc = "Register `GRXFSIZ` reader"]
pub type R = crate::R<GRXFSIZ_SPEC>;
#[doc = "Register `GRXFSIZ` writer"]
pub type W = crate::W<GRXFSIZ_SPEC>;
#[doc = "Field `RxFDep` reader - RxFIFO Depth"]
pub type RX_FDEP_R = crate::FieldReader<u16>;
#[doc = "Field `RxFDep` writer - RxFIFO Depth"]
pub type RX_FDEP_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - RxFIFO Depth"]
    #[inline(always)]
    pub fn rx_fdep(&self) -> RX_FDEP_R {
        RX_FDEP_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - RxFIFO Depth"]
    #[inline(always)]
    #[must_use]
    pub fn rx_fdep(&mut self) -> RX_FDEP_W<GRXFSIZ_SPEC> {
        RX_FDEP_W::new(self, 0)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Receive FIFO Size Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grxfsiz::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grxfsiz::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GRXFSIZ_SPEC;
impl crate::RegisterSpec for GRXFSIZ_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`grxfsiz::R`](R) reader structure"]
impl crate::Readable for GRXFSIZ_SPEC {}
#[doc = "`write(|w| ..)` method takes [`grxfsiz::W`](W) writer structure"]
impl crate::Writable for GRXFSIZ_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GRXFSIZ to value 0x011a"]
impl crate::Resettable for GRXFSIZ_SPEC {
    const RESET_VALUE: Self::Ux = 0x011a;
}
