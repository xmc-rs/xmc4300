#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Trap Status Register"]
    pub trapstat: TRAPSTAT,
    #[doc = "0x04 - Trap Raw Status Register"]
    pub trapraw: TRAPRAW,
    #[doc = "0x08 - Trap Disable Register"]
    pub trapdis: TRAPDIS,
    #[doc = "0x0c - Trap Clear Register"]
    pub trapclr: TRAPCLR,
    #[doc = "0x10 - Trap Set Register"]
    pub trapset: TRAPSET,
}
#[doc = "Trap Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trapstat](trapstat) module"]
pub type TRAPSTAT = crate::Reg<u32, _TRAPSTAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TRAPSTAT;
#[doc = "`read()` method returns [trapstat::R](trapstat::R) reader structure"]
impl crate::Readable for TRAPSTAT {}
#[doc = "Trap Status Register"]
pub mod trapstat;
#[doc = "Trap Raw Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trapraw](trapraw) module"]
pub type TRAPRAW = crate::Reg<u32, _TRAPRAW>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TRAPRAW;
#[doc = "`read()` method returns [trapraw::R](trapraw::R) reader structure"]
impl crate::Readable for TRAPRAW {}
#[doc = "Trap Raw Status Register"]
pub mod trapraw;
#[doc = "Trap Disable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trapdis](trapdis) module"]
pub type TRAPDIS = crate::Reg<u32, _TRAPDIS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TRAPDIS;
#[doc = "`read()` method returns [trapdis::R](trapdis::R) reader structure"]
impl crate::Readable for TRAPDIS {}
#[doc = "`write(|w| ..)` method takes [trapdis::W](trapdis::W) writer structure"]
impl crate::Writable for TRAPDIS {}
#[doc = "Trap Disable Register"]
pub mod trapdis;
#[doc = "Trap Clear Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trapclr](trapclr) module"]
pub type TRAPCLR = crate::Reg<u32, _TRAPCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TRAPCLR;
#[doc = "`write(|w| ..)` method takes [trapclr::W](trapclr::W) writer structure"]
impl crate::Writable for TRAPCLR {}
#[doc = "Trap Clear Register"]
pub mod trapclr;
#[doc = "Trap Set Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trapset](trapset) module"]
pub type TRAPSET = crate::Reg<u32, _TRAPSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TRAPSET;
#[doc = "`write(|w| ..)` method takes [trapset::W](trapset::W) writer structure"]
impl crate::Writable for TRAPSET {}
#[doc = "Trap Set Register"]
pub mod trapset;
