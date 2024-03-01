#[doc = "Register `ARBCFG` reader"]
pub type R = crate::R<ArbcfgSpec>;
#[doc = "Register `ARBCFG` writer"]
pub type W = crate::W<ArbcfgSpec>;
#[doc = "Field `ANONC` reader - Analog Converter Control"]
pub type AnoncR = crate::FieldReader;
#[doc = "Field `ANONC` writer - Analog Converter Control"]
pub type AnoncW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Arbitration Round Length\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Arbrnd {
    #[doc = "0: 4 arbitration slots per round (tARB = 4 / fADCD)"]
    Value1 = 0,
    #[doc = "1: 8 arbitration slots per round (tARB = 8 / fADCD)"]
    Value2 = 1,
    #[doc = "2: 16 arbitration slots per round (tARB = 16 / fADCD)"]
    Value3 = 2,
    #[doc = "3: 20 arbitration slots per round (tARB = 20 / fADCD)"]
    Value4 = 3,
}
impl From<Arbrnd> for u8 {
    #[inline(always)]
    fn from(variant: Arbrnd) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Arbrnd {
    type Ux = u8;
}
#[doc = "Field `ARBRND` reader - Arbitration Round Length"]
pub type ArbrndR = crate::FieldReader<Arbrnd>;
impl ArbrndR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Arbrnd {
        match self.bits {
            0 => Arbrnd::Value1,
            1 => Arbrnd::Value2,
            2 => Arbrnd::Value3,
            3 => Arbrnd::Value4,
            _ => unreachable!(),
        }
    }
    #[doc = "4 arbitration slots per round (tARB = 4 / fADCD)"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Arbrnd::Value1
    }
    #[doc = "8 arbitration slots per round (tARB = 8 / fADCD)"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Arbrnd::Value2
    }
    #[doc = "16 arbitration slots per round (tARB = 16 / fADCD)"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == Arbrnd::Value3
    }
    #[doc = "20 arbitration slots per round (tARB = 20 / fADCD)"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == Arbrnd::Value4
    }
}
#[doc = "Field `ARBRND` writer - Arbitration Round Length"]
pub type ArbrndW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Arbrnd>;
impl<'a, REG> ArbrndW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "4 arbitration slots per round (tARB = 4 / fADCD)"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Arbrnd::Value1)
    }
    #[doc = "8 arbitration slots per round (tARB = 8 / fADCD)"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Arbrnd::Value2)
    }
    #[doc = "16 arbitration slots per round (tARB = 16 / fADCD)"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(Arbrnd::Value3)
    }
    #[doc = "20 arbitration slots per round (tARB = 20 / fADCD)"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(Arbrnd::Value4)
    }
}
#[doc = "Arbitration Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Arbm {
    #[doc = "0: The arbiter runs permanently. This setting is required for a synchronization slave (see ) and for equidistant sampling using the signal ARBCNT (see )."]
    Value1 = 0,
    #[doc = "1: The arbiter only runs if at least one conversion request of an enabled request source is pending. This setting ensures a reproducible latency from an incoming request to the conversion start, if the converter is idle. Synchronized conversions are not supported."]
    Value2 = 1,
}
impl From<Arbm> for bool {
    #[inline(always)]
    fn from(variant: Arbm) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ARBM` reader - Arbitration Mode"]
pub type ArbmR = crate::BitReader<Arbm>;
impl ArbmR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Arbm {
        match self.bits {
            false => Arbm::Value1,
            true => Arbm::Value2,
        }
    }
    #[doc = "The arbiter runs permanently. This setting is required for a synchronization slave (see ) and for equidistant sampling using the signal ARBCNT (see )."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Arbm::Value1
    }
    #[doc = "The arbiter only runs if at least one conversion request of an enabled request source is pending. This setting ensures a reproducible latency from an incoming request to the conversion start, if the converter is idle. Synchronized conversions are not supported."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Arbm::Value2
    }
}
#[doc = "Field `ARBM` writer - Arbitration Mode"]
pub type ArbmW<'a, REG> = crate::BitWriter<'a, REG, Arbm>;
impl<'a, REG> ArbmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The arbiter runs permanently. This setting is required for a synchronization slave (see ) and for equidistant sampling using the signal ARBCNT (see )."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Arbm::Value1)
    }
    #[doc = "The arbiter only runs if at least one conversion request of an enabled request source is pending. This setting ensures a reproducible latency from an incoming request to the conversion start, if the converter is idle. Synchronized conversions are not supported."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Arbm::Value2)
    }
}
#[doc = "Analog Converter Control Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Anons {
    #[doc = "0: Analog converter off"]
    Value1 = 0,
    #[doc = "3: Normal operation (permanently on)"]
    Value4 = 3,
}
impl From<Anons> for u8 {
    #[inline(always)]
    fn from(variant: Anons) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Anons {
    type Ux = u8;
}
#[doc = "Field `ANONS` reader - Analog Converter Control Status"]
pub type AnonsR = crate::FieldReader<Anons>;
impl AnonsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Anons> {
        match self.bits {
            0 => Some(Anons::Value1),
            3 => Some(Anons::Value4),
            _ => None,
        }
    }
    #[doc = "Analog converter off"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Anons::Value1
    }
    #[doc = "Normal operation (permanently on)"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == Anons::Value4
    }
}
#[doc = "Start-Up Calibration Active Indication\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cal {
    #[doc = "0: Completed or not yet started"]
    Value1 = 0,
    #[doc = "1: Start-up calibration phase is active"]
    Value2 = 1,
}
impl From<Cal> for bool {
    #[inline(always)]
    fn from(variant: Cal) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CAL` reader - Start-Up Calibration Active Indication"]
pub type CalR = crate::BitReader<Cal>;
impl CalR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cal {
        match self.bits {
            false => Cal::Value1,
            true => Cal::Value2,
        }
    }
    #[doc = "Completed or not yet started"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Cal::Value1
    }
    #[doc = "Start-up calibration phase is active"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Cal::Value2
    }
}
#[doc = "Converter Busy Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Busy {
    #[doc = "0: Not busy"]
    Value1 = 0,
    #[doc = "1: Converter is busy with a conversion"]
    Value2 = 1,
}
impl From<Busy> for bool {
    #[inline(always)]
    fn from(variant: Busy) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BUSY` reader - Converter Busy Flag"]
pub type BusyR = crate::BitReader<Busy>;
impl BusyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Busy {
        match self.bits {
            false => Busy::Value1,
            true => Busy::Value2,
        }
    }
    #[doc = "Not busy"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Busy::Value1
    }
    #[doc = "Converter is busy with a conversion"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Busy::Value2
    }
}
#[doc = "Sample Phase Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sample {
    #[doc = "0: Converting or idle"]
    Value1 = 0,
    #[doc = "1: Input signal is currently sampled"]
    Value2 = 1,
}
impl From<Sample> for bool {
    #[inline(always)]
    fn from(variant: Sample) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SAMPLE` reader - Sample Phase Flag"]
pub type SampleR = crate::BitReader<Sample>;
impl SampleR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sample {
        match self.bits {
            false => Sample::Value1,
            true => Sample::Value2,
        }
    }
    #[doc = "Converting or idle"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Sample::Value1
    }
    #[doc = "Input signal is currently sampled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Sample::Value2
    }
}
impl R {
    #[doc = "Bits 0:1 - Analog Converter Control"]
    #[inline(always)]
    pub fn anonc(&self) -> AnoncR {
        AnoncR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 4:5 - Arbitration Round Length"]
    #[inline(always)]
    pub fn arbrnd(&self) -> ArbrndR {
        ArbrndR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 7 - Arbitration Mode"]
    #[inline(always)]
    pub fn arbm(&self) -> ArbmR {
        ArbmR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 16:17 - Analog Converter Control Status"]
    #[inline(always)]
    pub fn anons(&self) -> AnonsR {
        AnonsR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 28 - Start-Up Calibration Active Indication"]
    #[inline(always)]
    pub fn cal(&self) -> CalR {
        CalR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 30 - Converter Busy Flag"]
    #[inline(always)]
    pub fn busy(&self) -> BusyR {
        BusyR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Sample Phase Flag"]
    #[inline(always)]
    pub fn sample(&self) -> SampleR {
        SampleR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Analog Converter Control"]
    #[inline(always)]
    #[must_use]
    pub fn anonc(&mut self) -> AnoncW<ArbcfgSpec> {
        AnoncW::new(self, 0)
    }
    #[doc = "Bits 4:5 - Arbitration Round Length"]
    #[inline(always)]
    #[must_use]
    pub fn arbrnd(&mut self) -> ArbrndW<ArbcfgSpec> {
        ArbrndW::new(self, 4)
    }
    #[doc = "Bit 7 - Arbitration Mode"]
    #[inline(always)]
    #[must_use]
    pub fn arbm(&mut self) -> ArbmW<ArbcfgSpec> {
        ArbmW::new(self, 7)
    }
}
#[doc = "Arbitration Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`arbcfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`arbcfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ArbcfgSpec;
impl crate::RegisterSpec for ArbcfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`arbcfg::R`](R) reader structure"]
impl crate::Readable for ArbcfgSpec {}
#[doc = "`write(|w| ..)` method takes [`arbcfg::W`](W) writer structure"]
impl crate::Writable for ArbcfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ARBCFG to value 0"]
impl crate::Resettable for ArbcfgSpec {
    const RESET_VALUE: u32 = 0;
}
