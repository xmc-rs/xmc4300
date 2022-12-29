#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Ethernet 0 Port Control Register"]
    pub eth0_con: ETH0_CON,
}
#[doc = "ETH0_CON (rw) register accessor: an alias for `Reg<ETH0_CON_SPEC>`"]
pub type ETH0_CON = crate::Reg<eth0_con::ETH0_CON_SPEC>;
#[doc = "Ethernet 0 Port Control Register"]
pub mod eth0_con;
