#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Physical Start Address SyncManager 0"]
    pub sm_p_start_adr: SM_P_START_ADR,
    #[doc = "0x02 - Length SyncManager 0"]
    pub sm_len: SM_LEN,
    #[doc = "0x04 - Control Register SyncManager 0"]
    pub sm_control: SM_CONTROL,
    #[doc = "0x05 - Status Register SyncManager 0"]
    pub sm_status: SM_STATUS,
    #[doc = "0x06 - Activate SyncManager 0"]
    pub sm_act: SM_ACT,
    #[doc = "0x07 - PDI Control SyncManager 0"]
    pub sm_pdi_ctr: SM_PDI_CTR,
}
#[doc = "SM_P_START_ADR (r) register accessor: Physical Start Address SyncManager 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sm_p_start_adr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sm_p_start_adr`]
module"]
pub type SM_P_START_ADR = crate::Reg<sm_p_start_adr::SM_P_START_ADR_SPEC>;
#[doc = "Physical Start Address SyncManager 0"]
pub mod sm_p_start_adr;
#[doc = "SM_LEN (r) register accessor: Length SyncManager 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sm_len::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sm_len`]
module"]
pub type SM_LEN = crate::Reg<sm_len::SM_LEN_SPEC>;
#[doc = "Length SyncManager 0"]
pub mod sm_len;
#[doc = "SM_CONTROL (r) register accessor: Control Register SyncManager 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sm_control::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sm_control`]
module"]
pub type SM_CONTROL = crate::Reg<sm_control::SM_CONTROL_SPEC>;
#[doc = "Control Register SyncManager 0"]
pub mod sm_control;
#[doc = "SM_STATUS (r) register accessor: Status Register SyncManager 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sm_status::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sm_status`]
module"]
pub type SM_STATUS = crate::Reg<sm_status::SM_STATUS_SPEC>;
#[doc = "Status Register SyncManager 0"]
pub mod sm_status;
#[doc = "SM_ACT (r) register accessor: Activate SyncManager 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sm_act::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sm_act`]
module"]
pub type SM_ACT = crate::Reg<sm_act::SM_ACT_SPEC>;
#[doc = "Activate SyncManager 0"]
pub mod sm_act;
#[doc = "SM_PDI_CTR (rw) register accessor: PDI Control SyncManager 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sm_pdi_ctr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sm_pdi_ctr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sm_pdi_ctr`]
module"]
pub type SM_PDI_CTR = crate::Reg<sm_pdi_ctr::SM_PDI_CTR_SPEC>;
#[doc = "PDI Control SyncManager 0"]
pub mod sm_pdi_ctr;
