#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x80],
    arbcfg: Arbcfg,
    arbpr: Arbpr,
    chass: Chass,
    _reserved3: [u8; 0x14],
    iclass: [Iclass; 2],
    _reserved4: [u8; 0x08],
    alias: Alias,
    _reserved5: [u8; 0x04],
    bound: Bound,
    _reserved6: [u8; 0x04],
    synctr: Synctr,
    _reserved7: [u8; 0x04],
    bfl: Bfl,
    bfls: Bfls,
    bflc: Bflc,
    bflnp: Bflnp,
    _reserved11: [u8; 0x28],
    qctrl0: Qctrl0,
    qmr0: Qmr0,
    qsr0: Qsr0,
    q0r0: Q0r0,
    _reserved_15_qbur0: [u8; 0x04],
    _reserved16: [u8; 0x0c],
    asctrl: Asctrl,
    asmr: Asmr,
    assel: Assel,
    aspnd: Aspnd,
    _reserved20: [u8; 0x50],
    ceflag: Ceflag,
    reflag: Reflag,
    seflag: Seflag,
    _reserved23: [u8; 0x04],
    cefclr: Cefclr,
    refclr: Refclr,
    sefclr: Sefclr,
    _reserved26: [u8; 0x04],
    cevnp0: Cevnp0,
    _reserved27: [u8; 0x0c],
    revnp0: Revnp0,
    revnp1: Revnp1,
    _reserved29: [u8; 0x08],
    sevnp: Sevnp,
    _reserved30: [u8; 0x04],
    sract: Sract,
    _reserved31: [u8; 0x24],
    emuxctr: Emuxctr,
    _reserved32: [u8; 0x04],
    vfr: Vfr,
    _reserved33: [u8; 0x04],
    chctr: [Chctr; 8],
    _reserved34: [u8; 0x60],
    rcr: [Rcr; 16],
    _reserved35: [u8; 0x40],
    res: [Res; 16],
    _reserved36: [u8; 0x40],
    resd: [Resd; 16],
}
impl RegisterBlock {
    #[doc = "0x80 - Arbitration Configuration Register"]
    #[inline(always)]
    pub const fn arbcfg(&self) -> &Arbcfg {
        &self.arbcfg
    }
    #[doc = "0x84 - Arbitration Priority Register"]
    #[inline(always)]
    pub const fn arbpr(&self) -> &Arbpr {
        &self.arbpr
    }
    #[doc = "0x88 - Channel Assignment Register"]
    #[inline(always)]
    pub const fn chass(&self) -> &Chass {
        &self.chass
    }
    #[doc = "0xa0..0xa8 - Input Class Register"]
    #[inline(always)]
    pub const fn iclass(&self, n: usize) -> &Iclass {
        &self.iclass[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0xa0..0xa8 - Input Class Register"]
    #[inline(always)]
    pub fn iclass_iter(&self) -> impl Iterator<Item = &Iclass> {
        self.iclass.iter()
    }
    #[doc = "0xb0 - Alias Register"]
    #[inline(always)]
    pub const fn alias(&self) -> &Alias {
        &self.alias
    }
    #[doc = "0xb8 - Boundary Select Register"]
    #[inline(always)]
    pub const fn bound(&self) -> &Bound {
        &self.bound
    }
    #[doc = "0xc0 - Synchronization Control Register"]
    #[inline(always)]
    pub const fn synctr(&self) -> &Synctr {
        &self.synctr
    }
    #[doc = "0xc8 - Boundary Flag Register"]
    #[inline(always)]
    pub const fn bfl(&self) -> &Bfl {
        &self.bfl
    }
    #[doc = "0xcc - Boundary Flag Software Register"]
    #[inline(always)]
    pub const fn bfls(&self) -> &Bfls {
        &self.bfls
    }
    #[doc = "0xd0 - Boundary Flag Control Register"]
    #[inline(always)]
    pub const fn bflc(&self) -> &Bflc {
        &self.bflc
    }
    #[doc = "0xd4 - Boundary Flag Node Pointer Register"]
    #[inline(always)]
    pub const fn bflnp(&self) -> &Bflnp {
        &self.bflnp
    }
    #[doc = "0x100 - Queue 0 Source Control Register"]
    #[inline(always)]
    pub const fn qctrl0(&self) -> &Qctrl0 {
        &self.qctrl0
    }
    #[doc = "0x104 - Queue 0 Mode Register"]
    #[inline(always)]
    pub const fn qmr0(&self) -> &Qmr0 {
        &self.qmr0
    }
    #[doc = "0x108 - Queue 0 Status Register"]
    #[inline(always)]
    pub const fn qsr0(&self) -> &Qsr0 {
        &self.qsr0
    }
    #[doc = "0x10c - Queue 0 Register 0"]
    #[inline(always)]
    pub const fn q0r0(&self) -> &Q0r0 {
        &self.q0r0
    }
    #[doc = "0x110 - Queue 0 Backup Register"]
    #[inline(always)]
    pub const fn qbur0(&self) -> &Qbur0 {
        unsafe { &*(self as *const Self).cast::<u8>().add(272).cast() }
    }
    #[doc = "0x110 - Queue 0 Input Register"]
    #[inline(always)]
    pub const fn qinr0(&self) -> &Qinr0 {
        unsafe { &*(self as *const Self).cast::<u8>().add(272).cast() }
    }
    #[doc = "0x120 - Autoscan Source Control Register"]
    #[inline(always)]
    pub const fn asctrl(&self) -> &Asctrl {
        &self.asctrl
    }
    #[doc = "0x124 - Autoscan Source Mode Register"]
    #[inline(always)]
    pub const fn asmr(&self) -> &Asmr {
        &self.asmr
    }
    #[doc = "0x128 - Autoscan Source Channel Select Register"]
    #[inline(always)]
    pub const fn assel(&self) -> &Assel {
        &self.assel
    }
    #[doc = "0x12c - Autoscan Source Pending Register"]
    #[inline(always)]
    pub const fn aspnd(&self) -> &Aspnd {
        &self.aspnd
    }
    #[doc = "0x180 - Channel Event Flag Register"]
    #[inline(always)]
    pub const fn ceflag(&self) -> &Ceflag {
        &self.ceflag
    }
    #[doc = "0x184 - Result Event Flag Register"]
    #[inline(always)]
    pub const fn reflag(&self) -> &Reflag {
        &self.reflag
    }
    #[doc = "0x188 - Source Event Flag Register"]
    #[inline(always)]
    pub const fn seflag(&self) -> &Seflag {
        &self.seflag
    }
    #[doc = "0x190 - Channel Event Flag Clear Register"]
    #[inline(always)]
    pub const fn cefclr(&self) -> &Cefclr {
        &self.cefclr
    }
    #[doc = "0x194 - Result Event Flag Clear Register"]
    #[inline(always)]
    pub const fn refclr(&self) -> &Refclr {
        &self.refclr
    }
    #[doc = "0x198 - Source Event Flag Clear Register"]
    #[inline(always)]
    pub const fn sefclr(&self) -> &Sefclr {
        &self.sefclr
    }
    #[doc = "0x1a0 - Channel Event Node Pointer Register 0"]
    #[inline(always)]
    pub const fn cevnp0(&self) -> &Cevnp0 {
        &self.cevnp0
    }
    #[doc = "0x1b0 - Result Event Node Pointer Register 0"]
    #[inline(always)]
    pub const fn revnp0(&self) -> &Revnp0 {
        &self.revnp0
    }
    #[doc = "0x1b4 - Result Event Node Pointer Register 1"]
    #[inline(always)]
    pub const fn revnp1(&self) -> &Revnp1 {
        &self.revnp1
    }
    #[doc = "0x1c0 - Source Event Node Pointer Register"]
    #[inline(always)]
    pub const fn sevnp(&self) -> &Sevnp {
        &self.sevnp
    }
    #[doc = "0x1c8 - Service Request Software Activation Trigger"]
    #[inline(always)]
    pub const fn sract(&self) -> &Sract {
        &self.sract
    }
    #[doc = "0x1f0 - E0ternal Multiplexer Control Register"]
    #[inline(always)]
    pub const fn emuxctr(&self) -> &Emuxctr {
        &self.emuxctr
    }
    #[doc = "0x1f8 - Valid Flag Register"]
    #[inline(always)]
    pub const fn vfr(&self) -> &Vfr {
        &self.vfr
    }
    #[doc = "0x200..0x220 - Channel Ctrl. Reg."]
    #[inline(always)]
    pub const fn chctr(&self, n: usize) -> &Chctr {
        &self.chctr[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x200..0x220 - Channel Ctrl. Reg."]
    #[inline(always)]
    pub fn chctr_iter(&self) -> impl Iterator<Item = &Chctr> {
        self.chctr.iter()
    }
    #[doc = "0x280..0x2c0 - Result Control Register"]
    #[inline(always)]
    pub const fn rcr(&self, n: usize) -> &Rcr {
        &self.rcr[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x280..0x2c0 - Result Control Register"]
    #[inline(always)]
    pub fn rcr_iter(&self) -> impl Iterator<Item = &Rcr> {
        self.rcr.iter()
    }
    #[doc = "0x300..0x340 - Result Register"]
    #[inline(always)]
    pub const fn res(&self, n: usize) -> &Res {
        &self.res[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x300..0x340 - Result Register"]
    #[inline(always)]
    pub fn res_iter(&self) -> impl Iterator<Item = &Res> {
        self.res.iter()
    }
    #[doc = "0x380..0x3c0 - Result Register, Debug"]
    #[inline(always)]
    pub const fn resd(&self, n: usize) -> &Resd {
        &self.resd[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x380..0x3c0 - Result Register, Debug"]
    #[inline(always)]
    pub fn resd_iter(&self) -> impl Iterator<Item = &Resd> {
        self.resd.iter()
    }
}
#[doc = "ARBCFG (rw) register accessor: Arbitration Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`arbcfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`arbcfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@arbcfg`]
module"]
#[doc(alias = "ARBCFG")]
pub type Arbcfg = crate::Reg<arbcfg::ArbcfgSpec>;
#[doc = "Arbitration Configuration Register"]
pub mod arbcfg;
#[doc = "ARBPR (rw) register accessor: Arbitration Priority Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`arbpr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`arbpr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@arbpr`]
module"]
#[doc(alias = "ARBPR")]
pub type Arbpr = crate::Reg<arbpr::ArbprSpec>;
#[doc = "Arbitration Priority Register"]
pub mod arbpr;
#[doc = "CHASS (rw) register accessor: Channel Assignment Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`chass::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`chass::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chass`]
module"]
#[doc(alias = "CHASS")]
pub type Chass = crate::Reg<chass::ChassSpec>;
#[doc = "Channel Assignment Register"]
pub mod chass;
#[doc = "ICLASS (rw) register accessor: Input Class Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iclass::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iclass::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iclass`]
module"]
#[doc(alias = "ICLASS")]
pub type Iclass = crate::Reg<iclass::IclassSpec>;
#[doc = "Input Class Register"]
pub mod iclass;
#[doc = "ALIAS (rw) register accessor: Alias Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`alias::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`alias::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@alias`]
module"]
#[doc(alias = "ALIAS")]
pub type Alias = crate::Reg<alias::AliasSpec>;
#[doc = "Alias Register"]
pub mod alias;
#[doc = "BOUND (rw) register accessor: Boundary Select Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bound::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bound::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bound`]
module"]
#[doc(alias = "BOUND")]
pub type Bound = crate::Reg<bound::BoundSpec>;
#[doc = "Boundary Select Register"]
pub mod bound;
#[doc = "SYNCTR (rw) register accessor: Synchronization Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`synctr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`synctr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@synctr`]
module"]
#[doc(alias = "SYNCTR")]
pub type Synctr = crate::Reg<synctr::SynctrSpec>;
#[doc = "Synchronization Control Register"]
pub mod synctr;
#[doc = "BFL (rw) register accessor: Boundary Flag Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bfl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bfl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bfl`]
module"]
#[doc(alias = "BFL")]
pub type Bfl = crate::Reg<bfl::BflSpec>;
#[doc = "Boundary Flag Register"]
pub mod bfl;
#[doc = "BFLS (w) register accessor: Boundary Flag Software Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bfls::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bfls`]
module"]
#[doc(alias = "BFLS")]
pub type Bfls = crate::Reg<bfls::BflsSpec>;
#[doc = "Boundary Flag Software Register"]
pub mod bfls;
#[doc = "BFLC (rw) register accessor: Boundary Flag Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bflc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bflc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bflc`]
module"]
#[doc(alias = "BFLC")]
pub type Bflc = crate::Reg<bflc::BflcSpec>;
#[doc = "Boundary Flag Control Register"]
pub mod bflc;
#[doc = "BFLNP (rw) register accessor: Boundary Flag Node Pointer Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bflnp::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bflnp::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bflnp`]
module"]
#[doc(alias = "BFLNP")]
pub type Bflnp = crate::Reg<bflnp::BflnpSpec>;
#[doc = "Boundary Flag Node Pointer Register"]
pub mod bflnp;
#[doc = "QCTRL0 (rw) register accessor: Queue 0 Source Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`qctrl0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`qctrl0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@qctrl0`]
module"]
#[doc(alias = "QCTRL0")]
pub type Qctrl0 = crate::Reg<qctrl0::Qctrl0Spec>;
#[doc = "Queue 0 Source Control Register"]
pub mod qctrl0;
#[doc = "QMR0 (rw) register accessor: Queue 0 Mode Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`qmr0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`qmr0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@qmr0`]
module"]
#[doc(alias = "QMR0")]
pub type Qmr0 = crate::Reg<qmr0::Qmr0Spec>;
#[doc = "Queue 0 Mode Register"]
pub mod qmr0;
#[doc = "QSR0 (r) register accessor: Queue 0 Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`qsr0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@qsr0`]
module"]
#[doc(alias = "QSR0")]
pub type Qsr0 = crate::Reg<qsr0::Qsr0Spec>;
#[doc = "Queue 0 Status Register"]
pub mod qsr0;
#[doc = "Q0R0 (r) register accessor: Queue 0 Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`q0r0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@q0r0`]
module"]
#[doc(alias = "Q0R0")]
pub type Q0r0 = crate::Reg<q0r0::Q0r0Spec>;
#[doc = "Queue 0 Register 0"]
pub mod q0r0;
#[doc = "QINR0 (w) register accessor: Queue 0 Input Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`qinr0::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@qinr0`]
module"]
#[doc(alias = "QINR0")]
pub type Qinr0 = crate::Reg<qinr0::Qinr0Spec>;
#[doc = "Queue 0 Input Register"]
pub mod qinr0;
#[doc = "QBUR0 (r) register accessor: Queue 0 Backup Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`qbur0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@qbur0`]
module"]
#[doc(alias = "QBUR0")]
pub type Qbur0 = crate::Reg<qbur0::Qbur0Spec>;
#[doc = "Queue 0 Backup Register"]
pub mod qbur0;
#[doc = "ASCTRL (rw) register accessor: Autoscan Source Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`asctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`asctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@asctrl`]
module"]
#[doc(alias = "ASCTRL")]
pub type Asctrl = crate::Reg<asctrl::AsctrlSpec>;
#[doc = "Autoscan Source Control Register"]
pub mod asctrl;
#[doc = "ASMR (rw) register accessor: Autoscan Source Mode Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`asmr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`asmr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@asmr`]
module"]
#[doc(alias = "ASMR")]
pub type Asmr = crate::Reg<asmr::AsmrSpec>;
#[doc = "Autoscan Source Mode Register"]
pub mod asmr;
#[doc = "ASSEL (rw) register accessor: Autoscan Source Channel Select Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`assel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`assel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@assel`]
module"]
#[doc(alias = "ASSEL")]
pub type Assel = crate::Reg<assel::AsselSpec>;
#[doc = "Autoscan Source Channel Select Register"]
pub mod assel;
#[doc = "ASPND (rw) register accessor: Autoscan Source Pending Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aspnd::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aspnd::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aspnd`]
module"]
#[doc(alias = "ASPND")]
pub type Aspnd = crate::Reg<aspnd::AspndSpec>;
#[doc = "Autoscan Source Pending Register"]
pub mod aspnd;
#[doc = "CEFLAG (rw) register accessor: Channel Event Flag Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ceflag::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ceflag::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ceflag`]
module"]
#[doc(alias = "CEFLAG")]
pub type Ceflag = crate::Reg<ceflag::CeflagSpec>;
#[doc = "Channel Event Flag Register"]
pub mod ceflag;
#[doc = "REFLAG (rw) register accessor: Result Event Flag Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`reflag::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`reflag::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reflag`]
module"]
#[doc(alias = "REFLAG")]
pub type Reflag = crate::Reg<reflag::ReflagSpec>;
#[doc = "Result Event Flag Register"]
pub mod reflag;
#[doc = "SEFLAG (rw) register accessor: Source Event Flag Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`seflag::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`seflag::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@seflag`]
module"]
#[doc(alias = "SEFLAG")]
pub type Seflag = crate::Reg<seflag::SeflagSpec>;
#[doc = "Source Event Flag Register"]
pub mod seflag;
#[doc = "CEFCLR (w) register accessor: Channel Event Flag Clear Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cefclr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cefclr`]
module"]
#[doc(alias = "CEFCLR")]
pub type Cefclr = crate::Reg<cefclr::CefclrSpec>;
#[doc = "Channel Event Flag Clear Register"]
pub mod cefclr;
#[doc = "REFCLR (w) register accessor: Result Event Flag Clear Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`refclr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@refclr`]
module"]
#[doc(alias = "REFCLR")]
pub type Refclr = crate::Reg<refclr::RefclrSpec>;
#[doc = "Result Event Flag Clear Register"]
pub mod refclr;
#[doc = "SEFCLR (w) register accessor: Source Event Flag Clear Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sefclr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sefclr`]
module"]
#[doc(alias = "SEFCLR")]
pub type Sefclr = crate::Reg<sefclr::SefclrSpec>;
#[doc = "Source Event Flag Clear Register"]
pub mod sefclr;
#[doc = "CEVNP0 (rw) register accessor: Channel Event Node Pointer Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cevnp0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cevnp0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cevnp0`]
module"]
#[doc(alias = "CEVNP0")]
pub type Cevnp0 = crate::Reg<cevnp0::Cevnp0Spec>;
#[doc = "Channel Event Node Pointer Register 0"]
pub mod cevnp0;
#[doc = "REVNP0 (rw) register accessor: Result Event Node Pointer Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`revnp0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`revnp0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@revnp0`]
module"]
#[doc(alias = "REVNP0")]
pub type Revnp0 = crate::Reg<revnp0::Revnp0Spec>;
#[doc = "Result Event Node Pointer Register 0"]
pub mod revnp0;
#[doc = "REVNP1 (rw) register accessor: Result Event Node Pointer Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`revnp1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`revnp1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@revnp1`]
module"]
#[doc(alias = "REVNP1")]
pub type Revnp1 = crate::Reg<revnp1::Revnp1Spec>;
#[doc = "Result Event Node Pointer Register 1"]
pub mod revnp1;
#[doc = "SEVNP (rw) register accessor: Source Event Node Pointer Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sevnp::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sevnp::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sevnp`]
module"]
#[doc(alias = "SEVNP")]
pub type Sevnp = crate::Reg<sevnp::SevnpSpec>;
#[doc = "Source Event Node Pointer Register"]
pub mod sevnp;
#[doc = "SRACT (w) register accessor: Service Request Software Activation Trigger\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sract::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sract`]
module"]
#[doc(alias = "SRACT")]
pub type Sract = crate::Reg<sract::SractSpec>;
#[doc = "Service Request Software Activation Trigger"]
pub mod sract;
#[doc = "EMUXCTR (rw) register accessor: E0ternal Multiplexer Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`emuxctr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`emuxctr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@emuxctr`]
module"]
#[doc(alias = "EMUXCTR")]
pub type Emuxctr = crate::Reg<emuxctr::EmuxctrSpec>;
#[doc = "E0ternal Multiplexer Control Register"]
pub mod emuxctr;
#[doc = "VFR (rw) register accessor: Valid Flag Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vfr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vfr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vfr`]
module"]
#[doc(alias = "VFR")]
pub type Vfr = crate::Reg<vfr::VfrSpec>;
#[doc = "Valid Flag Register"]
pub mod vfr;
#[doc = "CHCTR (rw) register accessor: Channel Ctrl. Reg.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`chctr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`chctr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chctr`]
module"]
#[doc(alias = "CHCTR")]
pub type Chctr = crate::Reg<chctr::ChctrSpec>;
#[doc = "Channel Ctrl. Reg."]
pub mod chctr;
#[doc = "RCR (rw) register accessor: Result Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcr`]
module"]
#[doc(alias = "RCR")]
pub type Rcr = crate::Reg<rcr::RcrSpec>;
#[doc = "Result Control Register"]
pub mod rcr;
#[doc = "RES (rw) register accessor: Result Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`res::R`]. WARN: One or more dependent resources other than the current register are immediately affected by a read operation. You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`res::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@res`]
module"]
#[doc(alias = "RES")]
pub type Res = crate::Reg<res::ResSpec>;
#[doc = "Result Register"]
pub mod res;
#[doc = "RESD (r) register accessor: Result Register, Debug\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`resd::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@resd`]
module"]
#[doc(alias = "RESD")]
pub type Resd = crate::Reg<resd::ResdSpec>;
#[doc = "Result Register, Debug"]
pub mod resd;
