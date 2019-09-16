#[doc = r"Register block"]
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
    _reserved5: [u8; 224usize],
    #[doc = "0x100 - Interrupt Set-enable Register 0"]
    pub nvic_iser0: NVIC_ISER0,
    #[doc = "0x104 - Interrupt Set-enable Register 1"]
    pub nvic_iser1: NVIC_ISER1,
    #[doc = "0x108 - Interrupt Set-enable Register 2"]
    pub nvic_iser2: NVIC_ISER2,
    #[doc = "0x10c - Interrupt Set-enable Register 3"]
    pub nvic_iser3: NVIC_ISER3,
    _reserved9: [u8; 112usize],
    #[doc = "0x180 - Interrupt Clear-enable Register 0"]
    pub nvic_icer0: NVIC_ICER0,
    #[doc = "0x184 - Interrupt Clear-enable Register 1"]
    pub nvic_icer1: NVIC_ICER1,
    #[doc = "0x188 - Interrupt Clear-enable Register 2"]
    pub nvic_icer2: NVIC_ICER2,
    #[doc = "0x18c - Interrupt Clear-enable Register 3"]
    pub nvic_icer3: NVIC_ICER3,
    _reserved13: [u8; 112usize],
    #[doc = "0x200 - Interrupt Set-pending Register 0"]
    pub nvic_ispr0: NVIC_ISPR0,
    #[doc = "0x204 - Interrupt Set-pending Register 1"]
    pub nvic_ispr1: NVIC_ISPR1,
    #[doc = "0x208 - Interrupt Set-pending Register 2"]
    pub nvic_ispr2: NVIC_ISPR2,
    #[doc = "0x20c - Interrupt Set-pending Register 3"]
    pub nvic_ispr3: NVIC_ISPR3,
    _reserved17: [u8; 112usize],
    #[doc = "0x280 - Interrupt Clear-pending Register 0"]
    pub nvic_icpr0: NVIC_ICPR0,
    #[doc = "0x284 - Interrupt Clear-pending Register 1"]
    pub nvic_icpr1: NVIC_ICPR1,
    #[doc = "0x288 - Interrupt Clear-pending Register 2"]
    pub nvic_icpr2: NVIC_ICPR2,
    #[doc = "0x28c - Interrupt Clear-pending Register 3"]
    pub nvic_icpr3: NVIC_ICPR3,
    _reserved21: [u8; 112usize],
    #[doc = "0x300 - Interrupt Active Bit Register 0"]
    pub nvic_iabr0: NVIC_IABR0,
    #[doc = "0x304 - Interrupt Active Bit Register 1"]
    pub nvic_iabr1: NVIC_IABR1,
    #[doc = "0x308 - Interrupt Active Bit Register 2"]
    pub nvic_iabr2: NVIC_IABR2,
    #[doc = "0x30c - Interrupt Active Bit Register 3"]
    pub nvic_iabr3: NVIC_IABR3,
    _reserved25: [u8; 240usize],
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
    _reserved53: [u8; 2192usize],
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
    _reserved65: [u8; 4usize],
    #[doc = "0xd34 - MemManage Fault Address Register"]
    pub mmfar: MMFAR,
    #[doc = "0xd38 - BusFault Address Register"]
    pub bfar: BFAR,
    #[doc = "0xd3c - Auxiliary Fault Status Register"]
    pub afsr: AFSR,
    _reserved68: [u8; 72usize],
    #[doc = "0xd88 - Coprocessor Access Control Register"]
    pub cpacr: CPACR,
    _reserved69: [u8; 4usize],
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
    _reserved80: [u8; 324usize],
    #[doc = "0xf00 - Software Trigger Interrupt Register"]
    pub stir: STIR,
    _reserved81: [u8; 48usize],
    #[doc = "0xf34 - Floating-point Context Control Register"]
    pub fpccr: FPCCR,
    #[doc = "0xf38 - Floating-point Context Address Register"]
    pub fpcar: FPCAR,
    #[doc = "0xf3c - Floating-point Default Status Control Register"]
    pub fpdscr: FPDSCR,
}
#[doc = "Auxiliary Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [actlr](actlr) module"]
pub type ACTLR = crate::Reg<u32, _ACTLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ACTLR;
#[doc = "`read()` method returns [actlr::R](actlr::R) reader structure"]
impl crate::Readable for ACTLR {}
#[doc = "`write(|w| ..)` method takes [actlr::W](actlr::W) writer structure"]
impl crate::Writable for ACTLR {}
#[doc = "Auxiliary Control Register"]
pub mod actlr;
#[doc = "SysTick Control and Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [syst_csr](syst_csr) module"]
pub type SYST_CSR = crate::Reg<u32, _SYST_CSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYST_CSR;
#[doc = "`read()` method returns [syst_csr::R](syst_csr::R) reader structure"]
impl crate::Readable for SYST_CSR {}
#[doc = "`write(|w| ..)` method takes [syst_csr::W](syst_csr::W) writer structure"]
impl crate::Writable for SYST_CSR {}
#[doc = "SysTick Control and Status Register"]
pub mod syst_csr;
#[doc = "SysTick Reload Value Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [syst_rvr](syst_rvr) module"]
pub type SYST_RVR = crate::Reg<u32, _SYST_RVR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYST_RVR;
#[doc = "`read()` method returns [syst_rvr::R](syst_rvr::R) reader structure"]
impl crate::Readable for SYST_RVR {}
#[doc = "`write(|w| ..)` method takes [syst_rvr::W](syst_rvr::W) writer structure"]
impl crate::Writable for SYST_RVR {}
#[doc = "SysTick Reload Value Register"]
pub mod syst_rvr;
#[doc = "SysTick Current Value Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [syst_cvr](syst_cvr) module"]
pub type SYST_CVR = crate::Reg<u32, _SYST_CVR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYST_CVR;
#[doc = "`read()` method returns [syst_cvr::R](syst_cvr::R) reader structure"]
impl crate::Readable for SYST_CVR {}
#[doc = "`write(|w| ..)` method takes [syst_cvr::W](syst_cvr::W) writer structure"]
impl crate::Writable for SYST_CVR {}
#[doc = "SysTick Current Value Register"]
pub mod syst_cvr;
#[doc = "SysTick Calibration Value Register r\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [syst_calib](syst_calib) module"]
pub type SYST_CALIB = crate::Reg<u32, _SYST_CALIB>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYST_CALIB;
#[doc = "`read()` method returns [syst_calib::R](syst_calib::R) reader structure"]
impl crate::Readable for SYST_CALIB {}
#[doc = "`write(|w| ..)` method takes [syst_calib::W](syst_calib::W) writer structure"]
impl crate::Writable for SYST_CALIB {}
#[doc = "SysTick Calibration Value Register r"]
pub mod syst_calib;
#[doc = "Interrupt Set-enable Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [nvic_iser0](nvic_iser0) module"]
pub type NVIC_ISER0 = crate::Reg<u32, _NVIC_ISER0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NVIC_ISER0;
#[doc = "`read()` method returns [nvic_iser0::R](nvic_iser0::R) reader structure"]
impl crate::Readable for NVIC_ISER0 {}
#[doc = "`write(|w| ..)` method takes [nvic_iser0::W](nvic_iser0::W) writer structure"]
impl crate::Writable for NVIC_ISER0 {}
#[doc = "Interrupt Set-enable Register 0"]
pub mod nvic_iser0;
#[doc = "Interrupt Set-enable Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [nvic_iser1](nvic_iser1) module"]
pub type NVIC_ISER1 = crate::Reg<u32, _NVIC_ISER1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NVIC_ISER1;
#[doc = "`read()` method returns [nvic_iser1::R](nvic_iser1::R) reader structure"]
impl crate::Readable for NVIC_ISER1 {}
#[doc = "`write(|w| ..)` method takes [nvic_iser1::W](nvic_iser1::W) writer structure"]
impl crate::Writable for NVIC_ISER1 {}
#[doc = "Interrupt Set-enable Register 1"]
pub mod nvic_iser1;
#[doc = "Interrupt Set-enable Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [nvic_iser2](nvic_iser2) module"]
pub type NVIC_ISER2 = crate::Reg<u32, _NVIC_ISER2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NVIC_ISER2;
#[doc = "`read()` method returns [nvic_iser2::R](nvic_iser2::R) reader structure"]
impl crate::Readable for NVIC_ISER2 {}
#[doc = "`write(|w| ..)` method takes [nvic_iser2::W](nvic_iser2::W) writer structure"]
impl crate::Writable for NVIC_ISER2 {}
#[doc = "Interrupt Set-enable Register 2"]
pub mod nvic_iser2;
#[doc = "Interrupt Set-enable Register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [nvic_iser3](nvic_iser3) module"]
pub type NVIC_ISER3 = crate::Reg<u32, _NVIC_ISER3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NVIC_ISER3;
#[doc = "`read()` method returns [nvic_iser3::R](nvic_iser3::R) reader structure"]
impl crate::Readable for NVIC_ISER3 {}
#[doc = "`write(|w| ..)` method takes [nvic_iser3::W](nvic_iser3::W) writer structure"]
impl crate::Writable for NVIC_ISER3 {}
#[doc = "Interrupt Set-enable Register 3"]
pub mod nvic_iser3;
#[doc = "Interrupt Clear-enable Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [nvic_icer0](nvic_icer0) module"]
pub type NVIC_ICER0 = crate::Reg<u32, _NVIC_ICER0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NVIC_ICER0;
#[doc = "`read()` method returns [nvic_icer0::R](nvic_icer0::R) reader structure"]
impl crate::Readable for NVIC_ICER0 {}
#[doc = "`write(|w| ..)` method takes [nvic_icer0::W](nvic_icer0::W) writer structure"]
impl crate::Writable for NVIC_ICER0 {}
#[doc = "Interrupt Clear-enable Register 0"]
pub mod nvic_icer0;
#[doc = "Interrupt Clear-enable Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [nvic_icer1](nvic_icer1) module"]
pub type NVIC_ICER1 = crate::Reg<u32, _NVIC_ICER1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NVIC_ICER1;
#[doc = "`read()` method returns [nvic_icer1::R](nvic_icer1::R) reader structure"]
impl crate::Readable for NVIC_ICER1 {}
#[doc = "`write(|w| ..)` method takes [nvic_icer1::W](nvic_icer1::W) writer structure"]
impl crate::Writable for NVIC_ICER1 {}
#[doc = "Interrupt Clear-enable Register 1"]
pub mod nvic_icer1;
#[doc = "Interrupt Clear-enable Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [nvic_icer2](nvic_icer2) module"]
pub type NVIC_ICER2 = crate::Reg<u32, _NVIC_ICER2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NVIC_ICER2;
#[doc = "`read()` method returns [nvic_icer2::R](nvic_icer2::R) reader structure"]
impl crate::Readable for NVIC_ICER2 {}
#[doc = "`write(|w| ..)` method takes [nvic_icer2::W](nvic_icer2::W) writer structure"]
impl crate::Writable for NVIC_ICER2 {}
#[doc = "Interrupt Clear-enable Register 2"]
pub mod nvic_icer2;
#[doc = "Interrupt Clear-enable Register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [nvic_icer3](nvic_icer3) module"]
pub type NVIC_ICER3 = crate::Reg<u32, _NVIC_ICER3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NVIC_ICER3;
#[doc = "`read()` method returns [nvic_icer3::R](nvic_icer3::R) reader structure"]
impl crate::Readable for NVIC_ICER3 {}
#[doc = "`write(|w| ..)` method takes [nvic_icer3::W](nvic_icer3::W) writer structure"]
impl crate::Writable for NVIC_ICER3 {}
#[doc = "Interrupt Clear-enable Register 3"]
pub mod nvic_icer3;
#[doc = "Interrupt Set-pending Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [nvic_ispr0](nvic_ispr0) module"]
pub type NVIC_ISPR0 = crate::Reg<u32, _NVIC_ISPR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NVIC_ISPR0;
#[doc = "`read()` method returns [nvic_ispr0::R](nvic_ispr0::R) reader structure"]
impl crate::Readable for NVIC_ISPR0 {}
#[doc = "`write(|w| ..)` method takes [nvic_ispr0::W](nvic_ispr0::W) writer structure"]
impl crate::Writable for NVIC_ISPR0 {}
#[doc = "Interrupt Set-pending Register 0"]
pub mod nvic_ispr0;
#[doc = "Interrupt Set-pending Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [nvic_ispr1](nvic_ispr1) module"]
pub type NVIC_ISPR1 = crate::Reg<u32, _NVIC_ISPR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NVIC_ISPR1;
#[doc = "`read()` method returns [nvic_ispr1::R](nvic_ispr1::R) reader structure"]
impl crate::Readable for NVIC_ISPR1 {}
#[doc = "`write(|w| ..)` method takes [nvic_ispr1::W](nvic_ispr1::W) writer structure"]
impl crate::Writable for NVIC_ISPR1 {}
#[doc = "Interrupt Set-pending Register 1"]
pub mod nvic_ispr1;
#[doc = "Interrupt Set-pending Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [nvic_ispr2](nvic_ispr2) module"]
pub type NVIC_ISPR2 = crate::Reg<u32, _NVIC_ISPR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NVIC_ISPR2;
#[doc = "`read()` method returns [nvic_ispr2::R](nvic_ispr2::R) reader structure"]
impl crate::Readable for NVIC_ISPR2 {}
#[doc = "`write(|w| ..)` method takes [nvic_ispr2::W](nvic_ispr2::W) writer structure"]
impl crate::Writable for NVIC_ISPR2 {}
#[doc = "Interrupt Set-pending Register 2"]
pub mod nvic_ispr2;
#[doc = "Interrupt Set-pending Register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [nvic_ispr3](nvic_ispr3) module"]
pub type NVIC_ISPR3 = crate::Reg<u32, _NVIC_ISPR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NVIC_ISPR3;
#[doc = "`read()` method returns [nvic_ispr3::R](nvic_ispr3::R) reader structure"]
impl crate::Readable for NVIC_ISPR3 {}
#[doc = "`write(|w| ..)` method takes [nvic_ispr3::W](nvic_ispr3::W) writer structure"]
impl crate::Writable for NVIC_ISPR3 {}
#[doc = "Interrupt Set-pending Register 3"]
pub mod nvic_ispr3;
#[doc = "Interrupt Clear-pending Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [nvic_icpr0](nvic_icpr0) module"]
pub type NVIC_ICPR0 = crate::Reg<u32, _NVIC_ICPR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NVIC_ICPR0;
#[doc = "`read()` method returns [nvic_icpr0::R](nvic_icpr0::R) reader structure"]
impl crate::Readable for NVIC_ICPR0 {}
#[doc = "`write(|w| ..)` method takes [nvic_icpr0::W](nvic_icpr0::W) writer structure"]
impl crate::Writable for NVIC_ICPR0 {}
#[doc = "Interrupt Clear-pending Register 0"]
pub mod nvic_icpr0;
#[doc = "Interrupt Clear-pending Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [nvic_icpr1](nvic_icpr1) module"]
pub type NVIC_ICPR1 = crate::Reg<u32, _NVIC_ICPR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NVIC_ICPR1;
#[doc = "`read()` method returns [nvic_icpr1::R](nvic_icpr1::R) reader structure"]
impl crate::Readable for NVIC_ICPR1 {}
#[doc = "`write(|w| ..)` method takes [nvic_icpr1::W](nvic_icpr1::W) writer structure"]
impl crate::Writable for NVIC_ICPR1 {}
#[doc = "Interrupt Clear-pending Register 1"]
pub mod nvic_icpr1;
#[doc = "Interrupt Clear-pending Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [nvic_icpr2](nvic_icpr2) module"]
pub type NVIC_ICPR2 = crate::Reg<u32, _NVIC_ICPR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NVIC_ICPR2;
#[doc = "`read()` method returns [nvic_icpr2::R](nvic_icpr2::R) reader structure"]
impl crate::Readable for NVIC_ICPR2 {}
#[doc = "`write(|w| ..)` method takes [nvic_icpr2::W](nvic_icpr2::W) writer structure"]
impl crate::Writable for NVIC_ICPR2 {}
#[doc = "Interrupt Clear-pending Register 2"]
pub mod nvic_icpr2;
#[doc = "Interrupt Clear-pending Register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [nvic_icpr3](nvic_icpr3) module"]
pub type NVIC_ICPR3 = crate::Reg<u32, _NVIC_ICPR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NVIC_ICPR3;
#[doc = "`read()` method returns [nvic_icpr3::R](nvic_icpr3::R) reader structure"]
impl crate::Readable for NVIC_ICPR3 {}
#[doc = "`write(|w| ..)` method takes [nvic_icpr3::W](nvic_icpr3::W) writer structure"]
impl crate::Writable for NVIC_ICPR3 {}
#[doc = "Interrupt Clear-pending Register 3"]
pub mod nvic_icpr3;
#[doc = "Interrupt Active Bit Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [nvic_iabr0](nvic_iabr0) module"]
pub type NVIC_IABR0 = crate::Reg<u32, _NVIC_IABR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NVIC_IABR0;
#[doc = "`read()` method returns [nvic_iabr0::R](nvic_iabr0::R) reader structure"]
impl crate::Readable for NVIC_IABR0 {}
#[doc = "`write(|w| ..)` method takes [nvic_iabr0::W](nvic_iabr0::W) writer structure"]
impl crate::Writable for NVIC_IABR0 {}
#[doc = "Interrupt Active Bit Register 0"]
pub mod nvic_iabr0;
#[doc = "Interrupt Active Bit Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [nvic_iabr1](nvic_iabr1) module"]
pub type NVIC_IABR1 = crate::Reg<u32, _NVIC_IABR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NVIC_IABR1;
#[doc = "`read()` method returns [nvic_iabr1::R](nvic_iabr1::R) reader structure"]
impl crate::Readable for NVIC_IABR1 {}
#[doc = "`write(|w| ..)` method takes [nvic_iabr1::W](nvic_iabr1::W) writer structure"]
impl crate::Writable for NVIC_IABR1 {}
#[doc = "Interrupt Active Bit Register 1"]
pub mod nvic_iabr1;
#[doc = "Interrupt Active Bit Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [nvic_iabr2](nvic_iabr2) module"]
pub type NVIC_IABR2 = crate::Reg<u32, _NVIC_IABR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NVIC_IABR2;
#[doc = "`read()` method returns [nvic_iabr2::R](nvic_iabr2::R) reader structure"]
impl crate::Readable for NVIC_IABR2 {}
#[doc = "`write(|w| ..)` method takes [nvic_iabr2::W](nvic_iabr2::W) writer structure"]
impl crate::Writable for NVIC_IABR2 {}
#[doc = "Interrupt Active Bit Register 2"]
pub mod nvic_iabr2;
#[doc = "Interrupt Active Bit Register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [nvic_iabr3](nvic_iabr3) module"]
pub type NVIC_IABR3 = crate::Reg<u32, _NVIC_IABR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NVIC_IABR3;
#[doc = "`read()` method returns [nvic_iabr3::R](nvic_iabr3::R) reader structure"]
impl crate::Readable for NVIC_IABR3 {}
#[doc = "`write(|w| ..)` method takes [nvic_iabr3::W](nvic_iabr3::W) writer structure"]
impl crate::Writable for NVIC_IABR3 {}
#[doc = "Interrupt Active Bit Register 3"]
pub mod nvic_iabr3;
#[doc = "Interrupt Priority Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [nvic_ipr0](nvic_ipr0) module"]
pub type NVIC_IPR0 = crate::Reg<u32, _NVIC_IPR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NVIC_IPR0;
#[doc = "`read()` method returns [nvic_ipr0::R](nvic_ipr0::R) reader structure"]
impl crate::Readable for NVIC_IPR0 {}
#[doc = "`write(|w| ..)` method takes [nvic_ipr0::W](nvic_ipr0::W) writer structure"]
impl crate::Writable for NVIC_IPR0 {}
#[doc = "Interrupt Priority Register 0"]
pub mod nvic_ipr0;
#[doc = "Interrupt Priority Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [nvic_ipr1](nvic_ipr1) module"]
pub type NVIC_IPR1 = crate::Reg<u32, _NVIC_IPR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NVIC_IPR1;
#[doc = "`read()` method returns [nvic_ipr1::R](nvic_ipr1::R) reader structure"]
impl crate::Readable for NVIC_IPR1 {}
#[doc = "`write(|w| ..)` method takes [nvic_ipr1::W](nvic_ipr1::W) writer structure"]
impl crate::Writable for NVIC_IPR1 {}
#[doc = "Interrupt Priority Register 1"]
pub mod nvic_ipr1;
#[doc = "Interrupt Priority Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [nvic_ipr2](nvic_ipr2) module"]
pub type NVIC_IPR2 = crate::Reg<u32, _NVIC_IPR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NVIC_IPR2;
#[doc = "`read()` method returns [nvic_ipr2::R](nvic_ipr2::R) reader structure"]
impl crate::Readable for NVIC_IPR2 {}
#[doc = "`write(|w| ..)` method takes [nvic_ipr2::W](nvic_ipr2::W) writer structure"]
impl crate::Writable for NVIC_IPR2 {}
#[doc = "Interrupt Priority Register 2"]
pub mod nvic_ipr2;
#[doc = "Interrupt Priority Register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [nvic_ipr3](nvic_ipr3) module"]
pub type NVIC_IPR3 = crate::Reg<u32, _NVIC_IPR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NVIC_IPR3;
#[doc = "`read()` method returns [nvic_ipr3::R](nvic_ipr3::R) reader structure"]
impl crate::Readable for NVIC_IPR3 {}
#[doc = "`write(|w| ..)` method takes [nvic_ipr3::W](nvic_ipr3::W) writer structure"]
impl crate::Writable for NVIC_IPR3 {}
#[doc = "Interrupt Priority Register 3"]
pub mod nvic_ipr3;
#[doc = "Interrupt Priority Register 4\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [nvic_ipr4](nvic_ipr4) module"]
pub type NVIC_IPR4 = crate::Reg<u32, _NVIC_IPR4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NVIC_IPR4;
#[doc = "`read()` method returns [nvic_ipr4::R](nvic_ipr4::R) reader structure"]
impl crate::Readable for NVIC_IPR4 {}
#[doc = "`write(|w| ..)` method takes [nvic_ipr4::W](nvic_ipr4::W) writer structure"]
impl crate::Writable for NVIC_IPR4 {}
#[doc = "Interrupt Priority Register 4"]
pub mod nvic_ipr4;
#[doc = "Interrupt Priority Register 5\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [nvic_ipr5](nvic_ipr5) module"]
pub type NVIC_IPR5 = crate::Reg<u32, _NVIC_IPR5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NVIC_IPR5;
#[doc = "`read()` method returns [nvic_ipr5::R](nvic_ipr5::R) reader structure"]
impl crate::Readable for NVIC_IPR5 {}
#[doc = "`write(|w| ..)` method takes [nvic_ipr5::W](nvic_ipr5::W) writer structure"]
impl crate::Writable for NVIC_IPR5 {}
#[doc = "Interrupt Priority Register 5"]
pub mod nvic_ipr5;
#[doc = "Interrupt Priority Register 6\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [nvic_ipr6](nvic_ipr6) module"]
pub type NVIC_IPR6 = crate::Reg<u32, _NVIC_IPR6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NVIC_IPR6;
#[doc = "`read()` method returns [nvic_ipr6::R](nvic_ipr6::R) reader structure"]
impl crate::Readable for NVIC_IPR6 {}
#[doc = "`write(|w| ..)` method takes [nvic_ipr6::W](nvic_ipr6::W) writer structure"]
impl crate::Writable for NVIC_IPR6 {}
#[doc = "Interrupt Priority Register 6"]
pub mod nvic_ipr6;
#[doc = "Interrupt Priority Register 7\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [nvic_ipr7](nvic_ipr7) module"]
pub type NVIC_IPR7 = crate::Reg<u32, _NVIC_IPR7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NVIC_IPR7;
#[doc = "`read()` method returns [nvic_ipr7::R](nvic_ipr7::R) reader structure"]
impl crate::Readable for NVIC_IPR7 {}
#[doc = "`write(|w| ..)` method takes [nvic_ipr7::W](nvic_ipr7::W) writer structure"]
impl crate::Writable for NVIC_IPR7 {}
#[doc = "Interrupt Priority Register 7"]
pub mod nvic_ipr7;
#[doc = "Interrupt Priority Register 8\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [nvic_ipr8](nvic_ipr8) module"]
pub type NVIC_IPR8 = crate::Reg<u32, _NVIC_IPR8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NVIC_IPR8;
#[doc = "`read()` method returns [nvic_ipr8::R](nvic_ipr8::R) reader structure"]
impl crate::Readable for NVIC_IPR8 {}
#[doc = "`write(|w| ..)` method takes [nvic_ipr8::W](nvic_ipr8::W) writer structure"]
impl crate::Writable for NVIC_IPR8 {}
#[doc = "Interrupt Priority Register 8"]
pub mod nvic_ipr8;
#[doc = "Interrupt Priority Register 9\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [nvic_ipr9](nvic_ipr9) module"]
pub type NVIC_IPR9 = crate::Reg<u32, _NVIC_IPR9>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NVIC_IPR9;
#[doc = "`read()` method returns [nvic_ipr9::R](nvic_ipr9::R) reader structure"]
impl crate::Readable for NVIC_IPR9 {}
#[doc = "`write(|w| ..)` method takes [nvic_ipr9::W](nvic_ipr9::W) writer structure"]
impl crate::Writable for NVIC_IPR9 {}
#[doc = "Interrupt Priority Register 9"]
pub mod nvic_ipr9;
#[doc = "Interrupt Priority Register 10\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [nvic_ipr10](nvic_ipr10) module"]
pub type NVIC_IPR10 = crate::Reg<u32, _NVIC_IPR10>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NVIC_IPR10;
#[doc = "`read()` method returns [nvic_ipr10::R](nvic_ipr10::R) reader structure"]
impl crate::Readable for NVIC_IPR10 {}
#[doc = "`write(|w| ..)` method takes [nvic_ipr10::W](nvic_ipr10::W) writer structure"]
impl crate::Writable for NVIC_IPR10 {}
#[doc = "Interrupt Priority Register 10"]
pub mod nvic_ipr10;
#[doc = "Interrupt Priority Register 11\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [nvic_ipr11](nvic_ipr11) module"]
pub type NVIC_IPR11 = crate::Reg<u32, _NVIC_IPR11>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NVIC_IPR11;
#[doc = "`read()` method returns [nvic_ipr11::R](nvic_ipr11::R) reader structure"]
impl crate::Readable for NVIC_IPR11 {}
#[doc = "`write(|w| ..)` method takes [nvic_ipr11::W](nvic_ipr11::W) writer structure"]
impl crate::Writable for NVIC_IPR11 {}
#[doc = "Interrupt Priority Register 11"]
pub mod nvic_ipr11;
#[doc = "Interrupt Priority Register 12\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [nvic_ipr12](nvic_ipr12) module"]
pub type NVIC_IPR12 = crate::Reg<u32, _NVIC_IPR12>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NVIC_IPR12;
#[doc = "`read()` method returns [nvic_ipr12::R](nvic_ipr12::R) reader structure"]
impl crate::Readable for NVIC_IPR12 {}
#[doc = "`write(|w| ..)` method takes [nvic_ipr12::W](nvic_ipr12::W) writer structure"]
impl crate::Writable for NVIC_IPR12 {}
#[doc = "Interrupt Priority Register 12"]
pub mod nvic_ipr12;
#[doc = "Interrupt Priority Register 13\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [nvic_ipr13](nvic_ipr13) module"]
pub type NVIC_IPR13 = crate::Reg<u32, _NVIC_IPR13>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NVIC_IPR13;
#[doc = "`read()` method returns [nvic_ipr13::R](nvic_ipr13::R) reader structure"]
impl crate::Readable for NVIC_IPR13 {}
#[doc = "`write(|w| ..)` method takes [nvic_ipr13::W](nvic_ipr13::W) writer structure"]
impl crate::Writable for NVIC_IPR13 {}
#[doc = "Interrupt Priority Register 13"]
pub mod nvic_ipr13;
#[doc = "Interrupt Priority Register 14\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [nvic_ipr14](nvic_ipr14) module"]
pub type NVIC_IPR14 = crate::Reg<u32, _NVIC_IPR14>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NVIC_IPR14;
#[doc = "`read()` method returns [nvic_ipr14::R](nvic_ipr14::R) reader structure"]
impl crate::Readable for NVIC_IPR14 {}
#[doc = "`write(|w| ..)` method takes [nvic_ipr14::W](nvic_ipr14::W) writer structure"]
impl crate::Writable for NVIC_IPR14 {}
#[doc = "Interrupt Priority Register 14"]
pub mod nvic_ipr14;
#[doc = "Interrupt Priority Register 15\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [nvic_ipr15](nvic_ipr15) module"]
pub type NVIC_IPR15 = crate::Reg<u32, _NVIC_IPR15>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NVIC_IPR15;
#[doc = "`read()` method returns [nvic_ipr15::R](nvic_ipr15::R) reader structure"]
impl crate::Readable for NVIC_IPR15 {}
#[doc = "`write(|w| ..)` method takes [nvic_ipr15::W](nvic_ipr15::W) writer structure"]
impl crate::Writable for NVIC_IPR15 {}
#[doc = "Interrupt Priority Register 15"]
pub mod nvic_ipr15;
#[doc = "Interrupt Priority Register 16\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [nvic_ipr16](nvic_ipr16) module"]
pub type NVIC_IPR16 = crate::Reg<u32, _NVIC_IPR16>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NVIC_IPR16;
#[doc = "`read()` method returns [nvic_ipr16::R](nvic_ipr16::R) reader structure"]
impl crate::Readable for NVIC_IPR16 {}
#[doc = "`write(|w| ..)` method takes [nvic_ipr16::W](nvic_ipr16::W) writer structure"]
impl crate::Writable for NVIC_IPR16 {}
#[doc = "Interrupt Priority Register 16"]
pub mod nvic_ipr16;
#[doc = "Interrupt Priority Register 17\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [nvic_ipr17](nvic_ipr17) module"]
pub type NVIC_IPR17 = crate::Reg<u32, _NVIC_IPR17>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NVIC_IPR17;
#[doc = "`read()` method returns [nvic_ipr17::R](nvic_ipr17::R) reader structure"]
impl crate::Readable for NVIC_IPR17 {}
#[doc = "`write(|w| ..)` method takes [nvic_ipr17::W](nvic_ipr17::W) writer structure"]
impl crate::Writable for NVIC_IPR17 {}
#[doc = "Interrupt Priority Register 17"]
pub mod nvic_ipr17;
#[doc = "Interrupt Priority Register 18\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [nvic_ipr18](nvic_ipr18) module"]
pub type NVIC_IPR18 = crate::Reg<u32, _NVIC_IPR18>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NVIC_IPR18;
#[doc = "`read()` method returns [nvic_ipr18::R](nvic_ipr18::R) reader structure"]
impl crate::Readable for NVIC_IPR18 {}
#[doc = "`write(|w| ..)` method takes [nvic_ipr18::W](nvic_ipr18::W) writer structure"]
impl crate::Writable for NVIC_IPR18 {}
#[doc = "Interrupt Priority Register 18"]
pub mod nvic_ipr18;
#[doc = "Interrupt Priority Register 19\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [nvic_ipr19](nvic_ipr19) module"]
pub type NVIC_IPR19 = crate::Reg<u32, _NVIC_IPR19>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NVIC_IPR19;
#[doc = "`read()` method returns [nvic_ipr19::R](nvic_ipr19::R) reader structure"]
impl crate::Readable for NVIC_IPR19 {}
#[doc = "`write(|w| ..)` method takes [nvic_ipr19::W](nvic_ipr19::W) writer structure"]
impl crate::Writable for NVIC_IPR19 {}
#[doc = "Interrupt Priority Register 19"]
pub mod nvic_ipr19;
#[doc = "Interrupt Priority Register 20\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [nvic_ipr20](nvic_ipr20) module"]
pub type NVIC_IPR20 = crate::Reg<u32, _NVIC_IPR20>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NVIC_IPR20;
#[doc = "`read()` method returns [nvic_ipr20::R](nvic_ipr20::R) reader structure"]
impl crate::Readable for NVIC_IPR20 {}
#[doc = "`write(|w| ..)` method takes [nvic_ipr20::W](nvic_ipr20::W) writer structure"]
impl crate::Writable for NVIC_IPR20 {}
#[doc = "Interrupt Priority Register 20"]
pub mod nvic_ipr20;
#[doc = "Interrupt Priority Register 21\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [nvic_ipr21](nvic_ipr21) module"]
pub type NVIC_IPR21 = crate::Reg<u32, _NVIC_IPR21>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NVIC_IPR21;
#[doc = "`read()` method returns [nvic_ipr21::R](nvic_ipr21::R) reader structure"]
impl crate::Readable for NVIC_IPR21 {}
#[doc = "`write(|w| ..)` method takes [nvic_ipr21::W](nvic_ipr21::W) writer structure"]
impl crate::Writable for NVIC_IPR21 {}
#[doc = "Interrupt Priority Register 21"]
pub mod nvic_ipr21;
#[doc = "Interrupt Priority Register 22\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [nvic_ipr22](nvic_ipr22) module"]
pub type NVIC_IPR22 = crate::Reg<u32, _NVIC_IPR22>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NVIC_IPR22;
#[doc = "`read()` method returns [nvic_ipr22::R](nvic_ipr22::R) reader structure"]
impl crate::Readable for NVIC_IPR22 {}
#[doc = "`write(|w| ..)` method takes [nvic_ipr22::W](nvic_ipr22::W) writer structure"]
impl crate::Writable for NVIC_IPR22 {}
#[doc = "Interrupt Priority Register 22"]
pub mod nvic_ipr22;
#[doc = "Interrupt Priority Register 23\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [nvic_ipr23](nvic_ipr23) module"]
pub type NVIC_IPR23 = crate::Reg<u32, _NVIC_IPR23>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NVIC_IPR23;
#[doc = "`read()` method returns [nvic_ipr23::R](nvic_ipr23::R) reader structure"]
impl crate::Readable for NVIC_IPR23 {}
#[doc = "`write(|w| ..)` method takes [nvic_ipr23::W](nvic_ipr23::W) writer structure"]
impl crate::Writable for NVIC_IPR23 {}
#[doc = "Interrupt Priority Register 23"]
pub mod nvic_ipr23;
#[doc = "Interrupt Priority Register 24\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [nvic_ipr24](nvic_ipr24) module"]
pub type NVIC_IPR24 = crate::Reg<u32, _NVIC_IPR24>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NVIC_IPR24;
#[doc = "`read()` method returns [nvic_ipr24::R](nvic_ipr24::R) reader structure"]
impl crate::Readable for NVIC_IPR24 {}
#[doc = "`write(|w| ..)` method takes [nvic_ipr24::W](nvic_ipr24::W) writer structure"]
impl crate::Writable for NVIC_IPR24 {}
#[doc = "Interrupt Priority Register 24"]
pub mod nvic_ipr24;
#[doc = "Interrupt Priority Register 25\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [nvic_ipr25](nvic_ipr25) module"]
pub type NVIC_IPR25 = crate::Reg<u32, _NVIC_IPR25>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NVIC_IPR25;
#[doc = "`read()` method returns [nvic_ipr25::R](nvic_ipr25::R) reader structure"]
impl crate::Readable for NVIC_IPR25 {}
#[doc = "`write(|w| ..)` method takes [nvic_ipr25::W](nvic_ipr25::W) writer structure"]
impl crate::Writable for NVIC_IPR25 {}
#[doc = "Interrupt Priority Register 25"]
pub mod nvic_ipr25;
#[doc = "Interrupt Priority Register 26\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [nvic_ipr26](nvic_ipr26) module"]
pub type NVIC_IPR26 = crate::Reg<u32, _NVIC_IPR26>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NVIC_IPR26;
#[doc = "`read()` method returns [nvic_ipr26::R](nvic_ipr26::R) reader structure"]
impl crate::Readable for NVIC_IPR26 {}
#[doc = "`write(|w| ..)` method takes [nvic_ipr26::W](nvic_ipr26::W) writer structure"]
impl crate::Writable for NVIC_IPR26 {}
#[doc = "Interrupt Priority Register 26"]
pub mod nvic_ipr26;
#[doc = "Interrupt Priority Register 27\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [nvic_ipr27](nvic_ipr27) module"]
pub type NVIC_IPR27 = crate::Reg<u32, _NVIC_IPR27>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NVIC_IPR27;
#[doc = "`read()` method returns [nvic_ipr27::R](nvic_ipr27::R) reader structure"]
impl crate::Readable for NVIC_IPR27 {}
#[doc = "`write(|w| ..)` method takes [nvic_ipr27::W](nvic_ipr27::W) writer structure"]
impl crate::Writable for NVIC_IPR27 {}
#[doc = "Interrupt Priority Register 27"]
pub mod nvic_ipr27;
#[doc = "CPUID Base Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cpuid](cpuid) module"]
pub type CPUID = crate::Reg<u32, _CPUID>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CPUID;
#[doc = "`read()` method returns [cpuid::R](cpuid::R) reader structure"]
impl crate::Readable for CPUID {}
#[doc = "CPUID Base Register"]
pub mod cpuid;
#[doc = "Interrupt Control and State Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [icsr](icsr) module"]
pub type ICSR = crate::Reg<u32, _ICSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ICSR;
#[doc = "`read()` method returns [icsr::R](icsr::R) reader structure"]
impl crate::Readable for ICSR {}
#[doc = "`write(|w| ..)` method takes [icsr::W](icsr::W) writer structure"]
impl crate::Writable for ICSR {}
#[doc = "Interrupt Control and State Register"]
pub mod icsr;
#[doc = "Vector Table Offset Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [vtor](vtor) module"]
pub type VTOR = crate::Reg<u32, _VTOR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _VTOR;
#[doc = "`read()` method returns [vtor::R](vtor::R) reader structure"]
impl crate::Readable for VTOR {}
#[doc = "`write(|w| ..)` method takes [vtor::W](vtor::W) writer structure"]
impl crate::Writable for VTOR {}
#[doc = "Vector Table Offset Register"]
pub mod vtor;
#[doc = "Application Interrupt and Reset Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [aircr](aircr) module"]
pub type AIRCR = crate::Reg<u32, _AIRCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AIRCR;
#[doc = "`read()` method returns [aircr::R](aircr::R) reader structure"]
impl crate::Readable for AIRCR {}
#[doc = "`write(|w| ..)` method takes [aircr::W](aircr::W) writer structure"]
impl crate::Writable for AIRCR {}
#[doc = "Application Interrupt and Reset Control Register"]
pub mod aircr;
#[doc = "System Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [scr](scr) module"]
pub type SCR = crate::Reg<u32, _SCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCR;
#[doc = "`read()` method returns [scr::R](scr::R) reader structure"]
impl crate::Readable for SCR {}
#[doc = "`write(|w| ..)` method takes [scr::W](scr::W) writer structure"]
impl crate::Writable for SCR {}
#[doc = "System Control Register"]
pub mod scr;
#[doc = "Configuration and Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ccr](ccr) module"]
pub type CCR = crate::Reg<u32, _CCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CCR;
#[doc = "`read()` method returns [ccr::R](ccr::R) reader structure"]
impl crate::Readable for CCR {}
#[doc = "`write(|w| ..)` method takes [ccr::W](ccr::W) writer structure"]
impl crate::Writable for CCR {}
#[doc = "Configuration and Control Register"]
pub mod ccr;
#[doc = "System Handler Priority Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [shpr1](shpr1) module"]
pub type SHPR1 = crate::Reg<u32, _SHPR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SHPR1;
#[doc = "`read()` method returns [shpr1::R](shpr1::R) reader structure"]
impl crate::Readable for SHPR1 {}
#[doc = "`write(|w| ..)` method takes [shpr1::W](shpr1::W) writer structure"]
impl crate::Writable for SHPR1 {}
#[doc = "System Handler Priority Register 1"]
pub mod shpr1;
#[doc = "System Handler Priority Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [shpr2](shpr2) module"]
pub type SHPR2 = crate::Reg<u32, _SHPR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SHPR2;
#[doc = "`read()` method returns [shpr2::R](shpr2::R) reader structure"]
impl crate::Readable for SHPR2 {}
#[doc = "`write(|w| ..)` method takes [shpr2::W](shpr2::W) writer structure"]
impl crate::Writable for SHPR2 {}
#[doc = "System Handler Priority Register 2"]
pub mod shpr2;
#[doc = "System Handler Priority Register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [shpr3](shpr3) module"]
pub type SHPR3 = crate::Reg<u32, _SHPR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SHPR3;
#[doc = "`read()` method returns [shpr3::R](shpr3::R) reader structure"]
impl crate::Readable for SHPR3 {}
#[doc = "`write(|w| ..)` method takes [shpr3::W](shpr3::W) writer structure"]
impl crate::Writable for SHPR3 {}
#[doc = "System Handler Priority Register 3"]
pub mod shpr3;
#[doc = "System Handler Control and State Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [shcsr](shcsr) module"]
pub type SHCSR = crate::Reg<u32, _SHCSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SHCSR;
#[doc = "`read()` method returns [shcsr::R](shcsr::R) reader structure"]
impl crate::Readable for SHCSR {}
#[doc = "`write(|w| ..)` method takes [shcsr::W](shcsr::W) writer structure"]
impl crate::Writable for SHCSR {}
#[doc = "System Handler Control and State Register"]
pub mod shcsr;
#[doc = "Configurable Fault Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cfsr](cfsr) module"]
pub type CFSR = crate::Reg<u32, _CFSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CFSR;
#[doc = "`read()` method returns [cfsr::R](cfsr::R) reader structure"]
impl crate::Readable for CFSR {}
#[doc = "`write(|w| ..)` method takes [cfsr::W](cfsr::W) writer structure"]
impl crate::Writable for CFSR {}
#[doc = "Configurable Fault Status Register"]
pub mod cfsr;
#[doc = "HardFault Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [hfsr](hfsr) module"]
pub type HFSR = crate::Reg<u32, _HFSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HFSR;
#[doc = "`read()` method returns [hfsr::R](hfsr::R) reader structure"]
impl crate::Readable for HFSR {}
#[doc = "`write(|w| ..)` method takes [hfsr::W](hfsr::W) writer structure"]
impl crate::Writable for HFSR {}
#[doc = "HardFault Status Register"]
pub mod hfsr;
#[doc = "MemManage Fault Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mmfar](mmfar) module"]
pub type MMFAR = crate::Reg<u32, _MMFAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MMFAR;
#[doc = "`read()` method returns [mmfar::R](mmfar::R) reader structure"]
impl crate::Readable for MMFAR {}
#[doc = "`write(|w| ..)` method takes [mmfar::W](mmfar::W) writer structure"]
impl crate::Writable for MMFAR {}
#[doc = "MemManage Fault Address Register"]
pub mod mmfar;
#[doc = "BusFault Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [bfar](bfar) module"]
pub type BFAR = crate::Reg<u32, _BFAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BFAR;
#[doc = "`read()` method returns [bfar::R](bfar::R) reader structure"]
impl crate::Readable for BFAR {}
#[doc = "`write(|w| ..)` method takes [bfar::W](bfar::W) writer structure"]
impl crate::Writable for BFAR {}
#[doc = "BusFault Address Register"]
pub mod bfar;
#[doc = "Auxiliary Fault Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [afsr](afsr) module"]
pub type AFSR = crate::Reg<u32, _AFSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AFSR;
#[doc = "`read()` method returns [afsr::R](afsr::R) reader structure"]
impl crate::Readable for AFSR {}
#[doc = "`write(|w| ..)` method takes [afsr::W](afsr::W) writer structure"]
impl crate::Writable for AFSR {}
#[doc = "Auxiliary Fault Status Register"]
pub mod afsr;
#[doc = "Coprocessor Access Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cpacr](cpacr) module"]
pub type CPACR = crate::Reg<u32, _CPACR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CPACR;
#[doc = "`read()` method returns [cpacr::R](cpacr::R) reader structure"]
impl crate::Readable for CPACR {}
#[doc = "`write(|w| ..)` method takes [cpacr::W](cpacr::W) writer structure"]
impl crate::Writable for CPACR {}
#[doc = "Coprocessor Access Control Register"]
pub mod cpacr;
#[doc = "MPU Type Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mpu_type](mpu_type) module"]
pub type MPU_TYPE = crate::Reg<u32, _MPU_TYPE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MPU_TYPE;
#[doc = "`read()` method returns [mpu_type::R](mpu_type::R) reader structure"]
impl crate::Readable for MPU_TYPE {}
#[doc = "MPU Type Register"]
pub mod mpu_type;
#[doc = "MPU Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mpu_ctrl](mpu_ctrl) module"]
pub type MPU_CTRL = crate::Reg<u32, _MPU_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MPU_CTRL;
#[doc = "`read()` method returns [mpu_ctrl::R](mpu_ctrl::R) reader structure"]
impl crate::Readable for MPU_CTRL {}
#[doc = "`write(|w| ..)` method takes [mpu_ctrl::W](mpu_ctrl::W) writer structure"]
impl crate::Writable for MPU_CTRL {}
#[doc = "MPU Control Register"]
pub mod mpu_ctrl;
#[doc = "MPU Region Number Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mpu_rnr](mpu_rnr) module"]
pub type MPU_RNR = crate::Reg<u32, _MPU_RNR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MPU_RNR;
#[doc = "`read()` method returns [mpu_rnr::R](mpu_rnr::R) reader structure"]
impl crate::Readable for MPU_RNR {}
#[doc = "`write(|w| ..)` method takes [mpu_rnr::W](mpu_rnr::W) writer structure"]
impl crate::Writable for MPU_RNR {}
#[doc = "MPU Region Number Register"]
pub mod mpu_rnr;
#[doc = "MPU Region Base Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mpu_rbar](mpu_rbar) module"]
pub type MPU_RBAR = crate::Reg<u32, _MPU_RBAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MPU_RBAR;
#[doc = "`read()` method returns [mpu_rbar::R](mpu_rbar::R) reader structure"]
impl crate::Readable for MPU_RBAR {}
#[doc = "`write(|w| ..)` method takes [mpu_rbar::W](mpu_rbar::W) writer structure"]
impl crate::Writable for MPU_RBAR {}
#[doc = "MPU Region Base Address Register"]
pub mod mpu_rbar;
#[doc = "MPU Region Attribute and Size Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mpu_rasr](mpu_rasr) module"]
pub type MPU_RASR = crate::Reg<u32, _MPU_RASR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MPU_RASR;
#[doc = "`read()` method returns [mpu_rasr::R](mpu_rasr::R) reader structure"]
impl crate::Readable for MPU_RASR {}
#[doc = "`write(|w| ..)` method takes [mpu_rasr::W](mpu_rasr::W) writer structure"]
impl crate::Writable for MPU_RASR {}
#[doc = "MPU Region Attribute and Size Register"]
pub mod mpu_rasr;
#[doc = "MPU Region Base Address Register A1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mpu_rbar_a1](mpu_rbar_a1) module"]
pub type MPU_RBAR_A1 = crate::Reg<u32, _MPU_RBAR_A1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MPU_RBAR_A1;
#[doc = "`read()` method returns [mpu_rbar_a1::R](mpu_rbar_a1::R) reader structure"]
impl crate::Readable for MPU_RBAR_A1 {}
#[doc = "`write(|w| ..)` method takes [mpu_rbar_a1::W](mpu_rbar_a1::W) writer structure"]
impl crate::Writable for MPU_RBAR_A1 {}
#[doc = "MPU Region Base Address Register A1"]
pub mod mpu_rbar_a1;
#[doc = "MPU Region Attribute and Size Register A1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mpu_rasr_a1](mpu_rasr_a1) module"]
pub type MPU_RASR_A1 = crate::Reg<u32, _MPU_RASR_A1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MPU_RASR_A1;
#[doc = "`read()` method returns [mpu_rasr_a1::R](mpu_rasr_a1::R) reader structure"]
impl crate::Readable for MPU_RASR_A1 {}
#[doc = "`write(|w| ..)` method takes [mpu_rasr_a1::W](mpu_rasr_a1::W) writer structure"]
impl crate::Writable for MPU_RASR_A1 {}
#[doc = "MPU Region Attribute and Size Register A1"]
pub mod mpu_rasr_a1;
#[doc = "MPU Region Base Address Register A2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mpu_rbar_a2](mpu_rbar_a2) module"]
pub type MPU_RBAR_A2 = crate::Reg<u32, _MPU_RBAR_A2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MPU_RBAR_A2;
#[doc = "`read()` method returns [mpu_rbar_a2::R](mpu_rbar_a2::R) reader structure"]
impl crate::Readable for MPU_RBAR_A2 {}
#[doc = "`write(|w| ..)` method takes [mpu_rbar_a2::W](mpu_rbar_a2::W) writer structure"]
impl crate::Writable for MPU_RBAR_A2 {}
#[doc = "MPU Region Base Address Register A2"]
pub mod mpu_rbar_a2;
#[doc = "MPU Region Attribute and Size Register A2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mpu_rasr_a2](mpu_rasr_a2) module"]
pub type MPU_RASR_A2 = crate::Reg<u32, _MPU_RASR_A2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MPU_RASR_A2;
#[doc = "`read()` method returns [mpu_rasr_a2::R](mpu_rasr_a2::R) reader structure"]
impl crate::Readable for MPU_RASR_A2 {}
#[doc = "`write(|w| ..)` method takes [mpu_rasr_a2::W](mpu_rasr_a2::W) writer structure"]
impl crate::Writable for MPU_RASR_A2 {}
#[doc = "MPU Region Attribute and Size Register A2"]
pub mod mpu_rasr_a2;
#[doc = "MPU Region Base Address Register A3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mpu_rbar_a3](mpu_rbar_a3) module"]
pub type MPU_RBAR_A3 = crate::Reg<u32, _MPU_RBAR_A3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MPU_RBAR_A3;
#[doc = "`read()` method returns [mpu_rbar_a3::R](mpu_rbar_a3::R) reader structure"]
impl crate::Readable for MPU_RBAR_A3 {}
#[doc = "`write(|w| ..)` method takes [mpu_rbar_a3::W](mpu_rbar_a3::W) writer structure"]
impl crate::Writable for MPU_RBAR_A3 {}
#[doc = "MPU Region Base Address Register A3"]
pub mod mpu_rbar_a3;
#[doc = "MPU Region Attribute and Size Register A3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mpu_rasr_a3](mpu_rasr_a3) module"]
pub type MPU_RASR_A3 = crate::Reg<u32, _MPU_RASR_A3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MPU_RASR_A3;
#[doc = "`read()` method returns [mpu_rasr_a3::R](mpu_rasr_a3::R) reader structure"]
impl crate::Readable for MPU_RASR_A3 {}
#[doc = "`write(|w| ..)` method takes [mpu_rasr_a3::W](mpu_rasr_a3::W) writer structure"]
impl crate::Writable for MPU_RASR_A3 {}
#[doc = "MPU Region Attribute and Size Register A3"]
pub mod mpu_rasr_a3;
#[doc = "Software Trigger Interrupt Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [stir](stir) module"]
pub type STIR = crate::Reg<u32, _STIR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STIR;
#[doc = "`write(|w| ..)` method takes [stir::W](stir::W) writer structure"]
impl crate::Writable for STIR {}
#[doc = "Software Trigger Interrupt Register"]
pub mod stir;
#[doc = "Floating-point Context Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [fpccr](fpccr) module"]
pub type FPCCR = crate::Reg<u32, _FPCCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FPCCR;
#[doc = "`read()` method returns [fpccr::R](fpccr::R) reader structure"]
impl crate::Readable for FPCCR {}
#[doc = "`write(|w| ..)` method takes [fpccr::W](fpccr::W) writer structure"]
impl crate::Writable for FPCCR {}
#[doc = "Floating-point Context Control Register"]
pub mod fpccr;
#[doc = "Floating-point Context Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [fpcar](fpcar) module"]
pub type FPCAR = crate::Reg<u32, _FPCAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FPCAR;
#[doc = "`read()` method returns [fpcar::R](fpcar::R) reader structure"]
impl crate::Readable for FPCAR {}
#[doc = "`write(|w| ..)` method takes [fpcar::W](fpcar::W) writer structure"]
impl crate::Writable for FPCAR {}
#[doc = "Floating-point Context Address Register"]
pub mod fpcar;
#[doc = "Floating-point Default Status Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [fpdscr](fpdscr) module"]
pub type FPDSCR = crate::Reg<u32, _FPDSCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FPDSCR;
#[doc = "`read()` method returns [fpdscr::R](fpdscr::R) reader structure"]
impl crate::Readable for FPDSCR {}
#[doc = "`write(|w| ..)` method takes [fpdscr::W](fpdscr::W) writer structure"]
impl crate::Writable for FPDSCR {}
#[doc = "Floating-point Default Status Control Register"]
pub mod fpdscr;
