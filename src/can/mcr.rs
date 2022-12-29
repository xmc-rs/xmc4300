#[doc = "Register `MCR` reader"]
pub struct R(crate::R<MCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MCR` writer"]
pub struct W(crate::W<MCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MCR_SPEC>;
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
impl From<crate::W<MCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CLKSEL` reader - Baud Rate Logic Clock Select"]
pub type CLKSEL_R = crate::FieldReader<u8, CLKSEL_A>;
#[doc = "Baud Rate Logic Clock Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CLKSEL_A {
    #[doc = "0: No clock supplied"]
    VALUE1 = 0,
    #[doc = "1: fPERIPH"]
    VALUE2 = 1,
    #[doc = "2: fOHP"]
    VALUE3 = 2,
    #[doc = "4: hard wired to 0"]
    VALUE4 = 4,
    #[doc = "8: hard wired to 0"]
    VALUE5 = 8,
}
impl From<CLKSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: CLKSEL_A) -> Self {
        variant as _
    }
}
impl CLKSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CLKSEL_A> {
        match self.bits {
            0 => Some(CLKSEL_A::VALUE1),
            1 => Some(CLKSEL_A::VALUE2),
            2 => Some(CLKSEL_A::VALUE3),
            4 => Some(CLKSEL_A::VALUE4),
            8 => Some(CLKSEL_A::VALUE5),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CLKSEL_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CLKSEL_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == CLKSEL_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == CLKSEL_A::VALUE4
    }
    #[doc = "Checks if the value of the field is `VALUE5`"]
    #[inline(always)]
    pub fn is_value5(&self) -> bool {
        *self == CLKSEL_A::VALUE5
    }
}
#[doc = "Field `CLKSEL` writer - Baud Rate Logic Clock Select"]
pub type CLKSEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MCR_SPEC, u8, CLKSEL_A, 4, O>;
impl<'a, const O: u8> CLKSEL_W<'a, O> {
    #[doc = "No clock supplied"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CLKSEL_A::VALUE1)
    }
    #[doc = "fPERIPH"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CLKSEL_A::VALUE2)
    }
    #[doc = "fOHP"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(CLKSEL_A::VALUE3)
    }
    #[doc = "hard wired to 0"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(CLKSEL_A::VALUE4)
    }
    #[doc = "hard wired to 0"]
    #[inline(always)]
    pub fn value5(self) -> &'a mut W {
        self.variant(CLKSEL_A::VALUE5)
    }
}
#[doc = "Field `MPSEL` reader - Message Pending Selector"]
pub type MPSEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MPSEL` writer - Message Pending Selector"]
pub type MPSEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MCR_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:3 - Baud Rate Logic Clock Select"]
    #[inline(always)]
    pub fn clksel(&self) -> CLKSEL_R {
        CLKSEL_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Message Pending Selector"]
    #[inline(always)]
    pub fn mpsel(&self) -> MPSEL_R {
        MPSEL_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Baud Rate Logic Clock Select"]
    #[inline(always)]
    #[must_use]
    pub fn clksel(&mut self) -> CLKSEL_W<0> {
        CLKSEL_W::new(self)
    }
    #[doc = "Bits 12:15 - Message Pending Selector"]
    #[inline(always)]
    #[must_use]
    pub fn mpsel(&mut self) -> MPSEL_W<12> {
        MPSEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Module Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mcr](index.html) module"]
pub struct MCR_SPEC;
impl crate::RegisterSpec for MCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mcr::R](R) reader structure"]
impl crate::Readable for MCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mcr::W](W) writer structure"]
impl crate::Writable for MCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MCR to value 0"]
impl crate::Resettable for MCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
