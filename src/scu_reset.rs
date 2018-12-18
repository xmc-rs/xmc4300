#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - RCU Reset Status"]
    pub rststat: RSTSTAT,
    #[doc = "0x04 - RCU Reset Set Register"]
    pub rstset: RSTSET,
    #[doc = "0x08 - RCU Reset Clear Register"]
    pub rstclr: RSTCLR,
    #[doc = "0x0c - RCU Peripheral 0 Reset Status"]
    pub prstat0: PRSTAT0,
    #[doc = "0x10 - RCU Peripheral 0 Reset Set"]
    pub prset0: PRSET0,
    #[doc = "0x14 - RCU Peripheral 0 Reset Clear"]
    pub prclr0: PRCLR0,
    #[doc = "0x18 - RCU Peripheral 1 Reset Status"]
    pub prstat1: PRSTAT1,
    #[doc = "0x1c - RCU Peripheral 1 Reset Set"]
    pub prset1: PRSET1,
    #[doc = "0x20 - RCU Peripheral 1 Reset Clear"]
    pub prclr1: PRCLR1,
    #[doc = "0x24 - RCU Peripheral 2 Reset Status"]
    pub prstat2: PRSTAT2,
    #[doc = "0x28 - RCU Peripheral 2 Reset Set"]
    pub prset2: PRSET2,
    #[doc = "0x2c - RCU Peripheral 2 Reset Clear"]
    pub prclr2: PRCLR2,
}
#[doc = "RCU Reset Status"]
pub struct RSTSTAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RCU Reset Status"]
pub mod rststat;
#[doc = "RCU Reset Set Register"]
pub struct RSTSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RCU Reset Set Register"]
pub mod rstset;
#[doc = "RCU Reset Clear Register"]
pub struct RSTCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RCU Reset Clear Register"]
pub mod rstclr;
#[doc = "RCU Peripheral 0 Reset Status"]
pub struct PRSTAT0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RCU Peripheral 0 Reset Status"]
pub mod prstat0;
#[doc = "RCU Peripheral 0 Reset Set"]
pub struct PRSET0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RCU Peripheral 0 Reset Set"]
pub mod prset0;
#[doc = "RCU Peripheral 0 Reset Clear"]
pub struct PRCLR0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RCU Peripheral 0 Reset Clear"]
pub mod prclr0;
#[doc = "RCU Peripheral 1 Reset Status"]
pub struct PRSTAT1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RCU Peripheral 1 Reset Status"]
pub mod prstat1;
#[doc = "RCU Peripheral 1 Reset Set"]
pub struct PRSET1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RCU Peripheral 1 Reset Set"]
pub mod prset1;
#[doc = "RCU Peripheral 1 Reset Clear"]
pub struct PRCLR1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RCU Peripheral 1 Reset Clear"]
pub mod prclr1;
#[doc = "RCU Peripheral 2 Reset Status"]
pub struct PRSTAT2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RCU Peripheral 2 Reset Status"]
pub mod prstat2;
#[doc = "RCU Peripheral 2 Reset Set"]
pub struct PRSET2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RCU Peripheral 2 Reset Set"]
pub mod prset2;
#[doc = "RCU Peripheral 2 Reset Clear"]
pub struct PRCLR2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RCU Peripheral 2 Reset Clear"]
pub mod prclr2;
