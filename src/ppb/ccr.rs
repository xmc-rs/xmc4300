#[doc = "Register `CCR` reader"]
pub type R = crate::R<CcrSpec>;
#[doc = "Register `CCR` writer"]
pub type W = crate::W<CcrSpec>;
#[doc = "Non Base Thread Mode Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Nonbasethrdena {
    #[doc = "0: processor can enter Thread mode only when no exception is active."]
    Value1 = 0,
    #[doc = "1: processor can enter Thread mode from any level under the control of an EXC_RETURN value, see Exception returnException return occurs when the processor is in Handler mode and executes one of the following instructions to load the EXC_RETURN value into the PC:an LDM or POP instruction that loads the PCan LDR instruction with PC as the destinationa BX instruction using any register.EXC_RETURN is the value loaded into the LR on exception entry. The exception mechanism relies on this value to detect when the processor has completed an exception handler. The lowest five bits of this value provide information on the return stack and processor mode. shows the EXC_RETURN values with a description of the exception return behavior. All EXC_RETURN values have bits\\[31:5\\]
set to one. When this value is loaded into the PC it indicates to the processor that the exception is complete, and the processor initiates the appropriate exception return sequence.Exception return behaviorEXC_RETURN\\[31:0\\]Description 0xFFFFFFF1 Return to Handler mode, exception return uses non-floating-point state from the MSP and execution uses MSP after return. 0xFFFFFFF9 Return to Thread mode, exception return uses non-floating-point state from MSP and execution uses MSP after return. 0xFFFFFFFD Return to Thread mode, exception return uses non-floating-point state from the PSP and execution uses PSP after return. 0xFFFFFFE1 Return to Handler mode, exception return uses floating-point-state from MSP and execution uses MSP after return. 0xFFFFFFE9 Return to Thread mode, exception return uses floating-point state from MSP and execution uses MSP after return. 0xFFFFFFED Return to Thread mode, exception return uses floating-point state from PSP and execution uses PSP after return. ."]
    Value2 = 1,
}
impl From<Nonbasethrdena> for bool {
    #[inline(always)]
    fn from(variant: Nonbasethrdena) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NONBASETHRDENA` reader - Non Base Thread Mode Enable"]
pub type NonbasethrdenaR = crate::BitReader<Nonbasethrdena>;
impl NonbasethrdenaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Nonbasethrdena {
        match self.bits {
            false => Nonbasethrdena::Value1,
            true => Nonbasethrdena::Value2,
        }
    }
    #[doc = "processor can enter Thread mode only when no exception is active."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Nonbasethrdena::Value1
    }
    #[doc = "processor can enter Thread mode from any level under the control of an EXC_RETURN value, see Exception returnException return occurs when the processor is in Handler mode and executes one of the following instructions to load the EXC_RETURN value into the PC:an LDM or POP instruction that loads the PCan LDR instruction with PC as the destinationa BX instruction using any register.EXC_RETURN is the value loaded into the LR on exception entry. The exception mechanism relies on this value to detect when the processor has completed an exception handler. The lowest five bits of this value provide information on the return stack and processor mode. shows the EXC_RETURN values with a description of the exception return behavior. All EXC_RETURN values have bits\\[31:5\\]
set to one. When this value is loaded into the PC it indicates to the processor that the exception is complete, and the processor initiates the appropriate exception return sequence.Exception return behaviorEXC_RETURN\\[31:0\\]Description 0xFFFFFFF1 Return to Handler mode, exception return uses non-floating-point state from the MSP and execution uses MSP after return. 0xFFFFFFF9 Return to Thread mode, exception return uses non-floating-point state from MSP and execution uses MSP after return. 0xFFFFFFFD Return to Thread mode, exception return uses non-floating-point state from the PSP and execution uses PSP after return. 0xFFFFFFE1 Return to Handler mode, exception return uses floating-point-state from MSP and execution uses MSP after return. 0xFFFFFFE9 Return to Thread mode, exception return uses floating-point state from MSP and execution uses MSP after return. 0xFFFFFFED Return to Thread mode, exception return uses floating-point state from PSP and execution uses PSP after return. ."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Nonbasethrdena::Value2
    }
}
#[doc = "Field `NONBASETHRDENA` writer - Non Base Thread Mode Enable"]
pub type NonbasethrdenaW<'a, REG> = crate::BitWriter<'a, REG, Nonbasethrdena>;
impl<'a, REG> NonbasethrdenaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "processor can enter Thread mode only when no exception is active."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Nonbasethrdena::Value1)
    }
    #[doc = "processor can enter Thread mode from any level under the control of an EXC_RETURN value, see Exception returnException return occurs when the processor is in Handler mode and executes one of the following instructions to load the EXC_RETURN value into the PC:an LDM or POP instruction that loads the PCan LDR instruction with PC as the destinationa BX instruction using any register.EXC_RETURN is the value loaded into the LR on exception entry. The exception mechanism relies on this value to detect when the processor has completed an exception handler. The lowest five bits of this value provide information on the return stack and processor mode. shows the EXC_RETURN values with a description of the exception return behavior. All EXC_RETURN values have bits\\[31:5\\]
set to one. When this value is loaded into the PC it indicates to the processor that the exception is complete, and the processor initiates the appropriate exception return sequence.Exception return behaviorEXC_RETURN\\[31:0\\]Description 0xFFFFFFF1 Return to Handler mode, exception return uses non-floating-point state from the MSP and execution uses MSP after return. 0xFFFFFFF9 Return to Thread mode, exception return uses non-floating-point state from MSP and execution uses MSP after return. 0xFFFFFFFD Return to Thread mode, exception return uses non-floating-point state from the PSP and execution uses PSP after return. 0xFFFFFFE1 Return to Handler mode, exception return uses floating-point-state from MSP and execution uses MSP after return. 0xFFFFFFE9 Return to Thread mode, exception return uses floating-point state from MSP and execution uses MSP after return. 0xFFFFFFED Return to Thread mode, exception return uses floating-point state from PSP and execution uses PSP after return. ."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Nonbasethrdena::Value2)
    }
}
#[doc = "User Set Pending Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Usersetmpend {
    #[doc = "0: disable"]
    Value1 = 0,
    #[doc = "1: enable"]
    Value2 = 1,
}
impl From<Usersetmpend> for bool {
    #[inline(always)]
    fn from(variant: Usersetmpend) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USERSETMPEND` reader - User Set Pending Enable"]
pub type UsersetmpendR = crate::BitReader<Usersetmpend>;
impl UsersetmpendR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Usersetmpend {
        match self.bits {
            false => Usersetmpend::Value1,
            true => Usersetmpend::Value2,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Usersetmpend::Value1
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Usersetmpend::Value2
    }
}
#[doc = "Field `USERSETMPEND` writer - User Set Pending Enable"]
pub type UsersetmpendW<'a, REG> = crate::BitWriter<'a, REG, Usersetmpend>;
impl<'a, REG> UsersetmpendW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Usersetmpend::Value1)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Usersetmpend::Value2)
    }
}
#[doc = "Unaligned Access Trap Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UnalignTrp {
    #[doc = "0: do not trap unaligned halfword and word accesses"]
    Value1 = 0,
    #[doc = "1: trap unaligned halfword and word accesses."]
    Value2 = 1,
}
impl From<UnalignTrp> for bool {
    #[inline(always)]
    fn from(variant: UnalignTrp) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UNALIGN_TRP` reader - Unaligned Access Trap Enable"]
pub type UnalignTrpR = crate::BitReader<UnalignTrp>;
impl UnalignTrpR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> UnalignTrp {
        match self.bits {
            false => UnalignTrp::Value1,
            true => UnalignTrp::Value2,
        }
    }
    #[doc = "do not trap unaligned halfword and word accesses"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == UnalignTrp::Value1
    }
    #[doc = "trap unaligned halfword and word accesses."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == UnalignTrp::Value2
    }
}
#[doc = "Field `UNALIGN_TRP` writer - Unaligned Access Trap Enable"]
pub type UnalignTrpW<'a, REG> = crate::BitWriter<'a, REG, UnalignTrp>;
impl<'a, REG> UnalignTrpW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "do not trap unaligned halfword and word accesses"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(UnalignTrp::Value1)
    }
    #[doc = "trap unaligned halfword and word accesses."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(UnalignTrp::Value2)
    }
}
#[doc = "Divide by Zero Trap Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Div0Trp {
    #[doc = "0: do not trap divide by 0"]
    Value1 = 0,
    #[doc = "1: trap divide by 0."]
    Value2 = 1,
}
impl From<Div0Trp> for bool {
    #[inline(always)]
    fn from(variant: Div0Trp) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIV_0_TRP` reader - Divide by Zero Trap Enable"]
pub type Div0TrpR = crate::BitReader<Div0Trp>;
impl Div0TrpR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Div0Trp {
        match self.bits {
            false => Div0Trp::Value1,
            true => Div0Trp::Value2,
        }
    }
    #[doc = "do not trap divide by 0"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Div0Trp::Value1
    }
    #[doc = "trap divide by 0."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Div0Trp::Value2
    }
}
#[doc = "Field `DIV_0_TRP` writer - Divide by Zero Trap Enable"]
pub type Div0TrpW<'a, REG> = crate::BitWriter<'a, REG, Div0Trp>;
impl<'a, REG> Div0TrpW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "do not trap divide by 0"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Div0Trp::Value1)
    }
    #[doc = "trap divide by 0."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Div0Trp::Value2)
    }
}
#[doc = "Bus Fault Hard Fault and NMI Ignore\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bfhfnmign {
    #[doc = "0: data bus faults caused by load and store instructions cause a lock-up"]
    Value1 = 0,
    #[doc = "1: handlers running at priority -1 and -2 ignore data bus faults caused by load and store instructions."]
    Value2 = 1,
}
impl From<Bfhfnmign> for bool {
    #[inline(always)]
    fn from(variant: Bfhfnmign) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BFHFNMIGN` reader - Bus Fault Hard Fault and NMI Ignore"]
pub type BfhfnmignR = crate::BitReader<Bfhfnmign>;
impl BfhfnmignR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bfhfnmign {
        match self.bits {
            false => Bfhfnmign::Value1,
            true => Bfhfnmign::Value2,
        }
    }
    #[doc = "data bus faults caused by load and store instructions cause a lock-up"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Bfhfnmign::Value1
    }
    #[doc = "handlers running at priority -1 and -2 ignore data bus faults caused by load and store instructions."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Bfhfnmign::Value2
    }
}
#[doc = "Field `BFHFNMIGN` writer - Bus Fault Hard Fault and NMI Ignore"]
pub type BfhfnmignW<'a, REG> = crate::BitWriter<'a, REG, Bfhfnmign>;
impl<'a, REG> BfhfnmignW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "data bus faults caused by load and store instructions cause a lock-up"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Bfhfnmign::Value1)
    }
    #[doc = "handlers running at priority -1 and -2 ignore data bus faults caused by load and store instructions."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Bfhfnmign::Value2)
    }
}
#[doc = "Stack Alignment\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Stkalign {
    #[doc = "0: 4-byte aligned"]
    Value1 = 0,
    #[doc = "1: 8-byte aligned."]
    Value2 = 1,
}
impl From<Stkalign> for bool {
    #[inline(always)]
    fn from(variant: Stkalign) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STKALIGN` reader - Stack Alignment"]
pub type StkalignR = crate::BitReader<Stkalign>;
impl StkalignR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Stkalign {
        match self.bits {
            false => Stkalign::Value1,
            true => Stkalign::Value2,
        }
    }
    #[doc = "4-byte aligned"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Stkalign::Value1
    }
    #[doc = "8-byte aligned."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Stkalign::Value2
    }
}
#[doc = "Field `STKALIGN` writer - Stack Alignment"]
pub type StkalignW<'a, REG> = crate::BitWriter<'a, REG, Stkalign>;
impl<'a, REG> StkalignW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "4-byte aligned"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Stkalign::Value1)
    }
    #[doc = "8-byte aligned."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Stkalign::Value2)
    }
}
impl R {
    #[doc = "Bit 0 - Non Base Thread Mode Enable"]
    #[inline(always)]
    pub fn nonbasethrdena(&self) -> NonbasethrdenaR {
        NonbasethrdenaR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - User Set Pending Enable"]
    #[inline(always)]
    pub fn usersetmpend(&self) -> UsersetmpendR {
        UsersetmpendR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - Unaligned Access Trap Enable"]
    #[inline(always)]
    pub fn unalign_trp(&self) -> UnalignTrpR {
        UnalignTrpR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Divide by Zero Trap Enable"]
    #[inline(always)]
    pub fn div_0_trp(&self) -> Div0TrpR {
        Div0TrpR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - Bus Fault Hard Fault and NMI Ignore"]
    #[inline(always)]
    pub fn bfhfnmign(&self) -> BfhfnmignR {
        BfhfnmignR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Stack Alignment"]
    #[inline(always)]
    pub fn stkalign(&self) -> StkalignR {
        StkalignR::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Non Base Thread Mode Enable"]
    #[inline(always)]
    #[must_use]
    pub fn nonbasethrdena(&mut self) -> NonbasethrdenaW<CcrSpec> {
        NonbasethrdenaW::new(self, 0)
    }
    #[doc = "Bit 1 - User Set Pending Enable"]
    #[inline(always)]
    #[must_use]
    pub fn usersetmpend(&mut self) -> UsersetmpendW<CcrSpec> {
        UsersetmpendW::new(self, 1)
    }
    #[doc = "Bit 3 - Unaligned Access Trap Enable"]
    #[inline(always)]
    #[must_use]
    pub fn unalign_trp(&mut self) -> UnalignTrpW<CcrSpec> {
        UnalignTrpW::new(self, 3)
    }
    #[doc = "Bit 4 - Divide by Zero Trap Enable"]
    #[inline(always)]
    #[must_use]
    pub fn div_0_trp(&mut self) -> Div0TrpW<CcrSpec> {
        Div0TrpW::new(self, 4)
    }
    #[doc = "Bit 8 - Bus Fault Hard Fault and NMI Ignore"]
    #[inline(always)]
    #[must_use]
    pub fn bfhfnmign(&mut self) -> BfhfnmignW<CcrSpec> {
        BfhfnmignW::new(self, 8)
    }
    #[doc = "Bit 9 - Stack Alignment"]
    #[inline(always)]
    #[must_use]
    pub fn stkalign(&mut self) -> StkalignW<CcrSpec> {
        StkalignW::new(self, 9)
    }
}
#[doc = "Configuration and Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CcrSpec;
impl crate::RegisterSpec for CcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ccr::R`](R) reader structure"]
impl crate::Readable for CcrSpec {}
#[doc = "`write(|w| ..)` method takes [`ccr::W`](W) writer structure"]
impl crate::Writable for CcrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CCR to value 0x0200"]
impl crate::Resettable for CcrSpec {
    const RESET_VALUE: u32 = 0x0200;
}
