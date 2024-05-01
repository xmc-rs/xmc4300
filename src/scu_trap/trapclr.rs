#[doc = "Register `TRAPCLR` writer"]
pub type W = crate::W<TrapclrSpec>;
#[doc = "OSC_HP Oscillator Watchdog Trap Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Soscwdgt {
    #[doc = "0: No effect"]
    Const0 = 0,
    #[doc = "1: Clear trap request"]
    Const1 = 1,
}
impl From<Soscwdgt> for bool {
    #[inline(always)]
    fn from(variant: Soscwdgt) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SOSCWDGT` writer - OSC_HP Oscillator Watchdog Trap Clear"]
pub type SoscwdgtW<'a, REG> = crate::BitWriter<'a, REG, Soscwdgt>;
impl<'a, REG> SoscwdgtW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(Soscwdgt::Const0)
    }
    #[doc = "Clear trap request"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(Soscwdgt::Const1)
    }
}
#[doc = "System VCO Lock Trap Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Svcolckt {
    #[doc = "0: No effect"]
    Const0 = 0,
    #[doc = "1: Clear trap request"]
    Const1 = 1,
}
impl From<Svcolckt> for bool {
    #[inline(always)]
    fn from(variant: Svcolckt) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SVCOLCKT` writer - System VCO Lock Trap Clear"]
pub type SvcolcktW<'a, REG> = crate::BitWriter<'a, REG, Svcolckt>;
impl<'a, REG> SvcolcktW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(Svcolckt::Const0)
    }
    #[doc = "Clear trap request"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(Svcolckt::Const1)
    }
}
#[doc = "USB VCO Lock Trap Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Uvcolckt {
    #[doc = "0: No effect"]
    Const0 = 0,
    #[doc = "1: Clear trap request"]
    Const1 = 1,
}
impl From<Uvcolckt> for bool {
    #[inline(always)]
    fn from(variant: Uvcolckt) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UVCOLCKT` writer - USB VCO Lock Trap Clear"]
pub type UvcolcktW<'a, REG> = crate::BitWriter<'a, REG, Uvcolckt>;
impl<'a, REG> UvcolcktW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(Uvcolckt::Const0)
    }
    #[doc = "Clear trap request"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(Uvcolckt::Const1)
    }
}
#[doc = "Parity Error Trap Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pet {
    #[doc = "0: No effect"]
    Const0 = 0,
    #[doc = "1: Clear trap request"]
    Const1 = 1,
}
impl From<Pet> for bool {
    #[inline(always)]
    fn from(variant: Pet) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PET` writer - Parity Error Trap Clear"]
pub type PetW<'a, REG> = crate::BitWriter<'a, REG, Pet>;
impl<'a, REG> PetW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(Pet::Const0)
    }
    #[doc = "Clear trap request"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(Pet::Const1)
    }
}
#[doc = "Brown Out Trap Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Brwnt {
    #[doc = "0: No effect"]
    Const0 = 0,
    #[doc = "1: Clear trap request"]
    Const1 = 1,
}
impl From<Brwnt> for bool {
    #[inline(always)]
    fn from(variant: Brwnt) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BRWNT` writer - Brown Out Trap Clear"]
pub type BrwntW<'a, REG> = crate::BitWriter<'a, REG, Brwnt>;
impl<'a, REG> BrwntW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(Brwnt::Const0)
    }
    #[doc = "Clear trap request"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(Brwnt::Const1)
    }
}
#[doc = "OSC_ULP Oscillator Watchdog Trap Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ulpwdgt {
    #[doc = "0: No effect"]
    Const0 = 0,
    #[doc = "1: Clear trap request"]
    Const1 = 1,
}
impl From<Ulpwdgt> for bool {
    #[inline(always)]
    fn from(variant: Ulpwdgt) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ULPWDGT` writer - OSC_ULP Oscillator Watchdog Trap Clear"]
pub type UlpwdgtW<'a, REG> = crate::BitWriter<'a, REG, Ulpwdgt>;
impl<'a, REG> UlpwdgtW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(Ulpwdgt::Const0)
    }
    #[doc = "Clear trap request"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(Ulpwdgt::Const1)
    }
}
#[doc = "Peripheral Bridge 0 Trap Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bwerr0t {
    #[doc = "0: No effect"]
    Const0 = 0,
    #[doc = "1: Clear trap request"]
    Const1 = 1,
}
impl From<Bwerr0t> for bool {
    #[inline(always)]
    fn from(variant: Bwerr0t) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BWERR0T` writer - Peripheral Bridge 0 Trap Clear"]
pub type Bwerr0tW<'a, REG> = crate::BitWriter<'a, REG, Bwerr0t>;
impl<'a, REG> Bwerr0tW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(Bwerr0t::Const0)
    }
    #[doc = "Clear trap request"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(Bwerr0t::Const1)
    }
}
#[doc = "Peripheral Bridge 1 Trap Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bwerr1t {
    #[doc = "0: No effect"]
    Const0 = 0,
    #[doc = "1: Clear trap request"]
    Const1 = 1,
}
impl From<Bwerr1t> for bool {
    #[inline(always)]
    fn from(variant: Bwerr1t) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BWERR1T` writer - Peripheral Bridge 1 Trap Clear"]
pub type Bwerr1tW<'a, REG> = crate::BitWriter<'a, REG, Bwerr1t>;
impl<'a, REG> Bwerr1tW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(Bwerr1t::Const0)
    }
    #[doc = "Clear trap request"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(Bwerr1t::Const1)
    }
}
#[doc = "EtherCat Reset 0 Trap Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ecat0rst {
    #[doc = "0: No effect"]
    Const0 = 0,
    #[doc = "1: Set trap request"]
    Const1 = 1,
}
impl From<Ecat0rst> for bool {
    #[inline(always)]
    fn from(variant: Ecat0rst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ECAT0RST` writer - EtherCat Reset 0 Trap Clear"]
pub type Ecat0rstW<'a, REG> = crate::BitWriter<'a, REG, Ecat0rst>;
impl<'a, REG> Ecat0rstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(Ecat0rst::Const0)
    }
    #[doc = "Set trap request"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(Ecat0rst::Const1)
    }
}
impl W {
    #[doc = "Bit 0 - OSC_HP Oscillator Watchdog Trap Clear"]
    #[inline(always)]
    #[must_use]
    pub fn soscwdgt(&mut self) -> SoscwdgtW<TrapclrSpec> {
        SoscwdgtW::new(self, 0)
    }
    #[doc = "Bit 2 - System VCO Lock Trap Clear"]
    #[inline(always)]
    #[must_use]
    pub fn svcolckt(&mut self) -> SvcolcktW<TrapclrSpec> {
        SvcolcktW::new(self, 2)
    }
    #[doc = "Bit 3 - USB VCO Lock Trap Clear"]
    #[inline(always)]
    #[must_use]
    pub fn uvcolckt(&mut self) -> UvcolcktW<TrapclrSpec> {
        UvcolcktW::new(self, 3)
    }
    #[doc = "Bit 4 - Parity Error Trap Clear"]
    #[inline(always)]
    #[must_use]
    pub fn pet(&mut self) -> PetW<TrapclrSpec> {
        PetW::new(self, 4)
    }
    #[doc = "Bit 5 - Brown Out Trap Clear"]
    #[inline(always)]
    #[must_use]
    pub fn brwnt(&mut self) -> BrwntW<TrapclrSpec> {
        BrwntW::new(self, 5)
    }
    #[doc = "Bit 6 - OSC_ULP Oscillator Watchdog Trap Clear"]
    #[inline(always)]
    #[must_use]
    pub fn ulpwdgt(&mut self) -> UlpwdgtW<TrapclrSpec> {
        UlpwdgtW::new(self, 6)
    }
    #[doc = "Bit 7 - Peripheral Bridge 0 Trap Clear"]
    #[inline(always)]
    #[must_use]
    pub fn bwerr0t(&mut self) -> Bwerr0tW<TrapclrSpec> {
        Bwerr0tW::new(self, 7)
    }
    #[doc = "Bit 8 - Peripheral Bridge 1 Trap Clear"]
    #[inline(always)]
    #[must_use]
    pub fn bwerr1t(&mut self) -> Bwerr1tW<TrapclrSpec> {
        Bwerr1tW::new(self, 8)
    }
    #[doc = "Bit 16 - EtherCat Reset 0 Trap Clear"]
    #[inline(always)]
    #[must_use]
    pub fn ecat0rst(&mut self) -> Ecat0rstW<TrapclrSpec> {
        Ecat0rstW::new(self, 16)
    }
}
#[doc = "Trap Clear Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`trapclr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TrapclrSpec;
impl crate::RegisterSpec for TrapclrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`trapclr::W`](W) writer structure"]
impl crate::Writable for TrapclrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TRAPCLR to value 0"]
impl crate::Resettable for TrapclrSpec {
    const RESET_VALUE: u32 = 0;
}
