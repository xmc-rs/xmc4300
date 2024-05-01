#[doc = "Register `RXUDP_ERROR_OCTETS` reader"]
pub type R = crate::R<RxudpErrorOctetsSpec>;
#[doc = "Field `RXUDPERROCT` reader - This field indicates the number of bytes received in a UDP segment with checksum errors. This counter does not count the IP Header bytes. The Ethernet header, FCS, pad, or IP pad bytes are not included in this counter."]
pub type RxudperroctR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - This field indicates the number of bytes received in a UDP segment with checksum errors. This counter does not count the IP Header bytes. The Ethernet header, FCS, pad, or IP pad bytes are not included in this counter."]
    #[inline(always)]
    pub fn rxudperroct(&self) -> RxudperroctR {
        RxudperroctR::new(self.bits)
    }
}
#[doc = "Receive UDP Error Octets Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxudp_error_octets::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxudpErrorOctetsSpec;
impl crate::RegisterSpec for RxudpErrorOctetsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxudp_error_octets::R`](R) reader structure"]
impl crate::Readable for RxudpErrorOctetsSpec {}
#[doc = "`reset()` method sets RXUDP_ERROR_OCTETS to value 0"]
impl crate::Resettable for RxudpErrorOctetsSpec {
    const RESET_VALUE: u32 = 0;
}
