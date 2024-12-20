#[doc = "Register `DC_SYS_TIME_DELAY` reader"]
pub type R = crate::R<DC_SYS_TIME_DELAY_SPEC>;
#[doc = "Register `DC_SYS_TIME_DELAY` writer"]
pub type W = crate::W<DC_SYS_TIME_DELAY_SPEC>;
#[doc = "Field `CLK_DELAY` reader - Delay between Reference Clock and the ESC"]
pub type CLK_DELAY_R = crate::FieldReader<u32>;
#[doc = "Field `CLK_DELAY` writer - Delay between Reference Clock and the ESC"]
pub type CLK_DELAY_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Delay between Reference Clock and the ESC"]
    #[inline(always)]
    pub fn clk_delay(&self) -> CLK_DELAY_R {
        CLK_DELAY_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Delay between Reference Clock and the ESC"]
    #[inline(always)]
    pub fn clk_delay(&mut self) -> CLK_DELAY_W<DC_SYS_TIME_DELAY_SPEC> {
        CLK_DELAY_W::new(self, 0)
    }
}
#[doc = "System Time Delay\n\nYou can [`read`](crate::Reg::read) this register and get [`dc_sys_time_delay::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dc_sys_time_delay::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DC_SYS_TIME_DELAY_SPEC;
impl crate::RegisterSpec for DC_SYS_TIME_DELAY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dc_sys_time_delay::R`](R) reader structure"]
impl crate::Readable for DC_SYS_TIME_DELAY_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dc_sys_time_delay::W`](W) writer structure"]
impl crate::Writable for DC_SYS_TIME_DELAY_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DC_SYS_TIME_DELAY to value 0"]
impl crate::Resettable for DC_SYS_TIME_DELAY_SPEC {
    const RESET_VALUE: u32 = 0;
}
