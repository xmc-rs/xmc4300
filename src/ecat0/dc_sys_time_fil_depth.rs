#[doc = "Register `DC_SYS_TIME_FIL_DEPTH` reader"]
pub type R = crate::R<DC_SYS_TIME_FIL_DEPTH_SPEC>;
#[doc = "Register `DC_SYS_TIME_FIL_DEPTH` writer"]
pub type W = crate::W<DC_SYS_TIME_FIL_DEPTH_SPEC>;
#[doc = "Field `FILTER_DEPTH` reader - Filter depth for averaging the received System Time deviation"]
pub type FILTER_DEPTH_R = crate::FieldReader;
#[doc = "Field `FILTER_DEPTH` writer - Filter depth for averaging the received System Time deviation"]
pub type FILTER_DEPTH_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - Filter depth for averaging the received System Time deviation"]
    #[inline(always)]
    pub fn filter_depth(&self) -> FILTER_DEPTH_R {
        FILTER_DEPTH_R::new(self.bits & 0x0f)
    }
}
impl W {
    #[doc = "Bits 0:3 - Filter depth for averaging the received System Time deviation"]
    #[inline(always)]
    pub fn filter_depth(&mut self) -> FILTER_DEPTH_W<DC_SYS_TIME_FIL_DEPTH_SPEC> {
        FILTER_DEPTH_W::new(self, 0)
    }
}
#[doc = "System Time Difference Filter Depth\n\nYou can [`read`](crate::Reg::read) this register and get [`dc_sys_time_fil_depth::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dc_sys_time_fil_depth::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DC_SYS_TIME_FIL_DEPTH_SPEC;
impl crate::RegisterSpec for DC_SYS_TIME_FIL_DEPTH_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`dc_sys_time_fil_depth::R`](R) reader structure"]
impl crate::Readable for DC_SYS_TIME_FIL_DEPTH_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dc_sys_time_fil_depth::W`](W) writer structure"]
impl crate::Writable for DC_SYS_TIME_FIL_DEPTH_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets DC_SYS_TIME_FIL_DEPTH to value 0x04"]
impl crate::Resettable for DC_SYS_TIME_FIL_DEPTH_SPEC {
    const RESET_VALUE: u8 = 0x04;
}
