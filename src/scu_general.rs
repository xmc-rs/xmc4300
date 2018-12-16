#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - SCU Module ID Register"]
    pub id: ID,
    #[doc = "0x04 - Chip ID Register"]
    pub idchip: IDCHIP,
    #[doc = "0x08 - Manufactory ID Register"]
    pub idmanuf: IDMANUF,
    _reserved0: [u8; 4usize],
    #[doc = "0x10 - Startup Configuration Register"]
    pub stcon: STCON,
    _reserved1: [u8; 24usize],
    #[doc = "0x2c - General Purpose Register 0"]
    pub gpr0: GPR0,
    #[doc = "0x30 - General Purpose Register 1"]
    pub gpr1: GPR1,
    _reserved2: [u8; 24usize],
    #[doc = "0x4c - CCU Control Register"]
    pub ccucon: CCUCON,
    _reserved3: [u8; 60usize],
    #[doc = "0x8c - Die Temperature Sensor Control Register"]
    pub dtscon: DTSCON,
    #[doc = "0x90 - Die Temperature Sensor Status Register"]
    pub dtsstat: DTSSTAT,
    _reserved4: [u8; 8usize],
    #[doc = "0x9c - SD-MMC Delay Control Register"]
    pub sdmmcdel: SDMMCDEL,
    #[doc = "0xa0 - Out of Range Comparator Enable Register 0"]
    pub g0orcen: G0ORCEN,
    #[doc = "0xa4 - Out of Range Comparator Enable Register 1"]
    pub g1orcen: G1ORCEN,
    _reserved5: [u8; 28usize],
    #[doc = "0xc4 - Mirror Write Status Register"]
    pub mirrsts: MIRRSTS,
    #[doc = "0xc8 - Retention Memory Access Control Register"]
    pub rmacr: RMACR,
    #[doc = "0xcc - Retention Memory Access Data Register"]
    pub rmdata: RMDATA,
}
#[doc = "SCU Module ID Register"]
pub struct ID {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SCU Module ID Register"]
pub mod id;
#[doc = "Chip ID Register"]
pub struct IDCHIP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Chip ID Register"]
pub mod idchip;
#[doc = "Manufactory ID Register"]
pub struct IDMANUF {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Manufactory ID Register"]
pub mod idmanuf;
#[doc = "Startup Configuration Register"]
pub struct STCON {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Startup Configuration Register"]
pub mod stcon;
#[doc = "General Purpose Register 0"]
pub struct GPR0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "General Purpose Register 0"]
pub mod gpr0;
#[doc = "General Purpose Register 1"]
pub struct GPR1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "General Purpose Register 1"]
pub mod gpr1;
#[doc = "CCU Control Register"]
pub struct CCUCON {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CCU Control Register"]
pub mod ccucon;
#[doc = "Die Temperature Sensor Control Register"]
pub struct DTSCON {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Die Temperature Sensor Control Register"]
pub mod dtscon;
#[doc = "Die Temperature Sensor Status Register"]
pub struct DTSSTAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Die Temperature Sensor Status Register"]
pub mod dtsstat;
#[doc = "SD-MMC Delay Control Register"]
pub struct SDMMCDEL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SD-MMC Delay Control Register"]
pub mod sdmmcdel;
#[doc = "Out of Range Comparator Enable Register 0"]
pub struct G0ORCEN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Out of Range Comparator Enable Register 0"]
pub mod g0orcen;
#[doc = "Out of Range Comparator Enable Register 1"]
pub struct G1ORCEN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Out of Range Comparator Enable Register 1"]
pub mod g1orcen;
#[doc = "Mirror Write Status Register"]
pub struct MIRRSTS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Mirror Write Status Register"]
pub mod mirrsts;
#[doc = "Retention Memory Access Control Register"]
pub struct RMACR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Retention Memory Access Control Register"]
pub mod rmacr;
#[doc = "Retention Memory Access Data Register"]
pub struct RMDATA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Retention Memory Access Data Register"]
pub mod rmdata;
