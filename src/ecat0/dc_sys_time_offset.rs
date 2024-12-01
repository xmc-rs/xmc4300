#[doc = "Register `DC_SYS_TIME_OFFSET[%s]` reader"]
pub type R = crate::R<DC_SYS_TIME_OFFSET_SPEC>;
#[doc = "Register `DC_SYS_TIME_OFFSET[%s]` writer"]
pub type W = crate::W<DC_SYS_TIME_OFFSET_SPEC>;
#[doc = "Field `DC_SYS_TIME_OFFSET` reader - Difference between local time and System Time"]
pub type DC_SYS_TIME_OFFSET_R = crate::FieldReader<u32>;
#[doc = "Field `DC_SYS_TIME_OFFSET` writer - Difference between local time and System Time"]
pub type DC_SYS_TIME_OFFSET_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Difference between local time and System Time"]
    #[inline(always)]
    pub fn dc_sys_time_offset(&self) -> DC_SYS_TIME_OFFSET_R {
        DC_SYS_TIME_OFFSET_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Difference between local time and System Time"]
    #[inline(always)]
    pub fn dc_sys_time_offset(&mut self) -> DC_SYS_TIME_OFFSET_W<DC_SYS_TIME_OFFSET_SPEC> {
        DC_SYS_TIME_OFFSET_W::new(self, 0)
    }
}
#[doc = "Difference between local time and System Time\n\nYou can [`read`](crate::Reg::read) this register and get [`dc_sys_time_offset::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dc_sys_time_offset::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DC_SYS_TIME_OFFSET_SPEC;
impl crate::RegisterSpec for DC_SYS_TIME_OFFSET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dc_sys_time_offset::R`](R) reader structure"]
impl crate::Readable for DC_SYS_TIME_OFFSET_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dc_sys_time_offset::W`](W) writer structure"]
impl crate::Writable for DC_SYS_TIME_OFFSET_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DC_SYS_TIME_OFFSET[%s]
to value 0"]
impl crate::Resettable for DC_SYS_TIME_OFFSET_SPEC {
    const RESET_VALUE: u32 = 0;
}
