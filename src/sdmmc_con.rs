#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    sdmmc_con: SDMMC_CON,
}
impl RegisterBlock {
    #[doc = "0x00 - SDMMC Configuration"]
    #[inline(always)]
    pub const fn sdmmc_con(&self) -> &SDMMC_CON {
        &self.sdmmc_con
    }
}
#[doc = "SDMMC_CON (rw) register accessor: SDMMC Configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdmmc_con::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdmmc_con::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdmmc_con`]
module"]
pub type SDMMC_CON = crate::Reg<sdmmc_con::SDMMC_CON_SPEC>;
#[doc = "SDMMC Configuration"]
pub mod sdmmc_con;
