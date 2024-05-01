#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    fmmu_l_start_adr: FMMU_L_START_ADR,
    fmmu_len: FMMU_LEN,
    fmmu_l_start_bit: FMMU_L_START_BIT,
    fmmu_l_stop_bit: FMMU_L_STOP_BIT,
    fmmu_p_start_adr: FMMU_P_START_ADR,
    fmmu_p_start_bit: FMMU_P_START_BIT,
    fmmu_type: FMMU_TYPE,
    fmmu_act: FMMU_ACT,
}
impl RegisterBlock {
    #[doc = "0x00 - Logical Start address FMMU"]
    #[inline(always)]
    pub const fn fmmu_l_start_adr(&self) -> &FMMU_L_START_ADR {
        &self.fmmu_l_start_adr
    }
    #[doc = "0x04 - Length FMMU 0"]
    #[inline(always)]
    pub const fn fmmu_len(&self) -> &FMMU_LEN {
        &self.fmmu_len
    }
    #[doc = "0x06 - Start bit FMMU 0 in logical address space"]
    #[inline(always)]
    pub const fn fmmu_l_start_bit(&self) -> &FMMU_L_START_BIT {
        &self.fmmu_l_start_bit
    }
    #[doc = "0x07 - Stop bit FMMU 0 in logical address space"]
    #[inline(always)]
    pub const fn fmmu_l_stop_bit(&self) -> &FMMU_L_STOP_BIT {
        &self.fmmu_l_stop_bit
    }
    #[doc = "0x08 - Ph0sical Start address FMMU y"]
    #[inline(always)]
    pub const fn fmmu_p_start_adr(&self) -> &FMMU_P_START_ADR {
        &self.fmmu_p_start_adr
    }
    #[doc = "0x0a - Ph0sical Start bit FMMU y"]
    #[inline(always)]
    pub const fn fmmu_p_start_bit(&self) -> &FMMU_P_START_BIT {
        &self.fmmu_p_start_bit
    }
    #[doc = "0x0b - T0pe FMMU y"]
    #[inline(always)]
    pub const fn fmmu_type(&self) -> &FMMU_TYPE {
        &self.fmmu_type
    }
    #[doc = "0x0c - Activate FMMU 0"]
    #[inline(always)]
    pub const fn fmmu_act(&self) -> &FMMU_ACT {
        &self.fmmu_act
    }
}
#[doc = "FMMU_L_START_ADR (r) register accessor: Logical Start address FMMU\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fmmu_l_start_adr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fmmu_l_start_adr`]
module"]
pub type FMMU_L_START_ADR = crate::Reg<fmmu_l_start_adr::FMMU_L_START_ADR_SPEC>;
#[doc = "Logical Start address FMMU"]
pub mod fmmu_l_start_adr;
#[doc = "FMMU_LEN (r) register accessor: Length FMMU 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fmmu_len::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fmmu_len`]
module"]
pub type FMMU_LEN = crate::Reg<fmmu_len::FMMU_LEN_SPEC>;
#[doc = "Length FMMU 0"]
pub mod fmmu_len;
#[doc = "FMMU_L_START_BIT (r) register accessor: Start bit FMMU 0 in logical address space\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fmmu_l_start_bit::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fmmu_l_start_bit`]
module"]
pub type FMMU_L_START_BIT = crate::Reg<fmmu_l_start_bit::FMMU_L_START_BIT_SPEC>;
#[doc = "Start bit FMMU 0 in logical address space"]
pub mod fmmu_l_start_bit;
#[doc = "FMMU_L_STOP_BIT (r) register accessor: Stop bit FMMU 0 in logical address space\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fmmu_l_stop_bit::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fmmu_l_stop_bit`]
module"]
pub type FMMU_L_STOP_BIT = crate::Reg<fmmu_l_stop_bit::FMMU_L_STOP_BIT_SPEC>;
#[doc = "Stop bit FMMU 0 in logical address space"]
pub mod fmmu_l_stop_bit;
#[doc = "FMMU_P_START_ADR (r) register accessor: Ph0sical Start address FMMU y\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fmmu_p_start_adr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fmmu_p_start_adr`]
module"]
pub type FMMU_P_START_ADR = crate::Reg<fmmu_p_start_adr::FMMU_P_START_ADR_SPEC>;
#[doc = "Ph0sical Start address FMMU y"]
pub mod fmmu_p_start_adr;
#[doc = "FMMU_P_START_BIT (r) register accessor: Ph0sical Start bit FMMU y\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fmmu_p_start_bit::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fmmu_p_start_bit`]
module"]
pub type FMMU_P_START_BIT = crate::Reg<fmmu_p_start_bit::FMMU_P_START_BIT_SPEC>;
#[doc = "Ph0sical Start bit FMMU y"]
pub mod fmmu_p_start_bit;
#[doc = "FMMU_TYPE (r) register accessor: T0pe FMMU y\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fmmu_type::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fmmu_type`]
module"]
pub type FMMU_TYPE = crate::Reg<fmmu_type::FMMU_TYPE_SPEC>;
#[doc = "T0pe FMMU y"]
pub mod fmmu_type;
#[doc = "FMMU_ACT (r) register accessor: Activate FMMU 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fmmu_act::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fmmu_act`]
module"]
pub type FMMU_ACT = crate::Reg<fmmu_act::FMMU_ACT_SPEC>;
#[doc = "Activate FMMU 0"]
pub mod fmmu_act;
