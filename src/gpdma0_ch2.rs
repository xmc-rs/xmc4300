#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Source Address Register"]
    pub sar: SAR,
    _reserved0: [u8; 4usize],
    #[doc = "0x08 - Destination Address Register"]
    pub dar: DAR,
    _reserved1: [u8; 12usize],
    #[doc = "0x18 - Control Register Low"]
    pub ctll: CTLL,
    #[doc = "0x1c - Control Register High"]
    pub ctlh: CTLH,
    _reserved2: [u8; 32usize],
    #[doc = "0x40 - Configuration Register Low"]
    pub cfgl: CFGL,
    #[doc = "0x44 - Configuration Register High"]
    pub cfgh: CFGH,
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
