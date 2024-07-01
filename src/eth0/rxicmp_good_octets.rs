#[doc = "Register `RXICMP_GOOD_OCTETS` reader"]
pub type R = crate::R<RXICMP_GOOD_OCTETS_SPEC>;
#[doc = "Field `RXICMPGDOCT` reader - This field indicates the number of bytes received in a good ICMP segment. This counter does not count the IP Header bytes. The Ethernet header, FCS, pad, or IP pad bytes are not included in this counter."]
pub type RXICMPGDOCT_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - This field indicates the number of bytes received in a good ICMP segment. This counter does not count the IP Header bytes. The Ethernet header, FCS, pad, or IP pad bytes are not included in this counter."]
    #[inline(always)]
    pub fn rxicmpgdoct(&self) -> RXICMPGDOCT_R {
        RXICMPGDOCT_R::new(self.bits)
    }
}
#[doc = "Receive ICMP Good Octets Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rxicmp_good_octets::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RXICMP_GOOD_OCTETS_SPEC;
impl crate::RegisterSpec for RXICMP_GOOD_OCTETS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxicmp_good_octets::R`](R) reader structure"]
impl crate::Readable for RXICMP_GOOD_OCTETS_SPEC {}
#[doc = "`reset()` method sets RXICMP_GOOD_OCTETS to value 0"]
impl crate::Resettable for RXICMP_GOOD_OCTETS_SPEC {
    const RESET_VALUE: u32 = 0;
}
