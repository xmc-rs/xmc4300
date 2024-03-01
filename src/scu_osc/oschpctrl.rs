#[doc = "Register `OSCHPCTRL` reader"]
pub type R = crate::R<OschpctrlSpec>;
#[doc = "Register `OSCHPCTRL` writer"]
pub type W = crate::W<OschpctrlSpec>;
#[doc = "XTAL1 Data Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum X1den {
    #[doc = "0: Bit X1D is not updated"]
    Const0 = 0,
    #[doc = "1: Bit X1D can be updated"]
    Const1 = 1,
}
impl From<X1den> for bool {
    #[inline(always)]
    fn from(variant: X1den) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `X1DEN` reader - XTAL1 Data Enable"]
pub type X1denR = crate::BitReader<X1den>;
impl X1denR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> X1den {
        match self.bits {
            false => X1den::Const0,
            true => X1den::Const1,
        }
    }
    #[doc = "Bit X1D is not updated"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == X1den::Const0
    }
    #[doc = "Bit X1D can be updated"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == X1den::Const1
    }
}
#[doc = "Field `X1DEN` writer - XTAL1 Data Enable"]
pub type X1denW<'a, REG> = crate::BitWriter<'a, REG, X1den>;
impl<'a, REG> X1denW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Bit X1D is not updated"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(X1den::Const0)
    }
    #[doc = "Bit X1D can be updated"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(X1den::Const1)
    }
}
#[doc = "Shaper Bypass\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Shby {
    #[doc = "0: The shaper is not bypassed"]
    Const0 = 0,
    #[doc = "1: The shaper is bypassed"]
    Const1 = 1,
}
impl From<Shby> for bool {
    #[inline(always)]
    fn from(variant: Shby) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SHBY` reader - Shaper Bypass"]
pub type ShbyR = crate::BitReader<Shby>;
impl ShbyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Shby {
        match self.bits {
            false => Shby::Const0,
            true => Shby::Const1,
        }
    }
    #[doc = "The shaper is not bypassed"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == Shby::Const0
    }
    #[doc = "The shaper is bypassed"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == Shby::Const1
    }
}
#[doc = "Field `SHBY` writer - Shaper Bypass"]
pub type ShbyW<'a, REG> = crate::BitWriter<'a, REG, Shby>;
impl<'a, REG> ShbyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The shaper is not bypassed"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(Shby::Const0)
    }
    #[doc = "The shaper is bypassed"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(Shby::Const1)
    }
}
#[doc = "Oscillator Gain Selection\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gainsel {
    #[doc = "0: The gain control is configured for frequencies from 4 MHz to 8 MHz"]
    Const00 = 0,
    #[doc = "1: The gain control is configured for frequencies from 4 MHz to 16 MHz"]
    Const01 = 1,
    #[doc = "2: The gain control is configured for frequencies from 4 MHz to 20 MHz"]
    Const10 = 2,
    #[doc = "3: The gain control is configured for frequencies from 4 MHz to 25 MHz"]
    Const11 = 3,
}
impl From<Gainsel> for u8 {
    #[inline(always)]
    fn from(variant: Gainsel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gainsel {
    type Ux = u8;
}
#[doc = "Field `GAINSEL` reader - Oscillator Gain Selection"]
pub type GainselR = crate::FieldReader<Gainsel>;
impl GainselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gainsel {
        match self.bits {
            0 => Gainsel::Const00,
            1 => Gainsel::Const01,
            2 => Gainsel::Const10,
            3 => Gainsel::Const11,
            _ => unreachable!(),
        }
    }
    #[doc = "The gain control is configured for frequencies from 4 MHz to 8 MHz"]
    #[inline(always)]
    pub fn is_const_00(&self) -> bool {
        *self == Gainsel::Const00
    }
    #[doc = "The gain control is configured for frequencies from 4 MHz to 16 MHz"]
    #[inline(always)]
    pub fn is_const_01(&self) -> bool {
        *self == Gainsel::Const01
    }
    #[doc = "The gain control is configured for frequencies from 4 MHz to 20 MHz"]
    #[inline(always)]
    pub fn is_const_10(&self) -> bool {
        *self == Gainsel::Const10
    }
    #[doc = "The gain control is configured for frequencies from 4 MHz to 25 MHz"]
    #[inline(always)]
    pub fn is_const_11(&self) -> bool {
        *self == Gainsel::Const11
    }
}
#[doc = "Field `GAINSEL` writer - Oscillator Gain Selection"]
pub type GainselW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Gainsel>;
impl<'a, REG> GainselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "The gain control is configured for frequencies from 4 MHz to 8 MHz"]
    #[inline(always)]
    pub fn const_00(self) -> &'a mut crate::W<REG> {
        self.variant(Gainsel::Const00)
    }
    #[doc = "The gain control is configured for frequencies from 4 MHz to 16 MHz"]
    #[inline(always)]
    pub fn const_01(self) -> &'a mut crate::W<REG> {
        self.variant(Gainsel::Const01)
    }
    #[doc = "The gain control is configured for frequencies from 4 MHz to 20 MHz"]
    #[inline(always)]
    pub fn const_10(self) -> &'a mut crate::W<REG> {
        self.variant(Gainsel::Const10)
    }
    #[doc = "The gain control is configured for frequencies from 4 MHz to 25 MHz"]
    #[inline(always)]
    pub fn const_11(self) -> &'a mut crate::W<REG> {
        self.variant(Gainsel::Const11)
    }
}
#[doc = "Oscillator Mode\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Mode {
    #[doc = "0: External Crystal Mode and External Input Clock Mode. The oscillator Power-Saving Mode is not entered."]
    Const00 = 0,
    #[doc = "1: OSC is disabled. The oscillator Power-Saving Mode is not entered."]
    Const01 = 1,
    #[doc = "2: External Input Clock Mode and the oscillator Power-Saving Mode is entered"]
    Const10 = 2,
    #[doc = "3: OSC is disabled. The oscillator Power-Saving Mode is entered."]
    Const11 = 3,
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
#[doc = "Field `MODE` reader - Oscillator Mode"]
pub type ModeR = crate::FieldReader<Mode>;
impl ModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mode {
        match self.bits {
            0 => Mode::Const00,
            1 => Mode::Const01,
            2 => Mode::Const10,
            3 => Mode::Const11,
            _ => unreachable!(),
        }
    }
    #[doc = "External Crystal Mode and External Input Clock Mode. The oscillator Power-Saving Mode is not entered."]
    #[inline(always)]
    pub fn is_const_00(&self) -> bool {
        *self == Mode::Const00
    }
    #[doc = "OSC is disabled. The oscillator Power-Saving Mode is not entered."]
    #[inline(always)]
    pub fn is_const_01(&self) -> bool {
        *self == Mode::Const01
    }
    #[doc = "External Input Clock Mode and the oscillator Power-Saving Mode is entered"]
    #[inline(always)]
    pub fn is_const_10(&self) -> bool {
        *self == Mode::Const10
    }
    #[doc = "OSC is disabled. The oscillator Power-Saving Mode is entered."]
    #[inline(always)]
    pub fn is_const_11(&self) -> bool {
        *self == Mode::Const11
    }
}
#[doc = "Field `MODE` writer - Oscillator Mode"]
pub type ModeW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Mode>;
impl<'a, REG> ModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "External Crystal Mode and External Input Clock Mode. The oscillator Power-Saving Mode is not entered."]
    #[inline(always)]
    pub fn const_00(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::Const00)
    }
    #[doc = "OSC is disabled. The oscillator Power-Saving Mode is not entered."]
    #[inline(always)]
    pub fn const_01(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::Const01)
    }
    #[doc = "External Input Clock Mode and the oscillator Power-Saving Mode is entered"]
    #[inline(always)]
    pub fn const_10(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::Const10)
    }
    #[doc = "OSC is disabled. The oscillator Power-Saving Mode is entered."]
    #[inline(always)]
    pub fn const_11(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::Const11)
    }
}
#[doc = "Field `OSCVAL` reader - OSC Frequency Value"]
pub type OscvalR = crate::FieldReader;
#[doc = "Field `OSCVAL` writer - OSC Frequency Value"]
pub type OscvalW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bit 0 - XTAL1 Data Enable"]
    #[inline(always)]
    pub fn x1den(&self) -> X1denR {
        X1denR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Shaper Bypass"]
    #[inline(always)]
    pub fn shby(&self) -> ShbyR {
        ShbyR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - Oscillator Gain Selection"]
    #[inline(always)]
    pub fn gainsel(&self) -> GainselR {
        GainselR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Oscillator Mode"]
    #[inline(always)]
    pub fn mode(&self) -> ModeR {
        ModeR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 16:19 - OSC Frequency Value"]
    #[inline(always)]
    pub fn oscval(&self) -> OscvalR {
        OscvalR::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - XTAL1 Data Enable"]
    #[inline(always)]
    #[must_use]
    pub fn x1den(&mut self) -> X1denW<OschpctrlSpec> {
        X1denW::new(self, 0)
    }
    #[doc = "Bit 1 - Shaper Bypass"]
    #[inline(always)]
    #[must_use]
    pub fn shby(&mut self) -> ShbyW<OschpctrlSpec> {
        ShbyW::new(self, 1)
    }
    #[doc = "Bits 2:3 - Oscillator Gain Selection"]
    #[inline(always)]
    #[must_use]
    pub fn gainsel(&mut self) -> GainselW<OschpctrlSpec> {
        GainselW::new(self, 2)
    }
    #[doc = "Bits 4:5 - Oscillator Mode"]
    #[inline(always)]
    #[must_use]
    pub fn mode(&mut self) -> ModeW<OschpctrlSpec> {
        ModeW::new(self, 4)
    }
    #[doc = "Bits 16:19 - OSC Frequency Value"]
    #[inline(always)]
    #[must_use]
    pub fn oscval(&mut self) -> OscvalW<OschpctrlSpec> {
        OscvalW::new(self, 16)
    }
}
#[doc = "OSC_HP Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`oschpctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`oschpctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OschpctrlSpec;
impl crate::RegisterSpec for OschpctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`oschpctrl::R`](R) reader structure"]
impl crate::Readable for OschpctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`oschpctrl::W`](W) writer structure"]
impl crate::Writable for OschpctrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OSCHPCTRL to value 0x3c"]
impl crate::Resettable for OschpctrlSpec {
    const RESET_VALUE: u32 = 0x3c;
}
