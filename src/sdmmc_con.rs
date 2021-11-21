#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - SDMMC Configuration"]
    pub sdmmc_con: crate::Reg<sdmmc_con::SDMMC_CON_SPEC>,
}
#[doc = "SDMMC_CON register accessor: an alias for `Reg<SDMMC_CON_SPEC>`"]
pub type SDMMC_CON = crate::Reg<sdmmc_con::SDMMC_CON_SPEC>;
#[doc = "SDMMC Configuration"]
pub mod sdmmc_con;
