#[doc = "Register `RXICMP_ERROR_FRAMES` reader"]
pub type R = crate::R<RxicmpErrorFramesSpec>;
#[doc = "Field `RXICMPERRFRM` reader - This field indicates the number of good IP datagrams whose ICMP payload has a checksum error."]
pub type RxicmperrfrmR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - This field indicates the number of good IP datagrams whose ICMP payload has a checksum error."]
    #[inline(always)]
    pub fn rxicmperrfrm(&self) -> RxicmperrfrmR {
        RxicmperrfrmR::new(self.bits)
    }
}
#[doc = "RxICMP Error Frames Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxicmp_error_frames::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxicmpErrorFramesSpec;
impl crate::RegisterSpec for RxicmpErrorFramesSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxicmp_error_frames::R`](R) reader structure"]
impl crate::Readable for RxicmpErrorFramesSpec {}
#[doc = "`reset()` method sets RXICMP_ERROR_FRAMES to value 0"]
impl crate::Resettable for RxicmpErrorFramesSpec {
    const RESET_VALUE: u32 = 0;
}
