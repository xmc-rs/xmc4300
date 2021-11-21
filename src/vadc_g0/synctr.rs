#[doc = "Register `SYNCTR` reader"]
pub struct R(crate::R<SYNCTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYNCTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYNCTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYNCTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SYNCTR` writer"]
pub struct W(crate::W<SYNCTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SYNCTR_SPEC>;
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
impl From<crate::W<SYNCTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SYNCTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Start Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum STSEL_A {
    #[doc = "0: Kernel is synchronization master: Use own bitfield GxARBCFG.ANONC"]
    VALUE1 = 0,
    #[doc = "1: Kernel is synchronization slave: Control information from input CI1"]
    VALUE2 = 1,
    #[doc = "2: Kernel is synchronization slave: Control information from input CI2"]
    VALUE3 = 2,
    #[doc = "3: Kernel is synchronization slave: Control information from input CI3"]
    VALUE4 = 3,
}
impl From<STSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: STSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `STSEL` reader - Start Selection"]
pub struct STSEL_R(crate::FieldReader<u8, STSEL_A>);
impl STSEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        STSEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STSEL_A {
        match self.bits {
            0 => STSEL_A::VALUE1,
            1 => STSEL_A::VALUE2,
            2 => STSEL_A::VALUE3,
            3 => STSEL_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == STSEL_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == STSEL_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        **self == STSEL_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        **self == STSEL_A::VALUE4
    }
}
impl core::ops::Deref for STSEL_R {
    type Target = crate::FieldReader<u8, STSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STSEL` writer - Start Selection"]
pub struct STSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> STSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: STSEL_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Kernel is synchronization master: Use own bitfield GxARBCFG.ANONC"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(STSEL_A::VALUE1)
    }
    #[doc = "Kernel is synchronization slave: Control information from input CI1"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(STSEL_A::VALUE2)
    }
    #[doc = "Kernel is synchronization slave: Control information from input CI2"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(STSEL_A::VALUE3)
    }
    #[doc = "Kernel is synchronization slave: Control information from input CI3"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(STSEL_A::VALUE4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
#[doc = "Evaluate Ready Input Rx\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EVALR1_A {
    #[doc = "0: No ready input control"]
    VALUE1 = 0,
    #[doc = "1: Ready input Rx is considered for the start of a parallel conversion of this conversion group"]
    VALUE2 = 1,
}
impl From<EVALR1_A> for bool {
    #[inline(always)]
    fn from(variant: EVALR1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EVALR1` reader - Evaluate Ready Input Rx"]
pub struct EVALR1_R(crate::FieldReader<bool, EVALR1_A>);
impl EVALR1_R {
    pub(crate) fn new(bits: bool) -> Self {
        EVALR1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EVALR1_A {
        match self.bits {
            false => EVALR1_A::VALUE1,
            true => EVALR1_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == EVALR1_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == EVALR1_A::VALUE2
    }
}
impl core::ops::Deref for EVALR1_R {
    type Target = crate::FieldReader<bool, EVALR1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EVALR1` writer - Evaluate Ready Input Rx"]
pub struct EVALR1_W<'a> {
    w: &'a mut W,
}
impl<'a> EVALR1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EVALR1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No ready input control"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(EVALR1_A::VALUE1)
    }
    #[doc = "Ready input Rx is considered for the start of a parallel conversion of this conversion group"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(EVALR1_A::VALUE2)
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
#[doc = "Evaluate Ready Input Rx\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EVALR2_A {
    #[doc = "0: No ready input control"]
    VALUE1 = 0,
    #[doc = "1: Ready input Rx is considered for the start of a parallel conversion of this conversion group"]
    VALUE2 = 1,
}
impl From<EVALR2_A> for bool {
    #[inline(always)]
    fn from(variant: EVALR2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EVALR2` reader - Evaluate Ready Input Rx"]
pub struct EVALR2_R(crate::FieldReader<bool, EVALR2_A>);
impl EVALR2_R {
    pub(crate) fn new(bits: bool) -> Self {
        EVALR2_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EVALR2_A {
        match self.bits {
            false => EVALR2_A::VALUE1,
            true => EVALR2_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == EVALR2_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == EVALR2_A::VALUE2
    }
}
impl core::ops::Deref for EVALR2_R {
    type Target = crate::FieldReader<bool, EVALR2_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EVALR2` writer - Evaluate Ready Input Rx"]
pub struct EVALR2_W<'a> {
    w: &'a mut W,
}
impl<'a> EVALR2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EVALR2_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No ready input control"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(EVALR2_A::VALUE1)
    }
    #[doc = "Ready input Rx is considered for the start of a parallel conversion of this conversion group"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(EVALR2_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "Evaluate Ready Input Rx\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EVALR3_A {
    #[doc = "0: No ready input control"]
    VALUE1 = 0,
    #[doc = "1: Ready input Rx is considered for the start of a parallel conversion of this conversion group"]
    VALUE2 = 1,
}
impl From<EVALR3_A> for bool {
    #[inline(always)]
    fn from(variant: EVALR3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EVALR3` reader - Evaluate Ready Input Rx"]
pub struct EVALR3_R(crate::FieldReader<bool, EVALR3_A>);
impl EVALR3_R {
    pub(crate) fn new(bits: bool) -> Self {
        EVALR3_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EVALR3_A {
        match self.bits {
            false => EVALR3_A::VALUE1,
            true => EVALR3_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == EVALR3_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == EVALR3_A::VALUE2
    }
}
impl core::ops::Deref for EVALR3_R {
    type Target = crate::FieldReader<bool, EVALR3_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EVALR3` writer - Evaluate Ready Input Rx"]
pub struct EVALR3_W<'a> {
    w: &'a mut W,
}
impl<'a> EVALR3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EVALR3_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No ready input control"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(EVALR3_A::VALUE1)
    }
    #[doc = "Ready input Rx is considered for the start of a parallel conversion of this conversion group"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(EVALR3_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Start Selection"]
    #[inline(always)]
    pub fn stsel(&self) -> STSEL_R {
        STSEL_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bit 4 - Evaluate Ready Input Rx"]
    #[inline(always)]
    pub fn evalr1(&self) -> EVALR1_R {
        EVALR1_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Evaluate Ready Input Rx"]
    #[inline(always)]
    pub fn evalr2(&self) -> EVALR2_R {
        EVALR2_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Evaluate Ready Input Rx"]
    #[inline(always)]
    pub fn evalr3(&self) -> EVALR3_R {
        EVALR3_R::new(((self.bits >> 6) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Start Selection"]
    #[inline(always)]
    pub fn stsel(&mut self) -> STSEL_W {
        STSEL_W { w: self }
    }
    #[doc = "Bit 4 - Evaluate Ready Input Rx"]
    #[inline(always)]
    pub fn evalr1(&mut self) -> EVALR1_W {
        EVALR1_W { w: self }
    }
    #[doc = "Bit 5 - Evaluate Ready Input Rx"]
    #[inline(always)]
    pub fn evalr2(&mut self) -> EVALR2_W {
        EVALR2_W { w: self }
    }
    #[doc = "Bit 6 - Evaluate Ready Input Rx"]
    #[inline(always)]
    pub fn evalr3(&mut self) -> EVALR3_W {
        EVALR3_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Synchronization Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [synctr](index.html) module"]
pub struct SYNCTR_SPEC;
impl crate::RegisterSpec for SYNCTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [synctr::R](R) reader structure"]
impl crate::Readable for SYNCTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [synctr::W](W) writer structure"]
impl crate::Writable for SYNCTR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SYNCTR to value 0"]
impl crate::Resettable for SYNCTR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
