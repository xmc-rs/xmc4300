#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Source Address Register"]
    pub sar: SAR,
    _reserved0: [u8; 4usize],
    #[doc = "0x08 - Destination Address Register"]
    pub dar: DAR,
    _reserved1: [u8; 4usize],
    #[doc = "0x10 - Linked List Pointer Register"]
    pub llp: LLP,
    _reserved2: [u8; 4usize],
    #[doc = "0x18 - Control Register Low"]
    pub ctll: CTLL,
    #[doc = "0x1c - Control Register High"]
    pub ctlh: CTLH,
    #[doc = "0x20 - Source Status Register"]
    pub sstat: SSTAT,
    _reserved3: [u8; 4usize],
    #[doc = "0x28 - Destination Status Register"]
    pub dstat: DSTAT,
    _reserved4: [u8; 4usize],
    #[doc = "0x30 - Source Status Address Register"]
    pub sstatar: SSTATAR,
    _reserved5: [u8; 4usize],
    #[doc = "0x38 - Destination Status Address Register"]
    pub dstatar: DSTATAR,
    _reserved6: [u8; 4usize],
    #[doc = "0x40 - Configuration Register Low"]
    pub cfgl: CFGL,
    #[doc = "0x44 - Configuration Register High"]
    pub cfgh: CFGH,
    #[doc = "0x48 - Source Gather Register"]
    pub sgr: SGR,
    _reserved7: [u8; 4usize],
    #[doc = "0x50 - Destination Scatter Register"]
    pub dsr: DSR,
}
#[doc = "Source Address Register"]
pub struct SAR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Source Address Register"]
pub mod sar;
#[doc = "Destination Address Register"]
pub struct DAR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Destination Address Register"]
pub mod dar;
#[doc = "Linked List Pointer Register"]
pub struct LLP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Linked List Pointer Register"]
pub mod llp;
#[doc = "Control Register Low"]
pub struct CTLL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Control Register Low"]
pub mod ctll;
#[doc = "Control Register High"]
pub struct CTLH {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Control Register High"]
pub mod ctlh;
#[doc = "Source Status Register"]
pub struct SSTAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Source Status Register"]
pub mod sstat;
#[doc = "Destination Status Register"]
pub struct DSTAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Destination Status Register"]
pub mod dstat;
#[doc = "Source Status Address Register"]
pub struct SSTATAR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Source Status Address Register"]
pub mod sstatar;
#[doc = "Destination Status Address Register"]
pub struct DSTATAR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Destination Status Address Register"]
pub mod dstatar;
#[doc = "Configuration Register Low"]
pub struct CFGL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Configuration Register Low"]
pub mod cfgl;
#[doc = "Configuration Register High"]
pub struct CFGH {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Configuration Register High"]
pub mod cfgh;
#[doc = "Source Gather Register"]
pub struct SGR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Source Gather Register"]
pub mod sgr;
#[doc = "Destination Scatter Register"]
pub struct DSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Destination Scatter Register"]
pub mod dsr;
