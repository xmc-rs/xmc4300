#[doc = "Register `RXICMP_ERROR_OCTETS` reader"]
pub type R = crate::R<RxicmpErrorOctetsSpec>;
#[doc = "Field `RXICMPERROCT` reader - Number of bytes received in an ICMP segment with checksum errors"]
pub type RxicmperroctR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Number of bytes received in an ICMP segment with checksum errors"]
    #[inline(always)]
    pub fn rxicmperroct(&self) -> RxicmperroctR {
        RxicmperroctR::new(self.bits)
    }
}
#[doc = "Receive ICMP Error Octets Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxicmp_error_octets::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxicmpErrorOctetsSpec;
impl crate::RegisterSpec for RxicmpErrorOctetsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxicmp_error_octets::R`](R) reader structure"]
impl crate::Readable for RxicmpErrorOctetsSpec {}
#[doc = "`reset()` method sets RXICMP_ERROR_OCTETS to value 0"]
impl crate::Resettable for RxicmpErrorOctetsSpec {
    const RESET_VALUE: u32 = 0;
}
