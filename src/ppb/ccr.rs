#[doc = "Register `CCR` reader"]
pub struct R(crate::R<CCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CCR` writer"]
pub struct W(crate::W<CCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<CCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `NONBASETHRDENA` reader - Non Base Thread Mode Enable"]
pub type NONBASETHRDENA_R = crate::BitReader<NONBASETHRDENA_A>;
#[doc = "Non Base Thread Mode Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NONBASETHRDENA_A {
    #[doc = "0: processor can enter Thread mode only when no exception is active."]
    VALUE1 = 0,
    #[doc = "1: processor can enter Thread mode from any level under the control of an EXC_RETURN value, see Exception returnException return occurs when the processor is in Handler mode and executes one of the following instructions to load the EXC_RETURN value into the PC:an LDM or POP instruction that loads the PCan LDR instruction with PC as the destinationa BX instruction using any register.EXC_RETURN is the value loaded into the LR on exception entry. The exception mechanism relies on this value to detect when the processor has completed an exception handler. The lowest five bits of this value provide information on the return stack and processor mode. shows the EXC_RETURN values with a description of the exception return behavior. All EXC_RETURN values have bits\\[31:5\\]
set to one. When this value is loaded into the PC it indicates to the processor that the exception is complete, and the processor initiates the appropriate exception return sequence.Exception return behaviorEXC_RETURN\\[31:0\\]Description 0xFFFFFFF1 Return to Handler mode, exception return uses non-floating-point state from the MSP and execution uses MSP after return. 0xFFFFFFF9 Return to Thread mode, exception return uses non-floating-point state from MSP and execution uses MSP after return. 0xFFFFFFFD Return to Thread mode, exception return uses non-floating-point state from the PSP and execution uses PSP after return. 0xFFFFFFE1 Return to Handler mode, exception return uses floating-point-state from MSP and execution uses MSP after return. 0xFFFFFFE9 Return to Thread mode, exception return uses floating-point state from MSP and execution uses MSP after return. 0xFFFFFFED Return to Thread mode, exception return uses floating-point state from PSP and execution uses PSP after return. ."]
    VALUE2 = 1,
}
impl From<NONBASETHRDENA_A> for bool {
    #[inline(always)]
    fn from(variant: NONBASETHRDENA_A) -> Self {
        variant as u8 != 0
    }
}
impl NONBASETHRDENA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NONBASETHRDENA_A {
        match self.bits {
            false => NONBASETHRDENA_A::VALUE1,
            true => NONBASETHRDENA_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == NONBASETHRDENA_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == NONBASETHRDENA_A::VALUE2
    }
}
#[doc = "Field `NONBASETHRDENA` writer - Non Base Thread Mode Enable"]
pub type NONBASETHRDENA_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCR_SPEC, NONBASETHRDENA_A, O>;
impl<'a, const O: u8> NONBASETHRDENA_W<'a, O> {
    #[doc = "processor can enter Thread mode only when no exception is active."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(NONBASETHRDENA_A::VALUE1)
    }
    #[doc = "processor can enter Thread mode from any level under the control of an EXC_RETURN value, see Exception returnException return occurs when the processor is in Handler mode and executes one of the following instructions to load the EXC_RETURN value into the PC:an LDM or POP instruction that loads the PCan LDR instruction with PC as the destinationa BX instruction using any register.EXC_RETURN is the value loaded into the LR on exception entry. The exception mechanism relies on this value to detect when the processor has completed an exception handler. The lowest five bits of this value provide information on the return stack and processor mode. shows the EXC_RETURN values with a description of the exception return behavior. All EXC_RETURN values have bits\\[31:5\\]
set to one. When this value is loaded into the PC it indicates to the processor that the exception is complete, and the processor initiates the appropriate exception return sequence.Exception return behaviorEXC_RETURN\\[31:0\\]Description 0xFFFFFFF1 Return to Handler mode, exception return uses non-floating-point state from the MSP and execution uses MSP after return. 0xFFFFFFF9 Return to Thread mode, exception return uses non-floating-point state from MSP and execution uses MSP after return. 0xFFFFFFFD Return to Thread mode, exception return uses non-floating-point state from the PSP and execution uses PSP after return. 0xFFFFFFE1 Return to Handler mode, exception return uses floating-point-state from MSP and execution uses MSP after return. 0xFFFFFFE9 Return to Thread mode, exception return uses floating-point state from MSP and execution uses MSP after return. 0xFFFFFFED Return to Thread mode, exception return uses floating-point state from PSP and execution uses PSP after return. ."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(NONBASETHRDENA_A::VALUE2)
    }
}
#[doc = "Field `USERSETMPEND` reader - User Set Pending Enable"]
pub type USERSETMPEND_R = crate::BitReader<USERSETMPEND_A>;
#[doc = "User Set Pending Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USERSETMPEND_A {
    #[doc = "0: disable"]
    VALUE1 = 0,
    #[doc = "1: enable"]
    VALUE2 = 1,
}
impl From<USERSETMPEND_A> for bool {
    #[inline(always)]
    fn from(variant: USERSETMPEND_A) -> Self {
        variant as u8 != 0
    }
}
impl USERSETMPEND_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USERSETMPEND_A {
        match self.bits {
            false => USERSETMPEND_A::VALUE1,
            true => USERSETMPEND_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == USERSETMPEND_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == USERSETMPEND_A::VALUE2
    }
}
#[doc = "Field `USERSETMPEND` writer - User Set Pending Enable"]
pub type USERSETMPEND_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCR_SPEC, USERSETMPEND_A, O>;
impl<'a, const O: u8> USERSETMPEND_W<'a, O> {
    #[doc = "disable"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(USERSETMPEND_A::VALUE1)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(USERSETMPEND_A::VALUE2)
    }
}
#[doc = "Field `UNALIGN_TRP` reader - Unaligned Access Trap Enable"]
pub type UNALIGN_TRP_R = crate::BitReader<UNALIGN_TRP_A>;
#[doc = "Unaligned Access Trap Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UNALIGN_TRP_A {
    #[doc = "0: do not trap unaligned halfword and word accesses"]
    VALUE1 = 0,
    #[doc = "1: trap unaligned halfword and word accesses."]
    VALUE2 = 1,
}
impl From<UNALIGN_TRP_A> for bool {
    #[inline(always)]
    fn from(variant: UNALIGN_TRP_A) -> Self {
        variant as u8 != 0
    }
}
impl UNALIGN_TRP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UNALIGN_TRP_A {
        match self.bits {
            false => UNALIGN_TRP_A::VALUE1,
            true => UNALIGN_TRP_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == UNALIGN_TRP_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == UNALIGN_TRP_A::VALUE2
    }
}
#[doc = "Field `UNALIGN_TRP` writer - Unaligned Access Trap Enable"]
pub type UNALIGN_TRP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCR_SPEC, UNALIGN_TRP_A, O>;
impl<'a, const O: u8> UNALIGN_TRP_W<'a, O> {
    #[doc = "do not trap unaligned halfword and word accesses"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(UNALIGN_TRP_A::VALUE1)
    }
    #[doc = "trap unaligned halfword and word accesses."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(UNALIGN_TRP_A::VALUE2)
    }
}
#[doc = "Field `DIV_0_TRP` reader - Divide by Zero Trap Enable"]
pub type DIV_0_TRP_R = crate::BitReader<DIV_0_TRP_A>;
#[doc = "Divide by Zero Trap Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DIV_0_TRP_A {
    #[doc = "0: do not trap divide by 0"]
    VALUE1 = 0,
    #[doc = "1: trap divide by 0."]
    VALUE2 = 1,
}
impl From<DIV_0_TRP_A> for bool {
    #[inline(always)]
    fn from(variant: DIV_0_TRP_A) -> Self {
        variant as u8 != 0
    }
}
impl DIV_0_TRP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIV_0_TRP_A {
        match self.bits {
            false => DIV_0_TRP_A::VALUE1,
            true => DIV_0_TRP_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == DIV_0_TRP_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == DIV_0_TRP_A::VALUE2
    }
}
#[doc = "Field `DIV_0_TRP` writer - Divide by Zero Trap Enable"]
pub type DIV_0_TRP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCR_SPEC, DIV_0_TRP_A, O>;
impl<'a, const O: u8> DIV_0_TRP_W<'a, O> {
    #[doc = "do not trap divide by 0"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(DIV_0_TRP_A::VALUE1)
    }
    #[doc = "trap divide by 0."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(DIV_0_TRP_A::VALUE2)
    }
}
#[doc = "Field `BFHFNMIGN` reader - Bus Fault Hard Fault and NMI Ignore"]
pub type BFHFNMIGN_R = crate::BitReader<BFHFNMIGN_A>;
#[doc = "Bus Fault Hard Fault and NMI Ignore\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BFHFNMIGN_A {
    #[doc = "0: data bus faults caused by load and store instructions cause a lock-up"]
    VALUE1 = 0,
    #[doc = "1: handlers running at priority -1 and -2 ignore data bus faults caused by load and store instructions."]
    VALUE2 = 1,
}
impl From<BFHFNMIGN_A> for bool {
    #[inline(always)]
    fn from(variant: BFHFNMIGN_A) -> Self {
        variant as u8 != 0
    }
}
impl BFHFNMIGN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BFHFNMIGN_A {
        match self.bits {
            false => BFHFNMIGN_A::VALUE1,
            true => BFHFNMIGN_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == BFHFNMIGN_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == BFHFNMIGN_A::VALUE2
    }
}
#[doc = "Field `BFHFNMIGN` writer - Bus Fault Hard Fault and NMI Ignore"]
pub type BFHFNMIGN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCR_SPEC, BFHFNMIGN_A, O>;
impl<'a, const O: u8> BFHFNMIGN_W<'a, O> {
    #[doc = "data bus faults caused by load and store instructions cause a lock-up"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(BFHFNMIGN_A::VALUE1)
    }
    #[doc = "handlers running at priority -1 and -2 ignore data bus faults caused by load and store instructions."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(BFHFNMIGN_A::VALUE2)
    }
}
#[doc = "Field `STKALIGN` reader - Stack Alignment"]
pub type STKALIGN_R = crate::BitReader<STKALIGN_A>;
#[doc = "Stack Alignment\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum STKALIGN_A {
    #[doc = "0: 4-byte aligned"]
    VALUE1 = 0,
    #[doc = "1: 8-byte aligned."]
    VALUE2 = 1,
}
impl From<STKALIGN_A> for bool {
    #[inline(always)]
    fn from(variant: STKALIGN_A) -> Self {
        variant as u8 != 0
    }
}
impl STKALIGN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STKALIGN_A {
        match self.bits {
            false => STKALIGN_A::VALUE1,
            true => STKALIGN_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == STKALIGN_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == STKALIGN_A::VALUE2
    }
}
#[doc = "Field `STKALIGN` writer - Stack Alignment"]
pub type STKALIGN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCR_SPEC, STKALIGN_A, O>;
impl<'a, const O: u8> STKALIGN_W<'a, O> {
    #[doc = "4-byte aligned"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(STKALIGN_A::VALUE1)
    }
    #[doc = "8-byte aligned."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(STKALIGN_A::VALUE2)
    }
}
impl R {
    #[doc = "Bit 0 - Non Base Thread Mode Enable"]
    #[inline(always)]
    pub fn nonbasethrdena(&self) -> NONBASETHRDENA_R {
        NONBASETHRDENA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - User Set Pending Enable"]
    #[inline(always)]
    pub fn usersetmpend(&self) -> USERSETMPEND_R {
        USERSETMPEND_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - Unaligned Access Trap Enable"]
    #[inline(always)]
    pub fn unalign_trp(&self) -> UNALIGN_TRP_R {
        UNALIGN_TRP_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Divide by Zero Trap Enable"]
    #[inline(always)]
    pub fn div_0_trp(&self) -> DIV_0_TRP_R {
        DIV_0_TRP_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - Bus Fault Hard Fault and NMI Ignore"]
    #[inline(always)]
    pub fn bfhfnmign(&self) -> BFHFNMIGN_R {
        BFHFNMIGN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Stack Alignment"]
    #[inline(always)]
    pub fn stkalign(&self) -> STKALIGN_R {
        STKALIGN_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Non Base Thread Mode Enable"]
    #[inline(always)]
    #[must_use]
    pub fn nonbasethrdena(&mut self) -> NONBASETHRDENA_W<0> {
        NONBASETHRDENA_W::new(self)
    }
    #[doc = "Bit 1 - User Set Pending Enable"]
    #[inline(always)]
    #[must_use]
    pub fn usersetmpend(&mut self) -> USERSETMPEND_W<1> {
        USERSETMPEND_W::new(self)
    }
    #[doc = "Bit 3 - Unaligned Access Trap Enable"]
    #[inline(always)]
    #[must_use]
    pub fn unalign_trp(&mut self) -> UNALIGN_TRP_W<3> {
        UNALIGN_TRP_W::new(self)
    }
    #[doc = "Bit 4 - Divide by Zero Trap Enable"]
    #[inline(always)]
    #[must_use]
    pub fn div_0_trp(&mut self) -> DIV_0_TRP_W<4> {
        DIV_0_TRP_W::new(self)
    }
    #[doc = "Bit 8 - Bus Fault Hard Fault and NMI Ignore"]
    #[inline(always)]
    #[must_use]
    pub fn bfhfnmign(&mut self) -> BFHFNMIGN_W<8> {
        BFHFNMIGN_W::new(self)
    }
    #[doc = "Bit 9 - Stack Alignment"]
    #[inline(always)]
    #[must_use]
    pub fn stkalign(&mut self) -> STKALIGN_W<9> {
        STKALIGN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Configuration and Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ccr](index.html) module"]
pub struct CCR_SPEC;
impl crate::RegisterSpec for CCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ccr::R](R) reader structure"]
impl crate::Readable for CCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ccr::W](W) writer structure"]
impl crate::Writable for CCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CCR to value 0x0200"]
impl crate::Resettable for CCR_SPEC {
    const RESET_VALUE: Self::Ux = 0x0200;
}
