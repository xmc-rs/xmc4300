#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x1008],
    #[doc = "0x1008 - Flash Module Identification Register"]
    pub id: crate::Reg<id::ID_SPEC>,
    _reserved1: [u8; 0x04],
    #[doc = "0x1010 - Flash Status Register"]
    pub fsr: crate::Reg<fsr::FSR_SPEC>,
    #[doc = "0x1014 - Flash Configuration Register"]
    pub fcon: crate::Reg<fcon::FCON_SPEC>,
    #[doc = "0x1018 - Margin Control Register PFLASH"]
    pub marp: crate::Reg<marp::MARP_SPEC>,
    _reserved4: [u8; 0x04],
    #[doc = "0x1020 - Flash Protection Configuration Register User 0"]
    pub procon0: crate::Reg<procon0::PROCON0_SPEC>,
    #[doc = "0x1024 - Flash Protection Configuration Register User 1"]
    pub procon1: crate::Reg<procon1::PROCON1_SPEC>,
    #[doc = "0x1028 - Flash Protection Configuration Register User 2"]
    pub procon2: crate::Reg<procon2::PROCON2_SPEC>,
}
#[doc = "ID register accessor: an alias for `Reg<ID_SPEC>`"]
pub type ID = crate::Reg<id::ID_SPEC>;
#[doc = "Flash Module Identification Register"]
pub mod id;
#[doc = "FSR register accessor: an alias for `Reg<FSR_SPEC>`"]
pub type FSR = crate::Reg<fsr::FSR_SPEC>;
#[doc = "Flash Status Register"]
pub mod fsr;
#[doc = "FCON register accessor: an alias for `Reg<FCON_SPEC>`"]
pub type FCON = crate::Reg<fcon::FCON_SPEC>;
#[doc = "Flash Configuration Register"]
pub mod fcon;
#[doc = "MARP register accessor: an alias for `Reg<MARP_SPEC>`"]
pub type MARP = crate::Reg<marp::MARP_SPEC>;
#[doc = "Margin Control Register PFLASH"]
pub mod marp;
#[doc = "PROCON0 register accessor: an alias for `Reg<PROCON0_SPEC>`"]
pub type PROCON0 = crate::Reg<procon0::PROCON0_SPEC>;
#[doc = "Flash Protection Configuration Register User 0"]
pub mod procon0;
#[doc = "PROCON1 register accessor: an alias for `Reg<PROCON1_SPEC>`"]
pub type PROCON1 = crate::Reg<procon1::PROCON1_SPEC>;
#[doc = "Flash Protection Configuration Register User 1"]
pub mod procon1;
#[doc = "PROCON2 register accessor: an alias for `Reg<PROCON2_SPEC>`"]
pub type PROCON2 = crate::Reg<procon2::PROCON2_SPEC>;
#[doc = "Flash Protection Configuration Register User 2"]
pub mod procon2;
