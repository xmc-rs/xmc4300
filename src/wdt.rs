#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    id: ID,
    ctr: CTR,
    srv: SRV,
    tim: TIM,
    wlb: WLB,
    wub: WUB,
    wdtsts: WDTSTS,
    wdtclr: WDTCLR,
}
impl RegisterBlock {
    #[doc = "0x00 - WDT ID Register"]
    #[inline(always)]
    pub const fn id(&self) -> &ID {
        &self.id
    }
    #[doc = "0x04 - WDT Control Register"]
    #[inline(always)]
    pub const fn ctr(&self) -> &CTR {
        &self.ctr
    }
    #[doc = "0x08 - WDT Service Register"]
    #[inline(always)]
    pub const fn srv(&self) -> &SRV {
        &self.srv
    }
    #[doc = "0x0c - WDT Timer Register"]
    #[inline(always)]
    pub const fn tim(&self) -> &TIM {
        &self.tim
    }
    #[doc = "0x10 - WDT Window Lower Bound Register"]
    #[inline(always)]
    pub const fn wlb(&self) -> &WLB {
        &self.wlb
    }
    #[doc = "0x14 - WDT Window Upper Bound Register"]
    #[inline(always)]
    pub const fn wub(&self) -> &WUB {
        &self.wub
    }
    #[doc = "0x18 - WDT Status Register"]
    #[inline(always)]
    pub const fn wdtsts(&self) -> &WDTSTS {
        &self.wdtsts
    }
    #[doc = "0x1c - WDT Clear Register"]
    #[inline(always)]
    pub const fn wdtclr(&self) -> &WDTCLR {
        &self.wdtclr
    }
}
#[doc = "ID (r) register accessor: WDT ID Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`id::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@id`]
module"]
pub type ID = crate::Reg<id::ID_SPEC>;
#[doc = "WDT ID Register"]
pub mod id;
#[doc = "CTR (rw) register accessor: WDT Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctr`]
module"]
pub type CTR = crate::Reg<ctr::CTR_SPEC>;
#[doc = "WDT Control Register"]
pub mod ctr;
#[doc = "SRV (w) register accessor: WDT Service Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`srv::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@srv`]
module"]
pub type SRV = crate::Reg<srv::SRV_SPEC>;
#[doc = "WDT Service Register"]
pub mod srv;
#[doc = "TIM (r) register accessor: WDT Timer Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tim::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tim`]
module"]
pub type TIM = crate::Reg<tim::TIM_SPEC>;
#[doc = "WDT Timer Register"]
pub mod tim;
#[doc = "WLB (rw) register accessor: WDT Window Lower Bound Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wlb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wlb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wlb`]
module"]
pub type WLB = crate::Reg<wlb::WLB_SPEC>;
#[doc = "WDT Window Lower Bound Register"]
pub mod wlb;
#[doc = "WUB (rw) register accessor: WDT Window Upper Bound Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wub::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wub::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wub`]
module"]
pub type WUB = crate::Reg<wub::WUB_SPEC>;
#[doc = "WDT Window Upper Bound Register"]
pub mod wub;
#[doc = "WDTSTS (r) register accessor: WDT Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wdtsts::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wdtsts`]
module"]
pub type WDTSTS = crate::Reg<wdtsts::WDTSTS_SPEC>;
#[doc = "WDT Status Register"]
pub mod wdtsts;
#[doc = "WDTCLR (w) register accessor: WDT Clear Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdtclr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wdtclr`]
module"]
pub type WDTCLR = crate::Reg<wdtclr::WDTCLR_SPEC>;
#[doc = "WDT Clear Register"]
pub mod wdtclr;
