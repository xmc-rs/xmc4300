#[doc = "Register `FPCCR` reader"]
pub type R = crate::R<FpccrSpec>;
#[doc = "Register `FPCCR` writer"]
pub type W = crate::W<FpccrSpec>;
#[doc = "Lazy State Preservation Active\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lspact {
    #[doc = "0: Lazy state preservation is not active."]
    Value1 = 0,
    #[doc = "1: Lazy state preservation is active. floating-point stack frame has been allocated but saving state to it has been deferred."]
    Value2 = 1,
}
impl From<Lspact> for bool {
    #[inline(always)]
    fn from(variant: Lspact) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LSPACT` reader - Lazy State Preservation Active"]
pub type LspactR = crate::BitReader<Lspact>;
impl LspactR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lspact {
        match self.bits {
            false => Lspact::Value1,
            true => Lspact::Value2,
        }
    }
    #[doc = "Lazy state preservation is not active."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Lspact::Value1
    }
    #[doc = "Lazy state preservation is active. floating-point stack frame has been allocated but saving state to it has been deferred."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Lspact::Value2
    }
}
#[doc = "Field `LSPACT` writer - Lazy State Preservation Active"]
pub type LspactW<'a, REG> = crate::BitWriter<'a, REG, Lspact>;
impl<'a, REG> LspactW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Lazy state preservation is not active."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Lspact::Value1)
    }
    #[doc = "Lazy state preservation is active. floating-point stack frame has been allocated but saving state to it has been deferred."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Lspact::Value2)
    }
}
#[doc = "User allocated Stack Frame\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum User {
    #[doc = "0: Privilege level was not user when the floating-point stack frame was allocated."]
    Value1 = 0,
    #[doc = "1: Privilege level was user when the floating-point stack frame was allocated."]
    Value2 = 1,
}
impl From<User> for bool {
    #[inline(always)]
    fn from(variant: User) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USER` reader - User allocated Stack Frame"]
pub type UserR = crate::BitReader<User>;
impl UserR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> User {
        match self.bits {
            false => User::Value1,
            true => User::Value2,
        }
    }
    #[doc = "Privilege level was not user when the floating-point stack frame was allocated."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == User::Value1
    }
    #[doc = "Privilege level was user when the floating-point stack frame was allocated."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == User::Value2
    }
}
#[doc = "Field `USER` writer - User allocated Stack Frame"]
pub type UserW<'a, REG> = crate::BitWriter<'a, REG, User>;
impl<'a, REG> UserW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Privilege level was not user when the floating-point stack frame was allocated."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(User::Value1)
    }
    #[doc = "Privilege level was user when the floating-point stack frame was allocated."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(User::Value2)
    }
}
#[doc = "Thread Mode allocated Stack Frame\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Thread {
    #[doc = "0: Mode was not Thread Mode when the floating-point stack frame was allocated."]
    Value1 = 0,
    #[doc = "1: Mode was Thread Mode when the floating-point stack frame was allocated."]
    Value2 = 1,
}
impl From<Thread> for bool {
    #[inline(always)]
    fn from(variant: Thread) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `THREAD` reader - Thread Mode allocated Stack Frame"]
pub type ThreadR = crate::BitReader<Thread>;
impl ThreadR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Thread {
        match self.bits {
            false => Thread::Value1,
            true => Thread::Value2,
        }
    }
    #[doc = "Mode was not Thread Mode when the floating-point stack frame was allocated."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Thread::Value1
    }
    #[doc = "Mode was Thread Mode when the floating-point stack frame was allocated."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Thread::Value2
    }
}
#[doc = "Field `THREAD` writer - Thread Mode allocated Stack Frame"]
pub type ThreadW<'a, REG> = crate::BitWriter<'a, REG, Thread>;
impl<'a, REG> ThreadW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Mode was not Thread Mode when the floating-point stack frame was allocated."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Thread::Value1)
    }
    #[doc = "Mode was Thread Mode when the floating-point stack frame was allocated."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Thread::Value2)
    }
}
#[doc = "HardFault Ready\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hfrdy {
    #[doc = "0: Priority did not permit setting the HardFault handler to the pending state when the floating-point stack frame was allocated."]
    Value1 = 0,
    #[doc = "1: Priority permitted setting the HardFault handler to the pending state when the floating-point stack frame was allocated."]
    Value2 = 1,
}
impl From<Hfrdy> for bool {
    #[inline(always)]
    fn from(variant: Hfrdy) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HFRDY` reader - HardFault Ready"]
pub type HfrdyR = crate::BitReader<Hfrdy>;
impl HfrdyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hfrdy {
        match self.bits {
            false => Hfrdy::Value1,
            true => Hfrdy::Value2,
        }
    }
    #[doc = "Priority did not permit setting the HardFault handler to the pending state when the floating-point stack frame was allocated."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Hfrdy::Value1
    }
    #[doc = "Priority permitted setting the HardFault handler to the pending state when the floating-point stack frame was allocated."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Hfrdy::Value2
    }
}
#[doc = "Field `HFRDY` writer - HardFault Ready"]
pub type HfrdyW<'a, REG> = crate::BitWriter<'a, REG, Hfrdy>;
impl<'a, REG> HfrdyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Priority did not permit setting the HardFault handler to the pending state when the floating-point stack frame was allocated."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Hfrdy::Value1)
    }
    #[doc = "Priority permitted setting the HardFault handler to the pending state when the floating-point stack frame was allocated."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Hfrdy::Value2)
    }
}
#[doc = "MemManage Ready\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mmrdy {
    #[doc = "0: MemManage is disabled or priority did not permit setting the MemManage handler to the pending state when the floating-point stack frame was allocated."]
    Value1 = 0,
    #[doc = "1: MemManage is enabled and priority permitted setting the MemManage handler to the pending state when the floating-point stack frame was allocated."]
    Value2 = 1,
}
impl From<Mmrdy> for bool {
    #[inline(always)]
    fn from(variant: Mmrdy) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MMRDY` reader - MemManage Ready"]
pub type MmrdyR = crate::BitReader<Mmrdy>;
impl MmrdyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mmrdy {
        match self.bits {
            false => Mmrdy::Value1,
            true => Mmrdy::Value2,
        }
    }
    #[doc = "MemManage is disabled or priority did not permit setting the MemManage handler to the pending state when the floating-point stack frame was allocated."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Mmrdy::Value1
    }
    #[doc = "MemManage is enabled and priority permitted setting the MemManage handler to the pending state when the floating-point stack frame was allocated."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Mmrdy::Value2
    }
}
#[doc = "Field `MMRDY` writer - MemManage Ready"]
pub type MmrdyW<'a, REG> = crate::BitWriter<'a, REG, Mmrdy>;
impl<'a, REG> MmrdyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "MemManage is disabled or priority did not permit setting the MemManage handler to the pending state when the floating-point stack frame was allocated."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Mmrdy::Value1)
    }
    #[doc = "MemManage is enabled and priority permitted setting the MemManage handler to the pending state when the floating-point stack frame was allocated."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Mmrdy::Value2)
    }
}
#[doc = "BusFault Ready\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bfrdy {
    #[doc = "0: BusFault is disabled or priority did not permit setting the BusFault handler to the pending state when the floating-point stack frame was allocated."]
    Value1 = 0,
    #[doc = "1: BusFault is enabled and priority permitted setting the BusFault handler to the pending state when the floating-point stack frame was allocated."]
    Value2 = 1,
}
impl From<Bfrdy> for bool {
    #[inline(always)]
    fn from(variant: Bfrdy) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BFRDY` reader - BusFault Ready"]
pub type BfrdyR = crate::BitReader<Bfrdy>;
impl BfrdyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bfrdy {
        match self.bits {
            false => Bfrdy::Value1,
            true => Bfrdy::Value2,
        }
    }
    #[doc = "BusFault is disabled or priority did not permit setting the BusFault handler to the pending state when the floating-point stack frame was allocated."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Bfrdy::Value1
    }
    #[doc = "BusFault is enabled and priority permitted setting the BusFault handler to the pending state when the floating-point stack frame was allocated."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Bfrdy::Value2
    }
}
#[doc = "Field `BFRDY` writer - BusFault Ready"]
pub type BfrdyW<'a, REG> = crate::BitWriter<'a, REG, Bfrdy>;
impl<'a, REG> BfrdyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "BusFault is disabled or priority did not permit setting the BusFault handler to the pending state when the floating-point stack frame was allocated."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Bfrdy::Value1)
    }
    #[doc = "BusFault is enabled and priority permitted setting the BusFault handler to the pending state when the floating-point stack frame was allocated."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Bfrdy::Value2)
    }
}
#[doc = "Monitor Ready\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Monrdy {
    #[doc = "0: Debug Monitor is disabled or priority did not permit setting MON_PEND when the floating-point stack frame was allocated."]
    Value1 = 0,
    #[doc = "1: Debug Monitor is enabled and priority permits setting MON_PEND when the floating-point stack frame was allocated."]
    Value2 = 1,
}
impl From<Monrdy> for bool {
    #[inline(always)]
    fn from(variant: Monrdy) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MONRDY` reader - Monitor Ready"]
pub type MonrdyR = crate::BitReader<Monrdy>;
impl MonrdyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Monrdy {
        match self.bits {
            false => Monrdy::Value1,
            true => Monrdy::Value2,
        }
    }
    #[doc = "Debug Monitor is disabled or priority did not permit setting MON_PEND when the floating-point stack frame was allocated."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Monrdy::Value1
    }
    #[doc = "Debug Monitor is enabled and priority permits setting MON_PEND when the floating-point stack frame was allocated."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Monrdy::Value2
    }
}
#[doc = "Field `MONRDY` writer - Monitor Ready"]
pub type MonrdyW<'a, REG> = crate::BitWriter<'a, REG, Monrdy>;
impl<'a, REG> MonrdyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Debug Monitor is disabled or priority did not permit setting MON_PEND when the floating-point stack frame was allocated."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Monrdy::Value1)
    }
    #[doc = "Debug Monitor is enabled and priority permits setting MON_PEND when the floating-point stack frame was allocated."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Monrdy::Value2)
    }
}
#[doc = "Lazy State Preservation Enabled\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lspen {
    #[doc = "0: Disable automatic lazy state preservation for floating-point context."]
    Value1 = 0,
    #[doc = "1: Enable automatic lazy state preservation for floating-point context."]
    Value2 = 1,
}
impl From<Lspen> for bool {
    #[inline(always)]
    fn from(variant: Lspen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LSPEN` reader - Lazy State Preservation Enabled"]
pub type LspenR = crate::BitReader<Lspen>;
impl LspenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lspen {
        match self.bits {
            false => Lspen::Value1,
            true => Lspen::Value2,
        }
    }
    #[doc = "Disable automatic lazy state preservation for floating-point context."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Lspen::Value1
    }
    #[doc = "Enable automatic lazy state preservation for floating-point context."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Lspen::Value2
    }
}
#[doc = "Field `LSPEN` writer - Lazy State Preservation Enabled"]
pub type LspenW<'a, REG> = crate::BitWriter<'a, REG, Lspen>;
impl<'a, REG> LspenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable automatic lazy state preservation for floating-point context."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Lspen::Value1)
    }
    #[doc = "Enable automatic lazy state preservation for floating-point context."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Lspen::Value2)
    }
}
#[doc = "Automatic State Preservation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Aspen {
    #[doc = "0: Disable CONTROL setting on execution of a floating-point instruction."]
    Value1 = 0,
    #[doc = "1: Enable CONTROL setting on execution of a floating-point instruction."]
    Value2 = 1,
}
impl From<Aspen> for bool {
    #[inline(always)]
    fn from(variant: Aspen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ASPEN` reader - Automatic State Preservation"]
pub type AspenR = crate::BitReader<Aspen>;
impl AspenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Aspen {
        match self.bits {
            false => Aspen::Value1,
            true => Aspen::Value2,
        }
    }
    #[doc = "Disable CONTROL setting on execution of a floating-point instruction."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Aspen::Value1
    }
    #[doc = "Enable CONTROL setting on execution of a floating-point instruction."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Aspen::Value2
    }
}
#[doc = "Field `ASPEN` writer - Automatic State Preservation"]
pub type AspenW<'a, REG> = crate::BitWriter<'a, REG, Aspen>;
impl<'a, REG> AspenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable CONTROL setting on execution of a floating-point instruction."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Aspen::Value1)
    }
    #[doc = "Enable CONTROL setting on execution of a floating-point instruction."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Aspen::Value2)
    }
}
impl R {
    #[doc = "Bit 0 - Lazy State Preservation Active"]
    #[inline(always)]
    pub fn lspact(&self) -> LspactR {
        LspactR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - User allocated Stack Frame"]
    #[inline(always)]
    pub fn user(&self) -> UserR {
        UserR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - Thread Mode allocated Stack Frame"]
    #[inline(always)]
    pub fn thread(&self) -> ThreadR {
        ThreadR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - HardFault Ready"]
    #[inline(always)]
    pub fn hfrdy(&self) -> HfrdyR {
        HfrdyR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - MemManage Ready"]
    #[inline(always)]
    pub fn mmrdy(&self) -> MmrdyR {
        MmrdyR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - BusFault Ready"]
    #[inline(always)]
    pub fn bfrdy(&self) -> BfrdyR {
        BfrdyR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - Monitor Ready"]
    #[inline(always)]
    pub fn monrdy(&self) -> MonrdyR {
        MonrdyR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 30 - Lazy State Preservation Enabled"]
    #[inline(always)]
    pub fn lspen(&self) -> LspenR {
        LspenR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Automatic State Preservation"]
    #[inline(always)]
    pub fn aspen(&self) -> AspenR {
        AspenR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Lazy State Preservation Active"]
    #[inline(always)]
    #[must_use]
    pub fn lspact(&mut self) -> LspactW<FpccrSpec> {
        LspactW::new(self, 0)
    }
    #[doc = "Bit 1 - User allocated Stack Frame"]
    #[inline(always)]
    #[must_use]
    pub fn user(&mut self) -> UserW<FpccrSpec> {
        UserW::new(self, 1)
    }
    #[doc = "Bit 3 - Thread Mode allocated Stack Frame"]
    #[inline(always)]
    #[must_use]
    pub fn thread(&mut self) -> ThreadW<FpccrSpec> {
        ThreadW::new(self, 3)
    }
    #[doc = "Bit 4 - HardFault Ready"]
    #[inline(always)]
    #[must_use]
    pub fn hfrdy(&mut self) -> HfrdyW<FpccrSpec> {
        HfrdyW::new(self, 4)
    }
    #[doc = "Bit 5 - MemManage Ready"]
    #[inline(always)]
    #[must_use]
    pub fn mmrdy(&mut self) -> MmrdyW<FpccrSpec> {
        MmrdyW::new(self, 5)
    }
    #[doc = "Bit 6 - BusFault Ready"]
    #[inline(always)]
    #[must_use]
    pub fn bfrdy(&mut self) -> BfrdyW<FpccrSpec> {
        BfrdyW::new(self, 6)
    }
    #[doc = "Bit 8 - Monitor Ready"]
    #[inline(always)]
    #[must_use]
    pub fn monrdy(&mut self) -> MonrdyW<FpccrSpec> {
        MonrdyW::new(self, 8)
    }
    #[doc = "Bit 30 - Lazy State Preservation Enabled"]
    #[inline(always)]
    #[must_use]
    pub fn lspen(&mut self) -> LspenW<FpccrSpec> {
        LspenW::new(self, 30)
    }
    #[doc = "Bit 31 - Automatic State Preservation"]
    #[inline(always)]
    #[must_use]
    pub fn aspen(&mut self) -> AspenW<FpccrSpec> {
        AspenW::new(self, 31)
    }
}
#[doc = "Floating-point Context Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fpccr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fpccr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FpccrSpec;
impl crate::RegisterSpec for FpccrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fpccr::R`](R) reader structure"]
impl crate::Readable for FpccrSpec {}
#[doc = "`write(|w| ..)` method takes [`fpccr::W`](W) writer structure"]
impl crate::Writable for FpccrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FPCCR to value 0"]
impl crate::Resettable for FpccrSpec {
    const RESET_VALUE: u32 = 0;
}
