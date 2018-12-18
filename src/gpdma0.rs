#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Raw IntTfr Status"]
    pub rawtfr: RAWTFR,
    _reserved0: [u8; 4usize],
    #[doc = "0x08 - Raw IntBlock Status"]
    pub rawblock: RAWBLOCK,
    _reserved1: [u8; 4usize],
    #[doc = "0x10 - Raw IntSrcTran Status"]
    pub rawsrctran: RAWSRCTRAN,
    _reserved2: [u8; 4usize],
    #[doc = "0x18 - Raw IntBlock Status"]
    pub rawdsttran: RAWDSTTRAN,
    _reserved3: [u8; 4usize],
    #[doc = "0x20 - Raw IntErr Status"]
    pub rawerr: RAWERR,
    _reserved4: [u8; 4usize],
    #[doc = "0x28 - IntTfr Status"]
    pub statustfr: STATUSTFR,
    _reserved5: [u8; 4usize],
    #[doc = "0x30 - IntBlock Status"]
    pub statusblock: STATUSBLOCK,
    _reserved6: [u8; 4usize],
    #[doc = "0x38 - IntSrcTran Status"]
    pub statussrctran: STATUSSRCTRAN,
    _reserved7: [u8; 4usize],
    #[doc = "0x40 - IntBlock Status"]
    pub statusdsttran: STATUSDSTTRAN,
    _reserved8: [u8; 4usize],
    #[doc = "0x48 - IntErr Status"]
    pub statuserr: STATUSERR,
    _reserved9: [u8; 4usize],
    #[doc = "0x50 - Mask for Raw IntTfr Status"]
    pub masktfr: MASKTFR,
    _reserved10: [u8; 4usize],
    #[doc = "0x58 - Mask for Raw IntBlock Status"]
    pub maskblock: MASKBLOCK,
    _reserved11: [u8; 4usize],
    #[doc = "0x60 - Mask for Raw IntSrcTran Status"]
    pub masksrctran: MASKSRCTRAN,
    _reserved12: [u8; 4usize],
    #[doc = "0x68 - Mask for Raw IntBlock Status"]
    pub maskdsttran: MASKDSTTRAN,
    _reserved13: [u8; 4usize],
    #[doc = "0x70 - Mask for Raw IntErr Status"]
    pub maskerr: MASKERR,
    _reserved14: [u8; 4usize],
    #[doc = "0x78 - IntTfr Status"]
    pub cleartfr: CLEARTFR,
    _reserved15: [u8; 4usize],
    #[doc = "0x80 - IntBlock Status"]
    pub clearblock: CLEARBLOCK,
    _reserved16: [u8; 4usize],
    #[doc = "0x88 - IntSrcTran Status"]
    pub clearsrctran: CLEARSRCTRAN,
    _reserved17: [u8; 4usize],
    #[doc = "0x90 - IntBlock Status"]
    pub cleardsttran: CLEARDSTTRAN,
    _reserved18: [u8; 4usize],
    #[doc = "0x98 - IntErr Status"]
    pub clearerr: CLEARERR,
    _reserved19: [u8; 4usize],
    #[doc = "0xa0 - Combined Interrupt Status Register"]
    pub statusint: STATUSINT,
    _reserved20: [u8; 4usize],
    #[doc = "0xa8 - Source Software Transaction Request Register"]
    pub reqsrcreg: REQSRCREG,
    _reserved21: [u8; 4usize],
    #[doc = "0xb0 - Destination Software Transaction Request Register"]
    pub reqdstreg: REQDSTREG,
    _reserved22: [u8; 4usize],
    #[doc = "0xb8 - Single Source Transaction Request Register"]
    pub sglreqsrcreg: SGLREQSRCREG,
    _reserved23: [u8; 4usize],
    #[doc = "0xc0 - Single Destination Transaction Request Register"]
    pub sglreqdstreg: SGLREQDSTREG,
    _reserved24: [u8; 4usize],
    #[doc = "0xc8 - Last Source Transaction Request Register"]
    pub lstsrcreg: LSTSRCREG,
    _reserved25: [u8; 4usize],
    #[doc = "0xd0 - Last Destination Transaction Request Register"]
    pub lstdstreg: LSTDSTREG,
    _reserved26: [u8; 4usize],
    #[doc = "0xd8 - GPDMA Configuration Register"]
    pub dmacfgreg: DMACFGREG,
    _reserved27: [u8; 4usize],
    #[doc = "0xe0 - GPDMA Channel Enable Register"]
    pub chenreg: CHENREG,
    _reserved28: [u8; 4usize],
    #[doc = "0xe8 - GPDMA0 ID Register"]
    pub id: ID,
    _reserved29: [u8; 76usize],
    #[doc = "0x138 - GPDMA Component Type"]
    pub type_: TYPE,
    #[doc = "0x13c - DMA Component Version"]
    pub version: VERSION,
}
#[doc = "Raw IntTfr Status"]
pub struct RAWTFR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Raw IntTfr Status"]
pub mod rawtfr;
#[doc = "Raw IntBlock Status"]
pub struct RAWBLOCK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Raw IntBlock Status"]
pub mod rawblock;
#[doc = "Raw IntSrcTran Status"]
pub struct RAWSRCTRAN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Raw IntSrcTran Status"]
pub mod rawsrctran;
#[doc = "Raw IntBlock Status"]
pub struct RAWDSTTRAN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Raw IntBlock Status"]
pub mod rawdsttran;
#[doc = "Raw IntErr Status"]
pub struct RAWERR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Raw IntErr Status"]
pub mod rawerr;
#[doc = "IntTfr Status"]
pub struct STATUSTFR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "IntTfr Status"]
pub mod statustfr;
#[doc = "IntBlock Status"]
pub struct STATUSBLOCK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "IntBlock Status"]
pub mod statusblock;
#[doc = "IntSrcTran Status"]
pub struct STATUSSRCTRAN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "IntSrcTran Status"]
pub mod statussrctran;
#[doc = "IntBlock Status"]
pub struct STATUSDSTTRAN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "IntBlock Status"]
pub mod statusdsttran;
#[doc = "IntErr Status"]
pub struct STATUSERR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "IntErr Status"]
pub mod statuserr;
#[doc = "Mask for Raw IntTfr Status"]
pub struct MASKTFR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Mask for Raw IntTfr Status"]
pub mod masktfr;
#[doc = "Mask for Raw IntBlock Status"]
pub struct MASKBLOCK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Mask for Raw IntBlock Status"]
pub mod maskblock;
#[doc = "Mask for Raw IntSrcTran Status"]
pub struct MASKSRCTRAN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Mask for Raw IntSrcTran Status"]
pub mod masksrctran;
#[doc = "Mask for Raw IntBlock Status"]
pub struct MASKDSTTRAN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Mask for Raw IntBlock Status"]
pub mod maskdsttran;
#[doc = "Mask for Raw IntErr Status"]
pub struct MASKERR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Mask for Raw IntErr Status"]
pub mod maskerr;
#[doc = "IntTfr Status"]
pub struct CLEARTFR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "IntTfr Status"]
pub mod cleartfr;
#[doc = "IntBlock Status"]
pub struct CLEARBLOCK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "IntBlock Status"]
pub mod clearblock;
#[doc = "IntSrcTran Status"]
pub struct CLEARSRCTRAN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "IntSrcTran Status"]
pub mod clearsrctran;
#[doc = "IntBlock Status"]
pub struct CLEARDSTTRAN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "IntBlock Status"]
pub mod cleardsttran;
#[doc = "IntErr Status"]
pub struct CLEARERR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "IntErr Status"]
pub mod clearerr;
#[doc = "Combined Interrupt Status Register"]
pub struct STATUSINT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Combined Interrupt Status Register"]
pub mod statusint;
#[doc = "Source Software Transaction Request Register"]
pub struct REQSRCREG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Source Software Transaction Request Register"]
pub mod reqsrcreg;
#[doc = "Destination Software Transaction Request Register"]
pub struct REQDSTREG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Destination Software Transaction Request Register"]
pub mod reqdstreg;
#[doc = "Single Source Transaction Request Register"]
pub struct SGLREQSRCREG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Single Source Transaction Request Register"]
pub mod sglreqsrcreg;
#[doc = "Single Destination Transaction Request Register"]
pub struct SGLREQDSTREG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Single Destination Transaction Request Register"]
pub mod sglreqdstreg;
#[doc = "Last Source Transaction Request Register"]
pub struct LSTSRCREG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Last Source Transaction Request Register"]
pub mod lstsrcreg;
#[doc = "Last Destination Transaction Request Register"]
pub struct LSTDSTREG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Last Destination Transaction Request Register"]
pub mod lstdstreg;
#[doc = "GPDMA Configuration Register"]
pub struct DMACFGREG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPDMA Configuration Register"]
pub mod dmacfgreg;
#[doc = "GPDMA Channel Enable Register"]
pub struct CHENREG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPDMA Channel Enable Register"]
pub mod chenreg;
#[doc = "GPDMA0 ID Register"]
pub struct ID {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPDMA0 ID Register"]
pub mod id;
#[doc = "GPDMA Component Type"]
pub struct TYPE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPDMA Component Type"]
pub mod type_;
#[doc = "DMA Component Version"]
pub struct VERSION {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DMA Component Version"]
pub mod version;
