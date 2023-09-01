#[doc = "Register `RXIPV4_FRAGMENTED_OCTETS` reader"]
pub type R = crate::R<RXIPV4_FRAGMENTED_OCTETS_SPEC>;
#[doc = "Field `RXIPV4FRAGOCT` reader - This field indicates the number of bytes received in fragmented IPv4 datagrams. The value in the IPv4 headers Length field is used to update this counter."]
pub type RXIPV4FRAGOCT_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - This field indicates the number of bytes received in fragmented IPv4 datagrams. The value in the IPv4 headers Length field is used to update this counter."]
    #[inline(always)]
    pub fn rxipv4fragoct(&self) -> RXIPV4FRAGOCT_R {
        RXIPV4FRAGOCT_R::new(self.bits)
    }
}
#[doc = "Receive IPV4 Fragmented Octet Counter Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxipv4_fragmented_octets::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RXIPV4_FRAGMENTED_OCTETS_SPEC;
impl crate::RegisterSpec for RXIPV4_FRAGMENTED_OCTETS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxipv4_fragmented_octets::R`](R) reader structure"]
impl crate::Readable for RXIPV4_FRAGMENTED_OCTETS_SPEC {}
#[doc = "`reset()` method sets RXIPV4_FRAGMENTED_OCTETS to value 0"]
impl crate::Resettable for RXIPV4_FRAGMENTED_OCTETS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
