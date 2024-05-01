#[doc = "Register `RXIPV4_FRAGMENTED_OCTETS` reader"]
pub type R = crate::R<Rxipv4FragmentedOctetsSpec>;
#[doc = "Field `RXIPV4FRAGOCT` reader - This field indicates the number of bytes received in fragmented IPv4 datagrams. The value in the IPv4 headers Length field is used to update this counter."]
pub type Rxipv4fragoctR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - This field indicates the number of bytes received in fragmented IPv4 datagrams. The value in the IPv4 headers Length field is used to update this counter."]
    #[inline(always)]
    pub fn rxipv4fragoct(&self) -> Rxipv4fragoctR {
        Rxipv4fragoctR::new(self.bits)
    }
}
#[doc = "Receive IPV4 Fragmented Octet Counter Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxipv4_fragmented_octets::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Rxipv4FragmentedOctetsSpec;
impl crate::RegisterSpec for Rxipv4FragmentedOctetsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxipv4_fragmented_octets::R`](R) reader structure"]
impl crate::Readable for Rxipv4FragmentedOctetsSpec {}
#[doc = "`reset()` method sets RXIPV4_FRAGMENTED_OCTETS to value 0"]
impl crate::Resettable for Rxipv4FragmentedOctetsSpec {
    const RESET_VALUE: u32 = 0;
}
