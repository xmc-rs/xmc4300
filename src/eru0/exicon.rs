#[doc = "Register `EXICON[%s]` reader"]
pub type R = crate::R<ExiconSpec>;
#[doc = "Register `EXICON[%s]` writer"]
pub type W = crate::W<ExiconSpec>;
#[doc = "Output Trigger Pulse Enable for ETLx\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pe {
    #[doc = "0: The trigger pulse generation is disabled"]
    Value1 = 0,
    #[doc = "1: The trigger pulse generation is enabled"]
    Value2 = 1,
}
impl From<Pe> for bool {
    #[inline(always)]
    fn from(variant: Pe) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PE` reader - Output Trigger Pulse Enable for ETLx"]
pub type PeR = crate::BitReader<Pe>;
impl PeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pe {
        match self.bits {
            false => Pe::Value1,
            true => Pe::Value2,
        }
    }
    #[doc = "The trigger pulse generation is disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Pe::Value1
    }
    #[doc = "The trigger pulse generation is enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Pe::Value2
    }
}
#[doc = "Field `PE` writer - Output Trigger Pulse Enable for ETLx"]
pub type PeW<'a, REG> = crate::BitWriter<'a, REG, Pe>;
impl<'a, REG> PeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The trigger pulse generation is disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Pe::Value1)
    }
    #[doc = "The trigger pulse generation is enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Pe::Value2)
    }
}
#[doc = "Rebuild Level Detection for Status Flag for ETLx\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ld {
    #[doc = "0: The status flag FL is not cleared by hardware and is used as \"sticky\" bit. Once set, it is not influenced by any edge until it becomes cleared by software."]
    Value1 = 0,
    #[doc = "1: The status flag FL rebuilds a level detection of the desired event. It becomes automatically set with a rising edge if RE = 1 or with a falling edge if FE = 1. It becomes automatically cleared with a rising edge if RE = 0 or with a falling edge if FE = 0."]
    Value2 = 1,
}
impl From<Ld> for bool {
    #[inline(always)]
    fn from(variant: Ld) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LD` reader - Rebuild Level Detection for Status Flag for ETLx"]
pub type LdR = crate::BitReader<Ld>;
impl LdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ld {
        match self.bits {
            false => Ld::Value1,
            true => Ld::Value2,
        }
    }
    #[doc = "The status flag FL is not cleared by hardware and is used as \"sticky\" bit. Once set, it is not influenced by any edge until it becomes cleared by software."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Ld::Value1
    }
    #[doc = "The status flag FL rebuilds a level detection of the desired event. It becomes automatically set with a rising edge if RE = 1 or with a falling edge if FE = 1. It becomes automatically cleared with a rising edge if RE = 0 or with a falling edge if FE = 0."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Ld::Value2
    }
}
#[doc = "Field `LD` writer - Rebuild Level Detection for Status Flag for ETLx"]
pub type LdW<'a, REG> = crate::BitWriter<'a, REG, Ld>;
impl<'a, REG> LdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The status flag FL is not cleared by hardware and is used as \"sticky\" bit. Once set, it is not influenced by any edge until it becomes cleared by software."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Ld::Value1)
    }
    #[doc = "The status flag FL rebuilds a level detection of the desired event. It becomes automatically set with a rising edge if RE = 1 or with a falling edge if FE = 1. It becomes automatically cleared with a rising edge if RE = 0 or with a falling edge if FE = 0."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Ld::Value2)
    }
}
#[doc = "Rising Edge Detection Enable ETLx\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Re {
    #[doc = "0: A rising edge is not considered as edge event"]
    Value1 = 0,
    #[doc = "1: A rising edge is considered as edge event"]
    Value2 = 1,
}
impl From<Re> for bool {
    #[inline(always)]
    fn from(variant: Re) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RE` reader - Rising Edge Detection Enable ETLx"]
pub type ReR = crate::BitReader<Re>;
impl ReR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Re {
        match self.bits {
            false => Re::Value1,
            true => Re::Value2,
        }
    }
    #[doc = "A rising edge is not considered as edge event"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Re::Value1
    }
    #[doc = "A rising edge is considered as edge event"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Re::Value2
    }
}
#[doc = "Field `RE` writer - Rising Edge Detection Enable ETLx"]
pub type ReW<'a, REG> = crate::BitWriter<'a, REG, Re>;
impl<'a, REG> ReW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "A rising edge is not considered as edge event"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Re::Value1)
    }
    #[doc = "A rising edge is considered as edge event"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Re::Value2)
    }
}
#[doc = "Falling Edge Detection Enable ETLx\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fe {
    #[doc = "0: A falling edge is not considered as edge event"]
    Value1 = 0,
    #[doc = "1: A falling edge is considered as edge event"]
    Value2 = 1,
}
impl From<Fe> for bool {
    #[inline(always)]
    fn from(variant: Fe) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FE` reader - Falling Edge Detection Enable ETLx"]
pub type FeR = crate::BitReader<Fe>;
impl FeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fe {
        match self.bits {
            false => Fe::Value1,
            true => Fe::Value2,
        }
    }
    #[doc = "A falling edge is not considered as edge event"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Fe::Value1
    }
    #[doc = "A falling edge is considered as edge event"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Fe::Value2
    }
}
#[doc = "Field `FE` writer - Falling Edge Detection Enable ETLx"]
pub type FeW<'a, REG> = crate::BitWriter<'a, REG, Fe>;
impl<'a, REG> FeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "A falling edge is not considered as edge event"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Fe::Value1)
    }
    #[doc = "A falling edge is considered as edge event"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Fe::Value2)
    }
}
#[doc = "Output Channel Select for ETLx Output Trigger Pulse\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ocs {
    #[doc = "0: Trigger pulses are sent to OGU0"]
    Value1 = 0,
    #[doc = "1: Trigger pulses are sent to OGU1"]
    Value2 = 1,
    #[doc = "2: Trigger pulses are sent to OGU2"]
    Value3 = 2,
    #[doc = "3: Trigger pulses are sent to OGU3"]
    Value4 = 3,
}
impl From<Ocs> for u8 {
    #[inline(always)]
    fn from(variant: Ocs) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ocs {
    type Ux = u8;
}
impl crate::IsEnum for Ocs {}
#[doc = "Field `OCS` reader - Output Channel Select for ETLx Output Trigger Pulse"]
pub type OcsR = crate::FieldReader<Ocs>;
impl OcsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Ocs> {
        match self.bits {
            0 => Some(Ocs::Value1),
            1 => Some(Ocs::Value2),
            2 => Some(Ocs::Value3),
            3 => Some(Ocs::Value4),
            _ => None,
        }
    }
    #[doc = "Trigger pulses are sent to OGU0"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Ocs::Value1
    }
    #[doc = "Trigger pulses are sent to OGU1"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Ocs::Value2
    }
    #[doc = "Trigger pulses are sent to OGU2"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == Ocs::Value3
    }
    #[doc = "Trigger pulses are sent to OGU3"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == Ocs::Value4
    }
}
#[doc = "Field `OCS` writer - Output Channel Select for ETLx Output Trigger Pulse"]
pub type OcsW<'a, REG> = crate::FieldWriter<'a, REG, 3, Ocs>;
impl<'a, REG> OcsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Trigger pulses are sent to OGU0"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Ocs::Value1)
    }
    #[doc = "Trigger pulses are sent to OGU1"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Ocs::Value2)
    }
    #[doc = "Trigger pulses are sent to OGU2"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(Ocs::Value3)
    }
    #[doc = "Trigger pulses are sent to OGU3"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(Ocs::Value4)
    }
}
#[doc = "Status Flag for ETLx\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fl {
    #[doc = "0: The enabled edge event has not been detected"]
    Value1 = 0,
    #[doc = "1: The enabled edge event has been detected"]
    Value2 = 1,
}
impl From<Fl> for bool {
    #[inline(always)]
    fn from(variant: Fl) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FL` reader - Status Flag for ETLx"]
pub type FlR = crate::BitReader<Fl>;
impl FlR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fl {
        match self.bits {
            false => Fl::Value1,
            true => Fl::Value2,
        }
    }
    #[doc = "The enabled edge event has not been detected"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Fl::Value1
    }
    #[doc = "The enabled edge event has been detected"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Fl::Value2
    }
}
#[doc = "Field `FL` writer - Status Flag for ETLx"]
pub type FlW<'a, REG> = crate::BitWriter<'a, REG, Fl>;
impl<'a, REG> FlW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The enabled edge event has not been detected"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Fl::Value1)
    }
    #[doc = "The enabled edge event has been detected"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Fl::Value2)
    }
}
#[doc = "Input Source Select for ERSx\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ss {
    #[doc = "0: Input A without additional combination"]
    Value1 = 0,
    #[doc = "1: Input B without additional combination"]
    Value2 = 1,
    #[doc = "2: Input A OR input B"]
    Value3 = 2,
    #[doc = "3: Input A AND input B"]
    Value4 = 3,
}
impl From<Ss> for u8 {
    #[inline(always)]
    fn from(variant: Ss) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ss {
    type Ux = u8;
}
impl crate::IsEnum for Ss {}
#[doc = "Field `SS` reader - Input Source Select for ERSx"]
pub type SsR = crate::FieldReader<Ss>;
impl SsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ss {
        match self.bits {
            0 => Ss::Value1,
            1 => Ss::Value2,
            2 => Ss::Value3,
            3 => Ss::Value4,
            _ => unreachable!(),
        }
    }
    #[doc = "Input A without additional combination"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Ss::Value1
    }
    #[doc = "Input B without additional combination"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Ss::Value2
    }
    #[doc = "Input A OR input B"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == Ss::Value3
    }
    #[doc = "Input A AND input B"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == Ss::Value4
    }
}
#[doc = "Field `SS` writer - Input Source Select for ERSx"]
pub type SsW<'a, REG> = crate::FieldWriter<'a, REG, 2, Ss, crate::Safe>;
impl<'a, REG> SsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Input A without additional combination"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Ss::Value1)
    }
    #[doc = "Input B without additional combination"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Ss::Value2)
    }
    #[doc = "Input A OR input B"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(Ss::Value3)
    }
    #[doc = "Input A AND input B"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(Ss::Value4)
    }
}
#[doc = "Input A Negation Select for ERSx\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Na {
    #[doc = "0: Input A is used directly"]
    Value1 = 0,
    #[doc = "1: Input A is inverted"]
    Value2 = 1,
}
impl From<Na> for bool {
    #[inline(always)]
    fn from(variant: Na) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NA` reader - Input A Negation Select for ERSx"]
pub type NaR = crate::BitReader<Na>;
impl NaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Na {
        match self.bits {
            false => Na::Value1,
            true => Na::Value2,
        }
    }
    #[doc = "Input A is used directly"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Na::Value1
    }
    #[doc = "Input A is inverted"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Na::Value2
    }
}
#[doc = "Field `NA` writer - Input A Negation Select for ERSx"]
pub type NaW<'a, REG> = crate::BitWriter<'a, REG, Na>;
impl<'a, REG> NaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Input A is used directly"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Na::Value1)
    }
    #[doc = "Input A is inverted"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Na::Value2)
    }
}
#[doc = "Input B Negation Select for ERSx\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Nb {
    #[doc = "0: Input B is used directly"]
    Value1 = 0,
    #[doc = "1: Input B is inverted"]
    Value2 = 1,
}
impl From<Nb> for bool {
    #[inline(always)]
    fn from(variant: Nb) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NB` reader - Input B Negation Select for ERSx"]
pub type NbR = crate::BitReader<Nb>;
impl NbR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Nb {
        match self.bits {
            false => Nb::Value1,
            true => Nb::Value2,
        }
    }
    #[doc = "Input B is used directly"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Nb::Value1
    }
    #[doc = "Input B is inverted"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Nb::Value2
    }
}
#[doc = "Field `NB` writer - Input B Negation Select for ERSx"]
pub type NbW<'a, REG> = crate::BitWriter<'a, REG, Nb>;
impl<'a, REG> NbW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Input B is used directly"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Nb::Value1)
    }
    #[doc = "Input B is inverted"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Nb::Value2)
    }
}
impl R {
    #[doc = "Bit 0 - Output Trigger Pulse Enable for ETLx"]
    #[inline(always)]
    pub fn pe(&self) -> PeR {
        PeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Rebuild Level Detection for Status Flag for ETLx"]
    #[inline(always)]
    pub fn ld(&self) -> LdR {
        LdR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Rising Edge Detection Enable ETLx"]
    #[inline(always)]
    pub fn re(&self) -> ReR {
        ReR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Falling Edge Detection Enable ETLx"]
    #[inline(always)]
    pub fn fe(&self) -> FeR {
        FeR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - Output Channel Select for ETLx Output Trigger Pulse"]
    #[inline(always)]
    pub fn ocs(&self) -> OcsR {
        OcsR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - Status Flag for ETLx"]
    #[inline(always)]
    pub fn fl(&self) -> FlR {
        FlR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - Input Source Select for ERSx"]
    #[inline(always)]
    pub fn ss(&self) -> SsR {
        SsR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 10 - Input A Negation Select for ERSx"]
    #[inline(always)]
    pub fn na(&self) -> NaR {
        NaR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Input B Negation Select for ERSx"]
    #[inline(always)]
    pub fn nb(&self) -> NbR {
        NbR::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Output Trigger Pulse Enable for ETLx"]
    #[inline(always)]
    #[must_use]
    pub fn pe(&mut self) -> PeW<ExiconSpec> {
        PeW::new(self, 0)
    }
    #[doc = "Bit 1 - Rebuild Level Detection for Status Flag for ETLx"]
    #[inline(always)]
    #[must_use]
    pub fn ld(&mut self) -> LdW<ExiconSpec> {
        LdW::new(self, 1)
    }
    #[doc = "Bit 2 - Rising Edge Detection Enable ETLx"]
    #[inline(always)]
    #[must_use]
    pub fn re(&mut self) -> ReW<ExiconSpec> {
        ReW::new(self, 2)
    }
    #[doc = "Bit 3 - Falling Edge Detection Enable ETLx"]
    #[inline(always)]
    #[must_use]
    pub fn fe(&mut self) -> FeW<ExiconSpec> {
        FeW::new(self, 3)
    }
    #[doc = "Bits 4:6 - Output Channel Select for ETLx Output Trigger Pulse"]
    #[inline(always)]
    #[must_use]
    pub fn ocs(&mut self) -> OcsW<ExiconSpec> {
        OcsW::new(self, 4)
    }
    #[doc = "Bit 7 - Status Flag for ETLx"]
    #[inline(always)]
    #[must_use]
    pub fn fl(&mut self) -> FlW<ExiconSpec> {
        FlW::new(self, 7)
    }
    #[doc = "Bits 8:9 - Input Source Select for ERSx"]
    #[inline(always)]
    #[must_use]
    pub fn ss(&mut self) -> SsW<ExiconSpec> {
        SsW::new(self, 8)
    }
    #[doc = "Bit 10 - Input A Negation Select for ERSx"]
    #[inline(always)]
    #[must_use]
    pub fn na(&mut self) -> NaW<ExiconSpec> {
        NaW::new(self, 10)
    }
    #[doc = "Bit 11 - Input B Negation Select for ERSx"]
    #[inline(always)]
    #[must_use]
    pub fn nb(&mut self) -> NbW<ExiconSpec> {
        NbW::new(self, 11)
    }
}
#[doc = "Event Input Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`exicon::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`exicon::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ExiconSpec;
impl crate::RegisterSpec for ExiconSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`exicon::R`](R) reader structure"]
impl crate::Readable for ExiconSpec {}
#[doc = "`write(|w| ..)` method takes [`exicon::W`](W) writer structure"]
impl crate::Writable for ExiconSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EXICON[%s]
to value 0"]
impl crate::Resettable for ExiconSpec {
    const RESET_VALUE: u32 = 0;
}
