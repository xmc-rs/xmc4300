#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    fmmu_l_start_adr: FmmuLStartAdr,
    fmmu_len: FmmuLen,
    fmmu_l_start_bit: FmmuLStartBit,
    fmmu_l_stop_bit: FmmuLStopBit,
    fmmu_p_start_adr: FmmuPStartAdr,
    fmmu_p_start_bit: FmmuPStartBit,
    fmmu_type: FmmuType,
    fmmu_act: FmmuAct,
}
impl RegisterBlock {
    #[doc = "0x00 - Logical Start address FMMU"]
    #[inline(always)]
    pub const fn fmmu_l_start_adr(&self) -> &FmmuLStartAdr {
        &self.fmmu_l_start_adr
    }
    #[doc = "0x04 - Length FMMU 0"]
    #[inline(always)]
    pub const fn fmmu_len(&self) -> &FmmuLen {
        &self.fmmu_len
    }
    #[doc = "0x06 - Start bit FMMU 0 in logical address space"]
    #[inline(always)]
    pub const fn fmmu_l_start_bit(&self) -> &FmmuLStartBit {
        &self.fmmu_l_start_bit
    }
    #[doc = "0x07 - Stop bit FMMU 0 in logical address space"]
    #[inline(always)]
    pub const fn fmmu_l_stop_bit(&self) -> &FmmuLStopBit {
        &self.fmmu_l_stop_bit
    }
    #[doc = "0x08 - Ph0sical Start address FMMU y"]
    #[inline(always)]
    pub const fn fmmu_p_start_adr(&self) -> &FmmuPStartAdr {
        &self.fmmu_p_start_adr
    }
    #[doc = "0x0a - Ph0sical Start bit FMMU y"]
    #[inline(always)]
    pub const fn fmmu_p_start_bit(&self) -> &FmmuPStartBit {
        &self.fmmu_p_start_bit
    }
    #[doc = "0x0b - T0pe FMMU y"]
    #[inline(always)]
    pub const fn fmmu_type(&self) -> &FmmuType {
        &self.fmmu_type
    }
    #[doc = "0x0c - Activate FMMU 0"]
    #[inline(always)]
    pub const fn fmmu_act(&self) -> &FmmuAct {
        &self.fmmu_act
    }
}
#[doc = "FMMU_L_START_ADR (r) register accessor: Logical Start address FMMU\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fmmu_l_start_adr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fmmu_l_start_adr`]
module"]
#[doc(alias = "FMMU_L_START_ADR")]
pub type FmmuLStartAdr = crate::Reg<fmmu_l_start_adr::FmmuLStartAdrSpec>;
#[doc = "Logical Start address FMMU"]
pub mod fmmu_l_start_adr;
#[doc = "FMMU_LEN (r) register accessor: Length FMMU 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fmmu_len::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fmmu_len`]
module"]
#[doc(alias = "FMMU_LEN")]
pub type FmmuLen = crate::Reg<fmmu_len::FmmuLenSpec>;
#[doc = "Length FMMU 0"]
pub mod fmmu_len;
#[doc = "FMMU_L_START_BIT (r) register accessor: Start bit FMMU 0 in logical address space\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fmmu_l_start_bit::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fmmu_l_start_bit`]
module"]
#[doc(alias = "FMMU_L_START_BIT")]
pub type FmmuLStartBit = crate::Reg<fmmu_l_start_bit::FmmuLStartBitSpec>;
#[doc = "Start bit FMMU 0 in logical address space"]
pub mod fmmu_l_start_bit;
#[doc = "FMMU_L_STOP_BIT (r) register accessor: Stop bit FMMU 0 in logical address space\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fmmu_l_stop_bit::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fmmu_l_stop_bit`]
module"]
#[doc(alias = "FMMU_L_STOP_BIT")]
pub type FmmuLStopBit = crate::Reg<fmmu_l_stop_bit::FmmuLStopBitSpec>;
#[doc = "Stop bit FMMU 0 in logical address space"]
pub mod fmmu_l_stop_bit;
#[doc = "FMMU_P_START_ADR (r) register accessor: Ph0sical Start address FMMU y\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fmmu_p_start_adr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fmmu_p_start_adr`]
module"]
#[doc(alias = "FMMU_P_START_ADR")]
pub type FmmuPStartAdr = crate::Reg<fmmu_p_start_adr::FmmuPStartAdrSpec>;
#[doc = "Ph0sical Start address FMMU y"]
pub mod fmmu_p_start_adr;
#[doc = "FMMU_P_START_BIT (r) register accessor: Ph0sical Start bit FMMU y\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fmmu_p_start_bit::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fmmu_p_start_bit`]
module"]
#[doc(alias = "FMMU_P_START_BIT")]
pub type FmmuPStartBit = crate::Reg<fmmu_p_start_bit::FmmuPStartBitSpec>;
#[doc = "Ph0sical Start bit FMMU y"]
pub mod fmmu_p_start_bit;
#[doc = "FMMU_TYPE (r) register accessor: T0pe FMMU y\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fmmu_type::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fmmu_type`]
module"]
#[doc(alias = "FMMU_TYPE")]
pub type FmmuType = crate::Reg<fmmu_type::FmmuTypeSpec>;
#[doc = "T0pe FMMU y"]
pub mod fmmu_type;
#[doc = "FMMU_ACT (r) register accessor: Activate FMMU 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fmmu_act::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fmmu_act`]
module"]
#[doc(alias = "FMMU_ACT")]
pub type FmmuAct = crate::Reg<fmmu_act::FmmuActSpec>;
#[doc = "Activate FMMU 0"]
pub mod fmmu_act;
