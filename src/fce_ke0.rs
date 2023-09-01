#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Input Register"]
    pub ir: IR,
    #[doc = "0x04 - CRC Result Register"]
    pub res: RES,
    #[doc = "0x08 - CRC Configuration Register"]
    pub cfg: CFG,
    #[doc = "0x0c - CRC Status Register"]
    pub sts: STS,
    #[doc = "0x10 - CRC Length Register"]
    pub length: LENGTH,
    #[doc = "0x14 - CRC Check Register"]
    pub check: CHECK,
    #[doc = "0x18 - CRC Register"]
    pub crc: CRC,
    #[doc = "0x1c - CRC Test Register"]
    pub ctr: CTR,
}
#[doc = "IR (rw) register accessor: Input Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ir::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ir::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ir`]
module"]
pub type IR = crate::Reg<ir::IR_SPEC>;
#[doc = "Input Register"]
pub mod ir;
#[doc = "RES (r) register accessor: CRC Result Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`res::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`res`]
module"]
pub type RES = crate::Reg<res::RES_SPEC>;
#[doc = "CRC Result Register"]
pub mod res;
#[doc = "CFG (rw) register accessor: CRC Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub type CFG = crate::Reg<cfg::CFG_SPEC>;
#[doc = "CRC Configuration Register"]
pub mod cfg;
#[doc = "STS (rw) register accessor: CRC Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sts::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sts::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`sts`]
module"]
pub type STS = crate::Reg<sts::STS_SPEC>;
#[doc = "CRC Status Register"]
pub mod sts;
#[doc = "LENGTH (rw) register accessor: CRC Length Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`length::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`length::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`length`]
module"]
pub type LENGTH = crate::Reg<length::LENGTH_SPEC>;
#[doc = "CRC Length Register"]
pub mod length;
#[doc = "CHECK (rw) register accessor: CRC Check Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`check::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`check::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`check`]
module"]
pub type CHECK = crate::Reg<check::CHECK_SPEC>;
#[doc = "CRC Check Register"]
pub mod check;
#[doc = "CRC (rw) register accessor: CRC Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`crc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`crc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`crc`]
module"]
pub type CRC = crate::Reg<crc::CRC_SPEC>;
#[doc = "CRC Register"]
pub mod crc;
#[doc = "CTR (rw) register accessor: CRC Test Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ctr`]
module"]
pub type CTR = crate::Reg<ctr::CTR_SPEC>;
#[doc = "CRC Test Register"]
pub mod ctr;
