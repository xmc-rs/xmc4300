#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - SDMMC Configuration"]
    pub sdmmc_con: SDMMC_CON,
}
#[doc = "SDMMC Configuration"]
pub struct SDMMC_CON {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SDMMC Configuration"]
pub mod sdmmc_con;
