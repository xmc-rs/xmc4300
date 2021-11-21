#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Logical Start address FMMU"]
    pub fmmu_l_start_adr: crate::Reg<fmmu_l_start_adr::FMMU_L_START_ADR_SPEC>,
    #[doc = "0x04 - Length FMMU 0"]
    pub fmmu_len: crate::Reg<fmmu_len::FMMU_LEN_SPEC>,
    #[doc = "0x06 - Start bit FMMU 0 in logical address space"]
    pub fmmu_l_start_bit: crate::Reg<fmmu_l_start_bit::FMMU_L_START_BIT_SPEC>,
    #[doc = "0x07 - Stop bit FMMU 0 in logical address space"]
    pub fmmu_l_stop_bit: crate::Reg<fmmu_l_stop_bit::FMMU_L_STOP_BIT_SPEC>,
    #[doc = "0x08 - Ph0sical Start address FMMU y"]
    pub fmmu_p_start_adr: crate::Reg<fmmu_p_start_adr::FMMU_P_START_ADR_SPEC>,
    #[doc = "0x0a - Ph0sical Start bit FMMU y"]
    pub fmmu_p_start_bit: crate::Reg<fmmu_p_start_bit::FMMU_P_START_BIT_SPEC>,
    #[doc = "0x0b - T0pe FMMU y"]
    pub fmmu_type: crate::Reg<fmmu_type::FMMU_TYPE_SPEC>,
    #[doc = "0x0c - Activate FMMU 0"]
    pub fmmu_act: crate::Reg<fmmu_act::FMMU_ACT_SPEC>,
}
#[doc = "FMMU_L_START_ADR register accessor: an alias for `Reg<FMMU_L_START_ADR_SPEC>`"]
pub type FMMU_L_START_ADR = crate::Reg<fmmu_l_start_adr::FMMU_L_START_ADR_SPEC>;
#[doc = "Logical Start address FMMU"]
pub mod fmmu_l_start_adr;
#[doc = "FMMU_LEN register accessor: an alias for `Reg<FMMU_LEN_SPEC>`"]
pub type FMMU_LEN = crate::Reg<fmmu_len::FMMU_LEN_SPEC>;
#[doc = "Length FMMU 0"]
pub mod fmmu_len;
#[doc = "FMMU_L_START_BIT register accessor: an alias for `Reg<FMMU_L_START_BIT_SPEC>`"]
pub type FMMU_L_START_BIT = crate::Reg<fmmu_l_start_bit::FMMU_L_START_BIT_SPEC>;
#[doc = "Start bit FMMU 0 in logical address space"]
pub mod fmmu_l_start_bit;
#[doc = "FMMU_L_STOP_BIT register accessor: an alias for `Reg<FMMU_L_STOP_BIT_SPEC>`"]
pub type FMMU_L_STOP_BIT = crate::Reg<fmmu_l_stop_bit::FMMU_L_STOP_BIT_SPEC>;
#[doc = "Stop bit FMMU 0 in logical address space"]
pub mod fmmu_l_stop_bit;
#[doc = "FMMU_P_START_ADR register accessor: an alias for `Reg<FMMU_P_START_ADR_SPEC>`"]
pub type FMMU_P_START_ADR = crate::Reg<fmmu_p_start_adr::FMMU_P_START_ADR_SPEC>;
#[doc = "Ph0sical Start address FMMU y"]
pub mod fmmu_p_start_adr;
#[doc = "FMMU_P_START_BIT register accessor: an alias for `Reg<FMMU_P_START_BIT_SPEC>`"]
pub type FMMU_P_START_BIT = crate::Reg<fmmu_p_start_bit::FMMU_P_START_BIT_SPEC>;
#[doc = "Ph0sical Start bit FMMU y"]
pub mod fmmu_p_start_bit;
#[doc = "FMMU_TYPE register accessor: an alias for `Reg<FMMU_TYPE_SPEC>`"]
pub type FMMU_TYPE = crate::Reg<fmmu_type::FMMU_TYPE_SPEC>;
#[doc = "T0pe FMMU y"]
pub mod fmmu_type;
#[doc = "FMMU_ACT register accessor: an alias for `Reg<FMMU_ACT_SPEC>`"]
pub type FMMU_ACT = crate::Reg<fmmu_act::FMMU_ACT_SPEC>;
#[doc = "Activate FMMU 0"]
pub mod fmmu_act;
