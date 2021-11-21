#[doc = "Register `INTE` reader"]
pub struct R(crate::R<INTE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTE` writer"]
pub struct W(crate::W<INTE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTE_SPEC>;
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
impl From<crate::W<INTE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Period match while counting up enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PME_A {
    #[doc = "0: Period Match interrupt is disabled"]
    VALUE1 = 0,
    #[doc = "1: Period Match interrupt is enabled"]
    VALUE2 = 1,
}
impl From<PME_A> for bool {
    #[inline(always)]
    fn from(variant: PME_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PME` reader - Period match while counting up enable"]
pub struct PME_R(crate::FieldReader<bool, PME_A>);
impl PME_R {
    pub(crate) fn new(bits: bool) -> Self {
        PME_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PME_A {
        match self.bits {
            false => PME_A::VALUE1,
            true => PME_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == PME_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == PME_A::VALUE2
    }
}
impl core::ops::Deref for PME_R {
    type Target = crate::FieldReader<bool, PME_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PME` writer - Period match while counting up enable"]
pub struct PME_W<'a> {
    w: &'a mut W,
}
impl<'a> PME_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PME_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Period Match interrupt is disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(PME_A::VALUE1)
    }
    #[doc = "Period Match interrupt is enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(PME_A::VALUE2)
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
#[doc = "One match while counting down enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OME_A {
    #[doc = "0: One Match interrupt is disabled"]
    VALUE1 = 0,
    #[doc = "1: One Match interrupt is enabled"]
    VALUE2 = 1,
}
impl From<OME_A> for bool {
    #[inline(always)]
    fn from(variant: OME_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OME` reader - One match while counting down enable"]
pub struct OME_R(crate::FieldReader<bool, OME_A>);
impl OME_R {
    pub(crate) fn new(bits: bool) -> Self {
        OME_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OME_A {
        match self.bits {
            false => OME_A::VALUE1,
            true => OME_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == OME_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == OME_A::VALUE2
    }
}
impl core::ops::Deref for OME_R {
    type Target = crate::FieldReader<bool, OME_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OME` writer - One match while counting down enable"]
pub struct OME_W<'a> {
    w: &'a mut W,
}
impl<'a> OME_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OME_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "One Match interrupt is disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(OME_A::VALUE1)
    }
    #[doc = "One Match interrupt is enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(OME_A::VALUE2)
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
#[doc = "Compare match while counting up enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CMUE_A {
    #[doc = "0: Compare Match while counting up interrupt is disabled"]
    VALUE1 = 0,
    #[doc = "1: Compare Match while counting up interrupt is enabled"]
    VALUE2 = 1,
}
impl From<CMUE_A> for bool {
    #[inline(always)]
    fn from(variant: CMUE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMUE` reader - Compare match while counting up enable"]
pub struct CMUE_R(crate::FieldReader<bool, CMUE_A>);
impl CMUE_R {
    pub(crate) fn new(bits: bool) -> Self {
        CMUE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMUE_A {
        match self.bits {
            false => CMUE_A::VALUE1,
            true => CMUE_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == CMUE_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == CMUE_A::VALUE2
    }
}
impl core::ops::Deref for CMUE_R {
    type Target = crate::FieldReader<bool, CMUE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMUE` writer - Compare match while counting up enable"]
pub struct CMUE_W<'a> {
    w: &'a mut W,
}
impl<'a> CMUE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CMUE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Compare Match while counting up interrupt is disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CMUE_A::VALUE1)
    }
    #[doc = "Compare Match while counting up interrupt is enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CMUE_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Compare match while counting down enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CMDE_A {
    #[doc = "0: Compare Match while counting down interrupt is disabled"]
    VALUE1 = 0,
    #[doc = "1: Compare Match while counting down interrupt is enabled"]
    VALUE2 = 1,
}
impl From<CMDE_A> for bool {
    #[inline(always)]
    fn from(variant: CMDE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMDE` reader - Compare match while counting down enable"]
pub struct CMDE_R(crate::FieldReader<bool, CMDE_A>);
impl CMDE_R {
    pub(crate) fn new(bits: bool) -> Self {
        CMDE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMDE_A {
        match self.bits {
            false => CMDE_A::VALUE1,
            true => CMDE_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == CMDE_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == CMDE_A::VALUE2
    }
}
impl core::ops::Deref for CMDE_R {
    type Target = crate::FieldReader<bool, CMDE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMDE` writer - Compare match while counting down enable"]
pub struct CMDE_W<'a> {
    w: &'a mut W,
}
impl<'a> CMDE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CMDE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Compare Match while counting down interrupt is disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CMDE_A::VALUE1)
    }
    #[doc = "Compare Match while counting down interrupt is enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CMDE_A::VALUE2)
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
#[doc = "Event 0 interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum E0AE_A {
    #[doc = "0: Event 0 detection interrupt is disabled"]
    VALUE1 = 0,
    #[doc = "1: Event 0 detection interrupt is enabled"]
    VALUE2 = 1,
}
impl From<E0AE_A> for bool {
    #[inline(always)]
    fn from(variant: E0AE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `E0AE` reader - Event 0 interrupt enable"]
pub struct E0AE_R(crate::FieldReader<bool, E0AE_A>);
impl E0AE_R {
    pub(crate) fn new(bits: bool) -> Self {
        E0AE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> E0AE_A {
        match self.bits {
            false => E0AE_A::VALUE1,
            true => E0AE_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == E0AE_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == E0AE_A::VALUE2
    }
}
impl core::ops::Deref for E0AE_R {
    type Target = crate::FieldReader<bool, E0AE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `E0AE` writer - Event 0 interrupt enable"]
pub struct E0AE_W<'a> {
    w: &'a mut W,
}
impl<'a> E0AE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: E0AE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Event 0 detection interrupt is disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(E0AE_A::VALUE1)
    }
    #[doc = "Event 0 detection interrupt is enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(E0AE_A::VALUE2)
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
#[doc = "Event 1 interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum E1AE_A {
    #[doc = "0: Event 1 detection interrupt is disabled"]
    VALUE1 = 0,
    #[doc = "1: Event 1 detection interrupt is enabled"]
    VALUE2 = 1,
}
impl From<E1AE_A> for bool {
    #[inline(always)]
    fn from(variant: E1AE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `E1AE` reader - Event 1 interrupt enable"]
pub struct E1AE_R(crate::FieldReader<bool, E1AE_A>);
impl E1AE_R {
    pub(crate) fn new(bits: bool) -> Self {
        E1AE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> E1AE_A {
        match self.bits {
            false => E1AE_A::VALUE1,
            true => E1AE_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == E1AE_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == E1AE_A::VALUE2
    }
}
impl core::ops::Deref for E1AE_R {
    type Target = crate::FieldReader<bool, E1AE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `E1AE` writer - Event 1 interrupt enable"]
pub struct E1AE_W<'a> {
    w: &'a mut W,
}
impl<'a> E1AE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: E1AE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Event 1 detection interrupt is disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(E1AE_A::VALUE1)
    }
    #[doc = "Event 1 detection interrupt is enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(E1AE_A::VALUE2)
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
#[doc = "Event 2 interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum E2AE_A {
    #[doc = "0: Event 2 detection interrupt is disabled"]
    VALUE1 = 0,
    #[doc = "1: Event 2 detection interrupt is enabled"]
    VALUE2 = 1,
}
impl From<E2AE_A> for bool {
    #[inline(always)]
    fn from(variant: E2AE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `E2AE` reader - Event 2 interrupt enable"]
pub struct E2AE_R(crate::FieldReader<bool, E2AE_A>);
impl E2AE_R {
    pub(crate) fn new(bits: bool) -> Self {
        E2AE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> E2AE_A {
        match self.bits {
            false => E2AE_A::VALUE1,
            true => E2AE_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == E2AE_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == E2AE_A::VALUE2
    }
}
impl core::ops::Deref for E2AE_R {
    type Target = crate::FieldReader<bool, E2AE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `E2AE` writer - Event 2 interrupt enable"]
pub struct E2AE_W<'a> {
    w: &'a mut W,
}
impl<'a> E2AE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: E2AE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Event 2 detection interrupt is disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(E2AE_A::VALUE1)
    }
    #[doc = "Event 2 detection interrupt is enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(E2AE_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Period match while counting up enable"]
    #[inline(always)]
    pub fn pme(&self) -> PME_R {
        PME_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - One match while counting down enable"]
    #[inline(always)]
    pub fn ome(&self) -> OME_R {
        OME_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Compare match while counting up enable"]
    #[inline(always)]
    pub fn cmue(&self) -> CMUE_R {
        CMUE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Compare match while counting down enable"]
    #[inline(always)]
    pub fn cmde(&self) -> CMDE_R {
        CMDE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Event 0 interrupt enable"]
    #[inline(always)]
    pub fn e0ae(&self) -> E0AE_R {
        E0AE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Event 1 interrupt enable"]
    #[inline(always)]
    pub fn e1ae(&self) -> E1AE_R {
        E1AE_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Event 2 interrupt enable"]
    #[inline(always)]
    pub fn e2ae(&self) -> E2AE_R {
        E2AE_R::new(((self.bits >> 10) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Period match while counting up enable"]
    #[inline(always)]
    pub fn pme(&mut self) -> PME_W {
        PME_W { w: self }
    }
    #[doc = "Bit 1 - One match while counting down enable"]
    #[inline(always)]
    pub fn ome(&mut self) -> OME_W {
        OME_W { w: self }
    }
    #[doc = "Bit 2 - Compare match while counting up enable"]
    #[inline(always)]
    pub fn cmue(&mut self) -> CMUE_W {
        CMUE_W { w: self }
    }
    #[doc = "Bit 3 - Compare match while counting down enable"]
    #[inline(always)]
    pub fn cmde(&mut self) -> CMDE_W {
        CMDE_W { w: self }
    }
    #[doc = "Bit 8 - Event 0 interrupt enable"]
    #[inline(always)]
    pub fn e0ae(&mut self) -> E0AE_W {
        E0AE_W { w: self }
    }
    #[doc = "Bit 9 - Event 1 interrupt enable"]
    #[inline(always)]
    pub fn e1ae(&mut self) -> E1AE_W {
        E1AE_W { w: self }
    }
    #[doc = "Bit 10 - Event 2 interrupt enable"]
    #[inline(always)]
    pub fn e2ae(&mut self) -> E2AE_W {
        E2AE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Enable Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [inte](index.html) module"]
pub struct INTE_SPEC;
impl crate::RegisterSpec for INTE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [inte::R](R) reader structure"]
impl crate::Readable for INTE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [inte::W](W) writer structure"]
impl crate::Writable for INTE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INTE to value 0"]
impl crate::Resettable for INTE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
