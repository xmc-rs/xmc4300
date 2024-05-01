#[doc = "Register `RXIPV4_HEADER_ERROR_FRAMES` reader"]
pub type R = crate::R<Rxipv4HeaderErrorFramesSpec>;
#[doc = "Field `RXIPV4HDRERRFRM` reader - This field indicates the number of IPv4 datagrams received with header errors (checksum, length, or version mismatch)."]
pub type Rxipv4hdrerrfrmR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - This field indicates the number of IPv4 datagrams received with header errors (checksum, length, or version mismatch)."]
    #[inline(always)]
    pub fn rxipv4hdrerrfrm(&self) -> Rxipv4hdrerrfrmR {
        Rxipv4hdrerrfrmR::new(self.bits)
    }
}
#[doc = "Receive IPV4 Header Error Frame Counter Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxipv4_header_error_frames::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Rxipv4HeaderErrorFramesSpec;
impl crate::RegisterSpec for Rxipv4HeaderErrorFramesSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxipv4_header_error_frames::R`](R) reader structure"]
impl crate::Readable for Rxipv4HeaderErrorFramesSpec {}
#[doc = "`reset()` method sets RXIPV4_HEADER_ERROR_FRAMES to value 0"]
impl crate::Resettable for Rxipv4HeaderErrorFramesSpec {
    const RESET_VALUE: u32 = 0;
}
