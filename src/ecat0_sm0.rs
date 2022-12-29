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
#[doc = "SM_P_START_ADR (r) register accessor: an alias for `Reg<SM_P_START_ADR_SPEC>`"]
pub type SM_P_START_ADR = crate::Reg<sm_p_start_adr::SM_P_START_ADR_SPEC>;
#[doc = "Physical Start Address SyncManager 0"]
pub mod sm_p_start_adr;
#[doc = "SM_LEN (r) register accessor: an alias for `Reg<SM_LEN_SPEC>`"]
pub type SM_LEN = crate::Reg<sm_len::SM_LEN_SPEC>;
#[doc = "Length SyncManager 0"]
pub mod sm_len;
#[doc = "SM_CONTROL (r) register accessor: an alias for `Reg<SM_CONTROL_SPEC>`"]
pub type SM_CONTROL = crate::Reg<sm_control::SM_CONTROL_SPEC>;
#[doc = "Control Register SyncManager 0"]
pub mod sm_control;
#[doc = "SM_STATUS (r) register accessor: an alias for `Reg<SM_STATUS_SPEC>`"]
pub type SM_STATUS = crate::Reg<sm_status::SM_STATUS_SPEC>;
#[doc = "Status Register SyncManager 0"]
pub mod sm_status;
#[doc = "SM_ACT (r) register accessor: an alias for `Reg<SM_ACT_SPEC>`"]
pub type SM_ACT = crate::Reg<sm_act::SM_ACT_SPEC>;
#[doc = "Activate SyncManager 0"]
pub mod sm_act;
#[doc = "SM_PDI_CTR (rw) register accessor: an alias for `Reg<SM_PDI_CTR_SPEC>`"]
pub type SM_PDI_CTR = crate::Reg<sm_pdi_ctr::SM_PDI_CTR_SPEC>;
#[doc = "PDI Control SyncManager 0"]
pub mod sm_pdi_ctr;
