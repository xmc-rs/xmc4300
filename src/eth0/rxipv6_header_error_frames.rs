#[doc = "Register `RXIPV6_HEADER_ERROR_FRAMES` reader"]
pub type R = crate::R<Rxipv6HeaderErrorFramesSpec>;
#[doc = "Field `RXIPV6HDRERRFRM` reader - This field indicates the number of IPv6 datagrams received with header errors (length or version mismatch)."]
pub type Rxipv6hdrerrfrmR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - This field indicates the number of IPv6 datagrams received with header errors (length or version mismatch)."]
    #[inline(always)]
    pub fn rxipv6hdrerrfrm(&self) -> Rxipv6hdrerrfrmR {
        Rxipv6hdrerrfrmR::new(self.bits)
    }
}
#[doc = "Receive IPV6 Header Error Frame Counter Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxipv6_header_error_frames::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Rxipv6HeaderErrorFramesSpec;
impl crate::RegisterSpec for Rxipv6HeaderErrorFramesSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxipv6_header_error_frames::R`](R) reader structure"]
impl crate::Readable for Rxipv6HeaderErrorFramesSpec {}
#[doc = "`reset()` method sets RXIPV6_HEADER_ERROR_FRAMES to value 0"]
impl crate::Resettable for Rxipv6HeaderErrorFramesSpec {
    const RESET_VALUE: u32 = 0;
}
