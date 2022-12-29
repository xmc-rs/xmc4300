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
#[doc = "Field `MARGIN` reader - PFLASH Margin Selection"]
pub type MARGIN_R = crate::FieldReader<u8, MARGIN_A>;
#[doc = "PFLASH Margin Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl MARGIN_R {
    #[doc = "Get enumerated values variant"]
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
        *self == MARGIN_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == MARGIN_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == MARGIN_A::VALUE3
    }
}
#[doc = "Field `MARGIN` writer - PFLASH Margin Selection"]
pub type MARGIN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MARP_SPEC, u8, MARGIN_A, 4, O>;
impl<'a, const O: u8> MARGIN_W<'a, O> {
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
}
#[doc = "Field `TRAPDIS` reader - PFLASH Double-Bit Error Trap Disable"]
pub type TRAPDIS_R = crate::BitReader<TRAPDIS_A>;
#[doc = "PFLASH Double-Bit Error Trap Disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl TRAPDIS_R {
    #[doc = "Get enumerated values variant"]
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
        *self == TRAPDIS_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == TRAPDIS_A::VALUE2
    }
}
#[doc = "Field `TRAPDIS` writer - PFLASH Double-Bit Error Trap Disable"]
pub type TRAPDIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, MARP_SPEC, TRAPDIS_A, O>;
impl<'a, const O: u8> TRAPDIS_W<'a, O> {
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
        TRAPDIS_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - PFLASH Margin Selection"]
    #[inline(always)]
    #[must_use]
    pub fn margin(&mut self) -> MARGIN_W<0> {
        MARGIN_W::new(self)
    }
    #[doc = "Bit 15 - PFLASH Double-Bit Error Trap Disable"]
    #[inline(always)]
    #[must_use]
    pub fn trapdis(&mut self) -> TRAPDIS_W<15> {
        TRAPDIS_W::new(self)
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
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MARP to value 0"]
impl crate::Resettable for MARP_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
