#[doc = "Register `BRG` reader"]
pub type R = crate::R<BrgSpec>;
#[doc = "Register `BRG` writer"]
pub type W = crate::W<BrgSpec>;
#[doc = "Clock Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Clksel {
    #[doc = "0: The fractional divider frequency fFD is selected."]
    Value1 = 0,
    #[doc = "2: The trigger signal DX1T defines fPIN. Signal MCLK toggles with fPIN."]
    Value3 = 2,
    #[doc = "3: Signal MCLK corresponds to the DX1S signal and the frequency fPIN is derived from the rising edges of DX1S."]
    Value4 = 3,
}
impl From<Clksel> for u8 {
    #[inline(always)]
    fn from(variant: Clksel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Clksel {
    type Ux = u8;
}
impl crate::IsEnum for Clksel {}
#[doc = "Field `CLKSEL` reader - Clock Selection"]
pub type ClkselR = crate::FieldReader<Clksel>;
impl ClkselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Clksel> {
        match self.bits {
            0 => Some(Clksel::Value1),
            2 => Some(Clksel::Value3),
            3 => Some(Clksel::Value4),
            _ => None,
        }
    }
    #[doc = "The fractional divider frequency fFD is selected."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Clksel::Value1
    }
    #[doc = "The trigger signal DX1T defines fPIN. Signal MCLK toggles with fPIN."]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == Clksel::Value3
    }
    #[doc = "Signal MCLK corresponds to the DX1S signal and the frequency fPIN is derived from the rising edges of DX1S."]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == Clksel::Value4
    }
}
#[doc = "Field `CLKSEL` writer - Clock Selection"]
pub type ClkselW<'a, REG> = crate::FieldWriter<'a, REG, 2, Clksel>;
impl<'a, REG> ClkselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "The fractional divider frequency fFD is selected."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Clksel::Value1)
    }
    #[doc = "The trigger signal DX1T defines fPIN. Signal MCLK toggles with fPIN."]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(Clksel::Value3)
    }
    #[doc = "Signal MCLK corresponds to the DX1S signal and the frequency fPIN is derived from the rising edges of DX1S."]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(Clksel::Value4)
    }
}
#[doc = "Timing Measurement Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tmen {
    #[doc = "0: Timing measurement is disabled: The trigger signals DX0T and DX1T are ignored."]
    Value1 = 0,
    #[doc = "1: Timing measurement is enabled: The 10-bit counter is incremented by 1 with fPPP and stops counting when reaching its maximum value. If one of the trigger signals DX0T or DX1T become active, the counter value is captured into bit field CTV, the counter is cleared and a transmit shift event is generated."]
    Value2 = 1,
}
impl From<Tmen> for bool {
    #[inline(always)]
    fn from(variant: Tmen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TMEN` reader - Timing Measurement Enable"]
pub type TmenR = crate::BitReader<Tmen>;
impl TmenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tmen {
        match self.bits {
            false => Tmen::Value1,
            true => Tmen::Value2,
        }
    }
    #[doc = "Timing measurement is disabled: The trigger signals DX0T and DX1T are ignored."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Tmen::Value1
    }
    #[doc = "Timing measurement is enabled: The 10-bit counter is incremented by 1 with fPPP and stops counting when reaching its maximum value. If one of the trigger signals DX0T or DX1T become active, the counter value is captured into bit field CTV, the counter is cleared and a transmit shift event is generated."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Tmen::Value2
    }
}
#[doc = "Field `TMEN` writer - Timing Measurement Enable"]
pub type TmenW<'a, REG> = crate::BitWriter<'a, REG, Tmen>;
impl<'a, REG> TmenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Timing measurement is disabled: The trigger signals DX0T and DX1T are ignored."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Tmen::Value1)
    }
    #[doc = "Timing measurement is enabled: The 10-bit counter is incremented by 1 with fPPP and stops counting when reaching its maximum value. If one of the trigger signals DX0T or DX1T become active, the counter value is captured into bit field CTV, the counter is cleared and a transmit shift event is generated."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Tmen::Value2)
    }
}
#[doc = "Enable 2:1 Divider for fPPP\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pppen {
    #[doc = "0: The 2:1 divider for fPPP is disabled. fPPP = fPIN"]
    Value1 = 0,
    #[doc = "1: The 2:1 divider for fPPP is enabled. fPPP = fMCLK = fPIN / 2."]
    Value2 = 1,
}
impl From<Pppen> for bool {
    #[inline(always)]
    fn from(variant: Pppen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PPPEN` reader - Enable 2:1 Divider for fPPP"]
pub type PppenR = crate::BitReader<Pppen>;
impl PppenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pppen {
        match self.bits {
            false => Pppen::Value1,
            true => Pppen::Value2,
        }
    }
    #[doc = "The 2:1 divider for fPPP is disabled. fPPP = fPIN"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Pppen::Value1
    }
    #[doc = "The 2:1 divider for fPPP is enabled. fPPP = fMCLK = fPIN / 2."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Pppen::Value2
    }
}
#[doc = "Field `PPPEN` writer - Enable 2:1 Divider for fPPP"]
pub type PppenW<'a, REG> = crate::BitWriter<'a, REG, Pppen>;
impl<'a, REG> PppenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The 2:1 divider for fPPP is disabled. fPPP = fPIN"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Pppen::Value1)
    }
    #[doc = "The 2:1 divider for fPPP is enabled. fPPP = fMCLK = fPIN / 2."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Pppen::Value2)
    }
}
#[doc = "Input Selection for CTQ\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ctqsel {
    #[doc = "0: fCTQIN = fPDIV"]
    Value1 = 0,
    #[doc = "1: fCTQIN = fPPP"]
    Value2 = 1,
    #[doc = "2: fCTQIN = fSCLK"]
    Value3 = 2,
    #[doc = "3: fCTQIN = fMCLK"]
    Value4 = 3,
}
impl From<Ctqsel> for u8 {
    #[inline(always)]
    fn from(variant: Ctqsel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ctqsel {
    type Ux = u8;
}
impl crate::IsEnum for Ctqsel {}
#[doc = "Field `CTQSEL` reader - Input Selection for CTQ"]
pub type CtqselR = crate::FieldReader<Ctqsel>;
impl CtqselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ctqsel {
        match self.bits {
            0 => Ctqsel::Value1,
            1 => Ctqsel::Value2,
            2 => Ctqsel::Value3,
            3 => Ctqsel::Value4,
            _ => unreachable!(),
        }
    }
    #[doc = "fCTQIN = fPDIV"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Ctqsel::Value1
    }
    #[doc = "fCTQIN = fPPP"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Ctqsel::Value2
    }
    #[doc = "fCTQIN = fSCLK"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == Ctqsel::Value3
    }
    #[doc = "fCTQIN = fMCLK"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == Ctqsel::Value4
    }
}
#[doc = "Field `CTQSEL` writer - Input Selection for CTQ"]
pub type CtqselW<'a, REG> = crate::FieldWriter<'a, REG, 2, Ctqsel, crate::Safe>;
impl<'a, REG> CtqselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "fCTQIN = fPDIV"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Ctqsel::Value1)
    }
    #[doc = "fCTQIN = fPPP"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Ctqsel::Value2)
    }
    #[doc = "fCTQIN = fSCLK"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(Ctqsel::Value3)
    }
    #[doc = "fCTQIN = fMCLK"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(Ctqsel::Value4)
    }
}
#[doc = "Field `PCTQ` reader - Pre-Divider for Time Quanta Counter"]
pub type PctqR = crate::FieldReader;
#[doc = "Field `PCTQ` writer - Pre-Divider for Time Quanta Counter"]
pub type PctqW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DCTQ` reader - Denominator for Time Quanta Counter"]
pub type DctqR = crate::FieldReader;
#[doc = "Field `DCTQ` writer - Denominator for Time Quanta Counter"]
pub type DctqW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PDIV` reader - Divider Mode: Divider Factor to Generate fPDIV"]
pub type PdivR = crate::FieldReader<u16>;
#[doc = "Field `PDIV` writer - Divider Mode: Divider Factor to Generate fPDIV"]
pub type PdivW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Shift Clock Output Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sclkosel {
    #[doc = "0: SCLK from the baud rate generator is selected as the SCLKOUT input source."]
    Value1 = 0,
    #[doc = "1: The transmit shift clock from DX1 input stage is selected as the SCLKOUT input source."]
    Value2 = 1,
}
impl From<Sclkosel> for bool {
    #[inline(always)]
    fn from(variant: Sclkosel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SCLKOSEL` reader - Shift Clock Output Select"]
pub type SclkoselR = crate::BitReader<Sclkosel>;
impl SclkoselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sclkosel {
        match self.bits {
            false => Sclkosel::Value1,
            true => Sclkosel::Value2,
        }
    }
    #[doc = "SCLK from the baud rate generator is selected as the SCLKOUT input source."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Sclkosel::Value1
    }
    #[doc = "The transmit shift clock from DX1 input stage is selected as the SCLKOUT input source."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Sclkosel::Value2
    }
}
#[doc = "Field `SCLKOSEL` writer - Shift Clock Output Select"]
pub type SclkoselW<'a, REG> = crate::BitWriter<'a, REG, Sclkosel>;
impl<'a, REG> SclkoselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SCLK from the baud rate generator is selected as the SCLKOUT input source."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Sclkosel::Value1)
    }
    #[doc = "The transmit shift clock from DX1 input stage is selected as the SCLKOUT input source."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Sclkosel::Value2)
    }
}
#[doc = "Master Clock Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mclkcfg {
    #[doc = "0: The passive level is 0."]
    Value1 = 0,
    #[doc = "1: The passive level is 1."]
    Value2 = 1,
}
impl From<Mclkcfg> for bool {
    #[inline(always)]
    fn from(variant: Mclkcfg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MCLKCFG` reader - Master Clock Configuration"]
pub type MclkcfgR = crate::BitReader<Mclkcfg>;
impl MclkcfgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mclkcfg {
        match self.bits {
            false => Mclkcfg::Value1,
            true => Mclkcfg::Value2,
        }
    }
    #[doc = "The passive level is 0."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Mclkcfg::Value1
    }
    #[doc = "The passive level is 1."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Mclkcfg::Value2
    }
}
#[doc = "Field `MCLKCFG` writer - Master Clock Configuration"]
pub type MclkcfgW<'a, REG> = crate::BitWriter<'a, REG, Mclkcfg>;
impl<'a, REG> MclkcfgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The passive level is 0."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Mclkcfg::Value1)
    }
    #[doc = "The passive level is 1."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Mclkcfg::Value2)
    }
}
#[doc = "Shift Clock Output Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Sclkcfg {
    #[doc = "0: The passive level is 0 and the delay is disabled."]
    Value1 = 0,
    #[doc = "1: The passive level is 1 and the delay is disabled."]
    Value2 = 1,
    #[doc = "2: The passive level is 0 and the delay is enabled."]
    Value3 = 2,
    #[doc = "3: The passive level is 1 and the delay is enabled."]
    Value4 = 3,
}
impl From<Sclkcfg> for u8 {
    #[inline(always)]
    fn from(variant: Sclkcfg) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Sclkcfg {
    type Ux = u8;
}
impl crate::IsEnum for Sclkcfg {}
#[doc = "Field `SCLKCFG` reader - Shift Clock Output Configuration"]
pub type SclkcfgR = crate::FieldReader<Sclkcfg>;
impl SclkcfgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sclkcfg {
        match self.bits {
            0 => Sclkcfg::Value1,
            1 => Sclkcfg::Value2,
            2 => Sclkcfg::Value3,
            3 => Sclkcfg::Value4,
            _ => unreachable!(),
        }
    }
    #[doc = "The passive level is 0 and the delay is disabled."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Sclkcfg::Value1
    }
    #[doc = "The passive level is 1 and the delay is disabled."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Sclkcfg::Value2
    }
    #[doc = "The passive level is 0 and the delay is enabled."]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == Sclkcfg::Value3
    }
    #[doc = "The passive level is 1 and the delay is enabled."]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == Sclkcfg::Value4
    }
}
#[doc = "Field `SCLKCFG` writer - Shift Clock Output Configuration"]
pub type SclkcfgW<'a, REG> = crate::FieldWriter<'a, REG, 2, Sclkcfg, crate::Safe>;
impl<'a, REG> SclkcfgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "The passive level is 0 and the delay is disabled."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Sclkcfg::Value1)
    }
    #[doc = "The passive level is 1 and the delay is disabled."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Sclkcfg::Value2)
    }
    #[doc = "The passive level is 0 and the delay is enabled."]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(Sclkcfg::Value3)
    }
    #[doc = "The passive level is 1 and the delay is enabled."]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(Sclkcfg::Value4)
    }
}
impl R {
    #[doc = "Bits 0:1 - Clock Selection"]
    #[inline(always)]
    pub fn clksel(&self) -> ClkselR {
        ClkselR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 3 - Timing Measurement Enable"]
    #[inline(always)]
    pub fn tmen(&self) -> TmenR {
        TmenR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable 2:1 Divider for fPPP"]
    #[inline(always)]
    pub fn pppen(&self) -> PppenR {
        PppenR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 6:7 - Input Selection for CTQ"]
    #[inline(always)]
    pub fn ctqsel(&self) -> CtqselR {
        CtqselR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Pre-Divider for Time Quanta Counter"]
    #[inline(always)]
    pub fn pctq(&self) -> PctqR {
        PctqR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:14 - Denominator for Time Quanta Counter"]
    #[inline(always)]
    pub fn dctq(&self) -> DctqR {
        DctqR::new(((self.bits >> 10) & 0x1f) as u8)
    }
    #[doc = "Bits 16:25 - Divider Mode: Divider Factor to Generate fPDIV"]
    #[inline(always)]
    pub fn pdiv(&self) -> PdivR {
        PdivR::new(((self.bits >> 16) & 0x03ff) as u16)
    }
    #[doc = "Bit 28 - Shift Clock Output Select"]
    #[inline(always)]
    pub fn sclkosel(&self) -> SclkoselR {
        SclkoselR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Master Clock Configuration"]
    #[inline(always)]
    pub fn mclkcfg(&self) -> MclkcfgR {
        MclkcfgR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bits 30:31 - Shift Clock Output Configuration"]
    #[inline(always)]
    pub fn sclkcfg(&self) -> SclkcfgR {
        SclkcfgR::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Clock Selection"]
    #[inline(always)]
    #[must_use]
    pub fn clksel(&mut self) -> ClkselW<BrgSpec> {
        ClkselW::new(self, 0)
    }
    #[doc = "Bit 3 - Timing Measurement Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tmen(&mut self) -> TmenW<BrgSpec> {
        TmenW::new(self, 3)
    }
    #[doc = "Bit 4 - Enable 2:1 Divider for fPPP"]
    #[inline(always)]
    #[must_use]
    pub fn pppen(&mut self) -> PppenW<BrgSpec> {
        PppenW::new(self, 4)
    }
    #[doc = "Bits 6:7 - Input Selection for CTQ"]
    #[inline(always)]
    #[must_use]
    pub fn ctqsel(&mut self) -> CtqselW<BrgSpec> {
        CtqselW::new(self, 6)
    }
    #[doc = "Bits 8:9 - Pre-Divider for Time Quanta Counter"]
    #[inline(always)]
    #[must_use]
    pub fn pctq(&mut self) -> PctqW<BrgSpec> {
        PctqW::new(self, 8)
    }
    #[doc = "Bits 10:14 - Denominator for Time Quanta Counter"]
    #[inline(always)]
    #[must_use]
    pub fn dctq(&mut self) -> DctqW<BrgSpec> {
        DctqW::new(self, 10)
    }
    #[doc = "Bits 16:25 - Divider Mode: Divider Factor to Generate fPDIV"]
    #[inline(always)]
    #[must_use]
    pub fn pdiv(&mut self) -> PdivW<BrgSpec> {
        PdivW::new(self, 16)
    }
    #[doc = "Bit 28 - Shift Clock Output Select"]
    #[inline(always)]
    #[must_use]
    pub fn sclkosel(&mut self) -> SclkoselW<BrgSpec> {
        SclkoselW::new(self, 28)
    }
    #[doc = "Bit 29 - Master Clock Configuration"]
    #[inline(always)]
    #[must_use]
    pub fn mclkcfg(&mut self) -> MclkcfgW<BrgSpec> {
        MclkcfgW::new(self, 29)
    }
    #[doc = "Bits 30:31 - Shift Clock Output Configuration"]
    #[inline(always)]
    #[must_use]
    pub fn sclkcfg(&mut self) -> SclkcfgW<BrgSpec> {
        SclkcfgW::new(self, 30)
    }
}
#[doc = "Baud Rate Generator Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`brg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`brg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BrgSpec;
impl crate::RegisterSpec for BrgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`brg::R`](R) reader structure"]
impl crate::Readable for BrgSpec {}
#[doc = "`write(|w| ..)` method takes [`brg::W`](W) writer structure"]
impl crate::Writable for BrgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BRG to value 0"]
impl crate::Resettable for BrgSpec {
    const RESET_VALUE: u32 = 0;
}
