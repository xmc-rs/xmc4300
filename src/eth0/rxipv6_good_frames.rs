#[doc = "Register `RXIPV6_GOOD_FRAMES` reader"]
pub type R = crate::R<Rxipv6GoodFramesSpec>;
#[doc = "Field `RXIPV6GDFRM` reader - This field indicates the number of good IPv6 datagrams received with TCP, UDP, or ICMP payloads."]
pub type Rxipv6gdfrmR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - This field indicates the number of good IPv6 datagrams received with TCP, UDP, or ICMP payloads."]
    #[inline(always)]
    pub fn rxipv6gdfrm(&self) -> Rxipv6gdfrmR {
        Rxipv6gdfrmR::new(self.bits)
    }
}
#[doc = "RxIPv6 Good Frames Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxipv6_good_frames::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Rxipv6GoodFramesSpec;
impl crate::RegisterSpec for Rxipv6GoodFramesSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxipv6_good_frames::R`](R) reader structure"]
impl crate::Readable for Rxipv6GoodFramesSpec {}
#[doc = "`reset()` method sets RXIPV6_GOOD_FRAMES to value 0"]
impl crate::Resettable for Rxipv6GoodFramesSpec {
    const RESET_VALUE: u32 = 0;
}
