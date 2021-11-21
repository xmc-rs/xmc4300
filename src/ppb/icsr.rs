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
#[doc = "Active exception number\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Field `VECTACTIVE` reader - Active exception number"]
pub struct VECTACTIVE_R(crate::FieldReader<u16, VECTACTIVE_A>);
impl VECTACTIVE_R {
    pub(crate) fn new(bits: u16) -> Self {
        VECTACTIVE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
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
        **self == VECTACTIVE_A::VALUE1
    }
}
impl core::ops::Deref for VECTACTIVE_R {
    type Target = crate::FieldReader<u16, VECTACTIVE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Return to Base\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Field `RETTOBASE` reader - Return to Base"]
pub struct RETTOBASE_R(crate::FieldReader<bool, RETTOBASE_A>);
impl RETTOBASE_R {
    pub(crate) fn new(bits: bool) -> Self {
        RETTOBASE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
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
        **self == RETTOBASE_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == RETTOBASE_A::VALUE2
    }
}
impl core::ops::Deref for RETTOBASE_R {
    type Target = crate::FieldReader<bool, RETTOBASE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Vector Pending\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Field `VECTPENDING` reader - Vector Pending"]
pub struct VECTPENDING_R(crate::FieldReader<u8, VECTPENDING_A>);
impl VECTPENDING_R {
    pub(crate) fn new(bits: u8) -> Self {
        VECTPENDING_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
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
        **self == VECTPENDING_A::VALUE1
    }
}
impl core::ops::Deref for VECTPENDING_R {
    type Target = crate::FieldReader<u8, VECTPENDING_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Interrupt pending flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Field `ISRPENDING` reader - Interrupt pending flag"]
pub struct ISRPENDING_R(crate::FieldReader<bool, ISRPENDING_A>);
impl ISRPENDING_R {
    pub(crate) fn new(bits: bool) -> Self {
        ISRPENDING_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
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
        **self == ISRPENDING_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == ISRPENDING_A::VALUE2
    }
}
impl core::ops::Deref for ISRPENDING_R {
    type Target = crate::FieldReader<bool, ISRPENDING_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "SysTick exception clear-pending bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
pub struct PENDSTCLR_W<'a> {
    w: &'a mut W,
}
impl<'a> PENDSTCLR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PENDSTCLR_AW) -> &'a mut W {
        self.bit(variant.into())
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | ((value as u32 & 0x01) << 25);
        self.w
    }
}
#[doc = "SysTick exception set-pending bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Field `PENDSTSET` reader - SysTick exception set-pending bit"]
pub struct PENDSTSET_R(crate::FieldReader<bool, PENDSTSET_A>);
impl PENDSTSET_R {
    pub(crate) fn new(bits: bool) -> Self {
        PENDSTSET_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
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
        **self == PENDSTSET_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == PENDSTSET_A::VALUE2
    }
}
impl core::ops::Deref for PENDSTSET_R {
    type Target = crate::FieldReader<bool, PENDSTSET_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PENDSTSET` writer - SysTick exception set-pending bit"]
pub struct PENDSTSET_W<'a> {
    w: &'a mut W,
}
impl<'a> PENDSTSET_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PENDSTSET_A) -> &'a mut W {
        self.bit(variant.into())
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | ((value as u32 & 0x01) << 26);
        self.w
    }
}
#[doc = "PendSV clear-pending bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
pub struct PENDSVCLR_W<'a> {
    w: &'a mut W,
}
impl<'a> PENDSVCLR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PENDSVCLR_AW) -> &'a mut W {
        self.bit(variant.into())
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | ((value as u32 & 0x01) << 27);
        self.w
    }
}
#[doc = "Field `PENDSVSET` reader - PendSV set-pending bit: 0b0=no effect, 0b1=changes PendSV exception state to pending., 0b0=PendSV exception is not pending, 0b1=PendSV exception is pending.,"]
pub struct PENDSVSET_R(crate::FieldReader<bool, bool>);
impl PENDSVSET_R {
    pub(crate) fn new(bits: bool) -> Self {
        PENDSVSET_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PENDSVSET_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PENDSVSET` writer - PendSV set-pending bit: 0b0=no effect, 0b1=changes PendSV exception state to pending., 0b0=PendSV exception is not pending, 0b1=PendSV exception is pending.,"]
pub struct PENDSVSET_W<'a> {
    w: &'a mut W,
}
impl<'a> PENDSVSET_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | ((value as u32 & 0x01) << 28);
        self.w
    }
}
#[doc = "Field `NMIPENDSET` reader - NMI set-pending bit: 0b0=no effect, 0b1=changes NMI exception state to pending., 0b0=NMI exception is not pending, 0b1=NMI exception is pending.,"]
pub struct NMIPENDSET_R(crate::FieldReader<bool, bool>);
impl NMIPENDSET_R {
    pub(crate) fn new(bits: bool) -> Self {
        NMIPENDSET_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NMIPENDSET_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NMIPENDSET` writer - NMI set-pending bit: 0b0=no effect, 0b1=changes NMI exception state to pending., 0b0=NMI exception is not pending, 0b1=NMI exception is pending.,"]
pub struct NMIPENDSET_W<'a> {
    w: &'a mut W,
}
impl<'a> NMIPENDSET_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | ((value as u32 & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:8 - Active exception number"]
    #[inline(always)]
    pub fn vectactive(&self) -> VECTACTIVE_R {
        VECTACTIVE_R::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bit 11 - Return to Base"]
    #[inline(always)]
    pub fn rettobase(&self) -> RETTOBASE_R {
        RETTOBASE_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bits 12:17 - Vector Pending"]
    #[inline(always)]
    pub fn vectpending(&self) -> VECTPENDING_R {
        VECTPENDING_R::new(((self.bits >> 12) & 0x3f) as u8)
    }
    #[doc = "Bit 22 - Interrupt pending flag"]
    #[inline(always)]
    pub fn isrpending(&self) -> ISRPENDING_R {
        ISRPENDING_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 26 - SysTick exception set-pending bit"]
    #[inline(always)]
    pub fn pendstset(&self) -> PENDSTSET_R {
        PENDSTSET_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 28 - PendSV set-pending bit: 0b0=no effect, 0b1=changes PendSV exception state to pending., 0b0=PendSV exception is not pending, 0b1=PendSV exception is pending.,"]
    #[inline(always)]
    pub fn pendsvset(&self) -> PENDSVSET_R {
        PENDSVSET_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 31 - NMI set-pending bit: 0b0=no effect, 0b1=changes NMI exception state to pending., 0b0=NMI exception is not pending, 0b1=NMI exception is pending.,"]
    #[inline(always)]
    pub fn nmipendset(&self) -> NMIPENDSET_R {
        NMIPENDSET_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 25 - SysTick exception clear-pending bit"]
    #[inline(always)]
    pub fn pendstclr(&mut self) -> PENDSTCLR_W {
        PENDSTCLR_W { w: self }
    }
    #[doc = "Bit 26 - SysTick exception set-pending bit"]
    #[inline(always)]
    pub fn pendstset(&mut self) -> PENDSTSET_W {
        PENDSTSET_W { w: self }
    }
    #[doc = "Bit 27 - PendSV clear-pending bit"]
    #[inline(always)]
    pub fn pendsvclr(&mut self) -> PENDSVCLR_W {
        PENDSVCLR_W { w: self }
    }
    #[doc = "Bit 28 - PendSV set-pending bit: 0b0=no effect, 0b1=changes PendSV exception state to pending., 0b0=PendSV exception is not pending, 0b1=PendSV exception is pending.,"]
    #[inline(always)]
    pub fn pendsvset(&mut self) -> PENDSVSET_W {
        PENDSVSET_W { w: self }
    }
    #[doc = "Bit 31 - NMI set-pending bit: 0b0=no effect, 0b1=changes NMI exception state to pending., 0b0=NMI exception is not pending, 0b1=NMI exception is pending.,"]
    #[inline(always)]
    pub fn nmipendset(&mut self) -> NMIPENDSET_W {
        NMIPENDSET_W { w: self }
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
}
#[doc = "`reset()` method sets ICSR to value 0"]
impl crate::Resettable for ICSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
