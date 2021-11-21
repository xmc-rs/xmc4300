#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - RCU Reset Status"]
    pub rststat: crate::Reg<rststat::RSTSTAT_SPEC>,
    #[doc = "0x04 - RCU Reset Set Register"]
    pub rstset: crate::Reg<rstset::RSTSET_SPEC>,
    #[doc = "0x08 - RCU Reset Clear Register"]
    pub rstclr: crate::Reg<rstclr::RSTCLR_SPEC>,
    #[doc = "0x0c - RCU Peripheral 0 Reset Status"]
    pub prstat0: crate::Reg<prstat0::PRSTAT0_SPEC>,
    #[doc = "0x10 - RCU Peripheral 0 Reset Set"]
    pub prset0: crate::Reg<prset0::PRSET0_SPEC>,
    #[doc = "0x14 - RCU Peripheral 0 Reset Clear"]
    pub prclr0: crate::Reg<prclr0::PRCLR0_SPEC>,
    #[doc = "0x18 - RCU Peripheral 1 Reset Status"]
    pub prstat1: crate::Reg<prstat1::PRSTAT1_SPEC>,
    #[doc = "0x1c - RCU Peripheral 1 Reset Set"]
    pub prset1: crate::Reg<prset1::PRSET1_SPEC>,
    #[doc = "0x20 - RCU Peripheral 1 Reset Clear"]
    pub prclr1: crate::Reg<prclr1::PRCLR1_SPEC>,
    #[doc = "0x24 - RCU Peripheral 2 Reset Status"]
    pub prstat2: crate::Reg<prstat2::PRSTAT2_SPEC>,
    #[doc = "0x28 - RCU Peripheral 2 Reset Set"]
    pub prset2: crate::Reg<prset2::PRSET2_SPEC>,
    #[doc = "0x2c - RCU Peripheral 2 Reset Clear"]
    pub prclr2: crate::Reg<prclr2::PRCLR2_SPEC>,
}
#[doc = "RSTSTAT register accessor: an alias for `Reg<RSTSTAT_SPEC>`"]
pub type RSTSTAT = crate::Reg<rststat::RSTSTAT_SPEC>;
#[doc = "RCU Reset Status"]
pub mod rststat;
#[doc = "RSTSET register accessor: an alias for `Reg<RSTSET_SPEC>`"]
pub type RSTSET = crate::Reg<rstset::RSTSET_SPEC>;
#[doc = "RCU Reset Set Register"]
pub mod rstset;
#[doc = "RSTCLR register accessor: an alias for `Reg<RSTCLR_SPEC>`"]
pub type RSTCLR = crate::Reg<rstclr::RSTCLR_SPEC>;
#[doc = "RCU Reset Clear Register"]
pub mod rstclr;
#[doc = "PRSTAT0 register accessor: an alias for `Reg<PRSTAT0_SPEC>`"]
pub type PRSTAT0 = crate::Reg<prstat0::PRSTAT0_SPEC>;
#[doc = "RCU Peripheral 0 Reset Status"]
pub mod prstat0;
#[doc = "PRSET0 register accessor: an alias for `Reg<PRSET0_SPEC>`"]
pub type PRSET0 = crate::Reg<prset0::PRSET0_SPEC>;
#[doc = "RCU Peripheral 0 Reset Set"]
pub mod prset0;
#[doc = "PRCLR0 register accessor: an alias for `Reg<PRCLR0_SPEC>`"]
pub type PRCLR0 = crate::Reg<prclr0::PRCLR0_SPEC>;
#[doc = "RCU Peripheral 0 Reset Clear"]
pub mod prclr0;
#[doc = "PRSTAT1 register accessor: an alias for `Reg<PRSTAT1_SPEC>`"]
pub type PRSTAT1 = crate::Reg<prstat1::PRSTAT1_SPEC>;
#[doc = "RCU Peripheral 1 Reset Status"]
pub mod prstat1;
#[doc = "PRSET1 register accessor: an alias for `Reg<PRSET1_SPEC>`"]
pub type PRSET1 = crate::Reg<prset1::PRSET1_SPEC>;
#[doc = "RCU Peripheral 1 Reset Set"]
pub mod prset1;
#[doc = "PRCLR1 register accessor: an alias for `Reg<PRCLR1_SPEC>`"]
pub type PRCLR1 = crate::Reg<prclr1::PRCLR1_SPEC>;
#[doc = "RCU Peripheral 1 Reset Clear"]
pub mod prclr1;
#[doc = "PRSTAT2 register accessor: an alias for `Reg<PRSTAT2_SPEC>`"]
pub type PRSTAT2 = crate::Reg<prstat2::PRSTAT2_SPEC>;
#[doc = "RCU Peripheral 2 Reset Status"]
pub mod prstat2;
#[doc = "PRSET2 register accessor: an alias for `Reg<PRSET2_SPEC>`"]
pub type PRSET2 = crate::Reg<prset2::PRSET2_SPEC>;
#[doc = "RCU Peripheral 2 Reset Set"]
pub mod prset2;
#[doc = "PRCLR2 register accessor: an alias for `Reg<PRCLR2_SPEC>`"]
pub type PRCLR2 = crate::Reg<prclr2::PRCLR2_SPEC>;
#[doc = "RCU Peripheral 2 Reset Clear"]
pub mod prclr2;
