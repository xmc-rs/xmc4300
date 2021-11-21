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
#[doc = "Non Base Thread Mode Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Field `NONBASETHRDENA` reader - Non Base Thread Mode Enable"]
pub struct NONBASETHRDENA_R(crate::FieldReader<bool, NONBASETHRDENA_A>);
impl NONBASETHRDENA_R {
    pub(crate) fn new(bits: bool) -> Self {
        NONBASETHRDENA_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
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
        **self == NONBASETHRDENA_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == NONBASETHRDENA_A::VALUE2
    }
}
impl core::ops::Deref for NONBASETHRDENA_R {
    type Target = crate::FieldReader<bool, NONBASETHRDENA_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NONBASETHRDENA` writer - Non Base Thread Mode Enable"]
pub struct NONBASETHRDENA_W<'a> {
    w: &'a mut W,
}
impl<'a> NONBASETHRDENA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: NONBASETHRDENA_A) -> &'a mut W {
        self.bit(variant.into())
    }
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
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "User Set Pending Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Field `USERSETMPEND` reader - User Set Pending Enable"]
pub struct USERSETMPEND_R(crate::FieldReader<bool, USERSETMPEND_A>);
impl USERSETMPEND_R {
    pub(crate) fn new(bits: bool) -> Self {
        USERSETMPEND_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
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
        **self == USERSETMPEND_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == USERSETMPEND_A::VALUE2
    }
}
impl core::ops::Deref for USERSETMPEND_R {
    type Target = crate::FieldReader<bool, USERSETMPEND_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USERSETMPEND` writer - User Set Pending Enable"]
pub struct USERSETMPEND_W<'a> {
    w: &'a mut W,
}
impl<'a> USERSETMPEND_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USERSETMPEND_A) -> &'a mut W {
        self.bit(variant.into())
    }
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
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Unaligned Access Trap Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Field `UNALIGN_TRP` reader - Unaligned Access Trap Enable"]
pub struct UNALIGN_TRP_R(crate::FieldReader<bool, UNALIGN_TRP_A>);
impl UNALIGN_TRP_R {
    pub(crate) fn new(bits: bool) -> Self {
        UNALIGN_TRP_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
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
        **self == UNALIGN_TRP_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == UNALIGN_TRP_A::VALUE2
    }
}
impl core::ops::Deref for UNALIGN_TRP_R {
    type Target = crate::FieldReader<bool, UNALIGN_TRP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UNALIGN_TRP` writer - Unaligned Access Trap Enable"]
pub struct UNALIGN_TRP_W<'a> {
    w: &'a mut W,
}
impl<'a> UNALIGN_TRP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UNALIGN_TRP_A) -> &'a mut W {
        self.bit(variant.into())
    }
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
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "Divide by Zero Trap Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Field `DIV_0_TRP` reader - Divide by Zero Trap Enable"]
pub struct DIV_0_TRP_R(crate::FieldReader<bool, DIV_0_TRP_A>);
impl DIV_0_TRP_R {
    pub(crate) fn new(bits: bool) -> Self {
        DIV_0_TRP_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
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
        **self == DIV_0_TRP_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == DIV_0_TRP_A::VALUE2
    }
}
impl core::ops::Deref for DIV_0_TRP_R {
    type Target = crate::FieldReader<bool, DIV_0_TRP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIV_0_TRP` writer - Divide by Zero Trap Enable"]
pub struct DIV_0_TRP_W<'a> {
    w: &'a mut W,
}
impl<'a> DIV_0_TRP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIV_0_TRP_A) -> &'a mut W {
        self.bit(variant.into())
    }
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
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "Bus Fault Hard Fault and NMI Ignore\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Field `BFHFNMIGN` reader - Bus Fault Hard Fault and NMI Ignore"]
pub struct BFHFNMIGN_R(crate::FieldReader<bool, BFHFNMIGN_A>);
impl BFHFNMIGN_R {
    pub(crate) fn new(bits: bool) -> Self {
        BFHFNMIGN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
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
        **self == BFHFNMIGN_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == BFHFNMIGN_A::VALUE2
    }
}
impl core::ops::Deref for BFHFNMIGN_R {
    type Target = crate::FieldReader<bool, BFHFNMIGN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BFHFNMIGN` writer - Bus Fault Hard Fault and NMI Ignore"]
pub struct BFHFNMIGN_W<'a> {
    w: &'a mut W,
}
impl<'a> BFHFNMIGN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BFHFNMIGN_A) -> &'a mut W {
        self.bit(variant.into())
    }
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
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
#[doc = "Stack Alignment\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Field `STKALIGN` reader - Stack Alignment"]
pub struct STKALIGN_R(crate::FieldReader<bool, STKALIGN_A>);
impl STKALIGN_R {
    pub(crate) fn new(bits: bool) -> Self {
        STKALIGN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
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
        **self == STKALIGN_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == STKALIGN_A::VALUE2
    }
}
impl core::ops::Deref for STKALIGN_R {
    type Target = crate::FieldReader<bool, STKALIGN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STKALIGN` writer - Stack Alignment"]
pub struct STKALIGN_W<'a> {
    w: &'a mut W,
}
impl<'a> STKALIGN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: STKALIGN_A) -> &'a mut W {
        self.bit(variant.into())
    }
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
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Non Base Thread Mode Enable"]
    #[inline(always)]
    pub fn nonbasethrdena(&self) -> NONBASETHRDENA_R {
        NONBASETHRDENA_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - User Set Pending Enable"]
    #[inline(always)]
    pub fn usersetmpend(&self) -> USERSETMPEND_R {
        USERSETMPEND_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Unaligned Access Trap Enable"]
    #[inline(always)]
    pub fn unalign_trp(&self) -> UNALIGN_TRP_R {
        UNALIGN_TRP_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Divide by Zero Trap Enable"]
    #[inline(always)]
    pub fn div_0_trp(&self) -> DIV_0_TRP_R {
        DIV_0_TRP_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Bus Fault Hard Fault and NMI Ignore"]
    #[inline(always)]
    pub fn bfhfnmign(&self) -> BFHFNMIGN_R {
        BFHFNMIGN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Stack Alignment"]
    #[inline(always)]
    pub fn stkalign(&self) -> STKALIGN_R {
        STKALIGN_R::new(((self.bits >> 9) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Non Base Thread Mode Enable"]
    #[inline(always)]
    pub fn nonbasethrdena(&mut self) -> NONBASETHRDENA_W {
        NONBASETHRDENA_W { w: self }
    }
    #[doc = "Bit 1 - User Set Pending Enable"]
    #[inline(always)]
    pub fn usersetmpend(&mut self) -> USERSETMPEND_W {
        USERSETMPEND_W { w: self }
    }
    #[doc = "Bit 3 - Unaligned Access Trap Enable"]
    #[inline(always)]
    pub fn unalign_trp(&mut self) -> UNALIGN_TRP_W {
        UNALIGN_TRP_W { w: self }
    }
    #[doc = "Bit 4 - Divide by Zero Trap Enable"]
    #[inline(always)]
    pub fn div_0_trp(&mut self) -> DIV_0_TRP_W {
        DIV_0_TRP_W { w: self }
    }
    #[doc = "Bit 8 - Bus Fault Hard Fault and NMI Ignore"]
    #[inline(always)]
    pub fn bfhfnmign(&mut self) -> BFHFNMIGN_W {
        BFHFNMIGN_W { w: self }
    }
    #[doc = "Bit 9 - Stack Alignment"]
    #[inline(always)]
    pub fn stkalign(&mut self) -> STKALIGN_W {
        STKALIGN_W { w: self }
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
}
#[doc = "`reset()` method sets CCR to value 0x0200"]
impl crate::Resettable for CCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0200
    }
}
