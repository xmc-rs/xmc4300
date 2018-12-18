#[doc = r" Register block"]
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
#[doc = "Physical Start Address SyncManager 0"]
pub struct SM_P_START_ADR {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Physical Start Address SyncManager 0"]
pub mod sm_p_start_adr;
#[doc = "Length SyncManager 0"]
pub struct SM_LEN {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Length SyncManager 0"]
pub mod sm_len;
#[doc = "Control Register SyncManager 0"]
pub struct SM_CONTROL {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Control Register SyncManager 0"]
pub mod sm_control;
#[doc = "Status Register SyncManager 0"]
pub struct SM_STATUS {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Status Register SyncManager 0"]
pub mod sm_status;
#[doc = "Activate SyncManager 0"]
pub struct SM_ACT {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Activate SyncManager 0"]
pub mod sm_act;
#[doc = "PDI Control SyncManager 0"]
pub struct SM_PDI_CTR {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "PDI Control SyncManager 0"]
pub mod sm_pdi_ctr;
