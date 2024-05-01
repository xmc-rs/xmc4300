#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    id: Id,
    ctr: Ctr,
    rawstat: Rawstat,
    stssr: Stssr,
    msksr: Msksr,
    clrsr: Clrsr,
    atim0: Atim0,
    atim1: Atim1,
    tim0: Tim0,
    tim1: Tim1,
}
impl RegisterBlock {
    #[doc = "0x00 - RTC ID Register"]
    #[inline(always)]
    pub const fn id(&self) -> &Id {
        &self.id
    }
    #[doc = "0x04 - RTC Control Register"]
    #[inline(always)]
    pub const fn ctr(&self) -> &Ctr {
        &self.ctr
    }
    #[doc = "0x08 - RTC Raw Service Request Register"]
    #[inline(always)]
    pub const fn rawstat(&self) -> &Rawstat {
        &self.rawstat
    }
    #[doc = "0x0c - RTC Service Request Status Register"]
    #[inline(always)]
    pub const fn stssr(&self) -> &Stssr {
        &self.stssr
    }
    #[doc = "0x10 - RTC Service Request Mask Register"]
    #[inline(always)]
    pub const fn msksr(&self) -> &Msksr {
        &self.msksr
    }
    #[doc = "0x14 - RTC Clear Service Request Register"]
    #[inline(always)]
    pub const fn clrsr(&self) -> &Clrsr {
        &self.clrsr
    }
    #[doc = "0x18 - RTC Alarm Time Register 0"]
    #[inline(always)]
    pub const fn atim0(&self) -> &Atim0 {
        &self.atim0
    }
    #[doc = "0x1c - RTC Alarm Time Register 1"]
    #[inline(always)]
    pub const fn atim1(&self) -> &Atim1 {
        &self.atim1
    }
    #[doc = "0x20 - RTC Time Register 0"]
    #[inline(always)]
    pub const fn tim0(&self) -> &Tim0 {
        &self.tim0
    }
    #[doc = "0x24 - RTC Time Register 1"]
    #[inline(always)]
    pub const fn tim1(&self) -> &Tim1 {
        &self.tim1
    }
}
#[doc = "ID (r) register accessor: RTC ID Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`id::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@id`]
module"]
#[doc(alias = "ID")]
pub type Id = crate::Reg<id::IdSpec>;
#[doc = "RTC ID Register"]
pub mod id;
#[doc = "CTR (rw) register accessor: RTC Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctr`]
module"]
#[doc(alias = "CTR")]
pub type Ctr = crate::Reg<ctr::CtrSpec>;
#[doc = "RTC Control Register"]
pub mod ctr;
#[doc = "RAWSTAT (r) register accessor: RTC Raw Service Request Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rawstat::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rawstat`]
module"]
#[doc(alias = "RAWSTAT")]
pub type Rawstat = crate::Reg<rawstat::RawstatSpec>;
#[doc = "RTC Raw Service Request Register"]
pub mod rawstat;
#[doc = "STSSR (r) register accessor: RTC Service Request Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stssr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stssr`]
module"]
#[doc(alias = "STSSR")]
pub type Stssr = crate::Reg<stssr::StssrSpec>;
#[doc = "RTC Service Request Status Register"]
pub mod stssr;
#[doc = "MSKSR (rw) register accessor: RTC Service Request Mask Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`msksr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`msksr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@msksr`]
module"]
#[doc(alias = "MSKSR")]
pub type Msksr = crate::Reg<msksr::MsksrSpec>;
#[doc = "RTC Service Request Mask Register"]
pub mod msksr;
#[doc = "CLRSR (w) register accessor: RTC Clear Service Request Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clrsr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clrsr`]
module"]
#[doc(alias = "CLRSR")]
pub type Clrsr = crate::Reg<clrsr::ClrsrSpec>;
#[doc = "RTC Clear Service Request Register"]
pub mod clrsr;
#[doc = "ATIM0 (rw) register accessor: RTC Alarm Time Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`atim0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`atim0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@atim0`]
module"]
#[doc(alias = "ATIM0")]
pub type Atim0 = crate::Reg<atim0::Atim0Spec>;
#[doc = "RTC Alarm Time Register 0"]
pub mod atim0;
#[doc = "ATIM1 (rw) register accessor: RTC Alarm Time Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`atim1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`atim1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@atim1`]
module"]
#[doc(alias = "ATIM1")]
pub type Atim1 = crate::Reg<atim1::Atim1Spec>;
#[doc = "RTC Alarm Time Register 1"]
pub mod atim1;
#[doc = "TIM0 (rw) register accessor: RTC Time Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tim0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tim0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tim0`]
module"]
#[doc(alias = "TIM0")]
pub type Tim0 = crate::Reg<tim0::Tim0Spec>;
#[doc = "RTC Time Register 0"]
pub mod tim0;
#[doc = "TIM1 (rw) register accessor: RTC Time Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tim1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tim1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tim1`]
module"]
#[doc(alias = "TIM1")]
pub type Tim1 = crate::Reg<tim1::Tim1Spec>;
#[doc = "RTC Time Register 1"]
pub mod tim1;
