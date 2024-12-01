#[doc = "Register `BRG` reader"]
pub type R = crate::R<BRG_SPEC>;
#[doc = "Register `BRG` writer"]
pub type W = crate::W<BRG_SPEC>;
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
impl crate::FieldSpec for CLKSEL_A {
    type Ux = u8;
}
impl crate::IsEnum for CLKSEL_A {}
#[doc = "Field `CLKSEL` reader - Clock Selection"]
pub type CLKSEL_R = crate::FieldReader<CLKSEL_A>;
impl CLKSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<CLKSEL_A> {
        match self.bits {
            0 => Some(CLKSEL_A::VALUE1),
            2 => Some(CLKSEL_A::VALUE3),
            3 => Some(CLKSEL_A::VALUE4),
            _ => None,
        }
    }
    #[doc = "The fractional divider frequency fFD is selected."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CLKSEL_A::VALUE1
    }
    #[doc = "The trigger signal DX1T defines fPIN. Signal MCLK toggles with fPIN."]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == CLKSEL_A::VALUE3
    }
    #[doc = "Signal MCLK corresponds to the DX1S signal and the frequency fPIN is derived from the rising edges of DX1S."]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == CLKSEL_A::VALUE4
    }
}
#[doc = "Field `CLKSEL` writer - Clock Selection"]
pub type CLKSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2, CLKSEL_A>;
impl<'a, REG> CLKSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "The fractional divider frequency fFD is selected."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(CLKSEL_A::VALUE1)
    }
    #[doc = "The trigger signal DX1T defines fPIN. Signal MCLK toggles with fPIN."]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(CLKSEL_A::VALUE3)
    }
    #[doc = "Signal MCLK corresponds to the DX1S signal and the frequency fPIN is derived from the rising edges of DX1S."]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(CLKSEL_A::VALUE4)
    }
}
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
#[doc = "Field `TMEN` reader - Timing Measurement Enable"]
pub type TMEN_R = crate::BitReader<TMEN_A>;
impl TMEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TMEN_A {
        match self.bits {
            false => TMEN_A::VALUE1,
            true => TMEN_A::VALUE2,
        }
    }
    #[doc = "Timing measurement is disabled: The trigger signals DX0T and DX1T are ignored."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == TMEN_A::VALUE1
    }
    #[doc = "Timing measurement is enabled: The 10-bit counter is incremented by 1 with fPPP and stops counting when reaching its maximum value. If one of the trigger signals DX0T or DX1T become active, the counter value is captured into bit field CTV, the counter is cleared and a transmit shift event is generated."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == TMEN_A::VALUE2
    }
}
#[doc = "Field `TMEN` writer - Timing Measurement Enable"]
pub type TMEN_W<'a, REG> = crate::BitWriter<'a, REG, TMEN_A>;
impl<'a, REG> TMEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Timing measurement is disabled: The trigger signals DX0T and DX1T are ignored."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(TMEN_A::VALUE1)
    }
    #[doc = "Timing measurement is enabled: The 10-bit counter is incremented by 1 with fPPP and stops counting when reaching its maximum value. If one of the trigger signals DX0T or DX1T become active, the counter value is captured into bit field CTV, the counter is cleared and a transmit shift event is generated."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(TMEN_A::VALUE2)
    }
}
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
#[doc = "Field `PPPEN` reader - Enable 2:1 Divider for fPPP"]
pub type PPPEN_R = crate::BitReader<PPPEN_A>;
impl PPPEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PPPEN_A {
        match self.bits {
            false => PPPEN_A::VALUE1,
            true => PPPEN_A::VALUE2,
        }
    }
    #[doc = "The 2:1 divider for fPPP is disabled. fPPP = fPIN"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PPPEN_A::VALUE1
    }
    #[doc = "The 2:1 divider for fPPP is enabled. fPPP = fMCLK = fPIN / 2."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PPPEN_A::VALUE2
    }
}
#[doc = "Field `PPPEN` writer - Enable 2:1 Divider for fPPP"]
pub type PPPEN_W<'a, REG> = crate::BitWriter<'a, REG, PPPEN_A>;
impl<'a, REG> PPPEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The 2:1 divider for fPPP is disabled. fPPP = fPIN"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(PPPEN_A::VALUE1)
    }
    #[doc = "The 2:1 divider for fPPP is enabled. fPPP = fMCLK = fPIN / 2."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(PPPEN_A::VALUE2)
    }
}
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
impl crate::FieldSpec for CTQSEL_A {
    type Ux = u8;
}
impl crate::IsEnum for CTQSEL_A {}
#[doc = "Field `CTQSEL` reader - Input Selection for CTQ"]
pub type CTQSEL_R = crate::FieldReader<CTQSEL_A>;
impl CTQSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CTQSEL_A {
        match self.bits {
            0 => CTQSEL_A::VALUE1,
            1 => CTQSEL_A::VALUE2,
            2 => CTQSEL_A::VALUE3,
            3 => CTQSEL_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "fCTQIN = fPDIV"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CTQSEL_A::VALUE1
    }
    #[doc = "fCTQIN = fPPP"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CTQSEL_A::VALUE2
    }
    #[doc = "fCTQIN = fSCLK"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == CTQSEL_A::VALUE3
    }
    #[doc = "fCTQIN = fMCLK"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == CTQSEL_A::VALUE4
    }
}
#[doc = "Field `CTQSEL` writer - Input Selection for CTQ"]
pub type CTQSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2, CTQSEL_A, crate::Safe>;
impl<'a, REG> CTQSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "fCTQIN = fPDIV"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(CTQSEL_A::VALUE1)
    }
    #[doc = "fCTQIN = fPPP"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(CTQSEL_A::VALUE2)
    }
    #[doc = "fCTQIN = fSCLK"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(CTQSEL_A::VALUE3)
    }
    #[doc = "fCTQIN = fMCLK"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(CTQSEL_A::VALUE4)
    }
}
#[doc = "Field `PCTQ` reader - Pre-Divider for Time Quanta Counter"]
pub type PCTQ_R = crate::FieldReader;
#[doc = "Field `PCTQ` writer - Pre-Divider for Time Quanta Counter"]
pub type PCTQ_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DCTQ` reader - Denominator for Time Quanta Counter"]
pub type DCTQ_R = crate::FieldReader;
#[doc = "Field `DCTQ` writer - Denominator for Time Quanta Counter"]
pub type DCTQ_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PDIV` reader - Divider Mode: Divider Factor to Generate fPDIV"]
pub type PDIV_R = crate::FieldReader<u16>;
#[doc = "Field `PDIV` writer - Divider Mode: Divider Factor to Generate fPDIV"]
pub type PDIV_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
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
#[doc = "Field `SCLKOSEL` reader - Shift Clock Output Select"]
pub type SCLKOSEL_R = crate::BitReader<SCLKOSEL_A>;
impl SCLKOSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SCLKOSEL_A {
        match self.bits {
            false => SCLKOSEL_A::VALUE1,
            true => SCLKOSEL_A::VALUE2,
        }
    }
    #[doc = "SCLK from the baud rate generator is selected as the SCLKOUT input source."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SCLKOSEL_A::VALUE1
    }
    #[doc = "The transmit shift clock from DX1 input stage is selected as the SCLKOUT input source."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SCLKOSEL_A::VALUE2
    }
}
#[doc = "Field `SCLKOSEL` writer - Shift Clock Output Select"]
pub type SCLKOSEL_W<'a, REG> = crate::BitWriter<'a, REG, SCLKOSEL_A>;
impl<'a, REG> SCLKOSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SCLK from the baud rate generator is selected as the SCLKOUT input source."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(SCLKOSEL_A::VALUE1)
    }
    #[doc = "The transmit shift clock from DX1 input stage is selected as the SCLKOUT input source."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(SCLKOSEL_A::VALUE2)
    }
}
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
#[doc = "Field `MCLKCFG` reader - Master Clock Configuration"]
pub type MCLKCFG_R = crate::BitReader<MCLKCFG_A>;
impl MCLKCFG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MCLKCFG_A {
        match self.bits {
            false => MCLKCFG_A::VALUE1,
            true => MCLKCFG_A::VALUE2,
        }
    }
    #[doc = "The passive level is 0."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == MCLKCFG_A::VALUE1
    }
    #[doc = "The passive level is 1."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == MCLKCFG_A::VALUE2
    }
}
#[doc = "Field `MCLKCFG` writer - Master Clock Configuration"]
pub type MCLKCFG_W<'a, REG> = crate::BitWriter<'a, REG, MCLKCFG_A>;
impl<'a, REG> MCLKCFG_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The passive level is 0."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(MCLKCFG_A::VALUE1)
    }
    #[doc = "The passive level is 1."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(MCLKCFG_A::VALUE2)
    }
}
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
impl crate::FieldSpec for SCLKCFG_A {
    type Ux = u8;
}
impl crate::IsEnum for SCLKCFG_A {}
#[doc = "Field `SCLKCFG` reader - Shift Clock Output Configuration"]
pub type SCLKCFG_R = crate::FieldReader<SCLKCFG_A>;
impl SCLKCFG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SCLKCFG_A {
        match self.bits {
            0 => SCLKCFG_A::VALUE1,
            1 => SCLKCFG_A::VALUE2,
            2 => SCLKCFG_A::VALUE3,
            3 => SCLKCFG_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "The passive level is 0 and the delay is disabled."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SCLKCFG_A::VALUE1
    }
    #[doc = "The passive level is 1 and the delay is disabled."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SCLKCFG_A::VALUE2
    }
    #[doc = "The passive level is 0 and the delay is enabled."]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == SCLKCFG_A::VALUE3
    }
    #[doc = "The passive level is 1 and the delay is enabled."]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == SCLKCFG_A::VALUE4
    }
}
#[doc = "Field `SCLKCFG` writer - Shift Clock Output Configuration"]
pub type SCLKCFG_W<'a, REG> = crate::FieldWriter<'a, REG, 2, SCLKCFG_A, crate::Safe>;
impl<'a, REG> SCLKCFG_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "The passive level is 0 and the delay is disabled."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(SCLKCFG_A::VALUE1)
    }
    #[doc = "The passive level is 1 and the delay is disabled."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(SCLKCFG_A::VALUE2)
    }
    #[doc = "The passive level is 0 and the delay is enabled."]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(SCLKCFG_A::VALUE3)
    }
    #[doc = "The passive level is 1 and the delay is enabled."]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
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
    pub fn clksel(&mut self) -> CLKSEL_W<BRG_SPEC> {
        CLKSEL_W::new(self, 0)
    }
    #[doc = "Bit 3 - Timing Measurement Enable"]
    #[inline(always)]
    pub fn tmen(&mut self) -> TMEN_W<BRG_SPEC> {
        TMEN_W::new(self, 3)
    }
    #[doc = "Bit 4 - Enable 2:1 Divider for fPPP"]
    #[inline(always)]
    pub fn pppen(&mut self) -> PPPEN_W<BRG_SPEC> {
        PPPEN_W::new(self, 4)
    }
    #[doc = "Bits 6:7 - Input Selection for CTQ"]
    #[inline(always)]
    pub fn ctqsel(&mut self) -> CTQSEL_W<BRG_SPEC> {
        CTQSEL_W::new(self, 6)
    }
    #[doc = "Bits 8:9 - Pre-Divider for Time Quanta Counter"]
    #[inline(always)]
    pub fn pctq(&mut self) -> PCTQ_W<BRG_SPEC> {
        PCTQ_W::new(self, 8)
    }
    #[doc = "Bits 10:14 - Denominator for Time Quanta Counter"]
    #[inline(always)]
    pub fn dctq(&mut self) -> DCTQ_W<BRG_SPEC> {
        DCTQ_W::new(self, 10)
    }
    #[doc = "Bits 16:25 - Divider Mode: Divider Factor to Generate fPDIV"]
    #[inline(always)]
    pub fn pdiv(&mut self) -> PDIV_W<BRG_SPEC> {
        PDIV_W::new(self, 16)
    }
    #[doc = "Bit 28 - Shift Clock Output Select"]
    #[inline(always)]
    pub fn sclkosel(&mut self) -> SCLKOSEL_W<BRG_SPEC> {
        SCLKOSEL_W::new(self, 28)
    }
    #[doc = "Bit 29 - Master Clock Configuration"]
    #[inline(always)]
    pub fn mclkcfg(&mut self) -> MCLKCFG_W<BRG_SPEC> {
        MCLKCFG_W::new(self, 29)
    }
    #[doc = "Bits 30:31 - Shift Clock Output Configuration"]
    #[inline(always)]
    pub fn sclkcfg(&mut self) -> SCLKCFG_W<BRG_SPEC> {
        SCLKCFG_W::new(self, 30)
    }
}
#[doc = "Baud Rate Generator Register\n\nYou can [`read`](crate::Reg::read) this register and get [`brg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`brg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BRG_SPEC;
impl crate::RegisterSpec for BRG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`brg::R`](R) reader structure"]
impl crate::Readable for BRG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`brg::W`](W) writer structure"]
impl crate::Writable for BRG_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BRG to value 0"]
impl crate::Resettable for BRG_SPEC {
    const RESET_VALUE: u32 = 0;
}
