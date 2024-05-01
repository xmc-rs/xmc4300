#[doc = "Register `STATION_ADR` reader"]
pub type R = crate::R<StationAdrSpec>;
#[doc = "Field `NODE_ADDR` reader - Address used for node addressing (FPxx commands)"]
pub type NodeAddrR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Address used for node addressing (FPxx commands)"]
    #[inline(always)]
    pub fn node_addr(&self) -> NodeAddrR {
        NodeAddrR::new(self.bits)
    }
}
#[doc = "Configured Station Address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`station_adr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StationAdrSpec;
impl crate::RegisterSpec for StationAdrSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`station_adr::R`](R) reader structure"]
impl crate::Readable for StationAdrSpec {}
#[doc = "`reset()` method sets STATION_ADR to value 0"]
impl crate::Resettable for StationAdrSpec {
    const RESET_VALUE: u16 = 0;
}
