#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Node Control Register"]
    pub ncr: NCR,
    #[doc = "0x04 - Node Status Register"]
    pub nsr: NSR,
    #[doc = "0x08 - Node Interrupt Pointer Register"]
    pub nipr: NIPR,
    #[doc = "0x0c - Node Port Control Register"]
    pub npcr: NPCR,
    #[doc = "0x10 - Node Bit Timing Register"]
    pub nbtr: NBTR,
    #[doc = "0x14 - Node Error Counter Register"]
    pub necnt: NECNT,
    #[doc = "0x18 - Node Frame Counter Register"]
    pub nfcr: NFCR,
}
#[doc = "Node Control Register"]
pub struct NCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Node Control Register"]
pub mod ncr;
#[doc = "Node Status Register"]
pub struct NSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Node Status Register"]
pub mod nsr;
#[doc = "Node Interrupt Pointer Register"]
pub struct NIPR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Node Interrupt Pointer Register"]
pub mod nipr;
#[doc = "Node Port Control Register"]
pub struct NPCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Node Port Control Register"]
pub mod npcr;
#[doc = "Node Bit Timing Register"]
pub struct NBTR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Node Bit Timing Register"]
pub mod nbtr;
#[doc = "Node Error Counter Register"]
pub struct NECNT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Node Error Counter Register"]
pub mod necnt;
#[doc = "Node Frame Counter Register"]
pub struct NFCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Node Frame Counter Register"]
pub mod nfcr;
