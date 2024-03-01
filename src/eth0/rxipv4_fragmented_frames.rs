#[doc = "Register `RXIPV4_FRAGMENTED_FRAMES` reader"]
pub type R = crate::R<Rxipv4FragmentedFramesSpec>;
#[doc = "Field `RXIPV4FRAGFRM` reader - This field indicates the number of good IPv4 datagrams received with fragmentation."]
pub type Rxipv4fragfrmR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - This field indicates the number of good IPv4 datagrams received with fragmentation."]
    #[inline(always)]
    pub fn rxipv4fragfrm(&self) -> Rxipv4fragfrmR {
        Rxipv4fragfrmR::new(self.bits)
    }
}
#[doc = "Receive IPV4 Fragmented Frame Counter Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxipv4_fragmented_frames::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Rxipv4FragmentedFramesSpec;
impl crate::RegisterSpec for Rxipv4FragmentedFramesSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxipv4_fragmented_frames::R`](R) reader structure"]
impl crate::Readable for Rxipv4FragmentedFramesSpec {}
#[doc = "`reset()` method sets RXIPV4_FRAGMENTED_FRAMES to value 0"]
impl crate::Resettable for Rxipv4FragmentedFramesSpec {
    const RESET_VALUE: u32 = 0;
}
