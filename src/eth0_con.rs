#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Ethernet 0 Port Control Register"]
    pub eth0_con: ETH0_CON,
}
#[doc = "Ethernet 0 Port Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth0_con](eth0_con) module"]
pub type ETH0_CON = crate::Reg<u32, _ETH0_CON>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ETH0_CON;
#[doc = "`read()` method returns [eth0_con::R](eth0_con::R) reader structure"]
impl crate::Readable for ETH0_CON {}
#[doc = "`write(|w| ..)` method takes [eth0_con::W](eth0_con::W) writer structure"]
impl crate::Writable for ETH0_CON {}
#[doc = "Ethernet 0 Port Control Register"]
pub mod eth0_con;
