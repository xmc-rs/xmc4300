#[doc = "Register `FPCCR` reader"]
pub type R = crate::R<FPCCR_SPEC>;
#[doc = "Register `FPCCR` writer"]
pub type W = crate::W<FPCCR_SPEC>;
#[doc = "Field `LSPACT` reader - Lazy State Preservation Active"]
pub type LSPACT_R = crate::BitReader<LSPACT_A>;
#[doc = "Lazy State Preservation Active\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LSPACT_A {
    #[doc = "0: Lazy state preservation is not active."]
    VALUE1 = 0,
    #[doc = "1: Lazy state preservation is active. floating-point stack frame has been allocated but saving state to it has been deferred."]
    VALUE2 = 1,
}
impl From<LSPACT_A> for bool {
    #[inline(always)]
    fn from(variant: LSPACT_A) -> Self {
        variant as u8 != 0
    }
}
impl LSPACT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LSPACT_A {
        match self.bits {
            false => LSPACT_A::VALUE1,
            true => LSPACT_A::VALUE2,
        }
    }
    #[doc = "Lazy state preservation is not active."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == LSPACT_A::VALUE1
    }
    #[doc = "Lazy state preservation is active. floating-point stack frame has been allocated but saving state to it has been deferred."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == LSPACT_A::VALUE2
    }
}
#[doc = "Field `LSPACT` writer - Lazy State Preservation Active"]
pub type LSPACT_W<'a, REG> = crate::BitWriter<'a, REG, LSPACT_A>;
impl<'a, REG> LSPACT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Lazy state preservation is not active."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(LSPACT_A::VALUE1)
    }
    #[doc = "Lazy state preservation is active. floating-point stack frame has been allocated but saving state to it has been deferred."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(LSPACT_A::VALUE2)
    }
}
#[doc = "Field `USER` reader - User allocated Stack Frame"]
pub type USER_R = crate::BitReader<USER_A>;
#[doc = "User allocated Stack Frame\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USER_A {
    #[doc = "0: Privilege level was not user when the floating-point stack frame was allocated."]
    VALUE1 = 0,
    #[doc = "1: Privilege level was user when the floating-point stack frame was allocated."]
    VALUE2 = 1,
}
impl From<USER_A> for bool {
    #[inline(always)]
    fn from(variant: USER_A) -> Self {
        variant as u8 != 0
    }
}
impl USER_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> USER_A {
        match self.bits {
            false => USER_A::VALUE1,
            true => USER_A::VALUE2,
        }
    }
    #[doc = "Privilege level was not user when the floating-point stack frame was allocated."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == USER_A::VALUE1
    }
    #[doc = "Privilege level was user when the floating-point stack frame was allocated."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == USER_A::VALUE2
    }
}
#[doc = "Field `USER` writer - User allocated Stack Frame"]
pub type USER_W<'a, REG> = crate::BitWriter<'a, REG, USER_A>;
impl<'a, REG> USER_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Privilege level was not user when the floating-point stack frame was allocated."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(USER_A::VALUE1)
    }
    #[doc = "Privilege level was user when the floating-point stack frame was allocated."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(USER_A::VALUE2)
    }
}
#[doc = "Field `THREAD` reader - Thread Mode allocated Stack Frame"]
pub type THREAD_R = crate::BitReader<THREAD_A>;
#[doc = "Thread Mode allocated Stack Frame\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum THREAD_A {
    #[doc = "0: Mode was not Thread Mode when the floating-point stack frame was allocated."]
    VALUE1 = 0,
    #[doc = "1: Mode was Thread Mode when the floating-point stack frame was allocated."]
    VALUE2 = 1,
}
impl From<THREAD_A> for bool {
    #[inline(always)]
    fn from(variant: THREAD_A) -> Self {
        variant as u8 != 0
    }
}
impl THREAD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> THREAD_A {
        match self.bits {
            false => THREAD_A::VALUE1,
            true => THREAD_A::VALUE2,
        }
    }
    #[doc = "Mode was not Thread Mode when the floating-point stack frame was allocated."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == THREAD_A::VALUE1
    }
    #[doc = "Mode was Thread Mode when the floating-point stack frame was allocated."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == THREAD_A::VALUE2
    }
}
#[doc = "Field `THREAD` writer - Thread Mode allocated Stack Frame"]
pub type THREAD_W<'a, REG> = crate::BitWriter<'a, REG, THREAD_A>;
impl<'a, REG> THREAD_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Mode was not Thread Mode when the floating-point stack frame was allocated."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(THREAD_A::VALUE1)
    }
    #[doc = "Mode was Thread Mode when the floating-point stack frame was allocated."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(THREAD_A::VALUE2)
    }
}
#[doc = "Field `HFRDY` reader - HardFault Ready"]
pub type HFRDY_R = crate::BitReader<HFRDY_A>;
#[doc = "HardFault Ready\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HFRDY_A {
    #[doc = "0: Priority did not permit setting the HardFault handler to the pending state when the floating-point stack frame was allocated."]
    VALUE1 = 0,
    #[doc = "1: Priority permitted setting the HardFault handler to the pending state when the floating-point stack frame was allocated."]
    VALUE2 = 1,
}
impl From<HFRDY_A> for bool {
    #[inline(always)]
    fn from(variant: HFRDY_A) -> Self {
        variant as u8 != 0
    }
}
impl HFRDY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HFRDY_A {
        match self.bits {
            false => HFRDY_A::VALUE1,
            true => HFRDY_A::VALUE2,
        }
    }
    #[doc = "Priority did not permit setting the HardFault handler to the pending state when the floating-point stack frame was allocated."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == HFRDY_A::VALUE1
    }
    #[doc = "Priority permitted setting the HardFault handler to the pending state when the floating-point stack frame was allocated."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == HFRDY_A::VALUE2
    }
}
#[doc = "Field `HFRDY` writer - HardFault Ready"]
pub type HFRDY_W<'a, REG> = crate::BitWriter<'a, REG, HFRDY_A>;
impl<'a, REG> HFRDY_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Priority did not permit setting the HardFault handler to the pending state when the floating-point stack frame was allocated."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(HFRDY_A::VALUE1)
    }
    #[doc = "Priority permitted setting the HardFault handler to the pending state when the floating-point stack frame was allocated."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(HFRDY_A::VALUE2)
    }
}
#[doc = "Field `MMRDY` reader - MemManage Ready"]
pub type MMRDY_R = crate::BitReader<MMRDY_A>;
#[doc = "MemManage Ready\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MMRDY_A {
    #[doc = "0: MemManage is disabled or priority did not permit setting the MemManage handler to the pending state when the floating-point stack frame was allocated."]
    VALUE1 = 0,
    #[doc = "1: MemManage is enabled and priority permitted setting the MemManage handler to the pending state when the floating-point stack frame was allocated."]
    VALUE2 = 1,
}
impl From<MMRDY_A> for bool {
    #[inline(always)]
    fn from(variant: MMRDY_A) -> Self {
        variant as u8 != 0
    }
}
impl MMRDY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MMRDY_A {
        match self.bits {
            false => MMRDY_A::VALUE1,
            true => MMRDY_A::VALUE2,
        }
    }
    #[doc = "MemManage is disabled or priority did not permit setting the MemManage handler to the pending state when the floating-point stack frame was allocated."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == MMRDY_A::VALUE1
    }
    #[doc = "MemManage is enabled and priority permitted setting the MemManage handler to the pending state when the floating-point stack frame was allocated."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == MMRDY_A::VALUE2
    }
}
#[doc = "Field `MMRDY` writer - MemManage Ready"]
pub type MMRDY_W<'a, REG> = crate::BitWriter<'a, REG, MMRDY_A>;
impl<'a, REG> MMRDY_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "MemManage is disabled or priority did not permit setting the MemManage handler to the pending state when the floating-point stack frame was allocated."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(MMRDY_A::VALUE1)
    }
    #[doc = "MemManage is enabled and priority permitted setting the MemManage handler to the pending state when the floating-point stack frame was allocated."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(MMRDY_A::VALUE2)
    }
}
#[doc = "Field `BFRDY` reader - BusFault Ready"]
pub type BFRDY_R = crate::BitReader<BFRDY_A>;
#[doc = "BusFault Ready\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BFRDY_A {
    #[doc = "0: BusFault is disabled or priority did not permit setting the BusFault handler to the pending state when the floating-point stack frame was allocated."]
    VALUE1 = 0,
    #[doc = "1: BusFault is enabled and priority permitted setting the BusFault handler to the pending state when the floating-point stack frame was allocated."]
    VALUE2 = 1,
}
impl From<BFRDY_A> for bool {
    #[inline(always)]
    fn from(variant: BFRDY_A) -> Self {
        variant as u8 != 0
    }
}
impl BFRDY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BFRDY_A {
        match self.bits {
            false => BFRDY_A::VALUE1,
            true => BFRDY_A::VALUE2,
        }
    }
    #[doc = "BusFault is disabled or priority did not permit setting the BusFault handler to the pending state when the floating-point stack frame was allocated."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == BFRDY_A::VALUE1
    }
    #[doc = "BusFault is enabled and priority permitted setting the BusFault handler to the pending state when the floating-point stack frame was allocated."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == BFRDY_A::VALUE2
    }
}
#[doc = "Field `BFRDY` writer - BusFault Ready"]
pub type BFRDY_W<'a, REG> = crate::BitWriter<'a, REG, BFRDY_A>;
impl<'a, REG> BFRDY_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "BusFault is disabled or priority did not permit setting the BusFault handler to the pending state when the floating-point stack frame was allocated."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(BFRDY_A::VALUE1)
    }
    #[doc = "BusFault is enabled and priority permitted setting the BusFault handler to the pending state when the floating-point stack frame was allocated."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(BFRDY_A::VALUE2)
    }
}
#[doc = "Field `MONRDY` reader - Monitor Ready"]
pub type MONRDY_R = crate::BitReader<MONRDY_A>;
#[doc = "Monitor Ready\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MONRDY_A {
    #[doc = "0: Debug Monitor is disabled or priority did not permit setting MON_PEND when the floating-point stack frame was allocated."]
    VALUE1 = 0,
    #[doc = "1: Debug Monitor is enabled and priority permits setting MON_PEND when the floating-point stack frame was allocated."]
    VALUE2 = 1,
}
impl From<MONRDY_A> for bool {
    #[inline(always)]
    fn from(variant: MONRDY_A) -> Self {
        variant as u8 != 0
    }
}
impl MONRDY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MONRDY_A {
        match self.bits {
            false => MONRDY_A::VALUE1,
            true => MONRDY_A::VALUE2,
        }
    }
    #[doc = "Debug Monitor is disabled or priority did not permit setting MON_PEND when the floating-point stack frame was allocated."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == MONRDY_A::VALUE1
    }
    #[doc = "Debug Monitor is enabled and priority permits setting MON_PEND when the floating-point stack frame was allocated."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == MONRDY_A::VALUE2
    }
}
#[doc = "Field `MONRDY` writer - Monitor Ready"]
pub type MONRDY_W<'a, REG> = crate::BitWriter<'a, REG, MONRDY_A>;
impl<'a, REG> MONRDY_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Debug Monitor is disabled or priority did not permit setting MON_PEND when the floating-point stack frame was allocated."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(MONRDY_A::VALUE1)
    }
    #[doc = "Debug Monitor is enabled and priority permits setting MON_PEND when the floating-point stack frame was allocated."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(MONRDY_A::VALUE2)
    }
}
#[doc = "Field `LSPEN` reader - Lazy State Preservation Enabled"]
pub type LSPEN_R = crate::BitReader<LSPEN_A>;
#[doc = "Lazy State Preservation Enabled\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LSPEN_A {
    #[doc = "0: Disable automatic lazy state preservation for floating-point context."]
    VALUE1 = 0,
    #[doc = "1: Enable automatic lazy state preservation for floating-point context."]
    VALUE2 = 1,
}
impl From<LSPEN_A> for bool {
    #[inline(always)]
    fn from(variant: LSPEN_A) -> Self {
        variant as u8 != 0
    }
}
impl LSPEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LSPEN_A {
        match self.bits {
            false => LSPEN_A::VALUE1,
            true => LSPEN_A::VALUE2,
        }
    }
    #[doc = "Disable automatic lazy state preservation for floating-point context."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == LSPEN_A::VALUE1
    }
    #[doc = "Enable automatic lazy state preservation for floating-point context."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == LSPEN_A::VALUE2
    }
}
#[doc = "Field `LSPEN` writer - Lazy State Preservation Enabled"]
pub type LSPEN_W<'a, REG> = crate::BitWriter<'a, REG, LSPEN_A>;
impl<'a, REG> LSPEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable automatic lazy state preservation for floating-point context."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(LSPEN_A::VALUE1)
    }
    #[doc = "Enable automatic lazy state preservation for floating-point context."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(LSPEN_A::VALUE2)
    }
}
#[doc = "Field `ASPEN` reader - Automatic State Preservation"]
pub type ASPEN_R = crate::BitReader<ASPEN_A>;
#[doc = "Automatic State Preservation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ASPEN_A {
    #[doc = "0: Disable CONTROL setting on execution of a floating-point instruction."]
    VALUE1 = 0,
    #[doc = "1: Enable CONTROL setting on execution of a floating-point instruction."]
    VALUE2 = 1,
}
impl From<ASPEN_A> for bool {
    #[inline(always)]
    fn from(variant: ASPEN_A) -> Self {
        variant as u8 != 0
    }
}
impl ASPEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ASPEN_A {
        match self.bits {
            false => ASPEN_A::VALUE1,
            true => ASPEN_A::VALUE2,
        }
    }
    #[doc = "Disable CONTROL setting on execution of a floating-point instruction."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == ASPEN_A::VALUE1
    }
    #[doc = "Enable CONTROL setting on execution of a floating-point instruction."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == ASPEN_A::VALUE2
    }
}
#[doc = "Field `ASPEN` writer - Automatic State Preservation"]
pub type ASPEN_W<'a, REG> = crate::BitWriter<'a, REG, ASPEN_A>;
impl<'a, REG> ASPEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable CONTROL setting on execution of a floating-point instruction."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(ASPEN_A::VALUE1)
    }
    #[doc = "Enable CONTROL setting on execution of a floating-point instruction."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(ASPEN_A::VALUE2)
    }
}
impl R {
    #[doc = "Bit 0 - Lazy State Preservation Active"]
    #[inline(always)]
    pub fn lspact(&self) -> LSPACT_R {
        LSPACT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - User allocated Stack Frame"]
    #[inline(always)]
    pub fn user(&self) -> USER_R {
        USER_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - Thread Mode allocated Stack Frame"]
    #[inline(always)]
    pub fn thread(&self) -> THREAD_R {
        THREAD_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - HardFault Ready"]
    #[inline(always)]
    pub fn hfrdy(&self) -> HFRDY_R {
        HFRDY_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - MemManage Ready"]
    #[inline(always)]
    pub fn mmrdy(&self) -> MMRDY_R {
        MMRDY_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - BusFault Ready"]
    #[inline(always)]
    pub fn bfrdy(&self) -> BFRDY_R {
        BFRDY_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - Monitor Ready"]
    #[inline(always)]
    pub fn monrdy(&self) -> MONRDY_R {
        MONRDY_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 30 - Lazy State Preservation Enabled"]
    #[inline(always)]
    pub fn lspen(&self) -> LSPEN_R {
        LSPEN_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Automatic State Preservation"]
    #[inline(always)]
    pub fn aspen(&self) -> ASPEN_R {
        ASPEN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Lazy State Preservation Active"]
    #[inline(always)]
    #[must_use]
    pub fn lspact(&mut self) -> LSPACT_W<FPCCR_SPEC> {
        LSPACT_W::new(self, 0)
    }
    #[doc = "Bit 1 - User allocated Stack Frame"]
    #[inline(always)]
    #[must_use]
    pub fn user(&mut self) -> USER_W<FPCCR_SPEC> {
        USER_W::new(self, 1)
    }
    #[doc = "Bit 3 - Thread Mode allocated Stack Frame"]
    #[inline(always)]
    #[must_use]
    pub fn thread(&mut self) -> THREAD_W<FPCCR_SPEC> {
        THREAD_W::new(self, 3)
    }
    #[doc = "Bit 4 - HardFault Ready"]
    #[inline(always)]
    #[must_use]
    pub fn hfrdy(&mut self) -> HFRDY_W<FPCCR_SPEC> {
        HFRDY_W::new(self, 4)
    }
    #[doc = "Bit 5 - MemManage Ready"]
    #[inline(always)]
    #[must_use]
    pub fn mmrdy(&mut self) -> MMRDY_W<FPCCR_SPEC> {
        MMRDY_W::new(self, 5)
    }
    #[doc = "Bit 6 - BusFault Ready"]
    #[inline(always)]
    #[must_use]
    pub fn bfrdy(&mut self) -> BFRDY_W<FPCCR_SPEC> {
        BFRDY_W::new(self, 6)
    }
    #[doc = "Bit 8 - Monitor Ready"]
    #[inline(always)]
    #[must_use]
    pub fn monrdy(&mut self) -> MONRDY_W<FPCCR_SPEC> {
        MONRDY_W::new(self, 8)
    }
    #[doc = "Bit 30 - Lazy State Preservation Enabled"]
    #[inline(always)]
    #[must_use]
    pub fn lspen(&mut self) -> LSPEN_W<FPCCR_SPEC> {
        LSPEN_W::new(self, 30)
    }
    #[doc = "Bit 31 - Automatic State Preservation"]
    #[inline(always)]
    #[must_use]
    pub fn aspen(&mut self) -> ASPEN_W<FPCCR_SPEC> {
        ASPEN_W::new(self, 31)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Floating-point Context Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fpccr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fpccr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FPCCR_SPEC;
impl crate::RegisterSpec for FPCCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fpccr::R`](R) reader structure"]
impl crate::Readable for FPCCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`fpccr::W`](W) writer structure"]
impl crate::Writable for FPCCR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FPCCR to value 0"]
impl crate::Resettable for FPCCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
