#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    id: ID,
    idchip: IDCHIP,
    idmanuf: IDMANUF,
    _reserved3: [u8; 0x04],
    stcon: STCON,
    _reserved4: [u8; 0x18],
    gpr0: GPR0,
    gpr1: GPR1,
    _reserved6: [u8; 0x18],
    ccucon: CCUCON,
    _reserved7: [u8; 0x3c],
    dtscon: DTSCON,
    dtsstat: DTSSTAT,
    _reserved9: [u8; 0x08],
    sdmmcdel: SDMMCDEL,
    g0orcen: G0ORCEN,
    g1orcen: G1ORCEN,
    _reserved12: [u8; 0x1c],
    mirrsts: MIRRSTS,
    rmacr: RMACR,
    rmdata: RMDATA,
}
impl RegisterBlock {
    #[doc = "0x00 - SCU Module ID Register"]
    #[inline(always)]
    pub const fn id(&self) -> &ID {
        &self.id
    }
    #[doc = "0x04 - Chip ID Register"]
    #[inline(always)]
    pub const fn idchip(&self) -> &IDCHIP {
        &self.idchip
    }
    #[doc = "0x08 - Manufactory ID Register"]
    #[inline(always)]
    pub const fn idmanuf(&self) -> &IDMANUF {
        &self.idmanuf
    }
    #[doc = "0x10 - Startup Configuration Register"]
    #[inline(always)]
    pub const fn stcon(&self) -> &STCON {
        &self.stcon
    }
    #[doc = "0x2c - General Purpose Register 0"]
    #[inline(always)]
    pub const fn gpr0(&self) -> &GPR0 {
        &self.gpr0
    }
    #[doc = "0x30 - General Purpose Register 1"]
    #[inline(always)]
    pub const fn gpr1(&self) -> &GPR1 {
        &self.gpr1
    }
    #[doc = "0x4c - CCU Control Register"]
    #[inline(always)]
    pub const fn ccucon(&self) -> &CCUCON {
        &self.ccucon
    }
    #[doc = "0x8c - Die Temperature Sensor Control Register"]
    #[inline(always)]
    pub const fn dtscon(&self) -> &DTSCON {
        &self.dtscon
    }
    #[doc = "0x90 - Die Temperature Sensor Status Register"]
    #[inline(always)]
    pub const fn dtsstat(&self) -> &DTSSTAT {
        &self.dtsstat
    }
    #[doc = "0x9c - SD-MMC Delay Control Register"]
    #[inline(always)]
    pub const fn sdmmcdel(&self) -> &SDMMCDEL {
        &self.sdmmcdel
    }
    #[doc = "0xa0 - Out of Range Comparator Enable Register 0"]
    #[inline(always)]
    pub const fn g0orcen(&self) -> &G0ORCEN {
        &self.g0orcen
    }
    #[doc = "0xa4 - Out of Range Comparator Enable Register 1"]
    #[inline(always)]
    pub const fn g1orcen(&self) -> &G1ORCEN {
        &self.g1orcen
    }
    #[doc = "0xc4 - Mirror Write Status Register"]
    #[inline(always)]
    pub const fn mirrsts(&self) -> &MIRRSTS {
        &self.mirrsts
    }
    #[doc = "0xc8 - Retention Memory Access Control Register"]
    #[inline(always)]
    pub const fn rmacr(&self) -> &RMACR {
        &self.rmacr
    }
    #[doc = "0xcc - Retention Memory Access Data Register"]
    #[inline(always)]
    pub const fn rmdata(&self) -> &RMDATA {
        &self.rmdata
    }
}
#[doc = "ID (r) register accessor: SCU Module ID Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`id::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@id`]
module"]
pub type ID = crate::Reg<id::ID_SPEC>;
#[doc = "SCU Module ID Register"]
pub mod id;
#[doc = "IDCHIP (r) register accessor: Chip ID Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`idchip::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@idchip`]
module"]
pub type IDCHIP = crate::Reg<idchip::IDCHIP_SPEC>;
#[doc = "Chip ID Register"]
pub mod idchip;
#[doc = "IDMANUF (r) register accessor: Manufactory ID Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`idmanuf::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@idmanuf`]
module"]
pub type IDMANUF = crate::Reg<idmanuf::IDMANUF_SPEC>;
#[doc = "Manufactory ID Register"]
pub mod idmanuf;
#[doc = "STCON (rw) register accessor: Startup Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stcon::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stcon::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stcon`]
module"]
pub type STCON = crate::Reg<stcon::STCON_SPEC>;
#[doc = "Startup Configuration Register"]
pub mod stcon;
#[doc = "GPR0 (rw) register accessor: General Purpose Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpr0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpr0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpr0`]
module"]
pub type GPR0 = crate::Reg<gpr0::GPR0_SPEC>;
#[doc = "General Purpose Register 0"]
pub mod gpr0;
#[doc = "GPR1 (rw) register accessor: General Purpose Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpr1`]
module"]
pub type GPR1 = crate::Reg<gpr1::GPR1_SPEC>;
#[doc = "General Purpose Register 1"]
pub mod gpr1;
#[doc = "CCUCON (rw) register accessor: CCU Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccucon::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccucon::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccucon`]
module"]
pub type CCUCON = crate::Reg<ccucon::CCUCON_SPEC>;
#[doc = "CCU Control Register"]
pub mod ccucon;
#[doc = "DTSCON (rw) register accessor: Die Temperature Sensor Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dtscon::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dtscon::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dtscon`]
module"]
pub type DTSCON = crate::Reg<dtscon::DTSCON_SPEC>;
#[doc = "Die Temperature Sensor Control Register"]
pub mod dtscon;
#[doc = "DTSSTAT (r) register accessor: Die Temperature Sensor Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dtsstat::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dtsstat`]
module"]
pub type DTSSTAT = crate::Reg<dtsstat::DTSSTAT_SPEC>;
#[doc = "Die Temperature Sensor Status Register"]
pub mod dtsstat;
#[doc = "SDMMCDEL (rw) register accessor: SD-MMC Delay Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdmmcdel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdmmcdel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdmmcdel`]
module"]
pub type SDMMCDEL = crate::Reg<sdmmcdel::SDMMCDEL_SPEC>;
#[doc = "SD-MMC Delay Control Register"]
pub mod sdmmcdel;
#[doc = "G0ORCEN (rw) register accessor: Out of Range Comparator Enable Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`g0orcen::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`g0orcen::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@g0orcen`]
module"]
pub type G0ORCEN = crate::Reg<g0orcen::G0ORCEN_SPEC>;
#[doc = "Out of Range Comparator Enable Register 0"]
pub mod g0orcen;
#[doc = "G1ORCEN (rw) register accessor: Out of Range Comparator Enable Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`g1orcen::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`g1orcen::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@g1orcen`]
module"]
pub type G1ORCEN = crate::Reg<g1orcen::G1ORCEN_SPEC>;
#[doc = "Out of Range Comparator Enable Register 1"]
pub mod g1orcen;
#[doc = "MIRRSTS (r) register accessor: Mirror Write Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mirrsts::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mirrsts`]
module"]
pub type MIRRSTS = crate::Reg<mirrsts::MIRRSTS_SPEC>;
#[doc = "Mirror Write Status Register"]
pub mod mirrsts;
#[doc = "RMACR (rw) register accessor: Retention Memory Access Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rmacr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rmacr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rmacr`]
module"]
pub type RMACR = crate::Reg<rmacr::RMACR_SPEC>;
#[doc = "Retention Memory Access Control Register"]
pub mod rmacr;
#[doc = "RMDATA (rw) register accessor: Retention Memory Access Data Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rmdata::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rmdata::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rmdata`]
module"]
pub type RMDATA = crate::Reg<rmdata::RMDATA_SPEC>;
#[doc = "Retention Memory Access Data Register"]
pub mod rmdata;
