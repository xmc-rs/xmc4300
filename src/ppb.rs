#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x08],
    actlr: Actlr,
    _reserved1: [u8; 0x04],
    syst_csr: SystCsr,
    syst_rvr: SystRvr,
    syst_cvr: SystCvr,
    syst_calib: SystCalib,
    _reserved5: [u8; 0xe0],
    nvic_iser0: NvicIser0,
    nvic_iser1: NvicIser1,
    nvic_iser2: NvicIser2,
    nvic_iser3: NvicIser3,
    _reserved9: [u8; 0x70],
    nvic_icer0: NvicIcer0,
    nvic_icer1: NvicIcer1,
    nvic_icer2: NvicIcer2,
    nvic_icer3: NvicIcer3,
    _reserved13: [u8; 0x70],
    nvic_ispr0: NvicIspr0,
    nvic_ispr1: NvicIspr1,
    nvic_ispr2: NvicIspr2,
    nvic_ispr3: NvicIspr3,
    _reserved17: [u8; 0x70],
    nvic_icpr0: NvicIcpr0,
    nvic_icpr1: NvicIcpr1,
    nvic_icpr2: NvicIcpr2,
    nvic_icpr3: NvicIcpr3,
    _reserved21: [u8; 0x70],
    nvic_iabr0: NvicIabr0,
    nvic_iabr1: NvicIabr1,
    nvic_iabr2: NvicIabr2,
    nvic_iabr3: NvicIabr3,
    _reserved25: [u8; 0xf0],
    nvic_ipr0: NvicIpr0,
    nvic_ipr1: NvicIpr1,
    nvic_ipr2: NvicIpr2,
    nvic_ipr3: NvicIpr3,
    nvic_ipr4: NvicIpr4,
    nvic_ipr5: NvicIpr5,
    nvic_ipr6: NvicIpr6,
    nvic_ipr7: NvicIpr7,
    nvic_ipr8: NvicIpr8,
    nvic_ipr9: NvicIpr9,
    nvic_ipr10: NvicIpr10,
    nvic_ipr11: NvicIpr11,
    nvic_ipr12: NvicIpr12,
    nvic_ipr13: NvicIpr13,
    nvic_ipr14: NvicIpr14,
    nvic_ipr15: NvicIpr15,
    nvic_ipr16: NvicIpr16,
    nvic_ipr17: NvicIpr17,
    nvic_ipr18: NvicIpr18,
    nvic_ipr19: NvicIpr19,
    nvic_ipr20: NvicIpr20,
    nvic_ipr21: NvicIpr21,
    nvic_ipr22: NvicIpr22,
    nvic_ipr23: NvicIpr23,
    nvic_ipr24: NvicIpr24,
    nvic_ipr25: NvicIpr25,
    nvic_ipr26: NvicIpr26,
    nvic_ipr27: NvicIpr27,
    _reserved53: [u8; 0x0890],
    cpuid: Cpuid,
    icsr: Icsr,
    vtor: Vtor,
    aircr: Aircr,
    scr: Scr,
    ccr: Ccr,
    shpr1: Shpr1,
    shpr2: Shpr2,
    shpr3: Shpr3,
    shcsr: Shcsr,
    cfsr: Cfsr,
    hfsr: Hfsr,
    _reserved65: [u8; 0x04],
    mmfar: Mmfar,
    bfar: Bfar,
    afsr: Afsr,
    _reserved68: [u8; 0x48],
    cpacr: Cpacr,
    _reserved69: [u8; 0x04],
    mpu_type: MpuType,
    mpu_ctrl: MpuCtrl,
    mpu_rnr: MpuRnr,
    mpu_rbar: MpuRbar,
    mpu_rasr: MpuRasr,
    mpu_rbar_a1: MpuRbarA1,
    mpu_rasr_a1: MpuRasrA1,
    mpu_rbar_a2: MpuRbarA2,
    mpu_rasr_a2: MpuRasrA2,
    mpu_rbar_a3: MpuRbarA3,
    mpu_rasr_a3: MpuRasrA3,
    _reserved80: [u8; 0x0144],
    stir: Stir,
    _reserved81: [u8; 0x30],
    fpccr: Fpccr,
    fpcar: Fpcar,
    fpdscr: Fpdscr,
}
impl RegisterBlock {
    #[doc = "0x08 - Auxiliary Control Register"]
    #[inline(always)]
    pub const fn actlr(&self) -> &Actlr {
        &self.actlr
    }
    #[doc = "0x10 - SysTick Control and Status Register"]
    #[inline(always)]
    pub const fn syst_csr(&self) -> &SystCsr {
        &self.syst_csr
    }
    #[doc = "0x14 - SysTick Reload Value Register"]
    #[inline(always)]
    pub const fn syst_rvr(&self) -> &SystRvr {
        &self.syst_rvr
    }
    #[doc = "0x18 - SysTick Current Value Register"]
    #[inline(always)]
    pub const fn syst_cvr(&self) -> &SystCvr {
        &self.syst_cvr
    }
    #[doc = "0x1c - SysTick Calibration Value Register r"]
    #[inline(always)]
    pub const fn syst_calib(&self) -> &SystCalib {
        &self.syst_calib
    }
    #[doc = "0x100 - Interrupt Set-enable Register 0"]
    #[inline(always)]
    pub const fn nvic_iser0(&self) -> &NvicIser0 {
        &self.nvic_iser0
    }
    #[doc = "0x104 - Interrupt Set-enable Register 1"]
    #[inline(always)]
    pub const fn nvic_iser1(&self) -> &NvicIser1 {
        &self.nvic_iser1
    }
    #[doc = "0x108 - Interrupt Set-enable Register 2"]
    #[inline(always)]
    pub const fn nvic_iser2(&self) -> &NvicIser2 {
        &self.nvic_iser2
    }
    #[doc = "0x10c - Interrupt Set-enable Register 3"]
    #[inline(always)]
    pub const fn nvic_iser3(&self) -> &NvicIser3 {
        &self.nvic_iser3
    }
    #[doc = "0x180 - Interrupt Clear-enable Register 0"]
    #[inline(always)]
    pub const fn nvic_icer0(&self) -> &NvicIcer0 {
        &self.nvic_icer0
    }
    #[doc = "0x184 - Interrupt Clear-enable Register 1"]
    #[inline(always)]
    pub const fn nvic_icer1(&self) -> &NvicIcer1 {
        &self.nvic_icer1
    }
    #[doc = "0x188 - Interrupt Clear-enable Register 2"]
    #[inline(always)]
    pub const fn nvic_icer2(&self) -> &NvicIcer2 {
        &self.nvic_icer2
    }
    #[doc = "0x18c - Interrupt Clear-enable Register 3"]
    #[inline(always)]
    pub const fn nvic_icer3(&self) -> &NvicIcer3 {
        &self.nvic_icer3
    }
    #[doc = "0x200 - Interrupt Set-pending Register 0"]
    #[inline(always)]
    pub const fn nvic_ispr0(&self) -> &NvicIspr0 {
        &self.nvic_ispr0
    }
    #[doc = "0x204 - Interrupt Set-pending Register 1"]
    #[inline(always)]
    pub const fn nvic_ispr1(&self) -> &NvicIspr1 {
        &self.nvic_ispr1
    }
    #[doc = "0x208 - Interrupt Set-pending Register 2"]
    #[inline(always)]
    pub const fn nvic_ispr2(&self) -> &NvicIspr2 {
        &self.nvic_ispr2
    }
    #[doc = "0x20c - Interrupt Set-pending Register 3"]
    #[inline(always)]
    pub const fn nvic_ispr3(&self) -> &NvicIspr3 {
        &self.nvic_ispr3
    }
    #[doc = "0x280 - Interrupt Clear-pending Register 0"]
    #[inline(always)]
    pub const fn nvic_icpr0(&self) -> &NvicIcpr0 {
        &self.nvic_icpr0
    }
    #[doc = "0x284 - Interrupt Clear-pending Register 1"]
    #[inline(always)]
    pub const fn nvic_icpr1(&self) -> &NvicIcpr1 {
        &self.nvic_icpr1
    }
    #[doc = "0x288 - Interrupt Clear-pending Register 2"]
    #[inline(always)]
    pub const fn nvic_icpr2(&self) -> &NvicIcpr2 {
        &self.nvic_icpr2
    }
    #[doc = "0x28c - Interrupt Clear-pending Register 3"]
    #[inline(always)]
    pub const fn nvic_icpr3(&self) -> &NvicIcpr3 {
        &self.nvic_icpr3
    }
    #[doc = "0x300 - Interrupt Active Bit Register 0"]
    #[inline(always)]
    pub const fn nvic_iabr0(&self) -> &NvicIabr0 {
        &self.nvic_iabr0
    }
    #[doc = "0x304 - Interrupt Active Bit Register 1"]
    #[inline(always)]
    pub const fn nvic_iabr1(&self) -> &NvicIabr1 {
        &self.nvic_iabr1
    }
    #[doc = "0x308 - Interrupt Active Bit Register 2"]
    #[inline(always)]
    pub const fn nvic_iabr2(&self) -> &NvicIabr2 {
        &self.nvic_iabr2
    }
    #[doc = "0x30c - Interrupt Active Bit Register 3"]
    #[inline(always)]
    pub const fn nvic_iabr3(&self) -> &NvicIabr3 {
        &self.nvic_iabr3
    }
    #[doc = "0x400 - Interrupt Priority Register 0"]
    #[inline(always)]
    pub const fn nvic_ipr0(&self) -> &NvicIpr0 {
        &self.nvic_ipr0
    }
    #[doc = "0x404 - Interrupt Priority Register 1"]
    #[inline(always)]
    pub const fn nvic_ipr1(&self) -> &NvicIpr1 {
        &self.nvic_ipr1
    }
    #[doc = "0x408 - Interrupt Priority Register 2"]
    #[inline(always)]
    pub const fn nvic_ipr2(&self) -> &NvicIpr2 {
        &self.nvic_ipr2
    }
    #[doc = "0x40c - Interrupt Priority Register 3"]
    #[inline(always)]
    pub const fn nvic_ipr3(&self) -> &NvicIpr3 {
        &self.nvic_ipr3
    }
    #[doc = "0x410 - Interrupt Priority Register 4"]
    #[inline(always)]
    pub const fn nvic_ipr4(&self) -> &NvicIpr4 {
        &self.nvic_ipr4
    }
    #[doc = "0x414 - Interrupt Priority Register 5"]
    #[inline(always)]
    pub const fn nvic_ipr5(&self) -> &NvicIpr5 {
        &self.nvic_ipr5
    }
    #[doc = "0x418 - Interrupt Priority Register 6"]
    #[inline(always)]
    pub const fn nvic_ipr6(&self) -> &NvicIpr6 {
        &self.nvic_ipr6
    }
    #[doc = "0x41c - Interrupt Priority Register 7"]
    #[inline(always)]
    pub const fn nvic_ipr7(&self) -> &NvicIpr7 {
        &self.nvic_ipr7
    }
    #[doc = "0x420 - Interrupt Priority Register 8"]
    #[inline(always)]
    pub const fn nvic_ipr8(&self) -> &NvicIpr8 {
        &self.nvic_ipr8
    }
    #[doc = "0x424 - Interrupt Priority Register 9"]
    #[inline(always)]
    pub const fn nvic_ipr9(&self) -> &NvicIpr9 {
        &self.nvic_ipr9
    }
    #[doc = "0x428 - Interrupt Priority Register 10"]
    #[inline(always)]
    pub const fn nvic_ipr10(&self) -> &NvicIpr10 {
        &self.nvic_ipr10
    }
    #[doc = "0x42c - Interrupt Priority Register 11"]
    #[inline(always)]
    pub const fn nvic_ipr11(&self) -> &NvicIpr11 {
        &self.nvic_ipr11
    }
    #[doc = "0x430 - Interrupt Priority Register 12"]
    #[inline(always)]
    pub const fn nvic_ipr12(&self) -> &NvicIpr12 {
        &self.nvic_ipr12
    }
    #[doc = "0x434 - Interrupt Priority Register 13"]
    #[inline(always)]
    pub const fn nvic_ipr13(&self) -> &NvicIpr13 {
        &self.nvic_ipr13
    }
    #[doc = "0x438 - Interrupt Priority Register 14"]
    #[inline(always)]
    pub const fn nvic_ipr14(&self) -> &NvicIpr14 {
        &self.nvic_ipr14
    }
    #[doc = "0x43c - Interrupt Priority Register 15"]
    #[inline(always)]
    pub const fn nvic_ipr15(&self) -> &NvicIpr15 {
        &self.nvic_ipr15
    }
    #[doc = "0x440 - Interrupt Priority Register 16"]
    #[inline(always)]
    pub const fn nvic_ipr16(&self) -> &NvicIpr16 {
        &self.nvic_ipr16
    }
    #[doc = "0x444 - Interrupt Priority Register 17"]
    #[inline(always)]
    pub const fn nvic_ipr17(&self) -> &NvicIpr17 {
        &self.nvic_ipr17
    }
    #[doc = "0x448 - Interrupt Priority Register 18"]
    #[inline(always)]
    pub const fn nvic_ipr18(&self) -> &NvicIpr18 {
        &self.nvic_ipr18
    }
    #[doc = "0x44c - Interrupt Priority Register 19"]
    #[inline(always)]
    pub const fn nvic_ipr19(&self) -> &NvicIpr19 {
        &self.nvic_ipr19
    }
    #[doc = "0x450 - Interrupt Priority Register 20"]
    #[inline(always)]
    pub const fn nvic_ipr20(&self) -> &NvicIpr20 {
        &self.nvic_ipr20
    }
    #[doc = "0x454 - Interrupt Priority Register 21"]
    #[inline(always)]
    pub const fn nvic_ipr21(&self) -> &NvicIpr21 {
        &self.nvic_ipr21
    }
    #[doc = "0x458 - Interrupt Priority Register 22"]
    #[inline(always)]
    pub const fn nvic_ipr22(&self) -> &NvicIpr22 {
        &self.nvic_ipr22
    }
    #[doc = "0x45c - Interrupt Priority Register 23"]
    #[inline(always)]
    pub const fn nvic_ipr23(&self) -> &NvicIpr23 {
        &self.nvic_ipr23
    }
    #[doc = "0x460 - Interrupt Priority Register 24"]
    #[inline(always)]
    pub const fn nvic_ipr24(&self) -> &NvicIpr24 {
        &self.nvic_ipr24
    }
    #[doc = "0x464 - Interrupt Priority Register 25"]
    #[inline(always)]
    pub const fn nvic_ipr25(&self) -> &NvicIpr25 {
        &self.nvic_ipr25
    }
    #[doc = "0x468 - Interrupt Priority Register 26"]
    #[inline(always)]
    pub const fn nvic_ipr26(&self) -> &NvicIpr26 {
        &self.nvic_ipr26
    }
    #[doc = "0x46c - Interrupt Priority Register 27"]
    #[inline(always)]
    pub const fn nvic_ipr27(&self) -> &NvicIpr27 {
        &self.nvic_ipr27
    }
    #[doc = "0xd00 - CPUID Base Register"]
    #[inline(always)]
    pub const fn cpuid(&self) -> &Cpuid {
        &self.cpuid
    }
    #[doc = "0xd04 - Interrupt Control and State Register"]
    #[inline(always)]
    pub const fn icsr(&self) -> &Icsr {
        &self.icsr
    }
    #[doc = "0xd08 - Vector Table Offset Register"]
    #[inline(always)]
    pub const fn vtor(&self) -> &Vtor {
        &self.vtor
    }
    #[doc = "0xd0c - Application Interrupt and Reset Control Register"]
    #[inline(always)]
    pub const fn aircr(&self) -> &Aircr {
        &self.aircr
    }
    #[doc = "0xd10 - System Control Register"]
    #[inline(always)]
    pub const fn scr(&self) -> &Scr {
        &self.scr
    }
    #[doc = "0xd14 - Configuration and Control Register"]
    #[inline(always)]
    pub const fn ccr(&self) -> &Ccr {
        &self.ccr
    }
    #[doc = "0xd18 - System Handler Priority Register 1"]
    #[inline(always)]
    pub const fn shpr1(&self) -> &Shpr1 {
        &self.shpr1
    }
    #[doc = "0xd1c - System Handler Priority Register 2"]
    #[inline(always)]
    pub const fn shpr2(&self) -> &Shpr2 {
        &self.shpr2
    }
    #[doc = "0xd20 - System Handler Priority Register 3"]
    #[inline(always)]
    pub const fn shpr3(&self) -> &Shpr3 {
        &self.shpr3
    }
    #[doc = "0xd24 - System Handler Control and State Register"]
    #[inline(always)]
    pub const fn shcsr(&self) -> &Shcsr {
        &self.shcsr
    }
    #[doc = "0xd28 - Configurable Fault Status Register"]
    #[inline(always)]
    pub const fn cfsr(&self) -> &Cfsr {
        &self.cfsr
    }
    #[doc = "0xd2c - HardFault Status Register"]
    #[inline(always)]
    pub const fn hfsr(&self) -> &Hfsr {
        &self.hfsr
    }
    #[doc = "0xd34 - MemManage Fault Address Register"]
    #[inline(always)]
    pub const fn mmfar(&self) -> &Mmfar {
        &self.mmfar
    }
    #[doc = "0xd38 - BusFault Address Register"]
    #[inline(always)]
    pub const fn bfar(&self) -> &Bfar {
        &self.bfar
    }
    #[doc = "0xd3c - Auxiliary Fault Status Register"]
    #[inline(always)]
    pub const fn afsr(&self) -> &Afsr {
        &self.afsr
    }
    #[doc = "0xd88 - Coprocessor Access Control Register"]
    #[inline(always)]
    pub const fn cpacr(&self) -> &Cpacr {
        &self.cpacr
    }
    #[doc = "0xd90 - MPU Type Register"]
    #[inline(always)]
    pub const fn mpu_type(&self) -> &MpuType {
        &self.mpu_type
    }
    #[doc = "0xd94 - MPU Control Register"]
    #[inline(always)]
    pub const fn mpu_ctrl(&self) -> &MpuCtrl {
        &self.mpu_ctrl
    }
    #[doc = "0xd98 - MPU Region Number Register"]
    #[inline(always)]
    pub const fn mpu_rnr(&self) -> &MpuRnr {
        &self.mpu_rnr
    }
    #[doc = "0xd9c - MPU Region Base Address Register"]
    #[inline(always)]
    pub const fn mpu_rbar(&self) -> &MpuRbar {
        &self.mpu_rbar
    }
    #[doc = "0xda0 - MPU Region Attribute and Size Register"]
    #[inline(always)]
    pub const fn mpu_rasr(&self) -> &MpuRasr {
        &self.mpu_rasr
    }
    #[doc = "0xda4 - MPU Region Base Address Register A1"]
    #[inline(always)]
    pub const fn mpu_rbar_a1(&self) -> &MpuRbarA1 {
        &self.mpu_rbar_a1
    }
    #[doc = "0xda8 - MPU Region Attribute and Size Register A1"]
    #[inline(always)]
    pub const fn mpu_rasr_a1(&self) -> &MpuRasrA1 {
        &self.mpu_rasr_a1
    }
    #[doc = "0xdac - MPU Region Base Address Register A2"]
    #[inline(always)]
    pub const fn mpu_rbar_a2(&self) -> &MpuRbarA2 {
        &self.mpu_rbar_a2
    }
    #[doc = "0xdb0 - MPU Region Attribute and Size Register A2"]
    #[inline(always)]
    pub const fn mpu_rasr_a2(&self) -> &MpuRasrA2 {
        &self.mpu_rasr_a2
    }
    #[doc = "0xdb4 - MPU Region Base Address Register A3"]
    #[inline(always)]
    pub const fn mpu_rbar_a3(&self) -> &MpuRbarA3 {
        &self.mpu_rbar_a3
    }
    #[doc = "0xdb8 - MPU Region Attribute and Size Register A3"]
    #[inline(always)]
    pub const fn mpu_rasr_a3(&self) -> &MpuRasrA3 {
        &self.mpu_rasr_a3
    }
    #[doc = "0xf00 - Software Trigger Interrupt Register"]
    #[inline(always)]
    pub const fn stir(&self) -> &Stir {
        &self.stir
    }
    #[doc = "0xf34 - Floating-point Context Control Register"]
    #[inline(always)]
    pub const fn fpccr(&self) -> &Fpccr {
        &self.fpccr
    }
    #[doc = "0xf38 - Floating-point Context Address Register"]
    #[inline(always)]
    pub const fn fpcar(&self) -> &Fpcar {
        &self.fpcar
    }
    #[doc = "0xf3c - Floating-point Default Status Control Register"]
    #[inline(always)]
    pub const fn fpdscr(&self) -> &Fpdscr {
        &self.fpdscr
    }
}
#[doc = "ACTLR (rw) register accessor: Auxiliary Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`actlr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`actlr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@actlr`]
module"]
#[doc(alias = "ACTLR")]
pub type Actlr = crate::Reg<actlr::ActlrSpec>;
#[doc = "Auxiliary Control Register"]
pub mod actlr;
#[doc = "SYST_CSR (rw) register accessor: SysTick Control and Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`syst_csr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`syst_csr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@syst_csr`]
module"]
#[doc(alias = "SYST_CSR")]
pub type SystCsr = crate::Reg<syst_csr::SystCsrSpec>;
#[doc = "SysTick Control and Status Register"]
pub mod syst_csr;
#[doc = "SYST_RVR (rw) register accessor: SysTick Reload Value Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`syst_rvr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`syst_rvr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@syst_rvr`]
module"]
#[doc(alias = "SYST_RVR")]
pub type SystRvr = crate::Reg<syst_rvr::SystRvrSpec>;
#[doc = "SysTick Reload Value Register"]
pub mod syst_rvr;
#[doc = "SYST_CVR (rw) register accessor: SysTick Current Value Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`syst_cvr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`syst_cvr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@syst_cvr`]
module"]
#[doc(alias = "SYST_CVR")]
pub type SystCvr = crate::Reg<syst_cvr::SystCvrSpec>;
#[doc = "SysTick Current Value Register"]
pub mod syst_cvr;
#[doc = "SYST_CALIB (rw) register accessor: SysTick Calibration Value Register r\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`syst_calib::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`syst_calib::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@syst_calib`]
module"]
#[doc(alias = "SYST_CALIB")]
pub type SystCalib = crate::Reg<syst_calib::SystCalibSpec>;
#[doc = "SysTick Calibration Value Register r"]
pub mod syst_calib;
#[doc = "NVIC_ISER0 (rw) register accessor: Interrupt Set-enable Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`nvic_iser0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`nvic_iser0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nvic_iser0`]
module"]
#[doc(alias = "NVIC_ISER0")]
pub type NvicIser0 = crate::Reg<nvic_iser0::NvicIser0Spec>;
#[doc = "Interrupt Set-enable Register 0"]
pub mod nvic_iser0;
#[doc = "NVIC_ISER1 (rw) register accessor: Interrupt Set-enable Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`nvic_iser1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`nvic_iser1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nvic_iser1`]
module"]
#[doc(alias = "NVIC_ISER1")]
pub type NvicIser1 = crate::Reg<nvic_iser1::NvicIser1Spec>;
#[doc = "Interrupt Set-enable Register 1"]
pub mod nvic_iser1;
#[doc = "NVIC_ISER2 (rw) register accessor: Interrupt Set-enable Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`nvic_iser2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`nvic_iser2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nvic_iser2`]
module"]
#[doc(alias = "NVIC_ISER2")]
pub type NvicIser2 = crate::Reg<nvic_iser2::NvicIser2Spec>;
#[doc = "Interrupt Set-enable Register 2"]
pub mod nvic_iser2;
#[doc = "NVIC_ISER3 (rw) register accessor: Interrupt Set-enable Register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`nvic_iser3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`nvic_iser3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nvic_iser3`]
module"]
#[doc(alias = "NVIC_ISER3")]
pub type NvicIser3 = crate::Reg<nvic_iser3::NvicIser3Spec>;
#[doc = "Interrupt Set-enable Register 3"]
pub mod nvic_iser3;
#[doc = "NVIC_ICER0 (rw) register accessor: Interrupt Clear-enable Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`nvic_icer0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`nvic_icer0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nvic_icer0`]
module"]
#[doc(alias = "NVIC_ICER0")]
pub type NvicIcer0 = crate::Reg<nvic_icer0::NvicIcer0Spec>;
#[doc = "Interrupt Clear-enable Register 0"]
pub mod nvic_icer0;
#[doc = "NVIC_ICER1 (rw) register accessor: Interrupt Clear-enable Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`nvic_icer1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`nvic_icer1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nvic_icer1`]
module"]
#[doc(alias = "NVIC_ICER1")]
pub type NvicIcer1 = crate::Reg<nvic_icer1::NvicIcer1Spec>;
#[doc = "Interrupt Clear-enable Register 1"]
pub mod nvic_icer1;
#[doc = "NVIC_ICER2 (rw) register accessor: Interrupt Clear-enable Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`nvic_icer2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`nvic_icer2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nvic_icer2`]
module"]
#[doc(alias = "NVIC_ICER2")]
pub type NvicIcer2 = crate::Reg<nvic_icer2::NvicIcer2Spec>;
#[doc = "Interrupt Clear-enable Register 2"]
pub mod nvic_icer2;
#[doc = "NVIC_ICER3 (rw) register accessor: Interrupt Clear-enable Register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`nvic_icer3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`nvic_icer3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nvic_icer3`]
module"]
#[doc(alias = "NVIC_ICER3")]
pub type NvicIcer3 = crate::Reg<nvic_icer3::NvicIcer3Spec>;
#[doc = "Interrupt Clear-enable Register 3"]
pub mod nvic_icer3;
#[doc = "NVIC_ISPR0 (rw) register accessor: Interrupt Set-pending Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`nvic_ispr0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`nvic_ispr0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nvic_ispr0`]
module"]
#[doc(alias = "NVIC_ISPR0")]
pub type NvicIspr0 = crate::Reg<nvic_ispr0::NvicIspr0Spec>;
#[doc = "Interrupt Set-pending Register 0"]
pub mod nvic_ispr0;
#[doc = "NVIC_ISPR1 (rw) register accessor: Interrupt Set-pending Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`nvic_ispr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`nvic_ispr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nvic_ispr1`]
module"]
#[doc(alias = "NVIC_ISPR1")]
pub type NvicIspr1 = crate::Reg<nvic_ispr1::NvicIspr1Spec>;
#[doc = "Interrupt Set-pending Register 1"]
pub mod nvic_ispr1;
#[doc = "NVIC_ISPR2 (rw) register accessor: Interrupt Set-pending Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`nvic_ispr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`nvic_ispr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nvic_ispr2`]
module"]
#[doc(alias = "NVIC_ISPR2")]
pub type NvicIspr2 = crate::Reg<nvic_ispr2::NvicIspr2Spec>;
#[doc = "Interrupt Set-pending Register 2"]
pub mod nvic_ispr2;
#[doc = "NVIC_ISPR3 (rw) register accessor: Interrupt Set-pending Register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`nvic_ispr3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`nvic_ispr3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nvic_ispr3`]
module"]
#[doc(alias = "NVIC_ISPR3")]
pub type NvicIspr3 = crate::Reg<nvic_ispr3::NvicIspr3Spec>;
#[doc = "Interrupt Set-pending Register 3"]
pub mod nvic_ispr3;
#[doc = "NVIC_ICPR0 (rw) register accessor: Interrupt Clear-pending Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`nvic_icpr0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`nvic_icpr0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nvic_icpr0`]
module"]
#[doc(alias = "NVIC_ICPR0")]
pub type NvicIcpr0 = crate::Reg<nvic_icpr0::NvicIcpr0Spec>;
#[doc = "Interrupt Clear-pending Register 0"]
pub mod nvic_icpr0;
#[doc = "NVIC_ICPR1 (rw) register accessor: Interrupt Clear-pending Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`nvic_icpr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`nvic_icpr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nvic_icpr1`]
module"]
#[doc(alias = "NVIC_ICPR1")]
pub type NvicIcpr1 = crate::Reg<nvic_icpr1::NvicIcpr1Spec>;
#[doc = "Interrupt Clear-pending Register 1"]
pub mod nvic_icpr1;
#[doc = "NVIC_ICPR2 (rw) register accessor: Interrupt Clear-pending Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`nvic_icpr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`nvic_icpr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nvic_icpr2`]
module"]
#[doc(alias = "NVIC_ICPR2")]
pub type NvicIcpr2 = crate::Reg<nvic_icpr2::NvicIcpr2Spec>;
#[doc = "Interrupt Clear-pending Register 2"]
pub mod nvic_icpr2;
#[doc = "NVIC_ICPR3 (rw) register accessor: Interrupt Clear-pending Register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`nvic_icpr3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`nvic_icpr3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nvic_icpr3`]
module"]
#[doc(alias = "NVIC_ICPR3")]
pub type NvicIcpr3 = crate::Reg<nvic_icpr3::NvicIcpr3Spec>;
#[doc = "Interrupt Clear-pending Register 3"]
pub mod nvic_icpr3;
#[doc = "NVIC_IABR0 (rw) register accessor: Interrupt Active Bit Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`nvic_iabr0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`nvic_iabr0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nvic_iabr0`]
module"]
#[doc(alias = "NVIC_IABR0")]
pub type NvicIabr0 = crate::Reg<nvic_iabr0::NvicIabr0Spec>;
#[doc = "Interrupt Active Bit Register 0"]
pub mod nvic_iabr0;
#[doc = "NVIC_IABR1 (rw) register accessor: Interrupt Active Bit Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`nvic_iabr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`nvic_iabr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nvic_iabr1`]
module"]
#[doc(alias = "NVIC_IABR1")]
pub type NvicIabr1 = crate::Reg<nvic_iabr1::NvicIabr1Spec>;
#[doc = "Interrupt Active Bit Register 1"]
pub mod nvic_iabr1;
#[doc = "NVIC_IABR2 (rw) register accessor: Interrupt Active Bit Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`nvic_iabr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`nvic_iabr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nvic_iabr2`]
module"]
#[doc(alias = "NVIC_IABR2")]
pub type NvicIabr2 = crate::Reg<nvic_iabr2::NvicIabr2Spec>;
#[doc = "Interrupt Active Bit Register 2"]
pub mod nvic_iabr2;
#[doc = "NVIC_IABR3 (rw) register accessor: Interrupt Active Bit Register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`nvic_iabr3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`nvic_iabr3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nvic_iabr3`]
module"]
#[doc(alias = "NVIC_IABR3")]
pub type NvicIabr3 = crate::Reg<nvic_iabr3::NvicIabr3Spec>;
#[doc = "Interrupt Active Bit Register 3"]
pub mod nvic_iabr3;
#[doc = "NVIC_IPR0 (rw) register accessor: Interrupt Priority Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`nvic_ipr0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`nvic_ipr0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nvic_ipr0`]
module"]
#[doc(alias = "NVIC_IPR0")]
pub type NvicIpr0 = crate::Reg<nvic_ipr0::NvicIpr0Spec>;
#[doc = "Interrupt Priority Register 0"]
pub mod nvic_ipr0;
#[doc = "NVIC_IPR1 (rw) register accessor: Interrupt Priority Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`nvic_ipr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`nvic_ipr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nvic_ipr1`]
module"]
#[doc(alias = "NVIC_IPR1")]
pub type NvicIpr1 = crate::Reg<nvic_ipr1::NvicIpr1Spec>;
#[doc = "Interrupt Priority Register 1"]
pub mod nvic_ipr1;
#[doc = "NVIC_IPR2 (rw) register accessor: Interrupt Priority Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`nvic_ipr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`nvic_ipr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nvic_ipr2`]
module"]
#[doc(alias = "NVIC_IPR2")]
pub type NvicIpr2 = crate::Reg<nvic_ipr2::NvicIpr2Spec>;
#[doc = "Interrupt Priority Register 2"]
pub mod nvic_ipr2;
#[doc = "NVIC_IPR3 (rw) register accessor: Interrupt Priority Register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`nvic_ipr3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`nvic_ipr3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nvic_ipr3`]
module"]
#[doc(alias = "NVIC_IPR3")]
pub type NvicIpr3 = crate::Reg<nvic_ipr3::NvicIpr3Spec>;
#[doc = "Interrupt Priority Register 3"]
pub mod nvic_ipr3;
#[doc = "NVIC_IPR4 (rw) register accessor: Interrupt Priority Register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`nvic_ipr4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`nvic_ipr4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nvic_ipr4`]
module"]
#[doc(alias = "NVIC_IPR4")]
pub type NvicIpr4 = crate::Reg<nvic_ipr4::NvicIpr4Spec>;
#[doc = "Interrupt Priority Register 4"]
pub mod nvic_ipr4;
#[doc = "NVIC_IPR5 (rw) register accessor: Interrupt Priority Register 5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`nvic_ipr5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`nvic_ipr5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nvic_ipr5`]
module"]
#[doc(alias = "NVIC_IPR5")]
pub type NvicIpr5 = crate::Reg<nvic_ipr5::NvicIpr5Spec>;
#[doc = "Interrupt Priority Register 5"]
pub mod nvic_ipr5;
#[doc = "NVIC_IPR6 (rw) register accessor: Interrupt Priority Register 6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`nvic_ipr6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`nvic_ipr6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nvic_ipr6`]
module"]
#[doc(alias = "NVIC_IPR6")]
pub type NvicIpr6 = crate::Reg<nvic_ipr6::NvicIpr6Spec>;
#[doc = "Interrupt Priority Register 6"]
pub mod nvic_ipr6;
#[doc = "NVIC_IPR7 (rw) register accessor: Interrupt Priority Register 7\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`nvic_ipr7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`nvic_ipr7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nvic_ipr7`]
module"]
#[doc(alias = "NVIC_IPR7")]
pub type NvicIpr7 = crate::Reg<nvic_ipr7::NvicIpr7Spec>;
#[doc = "Interrupt Priority Register 7"]
pub mod nvic_ipr7;
#[doc = "NVIC_IPR8 (rw) register accessor: Interrupt Priority Register 8\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`nvic_ipr8::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`nvic_ipr8::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nvic_ipr8`]
module"]
#[doc(alias = "NVIC_IPR8")]
pub type NvicIpr8 = crate::Reg<nvic_ipr8::NvicIpr8Spec>;
#[doc = "Interrupt Priority Register 8"]
pub mod nvic_ipr8;
#[doc = "NVIC_IPR9 (rw) register accessor: Interrupt Priority Register 9\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`nvic_ipr9::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`nvic_ipr9::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nvic_ipr9`]
module"]
#[doc(alias = "NVIC_IPR9")]
pub type NvicIpr9 = crate::Reg<nvic_ipr9::NvicIpr9Spec>;
#[doc = "Interrupt Priority Register 9"]
pub mod nvic_ipr9;
#[doc = "NVIC_IPR10 (rw) register accessor: Interrupt Priority Register 10\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`nvic_ipr10::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`nvic_ipr10::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nvic_ipr10`]
module"]
#[doc(alias = "NVIC_IPR10")]
pub type NvicIpr10 = crate::Reg<nvic_ipr10::NvicIpr10Spec>;
#[doc = "Interrupt Priority Register 10"]
pub mod nvic_ipr10;
#[doc = "NVIC_IPR11 (rw) register accessor: Interrupt Priority Register 11\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`nvic_ipr11::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`nvic_ipr11::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nvic_ipr11`]
module"]
#[doc(alias = "NVIC_IPR11")]
pub type NvicIpr11 = crate::Reg<nvic_ipr11::NvicIpr11Spec>;
#[doc = "Interrupt Priority Register 11"]
pub mod nvic_ipr11;
#[doc = "NVIC_IPR12 (rw) register accessor: Interrupt Priority Register 12\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`nvic_ipr12::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`nvic_ipr12::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nvic_ipr12`]
module"]
#[doc(alias = "NVIC_IPR12")]
pub type NvicIpr12 = crate::Reg<nvic_ipr12::NvicIpr12Spec>;
#[doc = "Interrupt Priority Register 12"]
pub mod nvic_ipr12;
#[doc = "NVIC_IPR13 (rw) register accessor: Interrupt Priority Register 13\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`nvic_ipr13::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`nvic_ipr13::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nvic_ipr13`]
module"]
#[doc(alias = "NVIC_IPR13")]
pub type NvicIpr13 = crate::Reg<nvic_ipr13::NvicIpr13Spec>;
#[doc = "Interrupt Priority Register 13"]
pub mod nvic_ipr13;
#[doc = "NVIC_IPR14 (rw) register accessor: Interrupt Priority Register 14\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`nvic_ipr14::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`nvic_ipr14::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nvic_ipr14`]
module"]
#[doc(alias = "NVIC_IPR14")]
pub type NvicIpr14 = crate::Reg<nvic_ipr14::NvicIpr14Spec>;
#[doc = "Interrupt Priority Register 14"]
pub mod nvic_ipr14;
#[doc = "NVIC_IPR15 (rw) register accessor: Interrupt Priority Register 15\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`nvic_ipr15::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`nvic_ipr15::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nvic_ipr15`]
module"]
#[doc(alias = "NVIC_IPR15")]
pub type NvicIpr15 = crate::Reg<nvic_ipr15::NvicIpr15Spec>;
#[doc = "Interrupt Priority Register 15"]
pub mod nvic_ipr15;
#[doc = "NVIC_IPR16 (rw) register accessor: Interrupt Priority Register 16\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`nvic_ipr16::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`nvic_ipr16::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nvic_ipr16`]
module"]
#[doc(alias = "NVIC_IPR16")]
pub type NvicIpr16 = crate::Reg<nvic_ipr16::NvicIpr16Spec>;
#[doc = "Interrupt Priority Register 16"]
pub mod nvic_ipr16;
#[doc = "NVIC_IPR17 (rw) register accessor: Interrupt Priority Register 17\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`nvic_ipr17::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`nvic_ipr17::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nvic_ipr17`]
module"]
#[doc(alias = "NVIC_IPR17")]
pub type NvicIpr17 = crate::Reg<nvic_ipr17::NvicIpr17Spec>;
#[doc = "Interrupt Priority Register 17"]
pub mod nvic_ipr17;
#[doc = "NVIC_IPR18 (rw) register accessor: Interrupt Priority Register 18\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`nvic_ipr18::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`nvic_ipr18::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nvic_ipr18`]
module"]
#[doc(alias = "NVIC_IPR18")]
pub type NvicIpr18 = crate::Reg<nvic_ipr18::NvicIpr18Spec>;
#[doc = "Interrupt Priority Register 18"]
pub mod nvic_ipr18;
#[doc = "NVIC_IPR19 (rw) register accessor: Interrupt Priority Register 19\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`nvic_ipr19::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`nvic_ipr19::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nvic_ipr19`]
module"]
#[doc(alias = "NVIC_IPR19")]
pub type NvicIpr19 = crate::Reg<nvic_ipr19::NvicIpr19Spec>;
#[doc = "Interrupt Priority Register 19"]
pub mod nvic_ipr19;
#[doc = "NVIC_IPR20 (rw) register accessor: Interrupt Priority Register 20\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`nvic_ipr20::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`nvic_ipr20::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nvic_ipr20`]
module"]
#[doc(alias = "NVIC_IPR20")]
pub type NvicIpr20 = crate::Reg<nvic_ipr20::NvicIpr20Spec>;
#[doc = "Interrupt Priority Register 20"]
pub mod nvic_ipr20;
#[doc = "NVIC_IPR21 (rw) register accessor: Interrupt Priority Register 21\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`nvic_ipr21::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`nvic_ipr21::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nvic_ipr21`]
module"]
#[doc(alias = "NVIC_IPR21")]
pub type NvicIpr21 = crate::Reg<nvic_ipr21::NvicIpr21Spec>;
#[doc = "Interrupt Priority Register 21"]
pub mod nvic_ipr21;
#[doc = "NVIC_IPR22 (rw) register accessor: Interrupt Priority Register 22\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`nvic_ipr22::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`nvic_ipr22::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nvic_ipr22`]
module"]
#[doc(alias = "NVIC_IPR22")]
pub type NvicIpr22 = crate::Reg<nvic_ipr22::NvicIpr22Spec>;
#[doc = "Interrupt Priority Register 22"]
pub mod nvic_ipr22;
#[doc = "NVIC_IPR23 (rw) register accessor: Interrupt Priority Register 23\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`nvic_ipr23::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`nvic_ipr23::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nvic_ipr23`]
module"]
#[doc(alias = "NVIC_IPR23")]
pub type NvicIpr23 = crate::Reg<nvic_ipr23::NvicIpr23Spec>;
#[doc = "Interrupt Priority Register 23"]
pub mod nvic_ipr23;
#[doc = "NVIC_IPR24 (rw) register accessor: Interrupt Priority Register 24\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`nvic_ipr24::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`nvic_ipr24::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nvic_ipr24`]
module"]
#[doc(alias = "NVIC_IPR24")]
pub type NvicIpr24 = crate::Reg<nvic_ipr24::NvicIpr24Spec>;
#[doc = "Interrupt Priority Register 24"]
pub mod nvic_ipr24;
#[doc = "NVIC_IPR25 (rw) register accessor: Interrupt Priority Register 25\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`nvic_ipr25::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`nvic_ipr25::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nvic_ipr25`]
module"]
#[doc(alias = "NVIC_IPR25")]
pub type NvicIpr25 = crate::Reg<nvic_ipr25::NvicIpr25Spec>;
#[doc = "Interrupt Priority Register 25"]
pub mod nvic_ipr25;
#[doc = "NVIC_IPR26 (rw) register accessor: Interrupt Priority Register 26\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`nvic_ipr26::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`nvic_ipr26::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nvic_ipr26`]
module"]
#[doc(alias = "NVIC_IPR26")]
pub type NvicIpr26 = crate::Reg<nvic_ipr26::NvicIpr26Spec>;
#[doc = "Interrupt Priority Register 26"]
pub mod nvic_ipr26;
#[doc = "NVIC_IPR27 (rw) register accessor: Interrupt Priority Register 27\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`nvic_ipr27::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`nvic_ipr27::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nvic_ipr27`]
module"]
#[doc(alias = "NVIC_IPR27")]
pub type NvicIpr27 = crate::Reg<nvic_ipr27::NvicIpr27Spec>;
#[doc = "Interrupt Priority Register 27"]
pub mod nvic_ipr27;
#[doc = "CPUID (r) register accessor: CPUID Base Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpuid::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpuid`]
module"]
#[doc(alias = "CPUID")]
pub type Cpuid = crate::Reg<cpuid::CpuidSpec>;
#[doc = "CPUID Base Register"]
pub mod cpuid;
#[doc = "ICSR (rw) register accessor: Interrupt Control and State Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`icsr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`icsr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icsr`]
module"]
#[doc(alias = "ICSR")]
pub type Icsr = crate::Reg<icsr::IcsrSpec>;
#[doc = "Interrupt Control and State Register"]
pub mod icsr;
#[doc = "VTOR (rw) register accessor: Vector Table Offset Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vtor::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vtor::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vtor`]
module"]
#[doc(alias = "VTOR")]
pub type Vtor = crate::Reg<vtor::VtorSpec>;
#[doc = "Vector Table Offset Register"]
pub mod vtor;
#[doc = "AIRCR (rw) register accessor: Application Interrupt and Reset Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aircr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aircr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aircr`]
module"]
#[doc(alias = "AIRCR")]
pub type Aircr = crate::Reg<aircr::AircrSpec>;
#[doc = "Application Interrupt and Reset Control Register"]
pub mod aircr;
#[doc = "SCR (rw) register accessor: System Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`scr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scr`]
module"]
#[doc(alias = "SCR")]
pub type Scr = crate::Reg<scr::ScrSpec>;
#[doc = "System Control Register"]
pub mod scr;
#[doc = "CCR (rw) register accessor: Configuration and Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccr`]
module"]
#[doc(alias = "CCR")]
pub type Ccr = crate::Reg<ccr::CcrSpec>;
#[doc = "Configuration and Control Register"]
pub mod ccr;
#[doc = "SHPR1 (rw) register accessor: System Handler Priority Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`shpr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`shpr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@shpr1`]
module"]
#[doc(alias = "SHPR1")]
pub type Shpr1 = crate::Reg<shpr1::Shpr1Spec>;
#[doc = "System Handler Priority Register 1"]
pub mod shpr1;
#[doc = "SHPR2 (rw) register accessor: System Handler Priority Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`shpr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`shpr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@shpr2`]
module"]
#[doc(alias = "SHPR2")]
pub type Shpr2 = crate::Reg<shpr2::Shpr2Spec>;
#[doc = "System Handler Priority Register 2"]
pub mod shpr2;
#[doc = "SHPR3 (rw) register accessor: System Handler Priority Register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`shpr3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`shpr3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@shpr3`]
module"]
#[doc(alias = "SHPR3")]
pub type Shpr3 = crate::Reg<shpr3::Shpr3Spec>;
#[doc = "System Handler Priority Register 3"]
pub mod shpr3;
#[doc = "SHCSR (rw) register accessor: System Handler Control and State Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`shcsr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`shcsr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@shcsr`]
module"]
#[doc(alias = "SHCSR")]
pub type Shcsr = crate::Reg<shcsr::ShcsrSpec>;
#[doc = "System Handler Control and State Register"]
pub mod shcsr;
#[doc = "CFSR (rw) register accessor: Configurable Fault Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfsr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfsr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfsr`]
module"]
#[doc(alias = "CFSR")]
pub type Cfsr = crate::Reg<cfsr::CfsrSpec>;
#[doc = "Configurable Fault Status Register"]
pub mod cfsr;
#[doc = "HFSR (rw) register accessor: HardFault Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hfsr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hfsr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hfsr`]
module"]
#[doc(alias = "HFSR")]
pub type Hfsr = crate::Reg<hfsr::HfsrSpec>;
#[doc = "HardFault Status Register"]
pub mod hfsr;
#[doc = "MMFAR (rw) register accessor: MemManage Fault Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmfar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mmfar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mmfar`]
module"]
#[doc(alias = "MMFAR")]
pub type Mmfar = crate::Reg<mmfar::MmfarSpec>;
#[doc = "MemManage Fault Address Register"]
pub mod mmfar;
#[doc = "BFAR (rw) register accessor: BusFault Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bfar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bfar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bfar`]
module"]
#[doc(alias = "BFAR")]
pub type Bfar = crate::Reg<bfar::BfarSpec>;
#[doc = "BusFault Address Register"]
pub mod bfar;
#[doc = "AFSR (rw) register accessor: Auxiliary Fault Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`afsr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`afsr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@afsr`]
module"]
#[doc(alias = "AFSR")]
pub type Afsr = crate::Reg<afsr::AfsrSpec>;
#[doc = "Auxiliary Fault Status Register"]
pub mod afsr;
#[doc = "CPACR (rw) register accessor: Coprocessor Access Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpacr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpacr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpacr`]
module"]
#[doc(alias = "CPACR")]
pub type Cpacr = crate::Reg<cpacr::CpacrSpec>;
#[doc = "Coprocessor Access Control Register"]
pub mod cpacr;
#[doc = "MPU_TYPE (r) register accessor: MPU Type Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mpu_type::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpu_type`]
module"]
#[doc(alias = "MPU_TYPE")]
pub type MpuType = crate::Reg<mpu_type::MpuTypeSpec>;
#[doc = "MPU Type Register"]
pub mod mpu_type;
#[doc = "MPU_CTRL (rw) register accessor: MPU Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mpu_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mpu_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpu_ctrl`]
module"]
#[doc(alias = "MPU_CTRL")]
pub type MpuCtrl = crate::Reg<mpu_ctrl::MpuCtrlSpec>;
#[doc = "MPU Control Register"]
pub mod mpu_ctrl;
#[doc = "MPU_RNR (rw) register accessor: MPU Region Number Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mpu_rnr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mpu_rnr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpu_rnr`]
module"]
#[doc(alias = "MPU_RNR")]
pub type MpuRnr = crate::Reg<mpu_rnr::MpuRnrSpec>;
#[doc = "MPU Region Number Register"]
pub mod mpu_rnr;
#[doc = "MPU_RBAR (rw) register accessor: MPU Region Base Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mpu_rbar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mpu_rbar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpu_rbar`]
module"]
#[doc(alias = "MPU_RBAR")]
pub type MpuRbar = crate::Reg<mpu_rbar::MpuRbarSpec>;
#[doc = "MPU Region Base Address Register"]
pub mod mpu_rbar;
#[doc = "MPU_RASR (rw) register accessor: MPU Region Attribute and Size Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mpu_rasr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mpu_rasr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpu_rasr`]
module"]
#[doc(alias = "MPU_RASR")]
pub type MpuRasr = crate::Reg<mpu_rasr::MpuRasrSpec>;
#[doc = "MPU Region Attribute and Size Register"]
pub mod mpu_rasr;
#[doc = "MPU_RBAR_A1 (rw) register accessor: MPU Region Base Address Register A1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mpu_rbar_a1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mpu_rbar_a1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpu_rbar_a1`]
module"]
#[doc(alias = "MPU_RBAR_A1")]
pub type MpuRbarA1 = crate::Reg<mpu_rbar_a1::MpuRbarA1Spec>;
#[doc = "MPU Region Base Address Register A1"]
pub mod mpu_rbar_a1;
#[doc = "MPU_RASR_A1 (rw) register accessor: MPU Region Attribute and Size Register A1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mpu_rasr_a1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mpu_rasr_a1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpu_rasr_a1`]
module"]
#[doc(alias = "MPU_RASR_A1")]
pub type MpuRasrA1 = crate::Reg<mpu_rasr_a1::MpuRasrA1Spec>;
#[doc = "MPU Region Attribute and Size Register A1"]
pub mod mpu_rasr_a1;
#[doc = "MPU_RBAR_A2 (rw) register accessor: MPU Region Base Address Register A2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mpu_rbar_a2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mpu_rbar_a2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpu_rbar_a2`]
module"]
#[doc(alias = "MPU_RBAR_A2")]
pub type MpuRbarA2 = crate::Reg<mpu_rbar_a2::MpuRbarA2Spec>;
#[doc = "MPU Region Base Address Register A2"]
pub mod mpu_rbar_a2;
#[doc = "MPU_RASR_A2 (rw) register accessor: MPU Region Attribute and Size Register A2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mpu_rasr_a2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mpu_rasr_a2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpu_rasr_a2`]
module"]
#[doc(alias = "MPU_RASR_A2")]
pub type MpuRasrA2 = crate::Reg<mpu_rasr_a2::MpuRasrA2Spec>;
#[doc = "MPU Region Attribute and Size Register A2"]
pub mod mpu_rasr_a2;
#[doc = "MPU_RBAR_A3 (rw) register accessor: MPU Region Base Address Register A3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mpu_rbar_a3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mpu_rbar_a3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpu_rbar_a3`]
module"]
#[doc(alias = "MPU_RBAR_A3")]
pub type MpuRbarA3 = crate::Reg<mpu_rbar_a3::MpuRbarA3Spec>;
#[doc = "MPU Region Base Address Register A3"]
pub mod mpu_rbar_a3;
#[doc = "MPU_RASR_A3 (rw) register accessor: MPU Region Attribute and Size Register A3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mpu_rasr_a3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mpu_rasr_a3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpu_rasr_a3`]
module"]
#[doc(alias = "MPU_RASR_A3")]
pub type MpuRasrA3 = crate::Reg<mpu_rasr_a3::MpuRasrA3Spec>;
#[doc = "MPU Region Attribute and Size Register A3"]
pub mod mpu_rasr_a3;
#[doc = "STIR (w) register accessor: Software Trigger Interrupt Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stir::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stir`]
module"]
#[doc(alias = "STIR")]
pub type Stir = crate::Reg<stir::StirSpec>;
#[doc = "Software Trigger Interrupt Register"]
pub mod stir;
#[doc = "FPCCR (rw) register accessor: Floating-point Context Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fpccr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fpccr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fpccr`]
module"]
#[doc(alias = "FPCCR")]
pub type Fpccr = crate::Reg<fpccr::FpccrSpec>;
#[doc = "Floating-point Context Control Register"]
pub mod fpccr;
#[doc = "FPCAR (rw) register accessor: Floating-point Context Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fpcar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fpcar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fpcar`]
module"]
#[doc(alias = "FPCAR")]
pub type Fpcar = crate::Reg<fpcar::FpcarSpec>;
#[doc = "Floating-point Context Address Register"]
pub mod fpcar;
#[doc = "FPDSCR (rw) register accessor: Floating-point Default Status Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fpdscr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fpdscr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fpdscr`]
module"]
#[doc(alias = "FPDSCR")]
pub type Fpdscr = crate::Reg<fpdscr::FpdscrSpec>;
#[doc = "Floating-point Default Status Control Register"]
pub mod fpdscr;
