#[doc = "Register `OSCHPCTRL` reader"]
pub struct R(crate::R<OSCHPCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OSCHPCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OSCHPCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OSCHPCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OSCHPCTRL` writer"]
pub struct W(crate::W<OSCHPCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OSCHPCTRL_SPEC>;
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
impl From<crate::W<OSCHPCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OSCHPCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "XTAL1 Data Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum X1DEN_A {
    #[doc = "0: Bit X1D is not updated"]
    CONST_0 = 0,
    #[doc = "1: Bit X1D can be updated"]
    CONST_1 = 1,
}
impl From<X1DEN_A> for bool {
    #[inline(always)]
    fn from(variant: X1DEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `X1DEN` reader - XTAL1 Data Enable"]
pub struct X1DEN_R(crate::FieldReader<bool, X1DEN_A>);
impl X1DEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        X1DEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
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
        **self == X1DEN_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        **self == X1DEN_A::CONST_1
    }
}
impl core::ops::Deref for X1DEN_R {
    type Target = crate::FieldReader<bool, X1DEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `X1DEN` writer - XTAL1 Data Enable"]
pub struct X1DEN_W<'a> {
    w: &'a mut W,
}
impl<'a> X1DEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: X1DEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Bit X1D is not updated"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(X1DEN_A::CONST_0)
    }
    #[doc = "Bit X1D can be updated"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(X1DEN_A::CONST_1)
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
#[doc = "Shaper Bypass\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SHBY_A {
    #[doc = "0: The shaper is not bypassed"]
    CONST_0 = 0,
    #[doc = "1: The shaper is bypassed"]
    CONST_1 = 1,
}
impl From<SHBY_A> for bool {
    #[inline(always)]
    fn from(variant: SHBY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SHBY` reader - Shaper Bypass"]
pub struct SHBY_R(crate::FieldReader<bool, SHBY_A>);
impl SHBY_R {
    pub(crate) fn new(bits: bool) -> Self {
        SHBY_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SHBY_A {
        match self.bits {
            false => SHBY_A::CONST_0,
            true => SHBY_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        **self == SHBY_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        **self == SHBY_A::CONST_1
    }
}
impl core::ops::Deref for SHBY_R {
    type Target = crate::FieldReader<bool, SHBY_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SHBY` writer - Shaper Bypass"]
pub struct SHBY_W<'a> {
    w: &'a mut W,
}
impl<'a> SHBY_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SHBY_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The shaper is not bypassed"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(SHBY_A::CONST_0)
    }
    #[doc = "The shaper is bypassed"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(SHBY_A::CONST_1)
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
#[doc = "Oscillator Gain Selection\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum GAINSEL_A {
    #[doc = "0: The gain control is configured for frequencies from 4 MHz to 8 MHz"]
    CONST_00 = 0,
    #[doc = "1: The gain control is configured for frequencies from 4 MHz to 16 MHz"]
    CONST_01 = 1,
    #[doc = "2: The gain control is configured for frequencies from 4 MHz to 20 MHz"]
    CONST_10 = 2,
    #[doc = "3: The gain control is configured for frequencies from 4 MHz to 25 MHz"]
    CONST_11 = 3,
}
impl From<GAINSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: GAINSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `GAINSEL` reader - Oscillator Gain Selection"]
pub struct GAINSEL_R(crate::FieldReader<u8, GAINSEL_A>);
impl GAINSEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        GAINSEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GAINSEL_A {
        match self.bits {
            0 => GAINSEL_A::CONST_00,
            1 => GAINSEL_A::CONST_01,
            2 => GAINSEL_A::CONST_10,
            3 => GAINSEL_A::CONST_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CONST_00`"]
    #[inline(always)]
    pub fn is_const_00(&self) -> bool {
        **self == GAINSEL_A::CONST_00
    }
    #[doc = "Checks if the value of the field is `CONST_01`"]
    #[inline(always)]
    pub fn is_const_01(&self) -> bool {
        **self == GAINSEL_A::CONST_01
    }
    #[doc = "Checks if the value of the field is `CONST_10`"]
    #[inline(always)]
    pub fn is_const_10(&self) -> bool {
        **self == GAINSEL_A::CONST_10
    }
    #[doc = "Checks if the value of the field is `CONST_11`"]
    #[inline(always)]
    pub fn is_const_11(&self) -> bool {
        **self == GAINSEL_A::CONST_11
    }
}
impl core::ops::Deref for GAINSEL_R {
    type Target = crate::FieldReader<u8, GAINSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GAINSEL` writer - Oscillator Gain Selection"]
pub struct GAINSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> GAINSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GAINSEL_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "The gain control is configured for frequencies from 4 MHz to 8 MHz"]
    #[inline(always)]
    pub fn const_00(self) -> &'a mut W {
        self.variant(GAINSEL_A::CONST_00)
    }
    #[doc = "The gain control is configured for frequencies from 4 MHz to 16 MHz"]
    #[inline(always)]
    pub fn const_01(self) -> &'a mut W {
        self.variant(GAINSEL_A::CONST_01)
    }
    #[doc = "The gain control is configured for frequencies from 4 MHz to 20 MHz"]
    #[inline(always)]
    pub fn const_10(self) -> &'a mut W {
        self.variant(GAINSEL_A::CONST_10)
    }
    #[doc = "The gain control is configured for frequencies from 4 MHz to 25 MHz"]
    #[inline(always)]
    pub fn const_11(self) -> &'a mut W {
        self.variant(GAINSEL_A::CONST_11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | ((value as u32 & 0x03) << 2);
        self.w
    }
}
#[doc = "Oscillator Mode\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MODE_A {
    #[doc = "0: External Crystal Mode and External Input Clock Mode. The oscillator Power-Saving Mode is not entered."]
    CONST_00 = 0,
    #[doc = "1: OSC is disabled. The oscillator Power-Saving Mode is not entered."]
    CONST_01 = 1,
    #[doc = "2: External Input Clock Mode and the oscillator Power-Saving Mode is entered"]
    CONST_10 = 2,
    #[doc = "3: OSC is disabled. The oscillator Power-Saving Mode is entered."]
    CONST_11 = 3,
}
impl From<MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: MODE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `MODE` reader - Oscillator Mode"]
pub struct MODE_R(crate::FieldReader<u8, MODE_A>);
impl MODE_R {
    pub(crate) fn new(bits: u8) -> Self {
        MODE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
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
        **self == MODE_A::CONST_00
    }
    #[doc = "Checks if the value of the field is `CONST_01`"]
    #[inline(always)]
    pub fn is_const_01(&self) -> bool {
        **self == MODE_A::CONST_01
    }
    #[doc = "Checks if the value of the field is `CONST_10`"]
    #[inline(always)]
    pub fn is_const_10(&self) -> bool {
        **self == MODE_A::CONST_10
    }
    #[doc = "Checks if the value of the field is `CONST_11`"]
    #[inline(always)]
    pub fn is_const_11(&self) -> bool {
        **self == MODE_A::CONST_11
    }
}
impl core::ops::Deref for MODE_R {
    type Target = crate::FieldReader<u8, MODE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MODE` writer - Oscillator Mode"]
pub struct MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MODE_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "External Crystal Mode and External Input Clock Mode. The oscillator Power-Saving Mode is not entered."]
    #[inline(always)]
    pub fn const_00(self) -> &'a mut W {
        self.variant(MODE_A::CONST_00)
    }
    #[doc = "OSC is disabled. The oscillator Power-Saving Mode is not entered."]
    #[inline(always)]
    pub fn const_01(self) -> &'a mut W {
        self.variant(MODE_A::CONST_01)
    }
    #[doc = "External Input Clock Mode and the oscillator Power-Saving Mode is entered"]
    #[inline(always)]
    pub fn const_10(self) -> &'a mut W {
        self.variant(MODE_A::CONST_10)
    }
    #[doc = "OSC is disabled. The oscillator Power-Saving Mode is entered."]
    #[inline(always)]
    pub fn const_11(self) -> &'a mut W {
        self.variant(MODE_A::CONST_11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | ((value as u32 & 0x03) << 4);
        self.w
    }
}
#[doc = "Field `OSCVAL` reader - OSC Frequency Value"]
pub struct OSCVAL_R(crate::FieldReader<u8, u8>);
impl OSCVAL_R {
    pub(crate) fn new(bits: u8) -> Self {
        OSCVAL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OSCVAL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OSCVAL` writer - OSC Frequency Value"]
pub struct OSCVAL_W<'a> {
    w: &'a mut W,
}
impl<'a> OSCVAL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | ((value as u32 & 0x0f) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - XTAL1 Data Enable"]
    #[inline(always)]
    pub fn x1den(&self) -> X1DEN_R {
        X1DEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Shaper Bypass"]
    #[inline(always)]
    pub fn shby(&self) -> SHBY_R {
        SHBY_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bits 2:3 - Oscillator Gain Selection"]
    #[inline(always)]
    pub fn gainsel(&self) -> GAINSEL_R {
        GAINSEL_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - Oscillator Mode"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 16:19 - OSC Frequency Value"]
    #[inline(always)]
    pub fn oscval(&self) -> OSCVAL_R {
        OSCVAL_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - XTAL1 Data Enable"]
    #[inline(always)]
    pub fn x1den(&mut self) -> X1DEN_W {
        X1DEN_W { w: self }
    }
    #[doc = "Bit 1 - Shaper Bypass"]
    #[inline(always)]
    pub fn shby(&mut self) -> SHBY_W {
        SHBY_W { w: self }
    }
    #[doc = "Bits 2:3 - Oscillator Gain Selection"]
    #[inline(always)]
    pub fn gainsel(&mut self) -> GAINSEL_W {
        GAINSEL_W { w: self }
    }
    #[doc = "Bits 4:5 - Oscillator Mode"]
    #[inline(always)]
    pub fn mode(&mut self) -> MODE_W {
        MODE_W { w: self }
    }
    #[doc = "Bits 16:19 - OSC Frequency Value"]
    #[inline(always)]
    pub fn oscval(&mut self) -> OSCVAL_W {
        OSCVAL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "OSC_HP Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [oschpctrl](index.html) module"]
pub struct OSCHPCTRL_SPEC;
impl crate::RegisterSpec for OSCHPCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [oschpctrl::R](R) reader structure"]
impl crate::Readable for OSCHPCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [oschpctrl::W](W) writer structure"]
impl crate::Writable for OSCHPCTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OSCHPCTRL to value 0x3c"]
impl crate::Resettable for OSCHPCTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x3c
    }
}
