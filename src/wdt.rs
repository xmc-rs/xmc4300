#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    id: Id,
    ctr: Ctr,
    srv: Srv,
    tim: Tim,
    wlb: Wlb,
    wub: Wub,
    wdtsts: Wdtsts,
    wdtclr: Wdtclr,
}
impl RegisterBlock {
    #[doc = "0x00 - WDT ID Register"]
    #[inline(always)]
    pub const fn id(&self) -> &Id {
        &self.id
    }
    #[doc = "0x04 - WDT Control Register"]
    #[inline(always)]
    pub const fn ctr(&self) -> &Ctr {
        &self.ctr
    }
    #[doc = "0x08 - WDT Service Register"]
    #[inline(always)]
    pub const fn srv(&self) -> &Srv {
        &self.srv
    }
    #[doc = "0x0c - WDT Timer Register"]
    #[inline(always)]
    pub const fn tim(&self) -> &Tim {
        &self.tim
    }
    #[doc = "0x10 - WDT Window Lower Bound Register"]
    #[inline(always)]
    pub const fn wlb(&self) -> &Wlb {
        &self.wlb
    }
    #[doc = "0x14 - WDT Window Upper Bound Register"]
    #[inline(always)]
    pub const fn wub(&self) -> &Wub {
        &self.wub
    }
    #[doc = "0x18 - WDT Status Register"]
    #[inline(always)]
    pub const fn wdtsts(&self) -> &Wdtsts {
        &self.wdtsts
    }
    #[doc = "0x1c - WDT Clear Register"]
    #[inline(always)]
    pub const fn wdtclr(&self) -> &Wdtclr {
        &self.wdtclr
    }
}
#[doc = "ID (r) register accessor: WDT ID Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`id::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@id`]
module"]
#[doc(alias = "ID")]
pub type Id = crate::Reg<id::IdSpec>;
#[doc = "WDT ID Register"]
pub mod id;
#[doc = "CTR (rw) register accessor: WDT Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctr`]
module"]
#[doc(alias = "CTR")]
pub type Ctr = crate::Reg<ctr::CtrSpec>;
#[doc = "WDT Control Register"]
pub mod ctr;
#[doc = "SRV (w) register accessor: WDT Service Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`srv::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@srv`]
module"]
#[doc(alias = "SRV")]
pub type Srv = crate::Reg<srv::SrvSpec>;
#[doc = "WDT Service Register"]
pub mod srv;
#[doc = "TIM (r) register accessor: WDT Timer Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tim::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tim`]
module"]
#[doc(alias = "TIM")]
pub type Tim = crate::Reg<tim::TimSpec>;
#[doc = "WDT Timer Register"]
pub mod tim;
#[doc = "WLB (rw) register accessor: WDT Window Lower Bound Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wlb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wlb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wlb`]
module"]
#[doc(alias = "WLB")]
pub type Wlb = crate::Reg<wlb::WlbSpec>;
#[doc = "WDT Window Lower Bound Register"]
pub mod wlb;
#[doc = "WUB (rw) register accessor: WDT Window Upper Bound Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wub::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wub::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wub`]
module"]
#[doc(alias = "WUB")]
pub type Wub = crate::Reg<wub::WubSpec>;
#[doc = "WDT Window Upper Bound Register"]
pub mod wub;
#[doc = "WDTSTS (r) register accessor: WDT Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wdtsts::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wdtsts`]
module"]
#[doc(alias = "WDTSTS")]
pub type Wdtsts = crate::Reg<wdtsts::WdtstsSpec>;
#[doc = "WDT Status Register"]
pub mod wdtsts;
#[doc = "WDTCLR (w) register accessor: WDT Clear Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdtclr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wdtclr`]
module"]
#[doc(alias = "WDTCLR")]
pub type Wdtclr = crate::Reg<wdtclr::WdtclrSpec>;
#[doc = "WDT Clear Register"]
pub mod wdtclr;
