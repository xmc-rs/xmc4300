#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x04],
    ccfg: Ccfg,
    _reserved1: [u8; 0x04],
    kscfg: Kscfg,
    fdr: Fdr,
    brg: Brg,
    inpr: Inpr,
    dx0cr: Dx0cr,
    dx1cr: Dx1cr,
    dx2cr: Dx2cr,
    dx3cr: Dx3cr,
    dx4cr: Dx4cr,
    dx5cr: Dx5cr,
    sctr: Sctr,
    tcsr: Tcsr,
    _reserved_13_pcr: [u8; 0x04],
    ccr: Ccr,
    cmtr: Cmtr,
    _reserved_16_psr: [u8; 0x04],
    pscr: Pscr,
    rbufsr: Rbufsr,
    rbuf: Rbuf,
    rbufd: Rbufd,
    rbuf0: Rbuf0,
    rbuf1: Rbuf1,
    rbuf01sr: Rbuf01sr,
    fmr: Fmr,
    _reserved25: [u8; 0x14],
    tbuf: [Tbuf; 32],
    byp: Byp,
    bypcr: Bypcr,
    tbctr: Tbctr,
    rbctr: Rbctr,
    trbptr: Trbptr,
    trbsr: Trbsr,
    trbscr: Trbscr,
    outr: Outr,
    outdr: Outdr,
    _reserved35: [u8; 0x5c],
    in_: [In; 32],
}
impl RegisterBlock {
    #[doc = "0x04 - Channel Configuration Register"]
    #[inline(always)]
    pub const fn ccfg(&self) -> &Ccfg {
        &self.ccfg
    }
    #[doc = "0x0c - Kernel State Configuration Register"]
    #[inline(always)]
    pub const fn kscfg(&self) -> &Kscfg {
        &self.kscfg
    }
    #[doc = "0x10 - Fractional Divider Register"]
    #[inline(always)]
    pub const fn fdr(&self) -> &Fdr {
        &self.fdr
    }
    #[doc = "0x14 - Baud Rate Generator Register"]
    #[inline(always)]
    pub const fn brg(&self) -> &Brg {
        &self.brg
    }
    #[doc = "0x18 - Interrupt Node Pointer Register"]
    #[inline(always)]
    pub const fn inpr(&self) -> &Inpr {
        &self.inpr
    }
    #[doc = "0x1c - Input Control Register 0"]
    #[inline(always)]
    pub const fn dx0cr(&self) -> &Dx0cr {
        &self.dx0cr
    }
    #[doc = "0x20 - Input Control Register 1"]
    #[inline(always)]
    pub const fn dx1cr(&self) -> &Dx1cr {
        &self.dx1cr
    }
    #[doc = "0x24 - Input Control Register 2"]
    #[inline(always)]
    pub const fn dx2cr(&self) -> &Dx2cr {
        &self.dx2cr
    }
    #[doc = "0x28 - Input Control Register 3"]
    #[inline(always)]
    pub const fn dx3cr(&self) -> &Dx3cr {
        &self.dx3cr
    }
    #[doc = "0x2c - Input Control Register 4"]
    #[inline(always)]
    pub const fn dx4cr(&self) -> &Dx4cr {
        &self.dx4cr
    }
    #[doc = "0x30 - Input Control Register 5"]
    #[inline(always)]
    pub const fn dx5cr(&self) -> &Dx5cr {
        &self.dx5cr
    }
    #[doc = "0x34 - Shift Control Register"]
    #[inline(always)]
    pub const fn sctr(&self) -> &Sctr {
        &self.sctr
    }
    #[doc = "0x38 - Transmit Control/Status Register"]
    #[inline(always)]
    pub const fn tcsr(&self) -> &Tcsr {
        &self.tcsr
    }
    #[doc = "0x3c - Protocol Control Register \\[IIS Mode\\]"]
    #[inline(always)]
    pub const fn pcr_iismode(&self) -> &PcrIismode {
        unsafe { &*(self as *const Self).cast::<u8>().add(60).cast() }
    }
    #[doc = "0x3c - Protocol Control Register \\[IIC Mode\\]"]
    #[inline(always)]
    pub const fn pcr_iicmode(&self) -> &PcrIicmode {
        unsafe { &*(self as *const Self).cast::<u8>().add(60).cast() }
    }
    #[doc = "0x3c - Protocol Control Register \\[SSC Mode\\]"]
    #[inline(always)]
    pub const fn pcr_sscmode(&self) -> &PcrSscmode {
        unsafe { &*(self as *const Self).cast::<u8>().add(60).cast() }
    }
    #[doc = "0x3c - Protocol Control Register \\[ASC Mode\\]"]
    #[inline(always)]
    pub const fn pcr_ascmode(&self) -> &PcrAscmode {
        unsafe { &*(self as *const Self).cast::<u8>().add(60).cast() }
    }
    #[doc = "0x3c - Protocol Control Register"]
    #[inline(always)]
    pub const fn pcr(&self) -> &Pcr {
        unsafe { &*(self as *const Self).cast::<u8>().add(60).cast() }
    }
    #[doc = "0x40 - Channel Control Register"]
    #[inline(always)]
    pub const fn ccr(&self) -> &Ccr {
        &self.ccr
    }
    #[doc = "0x44 - Capture Mode Timer Register"]
    #[inline(always)]
    pub const fn cmtr(&self) -> &Cmtr {
        &self.cmtr
    }
    #[doc = "0x48 - Protocol Status Register \\[IIS Mode\\]"]
    #[inline(always)]
    pub const fn psr_iismode(&self) -> &PsrIismode {
        unsafe { &*(self as *const Self).cast::<u8>().add(72).cast() }
    }
    #[doc = "0x48 - Protocol Status Register \\[IIC Mode\\]"]
    #[inline(always)]
    pub const fn psr_iicmode(&self) -> &PsrIicmode {
        unsafe { &*(self as *const Self).cast::<u8>().add(72).cast() }
    }
    #[doc = "0x48 - Protocol Status Register \\[SSC Mode\\]"]
    #[inline(always)]
    pub const fn psr_sscmode(&self) -> &PsrSscmode {
        unsafe { &*(self as *const Self).cast::<u8>().add(72).cast() }
    }
    #[doc = "0x48 - Protocol Status Register \\[ASC Mode\\]"]
    #[inline(always)]
    pub const fn psr_ascmode(&self) -> &PsrAscmode {
        unsafe { &*(self as *const Self).cast::<u8>().add(72).cast() }
    }
    #[doc = "0x48 - Protocol Status Register"]
    #[inline(always)]
    pub const fn psr(&self) -> &Psr {
        unsafe { &*(self as *const Self).cast::<u8>().add(72).cast() }
    }
    #[doc = "0x4c - Protocol Status Clear Register"]
    #[inline(always)]
    pub const fn pscr(&self) -> &Pscr {
        &self.pscr
    }
    #[doc = "0x50 - Receiver Buffer Status Register"]
    #[inline(always)]
    pub const fn rbufsr(&self) -> &Rbufsr {
        &self.rbufsr
    }
    #[doc = "0x54 - Receiver Buffer Register"]
    #[inline(always)]
    pub const fn rbuf(&self) -> &Rbuf {
        &self.rbuf
    }
    #[doc = "0x58 - Receiver Buffer Register for Debugger"]
    #[inline(always)]
    pub const fn rbufd(&self) -> &Rbufd {
        &self.rbufd
    }
    #[doc = "0x5c - Receiver Buffer Register 0"]
    #[inline(always)]
    pub const fn rbuf0(&self) -> &Rbuf0 {
        &self.rbuf0
    }
    #[doc = "0x60 - Receiver Buffer Register 1"]
    #[inline(always)]
    pub const fn rbuf1(&self) -> &Rbuf1 {
        &self.rbuf1
    }
    #[doc = "0x64 - Receiver Buffer 01 Status Register"]
    #[inline(always)]
    pub const fn rbuf01sr(&self) -> &Rbuf01sr {
        &self.rbuf01sr
    }
    #[doc = "0x68 - Flag Modification Register"]
    #[inline(always)]
    pub const fn fmr(&self) -> &Fmr {
        &self.fmr
    }
    #[doc = "0x80..0x100 - Transmit Buffer"]
    #[inline(always)]
    pub const fn tbuf(&self, n: usize) -> &Tbuf {
        &self.tbuf[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x80..0x100 - Transmit Buffer"]
    #[inline(always)]
    pub fn tbuf_iter(&self) -> impl Iterator<Item = &Tbuf> {
        self.tbuf.iter()
    }
    #[doc = "0x100 - Bypass Data Register"]
    #[inline(always)]
    pub const fn byp(&self) -> &Byp {
        &self.byp
    }
    #[doc = "0x104 - Bypass Control Register"]
    #[inline(always)]
    pub const fn bypcr(&self) -> &Bypcr {
        &self.bypcr
    }
    #[doc = "0x108 - Transmitter Buffer Control Register"]
    #[inline(always)]
    pub const fn tbctr(&self) -> &Tbctr {
        &self.tbctr
    }
    #[doc = "0x10c - Receiver Buffer Control Register"]
    #[inline(always)]
    pub const fn rbctr(&self) -> &Rbctr {
        &self.rbctr
    }
    #[doc = "0x110 - Transmit/Receive Buffer Pointer Register"]
    #[inline(always)]
    pub const fn trbptr(&self) -> &Trbptr {
        &self.trbptr
    }
    #[doc = "0x114 - Transmit/Receive Buffer Status Register"]
    #[inline(always)]
    pub const fn trbsr(&self) -> &Trbsr {
        &self.trbsr
    }
    #[doc = "0x118 - Transmit/Receive Buffer Status Clear Register"]
    #[inline(always)]
    pub const fn trbscr(&self) -> &Trbscr {
        &self.trbscr
    }
    #[doc = "0x11c - Receiver Buffer Output Register"]
    #[inline(always)]
    pub const fn outr(&self) -> &Outr {
        &self.outr
    }
    #[doc = "0x120 - Receiver Buffer Output Register L for Debugger"]
    #[inline(always)]
    pub const fn outdr(&self) -> &Outdr {
        &self.outdr
    }
    #[doc = "0x180..0x200 - Transmit FIFO Buffer"]
    #[inline(always)]
    pub const fn in_(&self, n: usize) -> &In {
        &self.in_[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x180..0x200 - Transmit FIFO Buffer"]
    #[inline(always)]
    pub fn in__iter(&self) -> impl Iterator<Item = &In> {
        self.in_.iter()
    }
}
#[doc = "CCFG (r) register accessor: Channel Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccfg::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccfg`]
module"]
#[doc(alias = "CCFG")]
pub type Ccfg = crate::Reg<ccfg::CcfgSpec>;
#[doc = "Channel Configuration Register"]
pub mod ccfg;
#[doc = "KSCFG (rw) register accessor: Kernel State Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`kscfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`kscfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@kscfg`]
module"]
#[doc(alias = "KSCFG")]
pub type Kscfg = crate::Reg<kscfg::KscfgSpec>;
#[doc = "Kernel State Configuration Register"]
pub mod kscfg;
#[doc = "FDR (rw) register accessor: Fractional Divider Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fdr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fdr`]
module"]
#[doc(alias = "FDR")]
pub type Fdr = crate::Reg<fdr::FdrSpec>;
#[doc = "Fractional Divider Register"]
pub mod fdr;
#[doc = "BRG (rw) register accessor: Baud Rate Generator Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`brg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`brg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@brg`]
module"]
#[doc(alias = "BRG")]
pub type Brg = crate::Reg<brg::BrgSpec>;
#[doc = "Baud Rate Generator Register"]
pub mod brg;
#[doc = "INPR (rw) register accessor: Interrupt Node Pointer Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`inpr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`inpr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@inpr`]
module"]
#[doc(alias = "INPR")]
pub type Inpr = crate::Reg<inpr::InprSpec>;
#[doc = "Interrupt Node Pointer Register"]
pub mod inpr;
#[doc = "DX0CR (rw) register accessor: Input Control Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dx0cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dx0cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dx0cr`]
module"]
#[doc(alias = "DX0CR")]
pub type Dx0cr = crate::Reg<dx0cr::Dx0crSpec>;
#[doc = "Input Control Register 0"]
pub mod dx0cr;
#[doc = "DX1CR (rw) register accessor: Input Control Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dx1cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dx1cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dx1cr`]
module"]
#[doc(alias = "DX1CR")]
pub type Dx1cr = crate::Reg<dx1cr::Dx1crSpec>;
#[doc = "Input Control Register 1"]
pub mod dx1cr;
#[doc = "DX2CR (rw) register accessor: Input Control Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dx2cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dx2cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dx2cr`]
module"]
#[doc(alias = "DX2CR")]
pub type Dx2cr = crate::Reg<dx2cr::Dx2crSpec>;
#[doc = "Input Control Register 2"]
pub mod dx2cr;
#[doc = "DX3CR (rw) register accessor: Input Control Register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dx3cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dx3cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dx3cr`]
module"]
#[doc(alias = "DX3CR")]
pub type Dx3cr = crate::Reg<dx3cr::Dx3crSpec>;
#[doc = "Input Control Register 3"]
pub mod dx3cr;
#[doc = "DX4CR (rw) register accessor: Input Control Register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dx4cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dx4cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dx4cr`]
module"]
#[doc(alias = "DX4CR")]
pub type Dx4cr = crate::Reg<dx4cr::Dx4crSpec>;
#[doc = "Input Control Register 4"]
pub mod dx4cr;
#[doc = "DX5CR (rw) register accessor: Input Control Register 5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dx5cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dx5cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dx5cr`]
module"]
#[doc(alias = "DX5CR")]
pub type Dx5cr = crate::Reg<dx5cr::Dx5crSpec>;
#[doc = "Input Control Register 5"]
pub mod dx5cr;
#[doc = "SCTR (rw) register accessor: Shift Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sctr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sctr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sctr`]
module"]
#[doc(alias = "SCTR")]
pub type Sctr = crate::Reg<sctr::SctrSpec>;
#[doc = "Shift Control Register"]
pub mod sctr;
#[doc = "TCSR (rw) register accessor: Transmit Control/Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tcsr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tcsr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tcsr`]
module"]
#[doc(alias = "TCSR")]
pub type Tcsr = crate::Reg<tcsr::TcsrSpec>;
#[doc = "Transmit Control/Status Register"]
pub mod tcsr;
#[doc = "PCR (rw) register accessor: Protocol Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcr`]
module"]
#[doc(alias = "PCR")]
pub type Pcr = crate::Reg<pcr::PcrSpec>;
#[doc = "Protocol Control Register"]
pub mod pcr;
#[doc = "PCR_ASCMode (rw) register accessor: Protocol Control Register \\[ASC Mode\\]\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcr_ascmode::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcr_ascmode::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcr_ascmode`]
module"]
#[doc(alias = "PCR_ASCMode")]
pub type PcrAscmode = crate::Reg<pcr_ascmode::PcrAscmodeSpec>;
#[doc = "Protocol Control Register \\[ASC Mode\\]"]
pub mod pcr_ascmode;
#[doc = "PCR_SSCMode (rw) register accessor: Protocol Control Register \\[SSC Mode\\]\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcr_sscmode::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcr_sscmode::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcr_sscmode`]
module"]
#[doc(alias = "PCR_SSCMode")]
pub type PcrSscmode = crate::Reg<pcr_sscmode::PcrSscmodeSpec>;
#[doc = "Protocol Control Register \\[SSC Mode\\]"]
pub mod pcr_sscmode;
#[doc = "PCR_IICMode (rw) register accessor: Protocol Control Register \\[IIC Mode\\]\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcr_iicmode::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcr_iicmode::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcr_iicmode`]
module"]
#[doc(alias = "PCR_IICMode")]
pub type PcrIicmode = crate::Reg<pcr_iicmode::PcrIicmodeSpec>;
#[doc = "Protocol Control Register \\[IIC Mode\\]"]
pub mod pcr_iicmode;
#[doc = "PCR_IISMode (rw) register accessor: Protocol Control Register \\[IIS Mode\\]\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcr_iismode::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcr_iismode::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcr_iismode`]
module"]
#[doc(alias = "PCR_IISMode")]
pub type PcrIismode = crate::Reg<pcr_iismode::PcrIismodeSpec>;
#[doc = "Protocol Control Register \\[IIS Mode\\]"]
pub mod pcr_iismode;
#[doc = "CCR (rw) register accessor: Channel Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccr`]
module"]
#[doc(alias = "CCR")]
pub type Ccr = crate::Reg<ccr::CcrSpec>;
#[doc = "Channel Control Register"]
pub mod ccr;
#[doc = "CMTR (rw) register accessor: Capture Mode Timer Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmtr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmtr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmtr`]
module"]
#[doc(alias = "CMTR")]
pub type Cmtr = crate::Reg<cmtr::CmtrSpec>;
#[doc = "Capture Mode Timer Register"]
pub mod cmtr;
#[doc = "PSR (rw) register accessor: Protocol Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`psr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`psr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@psr`]
module"]
#[doc(alias = "PSR")]
pub type Psr = crate::Reg<psr::PsrSpec>;
#[doc = "Protocol Status Register"]
pub mod psr;
#[doc = "PSR_ASCMode (rw) register accessor: Protocol Status Register \\[ASC Mode\\]\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`psr_ascmode::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`psr_ascmode::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@psr_ascmode`]
module"]
#[doc(alias = "PSR_ASCMode")]
pub type PsrAscmode = crate::Reg<psr_ascmode::PsrAscmodeSpec>;
#[doc = "Protocol Status Register \\[ASC Mode\\]"]
pub mod psr_ascmode;
#[doc = "PSR_SSCMode (rw) register accessor: Protocol Status Register \\[SSC Mode\\]\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`psr_sscmode::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`psr_sscmode::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@psr_sscmode`]
module"]
#[doc(alias = "PSR_SSCMode")]
pub type PsrSscmode = crate::Reg<psr_sscmode::PsrSscmodeSpec>;
#[doc = "Protocol Status Register \\[SSC Mode\\]"]
pub mod psr_sscmode;
#[doc = "PSR_IICMode (rw) register accessor: Protocol Status Register \\[IIC Mode\\]\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`psr_iicmode::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`psr_iicmode::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@psr_iicmode`]
module"]
#[doc(alias = "PSR_IICMode")]
pub type PsrIicmode = crate::Reg<psr_iicmode::PsrIicmodeSpec>;
#[doc = "Protocol Status Register \\[IIC Mode\\]"]
pub mod psr_iicmode;
#[doc = "PSR_IISMode (rw) register accessor: Protocol Status Register \\[IIS Mode\\]\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`psr_iismode::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`psr_iismode::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@psr_iismode`]
module"]
#[doc(alias = "PSR_IISMode")]
pub type PsrIismode = crate::Reg<psr_iismode::PsrIismodeSpec>;
#[doc = "Protocol Status Register \\[IIS Mode\\]"]
pub mod psr_iismode;
#[doc = "PSCR (w) register accessor: Protocol Status Clear Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pscr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pscr`]
module"]
#[doc(alias = "PSCR")]
pub type Pscr = crate::Reg<pscr::PscrSpec>;
#[doc = "Protocol Status Clear Register"]
pub mod pscr;
#[doc = "RBUFSR (r) register accessor: Receiver Buffer Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rbufsr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rbufsr`]
module"]
#[doc(alias = "RBUFSR")]
pub type Rbufsr = crate::Reg<rbufsr::RbufsrSpec>;
#[doc = "Receiver Buffer Status Register"]
pub mod rbufsr;
#[doc = "RBUF (r) register accessor: Receiver Buffer Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rbuf::R`]. WARN: One or more dependent resources other than the current register are immediately affected by a read operation. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rbuf`]
module"]
#[doc(alias = "RBUF")]
pub type Rbuf = crate::Reg<rbuf::RbufSpec>;
#[doc = "Receiver Buffer Register"]
pub mod rbuf;
#[doc = "RBUFD (r) register accessor: Receiver Buffer Register for Debugger\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rbufd::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rbufd`]
module"]
#[doc(alias = "RBUFD")]
pub type Rbufd = crate::Reg<rbufd::RbufdSpec>;
#[doc = "Receiver Buffer Register for Debugger"]
pub mod rbufd;
#[doc = "RBUF0 (r) register accessor: Receiver Buffer Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rbuf0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rbuf0`]
module"]
#[doc(alias = "RBUF0")]
pub type Rbuf0 = crate::Reg<rbuf0::Rbuf0Spec>;
#[doc = "Receiver Buffer Register 0"]
pub mod rbuf0;
#[doc = "RBUF1 (r) register accessor: Receiver Buffer Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rbuf1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rbuf1`]
module"]
#[doc(alias = "RBUF1")]
pub type Rbuf1 = crate::Reg<rbuf1::Rbuf1Spec>;
#[doc = "Receiver Buffer Register 1"]
pub mod rbuf1;
#[doc = "RBUF01SR (r) register accessor: Receiver Buffer 01 Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rbuf01sr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rbuf01sr`]
module"]
#[doc(alias = "RBUF01SR")]
pub type Rbuf01sr = crate::Reg<rbuf01sr::Rbuf01srSpec>;
#[doc = "Receiver Buffer 01 Status Register"]
pub mod rbuf01sr;
#[doc = "FMR (w) register accessor: Flag Modification Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fmr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fmr`]
module"]
#[doc(alias = "FMR")]
pub type Fmr = crate::Reg<fmr::FmrSpec>;
#[doc = "Flag Modification Register"]
pub mod fmr;
#[doc = "TBUF (rw) register accessor: Transmit Buffer\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tbuf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tbuf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tbuf`]
module"]
#[doc(alias = "TBUF")]
pub type Tbuf = crate::Reg<tbuf::TbufSpec>;
#[doc = "Transmit Buffer"]
pub mod tbuf;
#[doc = "BYP (rw) register accessor: Bypass Data Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`byp::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`byp::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@byp`]
module"]
#[doc(alias = "BYP")]
pub type Byp = crate::Reg<byp::BypSpec>;
#[doc = "Bypass Data Register"]
pub mod byp;
#[doc = "BYPCR (rw) register accessor: Bypass Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bypcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bypcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bypcr`]
module"]
#[doc(alias = "BYPCR")]
pub type Bypcr = crate::Reg<bypcr::BypcrSpec>;
#[doc = "Bypass Control Register"]
pub mod bypcr;
#[doc = "TBCTR (rw) register accessor: Transmitter Buffer Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tbctr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tbctr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tbctr`]
module"]
#[doc(alias = "TBCTR")]
pub type Tbctr = crate::Reg<tbctr::TbctrSpec>;
#[doc = "Transmitter Buffer Control Register"]
pub mod tbctr;
#[doc = "RBCTR (rw) register accessor: Receiver Buffer Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rbctr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rbctr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rbctr`]
module"]
#[doc(alias = "RBCTR")]
pub type Rbctr = crate::Reg<rbctr::RbctrSpec>;
#[doc = "Receiver Buffer Control Register"]
pub mod rbctr;
#[doc = "TRBPTR (r) register accessor: Transmit/Receive Buffer Pointer Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`trbptr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@trbptr`]
module"]
#[doc(alias = "TRBPTR")]
pub type Trbptr = crate::Reg<trbptr::TrbptrSpec>;
#[doc = "Transmit/Receive Buffer Pointer Register"]
pub mod trbptr;
#[doc = "TRBSR (rw) register accessor: Transmit/Receive Buffer Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`trbsr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`trbsr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@trbsr`]
module"]
#[doc(alias = "TRBSR")]
pub type Trbsr = crate::Reg<trbsr::TrbsrSpec>;
#[doc = "Transmit/Receive Buffer Status Register"]
pub mod trbsr;
#[doc = "TRBSCR (w) register accessor: Transmit/Receive Buffer Status Clear Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`trbscr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@trbscr`]
module"]
#[doc(alias = "TRBSCR")]
pub type Trbscr = crate::Reg<trbscr::TrbscrSpec>;
#[doc = "Transmit/Receive Buffer Status Clear Register"]
pub mod trbscr;
#[doc = "OUTR (r) register accessor: Receiver Buffer Output Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`outr::R`]. WARN: One or more dependent resources other than the current register are immediately affected by a read operation. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@outr`]
module"]
#[doc(alias = "OUTR")]
pub type Outr = crate::Reg<outr::OutrSpec>;
#[doc = "Receiver Buffer Output Register"]
pub mod outr;
#[doc = "OUTDR (r) register accessor: Receiver Buffer Output Register L for Debugger\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`outdr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@outdr`]
module"]
#[doc(alias = "OUTDR")]
pub type Outdr = crate::Reg<outdr::OutdrSpec>;
#[doc = "Receiver Buffer Output Register L for Debugger"]
pub mod outdr;
#[doc = "IN (w) register accessor: Transmit FIFO Buffer\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`in_::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_`]
module"]
#[doc(alias = "IN")]
pub type In = crate::Reg<in_::InSpec>;
#[doc = "Transmit FIFO Buffer"]
pub mod in_;
