#[doc = "Register `ICSR` reader"]
pub struct R(crate::R<ICSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ICSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ICSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ICSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ICSR` writer"]
pub struct W(crate::W<ICSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ICSR_SPEC>;
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
impl From<crate::W<ICSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ICSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VECTACTIVE` reader - Active exception number"]
pub type VECTACTIVE_R = crate::FieldReader<u16, VECTACTIVE_A>;
#[doc = "Active exception number\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum VECTACTIVE_A {
    #[doc = "0: Thread mode"]
    VALUE1 = 0,
}
impl From<VECTACTIVE_A> for u16 {
    #[inline(always)]
    fn from(variant: VECTACTIVE_A) -> Self {
        variant as _
    }
}
impl VECTACTIVE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<VECTACTIVE_A> {
        match self.bits {
            0 => Some(VECTACTIVE_A::VALUE1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == VECTACTIVE_A::VALUE1
    }
}
#[doc = "Field `RETTOBASE` reader - Return to Base"]
pub type RETTOBASE_R = crate::BitReader<RETTOBASE_A>;
#[doc = "Return to Base\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RETTOBASE_A {
    #[doc = "0: there are preempted active exceptions to execute"]
    VALUE1 = 0,
    #[doc = "1: there are no active exceptions, or the currently-executing exception is the only active exception."]
    VALUE2 = 1,
}
impl From<RETTOBASE_A> for bool {
    #[inline(always)]
    fn from(variant: RETTOBASE_A) -> Self {
        variant as u8 != 0
    }
}
impl RETTOBASE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RETTOBASE_A {
        match self.bits {
            false => RETTOBASE_A::VALUE1,
            true => RETTOBASE_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == RETTOBASE_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == RETTOBASE_A::VALUE2
    }
}
#[doc = "Field `VECTPENDING` reader - Vector Pending"]
pub type VECTPENDING_R = crate::FieldReader<u8, VECTPENDING_A>;
#[doc = "Vector Pending\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum VECTPENDING_A {
    #[doc = "0: no pending exceptions"]
    VALUE1 = 0,
}
impl From<VECTPENDING_A> for u8 {
    #[inline(always)]
    fn from(variant: VECTPENDING_A) -> Self {
        variant as _
    }
}
impl VECTPENDING_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<VECTPENDING_A> {
        match self.bits {
            0 => Some(VECTPENDING_A::VALUE1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == VECTPENDING_A::VALUE1
    }
}
#[doc = "Field `ISRPENDING` reader - Interrupt pending flag"]
pub type ISRPENDING_R = crate::BitReader<ISRPENDING_A>;
#[doc = "Interrupt pending flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ISRPENDING_A {
    #[doc = "0: interrupt not pending"]
    VALUE1 = 0,
    #[doc = "1: interrupt pending."]
    VALUE2 = 1,
}
impl From<ISRPENDING_A> for bool {
    #[inline(always)]
    fn from(variant: ISRPENDING_A) -> Self {
        variant as u8 != 0
    }
}
impl ISRPENDING_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ISRPENDING_A {
        match self.bits {
            false => ISRPENDING_A::VALUE1,
            true => ISRPENDING_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == ISRPENDING_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == ISRPENDING_A::VALUE2
    }
}
#[doc = "SysTick exception clear-pending bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PENDSTCLR_AW {
    #[doc = "0: no effect"]
    VALUE1 = 0,
    #[doc = "1: removes the pending state from the SysTick exception."]
    VALUE2 = 1,
}
impl From<PENDSTCLR_AW> for bool {
    #[inline(always)]
    fn from(variant: PENDSTCLR_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PENDSTCLR` writer - SysTick exception clear-pending bit"]
pub type PENDSTCLR_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICSR_SPEC, PENDSTCLR_AW, O>;
impl<'a, const O: u8> PENDSTCLR_W<'a, O> {
    #[doc = "no effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(PENDSTCLR_AW::VALUE1)
    }
    #[doc = "removes the pending state from the SysTick exception."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(PENDSTCLR_AW::VALUE2)
    }
}
#[doc = "Field `PENDSTSET` reader - SysTick exception set-pending bit"]
pub type PENDSTSET_R = crate::BitReader<PENDSTSET_A>;
#[doc = "SysTick exception set-pending bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PENDSTSET_A {
    #[doc = "0: no effect"]
    VALUE1 = 0,
    #[doc = "1: changes SysTick exception state to pending."]
    VALUE2 = 1,
}
impl From<PENDSTSET_A> for bool {
    #[inline(always)]
    fn from(variant: PENDSTSET_A) -> Self {
        variant as u8 != 0
    }
}
impl PENDSTSET_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PENDSTSET_A {
        match self.bits {
            false => PENDSTSET_A::VALUE1,
            true => PENDSTSET_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PENDSTSET_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PENDSTSET_A::VALUE2
    }
}
#[doc = "Field `PENDSTSET` writer - SysTick exception set-pending bit"]
pub type PENDSTSET_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICSR_SPEC, PENDSTSET_A, O>;
impl<'a, const O: u8> PENDSTSET_W<'a, O> {
    #[doc = "no effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(PENDSTSET_A::VALUE1)
    }
    #[doc = "changes SysTick exception state to pending."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(PENDSTSET_A::VALUE2)
    }
}
#[doc = "PendSV clear-pending bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PENDSVCLR_AW {
    #[doc = "0: no effect"]
    VALUE1 = 0,
    #[doc = "1: removes the pending state from the PendSV exception."]
    VALUE2 = 1,
}
impl From<PENDSVCLR_AW> for bool {
    #[inline(always)]
    fn from(variant: PENDSVCLR_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PENDSVCLR` writer - PendSV clear-pending bit"]
pub type PENDSVCLR_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICSR_SPEC, PENDSVCLR_AW, O>;
impl<'a, const O: u8> PENDSVCLR_W<'a, O> {
    #[doc = "no effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(PENDSVCLR_AW::VALUE1)
    }
    #[doc = "removes the pending state from the PendSV exception."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(PENDSVCLR_AW::VALUE2)
    }
}
#[doc = "Field `PENDSVSET` reader - PendSV set-pending bit: 0b0=no effect, 0b1=changes PendSV exception state to pending., 0b0=PendSV exception is not pending, 0b1=PendSV exception is pending.,"]
pub type PENDSVSET_R = crate::BitReader<bool>;
#[doc = "Field `PENDSVSET` writer - PendSV set-pending bit: 0b0=no effect, 0b1=changes PendSV exception state to pending., 0b0=PendSV exception is not pending, 0b1=PendSV exception is pending.,"]
pub type PENDSVSET_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICSR_SPEC, bool, O>;
#[doc = "Field `NMIPENDSET` reader - NMI set-pending bit: 0b0=no effect, 0b1=changes NMI exception state to pending., 0b0=NMI exception is not pending, 0b1=NMI exception is pending.,"]
pub type NMIPENDSET_R = crate::BitReader<bool>;
#[doc = "Field `NMIPENDSET` writer - NMI set-pending bit: 0b0=no effect, 0b1=changes NMI exception state to pending., 0b0=NMI exception is not pending, 0b1=NMI exception is pending.,"]
pub type NMIPENDSET_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICSR_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:8 - Active exception number"]
    #[inline(always)]
    pub fn vectactive(&self) -> VECTACTIVE_R {
        VECTACTIVE_R::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bit 11 - Return to Base"]
    #[inline(always)]
    pub fn rettobase(&self) -> RETTOBASE_R {
        RETTOBASE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:17 - Vector Pending"]
    #[inline(always)]
    pub fn vectpending(&self) -> VECTPENDING_R {
        VECTPENDING_R::new(((self.bits >> 12) & 0x3f) as u8)
    }
    #[doc = "Bit 22 - Interrupt pending flag"]
    #[inline(always)]
    pub fn isrpending(&self) -> ISRPENDING_R {
        ISRPENDING_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 26 - SysTick exception set-pending bit"]
    #[inline(always)]
    pub fn pendstset(&self) -> PENDSTSET_R {
        PENDSTSET_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 28 - PendSV set-pending bit: 0b0=no effect, 0b1=changes PendSV exception state to pending., 0b0=PendSV exception is not pending, 0b1=PendSV exception is pending.,"]
    #[inline(always)]
    pub fn pendsvset(&self) -> PENDSVSET_R {
        PENDSVSET_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 31 - NMI set-pending bit: 0b0=no effect, 0b1=changes NMI exception state to pending., 0b0=NMI exception is not pending, 0b1=NMI exception is pending.,"]
    #[inline(always)]
    pub fn nmipendset(&self) -> NMIPENDSET_R {
        NMIPENDSET_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 25 - SysTick exception clear-pending bit"]
    #[inline(always)]
    #[must_use]
    pub fn pendstclr(&mut self) -> PENDSTCLR_W<25> {
        PENDSTCLR_W::new(self)
    }
    #[doc = "Bit 26 - SysTick exception set-pending bit"]
    #[inline(always)]
    #[must_use]
    pub fn pendstset(&mut self) -> PENDSTSET_W<26> {
        PENDSTSET_W::new(self)
    }
    #[doc = "Bit 27 - PendSV clear-pending bit"]
    #[inline(always)]
    #[must_use]
    pub fn pendsvclr(&mut self) -> PENDSVCLR_W<27> {
        PENDSVCLR_W::new(self)
    }
    #[doc = "Bit 28 - PendSV set-pending bit: 0b0=no effect, 0b1=changes PendSV exception state to pending., 0b0=PendSV exception is not pending, 0b1=PendSV exception is pending.,"]
    #[inline(always)]
    #[must_use]
    pub fn pendsvset(&mut self) -> PENDSVSET_W<28> {
        PENDSVSET_W::new(self)
    }
    #[doc = "Bit 31 - NMI set-pending bit: 0b0=no effect, 0b1=changes NMI exception state to pending., 0b0=NMI exception is not pending, 0b1=NMI exception is pending.,"]
    #[inline(always)]
    #[must_use]
    pub fn nmipendset(&mut self) -> NMIPENDSET_W<31> {
        NMIPENDSET_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Control and State Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [icsr](index.html) module"]
pub struct ICSR_SPEC;
impl crate::RegisterSpec for ICSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [icsr::R](R) reader structure"]
impl crate::Readable for ICSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [icsr::W](W) writer structure"]
impl crate::Writable for ICSR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ICSR to value 0"]
impl crate::Resettable for ICSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
