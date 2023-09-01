#[doc = "Register `STATION_ADR` reader"]
pub type R = crate::R<STATION_ADR_SPEC>;
#[doc = "Field `NODE_ADDR` reader - Address used for node addressing (FPxx commands)"]
pub type NODE_ADDR_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Address used for node addressing (FPxx commands)"]
    #[inline(always)]
    pub fn node_addr(&self) -> NODE_ADDR_R {
        NODE_ADDR_R::new(self.bits)
    }
}
#[doc = "Configured Station Address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`station_adr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STATION_ADR_SPEC;
impl crate::RegisterSpec for STATION_ADR_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`station_adr::R`](R) reader structure"]
impl crate::Readable for STATION_ADR_SPEC {}
#[doc = "`reset()` method sets STATION_ADR to value 0"]
impl crate::Resettable for STATION_ADR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
