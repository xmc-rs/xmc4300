#[doc = "Register `DAC0CFG0` reader"]
pub type R = crate::R<Dac0cfg0Spec>;
#[doc = "Register `DAC0CFG0` writer"]
pub type W = crate::W<Dac0cfg0Spec>;
#[doc = "Field `FREQ` reader - Integer Frequency Divider Value"]
pub type FreqR = crate::FieldReader<u32>;
#[doc = "Field `FREQ` writer - Integer Frequency Divider Value"]
pub type FreqW<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
#[doc = "Enables and Sets the Mode for DAC0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Mode {
    #[doc = "0: disable/switch-off DAC"]
    Value1 = 0,
    #[doc = "1: Single Value Mode"]
    Value2 = 1,
    #[doc = "2: Data Mode"]
    Value3 = 2,
    #[doc = "3: Patgen Mode"]
    Value4 = 3,
    #[doc = "4: Noise Mode"]
    Value5 = 4,
    #[doc = "5: Ramp Mode"]
    Value6 = 5,
    #[doc = "6: na"]
    Value7 = 6,
    #[doc = "7: na"]
    Value8 = 7,
}
impl From<Mode> for u8 {
    #[inline(always)]
    fn from(variant: Mode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Mode {
    type Ux = u8;
}
impl crate::IsEnum for Mode {}
#[doc = "Field `MODE` reader - Enables and Sets the Mode for DAC0"]
pub type ModeR = crate::FieldReader<Mode>;
impl ModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mode {
        match self.bits {
            0 => Mode::Value1,
            1 => Mode::Value2,
            2 => Mode::Value3,
            3 => Mode::Value4,
            4 => Mode::Value5,
            5 => Mode::Value6,
            6 => Mode::Value7,
            7 => Mode::Value8,
            _ => unreachable!(),
        }
    }
    #[doc = "disable/switch-off DAC"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Mode::Value1
    }
    #[doc = "Single Value Mode"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Mode::Value2
    }
    #[doc = "Data Mode"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == Mode::Value3
    }
    #[doc = "Patgen Mode"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == Mode::Value4
    }
    #[doc = "Noise Mode"]
    #[inline(always)]
    pub fn is_value5(&self) -> bool {
        *self == Mode::Value5
    }
    #[doc = "Ramp Mode"]
    #[inline(always)]
    pub fn is_value6(&self) -> bool {
        *self == Mode::Value6
    }
    #[doc = "na"]
    #[inline(always)]
    pub fn is_value7(&self) -> bool {
        *self == Mode::Value7
    }
    #[doc = "na"]
    #[inline(always)]
    pub fn is_value8(&self) -> bool {
        *self == Mode::Value8
    }
}
#[doc = "Field `MODE` writer - Enables and Sets the Mode for DAC0"]
pub type ModeW<'a, REG> = crate::FieldWriter<'a, REG, 3, Mode, crate::Safe>;
impl<'a, REG> ModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "disable/switch-off DAC"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::Value1)
    }
    #[doc = "Single Value Mode"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::Value2)
    }
    #[doc = "Data Mode"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::Value3)
    }
    #[doc = "Patgen Mode"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::Value4)
    }
    #[doc = "Noise Mode"]
    #[inline(always)]
    pub fn value5(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::Value5)
    }
    #[doc = "Ramp Mode"]
    #[inline(always)]
    pub fn value6(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::Value6)
    }
    #[doc = "na"]
    #[inline(always)]
    pub fn value7(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::Value7)
    }
    #[doc = "na"]
    #[inline(always)]
    pub fn value8(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::Value8)
    }
}
#[doc = "Selects Between Signed and Unsigned DAC0 Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sign {
    #[doc = "0: DAC expects unsigned input data"]
    Value1 = 0,
    #[doc = "1: DAC expects signed input data"]
    Value2 = 1,
}
impl From<Sign> for bool {
    #[inline(always)]
    fn from(variant: Sign) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SIGN` reader - Selects Between Signed and Unsigned DAC0 Mode"]
pub type SignR = crate::BitReader<Sign>;
impl SignR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sign {
        match self.bits {
            false => Sign::Value1,
            true => Sign::Value2,
        }
    }
    #[doc = "DAC expects unsigned input data"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Sign::Value1
    }
    #[doc = "DAC expects signed input data"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Sign::Value2
    }
}
#[doc = "Field `SIGN` writer - Selects Between Signed and Unsigned DAC0 Mode"]
pub type SignW<'a, REG> = crate::BitWriter<'a, REG, Sign>;
impl<'a, REG> SignW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DAC expects unsigned input data"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Sign::Value1)
    }
    #[doc = "DAC expects signed input data"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Sign::Value2)
    }
}
#[doc = "Field `FIFOIND` reader - Current write position inside the data FIFO"]
pub type FifoindR = crate::FieldReader;
#[doc = "Indicate if the FIFO is empty\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fifoemp {
    #[doc = "0: FIFO not empty"]
    Value1 = 0,
    #[doc = "1: FIFO empty"]
    Value2 = 1,
}
impl From<Fifoemp> for bool {
    #[inline(always)]
    fn from(variant: Fifoemp) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FIFOEMP` reader - Indicate if the FIFO is empty"]
pub type FifoempR = crate::BitReader<Fifoemp>;
impl FifoempR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fifoemp {
        match self.bits {
            false => Fifoemp::Value1,
            true => Fifoemp::Value2,
        }
    }
    #[doc = "FIFO not empty"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Fifoemp::Value1
    }
    #[doc = "FIFO empty"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Fifoemp::Value2
    }
}
#[doc = "Indicate if the FIFO is full\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fifoful {
    #[doc = "0: FIFO not full"]
    Value1 = 0,
    #[doc = "1: FIFO full"]
    Value2 = 1,
}
impl From<Fifoful> for bool {
    #[inline(always)]
    fn from(variant: Fifoful) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FIFOFUL` reader - Indicate if the FIFO is full"]
pub type FifofulR = crate::BitReader<Fifoful>;
impl FifofulR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fifoful {
        match self.bits {
            false => Fifoful::Value1,
            true => Fifoful::Value2,
        }
    }
    #[doc = "FIFO not full"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Fifoful::Value1
    }
    #[doc = "FIFO full"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Fifoful::Value2
    }
}
#[doc = "Negates the DAC0 output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Negate {
    #[doc = "0: DAC output not negated"]
    Value1 = 0,
    #[doc = "1: DAC output negated"]
    Value2 = 1,
}
impl From<Negate> for bool {
    #[inline(always)]
    fn from(variant: Negate) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NEGATE` reader - Negates the DAC0 output"]
pub type NegateR = crate::BitReader<Negate>;
impl NegateR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Negate {
        match self.bits {
            false => Negate::Value1,
            true => Negate::Value2,
        }
    }
    #[doc = "DAC output not negated"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Negate::Value1
    }
    #[doc = "DAC output negated"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Negate::Value2
    }
}
#[doc = "Field `NEGATE` writer - Negates the DAC0 output"]
pub type NegateW<'a, REG> = crate::BitWriter<'a, REG, Negate>;
impl<'a, REG> NegateW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DAC output not negated"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Negate::Value1)
    }
    #[doc = "DAC output negated"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Negate::Value2)
    }
}
#[doc = "Enable Sign Output of DAC0 Pattern Generator\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Signen {
    #[doc = "0: Disable"]
    Value1 = 0,
    #[doc = "1: Enable"]
    Value2 = 1,
}
impl From<Signen> for bool {
    #[inline(always)]
    fn from(variant: Signen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SIGNEN` reader - Enable Sign Output of DAC0 Pattern Generator"]
pub type SignenR = crate::BitReader<Signen>;
impl SignenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Signen {
        match self.bits {
            false => Signen::Value1,
            true => Signen::Value2,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Signen::Value1
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Signen::Value2
    }
}
#[doc = "Field `SIGNEN` writer - Enable Sign Output of DAC0 Pattern Generator"]
pub type SignenW<'a, REG> = crate::BitWriter<'a, REG, Signen>;
impl<'a, REG> SignenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Signen::Value1)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Signen::Value2)
    }
}
#[doc = "Enable DAC0 service request interrupt generation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sren {
    #[doc = "0: disable"]
    Value1 = 0,
    #[doc = "1: enable"]
    Value2 = 1,
}
impl From<Sren> for bool {
    #[inline(always)]
    fn from(variant: Sren) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SREN` reader - Enable DAC0 service request interrupt generation"]
pub type SrenR = crate::BitReader<Sren>;
impl SrenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sren {
        match self.bits {
            false => Sren::Value1,
            true => Sren::Value2,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Sren::Value1
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Sren::Value2
    }
}
#[doc = "Field `SREN` writer - Enable DAC0 service request interrupt generation"]
pub type SrenW<'a, REG> = crate::BitWriter<'a, REG, Sren>;
impl<'a, REG> SrenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Sren::Value1)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Sren::Value2)
    }
}
#[doc = "RUN indicates the current DAC0 operation status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Run {
    #[doc = "0: DAC0 channel disabled"]
    Value1 = 0,
    #[doc = "1: DAC0 channel in operation"]
    Value2 = 1,
}
impl From<Run> for bool {
    #[inline(always)]
    fn from(variant: Run) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RUN` reader - RUN indicates the current DAC0 operation status"]
pub type RunR = crate::BitReader<Run>;
impl RunR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Run {
        match self.bits {
            false => Run::Value1,
            true => Run::Value2,
        }
    }
    #[doc = "DAC0 channel disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Run::Value1
    }
    #[doc = "DAC0 channel in operation"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Run::Value2
    }
}
impl R {
    #[doc = "Bits 0:19 - Integer Frequency Divider Value"]
    #[inline(always)]
    pub fn freq(&self) -> FreqR {
        FreqR::new(self.bits & 0x000f_ffff)
    }
    #[doc = "Bits 20:22 - Enables and Sets the Mode for DAC0"]
    #[inline(always)]
    pub fn mode(&self) -> ModeR {
        ModeR::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bit 23 - Selects Between Signed and Unsigned DAC0 Mode"]
    #[inline(always)]
    pub fn sign(&self) -> SignR {
        SignR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:25 - Current write position inside the data FIFO"]
    #[inline(always)]
    pub fn fifoind(&self) -> FifoindR {
        FifoindR::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bit 26 - Indicate if the FIFO is empty"]
    #[inline(always)]
    pub fn fifoemp(&self) -> FifoempR {
        FifoempR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Indicate if the FIFO is full"]
    #[inline(always)]
    pub fn fifoful(&self) -> FifofulR {
        FifofulR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Negates the DAC0 output"]
    #[inline(always)]
    pub fn negate(&self) -> NegateR {
        NegateR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Enable Sign Output of DAC0 Pattern Generator"]
    #[inline(always)]
    pub fn signen(&self) -> SignenR {
        SignenR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Enable DAC0 service request interrupt generation"]
    #[inline(always)]
    pub fn sren(&self) -> SrenR {
        SrenR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - RUN indicates the current DAC0 operation status"]
    #[inline(always)]
    pub fn run(&self) -> RunR {
        RunR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:19 - Integer Frequency Divider Value"]
    #[inline(always)]
    #[must_use]
    pub fn freq(&mut self) -> FreqW<Dac0cfg0Spec> {
        FreqW::new(self, 0)
    }
    #[doc = "Bits 20:22 - Enables and Sets the Mode for DAC0"]
    #[inline(always)]
    #[must_use]
    pub fn mode(&mut self) -> ModeW<Dac0cfg0Spec> {
        ModeW::new(self, 20)
    }
    #[doc = "Bit 23 - Selects Between Signed and Unsigned DAC0 Mode"]
    #[inline(always)]
    #[must_use]
    pub fn sign(&mut self) -> SignW<Dac0cfg0Spec> {
        SignW::new(self, 23)
    }
    #[doc = "Bit 28 - Negates the DAC0 output"]
    #[inline(always)]
    #[must_use]
    pub fn negate(&mut self) -> NegateW<Dac0cfg0Spec> {
        NegateW::new(self, 28)
    }
    #[doc = "Bit 29 - Enable Sign Output of DAC0 Pattern Generator"]
    #[inline(always)]
    #[must_use]
    pub fn signen(&mut self) -> SignenW<Dac0cfg0Spec> {
        SignenW::new(self, 29)
    }
    #[doc = "Bit 30 - Enable DAC0 service request interrupt generation"]
    #[inline(always)]
    #[must_use]
    pub fn sren(&mut self) -> SrenW<Dac0cfg0Spec> {
        SrenW::new(self, 30)
    }
}
#[doc = "DAC0 Configuration Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dac0cfg0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dac0cfg0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dac0cfg0Spec;
impl crate::RegisterSpec for Dac0cfg0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dac0cfg0::R`](R) reader structure"]
impl crate::Readable for Dac0cfg0Spec {}
#[doc = "`write(|w| ..)` method takes [`dac0cfg0::W`](W) writer structure"]
impl crate::Writable for Dac0cfg0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DAC0CFG0 to value 0"]
impl crate::Resettable for Dac0cfg0Spec {
    const RESET_VALUE: u32 = 0;
}
