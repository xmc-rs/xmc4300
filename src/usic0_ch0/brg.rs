#[doc = "Register `BRG` reader"]
pub struct R(crate::R<BRG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BRG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BRG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BRG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BRG` writer"]
pub struct W(crate::W<BRG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BRG_SPEC>;
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
impl From<crate::W<BRG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BRG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CLKSEL` reader - Clock Selection"]
pub type CLKSEL_R = crate::FieldReader<u8, CLKSEL_A>;
#[doc = "Clock Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CLKSEL_A {
    #[doc = "0: The fractional divider frequency fFD is selected."]
    VALUE1 = 0,
    #[doc = "2: The trigger signal DX1T defines fPIN. Signal MCLK toggles with fPIN."]
    VALUE3 = 2,
    #[doc = "3: Signal MCLK corresponds to the DX1S signal and the frequency fPIN is derived from the rising edges of DX1S."]
    VALUE4 = 3,
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
            2 => Some(CLKSEL_A::VALUE3),
            3 => Some(CLKSEL_A::VALUE4),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CLKSEL_A::VALUE1
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
}
#[doc = "Field `CLKSEL` writer - Clock Selection"]
pub type CLKSEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BRG_SPEC, u8, CLKSEL_A, 2, O>;
impl<'a, const O: u8> CLKSEL_W<'a, O> {
    #[doc = "The fractional divider frequency fFD is selected."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CLKSEL_A::VALUE1)
    }
    #[doc = "The trigger signal DX1T defines fPIN. Signal MCLK toggles with fPIN."]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(CLKSEL_A::VALUE3)
    }
    #[doc = "Signal MCLK corresponds to the DX1S signal and the frequency fPIN is derived from the rising edges of DX1S."]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(CLKSEL_A::VALUE4)
    }
}
#[doc = "Field `TMEN` reader - Timing Measurement Enable"]
pub type TMEN_R = crate::BitReader<TMEN_A>;
#[doc = "Timing Measurement Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TMEN_A {
    #[doc = "0: Timing measurement is disabled: The trigger signals DX0T and DX1T are ignored."]
    VALUE1 = 0,
    #[doc = "1: Timing measurement is enabled: The 10-bit counter is incremented by 1 with fPPP and stops counting when reaching its maximum value. If one of the trigger signals DX0T or DX1T become active, the counter value is captured into bit field CTV, the counter is cleared and a transmit shift event is generated."]
    VALUE2 = 1,
}
impl From<TMEN_A> for bool {
    #[inline(always)]
    fn from(variant: TMEN_A) -> Self {
        variant as u8 != 0
    }
}
impl TMEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TMEN_A {
        match self.bits {
            false => TMEN_A::VALUE1,
            true => TMEN_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == TMEN_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == TMEN_A::VALUE2
    }
}
#[doc = "Field `TMEN` writer - Timing Measurement Enable"]
pub type TMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, BRG_SPEC, TMEN_A, O>;
impl<'a, const O: u8> TMEN_W<'a, O> {
    #[doc = "Timing measurement is disabled: The trigger signals DX0T and DX1T are ignored."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(TMEN_A::VALUE1)
    }
    #[doc = "Timing measurement is enabled: The 10-bit counter is incremented by 1 with fPPP and stops counting when reaching its maximum value. If one of the trigger signals DX0T or DX1T become active, the counter value is captured into bit field CTV, the counter is cleared and a transmit shift event is generated."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(TMEN_A::VALUE2)
    }
}
#[doc = "Field `PPPEN` reader - Enable 2:1 Divider for fPPP"]
pub type PPPEN_R = crate::BitReader<PPPEN_A>;
#[doc = "Enable 2:1 Divider for fPPP\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PPPEN_A {
    #[doc = "0: The 2:1 divider for fPPP is disabled. fPPP = fPIN"]
    VALUE1 = 0,
    #[doc = "1: The 2:1 divider for fPPP is enabled. fPPP = fMCLK = fPIN / 2."]
    VALUE2 = 1,
}
impl From<PPPEN_A> for bool {
    #[inline(always)]
    fn from(variant: PPPEN_A) -> Self {
        variant as u8 != 0
    }
}
impl PPPEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PPPEN_A {
        match self.bits {
            false => PPPEN_A::VALUE1,
            true => PPPEN_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PPPEN_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PPPEN_A::VALUE2
    }
}
#[doc = "Field `PPPEN` writer - Enable 2:1 Divider for fPPP"]
pub type PPPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, BRG_SPEC, PPPEN_A, O>;
impl<'a, const O: u8> PPPEN_W<'a, O> {
    #[doc = "The 2:1 divider for fPPP is disabled. fPPP = fPIN"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(PPPEN_A::VALUE1)
    }
    #[doc = "The 2:1 divider for fPPP is enabled. fPPP = fMCLK = fPIN / 2."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(PPPEN_A::VALUE2)
    }
}
#[doc = "Field `CTQSEL` reader - Input Selection for CTQ"]
pub type CTQSEL_R = crate::FieldReader<u8, CTQSEL_A>;
#[doc = "Input Selection for CTQ\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CTQSEL_A {
    #[doc = "0: fCTQIN = fPDIV"]
    VALUE1 = 0,
    #[doc = "1: fCTQIN = fPPP"]
    VALUE2 = 1,
    #[doc = "2: fCTQIN = fSCLK"]
    VALUE3 = 2,
    #[doc = "3: fCTQIN = fMCLK"]
    VALUE4 = 3,
}
impl From<CTQSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: CTQSEL_A) -> Self {
        variant as _
    }
}
impl CTQSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTQSEL_A {
        match self.bits {
            0 => CTQSEL_A::VALUE1,
            1 => CTQSEL_A::VALUE2,
            2 => CTQSEL_A::VALUE3,
            3 => CTQSEL_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CTQSEL_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CTQSEL_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == CTQSEL_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == CTQSEL_A::VALUE4
    }
}
#[doc = "Field `CTQSEL` writer - Input Selection for CTQ"]
pub type CTQSEL_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, BRG_SPEC, u8, CTQSEL_A, 2, O>;
impl<'a, const O: u8> CTQSEL_W<'a, O> {
    #[doc = "fCTQIN = fPDIV"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CTQSEL_A::VALUE1)
    }
    #[doc = "fCTQIN = fPPP"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CTQSEL_A::VALUE2)
    }
    #[doc = "fCTQIN = fSCLK"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(CTQSEL_A::VALUE3)
    }
    #[doc = "fCTQIN = fMCLK"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(CTQSEL_A::VALUE4)
    }
}
#[doc = "Field `PCTQ` reader - Pre-Divider for Time Quanta Counter"]
pub type PCTQ_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PCTQ` writer - Pre-Divider for Time Quanta Counter"]
pub type PCTQ_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BRG_SPEC, u8, u8, 2, O>;
#[doc = "Field `DCTQ` reader - Denominator for Time Quanta Counter"]
pub type DCTQ_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DCTQ` writer - Denominator for Time Quanta Counter"]
pub type DCTQ_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BRG_SPEC, u8, u8, 5, O>;
#[doc = "Field `PDIV` reader - Divider Mode: Divider Factor to Generate fPDIV"]
pub type PDIV_R = crate::FieldReader<u16, u16>;
#[doc = "Field `PDIV` writer - Divider Mode: Divider Factor to Generate fPDIV"]
pub type PDIV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BRG_SPEC, u16, u16, 10, O>;
#[doc = "Field `SCLKOSEL` reader - Shift Clock Output Select"]
pub type SCLKOSEL_R = crate::BitReader<SCLKOSEL_A>;
#[doc = "Shift Clock Output Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SCLKOSEL_A {
    #[doc = "0: SCLK from the baud rate generator is selected as the SCLKOUT input source."]
    VALUE1 = 0,
    #[doc = "1: The transmit shift clock from DX1 input stage is selected as the SCLKOUT input source."]
    VALUE2 = 1,
}
impl From<SCLKOSEL_A> for bool {
    #[inline(always)]
    fn from(variant: SCLKOSEL_A) -> Self {
        variant as u8 != 0
    }
}
impl SCLKOSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SCLKOSEL_A {
        match self.bits {
            false => SCLKOSEL_A::VALUE1,
            true => SCLKOSEL_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SCLKOSEL_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SCLKOSEL_A::VALUE2
    }
}
#[doc = "Field `SCLKOSEL` writer - Shift Clock Output Select"]
pub type SCLKOSEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, BRG_SPEC, SCLKOSEL_A, O>;
impl<'a, const O: u8> SCLKOSEL_W<'a, O> {
    #[doc = "SCLK from the baud rate generator is selected as the SCLKOUT input source."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(SCLKOSEL_A::VALUE1)
    }
    #[doc = "The transmit shift clock from DX1 input stage is selected as the SCLKOUT input source."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(SCLKOSEL_A::VALUE2)
    }
}
#[doc = "Field `MCLKCFG` reader - Master Clock Configuration"]
pub type MCLKCFG_R = crate::BitReader<MCLKCFG_A>;
#[doc = "Master Clock Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MCLKCFG_A {
    #[doc = "0: The passive level is 0."]
    VALUE1 = 0,
    #[doc = "1: The passive level is 1."]
    VALUE2 = 1,
}
impl From<MCLKCFG_A> for bool {
    #[inline(always)]
    fn from(variant: MCLKCFG_A) -> Self {
        variant as u8 != 0
    }
}
impl MCLKCFG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MCLKCFG_A {
        match self.bits {
            false => MCLKCFG_A::VALUE1,
            true => MCLKCFG_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == MCLKCFG_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == MCLKCFG_A::VALUE2
    }
}
#[doc = "Field `MCLKCFG` writer - Master Clock Configuration"]
pub type MCLKCFG_W<'a, const O: u8> = crate::BitWriter<'a, u32, BRG_SPEC, MCLKCFG_A, O>;
impl<'a, const O: u8> MCLKCFG_W<'a, O> {
    #[doc = "The passive level is 0."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(MCLKCFG_A::VALUE1)
    }
    #[doc = "The passive level is 1."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(MCLKCFG_A::VALUE2)
    }
}
#[doc = "Field `SCLKCFG` reader - Shift Clock Output Configuration"]
pub type SCLKCFG_R = crate::FieldReader<u8, SCLKCFG_A>;
#[doc = "Shift Clock Output Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SCLKCFG_A {
    #[doc = "0: The passive level is 0 and the delay is disabled."]
    VALUE1 = 0,
    #[doc = "1: The passive level is 1 and the delay is disabled."]
    VALUE2 = 1,
    #[doc = "2: The passive level is 0 and the delay is enabled."]
    VALUE3 = 2,
    #[doc = "3: The passive level is 1 and the delay is enabled."]
    VALUE4 = 3,
}
impl From<SCLKCFG_A> for u8 {
    #[inline(always)]
    fn from(variant: SCLKCFG_A) -> Self {
        variant as _
    }
}
impl SCLKCFG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SCLKCFG_A {
        match self.bits {
            0 => SCLKCFG_A::VALUE1,
            1 => SCLKCFG_A::VALUE2,
            2 => SCLKCFG_A::VALUE3,
            3 => SCLKCFG_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SCLKCFG_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SCLKCFG_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == SCLKCFG_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == SCLKCFG_A::VALUE4
    }
}
#[doc = "Field `SCLKCFG` writer - Shift Clock Output Configuration"]
pub type SCLKCFG_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, BRG_SPEC, u8, SCLKCFG_A, 2, O>;
impl<'a, const O: u8> SCLKCFG_W<'a, O> {
    #[doc = "The passive level is 0 and the delay is disabled."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(SCLKCFG_A::VALUE1)
    }
    #[doc = "The passive level is 1 and the delay is disabled."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(SCLKCFG_A::VALUE2)
    }
    #[doc = "The passive level is 0 and the delay is enabled."]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(SCLKCFG_A::VALUE3)
    }
    #[doc = "The passive level is 1 and the delay is enabled."]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(SCLKCFG_A::VALUE4)
    }
}
impl R {
    #[doc = "Bits 0:1 - Clock Selection"]
    #[inline(always)]
    pub fn clksel(&self) -> CLKSEL_R {
        CLKSEL_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 3 - Timing Measurement Enable"]
    #[inline(always)]
    pub fn tmen(&self) -> TMEN_R {
        TMEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable 2:1 Divider for fPPP"]
    #[inline(always)]
    pub fn pppen(&self) -> PPPEN_R {
        PPPEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 6:7 - Input Selection for CTQ"]
    #[inline(always)]
    pub fn ctqsel(&self) -> CTQSEL_R {
        CTQSEL_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Pre-Divider for Time Quanta Counter"]
    #[inline(always)]
    pub fn pctq(&self) -> PCTQ_R {
        PCTQ_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:14 - Denominator for Time Quanta Counter"]
    #[inline(always)]
    pub fn dctq(&self) -> DCTQ_R {
        DCTQ_R::new(((self.bits >> 10) & 0x1f) as u8)
    }
    #[doc = "Bits 16:25 - Divider Mode: Divider Factor to Generate fPDIV"]
    #[inline(always)]
    pub fn pdiv(&self) -> PDIV_R {
        PDIV_R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
    #[doc = "Bit 28 - Shift Clock Output Select"]
    #[inline(always)]
    pub fn sclkosel(&self) -> SCLKOSEL_R {
        SCLKOSEL_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Master Clock Configuration"]
    #[inline(always)]
    pub fn mclkcfg(&self) -> MCLKCFG_R {
        MCLKCFG_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bits 30:31 - Shift Clock Output Configuration"]
    #[inline(always)]
    pub fn sclkcfg(&self) -> SCLKCFG_R {
        SCLKCFG_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Clock Selection"]
    #[inline(always)]
    #[must_use]
    pub fn clksel(&mut self) -> CLKSEL_W<0> {
        CLKSEL_W::new(self)
    }
    #[doc = "Bit 3 - Timing Measurement Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tmen(&mut self) -> TMEN_W<3> {
        TMEN_W::new(self)
    }
    #[doc = "Bit 4 - Enable 2:1 Divider for fPPP"]
    #[inline(always)]
    #[must_use]
    pub fn pppen(&mut self) -> PPPEN_W<4> {
        PPPEN_W::new(self)
    }
    #[doc = "Bits 6:7 - Input Selection for CTQ"]
    #[inline(always)]
    #[must_use]
    pub fn ctqsel(&mut self) -> CTQSEL_W<6> {
        CTQSEL_W::new(self)
    }
    #[doc = "Bits 8:9 - Pre-Divider for Time Quanta Counter"]
    #[inline(always)]
    #[must_use]
    pub fn pctq(&mut self) -> PCTQ_W<8> {
        PCTQ_W::new(self)
    }
    #[doc = "Bits 10:14 - Denominator for Time Quanta Counter"]
    #[inline(always)]
    #[must_use]
    pub fn dctq(&mut self) -> DCTQ_W<10> {
        DCTQ_W::new(self)
    }
    #[doc = "Bits 16:25 - Divider Mode: Divider Factor to Generate fPDIV"]
    #[inline(always)]
    #[must_use]
    pub fn pdiv(&mut self) -> PDIV_W<16> {
        PDIV_W::new(self)
    }
    #[doc = "Bit 28 - Shift Clock Output Select"]
    #[inline(always)]
    #[must_use]
    pub fn sclkosel(&mut self) -> SCLKOSEL_W<28> {
        SCLKOSEL_W::new(self)
    }
    #[doc = "Bit 29 - Master Clock Configuration"]
    #[inline(always)]
    #[must_use]
    pub fn mclkcfg(&mut self) -> MCLKCFG_W<29> {
        MCLKCFG_W::new(self)
    }
    #[doc = "Bits 30:31 - Shift Clock Output Configuration"]
    #[inline(always)]
    #[must_use]
    pub fn sclkcfg(&mut self) -> SCLKCFG_W<30> {
        SCLKCFG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Baud Rate Generator Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [brg](index.html) module"]
pub struct BRG_SPEC;
impl crate::RegisterSpec for BRG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [brg::R](R) reader structure"]
impl crate::Readable for BRG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [brg::W](W) writer structure"]
impl crate::Writable for BRG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BRG to value 0"]
impl crate::Resettable for BRG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
