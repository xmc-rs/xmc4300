#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 8usize],
    #[doc = "0x08 - Auxiliary Control Register"]
    pub actlr: ACTLR,
    _reserved1: [u8; 4usize],
    #[doc = "0x10 - SysTick Control and Status Register"]
    pub syst_csr: SYST_CSR,
    #[doc = "0x14 - SysTick Reload Value Register"]
    pub syst_rvr: SYST_RVR,
    #[doc = "0x18 - SysTick Current Value Register"]
    pub syst_cvr: SYST_CVR,
    #[doc = "0x1c - SysTick Calibration Value Register r"]
    pub syst_calib: SYST_CALIB,
    _reserved2: [u8; 224usize],
    #[doc = "0x100 - Interrupt Set-enable Register 0"]
    pub nvic_iser0: NVIC_ISER0,
    #[doc = "0x104 - Interrupt Set-enable Register 1"]
    pub nvic_iser1: NVIC_ISER1,
    #[doc = "0x108 - Interrupt Set-enable Register 2"]
    pub nvic_iser2: NVIC_ISER2,
    #[doc = "0x10c - Interrupt Set-enable Register 3"]
    pub nvic_iser3: NVIC_ISER3,
    _reserved3: [u8; 112usize],
    #[doc = "0x180 - Interrupt Clear-enable Register 0"]
    pub nvic_icer0: NVIC_ICER0,
    #[doc = "0x184 - Interrupt Clear-enable Register 1"]
    pub nvic_icer1: NVIC_ICER1,
    #[doc = "0x188 - Interrupt Clear-enable Register 2"]
    pub nvic_icer2: NVIC_ICER2,
    #[doc = "0x18c - Interrupt Clear-enable Register 3"]
    pub nvic_icer3: NVIC_ICER3,
    _reserved4: [u8; 112usize],
    #[doc = "0x200 - Interrupt Set-pending Register 0"]
    pub nvic_ispr0: NVIC_ISPR0,
    #[doc = "0x204 - Interrupt Set-pending Register 1"]
    pub nvic_ispr1: NVIC_ISPR1,
    #[doc = "0x208 - Interrupt Set-pending Register 2"]
    pub nvic_ispr2: NVIC_ISPR2,
    #[doc = "0x20c - Interrupt Set-pending Register 3"]
    pub nvic_ispr3: NVIC_ISPR3,
    _reserved5: [u8; 112usize],
    #[doc = "0x280 - Interrupt Clear-pending Register 0"]
    pub nvic_icpr0: NVIC_ICPR0,
    #[doc = "0x284 - Interrupt Clear-pending Register 1"]
    pub nvic_icpr1: NVIC_ICPR1,
    #[doc = "0x288 - Interrupt Clear-pending Register 2"]
    pub nvic_icpr2: NVIC_ICPR2,
    #[doc = "0x28c - Interrupt Clear-pending Register 3"]
    pub nvic_icpr3: NVIC_ICPR3,
    _reserved6: [u8; 112usize],
    #[doc = "0x300 - Interrupt Active Bit Register 0"]
    pub nvic_iabr0: NVIC_IABR0,
    #[doc = "0x304 - Interrupt Active Bit Register 1"]
    pub nvic_iabr1: NVIC_IABR1,
    #[doc = "0x308 - Interrupt Active Bit Register 2"]
    pub nvic_iabr2: NVIC_IABR2,
    #[doc = "0x30c - Interrupt Active Bit Register 3"]
    pub nvic_iabr3: NVIC_IABR3,
    _reserved7: [u8; 240usize],
    #[doc = "0x400 - Interrupt Priority Register 0"]
    pub nvic_ipr0: NVIC_IPR0,
    #[doc = "0x404 - Interrupt Priority Register 1"]
    pub nvic_ipr1: NVIC_IPR1,
    #[doc = "0x408 - Interrupt Priority Register 2"]
    pub nvic_ipr2: NVIC_IPR2,
    #[doc = "0x40c - Interrupt Priority Register 3"]
    pub nvic_ipr3: NVIC_IPR3,
    #[doc = "0x410 - Interrupt Priority Register 4"]
    pub nvic_ipr4: NVIC_IPR4,
    #[doc = "0x414 - Interrupt Priority Register 5"]
    pub nvic_ipr5: NVIC_IPR5,
    #[doc = "0x418 - Interrupt Priority Register 6"]
    pub nvic_ipr6: NVIC_IPR6,
    #[doc = "0x41c - Interrupt Priority Register 7"]
    pub nvic_ipr7: NVIC_IPR7,
    #[doc = "0x420 - Interrupt Priority Register 8"]
    pub nvic_ipr8: NVIC_IPR8,
    #[doc = "0x424 - Interrupt Priority Register 9"]
    pub nvic_ipr9: NVIC_IPR9,
    #[doc = "0x428 - Interrupt Priority Register 10"]
    pub nvic_ipr10: NVIC_IPR10,
    #[doc = "0x42c - Interrupt Priority Register 11"]
    pub nvic_ipr11: NVIC_IPR11,
    #[doc = "0x430 - Interrupt Priority Register 12"]
    pub nvic_ipr12: NVIC_IPR12,
    #[doc = "0x434 - Interrupt Priority Register 13"]
    pub nvic_ipr13: NVIC_IPR13,
    #[doc = "0x438 - Interrupt Priority Register 14"]
    pub nvic_ipr14: NVIC_IPR14,
    #[doc = "0x43c - Interrupt Priority Register 15"]
    pub nvic_ipr15: NVIC_IPR15,
    #[doc = "0x440 - Interrupt Priority Register 16"]
    pub nvic_ipr16: NVIC_IPR16,
    #[doc = "0x444 - Interrupt Priority Register 17"]
    pub nvic_ipr17: NVIC_IPR17,
    #[doc = "0x448 - Interrupt Priority Register 18"]
    pub nvic_ipr18: NVIC_IPR18,
    #[doc = "0x44c - Interrupt Priority Register 19"]
    pub nvic_ipr19: NVIC_IPR19,
    #[doc = "0x450 - Interrupt Priority Register 20"]
    pub nvic_ipr20: NVIC_IPR20,
    #[doc = "0x454 - Interrupt Priority Register 21"]
    pub nvic_ipr21: NVIC_IPR21,
    #[doc = "0x458 - Interrupt Priority Register 22"]
    pub nvic_ipr22: NVIC_IPR22,
    #[doc = "0x45c - Interrupt Priority Register 23"]
    pub nvic_ipr23: NVIC_IPR23,
    #[doc = "0x460 - Interrupt Priority Register 24"]
    pub nvic_ipr24: NVIC_IPR24,
    #[doc = "0x464 - Interrupt Priority Register 25"]
    pub nvic_ipr25: NVIC_IPR25,
    #[doc = "0x468 - Interrupt Priority Register 26"]
    pub nvic_ipr26: NVIC_IPR26,
    #[doc = "0x46c - Interrupt Priority Register 27"]
    pub nvic_ipr27: NVIC_IPR27,
    _reserved8: [u8; 2192usize],
    #[doc = "0xd00 - CPUID Base Register"]
    pub cpuid: CPUID,
    #[doc = "0xd04 - Interrupt Control and State Register"]
    pub icsr: ICSR,
    #[doc = "0xd08 - Vector Table Offset Register"]
    pub vtor: VTOR,
    #[doc = "0xd0c - Application Interrupt and Reset Control Register"]
    pub aircr: AIRCR,
    #[doc = "0xd10 - System Control Register"]
    pub scr: SCR,
    #[doc = "0xd14 - Configuration and Control Register"]
    pub ccr: CCR,
    #[doc = "0xd18 - System Handler Priority Register 1"]
    pub shpr1: SHPR1,
    #[doc = "0xd1c - System Handler Priority Register 2"]
    pub shpr2: SHPR2,
    #[doc = "0xd20 - System Handler Priority Register 3"]
    pub shpr3: SHPR3,
    #[doc = "0xd24 - System Handler Control and State Register"]
    pub shcsr: SHCSR,
    #[doc = "0xd28 - Configurable Fault Status Register"]
    pub cfsr: CFSR,
    #[doc = "0xd2c - HardFault Status Register"]
    pub hfsr: HFSR,
    _reserved9: [u8; 4usize],
    #[doc = "0xd34 - MemManage Fault Address Register"]
    pub mmfar: MMFAR,
    #[doc = "0xd38 - BusFault Address Register"]
    pub bfar: BFAR,
    #[doc = "0xd3c - Auxiliary Fault Status Register"]
    pub afsr: AFSR,
    _reserved10: [u8; 72usize],
    #[doc = "0xd88 - Coprocessor Access Control Register"]
    pub cpacr: CPACR,
    _reserved11: [u8; 4usize],
    #[doc = "0xd90 - MPU Type Register"]
    pub mpu_type: MPU_TYPE,
    #[doc = "0xd94 - MPU Control Register"]
    pub mpu_ctrl: MPU_CTRL,
    #[doc = "0xd98 - MPU Region Number Register"]
    pub mpu_rnr: MPU_RNR,
    #[doc = "0xd9c - MPU Region Base Address Register"]
    pub mpu_rbar: MPU_RBAR,
    #[doc = "0xda0 - MPU Region Attribute and Size Register"]
    pub mpu_rasr: MPU_RASR,
    #[doc = "0xda4 - MPU Region Base Address Register A1"]
    pub mpu_rbar_a1: MPU_RBAR_A1,
    #[doc = "0xda8 - MPU Region Attribute and Size Register A1"]
    pub mpu_rasr_a1: MPU_RASR_A1,
    #[doc = "0xdac - MPU Region Base Address Register A2"]
    pub mpu_rbar_a2: MPU_RBAR_A2,
    #[doc = "0xdb0 - MPU Region Attribute and Size Register A2"]
    pub mpu_rasr_a2: MPU_RASR_A2,
    #[doc = "0xdb4 - MPU Region Base Address Register A3"]
    pub mpu_rbar_a3: MPU_RBAR_A3,
    #[doc = "0xdb8 - MPU Region Attribute and Size Register A3"]
    pub mpu_rasr_a3: MPU_RASR_A3,
    _reserved12: [u8; 324usize],
    #[doc = "0xf00 - Software Trigger Interrupt Register"]
    pub stir: STIR,
    _reserved13: [u8; 48usize],
    #[doc = "0xf34 - Floating-point Context Control Register"]
    pub fpccr: FPCCR,
    #[doc = "0xf38 - Floating-point Context Address Register"]
    pub fpcar: FPCAR,
    #[doc = "0xf3c - Floating-point Default Status Control Register"]
    pub fpdscr: FPDSCR,
}
#[doc = "Auxiliary Control Register"]
pub struct ACTLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Auxiliary Control Register"]
pub mod actlr;
#[doc = "SysTick Control and Status Register"]
pub struct SYST_CSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SysTick Control and Status Register"]
pub mod syst_csr;
#[doc = "SysTick Reload Value Register"]
pub struct SYST_RVR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SysTick Reload Value Register"]
pub mod syst_rvr;
#[doc = "SysTick Current Value Register"]
pub struct SYST_CVR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SysTick Current Value Register"]
pub mod syst_cvr;
#[doc = "SysTick Calibration Value Register r"]
pub struct SYST_CALIB {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SysTick Calibration Value Register r"]
pub mod syst_calib;
#[doc = "Interrupt Set-enable Register 0"]
pub struct NVIC_ISER0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Set-enable Register 0"]
pub mod nvic_iser0;
#[doc = "Interrupt Set-enable Register 1"]
pub struct NVIC_ISER1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Set-enable Register 1"]
pub mod nvic_iser1;
#[doc = "Interrupt Set-enable Register 2"]
pub struct NVIC_ISER2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Set-enable Register 2"]
pub mod nvic_iser2;
#[doc = "Interrupt Set-enable Register 3"]
pub struct NVIC_ISER3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Set-enable Register 3"]
pub mod nvic_iser3;
#[doc = "Interrupt Clear-enable Register 0"]
pub struct NVIC_ICER0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Clear-enable Register 0"]
pub mod nvic_icer0;
#[doc = "Interrupt Clear-enable Register 1"]
pub struct NVIC_ICER1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Clear-enable Register 1"]
pub mod nvic_icer1;
#[doc = "Interrupt Clear-enable Register 2"]
pub struct NVIC_ICER2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Clear-enable Register 2"]
pub mod nvic_icer2;
#[doc = "Interrupt Clear-enable Register 3"]
pub struct NVIC_ICER3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Clear-enable Register 3"]
pub mod nvic_icer3;
#[doc = "Interrupt Set-pending Register 0"]
pub struct NVIC_ISPR0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Set-pending Register 0"]
pub mod nvic_ispr0;
#[doc = "Interrupt Set-pending Register 1"]
pub struct NVIC_ISPR1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Set-pending Register 1"]
pub mod nvic_ispr1;
#[doc = "Interrupt Set-pending Register 2"]
pub struct NVIC_ISPR2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Set-pending Register 2"]
pub mod nvic_ispr2;
#[doc = "Interrupt Set-pending Register 3"]
pub struct NVIC_ISPR3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Set-pending Register 3"]
pub mod nvic_ispr3;
#[doc = "Interrupt Clear-pending Register 0"]
pub struct NVIC_ICPR0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Clear-pending Register 0"]
pub mod nvic_icpr0;
#[doc = "Interrupt Clear-pending Register 1"]
pub struct NVIC_ICPR1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Clear-pending Register 1"]
pub mod nvic_icpr1;
#[doc = "Interrupt Clear-pending Register 2"]
pub struct NVIC_ICPR2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Clear-pending Register 2"]
pub mod nvic_icpr2;
#[doc = "Interrupt Clear-pending Register 3"]
pub struct NVIC_ICPR3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Clear-pending Register 3"]
pub mod nvic_icpr3;
#[doc = "Interrupt Active Bit Register 0"]
pub struct NVIC_IABR0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Active Bit Register 0"]
pub mod nvic_iabr0;
#[doc = "Interrupt Active Bit Register 1"]
pub struct NVIC_IABR1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Active Bit Register 1"]
pub mod nvic_iabr1;
#[doc = "Interrupt Active Bit Register 2"]
pub struct NVIC_IABR2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Active Bit Register 2"]
pub mod nvic_iabr2;
#[doc = "Interrupt Active Bit Register 3"]
pub struct NVIC_IABR3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Active Bit Register 3"]
pub mod nvic_iabr3;
#[doc = "Interrupt Priority Register 0"]
pub struct NVIC_IPR0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Priority Register 0"]
pub mod nvic_ipr0;
#[doc = "Interrupt Priority Register 1"]
pub struct NVIC_IPR1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Priority Register 1"]
pub mod nvic_ipr1;
#[doc = "Interrupt Priority Register 2"]
pub struct NVIC_IPR2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Priority Register 2"]
pub mod nvic_ipr2;
#[doc = "Interrupt Priority Register 3"]
pub struct NVIC_IPR3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Priority Register 3"]
pub mod nvic_ipr3;
#[doc = "Interrupt Priority Register 4"]
pub struct NVIC_IPR4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Priority Register 4"]
pub mod nvic_ipr4;
#[doc = "Interrupt Priority Register 5"]
pub struct NVIC_IPR5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Priority Register 5"]
pub mod nvic_ipr5;
#[doc = "Interrupt Priority Register 6"]
pub struct NVIC_IPR6 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Priority Register 6"]
pub mod nvic_ipr6;
#[doc = "Interrupt Priority Register 7"]
pub struct NVIC_IPR7 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Priority Register 7"]
pub mod nvic_ipr7;
#[doc = "Interrupt Priority Register 8"]
pub struct NVIC_IPR8 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Priority Register 8"]
pub mod nvic_ipr8;
#[doc = "Interrupt Priority Register 9"]
pub struct NVIC_IPR9 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Priority Register 9"]
pub mod nvic_ipr9;
#[doc = "Interrupt Priority Register 10"]
pub struct NVIC_IPR10 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Priority Register 10"]
pub mod nvic_ipr10;
#[doc = "Interrupt Priority Register 11"]
pub struct NVIC_IPR11 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Priority Register 11"]
pub mod nvic_ipr11;
#[doc = "Interrupt Priority Register 12"]
pub struct NVIC_IPR12 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Priority Register 12"]
pub mod nvic_ipr12;
#[doc = "Interrupt Priority Register 13"]
pub struct NVIC_IPR13 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Priority Register 13"]
pub mod nvic_ipr13;
#[doc = "Interrupt Priority Register 14"]
pub struct NVIC_IPR14 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Priority Register 14"]
pub mod nvic_ipr14;
#[doc = "Interrupt Priority Register 15"]
pub struct NVIC_IPR15 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Priority Register 15"]
pub mod nvic_ipr15;
#[doc = "Interrupt Priority Register 16"]
pub struct NVIC_IPR16 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Priority Register 16"]
pub mod nvic_ipr16;
#[doc = "Interrupt Priority Register 17"]
pub struct NVIC_IPR17 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Priority Register 17"]
pub mod nvic_ipr17;
#[doc = "Interrupt Priority Register 18"]
pub struct NVIC_IPR18 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Priority Register 18"]
pub mod nvic_ipr18;
#[doc = "Interrupt Priority Register 19"]
pub struct NVIC_IPR19 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Priority Register 19"]
pub mod nvic_ipr19;
#[doc = "Interrupt Priority Register 20"]
pub struct NVIC_IPR20 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Priority Register 20"]
pub mod nvic_ipr20;
#[doc = "Interrupt Priority Register 21"]
pub struct NVIC_IPR21 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Priority Register 21"]
pub mod nvic_ipr21;
#[doc = "Interrupt Priority Register 22"]
pub struct NVIC_IPR22 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Priority Register 22"]
pub mod nvic_ipr22;
#[doc = "Interrupt Priority Register 23"]
pub struct NVIC_IPR23 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Priority Register 23"]
pub mod nvic_ipr23;
#[doc = "Interrupt Priority Register 24"]
pub struct NVIC_IPR24 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Priority Register 24"]
pub mod nvic_ipr24;
#[doc = "Interrupt Priority Register 25"]
pub struct NVIC_IPR25 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Priority Register 25"]
pub mod nvic_ipr25;
#[doc = "Interrupt Priority Register 26"]
pub struct NVIC_IPR26 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Priority Register 26"]
pub mod nvic_ipr26;
#[doc = "Interrupt Priority Register 27"]
pub struct NVIC_IPR27 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Priority Register 27"]
pub mod nvic_ipr27;
#[doc = "CPUID Base Register"]
pub struct CPUID {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CPUID Base Register"]
pub mod cpuid;
#[doc = "Interrupt Control and State Register"]
pub struct ICSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Control and State Register"]
pub mod icsr;
#[doc = "Vector Table Offset Register"]
pub struct VTOR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Vector Table Offset Register"]
pub mod vtor;
#[doc = "Application Interrupt and Reset Control Register"]
pub struct AIRCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Application Interrupt and Reset Control Register"]
pub mod aircr;
#[doc = "System Control Register"]
pub struct SCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "System Control Register"]
pub mod scr;
#[doc = "Configuration and Control Register"]
pub struct CCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Configuration and Control Register"]
pub mod ccr;
#[doc = "System Handler Priority Register 1"]
pub struct SHPR1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "System Handler Priority Register 1"]
pub mod shpr1;
#[doc = "System Handler Priority Register 2"]
pub struct SHPR2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "System Handler Priority Register 2"]
pub mod shpr2;
#[doc = "System Handler Priority Register 3"]
pub struct SHPR3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "System Handler Priority Register 3"]
pub mod shpr3;
#[doc = "System Handler Control and State Register"]
pub struct SHCSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "System Handler Control and State Register"]
pub mod shcsr;
#[doc = "Configurable Fault Status Register"]
pub struct CFSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Configurable Fault Status Register"]
pub mod cfsr;
#[doc = "HardFault Status Register"]
pub struct HFSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "HardFault Status Register"]
pub mod hfsr;
#[doc = "MemManage Fault Address Register"]
pub struct MMFAR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MemManage Fault Address Register"]
pub mod mmfar;
#[doc = "BusFault Address Register"]
pub struct BFAR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "BusFault Address Register"]
pub mod bfar;
#[doc = "Auxiliary Fault Status Register"]
pub struct AFSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Auxiliary Fault Status Register"]
pub mod afsr;
#[doc = "Coprocessor Access Control Register"]
pub struct CPACR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Coprocessor Access Control Register"]
pub mod cpacr;
#[doc = "MPU Type Register"]
pub struct MPU_TYPE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MPU Type Register"]
pub mod mpu_type;
#[doc = "MPU Control Register"]
pub struct MPU_CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MPU Control Register"]
pub mod mpu_ctrl;
#[doc = "MPU Region Number Register"]
pub struct MPU_RNR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MPU Region Number Register"]
pub mod mpu_rnr;
#[doc = "MPU Region Base Address Register"]
pub struct MPU_RBAR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MPU Region Base Address Register"]
pub mod mpu_rbar;
#[doc = "MPU Region Attribute and Size Register"]
pub struct MPU_RASR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MPU Region Attribute and Size Register"]
pub mod mpu_rasr;
#[doc = "MPU Region Base Address Register A1"]
pub struct MPU_RBAR_A1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MPU Region Base Address Register A1"]
pub mod mpu_rbar_a1;
#[doc = "MPU Region Attribute and Size Register A1"]
pub struct MPU_RASR_A1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MPU Region Attribute and Size Register A1"]
pub mod mpu_rasr_a1;
#[doc = "MPU Region Base Address Register A2"]
pub struct MPU_RBAR_A2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MPU Region Base Address Register A2"]
pub mod mpu_rbar_a2;
#[doc = "MPU Region Attribute and Size Register A2"]
pub struct MPU_RASR_A2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MPU Region Attribute and Size Register A2"]
pub mod mpu_rasr_a2;
#[doc = "MPU Region Base Address Register A3"]
pub struct MPU_RBAR_A3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MPU Region Base Address Register A3"]
pub mod mpu_rbar_a3;
#[doc = "MPU Region Attribute and Size Register A3"]
pub struct MPU_RASR_A3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MPU Region Attribute and Size Register A3"]
pub mod mpu_rasr_a3;
#[doc = "Software Trigger Interrupt Register"]
pub struct STIR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Software Trigger Interrupt Register"]
pub mod stir;
#[doc = "Floating-point Context Control Register"]
pub struct FPCCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Floating-point Context Control Register"]
pub mod fpccr;
#[doc = "Floating-point Context Address Register"]
pub struct FPCAR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Floating-point Context Address Register"]
pub mod fpcar;
#[doc = "Floating-point Default Status Control Register"]
pub struct FPDSCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Floating-point Default Status Control Register"]
pub mod fpdscr;
