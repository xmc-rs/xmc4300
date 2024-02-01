#[doc = "Register `DC_SYNC1_CYC_TIME` reader"]
pub type R = crate::R<DC_SYNC1_CYC_TIME_SPEC>;
#[doc = "Register `DC_SYNC1_CYC_TIME` writer"]
pub type W = crate::W<DC_SYNC1_CYC_TIME_SPEC>;
#[doc = "Field `TIME_SYNC1_SYNC0` reader - Time between SYNC1 pulses and SYNC0 pulse"]
pub type TIME_SYNC1_SYNC0_R = crate::FieldReader<u32>;
#[doc = "Field `TIME_SYNC1_SYNC0` writer - Time between SYNC1 pulses and SYNC0 pulse"]
pub type TIME_SYNC1_SYNC0_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Time between SYNC1 pulses and SYNC0 pulse"]
    #[inline(always)]
    pub fn time_sync1_sync0(&self) -> TIME_SYNC1_SYNC0_R {
        TIME_SYNC1_SYNC0_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Time between SYNC1 pulses and SYNC0 pulse"]
    #[inline(always)]
    #[must_use]
    pub fn time_sync1_sync0(&mut self) -> TIME_SYNC1_SYNC0_W<DC_SYNC1_CYC_TIME_SPEC> {
        TIME_SYNC1_SYNC0_W::new(self, 0)
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
#[doc = "SYNC1 Cycle Time\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dc_sync1_cyc_time::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dc_sync1_cyc_time::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DC_SYNC1_CYC_TIME_SPEC;
impl crate::RegisterSpec for DC_SYNC1_CYC_TIME_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dc_sync1_cyc_time::R`](R) reader structure"]
impl crate::Readable for DC_SYNC1_CYC_TIME_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dc_sync1_cyc_time::W`](W) writer structure"]
impl crate::Writable for DC_SYNC1_CYC_TIME_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DC_SYNC1_CYC_TIME to value 0"]
impl crate::Resettable for DC_SYNC1_CYC_TIME_SPEC {
    const RESET_VALUE: u32 = 0;
}
