#[doc = "Register `REMOTE_WAKE_UP_FRAME_FILTER` reader"]
pub type R = crate::R<REMOTE_WAKE_UP_FRAME_FILTER_SPEC>;
#[doc = "Register `REMOTE_WAKE_UP_FRAME_FILTER` writer"]
pub type W = crate::W<REMOTE_WAKE_UP_FRAME_FILTER_SPEC>;
#[doc = "Field `WKUPFRMFTR` reader - Remote Wake-Up Frame Filter"]
pub type WKUPFRMFTR_R = crate::FieldReader<u32>;
#[doc = "Field `WKUPFRMFTR` writer - Remote Wake-Up Frame Filter"]
pub type WKUPFRMFTR_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Remote Wake-Up Frame Filter"]
    #[inline(always)]
    pub fn wkupfrmftr(&self) -> WKUPFRMFTR_R {
        WKUPFRMFTR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Remote Wake-Up Frame Filter"]
    #[inline(always)]
    pub fn wkupfrmftr(&mut self) -> WKUPFRMFTR_W<REMOTE_WAKE_UP_FRAME_FILTER_SPEC> {
        WKUPFRMFTR_W::new(self, 0)
    }
}
#[doc = "Remote Wake Up Frame Filter Register\n\nYou can [`read`](crate::Reg::read) this register and get [`remote_wake_up_frame_filter::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`remote_wake_up_frame_filter::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct REMOTE_WAKE_UP_FRAME_FILTER_SPEC;
impl crate::RegisterSpec for REMOTE_WAKE_UP_FRAME_FILTER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`remote_wake_up_frame_filter::R`](R) reader structure"]
impl crate::Readable for REMOTE_WAKE_UP_FRAME_FILTER_SPEC {}
#[doc = "`write(|w| ..)` method takes [`remote_wake_up_frame_filter::W`](W) writer structure"]
impl crate::Writable for REMOTE_WAKE_UP_FRAME_FILTER_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets REMOTE_WAKE_UP_FRAME_FILTER to value 0"]
impl crate::Resettable for REMOTE_WAKE_UP_FRAME_FILTER_SPEC {
    const RESET_VALUE: u32 = 0;
}
