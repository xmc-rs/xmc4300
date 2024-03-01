#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    sm_p_start_adr: SmPStartAdr,
    sm_len: SmLen,
    sm_control: SmControl,
    sm_status: SmStatus,
    sm_act: SmAct,
    sm_pdi_ctr: SmPdiCtr,
}
impl RegisterBlock {
    #[doc = "0x00 - Physical Start Address SyncManager 0"]
    #[inline(always)]
    pub const fn sm_p_start_adr(&self) -> &SmPStartAdr {
        &self.sm_p_start_adr
    }
    #[doc = "0x02 - Length SyncManager 0"]
    #[inline(always)]
    pub const fn sm_len(&self) -> &SmLen {
        &self.sm_len
    }
    #[doc = "0x04 - Control Register SyncManager 0"]
    #[inline(always)]
    pub const fn sm_control(&self) -> &SmControl {
        &self.sm_control
    }
    #[doc = "0x05 - Status Register SyncManager 0"]
    #[inline(always)]
    pub const fn sm_status(&self) -> &SmStatus {
        &self.sm_status
    }
    #[doc = "0x06 - Activate SyncManager 0"]
    #[inline(always)]
    pub const fn sm_act(&self) -> &SmAct {
        &self.sm_act
    }
    #[doc = "0x07 - PDI Control SyncManager 0"]
    #[inline(always)]
    pub const fn sm_pdi_ctr(&self) -> &SmPdiCtr {
        &self.sm_pdi_ctr
    }
}
#[doc = "SM_P_START_ADR (r) register accessor: Physical Start Address SyncManager 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sm_p_start_adr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sm_p_start_adr`]
module"]
#[doc(alias = "SM_P_START_ADR")]
pub type SmPStartAdr = crate::Reg<sm_p_start_adr::SmPStartAdrSpec>;
#[doc = "Physical Start Address SyncManager 0"]
pub mod sm_p_start_adr;
#[doc = "SM_LEN (r) register accessor: Length SyncManager 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sm_len::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sm_len`]
module"]
#[doc(alias = "SM_LEN")]
pub type SmLen = crate::Reg<sm_len::SmLenSpec>;
#[doc = "Length SyncManager 0"]
pub mod sm_len;
#[doc = "SM_CONTROL (r) register accessor: Control Register SyncManager 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sm_control::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sm_control`]
module"]
#[doc(alias = "SM_CONTROL")]
pub type SmControl = crate::Reg<sm_control::SmControlSpec>;
#[doc = "Control Register SyncManager 0"]
pub mod sm_control;
#[doc = "SM_STATUS (r) register accessor: Status Register SyncManager 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sm_status::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sm_status`]
module"]
#[doc(alias = "SM_STATUS")]
pub type SmStatus = crate::Reg<sm_status::SmStatusSpec>;
#[doc = "Status Register SyncManager 0"]
pub mod sm_status;
#[doc = "SM_ACT (r) register accessor: Activate SyncManager 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sm_act::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sm_act`]
module"]
#[doc(alias = "SM_ACT")]
pub type SmAct = crate::Reg<sm_act::SmActSpec>;
#[doc = "Activate SyncManager 0"]
pub mod sm_act;
#[doc = "SM_PDI_CTR (rw) register accessor: PDI Control SyncManager 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sm_pdi_ctr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sm_pdi_ctr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sm_pdi_ctr`]
module"]
#[doc(alias = "SM_PDI_CTR")]
pub type SmPdiCtr = crate::Reg<sm_pdi_ctr::SmPdiCtrSpec>;
#[doc = "PDI Control SyncManager 0"]
pub mod sm_pdi_ctr;
