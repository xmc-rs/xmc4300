#[doc = "Register `RXIPV4_GOOD_FRAMES` reader"]
pub type R = crate::R<Rxipv4GoodFramesSpec>;
#[doc = "Field `RXIPV4GDFRM` reader - This field indicates the number of good IPv4 datagrams received with the TCP, UDP, or ICMP payload."]
pub type Rxipv4gdfrmR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - This field indicates the number of good IPv4 datagrams received with the TCP, UDP, or ICMP payload."]
    #[inline(always)]
    pub fn rxipv4gdfrm(&self) -> Rxipv4gdfrmR {
        Rxipv4gdfrmR::new(self.bits)
    }
}
#[doc = "RxIPv4 Good Frames Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxipv4_good_frames::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Rxipv4GoodFramesSpec;
impl crate::RegisterSpec for Rxipv4GoodFramesSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxipv4_good_frames::R`](R) reader structure"]
impl crate::Readable for Rxipv4GoodFramesSpec {}
#[doc = "`reset()` method sets RXIPV4_GOOD_FRAMES to value 0"]
impl crate::Resettable for Rxipv4GoodFramesSpec {
    const RESET_VALUE: u32 = 0;
}
