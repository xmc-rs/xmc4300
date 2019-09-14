#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Logical Start address FMMU"]
    pub fmmu_l_start_adr: FMMU_L_START_ADR,
    #[doc = "0x04 - Length FMMU 0"]
    pub fmmu_len: FMMU_LEN,
    #[doc = "0x06 - Start bit FMMU 0 in logical address space"]
    pub fmmu_l_start_bit: FMMU_L_START_BIT,
    #[doc = "0x07 - Stop bit FMMU 0 in logical address space"]
    pub fmmu_l_stop_bit: FMMU_L_STOP_BIT,
    #[doc = "0x08 - Ph0sical Start address FMMU y"]
    pub fmmu_p_start_adr: FMMU_P_START_ADR,
    #[doc = "0x0a - Ph0sical Start bit FMMU y"]
    pub fmmu_p_start_bit: FMMU_P_START_BIT,
    #[doc = "0x0b - T0pe FMMU y"]
    pub fmmu_type: FMMU_TYPE,
    #[doc = "0x0c - Activate FMMU 0"]
    pub fmmu_act: FMMU_ACT,
}
#[doc = "Logical Start address FMMU\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [fmmu_l_start_adr](fmmu_l_start_adr) module"]
pub type FMMU_L_START_ADR = crate::Reg<u32, _FMMU_L_START_ADR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FMMU_L_START_ADR;
#[doc = "`read()` method returns [fmmu_l_start_adr::R](fmmu_l_start_adr::R) reader structure"]
impl crate::Readable for FMMU_L_START_ADR {}
#[doc = "Logical Start address FMMU"]
pub mod fmmu_l_start_adr;
#[doc = "Length FMMU 0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [fmmu_len](fmmu_len) module"]
pub type FMMU_LEN = crate::Reg<u16, _FMMU_LEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FMMU_LEN;
#[doc = "`read()` method returns [fmmu_len::R](fmmu_len::R) reader structure"]
impl crate::Readable for FMMU_LEN {}
#[doc = "Length FMMU 0"]
pub mod fmmu_len;
#[doc = "Start bit FMMU 0 in logical address space\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [fmmu_l_start_bit](fmmu_l_start_bit) module"]
pub type FMMU_L_START_BIT = crate::Reg<u8, _FMMU_L_START_BIT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FMMU_L_START_BIT;
#[doc = "`read()` method returns [fmmu_l_start_bit::R](fmmu_l_start_bit::R) reader structure"]
impl crate::Readable for FMMU_L_START_BIT {}
#[doc = "Start bit FMMU 0 in logical address space"]
pub mod fmmu_l_start_bit;
#[doc = "Stop bit FMMU 0 in logical address space\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [fmmu_l_stop_bit](fmmu_l_stop_bit) module"]
pub type FMMU_L_STOP_BIT = crate::Reg<u8, _FMMU_L_STOP_BIT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FMMU_L_STOP_BIT;
#[doc = "`read()` method returns [fmmu_l_stop_bit::R](fmmu_l_stop_bit::R) reader structure"]
impl crate::Readable for FMMU_L_STOP_BIT {}
#[doc = "Stop bit FMMU 0 in logical address space"]
pub mod fmmu_l_stop_bit;
#[doc = "Ph0sical Start address FMMU y\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [fmmu_p_start_adr](fmmu_p_start_adr) module"]
pub type FMMU_P_START_ADR = crate::Reg<u16, _FMMU_P_START_ADR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FMMU_P_START_ADR;
#[doc = "`read()` method returns [fmmu_p_start_adr::R](fmmu_p_start_adr::R) reader structure"]
impl crate::Readable for FMMU_P_START_ADR {}
#[doc = "Ph0sical Start address FMMU y"]
pub mod fmmu_p_start_adr;
#[doc = "Ph0sical Start bit FMMU y\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [fmmu_p_start_bit](fmmu_p_start_bit) module"]
pub type FMMU_P_START_BIT = crate::Reg<u8, _FMMU_P_START_BIT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FMMU_P_START_BIT;
#[doc = "`read()` method returns [fmmu_p_start_bit::R](fmmu_p_start_bit::R) reader structure"]
impl crate::Readable for FMMU_P_START_BIT {}
#[doc = "Ph0sical Start bit FMMU y"]
pub mod fmmu_p_start_bit;
#[doc = "T0pe FMMU y\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [fmmu_type](fmmu_type) module"]
pub type FMMU_TYPE = crate::Reg<u8, _FMMU_TYPE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FMMU_TYPE;
#[doc = "`read()` method returns [fmmu_type::R](fmmu_type::R) reader structure"]
impl crate::Readable for FMMU_TYPE {}
#[doc = "T0pe FMMU y"]
pub mod fmmu_type;
#[doc = "Activate FMMU 0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [fmmu_act](fmmu_act) module"]
pub type FMMU_ACT = crate::Reg<u8, _FMMU_ACT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FMMU_ACT;
#[doc = "`read()` method returns [fmmu_act::R](fmmu_act::R) reader structure"]
impl crate::Readable for FMMU_ACT {}
#[doc = "Activate FMMU 0"]
pub mod fmmu_act;
