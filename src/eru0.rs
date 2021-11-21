#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Event Input Select"]
    pub exisel: crate::Reg<exisel::EXISEL_SPEC>,
    _reserved1: [u8; 0x0c],
    #[doc = "0x10..0x20 - Event Input Control"]
    pub exicon: [crate::Reg<exicon::EXICON_SPEC>; 4],
    #[doc = "0x20..0x30 - Event Output Trigger Control"]
    pub exocon: [crate::Reg<exocon::EXOCON_SPEC>; 4],
}
#[doc = "EXISEL register accessor: an alias for `Reg<EXISEL_SPEC>`"]
pub type EXISEL = crate::Reg<exisel::EXISEL_SPEC>;
#[doc = "Event Input Select"]
pub mod exisel;
#[doc = "EXICON register accessor: an alias for `Reg<EXICON_SPEC>`"]
pub type EXICON = crate::Reg<exicon::EXICON_SPEC>;
#[doc = "Event Input Control"]
pub mod exicon;
#[doc = "EXOCON register accessor: an alias for `Reg<EXOCON_SPEC>`"]
pub type EXOCON = crate::Reg<exocon::EXOCON_SPEC>;
#[doc = "Event Output Trigger Control"]
pub mod exocon;
