#[doc = "Register `DC_CYC_START_TIME[%s]` reader"]
pub type R = crate::R<DC_CYC_START_TIME_SPEC>;
#[doc = "Register `DC_CYC_START_TIME[%s]` writer"]
pub type W = crate::W<DC_CYC_START_TIME_SPEC>;
#[doc = "Field `DC_CYC_START_TIME` reader - Start Time Cyclic Operation"]
pub type DC_CYC_START_TIME_R = crate::FieldReader<u32>;
#[doc = "Field `DC_CYC_START_TIME` writer - Start Time Cyclic Operation"]
pub type DC_CYC_START_TIME_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Start Time Cyclic Operation"]
    #[inline(always)]
    pub fn dc_cyc_start_time(&self) -> DC_CYC_START_TIME_R {
        DC_CYC_START_TIME_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Start Time Cyclic Operation"]
    #[inline(always)]
    #[must_use]
    pub fn dc_cyc_start_time(&mut self) -> DC_CYC_START_TIME_W<DC_CYC_START_TIME_SPEC> {
        DC_CYC_START_TIME_W::new(self, 0)
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
#[doc = "Start Time Cyclic Operation\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dc_cyc_start_time::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dc_cyc_start_time::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DC_CYC_START_TIME_SPEC;
impl crate::RegisterSpec for DC_CYC_START_TIME_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dc_cyc_start_time::R`](R) reader structure"]
impl crate::Readable for DC_CYC_START_TIME_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dc_cyc_start_time::W`](W) writer structure"]
impl crate::Writable for DC_CYC_START_TIME_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DC_CYC_START_TIME[%s]
to value 0"]
impl crate::Resettable for DC_CYC_START_TIME_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
