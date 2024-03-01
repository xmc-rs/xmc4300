#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ins: Ins,
    cmc: Cmc,
    tcst: Tcst,
    tcset: Tcset,
    tcclr: Tcclr,
    tc: Tc,
    psl: Psl,
    dit: Dit,
    dits: Dits,
    psc: Psc,
    fpc: Fpc,
    fpcs: Fpcs,
    pr: Pr,
    prs: Prs,
    cr: Cr,
    crs: Crs,
    _reserved16: [u8; 0x30],
    timer: Timer,
    c0v: C0v,
    c1v: C1v,
    c2v: C2v,
    c3v: C3v,
    _reserved21: [u8; 0x1c],
    ints: Ints,
    inte: Inte,
    srs: Srs,
    sws: Sws,
    swr: Swr,
    _reserved26: [u8; 0x04],
    ecrd0: Ecrd0,
    ecrd1: Ecrd1,
}
impl RegisterBlock {
    #[doc = "0x00 - Input Selector Configuration"]
    #[inline(always)]
    pub const fn ins(&self) -> &Ins {
        &self.ins
    }
    #[doc = "0x04 - Connection Matrix Control"]
    #[inline(always)]
    pub const fn cmc(&self) -> &Cmc {
        &self.cmc
    }
    #[doc = "0x08 - Slice Timer Status"]
    #[inline(always)]
    pub const fn tcst(&self) -> &Tcst {
        &self.tcst
    }
    #[doc = "0x0c - Slice Timer Run Set"]
    #[inline(always)]
    pub const fn tcset(&self) -> &Tcset {
        &self.tcset
    }
    #[doc = "0x10 - Slice Timer Clear"]
    #[inline(always)]
    pub const fn tcclr(&self) -> &Tcclr {
        &self.tcclr
    }
    #[doc = "0x14 - Slice Timer Control"]
    #[inline(always)]
    pub const fn tc(&self) -> &Tc {
        &self.tc
    }
    #[doc = "0x18 - Passive Level Config"]
    #[inline(always)]
    pub const fn psl(&self) -> &Psl {
        &self.psl
    }
    #[doc = "0x1c - Dither Config"]
    #[inline(always)]
    pub const fn dit(&self) -> &Dit {
        &self.dit
    }
    #[doc = "0x20 - Dither Shadow Register"]
    #[inline(always)]
    pub const fn dits(&self) -> &Dits {
        &self.dits
    }
    #[doc = "0x24 - Prescaler Control"]
    #[inline(always)]
    pub const fn psc(&self) -> &Psc {
        &self.psc
    }
    #[doc = "0x28 - Floating Prescaler Control"]
    #[inline(always)]
    pub const fn fpc(&self) -> &Fpc {
        &self.fpc
    }
    #[doc = "0x2c - Floating Prescaler Shadow"]
    #[inline(always)]
    pub const fn fpcs(&self) -> &Fpcs {
        &self.fpcs
    }
    #[doc = "0x30 - Timer Period Value"]
    #[inline(always)]
    pub const fn pr(&self) -> &Pr {
        &self.pr
    }
    #[doc = "0x34 - Timer Shadow Period Value"]
    #[inline(always)]
    pub const fn prs(&self) -> &Prs {
        &self.prs
    }
    #[doc = "0x38 - Timer Compare Value"]
    #[inline(always)]
    pub const fn cr(&self) -> &Cr {
        &self.cr
    }
    #[doc = "0x3c - Timer Shadow Compare Value"]
    #[inline(always)]
    pub const fn crs(&self) -> &Crs {
        &self.crs
    }
    #[doc = "0x70 - Timer Value"]
    #[inline(always)]
    pub const fn timer(&self) -> &Timer {
        &self.timer
    }
    #[doc = "0x74 - Capture Register 0"]
    #[inline(always)]
    pub const fn c0v(&self) -> &C0v {
        &self.c0v
    }
    #[doc = "0x78 - Capture Register 1"]
    #[inline(always)]
    pub const fn c1v(&self) -> &C1v {
        &self.c1v
    }
    #[doc = "0x7c - Capture Register 2"]
    #[inline(always)]
    pub const fn c2v(&self) -> &C2v {
        &self.c2v
    }
    #[doc = "0x80 - Capture Register 3"]
    #[inline(always)]
    pub const fn c3v(&self) -> &C3v {
        &self.c3v
    }
    #[doc = "0xa0 - Interrupt Status"]
    #[inline(always)]
    pub const fn ints(&self) -> &Ints {
        &self.ints
    }
    #[doc = "0xa4 - Interrupt Enable Control"]
    #[inline(always)]
    pub const fn inte(&self) -> &Inte {
        &self.inte
    }
    #[doc = "0xa8 - Service Request Selector"]
    #[inline(always)]
    pub const fn srs(&self) -> &Srs {
        &self.srs
    }
    #[doc = "0xac - Interrupt Status Set"]
    #[inline(always)]
    pub const fn sws(&self) -> &Sws {
        &self.sws
    }
    #[doc = "0xb0 - Interrupt Status Clear"]
    #[inline(always)]
    pub const fn swr(&self) -> &Swr {
        &self.swr
    }
    #[doc = "0xb8 - Extended Read Back 0"]
    #[inline(always)]
    pub const fn ecrd0(&self) -> &Ecrd0 {
        &self.ecrd0
    }
    #[doc = "0xbc - Extended Read Back 1"]
    #[inline(always)]
    pub const fn ecrd1(&self) -> &Ecrd1 {
        &self.ecrd1
    }
}
#[doc = "INS (rw) register accessor: Input Selector Configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ins::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ins::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ins`]
module"]
#[doc(alias = "INS")]
pub type Ins = crate::Reg<ins::InsSpec>;
#[doc = "Input Selector Configuration"]
pub mod ins;
#[doc = "CMC (rw) register accessor: Connection Matrix Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmc`]
module"]
#[doc(alias = "CMC")]
pub type Cmc = crate::Reg<cmc::CmcSpec>;
#[doc = "Connection Matrix Control"]
pub mod cmc;
#[doc = "TCST (r) register accessor: Slice Timer Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tcst::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tcst`]
module"]
#[doc(alias = "TCST")]
pub type Tcst = crate::Reg<tcst::TcstSpec>;
#[doc = "Slice Timer Status"]
pub mod tcst;
#[doc = "TCSET (w) register accessor: Slice Timer Run Set\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tcset::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tcset`]
module"]
#[doc(alias = "TCSET")]
pub type Tcset = crate::Reg<tcset::TcsetSpec>;
#[doc = "Slice Timer Run Set"]
pub mod tcset;
#[doc = "TCCLR (w) register accessor: Slice Timer Clear\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tcclr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tcclr`]
module"]
#[doc(alias = "TCCLR")]
pub type Tcclr = crate::Reg<tcclr::TcclrSpec>;
#[doc = "Slice Timer Clear"]
pub mod tcclr;
#[doc = "TC (rw) register accessor: Slice Timer Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tc`]
module"]
#[doc(alias = "TC")]
pub type Tc = crate::Reg<tc::TcSpec>;
#[doc = "Slice Timer Control"]
pub mod tc;
#[doc = "PSL (rw) register accessor: Passive Level Config\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`psl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`psl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@psl`]
module"]
#[doc(alias = "PSL")]
pub type Psl = crate::Reg<psl::PslSpec>;
#[doc = "Passive Level Config"]
pub mod psl;
#[doc = "DIT (r) register accessor: Dither Config\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dit::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dit`]
module"]
#[doc(alias = "DIT")]
pub type Dit = crate::Reg<dit::DitSpec>;
#[doc = "Dither Config"]
pub mod dit;
#[doc = "DITS (rw) register accessor: Dither Shadow Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dits::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dits::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dits`]
module"]
#[doc(alias = "DITS")]
pub type Dits = crate::Reg<dits::DitsSpec>;
#[doc = "Dither Shadow Register"]
pub mod dits;
#[doc = "PSC (rw) register accessor: Prescaler Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`psc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`psc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@psc`]
module"]
#[doc(alias = "PSC")]
pub type Psc = crate::Reg<psc::PscSpec>;
#[doc = "Prescaler Control"]
pub mod psc;
#[doc = "FPC (rw) register accessor: Floating Prescaler Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fpc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fpc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fpc`]
module"]
#[doc(alias = "FPC")]
pub type Fpc = crate::Reg<fpc::FpcSpec>;
#[doc = "Floating Prescaler Control"]
pub mod fpc;
#[doc = "FPCS (rw) register accessor: Floating Prescaler Shadow\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fpcs::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fpcs::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fpcs`]
module"]
#[doc(alias = "FPCS")]
pub type Fpcs = crate::Reg<fpcs::FpcsSpec>;
#[doc = "Floating Prescaler Shadow"]
pub mod fpcs;
#[doc = "PR (r) register accessor: Timer Period Value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr`]
module"]
#[doc(alias = "PR")]
pub type Pr = crate::Reg<pr::PrSpec>;
#[doc = "Timer Period Value"]
pub mod pr;
#[doc = "PRS (rw) register accessor: Timer Shadow Period Value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`prs::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`prs::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prs`]
module"]
#[doc(alias = "PRS")]
pub type Prs = crate::Reg<prs::PrsSpec>;
#[doc = "Timer Shadow Period Value"]
pub mod prs;
#[doc = "CR (r) register accessor: Timer Compare Value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr`]
module"]
#[doc(alias = "CR")]
pub type Cr = crate::Reg<cr::CrSpec>;
#[doc = "Timer Compare Value"]
pub mod cr;
#[doc = "CRS (rw) register accessor: Timer Shadow Compare Value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`crs::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`crs::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crs`]
module"]
#[doc(alias = "CRS")]
pub type Crs = crate::Reg<crs::CrsSpec>;
#[doc = "Timer Shadow Compare Value"]
pub mod crs;
#[doc = "TIMER (rw) register accessor: Timer Value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timer::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timer::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timer`]
module"]
#[doc(alias = "TIMER")]
pub type Timer = crate::Reg<timer::TimerSpec>;
#[doc = "Timer Value"]
pub mod timer;
#[doc = "C0V (r) register accessor: Capture Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c0v::R`]. WARN: One or more dependent resources other than the current register are immediately affected by a read operation. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c0v`]
module"]
#[doc(alias = "C0V")]
pub type C0v = crate::Reg<c0v::C0vSpec>;
#[doc = "Capture Register 0"]
pub mod c0v;
#[doc = "C1V (r) register accessor: Capture Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c1v::R`]. WARN: One or more dependent resources other than the current register are immediately affected by a read operation. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c1v`]
module"]
#[doc(alias = "C1V")]
pub type C1v = crate::Reg<c1v::C1vSpec>;
#[doc = "Capture Register 1"]
pub mod c1v;
#[doc = "C2V (r) register accessor: Capture Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c2v::R`]. WARN: One or more dependent resources other than the current register are immediately affected by a read operation. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c2v`]
module"]
#[doc(alias = "C2V")]
pub type C2v = crate::Reg<c2v::C2vSpec>;
#[doc = "Capture Register 2"]
pub mod c2v;
#[doc = "C3V (r) register accessor: Capture Register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c3v::R`]. WARN: One or more dependent resources other than the current register are immediately affected by a read operation. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c3v`]
module"]
#[doc(alias = "C3V")]
pub type C3v = crate::Reg<c3v::C3vSpec>;
#[doc = "Capture Register 3"]
pub mod c3v;
#[doc = "INTS (r) register accessor: Interrupt Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ints::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ints`]
module"]
#[doc(alias = "INTS")]
pub type Ints = crate::Reg<ints::IntsSpec>;
#[doc = "Interrupt Status"]
pub mod ints;
#[doc = "INTE (rw) register accessor: Interrupt Enable Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`inte::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`inte::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@inte`]
module"]
#[doc(alias = "INTE")]
pub type Inte = crate::Reg<inte::InteSpec>;
#[doc = "Interrupt Enable Control"]
pub mod inte;
#[doc = "SRS (rw) register accessor: Service Request Selector\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`srs::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`srs::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@srs`]
module"]
#[doc(alias = "SRS")]
pub type Srs = crate::Reg<srs::SrsSpec>;
#[doc = "Service Request Selector"]
pub mod srs;
#[doc = "SWS (w) register accessor: Interrupt Status Set\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sws::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sws`]
module"]
#[doc(alias = "SWS")]
pub type Sws = crate::Reg<sws::SwsSpec>;
#[doc = "Interrupt Status Set"]
pub mod sws;
#[doc = "SWR (w) register accessor: Interrupt Status Clear\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swr`]
module"]
#[doc(alias = "SWR")]
pub type Swr = crate::Reg<swr::SwrSpec>;
#[doc = "Interrupt Status Clear"]
pub mod swr;
#[doc = "ECRD0 (r) register accessor: Extended Read Back 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ecrd0::R`]. WARN: One or more dependent resources other than the current register are immediately affected by a read operation. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ecrd0`]
module"]
#[doc(alias = "ECRD0")]
pub type Ecrd0 = crate::Reg<ecrd0::Ecrd0Spec>;
#[doc = "Extended Read Back 0"]
pub mod ecrd0;
#[doc = "ECRD1 (r) register accessor: Extended Read Back 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ecrd1::R`]. WARN: One or more dependent resources other than the current register are immediately affected by a read operation. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ecrd1`]
module"]
#[doc(alias = "ECRD1")]
pub type Ecrd1 = crate::Reg<ecrd1::Ecrd1Spec>;
#[doc = "Extended Read Back 1"]
pub mod ecrd1;
