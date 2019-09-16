#[doc = "Reader of register STATION_ADR"]
pub type R = crate::R<u16, super::STATION_ADR>;
#[doc = "Reader of field `NODE_ADDR`"]
pub type NODE_ADDR_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Address used for node addressing (FPxx commands)"]
    #[inline(always)]
    pub fn node_addr(&self) -> NODE_ADDR_R {
        NODE_ADDR_R::new((self.bits & 0xffff) as u16)
    }
}
