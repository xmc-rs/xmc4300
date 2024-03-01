#[doc = "Register `REMOTE_WAKE_UP_FRAME_FILTER` reader"]
pub type R = crate::R<RemoteWakeUpFrameFilterSpec>;
#[doc = "Register `REMOTE_WAKE_UP_FRAME_FILTER` writer"]
pub type W = crate::W<RemoteWakeUpFrameFilterSpec>;
#[doc = "Field `WKUPFRMFTR` reader - Remote Wake-Up Frame Filter"]
pub type WkupfrmftrR = crate::FieldReader<u32>;
#[doc = "Field `WKUPFRMFTR` writer - Remote Wake-Up Frame Filter"]
pub type WkupfrmftrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Remote Wake-Up Frame Filter"]
    #[inline(always)]
    pub fn wkupfrmftr(&self) -> WkupfrmftrR {
        WkupfrmftrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Remote Wake-Up Frame Filter"]
    #[inline(always)]
    #[must_use]
    pub fn wkupfrmftr(&mut self) -> WkupfrmftrW<RemoteWakeUpFrameFilterSpec> {
        WkupfrmftrW::new(self, 0)
    }
}
#[doc = "Remote Wake Up Frame Filter Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`remote_wake_up_frame_filter::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`remote_wake_up_frame_filter::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RemoteWakeUpFrameFilterSpec;
impl crate::RegisterSpec for RemoteWakeUpFrameFilterSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`remote_wake_up_frame_filter::R`](R) reader structure"]
impl crate::Readable for RemoteWakeUpFrameFilterSpec {}
#[doc = "`write(|w| ..)` method takes [`remote_wake_up_frame_filter::W`](W) writer structure"]
impl crate::Writable for RemoteWakeUpFrameFilterSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets REMOTE_WAKE_UP_FRAME_FILTER to value 0"]
impl crate::Resettable for RemoteWakeUpFrameFilterSpec {
    const RESET_VALUE: u32 = 0;
}
