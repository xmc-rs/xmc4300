#[doc = "Register `DC_SYS_TIME_DELAY` reader"]
pub type R = crate::R<DcSysTimeDelaySpec>;
#[doc = "Register `DC_SYS_TIME_DELAY` writer"]
pub type W = crate::W<DcSysTimeDelaySpec>;
#[doc = "Field `CLK_DELAY` reader - Delay between Reference Clock and the ESC"]
pub type ClkDelayR = crate::FieldReader<u32>;
#[doc = "Field `CLK_DELAY` writer - Delay between Reference Clock and the ESC"]
pub type ClkDelayW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Delay between Reference Clock and the ESC"]
    #[inline(always)]
    pub fn clk_delay(&self) -> ClkDelayR {
        ClkDelayR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Delay between Reference Clock and the ESC"]
    #[inline(always)]
    #[must_use]
    pub fn clk_delay(&mut self) -> ClkDelayW<DcSysTimeDelaySpec> {
        ClkDelayW::new(self, 0)
    }
}
#[doc = "System Time Delay\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dc_sys_time_delay::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dc_sys_time_delay::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DcSysTimeDelaySpec;
impl crate::RegisterSpec for DcSysTimeDelaySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dc_sys_time_delay::R`](R) reader structure"]
impl crate::Readable for DcSysTimeDelaySpec {}
#[doc = "`write(|w| ..)` method takes [`dc_sys_time_delay::W`](W) writer structure"]
impl crate::Writable for DcSysTimeDelaySpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DC_SYS_TIME_DELAY to value 0"]
impl crate::Resettable for DcSysTimeDelaySpec {
    const RESET_VALUE: u32 = 0;
}
