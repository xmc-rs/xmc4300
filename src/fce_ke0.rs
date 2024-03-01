#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ir: Ir,
    res: Res,
    cfg: Cfg,
    sts: Sts,
    length: Length,
    check: Check,
    crc: Crc,
    ctr: Ctr,
}
impl RegisterBlock {
    #[doc = "0x00 - Input Register"]
    #[inline(always)]
    pub const fn ir(&self) -> &Ir {
        &self.ir
    }
    #[doc = "0x04 - CRC Result Register"]
    #[inline(always)]
    pub const fn res(&self) -> &Res {
        &self.res
    }
    #[doc = "0x08 - CRC Configuration Register"]
    #[inline(always)]
    pub const fn cfg(&self) -> &Cfg {
        &self.cfg
    }
    #[doc = "0x0c - CRC Status Register"]
    #[inline(always)]
    pub const fn sts(&self) -> &Sts {
        &self.sts
    }
    #[doc = "0x10 - CRC Length Register"]
    #[inline(always)]
    pub const fn length(&self) -> &Length {
        &self.length
    }
    #[doc = "0x14 - CRC Check Register"]
    #[inline(always)]
    pub const fn check(&self) -> &Check {
        &self.check
    }
    #[doc = "0x18 - CRC Register"]
    #[inline(always)]
    pub const fn crc(&self) -> &Crc {
        &self.crc
    }
    #[doc = "0x1c - CRC Test Register"]
    #[inline(always)]
    pub const fn ctr(&self) -> &Ctr {
        &self.ctr
    }
}
#[doc = "IR (rw) register accessor: Input Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ir::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ir::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ir`]
module"]
#[doc(alias = "IR")]
pub type Ir = crate::Reg<ir::IrSpec>;
#[doc = "Input Register"]
pub mod ir;
#[doc = "RES (r) register accessor: CRC Result Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`res::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@res`]
module"]
#[doc(alias = "RES")]
pub type Res = crate::Reg<res::ResSpec>;
#[doc = "CRC Result Register"]
pub mod res;
#[doc = "CFG (rw) register accessor: CRC Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
#[doc(alias = "CFG")]
pub type Cfg = crate::Reg<cfg::CfgSpec>;
#[doc = "CRC Configuration Register"]
pub mod cfg;
#[doc = "STS (rw) register accessor: CRC Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sts::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sts::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sts`]
module"]
#[doc(alias = "STS")]
pub type Sts = crate::Reg<sts::StsSpec>;
#[doc = "CRC Status Register"]
pub mod sts;
#[doc = "LENGTH (rw) register accessor: CRC Length Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`length::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`length::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@length`]
module"]
#[doc(alias = "LENGTH")]
pub type Length = crate::Reg<length::LengthSpec>;
#[doc = "CRC Length Register"]
pub mod length;
#[doc = "CHECK (rw) register accessor: CRC Check Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`check::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`check::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@check`]
module"]
#[doc(alias = "CHECK")]
pub type Check = crate::Reg<check::CheckSpec>;
#[doc = "CRC Check Register"]
pub mod check;
#[doc = "CRC (rw) register accessor: CRC Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`crc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`crc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crc`]
module"]
#[doc(alias = "CRC")]
pub type Crc = crate::Reg<crc::CrcSpec>;
#[doc = "CRC Register"]
pub mod crc;
#[doc = "CTR (rw) register accessor: CRC Test Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctr`]
module"]
#[doc(alias = "CTR")]
pub type Ctr = crate::Reg<ctr::CtrSpec>;
#[doc = "CRC Test Register"]
pub mod ctr;
