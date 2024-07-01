#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    id: ID,
    ctr: CTR,
    rawstat: RAWSTAT,
    stssr: STSSR,
    msksr: MSKSR,
    clrsr: CLRSR,
    atim0: ATIM0,
    atim1: ATIM1,
    tim0: TIM0,
    tim1: TIM1,
}
impl RegisterBlock {
    #[doc = "0x00 - RTC ID Register"]
    #[inline(always)]
    pub const fn id(&self) -> &ID {
        &self.id
    }
    #[doc = "0x04 - RTC Control Register"]
    #[inline(always)]
    pub const fn ctr(&self) -> &CTR {
        &self.ctr
    }
    #[doc = "0x08 - RTC Raw Service Request Register"]
    #[inline(always)]
    pub const fn rawstat(&self) -> &RAWSTAT {
        &self.rawstat
    }
    #[doc = "0x0c - RTC Service Request Status Register"]
    #[inline(always)]
    pub const fn stssr(&self) -> &STSSR {
        &self.stssr
    }
    #[doc = "0x10 - RTC Service Request Mask Register"]
    #[inline(always)]
    pub const fn msksr(&self) -> &MSKSR {
        &self.msksr
    }
    #[doc = "0x14 - RTC Clear Service Request Register"]
    #[inline(always)]
    pub const fn clrsr(&self) -> &CLRSR {
        &self.clrsr
    }
    #[doc = "0x18 - RTC Alarm Time Register 0"]
    #[inline(always)]
    pub const fn atim0(&self) -> &ATIM0 {
        &self.atim0
    }
    #[doc = "0x1c - RTC Alarm Time Register 1"]
    #[inline(always)]
    pub const fn atim1(&self) -> &ATIM1 {
        &self.atim1
    }
    #[doc = "0x20 - RTC Time Register 0"]
    #[inline(always)]
    pub const fn tim0(&self) -> &TIM0 {
        &self.tim0
    }
    #[doc = "0x24 - RTC Time Register 1"]
    #[inline(always)]
    pub const fn tim1(&self) -> &TIM1 {
        &self.tim1
    }
}
#[doc = "ID (r) register accessor: RTC ID Register\n\nYou can [`read`](crate::Reg::read) this register and get [`id::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@id`]
module"]
pub type ID = crate::Reg<id::ID_SPEC>;
#[doc = "RTC ID Register"]
pub mod id;
#[doc = "CTR (rw) register accessor: RTC Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctr`]
module"]
pub type CTR = crate::Reg<ctr::CTR_SPEC>;
#[doc = "RTC Control Register"]
pub mod ctr;
#[doc = "RAWSTAT (r) register accessor: RTC Raw Service Request Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rawstat::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rawstat`]
module"]
pub type RAWSTAT = crate::Reg<rawstat::RAWSTAT_SPEC>;
#[doc = "RTC Raw Service Request Register"]
pub mod rawstat;
#[doc = "STSSR (r) register accessor: RTC Service Request Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`stssr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stssr`]
module"]
pub type STSSR = crate::Reg<stssr::STSSR_SPEC>;
#[doc = "RTC Service Request Status Register"]
pub mod stssr;
#[doc = "MSKSR (rw) register accessor: RTC Service Request Mask Register\n\nYou can [`read`](crate::Reg::read) this register and get [`msksr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`msksr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@msksr`]
module"]
pub type MSKSR = crate::Reg<msksr::MSKSR_SPEC>;
#[doc = "RTC Service Request Mask Register"]
pub mod msksr;
#[doc = "CLRSR (w) register accessor: RTC Clear Service Request Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clrsr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clrsr`]
module"]
pub type CLRSR = crate::Reg<clrsr::CLRSR_SPEC>;
#[doc = "RTC Clear Service Request Register"]
pub mod clrsr;
#[doc = "ATIM0 (rw) register accessor: RTC Alarm Time Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`atim0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`atim0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@atim0`]
module"]
pub type ATIM0 = crate::Reg<atim0::ATIM0_SPEC>;
#[doc = "RTC Alarm Time Register 0"]
pub mod atim0;
#[doc = "ATIM1 (rw) register accessor: RTC Alarm Time Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`atim1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`atim1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@atim1`]
module"]
pub type ATIM1 = crate::Reg<atim1::ATIM1_SPEC>;
#[doc = "RTC Alarm Time Register 1"]
pub mod atim1;
#[doc = "TIM0 (rw) register accessor: RTC Time Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`tim0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tim0`]
module"]
pub type TIM0 = crate::Reg<tim0::TIM0_SPEC>;
#[doc = "RTC Time Register 0"]
pub mod tim0;
#[doc = "TIM1 (rw) register accessor: RTC Time Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`tim1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tim1`]
module"]
pub type TIM1 = crate::Reg<tim1::TIM1_SPEC>;
#[doc = "RTC Time Register 1"]
pub mod tim1;
