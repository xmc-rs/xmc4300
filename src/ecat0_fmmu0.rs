#[doc = r" Register block"]
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
#[doc = "Logical Start address FMMU"]
pub struct FMMU_L_START_ADR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Logical Start address FMMU"]
pub mod fmmu_l_start_adr;
#[doc = "Length FMMU 0"]
pub struct FMMU_LEN {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Length FMMU 0"]
pub mod fmmu_len;
#[doc = "Start bit FMMU 0 in logical address space"]
pub struct FMMU_L_START_BIT {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Start bit FMMU 0 in logical address space"]
pub mod fmmu_l_start_bit;
#[doc = "Stop bit FMMU 0 in logical address space"]
pub struct FMMU_L_STOP_BIT {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Stop bit FMMU 0 in logical address space"]
pub mod fmmu_l_stop_bit;
#[doc = "Ph0sical Start address FMMU y"]
pub struct FMMU_P_START_ADR {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Ph0sical Start address FMMU y"]
pub mod fmmu_p_start_adr;
#[doc = "Ph0sical Start bit FMMU y"]
pub struct FMMU_P_START_BIT {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Ph0sical Start bit FMMU y"]
pub mod fmmu_p_start_bit;
#[doc = "T0pe FMMU y"]
pub struct FMMU_TYPE {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "T0pe FMMU y"]
pub mod fmmu_type;
#[doc = "Activate FMMU 0"]
pub struct FMMU_ACT {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Activate FMMU 0"]
pub mod fmmu_act;
