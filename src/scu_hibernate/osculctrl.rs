#[doc = "Register `OSCULCTRL` reader"]
pub struct R(crate::R<OSCULCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OSCULCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OSCULCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OSCULCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OSCULCTRL` writer"]
pub struct W(crate::W<OSCULCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OSCULCTRL_SPEC>;
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
impl From<crate::W<OSCULCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OSCULCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `X1DEN` reader - XTAL1 Data General Purpose Input Enable"]
pub type X1DEN_R = crate::BitReader<X1DEN_A>;
#[doc = "XTAL1 Data General Purpose Input Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum X1DEN_A {
    #[doc = "0: Data input inactivated, power down"]
    CONST_0 = 0,
    #[doc = "1: Data input active"]
    CONST_1 = 1,
}
impl From<X1DEN_A> for bool {
    #[inline(always)]
    fn from(variant: X1DEN_A) -> Self {
        variant as u8 != 0
    }
}
impl X1DEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> X1DEN_A {
        match self.bits {
            false => X1DEN_A::CONST_0,
            true => X1DEN_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == X1DEN_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == X1DEN_A::CONST_1
    }
}
#[doc = "Field `X1DEN` writer - XTAL1 Data General Purpose Input Enable"]
pub type X1DEN_W<'a, const O: u8> = crate::BitWriter<'a, OSCULCTRL_SPEC, O, X1DEN_A>;
impl<'a, const O: u8> X1DEN_W<'a, O> {
    #[doc = "Data input inactivated, power down"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(X1DEN_A::CONST_0)
    }
    #[doc = "Data input active"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(X1DEN_A::CONST_1)
    }
}
#[doc = "Field `MODE` reader - Oscillator Mode"]
pub type MODE_R = crate::FieldReader<MODE_A>;
#[doc = "Oscillator Mode\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MODE_A {
    #[doc = "0: Oscillator is enabled, in operation"]
    CONST_00 = 0,
    #[doc = "1: Oscillator is enabled, in bypass mode"]
    CONST_01 = 1,
    #[doc = "2: Oscillator in power down"]
    CONST_10 = 2,
    #[doc = "3: Oscillator in power down, can be used as GPI"]
    CONST_11 = 3,
}
impl From<MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: MODE_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for MODE_A {
    type Ux = u8;
}
impl MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MODE_A {
        match self.bits {
            0 => MODE_A::CONST_00,
            1 => MODE_A::CONST_01,
            2 => MODE_A::CONST_10,
            3 => MODE_A::CONST_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CONST_00`"]
    #[inline(always)]
    pub fn is_const_00(&self) -> bool {
        *self == MODE_A::CONST_00
    }
    #[doc = "Checks if the value of the field is `CONST_01`"]
    #[inline(always)]
    pub fn is_const_01(&self) -> bool {
        *self == MODE_A::CONST_01
    }
    #[doc = "Checks if the value of the field is `CONST_10`"]
    #[inline(always)]
    pub fn is_const_10(&self) -> bool {
        *self == MODE_A::CONST_10
    }
    #[doc = "Checks if the value of the field is `CONST_11`"]
    #[inline(always)]
    pub fn is_const_11(&self) -> bool {
        *self == MODE_A::CONST_11
    }
}
#[doc = "Field `MODE` writer - Oscillator Mode"]
pub type MODE_W<'a, const O: u8> = crate::FieldWriterSafe<'a, OSCULCTRL_SPEC, 2, O, MODE_A>;
impl<'a, const O: u8> MODE_W<'a, O> {
    #[doc = "Oscillator is enabled, in operation"]
    #[inline(always)]
    pub fn const_00(self) -> &'a mut W {
        self.variant(MODE_A::CONST_00)
    }
    #[doc = "Oscillator is enabled, in bypass mode"]
    #[inline(always)]
    pub fn const_01(self) -> &'a mut W {
        self.variant(MODE_A::CONST_01)
    }
    #[doc = "Oscillator in power down"]
    #[inline(always)]
    pub fn const_10(self) -> &'a mut W {
        self.variant(MODE_A::CONST_10)
    }
    #[doc = "Oscillator in power down, can be used as GPI"]
    #[inline(always)]
    pub fn const_11(self) -> &'a mut W {
        self.variant(MODE_A::CONST_11)
    }
}
impl R {
    #[doc = "Bit 0 - XTAL1 Data General Purpose Input Enable"]
    #[inline(always)]
    pub fn x1den(&self) -> X1DEN_R {
        X1DEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 4:5 - Oscillator Mode"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 4) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - XTAL1 Data General Purpose Input Enable"]
    #[inline(always)]
    #[must_use]
    pub fn x1den(&mut self) -> X1DEN_W<0> {
        X1DEN_W::new(self)
    }
    #[doc = "Bits 4:5 - Oscillator Mode"]
    #[inline(always)]
    #[must_use]
    pub fn mode(&mut self) -> MODE_W<4> {
        MODE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "OSC_ULP Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [osculctrl](index.html) module"]
pub struct OSCULCTRL_SPEC;
impl crate::RegisterSpec for OSCULCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [osculctrl::R](R) reader structure"]
impl crate::Readable for OSCULCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [osculctrl::W](W) writer structure"]
impl crate::Writable for OSCULCTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OSCULCTRL to value 0x20"]
impl crate::Resettable for OSCULCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0x20;
}
