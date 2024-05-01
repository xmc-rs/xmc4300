#[doc = "Register `RXUDP_ERROR_FRAMES` reader"]
pub type R = crate::R<RxudpErrorFramesSpec>;
#[doc = "Field `RXUDPERRFRM` reader - This field indicates the number of good IP datagrams whose UDP payload has a checksum error."]
pub type RxudperrfrmR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - This field indicates the number of good IP datagrams whose UDP payload has a checksum error."]
    #[inline(always)]
    pub fn rxudperrfrm(&self) -> RxudperrfrmR {
        RxudperrfrmR::new(self.bits)
    }
}
#[doc = "RxUDP Error Frames Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxudp_error_frames::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxudpErrorFramesSpec;
impl crate::RegisterSpec for RxudpErrorFramesSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxudp_error_frames::R`](R) reader structure"]
impl crate::Readable for RxudpErrorFramesSpec {}
#[doc = "`reset()` method sets RXUDP_ERROR_FRAMES to value 0"]
impl crate::Resettable for RxudpErrorFramesSpec {
    const RESET_VALUE: u32 = 0;
}
