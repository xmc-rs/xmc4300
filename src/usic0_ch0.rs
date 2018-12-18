#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 4usize],
    #[doc = "0x04 - Channel Configuration Register"]
    pub ccfg: CCFG,
    _reserved1: [u8; 4usize],
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
    #[doc = "0x3c - Protocol Control Register"]
    pub pcr: PCR,
    #[doc = "0x40 - Channel Control Register"]
    pub ccr: CCR,
    #[doc = "0x44 - Capture Mode Timer Register"]
    pub cmtr: CMTR,
    #[doc = "0x48 - Protocol Status Register"]
    pub psr: PSR,
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
    _reserved2: [u8; 20usize],
    #[doc = "0x80 - Transmit Buffer"]
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
    _reserved3: [u8; 92usize],
    #[doc = "0x180 - Transmit FIFO Buffer"]
    pub in_: [IN; 32],
}
#[doc = "Channel Configuration Register"]
pub struct CCFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel Configuration Register"]
pub mod ccfg;
#[doc = "Kernel State Configuration Register"]
pub struct KSCFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Kernel State Configuration Register"]
pub mod kscfg;
#[doc = "Fractional Divider Register"]
pub struct FDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Fractional Divider Register"]
pub mod fdr;
#[doc = "Baud Rate Generator Register"]
pub struct BRG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Baud Rate Generator Register"]
pub mod brg;
#[doc = "Interrupt Node Pointer Register"]
pub struct INPR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Node Pointer Register"]
pub mod inpr;
#[doc = "Input Control Register 0"]
pub struct DX0CR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Input Control Register 0"]
pub mod dx0cr;
#[doc = "Input Control Register 1"]
pub struct DX1CR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Input Control Register 1"]
pub mod dx1cr;
#[doc = "Input Control Register 2"]
pub struct DX2CR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Input Control Register 2"]
pub mod dx2cr;
#[doc = "Input Control Register 3"]
pub struct DX3CR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Input Control Register 3"]
pub mod dx3cr;
#[doc = "Input Control Register 4"]
pub struct DX4CR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Input Control Register 4"]
pub mod dx4cr;
#[doc = "Input Control Register 5"]
pub struct DX5CR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Input Control Register 5"]
pub mod dx5cr;
#[doc = "Shift Control Register"]
pub struct SCTR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Shift Control Register"]
pub mod sctr;
#[doc = "Transmit Control/Status Register"]
pub struct TCSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Transmit Control/Status Register"]
pub mod tcsr;
#[doc = "Protocol Control Register"]
pub struct PCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Protocol Control Register"]
pub mod pcr;
#[doc = "Protocol Control Register \\[ASC Mode\\]"]
pub struct PCR_ASCMODE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Protocol Control Register \\[ASC Mode\\]"]
pub mod pcr_ascmode;
#[doc = "Protocol Control Register \\[SSC Mode\\]"]
pub struct PCR_SSCMODE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Protocol Control Register \\[SSC Mode\\]"]
pub mod pcr_sscmode;
#[doc = "Protocol Control Register \\[IIC Mode\\]"]
pub struct PCR_IICMODE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Protocol Control Register \\[IIC Mode\\]"]
pub mod pcr_iicmode;
#[doc = "Protocol Control Register \\[IIS Mode\\]"]
pub struct PCR_IISMODE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Protocol Control Register \\[IIS Mode\\]"]
pub mod pcr_iismode;
#[doc = "Channel Control Register"]
pub struct CCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel Control Register"]
pub mod ccr;
#[doc = "Capture Mode Timer Register"]
pub struct CMTR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Capture Mode Timer Register"]
pub mod cmtr;
#[doc = "Protocol Status Register"]
pub struct PSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Protocol Status Register"]
pub mod psr;
#[doc = "Protocol Status Register \\[ASC Mode\\]"]
pub struct PSR_ASCMODE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Protocol Status Register \\[ASC Mode\\]"]
pub mod psr_ascmode;
#[doc = "Protocol Status Register \\[SSC Mode\\]"]
pub struct PSR_SSCMODE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Protocol Status Register \\[SSC Mode\\]"]
pub mod psr_sscmode;
#[doc = "Protocol Status Register \\[IIC Mode\\]"]
pub struct PSR_IICMODE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Protocol Status Register \\[IIC Mode\\]"]
pub mod psr_iicmode;
#[doc = "Protocol Status Register \\[IIS Mode\\]"]
pub struct PSR_IISMODE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Protocol Status Register \\[IIS Mode\\]"]
pub mod psr_iismode;
#[doc = "Protocol Status Clear Register"]
pub struct PSCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Protocol Status Clear Register"]
pub mod pscr;
#[doc = "Receiver Buffer Status Register"]
pub struct RBUFSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Receiver Buffer Status Register"]
pub mod rbufsr;
#[doc = "Receiver Buffer Register"]
pub struct RBUF {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Receiver Buffer Register"]
pub mod rbuf;
#[doc = "Receiver Buffer Register for Debugger"]
pub struct RBUFD {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Receiver Buffer Register for Debugger"]
pub mod rbufd;
#[doc = "Receiver Buffer Register 0"]
pub struct RBUF0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Receiver Buffer Register 0"]
pub mod rbuf0;
#[doc = "Receiver Buffer Register 1"]
pub struct RBUF1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Receiver Buffer Register 1"]
pub mod rbuf1;
#[doc = "Receiver Buffer 01 Status Register"]
pub struct RBUF01SR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Receiver Buffer 01 Status Register"]
pub mod rbuf01sr;
#[doc = "Flag Modification Register"]
pub struct FMR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Flag Modification Register"]
pub mod fmr;
#[doc = "Transmit Buffer"]
pub struct TBUF {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Transmit Buffer"]
pub mod tbuf;
#[doc = "Bypass Data Register"]
pub struct BYP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Bypass Data Register"]
pub mod byp;
#[doc = "Bypass Control Register"]
pub struct BYPCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Bypass Control Register"]
pub mod bypcr;
#[doc = "Transmitter Buffer Control Register"]
pub struct TBCTR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Transmitter Buffer Control Register"]
pub mod tbctr;
#[doc = "Receiver Buffer Control Register"]
pub struct RBCTR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Receiver Buffer Control Register"]
pub mod rbctr;
#[doc = "Transmit/Receive Buffer Pointer Register"]
pub struct TRBPTR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Transmit/Receive Buffer Pointer Register"]
pub mod trbptr;
#[doc = "Transmit/Receive Buffer Status Register"]
pub struct TRBSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Transmit/Receive Buffer Status Register"]
pub mod trbsr;
#[doc = "Transmit/Receive Buffer Status Clear Register"]
pub struct TRBSCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Transmit/Receive Buffer Status Clear Register"]
pub mod trbscr;
#[doc = "Receiver Buffer Output Register"]
pub struct OUTR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Receiver Buffer Output Register"]
pub mod outr;
#[doc = "Receiver Buffer Output Register L for Debugger"]
pub struct OUTDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Receiver Buffer Output Register L for Debugger"]
pub mod outdr;
#[doc = "Transmit FIFO Buffer"]
pub struct IN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Transmit FIFO Buffer"]
pub mod in_;
