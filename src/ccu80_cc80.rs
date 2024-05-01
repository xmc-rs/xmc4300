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
    cr1: Cr1,
    cr1s: Cr1s,
    cr2: Cr2,
    cr2s: Cr2s,
    chc: Chc,
    dtc: Dtc,
    dc1r: Dc1r,
    dc2r: Dc2r,
    _reserved22: [u8; 0x18],
    timer: Timer,
    c0v: C0v,
    c1v: C1v,
    c2v: C2v,
    c3v: C3v,
    _reserved27: [u8; 0x1c],
    ints: Ints,
    inte: Inte,
    srs: Srs,
    sws: Sws,
    swr: Swr,
    stc: Stc,
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
    #[doc = "0x38 - Channel 1 Compare Value"]
    #[inline(always)]
    pub const fn cr1(&self) -> &Cr1 {
        &self.cr1
    }
    #[doc = "0x3c - Channel 1 Compare Shadow Value"]
    #[inline(always)]
    pub const fn cr1s(&self) -> &Cr1s {
        &self.cr1s
    }
    #[doc = "0x40 - Channel 2 Compare Value"]
    #[inline(always)]
    pub const fn cr2(&self) -> &Cr2 {
        &self.cr2
    }
    #[doc = "0x44 - Channel 2 Compare Shadow Value"]
    #[inline(always)]
    pub const fn cr2s(&self) -> &Cr2s {
        &self.cr2s
    }
    #[doc = "0x48 - Channel Control"]
    #[inline(always)]
    pub const fn chc(&self) -> &Chc {
        &self.chc
    }
    #[doc = "0x4c - Dead Time Control"]
    #[inline(always)]
    pub const fn dtc(&self) -> &Dtc {
        &self.dtc
    }
    #[doc = "0x50 - Channel 1 Dead Time Values"]
    #[inline(always)]
    pub const fn dc1r(&self) -> &Dc1r {
        &self.dc1r
    }
    #[doc = "0x54 - Channel 2 Dead Time Values"]
    #[inline(always)]
    pub const fn dc2r(&self) -> &Dc2r {
        &self.dc2r
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
    #[doc = "0xb4 - Shadow transfer control"]
    #[inline(always)]
    pub const fn stc(&self) -> &Stc {
        &self.stc
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
#[doc = "CR1 (r) register accessor: Channel 1 Compare Value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr1`]
module"]
#[doc(alias = "CR1")]
pub type Cr1 = crate::Reg<cr1::Cr1Spec>;
#[doc = "Channel 1 Compare Value"]
pub mod cr1;
#[doc = "CR1S (rw) register accessor: Channel 1 Compare Shadow Value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr1s::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr1s::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr1s`]
module"]
#[doc(alias = "CR1S")]
pub type Cr1s = crate::Reg<cr1s::Cr1sSpec>;
#[doc = "Channel 1 Compare Shadow Value"]
pub mod cr1s;
#[doc = "CR2 (r) register accessor: Channel 2 Compare Value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr2`]
module"]
#[doc(alias = "CR2")]
pub type Cr2 = crate::Reg<cr2::Cr2Spec>;
#[doc = "Channel 2 Compare Value"]
pub mod cr2;
#[doc = "CR2S (rw) register accessor: Channel 2 Compare Shadow Value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr2s::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr2s::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr2s`]
module"]
#[doc(alias = "CR2S")]
pub type Cr2s = crate::Reg<cr2s::Cr2sSpec>;
#[doc = "Channel 2 Compare Shadow Value"]
pub mod cr2s;
#[doc = "CHC (rw) register accessor: Channel Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`chc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`chc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chc`]
module"]
#[doc(alias = "CHC")]
pub type Chc = crate::Reg<chc::ChcSpec>;
#[doc = "Channel Control"]
pub mod chc;
#[doc = "DTC (rw) register accessor: Dead Time Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dtc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dtc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dtc`]
module"]
#[doc(alias = "DTC")]
pub type Dtc = crate::Reg<dtc::DtcSpec>;
#[doc = "Dead Time Control"]
pub mod dtc;
#[doc = "DC1R (rw) register accessor: Channel 1 Dead Time Values\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dc1r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dc1r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dc1r`]
module"]
#[doc(alias = "DC1R")]
pub type Dc1r = crate::Reg<dc1r::Dc1rSpec>;
#[doc = "Channel 1 Dead Time Values"]
pub mod dc1r;
#[doc = "DC2R (rw) register accessor: Channel 2 Dead Time Values\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dc2r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dc2r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dc2r`]
module"]
#[doc(alias = "DC2R")]
pub type Dc2r = crate::Reg<dc2r::Dc2rSpec>;
#[doc = "Channel 2 Dead Time Values"]
pub mod dc2r;
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
#[doc = "STC (rw) register accessor: Shadow transfer control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stc`]
module"]
#[doc(alias = "STC")]
pub type Stc = crate::Reg<stc::StcSpec>;
#[doc = "Shadow transfer control"]
pub mod stc;
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
