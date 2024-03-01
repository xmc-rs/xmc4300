#[doc = "Register `RXIPV4_HEADER_ERROR_OCTETS` reader"]
pub type R = crate::R<Rxipv4HeaderErrorOctetsSpec>;
#[doc = "Field `RXIPV4HDRERROCT` reader - This field indicates the number of bytes received in the IPv4 datagrams with header errors (checksum, length, or version mismatch). The value in the Length field of IPv4 header is used to update this counter."]
pub type Rxipv4hdrerroctR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - This field indicates the number of bytes received in the IPv4 datagrams with header errors (checksum, length, or version mismatch). The value in the Length field of IPv4 header is used to update this counter."]
    #[inline(always)]
    pub fn rxipv4hdrerroct(&self) -> Rxipv4hdrerroctR {
        Rxipv4hdrerroctR::new(self.bits)
    }
}
#[doc = "Receive IPV4 Header Error Octet Counter Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxipv4_header_error_octets::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Rxipv4HeaderErrorOctetsSpec;
impl crate::RegisterSpec for Rxipv4HeaderErrorOctetsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxipv4_header_error_octets::R`](R) reader structure"]
impl crate::Readable for Rxipv4HeaderErrorOctetsSpec {}
#[doc = "`reset()` method sets RXIPV4_HEADER_ERROR_OCTETS to value 0"]
impl crate::Resettable for Rxipv4HeaderErrorOctetsSpec {
    const RESET_VALUE: u32 = 0;
}
