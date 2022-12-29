#[doc = "Register `NIPR` reader"]
pub struct R(crate::R<NIPR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<NIPR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<NIPR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<NIPR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `NIPR` writer"]
pub struct W(crate::W<NIPR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<NIPR_SPEC>;
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
impl From<crate::W<NIPR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<NIPR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ALINP` reader - Alert Interrupt Node Pointer"]
pub type ALINP_R = crate::FieldReader<u8, ALINP_A>;
#[doc = "Alert Interrupt Node Pointer\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ALINP_A {
    #[doc = "0: Interrupt output line INT_O0 is selected."]
    VALUE1 = 0,
    #[doc = "1: Interrupt output line INT_O1 is selected."]
    VALUE2 = 1,
    #[doc = "14: Interrupt output line INT_O14 is selected."]
    VALUE3 = 14,
    #[doc = "15: Interrupt output line INT_O15 is selected."]
    VALUE4 = 15,
}
impl From<ALINP_A> for u8 {
    #[inline(always)]
    fn from(variant: ALINP_A) -> Self {
        variant as _
    }
}
impl ALINP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<ALINP_A> {
        match self.bits {
            0 => Some(ALINP_A::VALUE1),
            1 => Some(ALINP_A::VALUE2),
            14 => Some(ALINP_A::VALUE3),
            15 => Some(ALINP_A::VALUE4),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == ALINP_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == ALINP_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == ALINP_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == ALINP_A::VALUE4
    }
}
#[doc = "Field `ALINP` writer - Alert Interrupt Node Pointer"]
pub type ALINP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, NIPR_SPEC, u8, ALINP_A, 4, O>;
impl<'a, const O: u8> ALINP_W<'a, O> {
    #[doc = "Interrupt output line INT_O0 is selected."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(ALINP_A::VALUE1)
    }
    #[doc = "Interrupt output line INT_O1 is selected."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(ALINP_A::VALUE2)
    }
    #[doc = "Interrupt output line INT_O14 is selected."]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(ALINP_A::VALUE3)
    }
    #[doc = "Interrupt output line INT_O15 is selected."]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(ALINP_A::VALUE4)
    }
}
#[doc = "Field `LECINP` reader - Last Error Code Interrupt Node Pointer"]
pub type LECINP_R = crate::FieldReader<u8, LECINP_A>;
#[doc = "Last Error Code Interrupt Node Pointer\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LECINP_A {
    #[doc = "0: Interrupt output line INT_O0 is selected."]
    VALUE1 = 0,
    #[doc = "1: Interrupt output line INT_O1 is selected."]
    VALUE2 = 1,
    #[doc = "14: Interrupt output line INT_O14 is selected."]
    VALUE3 = 14,
    #[doc = "15: Interrupt output line INT_O15 is selected."]
    VALUE4 = 15,
}
impl From<LECINP_A> for u8 {
    #[inline(always)]
    fn from(variant: LECINP_A) -> Self {
        variant as _
    }
}
impl LECINP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<LECINP_A> {
        match self.bits {
            0 => Some(LECINP_A::VALUE1),
            1 => Some(LECINP_A::VALUE2),
            14 => Some(LECINP_A::VALUE3),
            15 => Some(LECINP_A::VALUE4),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == LECINP_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == LECINP_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == LECINP_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == LECINP_A::VALUE4
    }
}
#[doc = "Field `LECINP` writer - Last Error Code Interrupt Node Pointer"]
pub type LECINP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, NIPR_SPEC, u8, LECINP_A, 4, O>;
impl<'a, const O: u8> LECINP_W<'a, O> {
    #[doc = "Interrupt output line INT_O0 is selected."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(LECINP_A::VALUE1)
    }
    #[doc = "Interrupt output line INT_O1 is selected."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(LECINP_A::VALUE2)
    }
    #[doc = "Interrupt output line INT_O14 is selected."]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(LECINP_A::VALUE3)
    }
    #[doc = "Interrupt output line INT_O15 is selected."]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(LECINP_A::VALUE4)
    }
}
#[doc = "Field `TRINP` reader - Transfer OK Interrupt Node Pointer"]
pub type TRINP_R = crate::FieldReader<u8, TRINP_A>;
#[doc = "Transfer OK Interrupt Node Pointer\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TRINP_A {
    #[doc = "0: Interrupt output line INT_O0 is selected."]
    VALUE1 = 0,
    #[doc = "1: Interrupt output line INT_O1 is selected."]
    VALUE2 = 1,
    #[doc = "14: Interrupt output line INT_O14 is selected."]
    VALUE3 = 14,
    #[doc = "15: Interrupt output line INT_O15 is selected."]
    VALUE4 = 15,
}
impl From<TRINP_A> for u8 {
    #[inline(always)]
    fn from(variant: TRINP_A) -> Self {
        variant as _
    }
}
impl TRINP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TRINP_A> {
        match self.bits {
            0 => Some(TRINP_A::VALUE1),
            1 => Some(TRINP_A::VALUE2),
            14 => Some(TRINP_A::VALUE3),
            15 => Some(TRINP_A::VALUE4),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == TRINP_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == TRINP_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == TRINP_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == TRINP_A::VALUE4
    }
}
#[doc = "Field `TRINP` writer - Transfer OK Interrupt Node Pointer"]
pub type TRINP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, NIPR_SPEC, u8, TRINP_A, 4, O>;
impl<'a, const O: u8> TRINP_W<'a, O> {
    #[doc = "Interrupt output line INT_O0 is selected."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(TRINP_A::VALUE1)
    }
    #[doc = "Interrupt output line INT_O1 is selected."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(TRINP_A::VALUE2)
    }
    #[doc = "Interrupt output line INT_O14 is selected."]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(TRINP_A::VALUE3)
    }
    #[doc = "Interrupt output line INT_O15 is selected."]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(TRINP_A::VALUE4)
    }
}
#[doc = "Field `CFCINP` reader - Frame Counter Interrupt Node Pointer"]
pub type CFCINP_R = crate::FieldReader<u8, CFCINP_A>;
#[doc = "Frame Counter Interrupt Node Pointer\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CFCINP_A {
    #[doc = "0: Interrupt output line INT_O0 is selected."]
    VALUE1 = 0,
    #[doc = "1: Interrupt output line INT_O1 is selected."]
    VALUE2 = 1,
    #[doc = "14: Interrupt output line INT_O14 is selected."]
    VALUE3 = 14,
    #[doc = "15: Interrupt output line INT_O15 is selected."]
    VALUE4 = 15,
}
impl From<CFCINP_A> for u8 {
    #[inline(always)]
    fn from(variant: CFCINP_A) -> Self {
        variant as _
    }
}
impl CFCINP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CFCINP_A> {
        match self.bits {
            0 => Some(CFCINP_A::VALUE1),
            1 => Some(CFCINP_A::VALUE2),
            14 => Some(CFCINP_A::VALUE3),
            15 => Some(CFCINP_A::VALUE4),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CFCINP_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CFCINP_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == CFCINP_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == CFCINP_A::VALUE4
    }
}
#[doc = "Field `CFCINP` writer - Frame Counter Interrupt Node Pointer"]
pub type CFCINP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, NIPR_SPEC, u8, CFCINP_A, 4, O>;
impl<'a, const O: u8> CFCINP_W<'a, O> {
    #[doc = "Interrupt output line INT_O0 is selected."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CFCINP_A::VALUE1)
    }
    #[doc = "Interrupt output line INT_O1 is selected."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CFCINP_A::VALUE2)
    }
    #[doc = "Interrupt output line INT_O14 is selected."]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(CFCINP_A::VALUE3)
    }
    #[doc = "Interrupt output line INT_O15 is selected."]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(CFCINP_A::VALUE4)
    }
}
impl R {
    #[doc = "Bits 0:3 - Alert Interrupt Node Pointer"]
    #[inline(always)]
    pub fn alinp(&self) -> ALINP_R {
        ALINP_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Last Error Code Interrupt Node Pointer"]
    #[inline(always)]
    pub fn lecinp(&self) -> LECINP_R {
        LECINP_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Transfer OK Interrupt Node Pointer"]
    #[inline(always)]
    pub fn trinp(&self) -> TRINP_R {
        TRINP_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Frame Counter Interrupt Node Pointer"]
    #[inline(always)]
    pub fn cfcinp(&self) -> CFCINP_R {
        CFCINP_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Alert Interrupt Node Pointer"]
    #[inline(always)]
    #[must_use]
    pub fn alinp(&mut self) -> ALINP_W<0> {
        ALINP_W::new(self)
    }
    #[doc = "Bits 4:7 - Last Error Code Interrupt Node Pointer"]
    #[inline(always)]
    #[must_use]
    pub fn lecinp(&mut self) -> LECINP_W<4> {
        LECINP_W::new(self)
    }
    #[doc = "Bits 8:11 - Transfer OK Interrupt Node Pointer"]
    #[inline(always)]
    #[must_use]
    pub fn trinp(&mut self) -> TRINP_W<8> {
        TRINP_W::new(self)
    }
    #[doc = "Bits 12:15 - Frame Counter Interrupt Node Pointer"]
    #[inline(always)]
    #[must_use]
    pub fn cfcinp(&mut self) -> CFCINP_W<12> {
        CFCINP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Node Interrupt Pointer Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nipr](index.html) module"]
pub struct NIPR_SPEC;
impl crate::RegisterSpec for NIPR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [nipr::R](R) reader structure"]
impl crate::Readable for NIPR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [nipr::W](W) writer structure"]
impl crate::Writable for NIPR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets NIPR to value 0"]
impl crate::Resettable for NIPR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
