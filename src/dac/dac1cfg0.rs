#[doc = "Register `DAC1CFG0` reader"]
pub type R = crate::R<DAC1CFG0_SPEC>;
#[doc = "Register `DAC1CFG0` writer"]
pub type W = crate::W<DAC1CFG0_SPEC>;
#[doc = "Field `FREQ` reader - Integer Frequency Divider Value"]
pub type FREQ_R = crate::FieldReader<u32>;
#[doc = "Field `FREQ` writer - Integer Frequency Divider Value"]
pub type FREQ_W<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
#[doc = "Field `MODE` reader - Enables and sets the Mode for DAC1"]
pub type MODE_R = crate::FieldReader<MODE_A>;
#[doc = "Enables and sets the Mode for DAC1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MODE_A {
    #[doc = "0: disable/switch-off DAC"]
    VALUE1 = 0,
    #[doc = "1: Single Value Mode"]
    VALUE2 = 1,
    #[doc = "2: Data Mode"]
    VALUE3 = 2,
    #[doc = "3: Patgen Mode"]
    VALUE4 = 3,
    #[doc = "4: Noise Mode"]
    VALUE5 = 4,
    #[doc = "5: Ramp Mode"]
    VALUE6 = 5,
    #[doc = "6: na"]
    VALUE7 = 6,
    #[doc = "7: na"]
    VALUE8 = 7,
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
    pub const fn variant(&self) -> MODE_A {
        match self.bits {
            0 => MODE_A::VALUE1,
            1 => MODE_A::VALUE2,
            2 => MODE_A::VALUE3,
            3 => MODE_A::VALUE4,
            4 => MODE_A::VALUE5,
            5 => MODE_A::VALUE6,
            6 => MODE_A::VALUE7,
            7 => MODE_A::VALUE8,
            _ => unreachable!(),
        }
    }
    #[doc = "disable/switch-off DAC"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == MODE_A::VALUE1
    }
    #[doc = "Single Value Mode"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == MODE_A::VALUE2
    }
    #[doc = "Data Mode"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == MODE_A::VALUE3
    }
    #[doc = "Patgen Mode"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == MODE_A::VALUE4
    }
    #[doc = "Noise Mode"]
    #[inline(always)]
    pub fn is_value5(&self) -> bool {
        *self == MODE_A::VALUE5
    }
    #[doc = "Ramp Mode"]
    #[inline(always)]
    pub fn is_value6(&self) -> bool {
        *self == MODE_A::VALUE6
    }
    #[doc = "na"]
    #[inline(always)]
    pub fn is_value7(&self) -> bool {
        *self == MODE_A::VALUE7
    }
    #[doc = "na"]
    #[inline(always)]
    pub fn is_value8(&self) -> bool {
        *self == MODE_A::VALUE8
    }
}
#[doc = "Field `MODE` writer - Enables and sets the Mode for DAC1"]
pub type MODE_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 3, MODE_A>;
impl<'a, REG> MODE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "disable/switch-off DAC"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(MODE_A::VALUE1)
    }
    #[doc = "Single Value Mode"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(MODE_A::VALUE2)
    }
    #[doc = "Data Mode"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(MODE_A::VALUE3)
    }
    #[doc = "Patgen Mode"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(MODE_A::VALUE4)
    }
    #[doc = "Noise Mode"]
    #[inline(always)]
    pub fn value5(self) -> &'a mut crate::W<REG> {
        self.variant(MODE_A::VALUE5)
    }
    #[doc = "Ramp Mode"]
    #[inline(always)]
    pub fn value6(self) -> &'a mut crate::W<REG> {
        self.variant(MODE_A::VALUE6)
    }
    #[doc = "na"]
    #[inline(always)]
    pub fn value7(self) -> &'a mut crate::W<REG> {
        self.variant(MODE_A::VALUE7)
    }
    #[doc = "na"]
    #[inline(always)]
    pub fn value8(self) -> &'a mut crate::W<REG> {
        self.variant(MODE_A::VALUE8)
    }
}
#[doc = "Field `SIGN` reader - Selects between signed and unsigned DAC1 mode"]
pub type SIGN_R = crate::BitReader<SIGN_A>;
#[doc = "Selects between signed and unsigned DAC1 mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SIGN_A {
    #[doc = "0: DAC expects unsigned input data"]
    VALUE1 = 0,
    #[doc = "1: DAC expects signed input data"]
    VALUE2 = 1,
}
impl From<SIGN_A> for bool {
    #[inline(always)]
    fn from(variant: SIGN_A) -> Self {
        variant as u8 != 0
    }
}
impl SIGN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SIGN_A {
        match self.bits {
            false => SIGN_A::VALUE1,
            true => SIGN_A::VALUE2,
        }
    }
    #[doc = "DAC expects unsigned input data"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SIGN_A::VALUE1
    }
    #[doc = "DAC expects signed input data"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SIGN_A::VALUE2
    }
}
#[doc = "Field `SIGN` writer - Selects between signed and unsigned DAC1 mode"]
pub type SIGN_W<'a, REG> = crate::BitWriter<'a, REG, SIGN_A>;
impl<'a, REG> SIGN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DAC expects unsigned input data"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(SIGN_A::VALUE1)
    }
    #[doc = "DAC expects signed input data"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(SIGN_A::VALUE2)
    }
}
#[doc = "Field `FIFOIND` reader - Current write position inside the data FIFO"]
pub type FIFOIND_R = crate::FieldReader;
#[doc = "Field `FIFOEMP` reader - Indicate if the FIFO is empty"]
pub type FIFOEMP_R = crate::BitReader<FIFOEMP_A>;
#[doc = "Indicate if the FIFO is empty\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FIFOEMP_A {
    #[doc = "0: FIFO not empty"]
    VALUE1 = 0,
    #[doc = "1: FIFO empty"]
    VALUE2 = 1,
}
impl From<FIFOEMP_A> for bool {
    #[inline(always)]
    fn from(variant: FIFOEMP_A) -> Self {
        variant as u8 != 0
    }
}
impl FIFOEMP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FIFOEMP_A {
        match self.bits {
            false => FIFOEMP_A::VALUE1,
            true => FIFOEMP_A::VALUE2,
        }
    }
    #[doc = "FIFO not empty"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == FIFOEMP_A::VALUE1
    }
    #[doc = "FIFO empty"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == FIFOEMP_A::VALUE2
    }
}
#[doc = "Field `FIFOFUL` reader - Indicate if the FIFO is full"]
pub type FIFOFUL_R = crate::BitReader<FIFOFUL_A>;
#[doc = "Indicate if the FIFO is full\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FIFOFUL_A {
    #[doc = "0: FIFO not full"]
    VALUE1 = 0,
    #[doc = "1: FIFO full"]
    VALUE2 = 1,
}
impl From<FIFOFUL_A> for bool {
    #[inline(always)]
    fn from(variant: FIFOFUL_A) -> Self {
        variant as u8 != 0
    }
}
impl FIFOFUL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FIFOFUL_A {
        match self.bits {
            false => FIFOFUL_A::VALUE1,
            true => FIFOFUL_A::VALUE2,
        }
    }
    #[doc = "FIFO not full"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == FIFOFUL_A::VALUE1
    }
    #[doc = "FIFO full"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == FIFOFUL_A::VALUE2
    }
}
#[doc = "Field `NEGATE` reader - Negates the DAC1 output"]
pub type NEGATE_R = crate::BitReader<NEGATE_A>;
#[doc = "Negates the DAC1 output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NEGATE_A {
    #[doc = "0: DAC output not negated"]
    VALUE1 = 0,
    #[doc = "1: DAC output negated"]
    VALUE2 = 1,
}
impl From<NEGATE_A> for bool {
    #[inline(always)]
    fn from(variant: NEGATE_A) -> Self {
        variant as u8 != 0
    }
}
impl NEGATE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> NEGATE_A {
        match self.bits {
            false => NEGATE_A::VALUE1,
            true => NEGATE_A::VALUE2,
        }
    }
    #[doc = "DAC output not negated"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == NEGATE_A::VALUE1
    }
    #[doc = "DAC output negated"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == NEGATE_A::VALUE2
    }
}
#[doc = "Field `NEGATE` writer - Negates the DAC1 output"]
pub type NEGATE_W<'a, REG> = crate::BitWriter<'a, REG, NEGATE_A>;
impl<'a, REG> NEGATE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DAC output not negated"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(NEGATE_A::VALUE1)
    }
    #[doc = "DAC output negated"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(NEGATE_A::VALUE2)
    }
}
#[doc = "Field `SIGNEN` reader - Enable sign output of DAC1 pattern generator"]
pub type SIGNEN_R = crate::BitReader<SIGNEN_A>;
#[doc = "Enable sign output of DAC1 pattern generator\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SIGNEN_A {
    #[doc = "0: disable"]
    VALUE1 = 0,
    #[doc = "1: enable"]
    VALUE2 = 1,
}
impl From<SIGNEN_A> for bool {
    #[inline(always)]
    fn from(variant: SIGNEN_A) -> Self {
        variant as u8 != 0
    }
}
impl SIGNEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SIGNEN_A {
        match self.bits {
            false => SIGNEN_A::VALUE1,
            true => SIGNEN_A::VALUE2,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SIGNEN_A::VALUE1
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SIGNEN_A::VALUE2
    }
}
#[doc = "Field `SIGNEN` writer - Enable sign output of DAC1 pattern generator"]
pub type SIGNEN_W<'a, REG> = crate::BitWriter<'a, REG, SIGNEN_A>;
impl<'a, REG> SIGNEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(SIGNEN_A::VALUE1)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(SIGNEN_A::VALUE2)
    }
}
#[doc = "Field `SREN` reader - Enable DAC1 service request interrupt generation"]
pub type SREN_R = crate::BitReader<SREN_A>;
#[doc = "Enable DAC1 service request interrupt generation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SREN_A {
    #[doc = "0: disable"]
    VALUE1 = 0,
    #[doc = "1: enable"]
    VALUE2 = 1,
}
impl From<SREN_A> for bool {
    #[inline(always)]
    fn from(variant: SREN_A) -> Self {
        variant as u8 != 0
    }
}
impl SREN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SREN_A {
        match self.bits {
            false => SREN_A::VALUE1,
            true => SREN_A::VALUE2,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SREN_A::VALUE1
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SREN_A::VALUE2
    }
}
#[doc = "Field `SREN` writer - Enable DAC1 service request interrupt generation"]
pub type SREN_W<'a, REG> = crate::BitWriter<'a, REG, SREN_A>;
impl<'a, REG> SREN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(SREN_A::VALUE1)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(SREN_A::VALUE2)
    }
}
#[doc = "Field `RUN` reader - RUN indicates the current DAC1 operation status"]
pub type RUN_R = crate::BitReader<RUN_A>;
#[doc = "RUN indicates the current DAC1 operation status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RUN_A {
    #[doc = "0: DAC1 channel disabled"]
    VALUE1 = 0,
    #[doc = "1: DAC1 channel in operation"]
    VALUE2 = 1,
}
impl From<RUN_A> for bool {
    #[inline(always)]
    fn from(variant: RUN_A) -> Self {
        variant as u8 != 0
    }
}
impl RUN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RUN_A {
        match self.bits {
            false => RUN_A::VALUE1,
            true => RUN_A::VALUE2,
        }
    }
    #[doc = "DAC1 channel disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == RUN_A::VALUE1
    }
    #[doc = "DAC1 channel in operation"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == RUN_A::VALUE2
    }
}
impl R {
    #[doc = "Bits 0:19 - Integer Frequency Divider Value"]
    #[inline(always)]
    pub fn freq(&self) -> FREQ_R {
        FREQ_R::new(self.bits & 0x000f_ffff)
    }
    #[doc = "Bits 20:22 - Enables and sets the Mode for DAC1"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bit 23 - Selects between signed and unsigned DAC1 mode"]
    #[inline(always)]
    pub fn sign(&self) -> SIGN_R {
        SIGN_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:25 - Current write position inside the data FIFO"]
    #[inline(always)]
    pub fn fifoind(&self) -> FIFOIND_R {
        FIFOIND_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bit 26 - Indicate if the FIFO is empty"]
    #[inline(always)]
    pub fn fifoemp(&self) -> FIFOEMP_R {
        FIFOEMP_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Indicate if the FIFO is full"]
    #[inline(always)]
    pub fn fifoful(&self) -> FIFOFUL_R {
        FIFOFUL_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Negates the DAC1 output"]
    #[inline(always)]
    pub fn negate(&self) -> NEGATE_R {
        NEGATE_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Enable sign output of DAC1 pattern generator"]
    #[inline(always)]
    pub fn signen(&self) -> SIGNEN_R {
        SIGNEN_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Enable DAC1 service request interrupt generation"]
    #[inline(always)]
    pub fn sren(&self) -> SREN_R {
        SREN_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - RUN indicates the current DAC1 operation status"]
    #[inline(always)]
    pub fn run(&self) -> RUN_R {
        RUN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:19 - Integer Frequency Divider Value"]
    #[inline(always)]
    #[must_use]
    pub fn freq(&mut self) -> FREQ_W<DAC1CFG0_SPEC> {
        FREQ_W::new(self, 0)
    }
    #[doc = "Bits 20:22 - Enables and sets the Mode for DAC1"]
    #[inline(always)]
    #[must_use]
    pub fn mode(&mut self) -> MODE_W<DAC1CFG0_SPEC> {
        MODE_W::new(self, 20)
    }
    #[doc = "Bit 23 - Selects between signed and unsigned DAC1 mode"]
    #[inline(always)]
    #[must_use]
    pub fn sign(&mut self) -> SIGN_W<DAC1CFG0_SPEC> {
        SIGN_W::new(self, 23)
    }
    #[doc = "Bit 28 - Negates the DAC1 output"]
    #[inline(always)]
    #[must_use]
    pub fn negate(&mut self) -> NEGATE_W<DAC1CFG0_SPEC> {
        NEGATE_W::new(self, 28)
    }
    #[doc = "Bit 29 - Enable sign output of DAC1 pattern generator"]
    #[inline(always)]
    #[must_use]
    pub fn signen(&mut self) -> SIGNEN_W<DAC1CFG0_SPEC> {
        SIGNEN_W::new(self, 29)
    }
    #[doc = "Bit 30 - Enable DAC1 service request interrupt generation"]
    #[inline(always)]
    #[must_use]
    pub fn sren(&mut self) -> SREN_W<DAC1CFG0_SPEC> {
        SREN_W::new(self, 30)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "DAC1 Configuration Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dac1cfg0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dac1cfg0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DAC1CFG0_SPEC;
impl crate::RegisterSpec for DAC1CFG0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dac1cfg0::R`](R) reader structure"]
impl crate::Readable for DAC1CFG0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dac1cfg0::W`](W) writer structure"]
impl crate::Writable for DAC1CFG0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DAC1CFG0 to value 0"]
impl crate::Resettable for DAC1CFG0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
