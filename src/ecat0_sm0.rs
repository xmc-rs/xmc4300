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
#[doc = "Physical Start Address SyncManager 0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sm_p_start_adr](sm_p_start_adr) module"]
pub type SM_P_START_ADR = crate::Reg<u16, _SM_P_START_ADR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SM_P_START_ADR;
#[doc = "`read()` method returns [sm_p_start_adr::R](sm_p_start_adr::R) reader structure"]
impl crate::Readable for SM_P_START_ADR {}
#[doc = "Physical Start Address SyncManager 0"]
pub mod sm_p_start_adr;
#[doc = "Length SyncManager 0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sm_len](sm_len) module"]
pub type SM_LEN = crate::Reg<u16, _SM_LEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SM_LEN;
#[doc = "`read()` method returns [sm_len::R](sm_len::R) reader structure"]
impl crate::Readable for SM_LEN {}
#[doc = "Length SyncManager 0"]
pub mod sm_len;
#[doc = "Control Register SyncManager 0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sm_control](sm_control) module"]
pub type SM_CONTROL = crate::Reg<u8, _SM_CONTROL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SM_CONTROL;
#[doc = "`read()` method returns [sm_control::R](sm_control::R) reader structure"]
impl crate::Readable for SM_CONTROL {}
#[doc = "Control Register SyncManager 0"]
pub mod sm_control;
#[doc = "Status Register SyncManager 0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sm_status](sm_status) module"]
pub type SM_STATUS = crate::Reg<u8, _SM_STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SM_STATUS;
#[doc = "`read()` method returns [sm_status::R](sm_status::R) reader structure"]
impl crate::Readable for SM_STATUS {}
#[doc = "Status Register SyncManager 0"]
pub mod sm_status;
#[doc = "Activate SyncManager 0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sm_act](sm_act) module"]
pub type SM_ACT = crate::Reg<u8, _SM_ACT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SM_ACT;
#[doc = "`read()` method returns [sm_act::R](sm_act::R) reader structure"]
impl crate::Readable for SM_ACT {}
#[doc = "Activate SyncManager 0"]
pub mod sm_act;
#[doc = "PDI Control SyncManager 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sm_pdi_ctr](sm_pdi_ctr) module"]
pub type SM_PDI_CTR = crate::Reg<u8, _SM_PDI_CTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SM_PDI_CTR;
#[doc = "`read()` method returns [sm_pdi_ctr::R](sm_pdi_ctr::R) reader structure"]
impl crate::Readable for SM_PDI_CTR {}
#[doc = "`write(|w| ..)` method takes [sm_pdi_ctr::W](sm_pdi_ctr::W) writer structure"]
impl crate::Writable for SM_PDI_CTR {}
#[doc = "PDI Control SyncManager 0"]
pub mod sm_pdi_ctr;
