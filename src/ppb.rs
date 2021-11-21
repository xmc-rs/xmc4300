#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x08],
    #[doc = "0x08 - Auxiliary Control Register"]
    pub actlr: crate::Reg<actlr::ACTLR_SPEC>,
    _reserved1: [u8; 0x04],
    #[doc = "0x10 - SysTick Control and Status Register"]
    pub syst_csr: crate::Reg<syst_csr::SYST_CSR_SPEC>,
    #[doc = "0x14 - SysTick Reload Value Register"]
    pub syst_rvr: crate::Reg<syst_rvr::SYST_RVR_SPEC>,
    #[doc = "0x18 - SysTick Current Value Register"]
    pub syst_cvr: crate::Reg<syst_cvr::SYST_CVR_SPEC>,
    #[doc = "0x1c - SysTick Calibration Value Register r"]
    pub syst_calib: crate::Reg<syst_calib::SYST_CALIB_SPEC>,
    _reserved5: [u8; 0xe0],
    #[doc = "0x100 - Interrupt Set-enable Register 0"]
    pub nvic_iser0: crate::Reg<nvic_iser0::NVIC_ISER0_SPEC>,
    #[doc = "0x104 - Interrupt Set-enable Register 1"]
    pub nvic_iser1: crate::Reg<nvic_iser1::NVIC_ISER1_SPEC>,
    #[doc = "0x108 - Interrupt Set-enable Register 2"]
    pub nvic_iser2: crate::Reg<nvic_iser2::NVIC_ISER2_SPEC>,
    #[doc = "0x10c - Interrupt Set-enable Register 3"]
    pub nvic_iser3: crate::Reg<nvic_iser3::NVIC_ISER3_SPEC>,
    _reserved9: [u8; 0x70],
    #[doc = "0x180 - Interrupt Clear-enable Register 0"]
    pub nvic_icer0: crate::Reg<nvic_icer0::NVIC_ICER0_SPEC>,
    #[doc = "0x184 - Interrupt Clear-enable Register 1"]
    pub nvic_icer1: crate::Reg<nvic_icer1::NVIC_ICER1_SPEC>,
    #[doc = "0x188 - Interrupt Clear-enable Register 2"]
    pub nvic_icer2: crate::Reg<nvic_icer2::NVIC_ICER2_SPEC>,
    #[doc = "0x18c - Interrupt Clear-enable Register 3"]
    pub nvic_icer3: crate::Reg<nvic_icer3::NVIC_ICER3_SPEC>,
    _reserved13: [u8; 0x70],
    #[doc = "0x200 - Interrupt Set-pending Register 0"]
    pub nvic_ispr0: crate::Reg<nvic_ispr0::NVIC_ISPR0_SPEC>,
    #[doc = "0x204 - Interrupt Set-pending Register 1"]
    pub nvic_ispr1: crate::Reg<nvic_ispr1::NVIC_ISPR1_SPEC>,
    #[doc = "0x208 - Interrupt Set-pending Register 2"]
    pub nvic_ispr2: crate::Reg<nvic_ispr2::NVIC_ISPR2_SPEC>,
    #[doc = "0x20c - Interrupt Set-pending Register 3"]
    pub nvic_ispr3: crate::Reg<nvic_ispr3::NVIC_ISPR3_SPEC>,
    _reserved17: [u8; 0x70],
    #[doc = "0x280 - Interrupt Clear-pending Register 0"]
    pub nvic_icpr0: crate::Reg<nvic_icpr0::NVIC_ICPR0_SPEC>,
    #[doc = "0x284 - Interrupt Clear-pending Register 1"]
    pub nvic_icpr1: crate::Reg<nvic_icpr1::NVIC_ICPR1_SPEC>,
    #[doc = "0x288 - Interrupt Clear-pending Register 2"]
    pub nvic_icpr2: crate::Reg<nvic_icpr2::NVIC_ICPR2_SPEC>,
    #[doc = "0x28c - Interrupt Clear-pending Register 3"]
    pub nvic_icpr3: crate::Reg<nvic_icpr3::NVIC_ICPR3_SPEC>,
    _reserved21: [u8; 0x70],
    #[doc = "0x300 - Interrupt Active Bit Register 0"]
    pub nvic_iabr0: crate::Reg<nvic_iabr0::NVIC_IABR0_SPEC>,
    #[doc = "0x304 - Interrupt Active Bit Register 1"]
    pub nvic_iabr1: crate::Reg<nvic_iabr1::NVIC_IABR1_SPEC>,
    #[doc = "0x308 - Interrupt Active Bit Register 2"]
    pub nvic_iabr2: crate::Reg<nvic_iabr2::NVIC_IABR2_SPEC>,
    #[doc = "0x30c - Interrupt Active Bit Register 3"]
    pub nvic_iabr3: crate::Reg<nvic_iabr3::NVIC_IABR3_SPEC>,
    _reserved25: [u8; 0xf0],
    #[doc = "0x400 - Interrupt Priority Register 0"]
    pub nvic_ipr0: crate::Reg<nvic_ipr0::NVIC_IPR0_SPEC>,
    #[doc = "0x404 - Interrupt Priority Register 1"]
    pub nvic_ipr1: crate::Reg<nvic_ipr1::NVIC_IPR1_SPEC>,
    #[doc = "0x408 - Interrupt Priority Register 2"]
    pub nvic_ipr2: crate::Reg<nvic_ipr2::NVIC_IPR2_SPEC>,
    #[doc = "0x40c - Interrupt Priority Register 3"]
    pub nvic_ipr3: crate::Reg<nvic_ipr3::NVIC_IPR3_SPEC>,
    #[doc = "0x410 - Interrupt Priority Register 4"]
    pub nvic_ipr4: crate::Reg<nvic_ipr4::NVIC_IPR4_SPEC>,
    #[doc = "0x414 - Interrupt Priority Register 5"]
    pub nvic_ipr5: crate::Reg<nvic_ipr5::NVIC_IPR5_SPEC>,
    #[doc = "0x418 - Interrupt Priority Register 6"]
    pub nvic_ipr6: crate::Reg<nvic_ipr6::NVIC_IPR6_SPEC>,
    #[doc = "0x41c - Interrupt Priority Register 7"]
    pub nvic_ipr7: crate::Reg<nvic_ipr7::NVIC_IPR7_SPEC>,
    #[doc = "0x420 - Interrupt Priority Register 8"]
    pub nvic_ipr8: crate::Reg<nvic_ipr8::NVIC_IPR8_SPEC>,
    #[doc = "0x424 - Interrupt Priority Register 9"]
    pub nvic_ipr9: crate::Reg<nvic_ipr9::NVIC_IPR9_SPEC>,
    #[doc = "0x428 - Interrupt Priority Register 10"]
    pub nvic_ipr10: crate::Reg<nvic_ipr10::NVIC_IPR10_SPEC>,
    #[doc = "0x42c - Interrupt Priority Register 11"]
    pub nvic_ipr11: crate::Reg<nvic_ipr11::NVIC_IPR11_SPEC>,
    #[doc = "0x430 - Interrupt Priority Register 12"]
    pub nvic_ipr12: crate::Reg<nvic_ipr12::NVIC_IPR12_SPEC>,
    #[doc = "0x434 - Interrupt Priority Register 13"]
    pub nvic_ipr13: crate::Reg<nvic_ipr13::NVIC_IPR13_SPEC>,
    #[doc = "0x438 - Interrupt Priority Register 14"]
    pub nvic_ipr14: crate::Reg<nvic_ipr14::NVIC_IPR14_SPEC>,
    #[doc = "0x43c - Interrupt Priority Register 15"]
    pub nvic_ipr15: crate::Reg<nvic_ipr15::NVIC_IPR15_SPEC>,
    #[doc = "0x440 - Interrupt Priority Register 16"]
    pub nvic_ipr16: crate::Reg<nvic_ipr16::NVIC_IPR16_SPEC>,
    #[doc = "0x444 - Interrupt Priority Register 17"]
    pub nvic_ipr17: crate::Reg<nvic_ipr17::NVIC_IPR17_SPEC>,
    #[doc = "0x448 - Interrupt Priority Register 18"]
    pub nvic_ipr18: crate::Reg<nvic_ipr18::NVIC_IPR18_SPEC>,
    #[doc = "0x44c - Interrupt Priority Register 19"]
    pub nvic_ipr19: crate::Reg<nvic_ipr19::NVIC_IPR19_SPEC>,
    #[doc = "0x450 - Interrupt Priority Register 20"]
    pub nvic_ipr20: crate::Reg<nvic_ipr20::NVIC_IPR20_SPEC>,
    #[doc = "0x454 - Interrupt Priority Register 21"]
    pub nvic_ipr21: crate::Reg<nvic_ipr21::NVIC_IPR21_SPEC>,
    #[doc = "0x458 - Interrupt Priority Register 22"]
    pub nvic_ipr22: crate::Reg<nvic_ipr22::NVIC_IPR22_SPEC>,
    #[doc = "0x45c - Interrupt Priority Register 23"]
    pub nvic_ipr23: crate::Reg<nvic_ipr23::NVIC_IPR23_SPEC>,
    #[doc = "0x460 - Interrupt Priority Register 24"]
    pub nvic_ipr24: crate::Reg<nvic_ipr24::NVIC_IPR24_SPEC>,
    #[doc = "0x464 - Interrupt Priority Register 25"]
    pub nvic_ipr25: crate::Reg<nvic_ipr25::NVIC_IPR25_SPEC>,
    #[doc = "0x468 - Interrupt Priority Register 26"]
    pub nvic_ipr26: crate::Reg<nvic_ipr26::NVIC_IPR26_SPEC>,
    #[doc = "0x46c - Interrupt Priority Register 27"]
    pub nvic_ipr27: crate::Reg<nvic_ipr27::NVIC_IPR27_SPEC>,
    _reserved53: [u8; 0x0890],
    #[doc = "0xd00 - CPUID Base Register"]
    pub cpuid: crate::Reg<cpuid::CPUID_SPEC>,
    #[doc = "0xd04 - Interrupt Control and State Register"]
    pub icsr: crate::Reg<icsr::ICSR_SPEC>,
    #[doc = "0xd08 - Vector Table Offset Register"]
    pub vtor: crate::Reg<vtor::VTOR_SPEC>,
    #[doc = "0xd0c - Application Interrupt and Reset Control Register"]
    pub aircr: crate::Reg<aircr::AIRCR_SPEC>,
    #[doc = "0xd10 - System Control Register"]
    pub scr: crate::Reg<scr::SCR_SPEC>,
    #[doc = "0xd14 - Configuration and Control Register"]
    pub ccr: crate::Reg<ccr::CCR_SPEC>,
    #[doc = "0xd18 - System Handler Priority Register 1"]
    pub shpr1: crate::Reg<shpr1::SHPR1_SPEC>,
    #[doc = "0xd1c - System Handler Priority Register 2"]
    pub shpr2: crate::Reg<shpr2::SHPR2_SPEC>,
    #[doc = "0xd20 - System Handler Priority Register 3"]
    pub shpr3: crate::Reg<shpr3::SHPR3_SPEC>,
    #[doc = "0xd24 - System Handler Control and State Register"]
    pub shcsr: crate::Reg<shcsr::SHCSR_SPEC>,
    #[doc = "0xd28 - Configurable Fault Status Register"]
    pub cfsr: crate::Reg<cfsr::CFSR_SPEC>,
    #[doc = "0xd2c - HardFault Status Register"]
    pub hfsr: crate::Reg<hfsr::HFSR_SPEC>,
    _reserved65: [u8; 0x04],
    #[doc = "0xd34 - MemManage Fault Address Register"]
    pub mmfar: crate::Reg<mmfar::MMFAR_SPEC>,
    #[doc = "0xd38 - BusFault Address Register"]
    pub bfar: crate::Reg<bfar::BFAR_SPEC>,
    #[doc = "0xd3c - Auxiliary Fault Status Register"]
    pub afsr: crate::Reg<afsr::AFSR_SPEC>,
    _reserved68: [u8; 0x48],
    #[doc = "0xd88 - Coprocessor Access Control Register"]
    pub cpacr: crate::Reg<cpacr::CPACR_SPEC>,
    _reserved69: [u8; 0x04],
    #[doc = "0xd90 - MPU Type Register"]
    pub mpu_type: crate::Reg<mpu_type::MPU_TYPE_SPEC>,
    #[doc = "0xd94 - MPU Control Register"]
    pub mpu_ctrl: crate::Reg<mpu_ctrl::MPU_CTRL_SPEC>,
    #[doc = "0xd98 - MPU Region Number Register"]
    pub mpu_rnr: crate::Reg<mpu_rnr::MPU_RNR_SPEC>,
    #[doc = "0xd9c - MPU Region Base Address Register"]
    pub mpu_rbar: crate::Reg<mpu_rbar::MPU_RBAR_SPEC>,
    #[doc = "0xda0 - MPU Region Attribute and Size Register"]
    pub mpu_rasr: crate::Reg<mpu_rasr::MPU_RASR_SPEC>,
    #[doc = "0xda4 - MPU Region Base Address Register A1"]
    pub mpu_rbar_a1: crate::Reg<mpu_rbar_a1::MPU_RBAR_A1_SPEC>,
    #[doc = "0xda8 - MPU Region Attribute and Size Register A1"]
    pub mpu_rasr_a1: crate::Reg<mpu_rasr_a1::MPU_RASR_A1_SPEC>,
    #[doc = "0xdac - MPU Region Base Address Register A2"]
    pub mpu_rbar_a2: crate::Reg<mpu_rbar_a2::MPU_RBAR_A2_SPEC>,
    #[doc = "0xdb0 - MPU Region Attribute and Size Register A2"]
    pub mpu_rasr_a2: crate::Reg<mpu_rasr_a2::MPU_RASR_A2_SPEC>,
    #[doc = "0xdb4 - MPU Region Base Address Register A3"]
    pub mpu_rbar_a3: crate::Reg<mpu_rbar_a3::MPU_RBAR_A3_SPEC>,
    #[doc = "0xdb8 - MPU Region Attribute and Size Register A3"]
    pub mpu_rasr_a3: crate::Reg<mpu_rasr_a3::MPU_RASR_A3_SPEC>,
    _reserved80: [u8; 0x0144],
    #[doc = "0xf00 - Software Trigger Interrupt Register"]
    pub stir: crate::Reg<stir::STIR_SPEC>,
    _reserved81: [u8; 0x30],
    #[doc = "0xf34 - Floating-point Context Control Register"]
    pub fpccr: crate::Reg<fpccr::FPCCR_SPEC>,
    #[doc = "0xf38 - Floating-point Context Address Register"]
    pub fpcar: crate::Reg<fpcar::FPCAR_SPEC>,
    #[doc = "0xf3c - Floating-point Default Status Control Register"]
    pub fpdscr: crate::Reg<fpdscr::FPDSCR_SPEC>,
}
#[doc = "ACTLR register accessor: an alias for `Reg<ACTLR_SPEC>`"]
pub type ACTLR = crate::Reg<actlr::ACTLR_SPEC>;
#[doc = "Auxiliary Control Register"]
pub mod actlr;
#[doc = "SYST_CSR register accessor: an alias for `Reg<SYST_CSR_SPEC>`"]
pub type SYST_CSR = crate::Reg<syst_csr::SYST_CSR_SPEC>;
#[doc = "SysTick Control and Status Register"]
pub mod syst_csr;
#[doc = "SYST_RVR register accessor: an alias for `Reg<SYST_RVR_SPEC>`"]
pub type SYST_RVR = crate::Reg<syst_rvr::SYST_RVR_SPEC>;
#[doc = "SysTick Reload Value Register"]
pub mod syst_rvr;
#[doc = "SYST_CVR register accessor: an alias for `Reg<SYST_CVR_SPEC>`"]
pub type SYST_CVR = crate::Reg<syst_cvr::SYST_CVR_SPEC>;
#[doc = "SysTick Current Value Register"]
pub mod syst_cvr;
#[doc = "SYST_CALIB register accessor: an alias for `Reg<SYST_CALIB_SPEC>`"]
pub type SYST_CALIB = crate::Reg<syst_calib::SYST_CALIB_SPEC>;
#[doc = "SysTick Calibration Value Register r"]
pub mod syst_calib;
#[doc = "NVIC_ISER0 register accessor: an alias for `Reg<NVIC_ISER0_SPEC>`"]
pub type NVIC_ISER0 = crate::Reg<nvic_iser0::NVIC_ISER0_SPEC>;
#[doc = "Interrupt Set-enable Register 0"]
pub mod nvic_iser0;
#[doc = "NVIC_ISER1 register accessor: an alias for `Reg<NVIC_ISER1_SPEC>`"]
pub type NVIC_ISER1 = crate::Reg<nvic_iser1::NVIC_ISER1_SPEC>;
#[doc = "Interrupt Set-enable Register 1"]
pub mod nvic_iser1;
#[doc = "NVIC_ISER2 register accessor: an alias for `Reg<NVIC_ISER2_SPEC>`"]
pub type NVIC_ISER2 = crate::Reg<nvic_iser2::NVIC_ISER2_SPEC>;
#[doc = "Interrupt Set-enable Register 2"]
pub mod nvic_iser2;
#[doc = "NVIC_ISER3 register accessor: an alias for `Reg<NVIC_ISER3_SPEC>`"]
pub type NVIC_ISER3 = crate::Reg<nvic_iser3::NVIC_ISER3_SPEC>;
#[doc = "Interrupt Set-enable Register 3"]
pub mod nvic_iser3;
#[doc = "NVIC_ICER0 register accessor: an alias for `Reg<NVIC_ICER0_SPEC>`"]
pub type NVIC_ICER0 = crate::Reg<nvic_icer0::NVIC_ICER0_SPEC>;
#[doc = "Interrupt Clear-enable Register 0"]
pub mod nvic_icer0;
#[doc = "NVIC_ICER1 register accessor: an alias for `Reg<NVIC_ICER1_SPEC>`"]
pub type NVIC_ICER1 = crate::Reg<nvic_icer1::NVIC_ICER1_SPEC>;
#[doc = "Interrupt Clear-enable Register 1"]
pub mod nvic_icer1;
#[doc = "NVIC_ICER2 register accessor: an alias for `Reg<NVIC_ICER2_SPEC>`"]
pub type NVIC_ICER2 = crate::Reg<nvic_icer2::NVIC_ICER2_SPEC>;
#[doc = "Interrupt Clear-enable Register 2"]
pub mod nvic_icer2;
#[doc = "NVIC_ICER3 register accessor: an alias for `Reg<NVIC_ICER3_SPEC>`"]
pub type NVIC_ICER3 = crate::Reg<nvic_icer3::NVIC_ICER3_SPEC>;
#[doc = "Interrupt Clear-enable Register 3"]
pub mod nvic_icer3;
#[doc = "NVIC_ISPR0 register accessor: an alias for `Reg<NVIC_ISPR0_SPEC>`"]
pub type NVIC_ISPR0 = crate::Reg<nvic_ispr0::NVIC_ISPR0_SPEC>;
#[doc = "Interrupt Set-pending Register 0"]
pub mod nvic_ispr0;
#[doc = "NVIC_ISPR1 register accessor: an alias for `Reg<NVIC_ISPR1_SPEC>`"]
pub type NVIC_ISPR1 = crate::Reg<nvic_ispr1::NVIC_ISPR1_SPEC>;
#[doc = "Interrupt Set-pending Register 1"]
pub mod nvic_ispr1;
#[doc = "NVIC_ISPR2 register accessor: an alias for `Reg<NVIC_ISPR2_SPEC>`"]
pub type NVIC_ISPR2 = crate::Reg<nvic_ispr2::NVIC_ISPR2_SPEC>;
#[doc = "Interrupt Set-pending Register 2"]
pub mod nvic_ispr2;
#[doc = "NVIC_ISPR3 register accessor: an alias for `Reg<NVIC_ISPR3_SPEC>`"]
pub type NVIC_ISPR3 = crate::Reg<nvic_ispr3::NVIC_ISPR3_SPEC>;
#[doc = "Interrupt Set-pending Register 3"]
pub mod nvic_ispr3;
#[doc = "NVIC_ICPR0 register accessor: an alias for `Reg<NVIC_ICPR0_SPEC>`"]
pub type NVIC_ICPR0 = crate::Reg<nvic_icpr0::NVIC_ICPR0_SPEC>;
#[doc = "Interrupt Clear-pending Register 0"]
pub mod nvic_icpr0;
#[doc = "NVIC_ICPR1 register accessor: an alias for `Reg<NVIC_ICPR1_SPEC>`"]
pub type NVIC_ICPR1 = crate::Reg<nvic_icpr1::NVIC_ICPR1_SPEC>;
#[doc = "Interrupt Clear-pending Register 1"]
pub mod nvic_icpr1;
#[doc = "NVIC_ICPR2 register accessor: an alias for `Reg<NVIC_ICPR2_SPEC>`"]
pub type NVIC_ICPR2 = crate::Reg<nvic_icpr2::NVIC_ICPR2_SPEC>;
#[doc = "Interrupt Clear-pending Register 2"]
pub mod nvic_icpr2;
#[doc = "NVIC_ICPR3 register accessor: an alias for `Reg<NVIC_ICPR3_SPEC>`"]
pub type NVIC_ICPR3 = crate::Reg<nvic_icpr3::NVIC_ICPR3_SPEC>;
#[doc = "Interrupt Clear-pending Register 3"]
pub mod nvic_icpr3;
#[doc = "NVIC_IABR0 register accessor: an alias for `Reg<NVIC_IABR0_SPEC>`"]
pub type NVIC_IABR0 = crate::Reg<nvic_iabr0::NVIC_IABR0_SPEC>;
#[doc = "Interrupt Active Bit Register 0"]
pub mod nvic_iabr0;
#[doc = "NVIC_IABR1 register accessor: an alias for `Reg<NVIC_IABR1_SPEC>`"]
pub type NVIC_IABR1 = crate::Reg<nvic_iabr1::NVIC_IABR1_SPEC>;
#[doc = "Interrupt Active Bit Register 1"]
pub mod nvic_iabr1;
#[doc = "NVIC_IABR2 register accessor: an alias for `Reg<NVIC_IABR2_SPEC>`"]
pub type NVIC_IABR2 = crate::Reg<nvic_iabr2::NVIC_IABR2_SPEC>;
#[doc = "Interrupt Active Bit Register 2"]
pub mod nvic_iabr2;
#[doc = "NVIC_IABR3 register accessor: an alias for `Reg<NVIC_IABR3_SPEC>`"]
pub type NVIC_IABR3 = crate::Reg<nvic_iabr3::NVIC_IABR3_SPEC>;
#[doc = "Interrupt Active Bit Register 3"]
pub mod nvic_iabr3;
#[doc = "NVIC_IPR0 register accessor: an alias for `Reg<NVIC_IPR0_SPEC>`"]
pub type NVIC_IPR0 = crate::Reg<nvic_ipr0::NVIC_IPR0_SPEC>;
#[doc = "Interrupt Priority Register 0"]
pub mod nvic_ipr0;
#[doc = "NVIC_IPR1 register accessor: an alias for `Reg<NVIC_IPR1_SPEC>`"]
pub type NVIC_IPR1 = crate::Reg<nvic_ipr1::NVIC_IPR1_SPEC>;
#[doc = "Interrupt Priority Register 1"]
pub mod nvic_ipr1;
#[doc = "NVIC_IPR2 register accessor: an alias for `Reg<NVIC_IPR2_SPEC>`"]
pub type NVIC_IPR2 = crate::Reg<nvic_ipr2::NVIC_IPR2_SPEC>;
#[doc = "Interrupt Priority Register 2"]
pub mod nvic_ipr2;
#[doc = "NVIC_IPR3 register accessor: an alias for `Reg<NVIC_IPR3_SPEC>`"]
pub type NVIC_IPR3 = crate::Reg<nvic_ipr3::NVIC_IPR3_SPEC>;
#[doc = "Interrupt Priority Register 3"]
pub mod nvic_ipr3;
#[doc = "NVIC_IPR4 register accessor: an alias for `Reg<NVIC_IPR4_SPEC>`"]
pub type NVIC_IPR4 = crate::Reg<nvic_ipr4::NVIC_IPR4_SPEC>;
#[doc = "Interrupt Priority Register 4"]
pub mod nvic_ipr4;
#[doc = "NVIC_IPR5 register accessor: an alias for `Reg<NVIC_IPR5_SPEC>`"]
pub type NVIC_IPR5 = crate::Reg<nvic_ipr5::NVIC_IPR5_SPEC>;
#[doc = "Interrupt Priority Register 5"]
pub mod nvic_ipr5;
#[doc = "NVIC_IPR6 register accessor: an alias for `Reg<NVIC_IPR6_SPEC>`"]
pub type NVIC_IPR6 = crate::Reg<nvic_ipr6::NVIC_IPR6_SPEC>;
#[doc = "Interrupt Priority Register 6"]
pub mod nvic_ipr6;
#[doc = "NVIC_IPR7 register accessor: an alias for `Reg<NVIC_IPR7_SPEC>`"]
pub type NVIC_IPR7 = crate::Reg<nvic_ipr7::NVIC_IPR7_SPEC>;
#[doc = "Interrupt Priority Register 7"]
pub mod nvic_ipr7;
#[doc = "NVIC_IPR8 register accessor: an alias for `Reg<NVIC_IPR8_SPEC>`"]
pub type NVIC_IPR8 = crate::Reg<nvic_ipr8::NVIC_IPR8_SPEC>;
#[doc = "Interrupt Priority Register 8"]
pub mod nvic_ipr8;
#[doc = "NVIC_IPR9 register accessor: an alias for `Reg<NVIC_IPR9_SPEC>`"]
pub type NVIC_IPR9 = crate::Reg<nvic_ipr9::NVIC_IPR9_SPEC>;
#[doc = "Interrupt Priority Register 9"]
pub mod nvic_ipr9;
#[doc = "NVIC_IPR10 register accessor: an alias for `Reg<NVIC_IPR10_SPEC>`"]
pub type NVIC_IPR10 = crate::Reg<nvic_ipr10::NVIC_IPR10_SPEC>;
#[doc = "Interrupt Priority Register 10"]
pub mod nvic_ipr10;
#[doc = "NVIC_IPR11 register accessor: an alias for `Reg<NVIC_IPR11_SPEC>`"]
pub type NVIC_IPR11 = crate::Reg<nvic_ipr11::NVIC_IPR11_SPEC>;
#[doc = "Interrupt Priority Register 11"]
pub mod nvic_ipr11;
#[doc = "NVIC_IPR12 register accessor: an alias for `Reg<NVIC_IPR12_SPEC>`"]
pub type NVIC_IPR12 = crate::Reg<nvic_ipr12::NVIC_IPR12_SPEC>;
#[doc = "Interrupt Priority Register 12"]
pub mod nvic_ipr12;
#[doc = "NVIC_IPR13 register accessor: an alias for `Reg<NVIC_IPR13_SPEC>`"]
pub type NVIC_IPR13 = crate::Reg<nvic_ipr13::NVIC_IPR13_SPEC>;
#[doc = "Interrupt Priority Register 13"]
pub mod nvic_ipr13;
#[doc = "NVIC_IPR14 register accessor: an alias for `Reg<NVIC_IPR14_SPEC>`"]
pub type NVIC_IPR14 = crate::Reg<nvic_ipr14::NVIC_IPR14_SPEC>;
#[doc = "Interrupt Priority Register 14"]
pub mod nvic_ipr14;
#[doc = "NVIC_IPR15 register accessor: an alias for `Reg<NVIC_IPR15_SPEC>`"]
pub type NVIC_IPR15 = crate::Reg<nvic_ipr15::NVIC_IPR15_SPEC>;
#[doc = "Interrupt Priority Register 15"]
pub mod nvic_ipr15;
#[doc = "NVIC_IPR16 register accessor: an alias for `Reg<NVIC_IPR16_SPEC>`"]
pub type NVIC_IPR16 = crate::Reg<nvic_ipr16::NVIC_IPR16_SPEC>;
#[doc = "Interrupt Priority Register 16"]
pub mod nvic_ipr16;
#[doc = "NVIC_IPR17 register accessor: an alias for `Reg<NVIC_IPR17_SPEC>`"]
pub type NVIC_IPR17 = crate::Reg<nvic_ipr17::NVIC_IPR17_SPEC>;
#[doc = "Interrupt Priority Register 17"]
pub mod nvic_ipr17;
#[doc = "NVIC_IPR18 register accessor: an alias for `Reg<NVIC_IPR18_SPEC>`"]
pub type NVIC_IPR18 = crate::Reg<nvic_ipr18::NVIC_IPR18_SPEC>;
#[doc = "Interrupt Priority Register 18"]
pub mod nvic_ipr18;
#[doc = "NVIC_IPR19 register accessor: an alias for `Reg<NVIC_IPR19_SPEC>`"]
pub type NVIC_IPR19 = crate::Reg<nvic_ipr19::NVIC_IPR19_SPEC>;
#[doc = "Interrupt Priority Register 19"]
pub mod nvic_ipr19;
#[doc = "NVIC_IPR20 register accessor: an alias for `Reg<NVIC_IPR20_SPEC>`"]
pub type NVIC_IPR20 = crate::Reg<nvic_ipr20::NVIC_IPR20_SPEC>;
#[doc = "Interrupt Priority Register 20"]
pub mod nvic_ipr20;
#[doc = "NVIC_IPR21 register accessor: an alias for `Reg<NVIC_IPR21_SPEC>`"]
pub type NVIC_IPR21 = crate::Reg<nvic_ipr21::NVIC_IPR21_SPEC>;
#[doc = "Interrupt Priority Register 21"]
pub mod nvic_ipr21;
#[doc = "NVIC_IPR22 register accessor: an alias for `Reg<NVIC_IPR22_SPEC>`"]
pub type NVIC_IPR22 = crate::Reg<nvic_ipr22::NVIC_IPR22_SPEC>;
#[doc = "Interrupt Priority Register 22"]
pub mod nvic_ipr22;
#[doc = "NVIC_IPR23 register accessor: an alias for `Reg<NVIC_IPR23_SPEC>`"]
pub type NVIC_IPR23 = crate::Reg<nvic_ipr23::NVIC_IPR23_SPEC>;
#[doc = "Interrupt Priority Register 23"]
pub mod nvic_ipr23;
#[doc = "NVIC_IPR24 register accessor: an alias for `Reg<NVIC_IPR24_SPEC>`"]
pub type NVIC_IPR24 = crate::Reg<nvic_ipr24::NVIC_IPR24_SPEC>;
#[doc = "Interrupt Priority Register 24"]
pub mod nvic_ipr24;
#[doc = "NVIC_IPR25 register accessor: an alias for `Reg<NVIC_IPR25_SPEC>`"]
pub type NVIC_IPR25 = crate::Reg<nvic_ipr25::NVIC_IPR25_SPEC>;
#[doc = "Interrupt Priority Register 25"]
pub mod nvic_ipr25;
#[doc = "NVIC_IPR26 register accessor: an alias for `Reg<NVIC_IPR26_SPEC>`"]
pub type NVIC_IPR26 = crate::Reg<nvic_ipr26::NVIC_IPR26_SPEC>;
#[doc = "Interrupt Priority Register 26"]
pub mod nvic_ipr26;
#[doc = "NVIC_IPR27 register accessor: an alias for `Reg<NVIC_IPR27_SPEC>`"]
pub type NVIC_IPR27 = crate::Reg<nvic_ipr27::NVIC_IPR27_SPEC>;
#[doc = "Interrupt Priority Register 27"]
pub mod nvic_ipr27;
#[doc = "CPUID register accessor: an alias for `Reg<CPUID_SPEC>`"]
pub type CPUID = crate::Reg<cpuid::CPUID_SPEC>;
#[doc = "CPUID Base Register"]
pub mod cpuid;
#[doc = "ICSR register accessor: an alias for `Reg<ICSR_SPEC>`"]
pub type ICSR = crate::Reg<icsr::ICSR_SPEC>;
#[doc = "Interrupt Control and State Register"]
pub mod icsr;
#[doc = "VTOR register accessor: an alias for `Reg<VTOR_SPEC>`"]
pub type VTOR = crate::Reg<vtor::VTOR_SPEC>;
#[doc = "Vector Table Offset Register"]
pub mod vtor;
#[doc = "AIRCR register accessor: an alias for `Reg<AIRCR_SPEC>`"]
pub type AIRCR = crate::Reg<aircr::AIRCR_SPEC>;
#[doc = "Application Interrupt and Reset Control Register"]
pub mod aircr;
#[doc = "SCR register accessor: an alias for `Reg<SCR_SPEC>`"]
pub type SCR = crate::Reg<scr::SCR_SPEC>;
#[doc = "System Control Register"]
pub mod scr;
#[doc = "CCR register accessor: an alias for `Reg<CCR_SPEC>`"]
pub type CCR = crate::Reg<ccr::CCR_SPEC>;
#[doc = "Configuration and Control Register"]
pub mod ccr;
#[doc = "SHPR1 register accessor: an alias for `Reg<SHPR1_SPEC>`"]
pub type SHPR1 = crate::Reg<shpr1::SHPR1_SPEC>;
#[doc = "System Handler Priority Register 1"]
pub mod shpr1;
#[doc = "SHPR2 register accessor: an alias for `Reg<SHPR2_SPEC>`"]
pub type SHPR2 = crate::Reg<shpr2::SHPR2_SPEC>;
#[doc = "System Handler Priority Register 2"]
pub mod shpr2;
#[doc = "SHPR3 register accessor: an alias for `Reg<SHPR3_SPEC>`"]
pub type SHPR3 = crate::Reg<shpr3::SHPR3_SPEC>;
#[doc = "System Handler Priority Register 3"]
pub mod shpr3;
#[doc = "SHCSR register accessor: an alias for `Reg<SHCSR_SPEC>`"]
pub type SHCSR = crate::Reg<shcsr::SHCSR_SPEC>;
#[doc = "System Handler Control and State Register"]
pub mod shcsr;
#[doc = "CFSR register accessor: an alias for `Reg<CFSR_SPEC>`"]
pub type CFSR = crate::Reg<cfsr::CFSR_SPEC>;
#[doc = "Configurable Fault Status Register"]
pub mod cfsr;
#[doc = "HFSR register accessor: an alias for `Reg<HFSR_SPEC>`"]
pub type HFSR = crate::Reg<hfsr::HFSR_SPEC>;
#[doc = "HardFault Status Register"]
pub mod hfsr;
#[doc = "MMFAR register accessor: an alias for `Reg<MMFAR_SPEC>`"]
pub type MMFAR = crate::Reg<mmfar::MMFAR_SPEC>;
#[doc = "MemManage Fault Address Register"]
pub mod mmfar;
#[doc = "BFAR register accessor: an alias for `Reg<BFAR_SPEC>`"]
pub type BFAR = crate::Reg<bfar::BFAR_SPEC>;
#[doc = "BusFault Address Register"]
pub mod bfar;
#[doc = "AFSR register accessor: an alias for `Reg<AFSR_SPEC>`"]
pub type AFSR = crate::Reg<afsr::AFSR_SPEC>;
#[doc = "Auxiliary Fault Status Register"]
pub mod afsr;
#[doc = "CPACR register accessor: an alias for `Reg<CPACR_SPEC>`"]
pub type CPACR = crate::Reg<cpacr::CPACR_SPEC>;
#[doc = "Coprocessor Access Control Register"]
pub mod cpacr;
#[doc = "MPU_TYPE register accessor: an alias for `Reg<MPU_TYPE_SPEC>`"]
pub type MPU_TYPE = crate::Reg<mpu_type::MPU_TYPE_SPEC>;
#[doc = "MPU Type Register"]
pub mod mpu_type;
#[doc = "MPU_CTRL register accessor: an alias for `Reg<MPU_CTRL_SPEC>`"]
pub type MPU_CTRL = crate::Reg<mpu_ctrl::MPU_CTRL_SPEC>;
#[doc = "MPU Control Register"]
pub mod mpu_ctrl;
#[doc = "MPU_RNR register accessor: an alias for `Reg<MPU_RNR_SPEC>`"]
pub type MPU_RNR = crate::Reg<mpu_rnr::MPU_RNR_SPEC>;
#[doc = "MPU Region Number Register"]
pub mod mpu_rnr;
#[doc = "MPU_RBAR register accessor: an alias for `Reg<MPU_RBAR_SPEC>`"]
pub type MPU_RBAR = crate::Reg<mpu_rbar::MPU_RBAR_SPEC>;
#[doc = "MPU Region Base Address Register"]
pub mod mpu_rbar;
#[doc = "MPU_RASR register accessor: an alias for `Reg<MPU_RASR_SPEC>`"]
pub type MPU_RASR = crate::Reg<mpu_rasr::MPU_RASR_SPEC>;
#[doc = "MPU Region Attribute and Size Register"]
pub mod mpu_rasr;
#[doc = "MPU_RBAR_A1 register accessor: an alias for `Reg<MPU_RBAR_A1_SPEC>`"]
pub type MPU_RBAR_A1 = crate::Reg<mpu_rbar_a1::MPU_RBAR_A1_SPEC>;
#[doc = "MPU Region Base Address Register A1"]
pub mod mpu_rbar_a1;
#[doc = "MPU_RASR_A1 register accessor: an alias for `Reg<MPU_RASR_A1_SPEC>`"]
pub type MPU_RASR_A1 = crate::Reg<mpu_rasr_a1::MPU_RASR_A1_SPEC>;
#[doc = "MPU Region Attribute and Size Register A1"]
pub mod mpu_rasr_a1;
#[doc = "MPU_RBAR_A2 register accessor: an alias for `Reg<MPU_RBAR_A2_SPEC>`"]
pub type MPU_RBAR_A2 = crate::Reg<mpu_rbar_a2::MPU_RBAR_A2_SPEC>;
#[doc = "MPU Region Base Address Register A2"]
pub mod mpu_rbar_a2;
#[doc = "MPU_RASR_A2 register accessor: an alias for `Reg<MPU_RASR_A2_SPEC>`"]
pub type MPU_RASR_A2 = crate::Reg<mpu_rasr_a2::MPU_RASR_A2_SPEC>;
#[doc = "MPU Region Attribute and Size Register A2"]
pub mod mpu_rasr_a2;
#[doc = "MPU_RBAR_A3 register accessor: an alias for `Reg<MPU_RBAR_A3_SPEC>`"]
pub type MPU_RBAR_A3 = crate::Reg<mpu_rbar_a3::MPU_RBAR_A3_SPEC>;
#[doc = "MPU Region Base Address Register A3"]
pub mod mpu_rbar_a3;
#[doc = "MPU_RASR_A3 register accessor: an alias for `Reg<MPU_RASR_A3_SPEC>`"]
pub type MPU_RASR_A3 = crate::Reg<mpu_rasr_a3::MPU_RASR_A3_SPEC>;
#[doc = "MPU Region Attribute and Size Register A3"]
pub mod mpu_rasr_a3;
#[doc = "STIR register accessor: an alias for `Reg<STIR_SPEC>`"]
pub type STIR = crate::Reg<stir::STIR_SPEC>;
#[doc = "Software Trigger Interrupt Register"]
pub mod stir;
#[doc = "FPCCR register accessor: an alias for `Reg<FPCCR_SPEC>`"]
pub type FPCCR = crate::Reg<fpccr::FPCCR_SPEC>;
#[doc = "Floating-point Context Control Register"]
pub mod fpccr;
#[doc = "FPCAR register accessor: an alias for `Reg<FPCAR_SPEC>`"]
pub type FPCAR = crate::Reg<fpcar::FPCAR_SPEC>;
#[doc = "Floating-point Context Address Register"]
pub mod fpcar;
#[doc = "FPDSCR register accessor: an alias for `Reg<FPDSCR_SPEC>`"]
pub type FPDSCR = crate::Reg<fpdscr::FPDSCR_SPEC>;
#[doc = "Floating-point Default Status Control Register"]
pub mod fpdscr;
