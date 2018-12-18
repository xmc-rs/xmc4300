#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Ethernet 0 Port Control Register"]
    pub eth0_con: ETH0_CON,
}
#[doc = "Ethernet 0 Port Control Register"]
pub struct ETH0_CON {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Ethernet 0 Port Control Register"]
pub mod eth0_con;
