#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Event Input Select"]
    pub exisel: EXISEL,
    _reserved1: [u8; 0x0c],
    #[doc = "0x10..0x20 - Event Input Control"]
    pub exicon: [EXICON; 4],
    #[doc = "0x20..0x30 - Event Output Trigger Control"]
    pub exocon: [EXOCON; 4],
}
#[doc = "EXISEL (rw) register accessor: Event Input Select\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`exisel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`exisel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`exisel`]
module"]
pub type EXISEL = crate::Reg<exisel::EXISEL_SPEC>;
#[doc = "Event Input Select"]
pub mod exisel;
#[doc = "EXICON (rw) register accessor: Event Input Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`exicon::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`exicon::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`exicon`]
module"]
pub type EXICON = crate::Reg<exicon::EXICON_SPEC>;
#[doc = "Event Input Control"]
pub mod exicon;
#[doc = "EXOCON (rw) register accessor: Event Output Trigger Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`exocon::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`exocon::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`exocon`]
module"]
pub type EXOCON = crate::Reg<exocon::EXOCON_SPEC>;
#[doc = "Event Output Trigger Control"]
pub mod exocon;
