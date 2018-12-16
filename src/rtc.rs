#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - RTC ID Register"]
    pub id: ID,
    #[doc = "0x04 - RTC Control Register"]
    pub ctr: CTR,
    #[doc = "0x08 - RTC Raw Service Request Register"]
    pub rawstat: RAWSTAT,
    #[doc = "0x0c - RTC Service Request Status Register"]
    pub stssr: STSSR,
    #[doc = "0x10 - RTC Service Request Mask Register"]
    pub msksr: MSKSR,
    #[doc = "0x14 - RTC Clear Service Request Register"]
    pub clrsr: CLRSR,
    #[doc = "0x18 - RTC Alarm Time Register 0"]
    pub atim0: ATIM0,
    #[doc = "0x1c - RTC Alarm Time Register 1"]
    pub atim1: ATIM1,
    #[doc = "0x20 - RTC Time Register 0"]
    pub tim0: TIM0,
    #[doc = "0x24 - RTC Time Register 1"]
    pub tim1: TIM1,
}
#[doc = "RTC ID Register"]
pub struct ID {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RTC ID Register"]
pub mod id;
#[doc = "RTC Control Register"]
pub struct CTR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RTC Control Register"]
pub mod ctr;
#[doc = "RTC Raw Service Request Register"]
pub struct RAWSTAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RTC Raw Service Request Register"]
pub mod rawstat;
#[doc = "RTC Service Request Status Register"]
pub struct STSSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RTC Service Request Status Register"]
pub mod stssr;
#[doc = "RTC Service Request Mask Register"]
pub struct MSKSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RTC Service Request Mask Register"]
pub mod msksr;
#[doc = "RTC Clear Service Request Register"]
pub struct CLRSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RTC Clear Service Request Register"]
pub mod clrsr;
#[doc = "RTC Alarm Time Register 0"]
pub struct ATIM0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RTC Alarm Time Register 0"]
pub mod atim0;
#[doc = "RTC Alarm Time Register 1"]
pub struct ATIM1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RTC Alarm Time Register 1"]
pub mod atim1;
#[doc = "RTC Time Register 0"]
pub struct TIM0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RTC Time Register 0"]
pub mod tim0;
#[doc = "RTC Time Register 1"]
pub struct TIM1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RTC Time Register 1"]
pub mod tim1;
