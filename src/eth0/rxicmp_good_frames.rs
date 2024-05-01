#[doc = "Register `RXICMP_GOOD_FRAMES` reader"]
pub type R = crate::R<RxicmpGoodFramesSpec>;
#[doc = "Field `RXICMPGDFRM` reader - This field indicates the number of good IP datagrams with a good ICMP payload."]
pub type RxicmpgdfrmR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - This field indicates the number of good IP datagrams with a good ICMP payload."]
    #[inline(always)]
    pub fn rxicmpgdfrm(&self) -> RxicmpgdfrmR {
        RxicmpgdfrmR::new(self.bits)
    }
}
#[doc = "RxICMP Good Frames Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxicmp_good_frames::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxicmpGoodFramesSpec;
impl crate::RegisterSpec for RxicmpGoodFramesSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxicmp_good_frames::R`](R) reader structure"]
impl crate::Readable for RxicmpGoodFramesSpec {}
#[doc = "`reset()` method sets RXICMP_GOOD_FRAMES to value 0"]
impl crate::Resettable for RxicmpGoodFramesSpec {
    const RESET_VALUE: u32 = 0;
}
