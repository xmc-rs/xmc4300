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
#[doc = "ARBCFG (rw) register accessor: Arbitration Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`arbcfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`arbcfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@arbcfg`]
module"]
pub type ARBCFG = crate::Reg<arbcfg::ARBCFG_SPEC>;
#[doc = "Arbitration Configuration Register"]
pub mod arbcfg;
#[doc = "ARBPR (rw) register accessor: Arbitration Priority Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`arbpr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`arbpr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@arbpr`]
module"]
pub type ARBPR = crate::Reg<arbpr::ARBPR_SPEC>;
#[doc = "Arbitration Priority Register"]
pub mod arbpr;
#[doc = "CHASS (rw) register accessor: Channel Assignment Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`chass::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`chass::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chass`]
module"]
pub type CHASS = crate::Reg<chass::CHASS_SPEC>;
#[doc = "Channel Assignment Register"]
pub mod chass;
#[doc = "ICLASS (rw) register accessor: Input Class Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iclass::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iclass::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iclass`]
module"]
pub type ICLASS = crate::Reg<iclass::ICLASS_SPEC>;
#[doc = "Input Class Register"]
pub mod iclass;
#[doc = "ALIAS (rw) register accessor: Alias Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`alias::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`alias::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@alias`]
module"]
pub type ALIAS = crate::Reg<alias::ALIAS_SPEC>;
#[doc = "Alias Register"]
pub mod alias;
#[doc = "BOUND (rw) register accessor: Boundary Select Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bound::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bound::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bound`]
module"]
pub type BOUND = crate::Reg<bound::BOUND_SPEC>;
#[doc = "Boundary Select Register"]
pub mod bound;
#[doc = "SYNCTR (rw) register accessor: Synchronization Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`synctr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`synctr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@synctr`]
module"]
pub type SYNCTR = crate::Reg<synctr::SYNCTR_SPEC>;
#[doc = "Synchronization Control Register"]
pub mod synctr;
#[doc = "BFL (rw) register accessor: Boundary Flag Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bfl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bfl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bfl`]
module"]
pub type BFL = crate::Reg<bfl::BFL_SPEC>;
#[doc = "Boundary Flag Register"]
pub mod bfl;
#[doc = "BFLS (w) register accessor: Boundary Flag Software Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bfls::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bfls`]
module"]
pub type BFLS = crate::Reg<bfls::BFLS_SPEC>;
#[doc = "Boundary Flag Software Register"]
pub mod bfls;
#[doc = "BFLC (rw) register accessor: Boundary Flag Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bflc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bflc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bflc`]
module"]
pub type BFLC = crate::Reg<bflc::BFLC_SPEC>;
#[doc = "Boundary Flag Control Register"]
pub mod bflc;
#[doc = "BFLNP (rw) register accessor: Boundary Flag Node Pointer Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bflnp::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bflnp::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bflnp`]
module"]
pub type BFLNP = crate::Reg<bflnp::BFLNP_SPEC>;
#[doc = "Boundary Flag Node Pointer Register"]
pub mod bflnp;
#[doc = "QCTRL0 (rw) register accessor: Queue 0 Source Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`qctrl0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`qctrl0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@qctrl0`]
module"]
pub type QCTRL0 = crate::Reg<qctrl0::QCTRL0_SPEC>;
#[doc = "Queue 0 Source Control Register"]
pub mod qctrl0;
#[doc = "QMR0 (rw) register accessor: Queue 0 Mode Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`qmr0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`qmr0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@qmr0`]
module"]
pub type QMR0 = crate::Reg<qmr0::QMR0_SPEC>;
#[doc = "Queue 0 Mode Register"]
pub mod qmr0;
#[doc = "QSR0 (r) register accessor: Queue 0 Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`qsr0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@qsr0`]
module"]
pub type QSR0 = crate::Reg<qsr0::QSR0_SPEC>;
#[doc = "Queue 0 Status Register"]
pub mod qsr0;
#[doc = "Q0R0 (r) register accessor: Queue 0 Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`q0r0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@q0r0`]
module"]
pub type Q0R0 = crate::Reg<q0r0::Q0R0_SPEC>;
#[doc = "Queue 0 Register 0"]
pub mod q0r0;
#[doc = "QINR0 (w) register accessor: Queue 0 Input Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`qinr0::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@qinr0`]
module"]
pub type QINR0 = crate::Reg<qinr0::QINR0_SPEC>;
#[doc = "Queue 0 Input Register"]
pub mod qinr0;
#[doc = "QBUR0 (r) register accessor: Queue 0 Backup Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`qbur0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@qbur0`]
module"]
pub type QBUR0 = crate::Reg<qbur0::QBUR0_SPEC>;
#[doc = "Queue 0 Backup Register"]
pub mod qbur0;
#[doc = "ASCTRL (rw) register accessor: Autoscan Source Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`asctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`asctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@asctrl`]
module"]
pub type ASCTRL = crate::Reg<asctrl::ASCTRL_SPEC>;
#[doc = "Autoscan Source Control Register"]
pub mod asctrl;
#[doc = "ASMR (rw) register accessor: Autoscan Source Mode Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`asmr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`asmr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@asmr`]
module"]
pub type ASMR = crate::Reg<asmr::ASMR_SPEC>;
#[doc = "Autoscan Source Mode Register"]
pub mod asmr;
#[doc = "ASSEL (rw) register accessor: Autoscan Source Channel Select Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`assel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`assel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@assel`]
module"]
pub type ASSEL = crate::Reg<assel::ASSEL_SPEC>;
#[doc = "Autoscan Source Channel Select Register"]
pub mod assel;
#[doc = "ASPND (rw) register accessor: Autoscan Source Pending Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aspnd::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aspnd::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aspnd`]
module"]
pub type ASPND = crate::Reg<aspnd::ASPND_SPEC>;
#[doc = "Autoscan Source Pending Register"]
pub mod aspnd;
#[doc = "CEFLAG (rw) register accessor: Channel Event Flag Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ceflag::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ceflag::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ceflag`]
module"]
pub type CEFLAG = crate::Reg<ceflag::CEFLAG_SPEC>;
#[doc = "Channel Event Flag Register"]
pub mod ceflag;
#[doc = "REFLAG (rw) register accessor: Result Event Flag Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`reflag::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`reflag::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reflag`]
module"]
pub type REFLAG = crate::Reg<reflag::REFLAG_SPEC>;
#[doc = "Result Event Flag Register"]
pub mod reflag;
#[doc = "SEFLAG (rw) register accessor: Source Event Flag Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`seflag::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`seflag::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@seflag`]
module"]
pub type SEFLAG = crate::Reg<seflag::SEFLAG_SPEC>;
#[doc = "Source Event Flag Register"]
pub mod seflag;
#[doc = "CEFCLR (w) register accessor: Channel Event Flag Clear Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cefclr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cefclr`]
module"]
pub type CEFCLR = crate::Reg<cefclr::CEFCLR_SPEC>;
#[doc = "Channel Event Flag Clear Register"]
pub mod cefclr;
#[doc = "REFCLR (w) register accessor: Result Event Flag Clear Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`refclr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@refclr`]
module"]
pub type REFCLR = crate::Reg<refclr::REFCLR_SPEC>;
#[doc = "Result Event Flag Clear Register"]
pub mod refclr;
#[doc = "SEFCLR (w) register accessor: Source Event Flag Clear Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sefclr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sefclr`]
module"]
pub type SEFCLR = crate::Reg<sefclr::SEFCLR_SPEC>;
#[doc = "Source Event Flag Clear Register"]
pub mod sefclr;
#[doc = "CEVNP0 (rw) register accessor: Channel Event Node Pointer Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cevnp0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cevnp0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cevnp0`]
module"]
pub type CEVNP0 = crate::Reg<cevnp0::CEVNP0_SPEC>;
#[doc = "Channel Event Node Pointer Register 0"]
pub mod cevnp0;
#[doc = "REVNP0 (rw) register accessor: Result Event Node Pointer Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`revnp0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`revnp0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@revnp0`]
module"]
pub type REVNP0 = crate::Reg<revnp0::REVNP0_SPEC>;
#[doc = "Result Event Node Pointer Register 0"]
pub mod revnp0;
#[doc = "REVNP1 (rw) register accessor: Result Event Node Pointer Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`revnp1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`revnp1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@revnp1`]
module"]
pub type REVNP1 = crate::Reg<revnp1::REVNP1_SPEC>;
#[doc = "Result Event Node Pointer Register 1"]
pub mod revnp1;
#[doc = "SEVNP (rw) register accessor: Source Event Node Pointer Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sevnp::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sevnp::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sevnp`]
module"]
pub type SEVNP = crate::Reg<sevnp::SEVNP_SPEC>;
#[doc = "Source Event Node Pointer Register"]
pub mod sevnp;
#[doc = "SRACT (w) register accessor: Service Request Software Activation Trigger\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sract::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sract`]
module"]
pub type SRACT = crate::Reg<sract::SRACT_SPEC>;
#[doc = "Service Request Software Activation Trigger"]
pub mod sract;
#[doc = "EMUXCTR (rw) register accessor: E0ternal Multiplexer Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`emuxctr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`emuxctr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@emuxctr`]
module"]
pub type EMUXCTR = crate::Reg<emuxctr::EMUXCTR_SPEC>;
#[doc = "E0ternal Multiplexer Control Register"]
pub mod emuxctr;
#[doc = "VFR (rw) register accessor: Valid Flag Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vfr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vfr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vfr`]
module"]
pub type VFR = crate::Reg<vfr::VFR_SPEC>;
#[doc = "Valid Flag Register"]
pub mod vfr;
#[doc = "CHCTR (rw) register accessor: Channel Ctrl. Reg.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`chctr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`chctr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chctr`]
module"]
pub type CHCTR = crate::Reg<chctr::CHCTR_SPEC>;
#[doc = "Channel Ctrl. Reg."]
pub mod chctr;
#[doc = "RCR (rw) register accessor: Result Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcr`]
module"]
pub type RCR = crate::Reg<rcr::RCR_SPEC>;
#[doc = "Result Control Register"]
pub mod rcr;
#[doc = "RES (rw) register accessor: Result Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`res::R`]. WARN: One or more dependent resources other than the current register are immediately affected by a read operation. You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`res::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@res`]
module"]
pub type RES = crate::Reg<res::RES_SPEC>;
#[doc = "Result Register"]
pub mod res;
#[doc = "RESD (r) register accessor: Result Register, Debug\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`resd::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@resd`]
module"]
pub type RESD = crate::Reg<resd::RESD_SPEC>;
#[doc = "Result Register, Debug"]
pub mod resd;
