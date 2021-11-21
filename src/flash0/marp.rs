#[doc = "Register `MARP` reader"]
pub struct R(crate::R<MARP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MARP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MARP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MARP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MARP` writer"]
pub struct W(crate::W<MARP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MARP_SPEC>;
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
impl From<crate::W<MARP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MARP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "PFLASH Margin Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MARGIN_A {
    #[doc = "0: Standard (default) margin."]
    VALUE1 = 0,
    #[doc = "1: Tight margin for 0 (low) level. Suboptimal 0-bits are read as 1s."]
    VALUE2 = 1,
    #[doc = "4: Tight margin for 1 (high) level. Suboptimal 1-bits are read as 0s."]
    VALUE3 = 4,
}
impl From<MARGIN_A> for u8 {
    #[inline(always)]
    fn from(variant: MARGIN_A) -> Self {
        variant as _
    }
}
#[doc = "Field `MARGIN` reader - PFLASH Margin Selection"]
pub struct MARGIN_R(crate::FieldReader<u8, MARGIN_A>);
impl MARGIN_R {
    pub(crate) fn new(bits: u8) -> Self {
        MARGIN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<MARGIN_A> {
        match self.bits {
            0 => Some(MARGIN_A::VALUE1),
            1 => Some(MARGIN_A::VALUE2),
            4 => Some(MARGIN_A::VALUE3),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == MARGIN_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == MARGIN_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        **self == MARGIN_A::VALUE3
    }
}
impl core::ops::Deref for MARGIN_R {
    type Target = crate::FieldReader<u8, MARGIN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MARGIN` writer - PFLASH Margin Selection"]
pub struct MARGIN_W<'a> {
    w: &'a mut W,
}
impl<'a> MARGIN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MARGIN_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Standard (default) margin."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(MARGIN_A::VALUE1)
    }
    #[doc = "Tight margin for 0 (low) level. Suboptimal 0-bits are read as 1s."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(MARGIN_A::VALUE2)
    }
    #[doc = "Tight margin for 1 (high) level. Suboptimal 1-bits are read as 0s."]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(MARGIN_A::VALUE3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
#[doc = "PFLASH Double-Bit Error Trap Disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRAPDIS_A {
    #[doc = "0: If a double-bit error occurs in PFLASH, a bus error trap is generated."]
    VALUE1 = 0,
    #[doc = "1: The double-bit error trap is disabled. Shall be used only during margin check"]
    VALUE2 = 1,
}
impl From<TRAPDIS_A> for bool {
    #[inline(always)]
    fn from(variant: TRAPDIS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TRAPDIS` reader - PFLASH Double-Bit Error Trap Disable"]
pub struct TRAPDIS_R(crate::FieldReader<bool, TRAPDIS_A>);
impl TRAPDIS_R {
    pub(crate) fn new(bits: bool) -> Self {
        TRAPDIS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRAPDIS_A {
        match self.bits {
            false => TRAPDIS_A::VALUE1,
            true => TRAPDIS_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == TRAPDIS_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == TRAPDIS_A::VALUE2
    }
}
impl core::ops::Deref for TRAPDIS_R {
    type Target = crate::FieldReader<bool, TRAPDIS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TRAPDIS` writer - PFLASH Double-Bit Error Trap Disable"]
pub struct TRAPDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> TRAPDIS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TRAPDIS_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "If a double-bit error occurs in PFLASH, a bus error trap is generated."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(TRAPDIS_A::VALUE1)
    }
    #[doc = "The double-bit error trap is disabled. Shall be used only during margin check"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(TRAPDIS_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | ((value as u32 & 0x01) << 15);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - PFLASH Margin Selection"]
    #[inline(always)]
    pub fn margin(&self) -> MARGIN_R {
        MARGIN_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 15 - PFLASH Double-Bit Error Trap Disable"]
    #[inline(always)]
    pub fn trapdis(&self) -> TRAPDIS_R {
        TRAPDIS_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - PFLASH Margin Selection"]
    #[inline(always)]
    pub fn margin(&mut self) -> MARGIN_W {
        MARGIN_W { w: self }
    }
    #[doc = "Bit 15 - PFLASH Double-Bit Error Trap Disable"]
    #[inline(always)]
    pub fn trapdis(&mut self) -> TRAPDIS_W {
        TRAPDIS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Margin Control Register PFLASH\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [marp](index.html) module"]
pub struct MARP_SPEC;
impl crate::RegisterSpec for MARP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [marp::R](R) reader structure"]
impl crate::Readable for MARP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [marp::W](W) writer structure"]
impl crate::Writable for MARP_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MARP to value 0"]
impl crate::Resettable for MARP_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
