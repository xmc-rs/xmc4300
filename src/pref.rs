#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Prefetch Configuration Register"]
    pub pcon: PCON,
}
#[doc = "PCON (rw) register accessor: an alias for `Reg<PCON_SPEC>`"]
pub type PCON = crate::Reg<pcon::PCON_SPEC>;
#[doc = "Prefetch Configuration Register"]
pub mod pcon;
