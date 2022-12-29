#[doc = r"Register block"]
#[repr(C)]
pub struct MO {
    #[doc = "0x00 - Message Object Function Control Register"]
    pub mofcr: MOFCR,
    #[doc = "0x04 - Message Object FIFO/Gateway Pointer Register"]
    pub mofgpr: MOFGPR,
    #[doc = "0x08 - Message Object Interrupt Pointer Register"]
    pub moipr: MOIPR,
    #[doc = "0x0c - Message Object Acceptance Mask Register"]
    pub moamr: MOAMR,
    #[doc = "0x10 - Message Object Data Register Low"]
    pub modatal: MODATAL,
    #[doc = "0x14 - Message Object Data Register High"]
    pub modatah: MODATAH,
    #[doc = "0x18 - Message Object Arbitration Register"]
    pub moar: MOAR,
    _reserved_7_moctr: [u8; 0x04],
}
impl MO {
    #[doc = "0x1c - Message Object Status Register"]
    #[inline(always)]
    pub const fn mostat(&self) -> &MOSTAT {
        unsafe { &*(self as *const Self).cast::<u8>().add(28usize).cast() }
    }
    #[doc = "0x1c - Message Object Control Register"]
    #[inline(always)]
    pub const fn moctr(&self) -> &MOCTR {
        unsafe { &*(self as *const Self).cast::<u8>().add(28usize).cast() }
    }
}
#[doc = "MOFCR (rw) register accessor: an alias for `Reg<MOFCR_SPEC>`"]
pub type MOFCR = crate::Reg<mofcr::MOFCR_SPEC>;
#[doc = "Message Object Function Control Register"]
pub mod mofcr;
#[doc = "MOFGPR (rw) register accessor: an alias for `Reg<MOFGPR_SPEC>`"]
pub type MOFGPR = crate::Reg<mofgpr::MOFGPR_SPEC>;
#[doc = "Message Object FIFO/Gateway Pointer Register"]
pub mod mofgpr;
#[doc = "MOIPR (rw) register accessor: an alias for `Reg<MOIPR_SPEC>`"]
pub type MOIPR = crate::Reg<moipr::MOIPR_SPEC>;
#[doc = "Message Object Interrupt Pointer Register"]
pub mod moipr;
#[doc = "MOAMR (rw) register accessor: an alias for `Reg<MOAMR_SPEC>`"]
pub type MOAMR = crate::Reg<moamr::MOAMR_SPEC>;
#[doc = "Message Object Acceptance Mask Register"]
pub mod moamr;
#[doc = "MODATAL (rw) register accessor: an alias for `Reg<MODATAL_SPEC>`"]
pub type MODATAL = crate::Reg<modatal::MODATAL_SPEC>;
#[doc = "Message Object Data Register Low"]
pub mod modatal;
#[doc = "MODATAH (rw) register accessor: an alias for `Reg<MODATAH_SPEC>`"]
pub type MODATAH = crate::Reg<modatah::MODATAH_SPEC>;
#[doc = "Message Object Data Register High"]
pub mod modatah;
#[doc = "MOAR (rw) register accessor: an alias for `Reg<MOAR_SPEC>`"]
pub type MOAR = crate::Reg<moar::MOAR_SPEC>;
#[doc = "Message Object Arbitration Register"]
pub mod moar;
#[doc = "MOCTR (w) register accessor: an alias for `Reg<MOCTR_SPEC>`"]
pub type MOCTR = crate::Reg<moctr::MOCTR_SPEC>;
#[doc = "Message Object Control Register"]
pub mod moctr;
#[doc = "MOSTAT (r) register accessor: an alias for `Reg<MOSTAT_SPEC>`"]
pub type MOSTAT = crate::Reg<mostat::MOSTAT_SPEC>;
#[doc = "Message Object Status Register"]
pub mod mostat;
