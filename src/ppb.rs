#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x08],
    actlr: ACTLR,
    _reserved1: [u8; 0x04],
    syst_csr: SYST_CSR,
    syst_rvr: SYST_RVR,
    syst_cvr: SYST_CVR,
    syst_calib: SYST_CALIB,
    _reserved5: [u8; 0xe0],
    nvic_iser0: NVIC_ISER0,
    nvic_iser1: NVIC_ISER1,
    nvic_iser2: NVIC_ISER2,
    nvic_iser3: NVIC_ISER3,
    _reserved9: [u8; 0x70],
    nvic_icer0: NVIC_ICER0,
    nvic_icer1: NVIC_ICER1,
    nvic_icer2: NVIC_ICER2,
    nvic_icer3: NVIC_ICER3,
    _reserved13: [u8; 0x70],
    nvic_ispr0: NVIC_ISPR0,
    nvic_ispr1: NVIC_ISPR1,
    nvic_ispr2: NVIC_ISPR2,
    nvic_ispr3: NVIC_ISPR3,
    _reserved17: [u8; 0x70],
    nvic_icpr0: NVIC_ICPR0,
    nvic_icpr1: NVIC_ICPR1,
    nvic_icpr2: NVIC_ICPR2,
    nvic_icpr3: NVIC_ICPR3,
    _reserved21: [u8; 0x70],
    nvic_iabr0: NVIC_IABR0,
    nvic_iabr1: NVIC_IABR1,
    nvic_iabr2: NVIC_IABR2,
    nvic_iabr3: NVIC_IABR3,
    _reserved25: [u8; 0xf0],
    nvic_ipr0: NVIC_IPR0,
    nvic_ipr1: NVIC_IPR1,
    nvic_ipr2: NVIC_IPR2,
    nvic_ipr3: NVIC_IPR3,
    nvic_ipr4: NVIC_IPR4,
    nvic_ipr5: NVIC_IPR5,
    nvic_ipr6: NVIC_IPR6,
    nvic_ipr7: NVIC_IPR7,
    nvic_ipr8: NVIC_IPR8,
    nvic_ipr9: NVIC_IPR9,
    nvic_ipr10: NVIC_IPR10,
    nvic_ipr11: NVIC_IPR11,
    nvic_ipr12: NVIC_IPR12,
    nvic_ipr13: NVIC_IPR13,
    nvic_ipr14: NVIC_IPR14,
    nvic_ipr15: NVIC_IPR15,
    nvic_ipr16: NVIC_IPR16,
    nvic_ipr17: NVIC_IPR17,
    nvic_ipr18: NVIC_IPR18,
    nvic_ipr19: NVIC_IPR19,
    nvic_ipr20: NVIC_IPR20,
    nvic_ipr21: NVIC_IPR21,
    nvic_ipr22: NVIC_IPR22,
    nvic_ipr23: NVIC_IPR23,
    nvic_ipr24: NVIC_IPR24,
    nvic_ipr25: NVIC_IPR25,
    nvic_ipr26: NVIC_IPR26,
    nvic_ipr27: NVIC_IPR27,
    _reserved53: [u8; 0x0890],
    cpuid: CPUID,
    icsr: ICSR,
    vtor: VTOR,
    aircr: AIRCR,
    scr: SCR,
    ccr: CCR,
    shpr1: SHPR1,
    shpr2: SHPR2,
    shpr3: SHPR3,
    shcsr: SHCSR,
    cfsr: CFSR,
    hfsr: HFSR,
    _reserved65: [u8; 0x04],
    mmfar: MMFAR,
    bfar: BFAR,
    afsr: AFSR,
    _reserved68: [u8; 0x48],
    cpacr: CPACR,
    _reserved69: [u8; 0x04],
    mpu_type: MPU_TYPE,
    mpu_ctrl: MPU_CTRL,
    mpu_rnr: MPU_RNR,
    mpu_rbar: MPU_RBAR,
    mpu_rasr: MPU_RASR,
    mpu_rbar_a1: MPU_RBAR_A1,
    mpu_rasr_a1: MPU_RASR_A1,
    mpu_rbar_a2: MPU_RBAR_A2,
    mpu_rasr_a2: MPU_RASR_A2,
    mpu_rbar_a3: MPU_RBAR_A3,
    mpu_rasr_a3: MPU_RASR_A3,
    _reserved80: [u8; 0x0144],
    stir: STIR,
    _reserved81: [u8; 0x30],
    fpccr: FPCCR,
    fpcar: FPCAR,
    fpdscr: FPDSCR,
}
impl RegisterBlock {
    #[doc = "0x08 - Auxiliary Control Register"]
    #[inline(always)]
    pub const fn actlr(&self) -> &ACTLR {
        &self.actlr
    }
    #[doc = "0x10 - SysTick Control and Status Register"]
    #[inline(always)]
    pub const fn syst_csr(&self) -> &SYST_CSR {
        &self.syst_csr
    }
    #[doc = "0x14 - SysTick Reload Value Register"]
    #[inline(always)]
    pub const fn syst_rvr(&self) -> &SYST_RVR {
        &self.syst_rvr
    }
    #[doc = "0x18 - SysTick Current Value Register"]
    #[inline(always)]
    pub const fn syst_cvr(&self) -> &SYST_CVR {
        &self.syst_cvr
    }
    #[doc = "0x1c - SysTick Calibration Value Register r"]
    #[inline(always)]
    pub const fn syst_calib(&self) -> &SYST_CALIB {
        &self.syst_calib
    }
    #[doc = "0x100 - Interrupt Set-enable Register 0"]
    #[inline(always)]
    pub const fn nvic_iser0(&self) -> &NVIC_ISER0 {
        &self.nvic_iser0
    }
    #[doc = "0x104 - Interrupt Set-enable Register 1"]
    #[inline(always)]
    pub const fn nvic_iser1(&self) -> &NVIC_ISER1 {
        &self.nvic_iser1
    }
    #[doc = "0x108 - Interrupt Set-enable Register 2"]
    #[inline(always)]
    pub const fn nvic_iser2(&self) -> &NVIC_ISER2 {
        &self.nvic_iser2
    }
    #[doc = "0x10c - Interrupt Set-enable Register 3"]
    #[inline(always)]
    pub const fn nvic_iser3(&self) -> &NVIC_ISER3 {
        &self.nvic_iser3
    }
    #[doc = "0x180 - Interrupt Clear-enable Register 0"]
    #[inline(always)]
    pub const fn nvic_icer0(&self) -> &NVIC_ICER0 {
        &self.nvic_icer0
    }
    #[doc = "0x184 - Interrupt Clear-enable Register 1"]
    #[inline(always)]
    pub const fn nvic_icer1(&self) -> &NVIC_ICER1 {
        &self.nvic_icer1
    }
    #[doc = "0x188 - Interrupt Clear-enable Register 2"]
    #[inline(always)]
    pub const fn nvic_icer2(&self) -> &NVIC_ICER2 {
        &self.nvic_icer2
    }
    #[doc = "0x18c - Interrupt Clear-enable Register 3"]
    #[inline(always)]
    pub const fn nvic_icer3(&self) -> &NVIC_ICER3 {
        &self.nvic_icer3
    }
    #[doc = "0x200 - Interrupt Set-pending Register 0"]
    #[inline(always)]
    pub const fn nvic_ispr0(&self) -> &NVIC_ISPR0 {
        &self.nvic_ispr0
    }
    #[doc = "0x204 - Interrupt Set-pending Register 1"]
    #[inline(always)]
    pub const fn nvic_ispr1(&self) -> &NVIC_ISPR1 {
        &self.nvic_ispr1
    }
    #[doc = "0x208 - Interrupt Set-pending Register 2"]
    #[inline(always)]
    pub const fn nvic_ispr2(&self) -> &NVIC_ISPR2 {
        &self.nvic_ispr2
    }
    #[doc = "0x20c - Interrupt Set-pending Register 3"]
    #[inline(always)]
    pub const fn nvic_ispr3(&self) -> &NVIC_ISPR3 {
        &self.nvic_ispr3
    }
    #[doc = "0x280 - Interrupt Clear-pending Register 0"]
    #[inline(always)]
    pub const fn nvic_icpr0(&self) -> &NVIC_ICPR0 {
        &self.nvic_icpr0
    }
    #[doc = "0x284 - Interrupt Clear-pending Register 1"]
    #[inline(always)]
    pub const fn nvic_icpr1(&self) -> &NVIC_ICPR1 {
        &self.nvic_icpr1
    }
    #[doc = "0x288 - Interrupt Clear-pending Register 2"]
    #[inline(always)]
    pub const fn nvic_icpr2(&self) -> &NVIC_ICPR2 {
        &self.nvic_icpr2
    }
    #[doc = "0x28c - Interrupt Clear-pending Register 3"]
    #[inline(always)]
    pub const fn nvic_icpr3(&self) -> &NVIC_ICPR3 {
        &self.nvic_icpr3
    }
    #[doc = "0x300 - Interrupt Active Bit Register 0"]
    #[inline(always)]
    pub const fn nvic_iabr0(&self) -> &NVIC_IABR0 {
        &self.nvic_iabr0
    }
    #[doc = "0x304 - Interrupt Active Bit Register 1"]
    #[inline(always)]
    pub const fn nvic_iabr1(&self) -> &NVIC_IABR1 {
        &self.nvic_iabr1
    }
    #[doc = "0x308 - Interrupt Active Bit Register 2"]
    #[inline(always)]
    pub const fn nvic_iabr2(&self) -> &NVIC_IABR2 {
        &self.nvic_iabr2
    }
    #[doc = "0x30c - Interrupt Active Bit Register 3"]
    #[inline(always)]
    pub const fn nvic_iabr3(&self) -> &NVIC_IABR3 {
        &self.nvic_iabr3
    }
    #[doc = "0x400 - Interrupt Priority Register 0"]
    #[inline(always)]
    pub const fn nvic_ipr0(&self) -> &NVIC_IPR0 {
        &self.nvic_ipr0
    }
    #[doc = "0x404 - Interrupt Priority Register 1"]
    #[inline(always)]
    pub const fn nvic_ipr1(&self) -> &NVIC_IPR1 {
        &self.nvic_ipr1
    }
    #[doc = "0x408 - Interrupt Priority Register 2"]
    #[inline(always)]
    pub const fn nvic_ipr2(&self) -> &NVIC_IPR2 {
        &self.nvic_ipr2
    }
    #[doc = "0x40c - Interrupt Priority Register 3"]
    #[inline(always)]
    pub const fn nvic_ipr3(&self) -> &NVIC_IPR3 {
        &self.nvic_ipr3
    }
    #[doc = "0x410 - Interrupt Priority Register 4"]
    #[inline(always)]
    pub const fn nvic_ipr4(&self) -> &NVIC_IPR4 {
        &self.nvic_ipr4
    }
    #[doc = "0x414 - Interrupt Priority Register 5"]
    #[inline(always)]
    pub const fn nvic_ipr5(&self) -> &NVIC_IPR5 {
        &self.nvic_ipr5
    }
    #[doc = "0x418 - Interrupt Priority Register 6"]
    #[inline(always)]
    pub const fn nvic_ipr6(&self) -> &NVIC_IPR6 {
        &self.nvic_ipr6
    }
    #[doc = "0x41c - Interrupt Priority Register 7"]
    #[inline(always)]
    pub const fn nvic_ipr7(&self) -> &NVIC_IPR7 {
        &self.nvic_ipr7
    }
    #[doc = "0x420 - Interrupt Priority Register 8"]
    #[inline(always)]
    pub const fn nvic_ipr8(&self) -> &NVIC_IPR8 {
        &self.nvic_ipr8
    }
    #[doc = "0x424 - Interrupt Priority Register 9"]
    #[inline(always)]
    pub const fn nvic_ipr9(&self) -> &NVIC_IPR9 {
        &self.nvic_ipr9
    }
    #[doc = "0x428 - Interrupt Priority Register 10"]
    #[inline(always)]
    pub const fn nvic_ipr10(&self) -> &NVIC_IPR10 {
        &self.nvic_ipr10
    }
    #[doc = "0x42c - Interrupt Priority Register 11"]
    #[inline(always)]
    pub const fn nvic_ipr11(&self) -> &NVIC_IPR11 {
        &self.nvic_ipr11
    }
    #[doc = "0x430 - Interrupt Priority Register 12"]
    #[inline(always)]
    pub const fn nvic_ipr12(&self) -> &NVIC_IPR12 {
        &self.nvic_ipr12
    }
    #[doc = "0x434 - Interrupt Priority Register 13"]
    #[inline(always)]
    pub const fn nvic_ipr13(&self) -> &NVIC_IPR13 {
        &self.nvic_ipr13
    }
    #[doc = "0x438 - Interrupt Priority Register 14"]
    #[inline(always)]
    pub const fn nvic_ipr14(&self) -> &NVIC_IPR14 {
        &self.nvic_ipr14
    }
    #[doc = "0x43c - Interrupt Priority Register 15"]
    #[inline(always)]
    pub const fn nvic_ipr15(&self) -> &NVIC_IPR15 {
        &self.nvic_ipr15
    }
    #[doc = "0x440 - Interrupt Priority Register 16"]
    #[inline(always)]
    pub const fn nvic_ipr16(&self) -> &NVIC_IPR16 {
        &self.nvic_ipr16
    }
    #[doc = "0x444 - Interrupt Priority Register 17"]
    #[inline(always)]
    pub const fn nvic_ipr17(&self) -> &NVIC_IPR17 {
        &self.nvic_ipr17
    }
    #[doc = "0x448 - Interrupt Priority Register 18"]
    #[inline(always)]
    pub const fn nvic_ipr18(&self) -> &NVIC_IPR18 {
        &self.nvic_ipr18
    }
    #[doc = "0x44c - Interrupt Priority Register 19"]
    #[inline(always)]
    pub const fn nvic_ipr19(&self) -> &NVIC_IPR19 {
        &self.nvic_ipr19
    }
    #[doc = "0x450 - Interrupt Priority Register 20"]
    #[inline(always)]
    pub const fn nvic_ipr20(&self) -> &NVIC_IPR20 {
        &self.nvic_ipr20
    }
    #[doc = "0x454 - Interrupt Priority Register 21"]
    #[inline(always)]
    pub const fn nvic_ipr21(&self) -> &NVIC_IPR21 {
        &self.nvic_ipr21
    }
    #[doc = "0x458 - Interrupt Priority Register 22"]
    #[inline(always)]
    pub const fn nvic_ipr22(&self) -> &NVIC_IPR22 {
        &self.nvic_ipr22
    }
    #[doc = "0x45c - Interrupt Priority Register 23"]
    #[inline(always)]
    pub const fn nvic_ipr23(&self) -> &NVIC_IPR23 {
        &self.nvic_ipr23
    }
    #[doc = "0x460 - Interrupt Priority Register 24"]
    #[inline(always)]
    pub const fn nvic_ipr24(&self) -> &NVIC_IPR24 {
        &self.nvic_ipr24
    }
    #[doc = "0x464 - Interrupt Priority Register 25"]
    #[inline(always)]
    pub const fn nvic_ipr25(&self) -> &NVIC_IPR25 {
        &self.nvic_ipr25
    }
    #[doc = "0x468 - Interrupt Priority Register 26"]
    #[inline(always)]
    pub const fn nvic_ipr26(&self) -> &NVIC_IPR26 {
        &self.nvic_ipr26
    }
    #[doc = "0x46c - Interrupt Priority Register 27"]
    #[inline(always)]
    pub const fn nvic_ipr27(&self) -> &NVIC_IPR27 {
        &self.nvic_ipr27
    }
    #[doc = "0xd00 - CPUID Base Register"]
    #[inline(always)]
    pub const fn cpuid(&self) -> &CPUID {
        &self.cpuid
    }
    #[doc = "0xd04 - Interrupt Control and State Register"]
    #[inline(always)]
    pub const fn icsr(&self) -> &ICSR {
        &self.icsr
    }
    #[doc = "0xd08 - Vector Table Offset Register"]
    #[inline(always)]
    pub const fn vtor(&self) -> &VTOR {
        &self.vtor
    }
    #[doc = "0xd0c - Application Interrupt and Reset Control Register"]
    #[inline(always)]
    pub const fn aircr(&self) -> &AIRCR {
        &self.aircr
    }
    #[doc = "0xd10 - System Control Register"]
    #[inline(always)]
    pub const fn scr(&self) -> &SCR {
        &self.scr
    }
    #[doc = "0xd14 - Configuration and Control Register"]
    #[inline(always)]
    pub const fn ccr(&self) -> &CCR {
        &self.ccr
    }
    #[doc = "0xd18 - System Handler Priority Register 1"]
    #[inline(always)]
    pub const fn shpr1(&self) -> &SHPR1 {
        &self.shpr1
    }
    #[doc = "0xd1c - System Handler Priority Register 2"]
    #[inline(always)]
    pub const fn shpr2(&self) -> &SHPR2 {
        &self.shpr2
    }
    #[doc = "0xd20 - System Handler Priority Register 3"]
    #[inline(always)]
    pub const fn shpr3(&self) -> &SHPR3 {
        &self.shpr3
    }
    #[doc = "0xd24 - System Handler Control and State Register"]
    #[inline(always)]
    pub const fn shcsr(&self) -> &SHCSR {
        &self.shcsr
    }
    #[doc = "0xd28 - Configurable Fault Status Register"]
    #[inline(always)]
    pub const fn cfsr(&self) -> &CFSR {
        &self.cfsr
    }
    #[doc = "0xd2c - HardFault Status Register"]
    #[inline(always)]
    pub const fn hfsr(&self) -> &HFSR {
        &self.hfsr
    }
    #[doc = "0xd34 - MemManage Fault Address Register"]
    #[inline(always)]
    pub const fn mmfar(&self) -> &MMFAR {
        &self.mmfar
    }
    #[doc = "0xd38 - BusFault Address Register"]
    #[inline(always)]
    pub const fn bfar(&self) -> &BFAR {
        &self.bfar
    }
    #[doc = "0xd3c - Auxiliary Fault Status Register"]
    #[inline(always)]
    pub const fn afsr(&self) -> &AFSR {
        &self.afsr
    }
    #[doc = "0xd88 - Coprocessor Access Control Register"]
    #[inline(always)]
    pub const fn cpacr(&self) -> &CPACR {
        &self.cpacr
    }
    #[doc = "0xd90 - MPU Type Register"]
    #[inline(always)]
    pub const fn mpu_type(&self) -> &MPU_TYPE {
        &self.mpu_type
    }
    #[doc = "0xd94 - MPU Control Register"]
    #[inline(always)]
    pub const fn mpu_ctrl(&self) -> &MPU_CTRL {
        &self.mpu_ctrl
    }
    #[doc = "0xd98 - MPU Region Number Register"]
    #[inline(always)]
    pub const fn mpu_rnr(&self) -> &MPU_RNR {
        &self.mpu_rnr
    }
    #[doc = "0xd9c - MPU Region Base Address Register"]
    #[inline(always)]
    pub const fn mpu_rbar(&self) -> &MPU_RBAR {
        &self.mpu_rbar
    }
    #[doc = "0xda0 - MPU Region Attribute and Size Register"]
    #[inline(always)]
    pub const fn mpu_rasr(&self) -> &MPU_RASR {
        &self.mpu_rasr
    }
    #[doc = "0xda4 - MPU Region Base Address Register A1"]
    #[inline(always)]
    pub const fn mpu_rbar_a1(&self) -> &MPU_RBAR_A1 {
        &self.mpu_rbar_a1
    }
    #[doc = "0xda8 - MPU Region Attribute and Size Register A1"]
    #[inline(always)]
    pub const fn mpu_rasr_a1(&self) -> &MPU_RASR_A1 {
        &self.mpu_rasr_a1
    }
    #[doc = "0xdac - MPU Region Base Address Register A2"]
    #[inline(always)]
    pub const fn mpu_rbar_a2(&self) -> &MPU_RBAR_A2 {
        &self.mpu_rbar_a2
    }
    #[doc = "0xdb0 - MPU Region Attribute and Size Register A2"]
    #[inline(always)]
    pub const fn mpu_rasr_a2(&self) -> &MPU_RASR_A2 {
        &self.mpu_rasr_a2
    }
    #[doc = "0xdb4 - MPU Region Base Address Register A3"]
    #[inline(always)]
    pub const fn mpu_rbar_a3(&self) -> &MPU_RBAR_A3 {
        &self.mpu_rbar_a3
    }
    #[doc = "0xdb8 - MPU Region Attribute and Size Register A3"]
    #[inline(always)]
    pub const fn mpu_rasr_a3(&self) -> &MPU_RASR_A3 {
        &self.mpu_rasr_a3
    }
    #[doc = "0xf00 - Software Trigger Interrupt Register"]
    #[inline(always)]
    pub const fn stir(&self) -> &STIR {
        &self.stir
    }
    #[doc = "0xf34 - Floating-point Context Control Register"]
    #[inline(always)]
    pub const fn fpccr(&self) -> &FPCCR {
        &self.fpccr
    }
    #[doc = "0xf38 - Floating-point Context Address Register"]
    #[inline(always)]
    pub const fn fpcar(&self) -> &FPCAR {
        &self.fpcar
    }
    #[doc = "0xf3c - Floating-point Default Status Control Register"]
    #[inline(always)]
    pub const fn fpdscr(&self) -> &FPDSCR {
        &self.fpdscr
    }
}
#[doc = "ACTLR (rw) register accessor: Auxiliary Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`actlr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`actlr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@actlr`]
module"]
pub type ACTLR = crate::Reg<actlr::ACTLR_SPEC>;
#[doc = "Auxiliary Control Register"]
pub mod actlr;
#[doc = "SYST_CSR (rw) register accessor: SysTick Control and Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`syst_csr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`syst_csr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@syst_csr`]
module"]
pub type SYST_CSR = crate::Reg<syst_csr::SYST_CSR_SPEC>;
#[doc = "SysTick Control and Status Register"]
pub mod syst_csr;
#[doc = "SYST_RVR (rw) register accessor: SysTick Reload Value Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`syst_rvr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`syst_rvr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@syst_rvr`]
module"]
pub type SYST_RVR = crate::Reg<syst_rvr::SYST_RVR_SPEC>;
#[doc = "SysTick Reload Value Register"]
pub mod syst_rvr;
#[doc = "SYST_CVR (rw) register accessor: SysTick Current Value Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`syst_cvr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`syst_cvr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@syst_cvr`]
module"]
pub type SYST_CVR = crate::Reg<syst_cvr::SYST_CVR_SPEC>;
#[doc = "SysTick Current Value Register"]
pub mod syst_cvr;
#[doc = "SYST_CALIB (rw) register accessor: SysTick Calibration Value Register r\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`syst_calib::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`syst_calib::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@syst_calib`]
module"]
pub type SYST_CALIB = crate::Reg<syst_calib::SYST_CALIB_SPEC>;
#[doc = "SysTick Calibration Value Register r"]
pub mod syst_calib;
#[doc = "NVIC_ISER0 (rw) register accessor: Interrupt Set-enable Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`nvic_iser0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`nvic_iser0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nvic_iser0`]
module"]
pub type NVIC_ISER0 = crate::Reg<nvic_iser0::NVIC_ISER0_SPEC>;
#[doc = "Interrupt Set-enable Register 0"]
pub mod nvic_iser0;
#[doc = "NVIC_ISER1 (rw) register accessor: Interrupt Set-enable Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`nvic_iser1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`nvic_iser1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nvic_iser1`]
module"]
pub type NVIC_ISER1 = crate::Reg<nvic_iser1::NVIC_ISER1_SPEC>;
#[doc = "Interrupt Set-enable Register 1"]
pub mod nvic_iser1;
#[doc = "NVIC_ISER2 (rw) register accessor: Interrupt Set-enable Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`nvic_iser2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`nvic_iser2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nvic_iser2`]
module"]
pub type NVIC_ISER2 = crate::Reg<nvic_iser2::NVIC_ISER2_SPEC>;
#[doc = "Interrupt Set-enable Register 2"]
pub mod nvic_iser2;
#[doc = "NVIC_ISER3 (rw) register accessor: Interrupt Set-enable Register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`nvic_iser3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`nvic_iser3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nvic_iser3`]
module"]
pub type NVIC_ISER3 = crate::Reg<nvic_iser3::NVIC_ISER3_SPEC>;
#[doc = "Interrupt Set-enable Register 3"]
pub mod nvic_iser3;
#[doc = "NVIC_ICER0 (rw) register accessor: Interrupt Clear-enable Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`nvic_icer0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`nvic_icer0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nvic_icer0`]
module"]
pub type NVIC_ICER0 = crate::Reg<nvic_icer0::NVIC_ICER0_SPEC>;
#[doc = "Interrupt Clear-enable Register 0"]
pub mod nvic_icer0;
#[doc = "NVIC_ICER1 (rw) register accessor: Interrupt Clear-enable Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`nvic_icer1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`nvic_icer1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nvic_icer1`]
module"]
pub type NVIC_ICER1 = crate::Reg<nvic_icer1::NVIC_ICER1_SPEC>;
#[doc = "Interrupt Clear-enable Register 1"]
pub mod nvic_icer1;
#[doc = "NVIC_ICER2 (rw) register accessor: Interrupt Clear-enable Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`nvic_icer2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`nvic_icer2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nvic_icer2`]
module"]
pub type NVIC_ICER2 = crate::Reg<nvic_icer2::NVIC_ICER2_SPEC>;
#[doc = "Interrupt Clear-enable Register 2"]
pub mod nvic_icer2;
#[doc = "NVIC_ICER3 (rw) register accessor: Interrupt Clear-enable Register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`nvic_icer3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`nvic_icer3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nvic_icer3`]
module"]
pub type NVIC_ICER3 = crate::Reg<nvic_icer3::NVIC_ICER3_SPEC>;
#[doc = "Interrupt Clear-enable Register 3"]
pub mod nvic_icer3;
#[doc = "NVIC_ISPR0 (rw) register accessor: Interrupt Set-pending Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`nvic_ispr0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`nvic_ispr0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nvic_ispr0`]
module"]
pub type NVIC_ISPR0 = crate::Reg<nvic_ispr0::NVIC_ISPR0_SPEC>;
#[doc = "Interrupt Set-pending Register 0"]
pub mod nvic_ispr0;
#[doc = "NVIC_ISPR1 (rw) register accessor: Interrupt Set-pending Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`nvic_ispr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`nvic_ispr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nvic_ispr1`]
module"]
pub type NVIC_ISPR1 = crate::Reg<nvic_ispr1::NVIC_ISPR1_SPEC>;
#[doc = "Interrupt Set-pending Register 1"]
pub mod nvic_ispr1;
#[doc = "NVIC_ISPR2 (rw) register accessor: Interrupt Set-pending Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`nvic_ispr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`nvic_ispr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nvic_ispr2`]
module"]
pub type NVIC_ISPR2 = crate::Reg<nvic_ispr2::NVIC_ISPR2_SPEC>;
#[doc = "Interrupt Set-pending Register 2"]
pub mod nvic_ispr2;
#[doc = "NVIC_ISPR3 (rw) register accessor: Interrupt Set-pending Register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`nvic_ispr3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`nvic_ispr3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nvic_ispr3`]
module"]
pub type NVIC_ISPR3 = crate::Reg<nvic_ispr3::NVIC_ISPR3_SPEC>;
#[doc = "Interrupt Set-pending Register 3"]
pub mod nvic_ispr3;
#[doc = "NVIC_ICPR0 (rw) register accessor: Interrupt Clear-pending Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`nvic_icpr0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`nvic_icpr0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nvic_icpr0`]
module"]
pub type NVIC_ICPR0 = crate::Reg<nvic_icpr0::NVIC_ICPR0_SPEC>;
#[doc = "Interrupt Clear-pending Register 0"]
pub mod nvic_icpr0;
#[doc = "NVIC_ICPR1 (rw) register accessor: Interrupt Clear-pending Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`nvic_icpr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`nvic_icpr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nvic_icpr1`]
module"]
pub type NVIC_ICPR1 = crate::Reg<nvic_icpr1::NVIC_ICPR1_SPEC>;
#[doc = "Interrupt Clear-pending Register 1"]
pub mod nvic_icpr1;
#[doc = "NVIC_ICPR2 (rw) register accessor: Interrupt Clear-pending Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`nvic_icpr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`nvic_icpr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nvic_icpr2`]
module"]
pub type NVIC_ICPR2 = crate::Reg<nvic_icpr2::NVIC_ICPR2_SPEC>;
#[doc = "Interrupt Clear-pending Register 2"]
pub mod nvic_icpr2;
#[doc = "NVIC_ICPR3 (rw) register accessor: Interrupt Clear-pending Register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`nvic_icpr3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`nvic_icpr3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nvic_icpr3`]
module"]
pub type NVIC_ICPR3 = crate::Reg<nvic_icpr3::NVIC_ICPR3_SPEC>;
#[doc = "Interrupt Clear-pending Register 3"]
pub mod nvic_icpr3;
#[doc = "NVIC_IABR0 (rw) register accessor: Interrupt Active Bit Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`nvic_iabr0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`nvic_iabr0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nvic_iabr0`]
module"]
pub type NVIC_IABR0 = crate::Reg<nvic_iabr0::NVIC_IABR0_SPEC>;
#[doc = "Interrupt Active Bit Register 0"]
pub mod nvic_iabr0;
#[doc = "NVIC_IABR1 (rw) register accessor: Interrupt Active Bit Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`nvic_iabr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`nvic_iabr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nvic_iabr1`]
module"]
pub type NVIC_IABR1 = crate::Reg<nvic_iabr1::NVIC_IABR1_SPEC>;
#[doc = "Interrupt Active Bit Register 1"]
pub mod nvic_iabr1;
#[doc = "NVIC_IABR2 (rw) register accessor: Interrupt Active Bit Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`nvic_iabr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`nvic_iabr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nvic_iabr2`]
module"]
pub type NVIC_IABR2 = crate::Reg<nvic_iabr2::NVIC_IABR2_SPEC>;
#[doc = "Interrupt Active Bit Register 2"]
pub mod nvic_iabr2;
#[doc = "NVIC_IABR3 (rw) register accessor: Interrupt Active Bit Register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`nvic_iabr3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`nvic_iabr3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nvic_iabr3`]
module"]
pub type NVIC_IABR3 = crate::Reg<nvic_iabr3::NVIC_IABR3_SPEC>;
#[doc = "Interrupt Active Bit Register 3"]
pub mod nvic_iabr3;
#[doc = "NVIC_IPR0 (rw) register accessor: Interrupt Priority Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`nvic_ipr0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`nvic_ipr0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nvic_ipr0`]
module"]
pub type NVIC_IPR0 = crate::Reg<nvic_ipr0::NVIC_IPR0_SPEC>;
#[doc = "Interrupt Priority Register 0"]
pub mod nvic_ipr0;
#[doc = "NVIC_IPR1 (rw) register accessor: Interrupt Priority Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`nvic_ipr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`nvic_ipr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nvic_ipr1`]
module"]
pub type NVIC_IPR1 = crate::Reg<nvic_ipr1::NVIC_IPR1_SPEC>;
#[doc = "Interrupt Priority Register 1"]
pub mod nvic_ipr1;
#[doc = "NVIC_IPR2 (rw) register accessor: Interrupt Priority Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`nvic_ipr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`nvic_ipr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nvic_ipr2`]
module"]
pub type NVIC_IPR2 = crate::Reg<nvic_ipr2::NVIC_IPR2_SPEC>;
#[doc = "Interrupt Priority Register 2"]
pub mod nvic_ipr2;
#[doc = "NVIC_IPR3 (rw) register accessor: Interrupt Priority Register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`nvic_ipr3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`nvic_ipr3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nvic_ipr3`]
module"]
pub type NVIC_IPR3 = crate::Reg<nvic_ipr3::NVIC_IPR3_SPEC>;
#[doc = "Interrupt Priority Register 3"]
pub mod nvic_ipr3;
#[doc = "NVIC_IPR4 (rw) register accessor: Interrupt Priority Register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`nvic_ipr4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`nvic_ipr4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nvic_ipr4`]
module"]
pub type NVIC_IPR4 = crate::Reg<nvic_ipr4::NVIC_IPR4_SPEC>;
#[doc = "Interrupt Priority Register 4"]
pub mod nvic_ipr4;
#[doc = "NVIC_IPR5 (rw) register accessor: Interrupt Priority Register 5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`nvic_ipr5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`nvic_ipr5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nvic_ipr5`]
module"]
pub type NVIC_IPR5 = crate::Reg<nvic_ipr5::NVIC_IPR5_SPEC>;
#[doc = "Interrupt Priority Register 5"]
pub mod nvic_ipr5;
#[doc = "NVIC_IPR6 (rw) register accessor: Interrupt Priority Register 6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`nvic_ipr6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`nvic_ipr6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nvic_ipr6`]
module"]
pub type NVIC_IPR6 = crate::Reg<nvic_ipr6::NVIC_IPR6_SPEC>;
#[doc = "Interrupt Priority Register 6"]
pub mod nvic_ipr6;
#[doc = "NVIC_IPR7 (rw) register accessor: Interrupt Priority Register 7\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`nvic_ipr7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`nvic_ipr7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nvic_ipr7`]
module"]
pub type NVIC_IPR7 = crate::Reg<nvic_ipr7::NVIC_IPR7_SPEC>;
#[doc = "Interrupt Priority Register 7"]
pub mod nvic_ipr7;
#[doc = "NVIC_IPR8 (rw) register accessor: Interrupt Priority Register 8\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`nvic_ipr8::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`nvic_ipr8::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nvic_ipr8`]
module"]
pub type NVIC_IPR8 = crate::Reg<nvic_ipr8::NVIC_IPR8_SPEC>;
#[doc = "Interrupt Priority Register 8"]
pub mod nvic_ipr8;
#[doc = "NVIC_IPR9 (rw) register accessor: Interrupt Priority Register 9\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`nvic_ipr9::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`nvic_ipr9::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nvic_ipr9`]
module"]
pub type NVIC_IPR9 = crate::Reg<nvic_ipr9::NVIC_IPR9_SPEC>;
#[doc = "Interrupt Priority Register 9"]
pub mod nvic_ipr9;
#[doc = "NVIC_IPR10 (rw) register accessor: Interrupt Priority Register 10\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`nvic_ipr10::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`nvic_ipr10::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nvic_ipr10`]
module"]
pub type NVIC_IPR10 = crate::Reg<nvic_ipr10::NVIC_IPR10_SPEC>;
#[doc = "Interrupt Priority Register 10"]
pub mod nvic_ipr10;
#[doc = "NVIC_IPR11 (rw) register accessor: Interrupt Priority Register 11\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`nvic_ipr11::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`nvic_ipr11::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nvic_ipr11`]
module"]
pub type NVIC_IPR11 = crate::Reg<nvic_ipr11::NVIC_IPR11_SPEC>;
#[doc = "Interrupt Priority Register 11"]
pub mod nvic_ipr11;
#[doc = "NVIC_IPR12 (rw) register accessor: Interrupt Priority Register 12\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`nvic_ipr12::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`nvic_ipr12::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nvic_ipr12`]
module"]
pub type NVIC_IPR12 = crate::Reg<nvic_ipr12::NVIC_IPR12_SPEC>;
#[doc = "Interrupt Priority Register 12"]
pub mod nvic_ipr12;
#[doc = "NVIC_IPR13 (rw) register accessor: Interrupt Priority Register 13\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`nvic_ipr13::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`nvic_ipr13::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nvic_ipr13`]
module"]
pub type NVIC_IPR13 = crate::Reg<nvic_ipr13::NVIC_IPR13_SPEC>;
#[doc = "Interrupt Priority Register 13"]
pub mod nvic_ipr13;
#[doc = "NVIC_IPR14 (rw) register accessor: Interrupt Priority Register 14\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`nvic_ipr14::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`nvic_ipr14::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nvic_ipr14`]
module"]
pub type NVIC_IPR14 = crate::Reg<nvic_ipr14::NVIC_IPR14_SPEC>;
#[doc = "Interrupt Priority Register 14"]
pub mod nvic_ipr14;
#[doc = "NVIC_IPR15 (rw) register accessor: Interrupt Priority Register 15\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`nvic_ipr15::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`nvic_ipr15::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nvic_ipr15`]
module"]
pub type NVIC_IPR15 = crate::Reg<nvic_ipr15::NVIC_IPR15_SPEC>;
#[doc = "Interrupt Priority Register 15"]
pub mod nvic_ipr15;
#[doc = "NVIC_IPR16 (rw) register accessor: Interrupt Priority Register 16\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`nvic_ipr16::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`nvic_ipr16::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nvic_ipr16`]
module"]
pub type NVIC_IPR16 = crate::Reg<nvic_ipr16::NVIC_IPR16_SPEC>;
#[doc = "Interrupt Priority Register 16"]
pub mod nvic_ipr16;
#[doc = "NVIC_IPR17 (rw) register accessor: Interrupt Priority Register 17\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`nvic_ipr17::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`nvic_ipr17::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nvic_ipr17`]
module"]
pub type NVIC_IPR17 = crate::Reg<nvic_ipr17::NVIC_IPR17_SPEC>;
#[doc = "Interrupt Priority Register 17"]
pub mod nvic_ipr17;
#[doc = "NVIC_IPR18 (rw) register accessor: Interrupt Priority Register 18\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`nvic_ipr18::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`nvic_ipr18::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nvic_ipr18`]
module"]
pub type NVIC_IPR18 = crate::Reg<nvic_ipr18::NVIC_IPR18_SPEC>;
#[doc = "Interrupt Priority Register 18"]
pub mod nvic_ipr18;
#[doc = "NVIC_IPR19 (rw) register accessor: Interrupt Priority Register 19\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`nvic_ipr19::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`nvic_ipr19::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nvic_ipr19`]
module"]
pub type NVIC_IPR19 = crate::Reg<nvic_ipr19::NVIC_IPR19_SPEC>;
#[doc = "Interrupt Priority Register 19"]
pub mod nvic_ipr19;
#[doc = "NVIC_IPR20 (rw) register accessor: Interrupt Priority Register 20\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`nvic_ipr20::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`nvic_ipr20::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nvic_ipr20`]
module"]
pub type NVIC_IPR20 = crate::Reg<nvic_ipr20::NVIC_IPR20_SPEC>;
#[doc = "Interrupt Priority Register 20"]
pub mod nvic_ipr20;
#[doc = "NVIC_IPR21 (rw) register accessor: Interrupt Priority Register 21\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`nvic_ipr21::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`nvic_ipr21::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nvic_ipr21`]
module"]
pub type NVIC_IPR21 = crate::Reg<nvic_ipr21::NVIC_IPR21_SPEC>;
#[doc = "Interrupt Priority Register 21"]
pub mod nvic_ipr21;
#[doc = "NVIC_IPR22 (rw) register accessor: Interrupt Priority Register 22\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`nvic_ipr22::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`nvic_ipr22::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nvic_ipr22`]
module"]
pub type NVIC_IPR22 = crate::Reg<nvic_ipr22::NVIC_IPR22_SPEC>;
#[doc = "Interrupt Priority Register 22"]
pub mod nvic_ipr22;
#[doc = "NVIC_IPR23 (rw) register accessor: Interrupt Priority Register 23\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`nvic_ipr23::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`nvic_ipr23::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nvic_ipr23`]
module"]
pub type NVIC_IPR23 = crate::Reg<nvic_ipr23::NVIC_IPR23_SPEC>;
#[doc = "Interrupt Priority Register 23"]
pub mod nvic_ipr23;
#[doc = "NVIC_IPR24 (rw) register accessor: Interrupt Priority Register 24\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`nvic_ipr24::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`nvic_ipr24::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nvic_ipr24`]
module"]
pub type NVIC_IPR24 = crate::Reg<nvic_ipr24::NVIC_IPR24_SPEC>;
#[doc = "Interrupt Priority Register 24"]
pub mod nvic_ipr24;
#[doc = "NVIC_IPR25 (rw) register accessor: Interrupt Priority Register 25\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`nvic_ipr25::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`nvic_ipr25::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nvic_ipr25`]
module"]
pub type NVIC_IPR25 = crate::Reg<nvic_ipr25::NVIC_IPR25_SPEC>;
#[doc = "Interrupt Priority Register 25"]
pub mod nvic_ipr25;
#[doc = "NVIC_IPR26 (rw) register accessor: Interrupt Priority Register 26\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`nvic_ipr26::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`nvic_ipr26::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nvic_ipr26`]
module"]
pub type NVIC_IPR26 = crate::Reg<nvic_ipr26::NVIC_IPR26_SPEC>;
#[doc = "Interrupt Priority Register 26"]
pub mod nvic_ipr26;
#[doc = "NVIC_IPR27 (rw) register accessor: Interrupt Priority Register 27\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`nvic_ipr27::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`nvic_ipr27::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nvic_ipr27`]
module"]
pub type NVIC_IPR27 = crate::Reg<nvic_ipr27::NVIC_IPR27_SPEC>;
#[doc = "Interrupt Priority Register 27"]
pub mod nvic_ipr27;
#[doc = "CPUID (r) register accessor: CPUID Base Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpuid::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpuid`]
module"]
pub type CPUID = crate::Reg<cpuid::CPUID_SPEC>;
#[doc = "CPUID Base Register"]
pub mod cpuid;
#[doc = "ICSR (rw) register accessor: Interrupt Control and State Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`icsr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`icsr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icsr`]
module"]
pub type ICSR = crate::Reg<icsr::ICSR_SPEC>;
#[doc = "Interrupt Control and State Register"]
pub mod icsr;
#[doc = "VTOR (rw) register accessor: Vector Table Offset Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vtor::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vtor::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vtor`]
module"]
pub type VTOR = crate::Reg<vtor::VTOR_SPEC>;
#[doc = "Vector Table Offset Register"]
pub mod vtor;
#[doc = "AIRCR (rw) register accessor: Application Interrupt and Reset Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aircr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aircr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aircr`]
module"]
pub type AIRCR = crate::Reg<aircr::AIRCR_SPEC>;
#[doc = "Application Interrupt and Reset Control Register"]
pub mod aircr;
#[doc = "SCR (rw) register accessor: System Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`scr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scr`]
module"]
pub type SCR = crate::Reg<scr::SCR_SPEC>;
#[doc = "System Control Register"]
pub mod scr;
#[doc = "CCR (rw) register accessor: Configuration and Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccr`]
module"]
pub type CCR = crate::Reg<ccr::CCR_SPEC>;
#[doc = "Configuration and Control Register"]
pub mod ccr;
#[doc = "SHPR1 (rw) register accessor: System Handler Priority Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`shpr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`shpr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@shpr1`]
module"]
pub type SHPR1 = crate::Reg<shpr1::SHPR1_SPEC>;
#[doc = "System Handler Priority Register 1"]
pub mod shpr1;
#[doc = "SHPR2 (rw) register accessor: System Handler Priority Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`shpr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`shpr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@shpr2`]
module"]
pub type SHPR2 = crate::Reg<shpr2::SHPR2_SPEC>;
#[doc = "System Handler Priority Register 2"]
pub mod shpr2;
#[doc = "SHPR3 (rw) register accessor: System Handler Priority Register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`shpr3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`shpr3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@shpr3`]
module"]
pub type SHPR3 = crate::Reg<shpr3::SHPR3_SPEC>;
#[doc = "System Handler Priority Register 3"]
pub mod shpr3;
#[doc = "SHCSR (rw) register accessor: System Handler Control and State Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`shcsr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`shcsr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@shcsr`]
module"]
pub type SHCSR = crate::Reg<shcsr::SHCSR_SPEC>;
#[doc = "System Handler Control and State Register"]
pub mod shcsr;
#[doc = "CFSR (rw) register accessor: Configurable Fault Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfsr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfsr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfsr`]
module"]
pub type CFSR = crate::Reg<cfsr::CFSR_SPEC>;
#[doc = "Configurable Fault Status Register"]
pub mod cfsr;
#[doc = "HFSR (rw) register accessor: HardFault Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hfsr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hfsr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hfsr`]
module"]
pub type HFSR = crate::Reg<hfsr::HFSR_SPEC>;
#[doc = "HardFault Status Register"]
pub mod hfsr;
#[doc = "MMFAR (rw) register accessor: MemManage Fault Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmfar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mmfar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mmfar`]
module"]
pub type MMFAR = crate::Reg<mmfar::MMFAR_SPEC>;
#[doc = "MemManage Fault Address Register"]
pub mod mmfar;
#[doc = "BFAR (rw) register accessor: BusFault Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bfar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bfar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bfar`]
module"]
pub type BFAR = crate::Reg<bfar::BFAR_SPEC>;
#[doc = "BusFault Address Register"]
pub mod bfar;
#[doc = "AFSR (rw) register accessor: Auxiliary Fault Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`afsr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`afsr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@afsr`]
module"]
pub type AFSR = crate::Reg<afsr::AFSR_SPEC>;
#[doc = "Auxiliary Fault Status Register"]
pub mod afsr;
#[doc = "CPACR (rw) register accessor: Coprocessor Access Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpacr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpacr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpacr`]
module"]
pub type CPACR = crate::Reg<cpacr::CPACR_SPEC>;
#[doc = "Coprocessor Access Control Register"]
pub mod cpacr;
#[doc = "MPU_TYPE (r) register accessor: MPU Type Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mpu_type::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpu_type`]
module"]
pub type MPU_TYPE = crate::Reg<mpu_type::MPU_TYPE_SPEC>;
#[doc = "MPU Type Register"]
pub mod mpu_type;
#[doc = "MPU_CTRL (rw) register accessor: MPU Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mpu_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mpu_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpu_ctrl`]
module"]
pub type MPU_CTRL = crate::Reg<mpu_ctrl::MPU_CTRL_SPEC>;
#[doc = "MPU Control Register"]
pub mod mpu_ctrl;
#[doc = "MPU_RNR (rw) register accessor: MPU Region Number Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mpu_rnr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mpu_rnr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpu_rnr`]
module"]
pub type MPU_RNR = crate::Reg<mpu_rnr::MPU_RNR_SPEC>;
#[doc = "MPU Region Number Register"]
pub mod mpu_rnr;
#[doc = "MPU_RBAR (rw) register accessor: MPU Region Base Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mpu_rbar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mpu_rbar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpu_rbar`]
module"]
pub type MPU_RBAR = crate::Reg<mpu_rbar::MPU_RBAR_SPEC>;
#[doc = "MPU Region Base Address Register"]
pub mod mpu_rbar;
#[doc = "MPU_RASR (rw) register accessor: MPU Region Attribute and Size Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mpu_rasr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mpu_rasr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpu_rasr`]
module"]
pub type MPU_RASR = crate::Reg<mpu_rasr::MPU_RASR_SPEC>;
#[doc = "MPU Region Attribute and Size Register"]
pub mod mpu_rasr;
#[doc = "MPU_RBAR_A1 (rw) register accessor: MPU Region Base Address Register A1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mpu_rbar_a1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mpu_rbar_a1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpu_rbar_a1`]
module"]
pub type MPU_RBAR_A1 = crate::Reg<mpu_rbar_a1::MPU_RBAR_A1_SPEC>;
#[doc = "MPU Region Base Address Register A1"]
pub mod mpu_rbar_a1;
#[doc = "MPU_RASR_A1 (rw) register accessor: MPU Region Attribute and Size Register A1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mpu_rasr_a1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mpu_rasr_a1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpu_rasr_a1`]
module"]
pub type MPU_RASR_A1 = crate::Reg<mpu_rasr_a1::MPU_RASR_A1_SPEC>;
#[doc = "MPU Region Attribute and Size Register A1"]
pub mod mpu_rasr_a1;
#[doc = "MPU_RBAR_A2 (rw) register accessor: MPU Region Base Address Register A2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mpu_rbar_a2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mpu_rbar_a2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpu_rbar_a2`]
module"]
pub type MPU_RBAR_A2 = crate::Reg<mpu_rbar_a2::MPU_RBAR_A2_SPEC>;
#[doc = "MPU Region Base Address Register A2"]
pub mod mpu_rbar_a2;
#[doc = "MPU_RASR_A2 (rw) register accessor: MPU Region Attribute and Size Register A2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mpu_rasr_a2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mpu_rasr_a2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpu_rasr_a2`]
module"]
pub type MPU_RASR_A2 = crate::Reg<mpu_rasr_a2::MPU_RASR_A2_SPEC>;
#[doc = "MPU Region Attribute and Size Register A2"]
pub mod mpu_rasr_a2;
#[doc = "MPU_RBAR_A3 (rw) register accessor: MPU Region Base Address Register A3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mpu_rbar_a3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mpu_rbar_a3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpu_rbar_a3`]
module"]
pub type MPU_RBAR_A3 = crate::Reg<mpu_rbar_a3::MPU_RBAR_A3_SPEC>;
#[doc = "MPU Region Base Address Register A3"]
pub mod mpu_rbar_a3;
#[doc = "MPU_RASR_A3 (rw) register accessor: MPU Region Attribute and Size Register A3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mpu_rasr_a3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mpu_rasr_a3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpu_rasr_a3`]
module"]
pub type MPU_RASR_A3 = crate::Reg<mpu_rasr_a3::MPU_RASR_A3_SPEC>;
#[doc = "MPU Region Attribute and Size Register A3"]
pub mod mpu_rasr_a3;
#[doc = "STIR (w) register accessor: Software Trigger Interrupt Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stir::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stir`]
module"]
pub type STIR = crate::Reg<stir::STIR_SPEC>;
#[doc = "Software Trigger Interrupt Register"]
pub mod stir;
#[doc = "FPCCR (rw) register accessor: Floating-point Context Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fpccr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fpccr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fpccr`]
module"]
pub type FPCCR = crate::Reg<fpccr::FPCCR_SPEC>;
#[doc = "Floating-point Context Control Register"]
pub mod fpccr;
#[doc = "FPCAR (rw) register accessor: Floating-point Context Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fpcar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fpcar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fpcar`]
module"]
pub type FPCAR = crate::Reg<fpcar::FPCAR_SPEC>;
#[doc = "Floating-point Context Address Register"]
pub mod fpcar;
#[doc = "FPDSCR (rw) register accessor: Floating-point Default Status Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fpdscr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fpdscr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fpdscr`]
module"]
pub type FPDSCR = crate::Reg<fpdscr::FPDSCR_SPEC>;
#[doc = "Floating-point Default Status Control Register"]
pub mod fpdscr;
