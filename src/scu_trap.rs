#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Trap Status Register"]
    pub trapstat: crate::Reg<trapstat::TRAPSTAT_SPEC>,
    #[doc = "0x04 - Trap Raw Status Register"]
    pub trapraw: crate::Reg<trapraw::TRAPRAW_SPEC>,
    #[doc = "0x08 - Trap Disable Register"]
    pub trapdis: crate::Reg<trapdis::TRAPDIS_SPEC>,
    #[doc = "0x0c - Trap Clear Register"]
    pub trapclr: crate::Reg<trapclr::TRAPCLR_SPEC>,
    #[doc = "0x10 - Trap Set Register"]
    pub trapset: crate::Reg<trapset::TRAPSET_SPEC>,
}
#[doc = "TRAPSTAT register accessor: an alias for `Reg<TRAPSTAT_SPEC>`"]
pub type TRAPSTAT = crate::Reg<trapstat::TRAPSTAT_SPEC>;
#[doc = "Trap Status Register"]
pub mod trapstat;
#[doc = "TRAPRAW register accessor: an alias for `Reg<TRAPRAW_SPEC>`"]
pub type TRAPRAW = crate::Reg<trapraw::TRAPRAW_SPEC>;
#[doc = "Trap Raw Status Register"]
pub mod trapraw;
#[doc = "TRAPDIS register accessor: an alias for `Reg<TRAPDIS_SPEC>`"]
pub type TRAPDIS = crate::Reg<trapdis::TRAPDIS_SPEC>;
#[doc = "Trap Disable Register"]
pub mod trapdis;
#[doc = "TRAPCLR register accessor: an alias for `Reg<TRAPCLR_SPEC>`"]
pub type TRAPCLR = crate::Reg<trapclr::TRAPCLR_SPEC>;
#[doc = "Trap Clear Register"]
pub mod trapclr;
#[doc = "TRAPSET register accessor: an alias for `Reg<TRAPSET_SPEC>`"]
pub type TRAPSET = crate::Reg<trapset::TRAPSET_SPEC>;
#[doc = "Trap Set Register"]
pub mod trapset;
