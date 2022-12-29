#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x04],
    #[doc = "0x04 - Channel Configuration Register"]
    pub ccfg: CCFG,
    _reserved1: [u8; 0x04],
    #[doc = "0x0c - Kernel State Configuration Register"]
    pub kscfg: KSCFG,
    #[doc = "0x10 - Fractional Divider Register"]
    pub fdr: FDR,
    #[doc = "0x14 - Baud Rate Generator Register"]
    pub brg: BRG,
    #[doc = "0x18 - Interrupt Node Pointer Register"]
    pub inpr: INPR,
    #[doc = "0x1c - Input Control Register 0"]
    pub dx0cr: DX0CR,
    #[doc = "0x20 - Input Control Register 1"]
    pub dx1cr: DX1CR,
    #[doc = "0x24 - Input Control Register 2"]
    pub dx2cr: DX2CR,
    #[doc = "0x28 - Input Control Register 3"]
    pub dx3cr: DX3CR,
    #[doc = "0x2c - Input Control Register 4"]
    pub dx4cr: DX4CR,
    #[doc = "0x30 - Input Control Register 5"]
    pub dx5cr: DX5CR,
    #[doc = "0x34 - Shift Control Register"]
    pub sctr: SCTR,
    #[doc = "0x38 - Transmit Control/Status Register"]
    pub tcsr: TCSR,
    _reserved_13_pcr: [u8; 0x04],
    #[doc = "0x40 - Channel Control Register"]
    pub ccr: CCR,
    #[doc = "0x44 - Capture Mode Timer Register"]
    pub cmtr: CMTR,
    _reserved_16_psr: [u8; 0x04],
    #[doc = "0x4c - Protocol Status Clear Register"]
    pub pscr: PSCR,
    #[doc = "0x50 - Receiver Buffer Status Register"]
    pub rbufsr: RBUFSR,
    #[doc = "0x54 - Receiver Buffer Register"]
    pub rbuf: RBUF,
    #[doc = "0x58 - Receiver Buffer Register for Debugger"]
    pub rbufd: RBUFD,
    #[doc = "0x5c - Receiver Buffer Register 0"]
    pub rbuf0: RBUF0,
    #[doc = "0x60 - Receiver Buffer Register 1"]
    pub rbuf1: RBUF1,
    #[doc = "0x64 - Receiver Buffer 01 Status Register"]
    pub rbuf01sr: RBUF01SR,
    #[doc = "0x68 - Flag Modification Register"]
    pub fmr: FMR,
    _reserved25: [u8; 0x14],
    #[doc = "0x80..0x100 - Transmit Buffer"]
    pub tbuf: [TBUF; 32],
    #[doc = "0x100 - Bypass Data Register"]
    pub byp: BYP,
    #[doc = "0x104 - Bypass Control Register"]
    pub bypcr: BYPCR,
    #[doc = "0x108 - Transmitter Buffer Control Register"]
    pub tbctr: TBCTR,
    #[doc = "0x10c - Receiver Buffer Control Register"]
    pub rbctr: RBCTR,
    #[doc = "0x110 - Transmit/Receive Buffer Pointer Register"]
    pub trbptr: TRBPTR,
    #[doc = "0x114 - Transmit/Receive Buffer Status Register"]
    pub trbsr: TRBSR,
    #[doc = "0x118 - Transmit/Receive Buffer Status Clear Register"]
    pub trbscr: TRBSCR,
    #[doc = "0x11c - Receiver Buffer Output Register"]
    pub outr: OUTR,
    #[doc = "0x120 - Receiver Buffer Output Register L for Debugger"]
    pub outdr: OUTDR,
    _reserved35: [u8; 0x5c],
    #[doc = "0x180..0x200 - Transmit FIFO Buffer"]
    pub in_: [IN; 32],
}
impl RegisterBlock {
    #[doc = "0x3c - Protocol Control Register \\[IIS Mode\\]"]
    #[inline(always)]
    pub const fn pcr_iismode(&self) -> &PCR_IISMODE {
        unsafe { &*(self as *const Self).cast::<u8>().add(60usize).cast() }
    }
    #[doc = "0x3c - Protocol Control Register \\[IIC Mode\\]"]
    #[inline(always)]
    pub const fn pcr_iicmode(&self) -> &PCR_IICMODE {
        unsafe { &*(self as *const Self).cast::<u8>().add(60usize).cast() }
    }
    #[doc = "0x3c - Protocol Control Register \\[SSC Mode\\]"]
    #[inline(always)]
    pub const fn pcr_sscmode(&self) -> &PCR_SSCMODE {
        unsafe { &*(self as *const Self).cast::<u8>().add(60usize).cast() }
    }
    #[doc = "0x3c - Protocol Control Register \\[ASC Mode\\]"]
    #[inline(always)]
    pub const fn pcr_ascmode(&self) -> &PCR_ASCMODE {
        unsafe { &*(self as *const Self).cast::<u8>().add(60usize).cast() }
    }
    #[doc = "0x3c - Protocol Control Register"]
    #[inline(always)]
    pub const fn pcr(&self) -> &PCR {
        unsafe { &*(self as *const Self).cast::<u8>().add(60usize).cast() }
    }
    #[doc = "0x48 - Protocol Status Register \\[IIS Mode\\]"]
    #[inline(always)]
    pub const fn psr_iismode(&self) -> &PSR_IISMODE {
        unsafe { &*(self as *const Self).cast::<u8>().add(72usize).cast() }
    }
    #[doc = "0x48 - Protocol Status Register \\[IIC Mode\\]"]
    #[inline(always)]
    pub const fn psr_iicmode(&self) -> &PSR_IICMODE {
        unsafe { &*(self as *const Self).cast::<u8>().add(72usize).cast() }
    }
    #[doc = "0x48 - Protocol Status Register \\[SSC Mode\\]"]
    #[inline(always)]
    pub const fn psr_sscmode(&self) -> &PSR_SSCMODE {
        unsafe { &*(self as *const Self).cast::<u8>().add(72usize).cast() }
    }
    #[doc = "0x48 - Protocol Status Register \\[ASC Mode\\]"]
    #[inline(always)]
    pub const fn psr_ascmode(&self) -> &PSR_ASCMODE {
        unsafe { &*(self as *const Self).cast::<u8>().add(72usize).cast() }
    }
    #[doc = "0x48 - Protocol Status Register"]
    #[inline(always)]
    pub const fn psr(&self) -> &PSR {
        unsafe { &*(self as *const Self).cast::<u8>().add(72usize).cast() }
    }
}
#[doc = "CCFG (r) register accessor: an alias for `Reg<CCFG_SPEC>`"]
pub type CCFG = crate::Reg<ccfg::CCFG_SPEC>;
#[doc = "Channel Configuration Register"]
pub mod ccfg;
#[doc = "KSCFG (rw) register accessor: an alias for `Reg<KSCFG_SPEC>`"]
pub type KSCFG = crate::Reg<kscfg::KSCFG_SPEC>;
#[doc = "Kernel State Configuration Register"]
pub mod kscfg;
#[doc = "FDR (rw) register accessor: an alias for `Reg<FDR_SPEC>`"]
pub type FDR = crate::Reg<fdr::FDR_SPEC>;
#[doc = "Fractional Divider Register"]
pub mod fdr;
#[doc = "BRG (rw) register accessor: an alias for `Reg<BRG_SPEC>`"]
pub type BRG = crate::Reg<brg::BRG_SPEC>;
#[doc = "Baud Rate Generator Register"]
pub mod brg;
#[doc = "INPR (rw) register accessor: an alias for `Reg<INPR_SPEC>`"]
pub type INPR = crate::Reg<inpr::INPR_SPEC>;
#[doc = "Interrupt Node Pointer Register"]
pub mod inpr;
#[doc = "DX0CR (rw) register accessor: an alias for `Reg<DX0CR_SPEC>`"]
pub type DX0CR = crate::Reg<dx0cr::DX0CR_SPEC>;
#[doc = "Input Control Register 0"]
pub mod dx0cr;
#[doc = "DX1CR (rw) register accessor: an alias for `Reg<DX1CR_SPEC>`"]
pub type DX1CR = crate::Reg<dx1cr::DX1CR_SPEC>;
#[doc = "Input Control Register 1"]
pub mod dx1cr;
#[doc = "DX2CR (rw) register accessor: an alias for `Reg<DX2CR_SPEC>`"]
pub type DX2CR = crate::Reg<dx2cr::DX2CR_SPEC>;
#[doc = "Input Control Register 2"]
pub mod dx2cr;
#[doc = "DX3CR (rw) register accessor: an alias for `Reg<DX3CR_SPEC>`"]
pub type DX3CR = crate::Reg<dx3cr::DX3CR_SPEC>;
#[doc = "Input Control Register 3"]
pub mod dx3cr;
#[doc = "DX4CR (rw) register accessor: an alias for `Reg<DX4CR_SPEC>`"]
pub type DX4CR = crate::Reg<dx4cr::DX4CR_SPEC>;
#[doc = "Input Control Register 4"]
pub mod dx4cr;
#[doc = "DX5CR (rw) register accessor: an alias for `Reg<DX5CR_SPEC>`"]
pub type DX5CR = crate::Reg<dx5cr::DX5CR_SPEC>;
#[doc = "Input Control Register 5"]
pub mod dx5cr;
#[doc = "SCTR (rw) register accessor: an alias for `Reg<SCTR_SPEC>`"]
pub type SCTR = crate::Reg<sctr::SCTR_SPEC>;
#[doc = "Shift Control Register"]
pub mod sctr;
#[doc = "TCSR (rw) register accessor: an alias for `Reg<TCSR_SPEC>`"]
pub type TCSR = crate::Reg<tcsr::TCSR_SPEC>;
#[doc = "Transmit Control/Status Register"]
pub mod tcsr;
#[doc = "PCR (rw) register accessor: an alias for `Reg<PCR_SPEC>`"]
pub type PCR = crate::Reg<pcr::PCR_SPEC>;
#[doc = "Protocol Control Register"]
pub mod pcr;
#[doc = "PCR_ASCMode (rw) register accessor: an alias for `Reg<PCR_ASCMODE_SPEC>`"]
pub type PCR_ASCMODE = crate::Reg<pcr_ascmode::PCR_ASCMODE_SPEC>;
#[doc = "Protocol Control Register \\[ASC Mode\\]"]
pub mod pcr_ascmode;
#[doc = "PCR_SSCMode (rw) register accessor: an alias for `Reg<PCR_SSCMODE_SPEC>`"]
pub type PCR_SSCMODE = crate::Reg<pcr_sscmode::PCR_SSCMODE_SPEC>;
#[doc = "Protocol Control Register \\[SSC Mode\\]"]
pub mod pcr_sscmode;
#[doc = "PCR_IICMode (rw) register accessor: an alias for `Reg<PCR_IICMODE_SPEC>`"]
pub type PCR_IICMODE = crate::Reg<pcr_iicmode::PCR_IICMODE_SPEC>;
#[doc = "Protocol Control Register \\[IIC Mode\\]"]
pub mod pcr_iicmode;
#[doc = "PCR_IISMode (rw) register accessor: an alias for `Reg<PCR_IISMODE_SPEC>`"]
pub type PCR_IISMODE = crate::Reg<pcr_iismode::PCR_IISMODE_SPEC>;
#[doc = "Protocol Control Register \\[IIS Mode\\]"]
pub mod pcr_iismode;
#[doc = "CCR (rw) register accessor: an alias for `Reg<CCR_SPEC>`"]
pub type CCR = crate::Reg<ccr::CCR_SPEC>;
#[doc = "Channel Control Register"]
pub mod ccr;
#[doc = "CMTR (rw) register accessor: an alias for `Reg<CMTR_SPEC>`"]
pub type CMTR = crate::Reg<cmtr::CMTR_SPEC>;
#[doc = "Capture Mode Timer Register"]
pub mod cmtr;
#[doc = "PSR (rw) register accessor: an alias for `Reg<PSR_SPEC>`"]
pub type PSR = crate::Reg<psr::PSR_SPEC>;
#[doc = "Protocol Status Register"]
pub mod psr;
#[doc = "PSR_ASCMode (rw) register accessor: an alias for `Reg<PSR_ASCMODE_SPEC>`"]
pub type PSR_ASCMODE = crate::Reg<psr_ascmode::PSR_ASCMODE_SPEC>;
#[doc = "Protocol Status Register \\[ASC Mode\\]"]
pub mod psr_ascmode;
#[doc = "PSR_SSCMode (rw) register accessor: an alias for `Reg<PSR_SSCMODE_SPEC>`"]
pub type PSR_SSCMODE = crate::Reg<psr_sscmode::PSR_SSCMODE_SPEC>;
#[doc = "Protocol Status Register \\[SSC Mode\\]"]
pub mod psr_sscmode;
#[doc = "PSR_IICMode (rw) register accessor: an alias for `Reg<PSR_IICMODE_SPEC>`"]
pub type PSR_IICMODE = crate::Reg<psr_iicmode::PSR_IICMODE_SPEC>;
#[doc = "Protocol Status Register \\[IIC Mode\\]"]
pub mod psr_iicmode;
#[doc = "PSR_IISMode (rw) register accessor: an alias for `Reg<PSR_IISMODE_SPEC>`"]
pub type PSR_IISMODE = crate::Reg<psr_iismode::PSR_IISMODE_SPEC>;
#[doc = "Protocol Status Register \\[IIS Mode\\]"]
pub mod psr_iismode;
#[doc = "PSCR (w) register accessor: an alias for `Reg<PSCR_SPEC>`"]
pub type PSCR = crate::Reg<pscr::PSCR_SPEC>;
#[doc = "Protocol Status Clear Register"]
pub mod pscr;
#[doc = "RBUFSR (r) register accessor: an alias for `Reg<RBUFSR_SPEC>`"]
pub type RBUFSR = crate::Reg<rbufsr::RBUFSR_SPEC>;
#[doc = "Receiver Buffer Status Register"]
pub mod rbufsr;
#[doc = "RBUF (r) register accessor: an alias for `Reg<RBUF_SPEC>`"]
pub type RBUF = crate::Reg<rbuf::RBUF_SPEC>;
#[doc = "Receiver Buffer Register"]
pub mod rbuf;
#[doc = "RBUFD (r) register accessor: an alias for `Reg<RBUFD_SPEC>`"]
pub type RBUFD = crate::Reg<rbufd::RBUFD_SPEC>;
#[doc = "Receiver Buffer Register for Debugger"]
pub mod rbufd;
#[doc = "RBUF0 (r) register accessor: an alias for `Reg<RBUF0_SPEC>`"]
pub type RBUF0 = crate::Reg<rbuf0::RBUF0_SPEC>;
#[doc = "Receiver Buffer Register 0"]
pub mod rbuf0;
#[doc = "RBUF1 (r) register accessor: an alias for `Reg<RBUF1_SPEC>`"]
pub type RBUF1 = crate::Reg<rbuf1::RBUF1_SPEC>;
#[doc = "Receiver Buffer Register 1"]
pub mod rbuf1;
#[doc = "RBUF01SR (r) register accessor: an alias for `Reg<RBUF01SR_SPEC>`"]
pub type RBUF01SR = crate::Reg<rbuf01sr::RBUF01SR_SPEC>;
#[doc = "Receiver Buffer 01 Status Register"]
pub mod rbuf01sr;
#[doc = "FMR (w) register accessor: an alias for `Reg<FMR_SPEC>`"]
pub type FMR = crate::Reg<fmr::FMR_SPEC>;
#[doc = "Flag Modification Register"]
pub mod fmr;
#[doc = "TBUF (rw) register accessor: an alias for `Reg<TBUF_SPEC>`"]
pub type TBUF = crate::Reg<tbuf::TBUF_SPEC>;
#[doc = "Transmit Buffer"]
pub mod tbuf;
#[doc = "BYP (rw) register accessor: an alias for `Reg<BYP_SPEC>`"]
pub type BYP = crate::Reg<byp::BYP_SPEC>;
#[doc = "Bypass Data Register"]
pub mod byp;
#[doc = "BYPCR (rw) register accessor: an alias for `Reg<BYPCR_SPEC>`"]
pub type BYPCR = crate::Reg<bypcr::BYPCR_SPEC>;
#[doc = "Bypass Control Register"]
pub mod bypcr;
#[doc = "TBCTR (rw) register accessor: an alias for `Reg<TBCTR_SPEC>`"]
pub type TBCTR = crate::Reg<tbctr::TBCTR_SPEC>;
#[doc = "Transmitter Buffer Control Register"]
pub mod tbctr;
#[doc = "RBCTR (rw) register accessor: an alias for `Reg<RBCTR_SPEC>`"]
pub type RBCTR = crate::Reg<rbctr::RBCTR_SPEC>;
#[doc = "Receiver Buffer Control Register"]
pub mod rbctr;
#[doc = "TRBPTR (r) register accessor: an alias for `Reg<TRBPTR_SPEC>`"]
pub type TRBPTR = crate::Reg<trbptr::TRBPTR_SPEC>;
#[doc = "Transmit/Receive Buffer Pointer Register"]
pub mod trbptr;
#[doc = "TRBSR (rw) register accessor: an alias for `Reg<TRBSR_SPEC>`"]
pub type TRBSR = crate::Reg<trbsr::TRBSR_SPEC>;
#[doc = "Transmit/Receive Buffer Status Register"]
pub mod trbsr;
#[doc = "TRBSCR (w) register accessor: an alias for `Reg<TRBSCR_SPEC>`"]
pub type TRBSCR = crate::Reg<trbscr::TRBSCR_SPEC>;
#[doc = "Transmit/Receive Buffer Status Clear Register"]
pub mod trbscr;
#[doc = "OUTR (r) register accessor: an alias for `Reg<OUTR_SPEC>`"]
pub type OUTR = crate::Reg<outr::OUTR_SPEC>;
#[doc = "Receiver Buffer Output Register"]
pub mod outr;
#[doc = "OUTDR (r) register accessor: an alias for `Reg<OUTDR_SPEC>`"]
pub type OUTDR = crate::Reg<outdr::OUTDR_SPEC>;
#[doc = "Receiver Buffer Output Register L for Debugger"]
pub mod outdr;
#[doc = "IN (w) register accessor: an alias for `Reg<IN_SPEC>`"]
pub type IN = crate::Reg<in_::IN_SPEC>;
#[doc = "Transmit FIFO Buffer"]
pub mod in_;
