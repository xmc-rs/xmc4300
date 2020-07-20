#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - SDMMC Configuration"]
    pub sdmmc_con: SDMMC_CON,
}
#[doc = "SDMMC Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sdmmc_con](sdmmc_con) module"]
pub type SDMMC_CON = crate::Reg<u32, _SDMMC_CON>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SDMMC_CON;
#[doc = "`read()` method returns [sdmmc_con::R](sdmmc_con::R) reader structure"]
impl crate::Readable for SDMMC_CON {}
#[doc = "`write(|w| ..)` method takes [sdmmc_con::W](sdmmc_con::W) writer structure"]
impl crate::Writable for SDMMC_CON {}
#[doc = "SDMMC Configuration"]
pub mod sdmmc_con;
