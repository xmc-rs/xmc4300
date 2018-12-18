#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - CAN Clock Control Register"]
    pub clc: CLC,
    _reserved0: [u8; 4usize],
    #[doc = "0x08 - Module Identification Register"]
    pub id: ID,
    #[doc = "0x0c - CAN Fractional Divider Register"]
    pub fdr: FDR,
    _reserved1: [u8; 240usize],
    #[doc = "0x100 - List Register"]
    pub list: [LIST; 16],
    #[doc = "0x140 - Message Pending Register"]
    pub mspnd: [MSPND; 8],
    _reserved2: [u8; 32usize],
    #[doc = "0x180 - Message Index Register"]
    pub msid: [MSID; 8],
    _reserved3: [u8; 32usize],
    #[doc = "0x1c0 - Message Index Mask Register"]
    pub msimask: MSIMASK,
    #[doc = "0x1c4 - Panel Control Register"]
    pub panctr: PANCTR,
    #[doc = "0x1c8 - Module Control Register"]
    pub mcr: MCR,
    #[doc = "0x1cc - Module Interrupt Trigger Register"]
    pub mitr: MITR,
}
#[doc = "CAN Clock Control Register"]
pub struct CLC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CAN Clock Control Register"]
pub mod clc;
#[doc = "Module Identification Register"]
pub struct ID {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Module Identification Register"]
pub mod id;
#[doc = "CAN Fractional Divider Register"]
pub struct FDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CAN Fractional Divider Register"]
pub mod fdr;
#[doc = "List Register"]
pub struct LIST {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "List Register"]
pub mod list;
#[doc = "Message Pending Register"]
pub struct MSPND {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Pending Register"]
pub mod mspnd;
#[doc = "Message Index Register"]
pub struct MSID {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Index Register"]
pub mod msid;
#[doc = "Message Index Mask Register"]
pub struct MSIMASK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Index Mask Register"]
pub mod msimask;
#[doc = "Panel Control Register"]
pub struct PANCTR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Panel Control Register"]
pub mod panctr;
#[doc = "Module Control Register"]
pub struct MCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Module Control Register"]
pub mod mcr;
#[doc = "Module Interrupt Trigger Register"]
pub struct MITR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Module Interrupt Trigger Register"]
pub mod mitr;
