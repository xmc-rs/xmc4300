#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x04],
    ccfg: CCFG,
    _reserved1: [u8; 0x04],
    kscfg: KSCFG,
    fdr: FDR,
    brg: BRG,
    inpr: INPR,
    dx0cr: DX0CR,
    dx1cr: DX1CR,
    dx2cr: DX2CR,
    dx3cr: DX3CR,
    dx4cr: DX4CR,
    dx5cr: DX5CR,
    sctr: SCTR,
    tcsr: TCSR,
    _reserved_13_pcr: [u8; 0x04],
    ccr: CCR,
    cmtr: CMTR,
    _reserved_16_psr: [u8; 0x04],
    pscr: PSCR,
    rbufsr: RBUFSR,
    rbuf: RBUF,
    rbufd: RBUFD,
    rbuf0: RBUF0,
    rbuf1: RBUF1,
    rbuf01sr: RBUF01SR,
    fmr: FMR,
    _reserved25: [u8; 0x14],
    tbuf: [TBUF; 32],
    byp: BYP,
    bypcr: BYPCR,
    tbctr: TBCTR,
    rbctr: RBCTR,
    trbptr: TRBPTR,
    trbsr: TRBSR,
    trbscr: TRBSCR,
    outr: OUTR,
    outdr: OUTDR,
    _reserved35: [u8; 0x5c],
    in_: [IN; 32],
}
impl RegisterBlock {
    #[doc = "0x04 - Channel Configuration Register"]
    #[inline(always)]
    pub const fn ccfg(&self) -> &CCFG {
        &self.ccfg
    }
    #[doc = "0x0c - Kernel State Configuration Register"]
    #[inline(always)]
    pub const fn kscfg(&self) -> &KSCFG {
        &self.kscfg
    }
    #[doc = "0x10 - Fractional Divider Register"]
    #[inline(always)]
    pub const fn fdr(&self) -> &FDR {
        &self.fdr
    }
    #[doc = "0x14 - Baud Rate Generator Register"]
    #[inline(always)]
    pub const fn brg(&self) -> &BRG {
        &self.brg
    }
    #[doc = "0x18 - Interrupt Node Pointer Register"]
    #[inline(always)]
    pub const fn inpr(&self) -> &INPR {
        &self.inpr
    }
    #[doc = "0x1c - Input Control Register 0"]
    #[inline(always)]
    pub const fn dx0cr(&self) -> &DX0CR {
        &self.dx0cr
    }
    #[doc = "0x20 - Input Control Register 1"]
    #[inline(always)]
    pub const fn dx1cr(&self) -> &DX1CR {
        &self.dx1cr
    }
    #[doc = "0x24 - Input Control Register 2"]
    #[inline(always)]
    pub const fn dx2cr(&self) -> &DX2CR {
        &self.dx2cr
    }
    #[doc = "0x28 - Input Control Register 3"]
    #[inline(always)]
    pub const fn dx3cr(&self) -> &DX3CR {
        &self.dx3cr
    }
    #[doc = "0x2c - Input Control Register 4"]
    #[inline(always)]
    pub const fn dx4cr(&self) -> &DX4CR {
        &self.dx4cr
    }
    #[doc = "0x30 - Input Control Register 5"]
    #[inline(always)]
    pub const fn dx5cr(&self) -> &DX5CR {
        &self.dx5cr
    }
    #[doc = "0x34 - Shift Control Register"]
    #[inline(always)]
    pub const fn sctr(&self) -> &SCTR {
        &self.sctr
    }
    #[doc = "0x38 - Transmit Control/Status Register"]
    #[inline(always)]
    pub const fn tcsr(&self) -> &TCSR {
        &self.tcsr
    }
    #[doc = "0x3c - Protocol Control Register \\[IIS Mode\\]"]
    #[inline(always)]
    pub const fn pcr_iismode(&self) -> &PCR_IISMODE {
        unsafe { &*(self as *const Self).cast::<u8>().add(60).cast() }
    }
    #[doc = "0x3c - Protocol Control Register \\[IIC Mode\\]"]
    #[inline(always)]
    pub const fn pcr_iicmode(&self) -> &PCR_IICMODE {
        unsafe { &*(self as *const Self).cast::<u8>().add(60).cast() }
    }
    #[doc = "0x3c - Protocol Control Register \\[SSC Mode\\]"]
    #[inline(always)]
    pub const fn pcr_sscmode(&self) -> &PCR_SSCMODE {
        unsafe { &*(self as *const Self).cast::<u8>().add(60).cast() }
    }
    #[doc = "0x3c - Protocol Control Register \\[ASC Mode\\]"]
    #[inline(always)]
    pub const fn pcr_ascmode(&self) -> &PCR_ASCMODE {
        unsafe { &*(self as *const Self).cast::<u8>().add(60).cast() }
    }
    #[doc = "0x3c - Protocol Control Register"]
    #[inline(always)]
    pub const fn pcr(&self) -> &PCR {
        unsafe { &*(self as *const Self).cast::<u8>().add(60).cast() }
    }
    #[doc = "0x40 - Channel Control Register"]
    #[inline(always)]
    pub const fn ccr(&self) -> &CCR {
        &self.ccr
    }
    #[doc = "0x44 - Capture Mode Timer Register"]
    #[inline(always)]
    pub const fn cmtr(&self) -> &CMTR {
        &self.cmtr
    }
    #[doc = "0x48 - Protocol Status Register \\[IIS Mode\\]"]
    #[inline(always)]
    pub const fn psr_iismode(&self) -> &PSR_IISMODE {
        unsafe { &*(self as *const Self).cast::<u8>().add(72).cast() }
    }
    #[doc = "0x48 - Protocol Status Register \\[IIC Mode\\]"]
    #[inline(always)]
    pub const fn psr_iicmode(&self) -> &PSR_IICMODE {
        unsafe { &*(self as *const Self).cast::<u8>().add(72).cast() }
    }
    #[doc = "0x48 - Protocol Status Register \\[SSC Mode\\]"]
    #[inline(always)]
    pub const fn psr_sscmode(&self) -> &PSR_SSCMODE {
        unsafe { &*(self as *const Self).cast::<u8>().add(72).cast() }
    }
    #[doc = "0x48 - Protocol Status Register \\[ASC Mode\\]"]
    #[inline(always)]
    pub const fn psr_ascmode(&self) -> &PSR_ASCMODE {
        unsafe { &*(self as *const Self).cast::<u8>().add(72).cast() }
    }
    #[doc = "0x48 - Protocol Status Register"]
    #[inline(always)]
    pub const fn psr(&self) -> &PSR {
        unsafe { &*(self as *const Self).cast::<u8>().add(72).cast() }
    }
    #[doc = "0x4c - Protocol Status Clear Register"]
    #[inline(always)]
    pub const fn pscr(&self) -> &PSCR {
        &self.pscr
    }
    #[doc = "0x50 - Receiver Buffer Status Register"]
    #[inline(always)]
    pub const fn rbufsr(&self) -> &RBUFSR {
        &self.rbufsr
    }
    #[doc = "0x54 - Receiver Buffer Register"]
    #[inline(always)]
    pub const fn rbuf(&self) -> &RBUF {
        &self.rbuf
    }
    #[doc = "0x58 - Receiver Buffer Register for Debugger"]
    #[inline(always)]
    pub const fn rbufd(&self) -> &RBUFD {
        &self.rbufd
    }
    #[doc = "0x5c - Receiver Buffer Register 0"]
    #[inline(always)]
    pub const fn rbuf0(&self) -> &RBUF0 {
        &self.rbuf0
    }
    #[doc = "0x60 - Receiver Buffer Register 1"]
    #[inline(always)]
    pub const fn rbuf1(&self) -> &RBUF1 {
        &self.rbuf1
    }
    #[doc = "0x64 - Receiver Buffer 01 Status Register"]
    #[inline(always)]
    pub const fn rbuf01sr(&self) -> &RBUF01SR {
        &self.rbuf01sr
    }
    #[doc = "0x68 - Flag Modification Register"]
    #[inline(always)]
    pub const fn fmr(&self) -> &FMR {
        &self.fmr
    }
    #[doc = "0x80..0x100 - Transmit Buffer"]
    #[inline(always)]
    pub const fn tbuf(&self, n: usize) -> &TBUF {
        &self.tbuf[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x80..0x100 - Transmit Buffer"]
    #[inline(always)]
    pub fn tbuf_iter(&self) -> impl Iterator<Item = &TBUF> {
        self.tbuf.iter()
    }
    #[doc = "0x100 - Bypass Data Register"]
    #[inline(always)]
    pub const fn byp(&self) -> &BYP {
        &self.byp
    }
    #[doc = "0x104 - Bypass Control Register"]
    #[inline(always)]
    pub const fn bypcr(&self) -> &BYPCR {
        &self.bypcr
    }
    #[doc = "0x108 - Transmitter Buffer Control Register"]
    #[inline(always)]
    pub const fn tbctr(&self) -> &TBCTR {
        &self.tbctr
    }
    #[doc = "0x10c - Receiver Buffer Control Register"]
    #[inline(always)]
    pub const fn rbctr(&self) -> &RBCTR {
        &self.rbctr
    }
    #[doc = "0x110 - Transmit/Receive Buffer Pointer Register"]
    #[inline(always)]
    pub const fn trbptr(&self) -> &TRBPTR {
        &self.trbptr
    }
    #[doc = "0x114 - Transmit/Receive Buffer Status Register"]
    #[inline(always)]
    pub const fn trbsr(&self) -> &TRBSR {
        &self.trbsr
    }
    #[doc = "0x118 - Transmit/Receive Buffer Status Clear Register"]
    #[inline(always)]
    pub const fn trbscr(&self) -> &TRBSCR {
        &self.trbscr
    }
    #[doc = "0x11c - Receiver Buffer Output Register"]
    #[inline(always)]
    pub const fn outr(&self) -> &OUTR {
        &self.outr
    }
    #[doc = "0x120 - Receiver Buffer Output Register L for Debugger"]
    #[inline(always)]
    pub const fn outdr(&self) -> &OUTDR {
        &self.outdr
    }
    #[doc = "0x180..0x200 - Transmit FIFO Buffer"]
    #[inline(always)]
    pub const fn in_(&self, n: usize) -> &IN {
        &self.in_[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x180..0x200 - Transmit FIFO Buffer"]
    #[inline(always)]
    pub fn in__iter(&self) -> impl Iterator<Item = &IN> {
        self.in_.iter()
    }
}
#[doc = "CCFG (r) register accessor: Channel Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccfg::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccfg`]
module"]
pub type CCFG = crate::Reg<ccfg::CCFG_SPEC>;
#[doc = "Channel Configuration Register"]
pub mod ccfg;
#[doc = "KSCFG (rw) register accessor: Kernel State Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`kscfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`kscfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@kscfg`]
module"]
pub type KSCFG = crate::Reg<kscfg::KSCFG_SPEC>;
#[doc = "Kernel State Configuration Register"]
pub mod kscfg;
#[doc = "FDR (rw) register accessor: Fractional Divider Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fdr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fdr`]
module"]
pub type FDR = crate::Reg<fdr::FDR_SPEC>;
#[doc = "Fractional Divider Register"]
pub mod fdr;
#[doc = "BRG (rw) register accessor: Baud Rate Generator Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`brg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`brg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@brg`]
module"]
pub type BRG = crate::Reg<brg::BRG_SPEC>;
#[doc = "Baud Rate Generator Register"]
pub mod brg;
#[doc = "INPR (rw) register accessor: Interrupt Node Pointer Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`inpr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`inpr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@inpr`]
module"]
pub type INPR = crate::Reg<inpr::INPR_SPEC>;
#[doc = "Interrupt Node Pointer Register"]
pub mod inpr;
#[doc = "DX0CR (rw) register accessor: Input Control Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dx0cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dx0cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dx0cr`]
module"]
pub type DX0CR = crate::Reg<dx0cr::DX0CR_SPEC>;
#[doc = "Input Control Register 0"]
pub mod dx0cr;
#[doc = "DX1CR (rw) register accessor: Input Control Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dx1cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dx1cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dx1cr`]
module"]
pub type DX1CR = crate::Reg<dx1cr::DX1CR_SPEC>;
#[doc = "Input Control Register 1"]
pub mod dx1cr;
#[doc = "DX2CR (rw) register accessor: Input Control Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dx2cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dx2cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dx2cr`]
module"]
pub type DX2CR = crate::Reg<dx2cr::DX2CR_SPEC>;
#[doc = "Input Control Register 2"]
pub mod dx2cr;
#[doc = "DX3CR (rw) register accessor: Input Control Register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dx3cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dx3cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dx3cr`]
module"]
pub type DX3CR = crate::Reg<dx3cr::DX3CR_SPEC>;
#[doc = "Input Control Register 3"]
pub mod dx3cr;
#[doc = "DX4CR (rw) register accessor: Input Control Register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dx4cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dx4cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dx4cr`]
module"]
pub type DX4CR = crate::Reg<dx4cr::DX4CR_SPEC>;
#[doc = "Input Control Register 4"]
pub mod dx4cr;
#[doc = "DX5CR (rw) register accessor: Input Control Register 5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dx5cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dx5cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dx5cr`]
module"]
pub type DX5CR = crate::Reg<dx5cr::DX5CR_SPEC>;
#[doc = "Input Control Register 5"]
pub mod dx5cr;
#[doc = "SCTR (rw) register accessor: Shift Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sctr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sctr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sctr`]
module"]
pub type SCTR = crate::Reg<sctr::SCTR_SPEC>;
#[doc = "Shift Control Register"]
pub mod sctr;
#[doc = "TCSR (rw) register accessor: Transmit Control/Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tcsr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tcsr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tcsr`]
module"]
pub type TCSR = crate::Reg<tcsr::TCSR_SPEC>;
#[doc = "Transmit Control/Status Register"]
pub mod tcsr;
#[doc = "PCR (rw) register accessor: Protocol Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcr`]
module"]
pub type PCR = crate::Reg<pcr::PCR_SPEC>;
#[doc = "Protocol Control Register"]
pub mod pcr;
#[doc = "PCR_ASCMode (rw) register accessor: Protocol Control Register \\[ASC Mode\\]\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcr_ascmode::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcr_ascmode::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcr_ascmode`]
module"]
pub type PCR_ASCMODE = crate::Reg<pcr_ascmode::PCR_ASCMODE_SPEC>;
#[doc = "Protocol Control Register \\[ASC Mode\\]"]
pub mod pcr_ascmode;
#[doc = "PCR_SSCMode (rw) register accessor: Protocol Control Register \\[SSC Mode\\]\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcr_sscmode::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcr_sscmode::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcr_sscmode`]
module"]
pub type PCR_SSCMODE = crate::Reg<pcr_sscmode::PCR_SSCMODE_SPEC>;
#[doc = "Protocol Control Register \\[SSC Mode\\]"]
pub mod pcr_sscmode;
#[doc = "PCR_IICMode (rw) register accessor: Protocol Control Register \\[IIC Mode\\]\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcr_iicmode::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcr_iicmode::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcr_iicmode`]
module"]
pub type PCR_IICMODE = crate::Reg<pcr_iicmode::PCR_IICMODE_SPEC>;
#[doc = "Protocol Control Register \\[IIC Mode\\]"]
pub mod pcr_iicmode;
#[doc = "PCR_IISMode (rw) register accessor: Protocol Control Register \\[IIS Mode\\]\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcr_iismode::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcr_iismode::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcr_iismode`]
module"]
pub type PCR_IISMODE = crate::Reg<pcr_iismode::PCR_IISMODE_SPEC>;
#[doc = "Protocol Control Register \\[IIS Mode\\]"]
pub mod pcr_iismode;
#[doc = "CCR (rw) register accessor: Channel Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccr`]
module"]
pub type CCR = crate::Reg<ccr::CCR_SPEC>;
#[doc = "Channel Control Register"]
pub mod ccr;
#[doc = "CMTR (rw) register accessor: Capture Mode Timer Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmtr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmtr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmtr`]
module"]
pub type CMTR = crate::Reg<cmtr::CMTR_SPEC>;
#[doc = "Capture Mode Timer Register"]
pub mod cmtr;
#[doc = "PSR (rw) register accessor: Protocol Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`psr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`psr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@psr`]
module"]
pub type PSR = crate::Reg<psr::PSR_SPEC>;
#[doc = "Protocol Status Register"]
pub mod psr;
#[doc = "PSR_ASCMode (rw) register accessor: Protocol Status Register \\[ASC Mode\\]\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`psr_ascmode::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`psr_ascmode::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@psr_ascmode`]
module"]
pub type PSR_ASCMODE = crate::Reg<psr_ascmode::PSR_ASCMODE_SPEC>;
#[doc = "Protocol Status Register \\[ASC Mode\\]"]
pub mod psr_ascmode;
#[doc = "PSR_SSCMode (rw) register accessor: Protocol Status Register \\[SSC Mode\\]\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`psr_sscmode::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`psr_sscmode::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@psr_sscmode`]
module"]
pub type PSR_SSCMODE = crate::Reg<psr_sscmode::PSR_SSCMODE_SPEC>;
#[doc = "Protocol Status Register \\[SSC Mode\\]"]
pub mod psr_sscmode;
#[doc = "PSR_IICMode (rw) register accessor: Protocol Status Register \\[IIC Mode\\]\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`psr_iicmode::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`psr_iicmode::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@psr_iicmode`]
module"]
pub type PSR_IICMODE = crate::Reg<psr_iicmode::PSR_IICMODE_SPEC>;
#[doc = "Protocol Status Register \\[IIC Mode\\]"]
pub mod psr_iicmode;
#[doc = "PSR_IISMode (rw) register accessor: Protocol Status Register \\[IIS Mode\\]\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`psr_iismode::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`psr_iismode::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@psr_iismode`]
module"]
pub type PSR_IISMODE = crate::Reg<psr_iismode::PSR_IISMODE_SPEC>;
#[doc = "Protocol Status Register \\[IIS Mode\\]"]
pub mod psr_iismode;
#[doc = "PSCR (w) register accessor: Protocol Status Clear Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pscr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pscr`]
module"]
pub type PSCR = crate::Reg<pscr::PSCR_SPEC>;
#[doc = "Protocol Status Clear Register"]
pub mod pscr;
#[doc = "RBUFSR (r) register accessor: Receiver Buffer Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rbufsr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rbufsr`]
module"]
pub type RBUFSR = crate::Reg<rbufsr::RBUFSR_SPEC>;
#[doc = "Receiver Buffer Status Register"]
pub mod rbufsr;
#[doc = "RBUF (r) register accessor: Receiver Buffer Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rbuf::R`]. WARN: One or more dependent resources other than the current register are immediately affected by a read operation. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rbuf`]
module"]
pub type RBUF = crate::Reg<rbuf::RBUF_SPEC>;
#[doc = "Receiver Buffer Register"]
pub mod rbuf;
#[doc = "RBUFD (r) register accessor: Receiver Buffer Register for Debugger\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rbufd::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rbufd`]
module"]
pub type RBUFD = crate::Reg<rbufd::RBUFD_SPEC>;
#[doc = "Receiver Buffer Register for Debugger"]
pub mod rbufd;
#[doc = "RBUF0 (r) register accessor: Receiver Buffer Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rbuf0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rbuf0`]
module"]
pub type RBUF0 = crate::Reg<rbuf0::RBUF0_SPEC>;
#[doc = "Receiver Buffer Register 0"]
pub mod rbuf0;
#[doc = "RBUF1 (r) register accessor: Receiver Buffer Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rbuf1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rbuf1`]
module"]
pub type RBUF1 = crate::Reg<rbuf1::RBUF1_SPEC>;
#[doc = "Receiver Buffer Register 1"]
pub mod rbuf1;
#[doc = "RBUF01SR (r) register accessor: Receiver Buffer 01 Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rbuf01sr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rbuf01sr`]
module"]
pub type RBUF01SR = crate::Reg<rbuf01sr::RBUF01SR_SPEC>;
#[doc = "Receiver Buffer 01 Status Register"]
pub mod rbuf01sr;
#[doc = "FMR (w) register accessor: Flag Modification Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fmr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fmr`]
module"]
pub type FMR = crate::Reg<fmr::FMR_SPEC>;
#[doc = "Flag Modification Register"]
pub mod fmr;
#[doc = "TBUF (rw) register accessor: Transmit Buffer\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tbuf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tbuf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tbuf`]
module"]
pub type TBUF = crate::Reg<tbuf::TBUF_SPEC>;
#[doc = "Transmit Buffer"]
pub mod tbuf;
#[doc = "BYP (rw) register accessor: Bypass Data Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`byp::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`byp::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@byp`]
module"]
pub type BYP = crate::Reg<byp::BYP_SPEC>;
#[doc = "Bypass Data Register"]
pub mod byp;
#[doc = "BYPCR (rw) register accessor: Bypass Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bypcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bypcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bypcr`]
module"]
pub type BYPCR = crate::Reg<bypcr::BYPCR_SPEC>;
#[doc = "Bypass Control Register"]
pub mod bypcr;
#[doc = "TBCTR (rw) register accessor: Transmitter Buffer Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tbctr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tbctr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tbctr`]
module"]
pub type TBCTR = crate::Reg<tbctr::TBCTR_SPEC>;
#[doc = "Transmitter Buffer Control Register"]
pub mod tbctr;
#[doc = "RBCTR (rw) register accessor: Receiver Buffer Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rbctr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rbctr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rbctr`]
module"]
pub type RBCTR = crate::Reg<rbctr::RBCTR_SPEC>;
#[doc = "Receiver Buffer Control Register"]
pub mod rbctr;
#[doc = "TRBPTR (r) register accessor: Transmit/Receive Buffer Pointer Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`trbptr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@trbptr`]
module"]
pub type TRBPTR = crate::Reg<trbptr::TRBPTR_SPEC>;
#[doc = "Transmit/Receive Buffer Pointer Register"]
pub mod trbptr;
#[doc = "TRBSR (rw) register accessor: Transmit/Receive Buffer Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`trbsr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`trbsr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@trbsr`]
module"]
pub type TRBSR = crate::Reg<trbsr::TRBSR_SPEC>;
#[doc = "Transmit/Receive Buffer Status Register"]
pub mod trbsr;
#[doc = "TRBSCR (w) register accessor: Transmit/Receive Buffer Status Clear Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`trbscr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@trbscr`]
module"]
pub type TRBSCR = crate::Reg<trbscr::TRBSCR_SPEC>;
#[doc = "Transmit/Receive Buffer Status Clear Register"]
pub mod trbscr;
#[doc = "OUTR (r) register accessor: Receiver Buffer Output Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`outr::R`]. WARN: One or more dependent resources other than the current register are immediately affected by a read operation. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@outr`]
module"]
pub type OUTR = crate::Reg<outr::OUTR_SPEC>;
#[doc = "Receiver Buffer Output Register"]
pub mod outr;
#[doc = "OUTDR (r) register accessor: Receiver Buffer Output Register L for Debugger\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`outdr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@outdr`]
module"]
pub type OUTDR = crate::Reg<outdr::OUTDR_SPEC>;
#[doc = "Receiver Buffer Output Register L for Debugger"]
pub mod outdr;
#[doc = "IN (w) register accessor: Transmit FIFO Buffer\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`in_::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_`]
module"]
pub type IN = crate::Reg<in_::IN_SPEC>;
#[doc = "Transmit FIFO Buffer"]
pub mod in_;
