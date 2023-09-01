#[doc = "Register `DC_SYS_TIME` writer"]
pub type W = crate::W<WRITEMODE_DC_SYS_TIME_SPEC>;
#[doc = "Field `WRITE_ACCESS` writer - Write access"]
pub type WRITE_ACCESS_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl W {
    #[doc = "Bits 0:31 - Write access"]
    #[inline(always)]
    #[must_use]
    pub fn write_access(&mut self) -> WRITE_ACCESS_W<WRITEMODE_DC_SYS_TIME_SPEC, 0> {
        WRITE_ACCESS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "System Time \\[WRITE Mode\\]\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`writemode_dc_sys_time::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WRITEMODE_DC_SYS_TIME_SPEC;
impl crate::RegisterSpec for WRITEMODE_DC_SYS_TIME_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`writemode_dc_sys_time::W`](W) writer structure"]
impl crate::Writable for WRITEMODE_DC_SYS_TIME_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DC_SYS_TIME to value 0"]
impl crate::Resettable for WRITEMODE_DC_SYS_TIME_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
