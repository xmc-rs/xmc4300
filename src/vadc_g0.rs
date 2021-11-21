#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x80],
    #[doc = "0x80 - Arbitration Configuration Register"]
    pub arbcfg: crate::Reg<arbcfg::ARBCFG_SPEC>,
    #[doc = "0x84 - Arbitration Priority Register"]
    pub arbpr: crate::Reg<arbpr::ARBPR_SPEC>,
    #[doc = "0x88 - Channel Assignment Register"]
    pub chass: crate::Reg<chass::CHASS_SPEC>,
    _reserved3: [u8; 0x14],
    #[doc = "0xa0..0xa8 - Input Class Register"]
    pub iclass: [crate::Reg<iclass::ICLASS_SPEC>; 2],
    _reserved4: [u8; 0x08],
    #[doc = "0xb0 - Alias Register"]
    pub alias: crate::Reg<alias::ALIAS_SPEC>,
    _reserved5: [u8; 0x04],
    #[doc = "0xb8 - Boundary Select Register"]
    pub bound: crate::Reg<bound::BOUND_SPEC>,
    _reserved6: [u8; 0x04],
    #[doc = "0xc0 - Synchronization Control Register"]
    pub synctr: crate::Reg<synctr::SYNCTR_SPEC>,
    _reserved7: [u8; 0x04],
    #[doc = "0xc8 - Boundary Flag Register"]
    pub bfl: crate::Reg<bfl::BFL_SPEC>,
    #[doc = "0xcc - Boundary Flag Software Register"]
    pub bfls: crate::Reg<bfls::BFLS_SPEC>,
    #[doc = "0xd0 - Boundary Flag Control Register"]
    pub bflc: crate::Reg<bflc::BFLC_SPEC>,
    #[doc = "0xd4 - Boundary Flag Node Pointer Register"]
    pub bflnp: crate::Reg<bflnp::BFLNP_SPEC>,
    _reserved11: [u8; 0x28],
    #[doc = "0x100 - Queue 0 Source Control Register"]
    pub qctrl0: crate::Reg<qctrl0::QCTRL0_SPEC>,
    #[doc = "0x104 - Queue 0 Mode Register"]
    pub qmr0: crate::Reg<qmr0::QMR0_SPEC>,
    #[doc = "0x108 - Queue 0 Status Register"]
    pub qsr0: crate::Reg<qsr0::QSR0_SPEC>,
    #[doc = "0x10c - Queue 0 Register 0"]
    pub q0r0: crate::Reg<q0r0::Q0R0_SPEC>,
    _reserved_15_qbur0: [u8; 0x04],
    _reserved16: [u8; 0x0c],
    #[doc = "0x120 - Autoscan Source Control Register"]
    pub asctrl: crate::Reg<asctrl::ASCTRL_SPEC>,
    #[doc = "0x124 - Autoscan Source Mode Register"]
    pub asmr: crate::Reg<asmr::ASMR_SPEC>,
    #[doc = "0x128 - Autoscan Source Channel Select Register"]
    pub assel: crate::Reg<assel::ASSEL_SPEC>,
    #[doc = "0x12c - Autoscan Source Pending Register"]
    pub aspnd: crate::Reg<aspnd::ASPND_SPEC>,
    _reserved20: [u8; 0x50],
    #[doc = "0x180 - Channel Event Flag Register"]
    pub ceflag: crate::Reg<ceflag::CEFLAG_SPEC>,
    #[doc = "0x184 - Result Event Flag Register"]
    pub reflag: crate::Reg<reflag::REFLAG_SPEC>,
    #[doc = "0x188 - Source Event Flag Register"]
    pub seflag: crate::Reg<seflag::SEFLAG_SPEC>,
    _reserved23: [u8; 0x04],
    #[doc = "0x190 - Channel Event Flag Clear Register"]
    pub cefclr: crate::Reg<cefclr::CEFCLR_SPEC>,
    #[doc = "0x194 - Result Event Flag Clear Register"]
    pub refclr: crate::Reg<refclr::REFCLR_SPEC>,
    #[doc = "0x198 - Source Event Flag Clear Register"]
    pub sefclr: crate::Reg<sefclr::SEFCLR_SPEC>,
    _reserved26: [u8; 0x04],
    #[doc = "0x1a0 - Channel Event Node Pointer Register 0"]
    pub cevnp0: crate::Reg<cevnp0::CEVNP0_SPEC>,
    _reserved27: [u8; 0x0c],
    #[doc = "0x1b0 - Result Event Node Pointer Register 0"]
    pub revnp0: crate::Reg<revnp0::REVNP0_SPEC>,
    #[doc = "0x1b4 - Result Event Node Pointer Register 1"]
    pub revnp1: crate::Reg<revnp1::REVNP1_SPEC>,
    _reserved29: [u8; 0x08],
    #[doc = "0x1c0 - Source Event Node Pointer Register"]
    pub sevnp: crate::Reg<sevnp::SEVNP_SPEC>,
    _reserved30: [u8; 0x04],
    #[doc = "0x1c8 - Service Request Software Activation Trigger"]
    pub sract: crate::Reg<sract::SRACT_SPEC>,
    _reserved31: [u8; 0x24],
    #[doc = "0x1f0 - E0ternal Multiplexer Control Register"]
    pub emuxctr: crate::Reg<emuxctr::EMUXCTR_SPEC>,
    _reserved32: [u8; 0x04],
    #[doc = "0x1f8 - Valid Flag Register"]
    pub vfr: crate::Reg<vfr::VFR_SPEC>,
    _reserved33: [u8; 0x04],
    #[doc = "0x200..0x220 - Channel Ctrl. Reg."]
    pub chctr: [crate::Reg<chctr::CHCTR_SPEC>; 8],
    _reserved34: [u8; 0x60],
    #[doc = "0x280..0x2c0 - Result Control Register"]
    pub rcr: [crate::Reg<rcr::RCR_SPEC>; 16],
    _reserved35: [u8; 0x40],
    #[doc = "0x300..0x340 - Result Register"]
    pub res: [crate::Reg<res::RES_SPEC>; 16],
    _reserved36: [u8; 0x40],
    #[doc = "0x380..0x3c0 - Result Register, Debug"]
    pub resd: [crate::Reg<resd::RESD_SPEC>; 16],
}
impl RegisterBlock {
    #[doc = "0x110 - Queue 0 Backup Register"]
    #[inline(always)]
    pub fn qbur0(&self) -> &crate::Reg<qbur0::QBUR0_SPEC> {
        unsafe { &*(((self as *const Self) as *const u8).add(272usize) as *const crate::Reg<qbur0::QBUR0_SPEC>) }
    }
    #[doc = "0x110 - Queue 0 Input Register"]
    #[inline(always)]
    pub fn qinr0(&self) -> &crate::Reg<qinr0::QINR0_SPEC> {
        unsafe { &*(((self as *const Self) as *const u8).add(272usize) as *const crate::Reg<qinr0::QINR0_SPEC>) }
    }
}
#[doc = "ARBCFG register accessor: an alias for `Reg<ARBCFG_SPEC>`"]
pub type ARBCFG = crate::Reg<arbcfg::ARBCFG_SPEC>;
#[doc = "Arbitration Configuration Register"]
pub mod arbcfg;
#[doc = "ARBPR register accessor: an alias for `Reg<ARBPR_SPEC>`"]
pub type ARBPR = crate::Reg<arbpr::ARBPR_SPEC>;
#[doc = "Arbitration Priority Register"]
pub mod arbpr;
#[doc = "CHASS register accessor: an alias for `Reg<CHASS_SPEC>`"]
pub type CHASS = crate::Reg<chass::CHASS_SPEC>;
#[doc = "Channel Assignment Register"]
pub mod chass;
#[doc = "ICLASS register accessor: an alias for `Reg<ICLASS_SPEC>`"]
pub type ICLASS = crate::Reg<iclass::ICLASS_SPEC>;
#[doc = "Input Class Register"]
pub mod iclass;
#[doc = "ALIAS register accessor: an alias for `Reg<ALIAS_SPEC>`"]
pub type ALIAS = crate::Reg<alias::ALIAS_SPEC>;
#[doc = "Alias Register"]
pub mod alias;
#[doc = "BOUND register accessor: an alias for `Reg<BOUND_SPEC>`"]
pub type BOUND = crate::Reg<bound::BOUND_SPEC>;
#[doc = "Boundary Select Register"]
pub mod bound;
#[doc = "SYNCTR register accessor: an alias for `Reg<SYNCTR_SPEC>`"]
pub type SYNCTR = crate::Reg<synctr::SYNCTR_SPEC>;
#[doc = "Synchronization Control Register"]
pub mod synctr;
#[doc = "BFL register accessor: an alias for `Reg<BFL_SPEC>`"]
pub type BFL = crate::Reg<bfl::BFL_SPEC>;
#[doc = "Boundary Flag Register"]
pub mod bfl;
#[doc = "BFLS register accessor: an alias for `Reg<BFLS_SPEC>`"]
pub type BFLS = crate::Reg<bfls::BFLS_SPEC>;
#[doc = "Boundary Flag Software Register"]
pub mod bfls;
#[doc = "BFLC register accessor: an alias for `Reg<BFLC_SPEC>`"]
pub type BFLC = crate::Reg<bflc::BFLC_SPEC>;
#[doc = "Boundary Flag Control Register"]
pub mod bflc;
#[doc = "BFLNP register accessor: an alias for `Reg<BFLNP_SPEC>`"]
pub type BFLNP = crate::Reg<bflnp::BFLNP_SPEC>;
#[doc = "Boundary Flag Node Pointer Register"]
pub mod bflnp;
#[doc = "QCTRL0 register accessor: an alias for `Reg<QCTRL0_SPEC>`"]
pub type QCTRL0 = crate::Reg<qctrl0::QCTRL0_SPEC>;
#[doc = "Queue 0 Source Control Register"]
pub mod qctrl0;
#[doc = "QMR0 register accessor: an alias for `Reg<QMR0_SPEC>`"]
pub type QMR0 = crate::Reg<qmr0::QMR0_SPEC>;
#[doc = "Queue 0 Mode Register"]
pub mod qmr0;
#[doc = "QSR0 register accessor: an alias for `Reg<QSR0_SPEC>`"]
pub type QSR0 = crate::Reg<qsr0::QSR0_SPEC>;
#[doc = "Queue 0 Status Register"]
pub mod qsr0;
#[doc = "Q0R0 register accessor: an alias for `Reg<Q0R0_SPEC>`"]
pub type Q0R0 = crate::Reg<q0r0::Q0R0_SPEC>;
#[doc = "Queue 0 Register 0"]
pub mod q0r0;
#[doc = "QINR0 register accessor: an alias for `Reg<QINR0_SPEC>`"]
pub type QINR0 = crate::Reg<qinr0::QINR0_SPEC>;
#[doc = "Queue 0 Input Register"]
pub mod qinr0;
#[doc = "QBUR0 register accessor: an alias for `Reg<QBUR0_SPEC>`"]
pub type QBUR0 = crate::Reg<qbur0::QBUR0_SPEC>;
#[doc = "Queue 0 Backup Register"]
pub mod qbur0;
#[doc = "ASCTRL register accessor: an alias for `Reg<ASCTRL_SPEC>`"]
pub type ASCTRL = crate::Reg<asctrl::ASCTRL_SPEC>;
#[doc = "Autoscan Source Control Register"]
pub mod asctrl;
#[doc = "ASMR register accessor: an alias for `Reg<ASMR_SPEC>`"]
pub type ASMR = crate::Reg<asmr::ASMR_SPEC>;
#[doc = "Autoscan Source Mode Register"]
pub mod asmr;
#[doc = "ASSEL register accessor: an alias for `Reg<ASSEL_SPEC>`"]
pub type ASSEL = crate::Reg<assel::ASSEL_SPEC>;
#[doc = "Autoscan Source Channel Select Register"]
pub mod assel;
#[doc = "ASPND register accessor: an alias for `Reg<ASPND_SPEC>`"]
pub type ASPND = crate::Reg<aspnd::ASPND_SPEC>;
#[doc = "Autoscan Source Pending Register"]
pub mod aspnd;
#[doc = "CEFLAG register accessor: an alias for `Reg<CEFLAG_SPEC>`"]
pub type CEFLAG = crate::Reg<ceflag::CEFLAG_SPEC>;
#[doc = "Channel Event Flag Register"]
pub mod ceflag;
#[doc = "REFLAG register accessor: an alias for `Reg<REFLAG_SPEC>`"]
pub type REFLAG = crate::Reg<reflag::REFLAG_SPEC>;
#[doc = "Result Event Flag Register"]
pub mod reflag;
#[doc = "SEFLAG register accessor: an alias for `Reg<SEFLAG_SPEC>`"]
pub type SEFLAG = crate::Reg<seflag::SEFLAG_SPEC>;
#[doc = "Source Event Flag Register"]
pub mod seflag;
#[doc = "CEFCLR register accessor: an alias for `Reg<CEFCLR_SPEC>`"]
pub type CEFCLR = crate::Reg<cefclr::CEFCLR_SPEC>;
#[doc = "Channel Event Flag Clear Register"]
pub mod cefclr;
#[doc = "REFCLR register accessor: an alias for `Reg<REFCLR_SPEC>`"]
pub type REFCLR = crate::Reg<refclr::REFCLR_SPEC>;
#[doc = "Result Event Flag Clear Register"]
pub mod refclr;
#[doc = "SEFCLR register accessor: an alias for `Reg<SEFCLR_SPEC>`"]
pub type SEFCLR = crate::Reg<sefclr::SEFCLR_SPEC>;
#[doc = "Source Event Flag Clear Register"]
pub mod sefclr;
#[doc = "CEVNP0 register accessor: an alias for `Reg<CEVNP0_SPEC>`"]
pub type CEVNP0 = crate::Reg<cevnp0::CEVNP0_SPEC>;
#[doc = "Channel Event Node Pointer Register 0"]
pub mod cevnp0;
#[doc = "REVNP0 register accessor: an alias for `Reg<REVNP0_SPEC>`"]
pub type REVNP0 = crate::Reg<revnp0::REVNP0_SPEC>;
#[doc = "Result Event Node Pointer Register 0"]
pub mod revnp0;
#[doc = "REVNP1 register accessor: an alias for `Reg<REVNP1_SPEC>`"]
pub type REVNP1 = crate::Reg<revnp1::REVNP1_SPEC>;
#[doc = "Result Event Node Pointer Register 1"]
pub mod revnp1;
#[doc = "SEVNP register accessor: an alias for `Reg<SEVNP_SPEC>`"]
pub type SEVNP = crate::Reg<sevnp::SEVNP_SPEC>;
#[doc = "Source Event Node Pointer Register"]
pub mod sevnp;
#[doc = "SRACT register accessor: an alias for `Reg<SRACT_SPEC>`"]
pub type SRACT = crate::Reg<sract::SRACT_SPEC>;
#[doc = "Service Request Software Activation Trigger"]
pub mod sract;
#[doc = "EMUXCTR register accessor: an alias for `Reg<EMUXCTR_SPEC>`"]
pub type EMUXCTR = crate::Reg<emuxctr::EMUXCTR_SPEC>;
#[doc = "E0ternal Multiplexer Control Register"]
pub mod emuxctr;
#[doc = "VFR register accessor: an alias for `Reg<VFR_SPEC>`"]
pub type VFR = crate::Reg<vfr::VFR_SPEC>;
#[doc = "Valid Flag Register"]
pub mod vfr;
#[doc = "CHCTR register accessor: an alias for `Reg<CHCTR_SPEC>`"]
pub type CHCTR = crate::Reg<chctr::CHCTR_SPEC>;
#[doc = "Channel Ctrl. Reg."]
pub mod chctr;
#[doc = "RCR register accessor: an alias for `Reg<RCR_SPEC>`"]
pub type RCR = crate::Reg<rcr::RCR_SPEC>;
#[doc = "Result Control Register"]
pub mod rcr;
#[doc = "RES register accessor: an alias for `Reg<RES_SPEC>`"]
pub type RES = crate::Reg<res::RES_SPEC>;
#[doc = "Result Register"]
pub mod res;
#[doc = "RESD register accessor: an alias for `Reg<RESD_SPEC>`"]
pub type RESD = crate::Reg<resd::RESD_SPEC>;
#[doc = "Result Register, Debug"]
pub mod resd;
