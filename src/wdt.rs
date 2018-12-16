#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - WDT ID Register"]
    pub id: ID,
    #[doc = "0x04 - WDT Control Register"]
    pub ctr: CTR,
    #[doc = "0x08 - WDT Service Register"]
    pub srv: SRV,
    #[doc = "0x0c - WDT Timer Register"]
    pub tim: TIM,
    #[doc = "0x10 - WDT Window Lower Bound Register"]
    pub wlb: WLB,
    #[doc = "0x14 - WDT Window Upper Bound Register"]
    pub wub: WUB,
    #[doc = "0x18 - WDT Status Register"]
    pub wdtsts: WDTSTS,
    #[doc = "0x1c - WDT Clear Register"]
    pub wdtclr: WDTCLR,
}
#[doc = "WDT ID Register"]
pub struct ID {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "WDT ID Register"]
pub mod id;
#[doc = "WDT Control Register"]
pub struct CTR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "WDT Control Register"]
pub mod ctr;
#[doc = "WDT Service Register"]
pub struct SRV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "WDT Service Register"]
pub mod srv;
#[doc = "WDT Timer Register"]
pub struct TIM {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "WDT Timer Register"]
pub mod tim;
#[doc = "WDT Window Lower Bound Register"]
pub struct WLB {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "WDT Window Lower Bound Register"]
pub mod wlb;
#[doc = "WDT Window Upper Bound Register"]
pub struct WUB {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "WDT Window Upper Bound Register"]
pub mod wub;
#[doc = "WDT Status Register"]
pub struct WDTSTS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "WDT Status Register"]
pub mod wdtsts;
#[doc = "WDT Clear Register"]
pub struct WDTCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "WDT Clear Register"]
pub mod wdtclr;
