#[doc = "Register `RXUDP_GOOD_FRAMES` reader"]
pub type R = crate::R<RxudpGoodFramesSpec>;
#[doc = "Field `RXUDPGDFRM` reader - This field indicates the number of good IP datagrams with a good UDP payload. This counter is not updated when the counter is incremented."]
pub type RxudpgdfrmR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - This field indicates the number of good IP datagrams with a good UDP payload. This counter is not updated when the counter is incremented."]
    #[inline(always)]
    pub fn rxudpgdfrm(&self) -> RxudpgdfrmR {
        RxudpgdfrmR::new(self.bits)
    }
}
#[doc = "RxUDP Good Frames Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxudp_good_frames::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxudpGoodFramesSpec;
impl crate::RegisterSpec for RxudpGoodFramesSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxudp_good_frames::R`](R) reader structure"]
impl crate::Readable for RxudpGoodFramesSpec {}
#[doc = "`reset()` method sets RXUDP_GOOD_FRAMES to value 0"]
impl crate::Resettable for RxudpGoodFramesSpec {
    const RESET_VALUE: u32 = 0;
}
