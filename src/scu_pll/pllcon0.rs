#[doc = "Register `PLLCON0` reader"]
pub struct R(crate::R<PLLCON0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PLLCON0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PLLCON0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PLLCON0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PLLCON0` writer"]
pub struct W(crate::W<PLLCON0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PLLCON0_SPEC>;
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
impl From<crate::W<PLLCON0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PLLCON0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VCOBYP` reader - VCO Bypass"]
pub type VCOBYP_R = crate::BitReader<VCOBYP_A>;
#[doc = "VCO Bypass\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VCOBYP_A {
    #[doc = "0: Normal operation, VCO is not bypassed"]
    CONST_0 = 0,
    #[doc = "1: Prescaler Mode, VCO is bypassed"]
    CONST_1 = 1,
}
impl From<VCOBYP_A> for bool {
    #[inline(always)]
    fn from(variant: VCOBYP_A) -> Self {
        variant as u8 != 0
    }
}
impl VCOBYP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VCOBYP_A {
        match self.bits {
            false => VCOBYP_A::CONST_0,
            true => VCOBYP_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == VCOBYP_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == VCOBYP_A::CONST_1
    }
}
#[doc = "Field `VCOBYP` writer - VCO Bypass"]
pub type VCOBYP_W<'a, const O: u8> = crate::BitWriter<'a, PLLCON0_SPEC, O, VCOBYP_A>;
impl<'a, const O: u8> VCOBYP_W<'a, O> {
    #[doc = "Normal operation, VCO is not bypassed"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(VCOBYP_A::CONST_0)
    }
    #[doc = "Prescaler Mode, VCO is bypassed"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(VCOBYP_A::CONST_1)
    }
}
#[doc = "Field `VCOPWD` reader - VCO Power Saving Mode"]
pub type VCOPWD_R = crate::BitReader<VCOPWD_A>;
#[doc = "VCO Power Saving Mode\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VCOPWD_A {
    #[doc = "0: Normal behavior"]
    CONST_0 = 0,
    #[doc = "1: The VCO is put into a Power Saving Mode and can no longer be used. Only the Bypass and Prescaler Mode are active if previously selected."]
    CONST_1 = 1,
}
impl From<VCOPWD_A> for bool {
    #[inline(always)]
    fn from(variant: VCOPWD_A) -> Self {
        variant as u8 != 0
    }
}
impl VCOPWD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VCOPWD_A {
        match self.bits {
            false => VCOPWD_A::CONST_0,
            true => VCOPWD_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == VCOPWD_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == VCOPWD_A::CONST_1
    }
}
#[doc = "Field `VCOPWD` writer - VCO Power Saving Mode"]
pub type VCOPWD_W<'a, const O: u8> = crate::BitWriter<'a, PLLCON0_SPEC, O, VCOPWD_A>;
impl<'a, const O: u8> VCOPWD_W<'a, O> {
    #[doc = "Normal behavior"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(VCOPWD_A::CONST_0)
    }
    #[doc = "The VCO is put into a Power Saving Mode and can no longer be used. Only the Bypass and Prescaler Mode are active if previously selected."]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(VCOPWD_A::CONST_1)
    }
}
#[doc = "Field `VCOTR` reader - VCO Trim Control"]
pub type VCOTR_R = crate::BitReader<VCOTR_A>;
#[doc = "VCO Trim Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VCOTR_A {
    #[doc = "0: VCO bandwidth is operation in the normal range. VCO output frequency is between 260 and 520 MHz for a input frequency between 8 and 16 MHz."]
    CONST_0 = 0,
    #[doc = "1: VCO bandwidth is operation in the test range. VCO output frequency is between 260 and 520 MHz for a input frequency between 8 and 16 MHz."]
    CONST_1 = 1,
}
impl From<VCOTR_A> for bool {
    #[inline(always)]
    fn from(variant: VCOTR_A) -> Self {
        variant as u8 != 0
    }
}
impl VCOTR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VCOTR_A {
        match self.bits {
            false => VCOTR_A::CONST_0,
            true => VCOTR_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == VCOTR_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == VCOTR_A::CONST_1
    }
}
#[doc = "Field `VCOTR` writer - VCO Trim Control"]
pub type VCOTR_W<'a, const O: u8> = crate::BitWriter<'a, PLLCON0_SPEC, O, VCOTR_A>;
impl<'a, const O: u8> VCOTR_W<'a, O> {
    #[doc = "VCO bandwidth is operation in the normal range. VCO output frequency is between 260 and 520 MHz for a input frequency between 8 and 16 MHz."]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(VCOTR_A::CONST_0)
    }
    #[doc = "VCO bandwidth is operation in the test range. VCO output frequency is between 260 and 520 MHz for a input frequency between 8 and 16 MHz."]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(VCOTR_A::CONST_1)
    }
}
#[doc = "Field `FINDIS` reader - Disconnect Oscillator from VCO"]
pub type FINDIS_R = crate::BitReader<FINDIS_A>;
#[doc = "Disconnect Oscillator from VCO\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FINDIS_A {
    #[doc = "0: connect oscillator to the VCO part"]
    CONST_0 = 0,
    #[doc = "1: disconnect oscillator from the VCO part."]
    CONST_1 = 1,
}
impl From<FINDIS_A> for bool {
    #[inline(always)]
    fn from(variant: FINDIS_A) -> Self {
        variant as u8 != 0
    }
}
impl FINDIS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FINDIS_A {
        match self.bits {
            false => FINDIS_A::CONST_0,
            true => FINDIS_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == FINDIS_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == FINDIS_A::CONST_1
    }
}
#[doc = "Field `FINDIS` writer - Disconnect Oscillator from VCO"]
pub type FINDIS_W<'a, const O: u8> = crate::BitWriter<'a, PLLCON0_SPEC, O, FINDIS_A>;
impl<'a, const O: u8> FINDIS_W<'a, O> {
    #[doc = "connect oscillator to the VCO part"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(FINDIS_A::CONST_0)
    }
    #[doc = "disconnect oscillator from the VCO part."]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(FINDIS_A::CONST_1)
    }
}
#[doc = "Field `OSCDISCDIS` reader - Oscillator Disconnect Disable"]
pub type OSCDISCDIS_R = crate::BitReader<OSCDISCDIS_A>;
#[doc = "Oscillator Disconnect Disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OSCDISCDIS_A {
    #[doc = "0: In case of a PLL loss-of-lock bit FINDIS is set"]
    CONST_0 = 0,
    #[doc = "1: In case of a PLL loss-of-lock bit FINDIS is cleared"]
    CONST_1 = 1,
}
impl From<OSCDISCDIS_A> for bool {
    #[inline(always)]
    fn from(variant: OSCDISCDIS_A) -> Self {
        variant as u8 != 0
    }
}
impl OSCDISCDIS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OSCDISCDIS_A {
        match self.bits {
            false => OSCDISCDIS_A::CONST_0,
            true => OSCDISCDIS_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == OSCDISCDIS_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == OSCDISCDIS_A::CONST_1
    }
}
#[doc = "Field `OSCDISCDIS` writer - Oscillator Disconnect Disable"]
pub type OSCDISCDIS_W<'a, const O: u8> = crate::BitWriter<'a, PLLCON0_SPEC, O, OSCDISCDIS_A>;
impl<'a, const O: u8> OSCDISCDIS_W<'a, O> {
    #[doc = "In case of a PLL loss-of-lock bit FINDIS is set"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(OSCDISCDIS_A::CONST_0)
    }
    #[doc = "In case of a PLL loss-of-lock bit FINDIS is cleared"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(OSCDISCDIS_A::CONST_1)
    }
}
#[doc = "Field `PLLPWD` reader - PLL Power Saving Mode"]
pub type PLLPWD_R = crate::BitReader<PLLPWD_A>;
#[doc = "PLL Power Saving Mode\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PLLPWD_A {
    #[doc = "0: Normal behavior"]
    CONST_0 = 0,
    #[doc = "1: The complete PLL block is put into a Power Saving Mode and can no longer be used. Only the Bypass Mode is active if previously selected."]
    CONST_1 = 1,
}
impl From<PLLPWD_A> for bool {
    #[inline(always)]
    fn from(variant: PLLPWD_A) -> Self {
        variant as u8 != 0
    }
}
impl PLLPWD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PLLPWD_A {
        match self.bits {
            false => PLLPWD_A::CONST_0,
            true => PLLPWD_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == PLLPWD_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == PLLPWD_A::CONST_1
    }
}
#[doc = "Field `PLLPWD` writer - PLL Power Saving Mode"]
pub type PLLPWD_W<'a, const O: u8> = crate::BitWriter<'a, PLLCON0_SPEC, O, PLLPWD_A>;
impl<'a, const O: u8> PLLPWD_W<'a, O> {
    #[doc = "Normal behavior"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(PLLPWD_A::CONST_0)
    }
    #[doc = "The complete PLL block is put into a Power Saving Mode and can no longer be used. Only the Bypass Mode is active if previously selected."]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(PLLPWD_A::CONST_1)
    }
}
#[doc = "Field `OSCRES` reader - Oscillator Watchdog Reset"]
pub type OSCRES_R = crate::BitReader<OSCRES_A>;
#[doc = "Oscillator Watchdog Reset\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OSCRES_A {
    #[doc = "0: The Oscillator Watchdog of the PLL is not reset and remains active"]
    CONST_0 = 0,
    #[doc = "1: The Oscillator Watchdog of the PLL is reset"]
    CONST_1 = 1,
}
impl From<OSCRES_A> for bool {
    #[inline(always)]
    fn from(variant: OSCRES_A) -> Self {
        variant as u8 != 0
    }
}
impl OSCRES_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OSCRES_A {
        match self.bits {
            false => OSCRES_A::CONST_0,
            true => OSCRES_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == OSCRES_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == OSCRES_A::CONST_1
    }
}
#[doc = "Field `OSCRES` writer - Oscillator Watchdog Reset"]
pub type OSCRES_W<'a, const O: u8> = crate::BitWriter<'a, PLLCON0_SPEC, O, OSCRES_A>;
impl<'a, const O: u8> OSCRES_W<'a, O> {
    #[doc = "The Oscillator Watchdog of the PLL is not reset and remains active"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(OSCRES_A::CONST_0)
    }
    #[doc = "The Oscillator Watchdog of the PLL is reset"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(OSCRES_A::CONST_1)
    }
}
#[doc = "Field `RESLD` writer - Restart VCO Lock Detection"]
pub type RESLD_W<'a, const O: u8> = crate::BitWriter<'a, PLLCON0_SPEC, O>;
#[doc = "Field `AOTREN` reader - Automatic Oscillator Calibration Enable"]
pub type AOTREN_R = crate::BitReader<AOTREN_A>;
#[doc = "Automatic Oscillator Calibration Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AOTREN_A {
    #[doc = "0: Disable"]
    CONST_0 = 0,
    #[doc = "1: Enable"]
    CONST_1 = 1,
}
impl From<AOTREN_A> for bool {
    #[inline(always)]
    fn from(variant: AOTREN_A) -> Self {
        variant as u8 != 0
    }
}
impl AOTREN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AOTREN_A {
        match self.bits {
            false => AOTREN_A::CONST_0,
            true => AOTREN_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == AOTREN_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == AOTREN_A::CONST_1
    }
}
#[doc = "Field `AOTREN` writer - Automatic Oscillator Calibration Enable"]
pub type AOTREN_W<'a, const O: u8> = crate::BitWriter<'a, PLLCON0_SPEC, O, AOTREN_A>;
impl<'a, const O: u8> AOTREN_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(AOTREN_A::CONST_0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(AOTREN_A::CONST_1)
    }
}
#[doc = "Field `FOTR` reader - Factory Oscillator Calibration"]
pub type FOTR_R = crate::BitReader<FOTR_A>;
#[doc = "Factory Oscillator Calibration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FOTR_A {
    #[doc = "0: No effect"]
    CONST_0 = 0,
    #[doc = "1: Force fixed-value trimming"]
    CONST_1 = 1,
}
impl From<FOTR_A> for bool {
    #[inline(always)]
    fn from(variant: FOTR_A) -> Self {
        variant as u8 != 0
    }
}
impl FOTR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FOTR_A {
        match self.bits {
            false => FOTR_A::CONST_0,
            true => FOTR_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == FOTR_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == FOTR_A::CONST_1
    }
}
#[doc = "Field `FOTR` writer - Factory Oscillator Calibration"]
pub type FOTR_W<'a, const O: u8> = crate::BitWriter<'a, PLLCON0_SPEC, O, FOTR_A>;
impl<'a, const O: u8> FOTR_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(FOTR_A::CONST_0)
    }
    #[doc = "Force fixed-value trimming"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(FOTR_A::CONST_1)
    }
}
impl R {
    #[doc = "Bit 0 - VCO Bypass"]
    #[inline(always)]
    pub fn vcobyp(&self) -> VCOBYP_R {
        VCOBYP_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - VCO Power Saving Mode"]
    #[inline(always)]
    pub fn vcopwd(&self) -> VCOPWD_R {
        VCOPWD_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - VCO Trim Control"]
    #[inline(always)]
    pub fn vcotr(&self) -> VCOTR_R {
        VCOTR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - Disconnect Oscillator from VCO"]
    #[inline(always)]
    pub fn findis(&self) -> FINDIS_R {
        FINDIS_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - Oscillator Disconnect Disable"]
    #[inline(always)]
    pub fn oscdiscdis(&self) -> OSCDISCDIS_R {
        OSCDISCDIS_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 16 - PLL Power Saving Mode"]
    #[inline(always)]
    pub fn pllpwd(&self) -> PLLPWD_R {
        PLLPWD_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Oscillator Watchdog Reset"]
    #[inline(always)]
    pub fn oscres(&self) -> OSCRES_R {
        OSCRES_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 19 - Automatic Oscillator Calibration Enable"]
    #[inline(always)]
    pub fn aotren(&self) -> AOTREN_R {
        AOTREN_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Factory Oscillator Calibration"]
    #[inline(always)]
    pub fn fotr(&self) -> FOTR_R {
        FOTR_R::new(((self.bits >> 20) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - VCO Bypass"]
    #[inline(always)]
    #[must_use]
    pub fn vcobyp(&mut self) -> VCOBYP_W<0> {
        VCOBYP_W::new(self)
    }
    #[doc = "Bit 1 - VCO Power Saving Mode"]
    #[inline(always)]
    #[must_use]
    pub fn vcopwd(&mut self) -> VCOPWD_W<1> {
        VCOPWD_W::new(self)
    }
    #[doc = "Bit 2 - VCO Trim Control"]
    #[inline(always)]
    #[must_use]
    pub fn vcotr(&mut self) -> VCOTR_W<2> {
        VCOTR_W::new(self)
    }
    #[doc = "Bit 4 - Disconnect Oscillator from VCO"]
    #[inline(always)]
    #[must_use]
    pub fn findis(&mut self) -> FINDIS_W<4> {
        FINDIS_W::new(self)
    }
    #[doc = "Bit 6 - Oscillator Disconnect Disable"]
    #[inline(always)]
    #[must_use]
    pub fn oscdiscdis(&mut self) -> OSCDISCDIS_W<6> {
        OSCDISCDIS_W::new(self)
    }
    #[doc = "Bit 16 - PLL Power Saving Mode"]
    #[inline(always)]
    #[must_use]
    pub fn pllpwd(&mut self) -> PLLPWD_W<16> {
        PLLPWD_W::new(self)
    }
    #[doc = "Bit 17 - Oscillator Watchdog Reset"]
    #[inline(always)]
    #[must_use]
    pub fn oscres(&mut self) -> OSCRES_W<17> {
        OSCRES_W::new(self)
    }
    #[doc = "Bit 18 - Restart VCO Lock Detection"]
    #[inline(always)]
    #[must_use]
    pub fn resld(&mut self) -> RESLD_W<18> {
        RESLD_W::new(self)
    }
    #[doc = "Bit 19 - Automatic Oscillator Calibration Enable"]
    #[inline(always)]
    #[must_use]
    pub fn aotren(&mut self) -> AOTREN_W<19> {
        AOTREN_W::new(self)
    }
    #[doc = "Bit 20 - Factory Oscillator Calibration"]
    #[inline(always)]
    #[must_use]
    pub fn fotr(&mut self) -> FOTR_W<20> {
        FOTR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PLL Configuration 0 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pllcon0](index.html) module"]
pub struct PLLCON0_SPEC;
impl crate::RegisterSpec for PLLCON0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pllcon0::R](R) reader structure"]
impl crate::Readable for PLLCON0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pllcon0::W](W) writer structure"]
impl crate::Writable for PLLCON0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PLLCON0 to value 0x0003_0003"]
impl crate::Resettable for PLLCON0_SPEC {
    const RESET_VALUE: Self::Ux = 0x0003_0003;
}
