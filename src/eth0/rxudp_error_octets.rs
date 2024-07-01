#[doc = "Register `RXUDP_ERROR_OCTETS` reader"]
pub type R = crate::R<RXUDP_ERROR_OCTETS_SPEC>;
#[doc = "Field `RXUDPERROCT` reader - This field indicates the number of bytes received in a UDP segment with checksum errors. This counter does not count the IP Header bytes. The Ethernet header, FCS, pad, or IP pad bytes are not included in this counter."]
pub type RXUDPERROCT_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - This field indicates the number of bytes received in a UDP segment with checksum errors. This counter does not count the IP Header bytes. The Ethernet header, FCS, pad, or IP pad bytes are not included in this counter."]
    #[inline(always)]
    pub fn rxudperroct(&self) -> RXUDPERROCT_R {
        RXUDPERROCT_R::new(self.bits)
    }
}
#[doc = "Receive UDP Error Octets Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rxudp_error_octets::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RXUDP_ERROR_OCTETS_SPEC;
impl crate::RegisterSpec for RXUDP_ERROR_OCTETS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxudp_error_octets::R`](R) reader structure"]
impl crate::Readable for RXUDP_ERROR_OCTETS_SPEC {}
#[doc = "`reset()` method sets RXUDP_ERROR_OCTETS to value 0"]
impl crate::Resettable for RXUDP_ERROR_OCTETS_SPEC {
    const RESET_VALUE: u32 = 0;
}
