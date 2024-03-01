#[doc = "Register `MLINKCLKCR` reader"]
pub type R = crate::R<MlinkclkcrSpec>;
#[doc = "Register `MLINKCLKCR` writer"]
pub type W = crate::W<MlinkclkcrSpec>;
#[doc = "Field `SYSDIV` reader - System Clock Division Value"]
pub type SysdivR = crate::FieldReader;
#[doc = "Field `SYSDIV` writer - System Clock Division Value"]
pub type SysdivW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "System Clock Selection Value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Syssel {
    #[doc = "0: fOFI clock"]
    Const0 = 0,
    #[doc = "1: fPLL clock"]
    Const1 = 1,
}
impl From<Syssel> for bool {
    #[inline(always)]
    fn from(variant: Syssel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SYSSEL` reader - System Clock Selection Value"]
pub type SysselR = crate::BitReader<Syssel>;
impl SysselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Syssel {
        match self.bits {
            false => Syssel::Const0,
            true => Syssel::Const1,
        }
    }
    #[doc = "fOFI clock"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == Syssel::Const0
    }
    #[doc = "fPLL clock"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == Syssel::Const1
    }
}
#[doc = "Field `SYSSEL` writer - System Clock Selection Value"]
pub type SysselW<'a, REG> = crate::BitWriter<'a, REG, Syssel>;
impl<'a, REG> SysselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "fOFI clock"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(Syssel::Const0)
    }
    #[doc = "fPLL clock"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(Syssel::Const1)
    }
}
#[doc = "CPU Clock Divider Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cpudiv {
    #[doc = "0: fCPU = fSYS"]
    Const0 = 0,
    #[doc = "1: fCPU = fSYS / 2"]
    Const1 = 1,
}
impl From<Cpudiv> for bool {
    #[inline(always)]
    fn from(variant: Cpudiv) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CPUDIV` reader - CPU Clock Divider Enable"]
pub type CpudivR = crate::BitReader<Cpudiv>;
impl CpudivR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cpudiv {
        match self.bits {
            false => Cpudiv::Const0,
            true => Cpudiv::Const1,
        }
    }
    #[doc = "fCPU = fSYS"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == Cpudiv::Const0
    }
    #[doc = "fCPU = fSYS / 2"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == Cpudiv::Const1
    }
}
#[doc = "Field `CPUDIV` writer - CPU Clock Divider Enable"]
pub type CpudivW<'a, REG> = crate::BitWriter<'a, REG, Cpudiv>;
impl<'a, REG> CpudivW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "fCPU = fSYS"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(Cpudiv::Const0)
    }
    #[doc = "fCPU = fSYS / 2"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(Cpudiv::Const1)
    }
}
#[doc = "PB Clock Divider Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pbdiv {
    #[doc = "0: fPERIPH = fCPU"]
    Const0 = 0,
    #[doc = "1: fPERIPH = fCPU / 2"]
    Const1 = 1,
}
impl From<Pbdiv> for bool {
    #[inline(always)]
    fn from(variant: Pbdiv) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PBDIV` reader - PB Clock Divider Enable"]
pub type PbdivR = crate::BitReader<Pbdiv>;
impl PbdivR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pbdiv {
        match self.bits {
            false => Pbdiv::Const0,
            true => Pbdiv::Const1,
        }
    }
    #[doc = "fPERIPH = fCPU"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == Pbdiv::Const0
    }
    #[doc = "fPERIPH = fCPU / 2"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == Pbdiv::Const1
    }
}
#[doc = "Field `PBDIV` writer - PB Clock Divider Enable"]
pub type PbdivW<'a, REG> = crate::BitWriter<'a, REG, Pbdiv>;
impl<'a, REG> PbdivW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "fPERIPH = fCPU"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(Pbdiv::Const0)
    }
    #[doc = "fPERIPH = fCPU / 2"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(Pbdiv::Const1)
    }
}
#[doc = "CCU Clock Divider Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ccudiv {
    #[doc = "0: fCCU = fSYS"]
    Const0 = 0,
    #[doc = "1: fCCU = fSYS / 2"]
    Const1 = 1,
}
impl From<Ccudiv> for bool {
    #[inline(always)]
    fn from(variant: Ccudiv) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCUDIV` reader - CCU Clock Divider Enable"]
pub type CcudivR = crate::BitReader<Ccudiv>;
impl CcudivR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ccudiv {
        match self.bits {
            false => Ccudiv::Const0,
            true => Ccudiv::Const1,
        }
    }
    #[doc = "fCCU = fSYS"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == Ccudiv::Const0
    }
    #[doc = "fCCU = fSYS / 2"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == Ccudiv::Const1
    }
}
#[doc = "Field `CCUDIV` writer - CCU Clock Divider Enable"]
pub type CcudivW<'a, REG> = crate::BitWriter<'a, REG, Ccudiv>;
impl<'a, REG> CcudivW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "fCCU = fSYS"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(Ccudiv::Const0)
    }
    #[doc = "fCCU = fSYS / 2"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(Ccudiv::Const1)
    }
}
#[doc = "Field `WDTDIV` reader - WDT Clock Divider Value"]
pub type WdtdivR = crate::FieldReader;
#[doc = "Field `WDTDIV` writer - WDT Clock Divider Value"]
pub type WdtdivW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "WDT Clock Selection Value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Wdtsel {
    #[doc = "0: fOFI clock"]
    Const00 = 0,
    #[doc = "1: fSTDBY clock"]
    Const01 = 1,
    #[doc = "2: fPLL clock"]
    Const10 = 2,
}
impl From<Wdtsel> for u8 {
    #[inline(always)]
    fn from(variant: Wdtsel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Wdtsel {
    type Ux = u8;
}
#[doc = "Field `WDTSEL` reader - WDT Clock Selection Value"]
pub type WdtselR = crate::FieldReader<Wdtsel>;
impl WdtselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Wdtsel> {
        match self.bits {
            0 => Some(Wdtsel::Const00),
            1 => Some(Wdtsel::Const01),
            2 => Some(Wdtsel::Const10),
            _ => None,
        }
    }
    #[doc = "fOFI clock"]
    #[inline(always)]
    pub fn is_const_00(&self) -> bool {
        *self == Wdtsel::Const00
    }
    #[doc = "fSTDBY clock"]
    #[inline(always)]
    pub fn is_const_01(&self) -> bool {
        *self == Wdtsel::Const01
    }
    #[doc = "fPLL clock"]
    #[inline(always)]
    pub fn is_const_10(&self) -> bool {
        *self == Wdtsel::Const10
    }
}
#[doc = "Field `WDTSEL` writer - WDT Clock Selection Value"]
pub type WdtselW<'a, REG> = crate::FieldWriter<'a, REG, 2, Wdtsel>;
impl<'a, REG> WdtselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "fOFI clock"]
    #[inline(always)]
    pub fn const_00(self) -> &'a mut crate::W<REG> {
        self.variant(Wdtsel::Const00)
    }
    #[doc = "fSTDBY clock"]
    #[inline(always)]
    pub fn const_01(self) -> &'a mut crate::W<REG> {
        self.variant(Wdtsel::Const01)
    }
    #[doc = "fPLL clock"]
    #[inline(always)]
    pub fn const_10(self) -> &'a mut crate::W<REG> {
        self.variant(Wdtsel::Const10)
    }
}
impl R {
    #[doc = "Bits 0:7 - System Clock Division Value"]
    #[inline(always)]
    pub fn sysdiv(&self) -> SysdivR {
        SysdivR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - System Clock Selection Value"]
    #[inline(always)]
    pub fn syssel(&self) -> SysselR {
        SysselR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 10 - CPU Clock Divider Enable"]
    #[inline(always)]
    pub fn cpudiv(&self) -> CpudivR {
        CpudivR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 12 - PB Clock Divider Enable"]
    #[inline(always)]
    pub fn pbdiv(&self) -> PbdivR {
        PbdivR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 14 - CCU Clock Divider Enable"]
    #[inline(always)]
    pub fn ccudiv(&self) -> CcudivR {
        CcudivR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bits 16:23 - WDT Clock Divider Value"]
    #[inline(always)]
    pub fn wdtdiv(&self) -> WdtdivR {
        WdtdivR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:25 - WDT Clock Selection Value"]
    #[inline(always)]
    pub fn wdtsel(&self) -> WdtselR {
        WdtselR::new(((self.bits >> 24) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - System Clock Division Value"]
    #[inline(always)]
    #[must_use]
    pub fn sysdiv(&mut self) -> SysdivW<MlinkclkcrSpec> {
        SysdivW::new(self, 0)
    }
    #[doc = "Bit 8 - System Clock Selection Value"]
    #[inline(always)]
    #[must_use]
    pub fn syssel(&mut self) -> SysselW<MlinkclkcrSpec> {
        SysselW::new(self, 8)
    }
    #[doc = "Bit 10 - CPU Clock Divider Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cpudiv(&mut self) -> CpudivW<MlinkclkcrSpec> {
        CpudivW::new(self, 10)
    }
    #[doc = "Bit 12 - PB Clock Divider Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pbdiv(&mut self) -> PbdivW<MlinkclkcrSpec> {
        PbdivW::new(self, 12)
    }
    #[doc = "Bit 14 - CCU Clock Divider Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ccudiv(&mut self) -> CcudivW<MlinkclkcrSpec> {
        CcudivW::new(self, 14)
    }
    #[doc = "Bits 16:23 - WDT Clock Divider Value"]
    #[inline(always)]
    #[must_use]
    pub fn wdtdiv(&mut self) -> WdtdivW<MlinkclkcrSpec> {
        WdtdivW::new(self, 16)
    }
    #[doc = "Bits 24:25 - WDT Clock Selection Value"]
    #[inline(always)]
    #[must_use]
    pub fn wdtsel(&mut self) -> WdtselW<MlinkclkcrSpec> {
        WdtselW::new(self, 24)
    }
}
#[doc = "Multi-Link Clock Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mlinkclkcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mlinkclkcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MlinkclkcrSpec;
impl crate::RegisterSpec for MlinkclkcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mlinkclkcr::R`](R) reader structure"]
impl crate::Readable for MlinkclkcrSpec {}
#[doc = "`write(|w| ..)` method takes [`mlinkclkcr::W`](W) writer structure"]
impl crate::Writable for MlinkclkcrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MLINKCLKCR to value 0"]
impl crate::Resettable for MlinkclkcrSpec {
    const RESET_VALUE: u32 = 0;
}
