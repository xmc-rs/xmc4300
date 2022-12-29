#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x80],
    #[doc = "0x80 - Arbitration Configuration Register"]
    pub arbcfg: ARBCFG,
    #[doc = "0x84 - Arbitration Priority Register"]
    pub arbpr: ARBPR,
    #[doc = "0x88 - Channel Assignment Register"]
    pub chass: CHASS,
    _reserved3: [u8; 0x14],
    #[doc = "0xa0..0xa8 - Input Class Register"]
    pub iclass: [ICLASS; 2],
    _reserved4: [u8; 0x08],
    #[doc = "0xb0 - Alias Register"]
    pub alias: ALIAS,
    _reserved5: [u8; 0x04],
    #[doc = "0xb8 - Boundary Select Register"]
    pub bound: BOUND,
    _reserved6: [u8; 0x04],
    #[doc = "0xc0 - Synchronization Control Register"]
    pub synctr: SYNCTR,
    _reserved7: [u8; 0x04],
    #[doc = "0xc8 - Boundary Flag Register"]
    pub bfl: BFL,
    #[doc = "0xcc - Boundary Flag Software Register"]
    pub bfls: BFLS,
    #[doc = "0xd0 - Boundary Flag Control Register"]
    pub bflc: BFLC,
    #[doc = "0xd4 - Boundary Flag Node Pointer Register"]
    pub bflnp: BFLNP,
    _reserved11: [u8; 0x28],
    #[doc = "0x100 - Queue 0 Source Control Register"]
    pub qctrl0: QCTRL0,
    #[doc = "0x104 - Queue 0 Mode Register"]
    pub qmr0: QMR0,
    #[doc = "0x108 - Queue 0 Status Register"]
    pub qsr0: QSR0,
    #[doc = "0x10c - Queue 0 Register 0"]
    pub q0r0: Q0R0,
    _reserved_15_qbur0: [u8; 0x04],
    _reserved16: [u8; 0x0c],
    #[doc = "0x120 - Autoscan Source Control Register"]
    pub asctrl: ASCTRL,
    #[doc = "0x124 - Autoscan Source Mode Register"]
    pub asmr: ASMR,
    #[doc = "0x128 - Autoscan Source Channel Select Register"]
    pub assel: ASSEL,
    #[doc = "0x12c - Autoscan Source Pending Register"]
    pub aspnd: ASPND,
    _reserved20: [u8; 0x50],
    #[doc = "0x180 - Channel Event Flag Register"]
    pub ceflag: CEFLAG,
    #[doc = "0x184 - Result Event Flag Register"]
    pub reflag: REFLAG,
    #[doc = "0x188 - Source Event Flag Register"]
    pub seflag: SEFLAG,
    _reserved23: [u8; 0x04],
    #[doc = "0x190 - Channel Event Flag Clear Register"]
    pub cefclr: CEFCLR,
    #[doc = "0x194 - Result Event Flag Clear Register"]
    pub refclr: REFCLR,
    #[doc = "0x198 - Source Event Flag Clear Register"]
    pub sefclr: SEFCLR,
    _reserved26: [u8; 0x04],
    #[doc = "0x1a0 - Channel Event Node Pointer Register 0"]
    pub cevnp0: CEVNP0,
    _reserved27: [u8; 0x0c],
    #[doc = "0x1b0 - Result Event Node Pointer Register 0"]
    pub revnp0: REVNP0,
    #[doc = "0x1b4 - Result Event Node Pointer Register 1"]
    pub revnp1: REVNP1,
    _reserved29: [u8; 0x08],
    #[doc = "0x1c0 - Source Event Node Pointer Register"]
    pub sevnp: SEVNP,
    _reserved30: [u8; 0x04],
    #[doc = "0x1c8 - Service Request Software Activation Trigger"]
    pub sract: SRACT,
    _reserved31: [u8; 0x24],
    #[doc = "0x1f0 - E0ternal Multiplexer Control Register"]
    pub emuxctr: EMUXCTR,
    _reserved32: [u8; 0x04],
    #[doc = "0x1f8 - Valid Flag Register"]
    pub vfr: VFR,
    _reserved33: [u8; 0x04],
    #[doc = "0x200..0x220 - Channel Ctrl. Reg."]
    pub chctr: [CHCTR; 8],
    _reserved34: [u8; 0x60],
    #[doc = "0x280..0x2c0 - Result Control Register"]
    pub rcr: [RCR; 16],
    _reserved35: [u8; 0x40],
    #[doc = "0x300..0x340 - Result Register"]
    pub res: [RES; 16],
    _reserved36: [u8; 0x40],
    #[doc = "0x380..0x3c0 - Result Register, Debug"]
    pub resd: [RESD; 16],
}
impl RegisterBlock {
    #[doc = "0x110 - Queue 0 Backup Register"]
    #[inline(always)]
    pub const fn qbur0(&self) -> &QBUR0 {
        unsafe { &*(self as *const Self).cast::<u8>().add(272usize).cast() }
    }
    #[doc = "0x110 - Queue 0 Input Register"]
    #[inline(always)]
    pub const fn qinr0(&self) -> &QINR0 {
        unsafe { &*(self as *const Self).cast::<u8>().add(272usize).cast() }
    }
}
#[doc = "ARBCFG (rw) register accessor: an alias for `Reg<ARBCFG_SPEC>`"]
pub type ARBCFG = crate::Reg<arbcfg::ARBCFG_SPEC>;
#[doc = "Arbitration Configuration Register"]
pub mod arbcfg;
#[doc = "ARBPR (rw) register accessor: an alias for `Reg<ARBPR_SPEC>`"]
pub type ARBPR = crate::Reg<arbpr::ARBPR_SPEC>;
#[doc = "Arbitration Priority Register"]
pub mod arbpr;
#[doc = "CHASS (rw) register accessor: an alias for `Reg<CHASS_SPEC>`"]
pub type CHASS = crate::Reg<chass::CHASS_SPEC>;
#[doc = "Channel Assignment Register"]
pub mod chass;
#[doc = "ICLASS (rw) register accessor: an alias for `Reg<ICLASS_SPEC>`"]
pub type ICLASS = crate::Reg<iclass::ICLASS_SPEC>;
#[doc = "Input Class Register"]
pub mod iclass;
#[doc = "ALIAS (rw) register accessor: an alias for `Reg<ALIAS_SPEC>`"]
pub type ALIAS = crate::Reg<alias::ALIAS_SPEC>;
#[doc = "Alias Register"]
pub mod alias;
#[doc = "BOUND (rw) register accessor: an alias for `Reg<BOUND_SPEC>`"]
pub type BOUND = crate::Reg<bound::BOUND_SPEC>;
#[doc = "Boundary Select Register"]
pub mod bound;
#[doc = "SYNCTR (rw) register accessor: an alias for `Reg<SYNCTR_SPEC>`"]
pub type SYNCTR = crate::Reg<synctr::SYNCTR_SPEC>;
#[doc = "Synchronization Control Register"]
pub mod synctr;
#[doc = "BFL (rw) register accessor: an alias for `Reg<BFL_SPEC>`"]
pub type BFL = crate::Reg<bfl::BFL_SPEC>;
#[doc = "Boundary Flag Register"]
pub mod bfl;
#[doc = "BFLS (w) register accessor: an alias for `Reg<BFLS_SPEC>`"]
pub type BFLS = crate::Reg<bfls::BFLS_SPEC>;
#[doc = "Boundary Flag Software Register"]
pub mod bfls;
#[doc = "BFLC (rw) register accessor: an alias for `Reg<BFLC_SPEC>`"]
pub type BFLC = crate::Reg<bflc::BFLC_SPEC>;
#[doc = "Boundary Flag Control Register"]
pub mod bflc;
#[doc = "BFLNP (rw) register accessor: an alias for `Reg<BFLNP_SPEC>`"]
pub type BFLNP = crate::Reg<bflnp::BFLNP_SPEC>;
#[doc = "Boundary Flag Node Pointer Register"]
pub mod bflnp;
#[doc = "QCTRL0 (rw) register accessor: an alias for `Reg<QCTRL0_SPEC>`"]
pub type QCTRL0 = crate::Reg<qctrl0::QCTRL0_SPEC>;
#[doc = "Queue 0 Source Control Register"]
pub mod qctrl0;
#[doc = "QMR0 (rw) register accessor: an alias for `Reg<QMR0_SPEC>`"]
pub type QMR0 = crate::Reg<qmr0::QMR0_SPEC>;
#[doc = "Queue 0 Mode Register"]
pub mod qmr0;
#[doc = "QSR0 (r) register accessor: an alias for `Reg<QSR0_SPEC>`"]
pub type QSR0 = crate::Reg<qsr0::QSR0_SPEC>;
#[doc = "Queue 0 Status Register"]
pub mod qsr0;
#[doc = "Q0R0 (r) register accessor: an alias for `Reg<Q0R0_SPEC>`"]
pub type Q0R0 = crate::Reg<q0r0::Q0R0_SPEC>;
#[doc = "Queue 0 Register 0"]
pub mod q0r0;
#[doc = "QINR0 (w) register accessor: an alias for `Reg<QINR0_SPEC>`"]
pub type QINR0 = crate::Reg<qinr0::QINR0_SPEC>;
#[doc = "Queue 0 Input Register"]
pub mod qinr0;
#[doc = "QBUR0 (r) register accessor: an alias for `Reg<QBUR0_SPEC>`"]
pub type QBUR0 = crate::Reg<qbur0::QBUR0_SPEC>;
#[doc = "Queue 0 Backup Register"]
pub mod qbur0;
#[doc = "ASCTRL (rw) register accessor: an alias for `Reg<ASCTRL_SPEC>`"]
pub type ASCTRL = crate::Reg<asctrl::ASCTRL_SPEC>;
#[doc = "Autoscan Source Control Register"]
pub mod asctrl;
#[doc = "ASMR (rw) register accessor: an alias for `Reg<ASMR_SPEC>`"]
pub type ASMR = crate::Reg<asmr::ASMR_SPEC>;
#[doc = "Autoscan Source Mode Register"]
pub mod asmr;
#[doc = "ASSEL (rw) register accessor: an alias for `Reg<ASSEL_SPEC>`"]
pub type ASSEL = crate::Reg<assel::ASSEL_SPEC>;
#[doc = "Autoscan Source Channel Select Register"]
pub mod assel;
#[doc = "ASPND (rw) register accessor: an alias for `Reg<ASPND_SPEC>`"]
pub type ASPND = crate::Reg<aspnd::ASPND_SPEC>;
#[doc = "Autoscan Source Pending Register"]
pub mod aspnd;
#[doc = "CEFLAG (rw) register accessor: an alias for `Reg<CEFLAG_SPEC>`"]
pub type CEFLAG = crate::Reg<ceflag::CEFLAG_SPEC>;
#[doc = "Channel Event Flag Register"]
pub mod ceflag;
#[doc = "REFLAG (rw) register accessor: an alias for `Reg<REFLAG_SPEC>`"]
pub type REFLAG = crate::Reg<reflag::REFLAG_SPEC>;
#[doc = "Result Event Flag Register"]
pub mod reflag;
#[doc = "SEFLAG (rw) register accessor: an alias for `Reg<SEFLAG_SPEC>`"]
pub type SEFLAG = crate::Reg<seflag::SEFLAG_SPEC>;
#[doc = "Source Event Flag Register"]
pub mod seflag;
#[doc = "CEFCLR (w) register accessor: an alias for `Reg<CEFCLR_SPEC>`"]
pub type CEFCLR = crate::Reg<cefclr::CEFCLR_SPEC>;
#[doc = "Channel Event Flag Clear Register"]
pub mod cefclr;
#[doc = "REFCLR (w) register accessor: an alias for `Reg<REFCLR_SPEC>`"]
pub type REFCLR = crate::Reg<refclr::REFCLR_SPEC>;
#[doc = "Result Event Flag Clear Register"]
pub mod refclr;
#[doc = "SEFCLR (w) register accessor: an alias for `Reg<SEFCLR_SPEC>`"]
pub type SEFCLR = crate::Reg<sefclr::SEFCLR_SPEC>;
#[doc = "Source Event Flag Clear Register"]
pub mod sefclr;
#[doc = "CEVNP0 (rw) register accessor: an alias for `Reg<CEVNP0_SPEC>`"]
pub type CEVNP0 = crate::Reg<cevnp0::CEVNP0_SPEC>;
#[doc = "Channel Event Node Pointer Register 0"]
pub mod cevnp0;
#[doc = "REVNP0 (rw) register accessor: an alias for `Reg<REVNP0_SPEC>`"]
pub type REVNP0 = crate::Reg<revnp0::REVNP0_SPEC>;
#[doc = "Result Event Node Pointer Register 0"]
pub mod revnp0;
#[doc = "REVNP1 (rw) register accessor: an alias for `Reg<REVNP1_SPEC>`"]
pub type REVNP1 = crate::Reg<revnp1::REVNP1_SPEC>;
#[doc = "Result Event Node Pointer Register 1"]
pub mod revnp1;
#[doc = "SEVNP (rw) register accessor: an alias for `Reg<SEVNP_SPEC>`"]
pub type SEVNP = crate::Reg<sevnp::SEVNP_SPEC>;
#[doc = "Source Event Node Pointer Register"]
pub mod sevnp;
#[doc = "SRACT (w) register accessor: an alias for `Reg<SRACT_SPEC>`"]
pub type SRACT = crate::Reg<sract::SRACT_SPEC>;
#[doc = "Service Request Software Activation Trigger"]
pub mod sract;
#[doc = "EMUXCTR (rw) register accessor: an alias for `Reg<EMUXCTR_SPEC>`"]
pub type EMUXCTR = crate::Reg<emuxctr::EMUXCTR_SPEC>;
#[doc = "E0ternal Multiplexer Control Register"]
pub mod emuxctr;
#[doc = "VFR (rw) register accessor: an alias for `Reg<VFR_SPEC>`"]
pub type VFR = crate::Reg<vfr::VFR_SPEC>;
#[doc = "Valid Flag Register"]
pub mod vfr;
#[doc = "CHCTR (rw) register accessor: an alias for `Reg<CHCTR_SPEC>`"]
pub type CHCTR = crate::Reg<chctr::CHCTR_SPEC>;
#[doc = "Channel Ctrl. Reg."]
pub mod chctr;
#[doc = "RCR (rw) register accessor: an alias for `Reg<RCR_SPEC>`"]
pub type RCR = crate::Reg<rcr::RCR_SPEC>;
#[doc = "Result Control Register"]
pub mod rcr;
#[doc = "RES (rw) register accessor: an alias for `Reg<RES_SPEC>`"]
pub type RES = crate::Reg<res::RES_SPEC>;
#[doc = "Result Register"]
pub mod res;
#[doc = "RESD (r) register accessor: an alias for `Reg<RESD_SPEC>`"]
pub type RESD = crate::Reg<resd::RESD_SPEC>;
#[doc = "Result Register, Debug"]
pub mod resd;
