#[doc = "Register `PLLSTAT` reader"]
pub type R = crate::R<PllstatSpec>;
#[doc = "VCO Bypass Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Vcobyst {
    #[doc = "0: Free-running / Normal Mode is entered"]
    Const0 = 0,
    #[doc = "1: Prescaler Mode is entered"]
    Const1 = 1,
}
impl From<Vcobyst> for bool {
    #[inline(always)]
    fn from(variant: Vcobyst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VCOBYST` reader - VCO Bypass Status"]
pub type VcobystR = crate::BitReader<Vcobyst>;
impl VcobystR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Vcobyst {
        match self.bits {
            false => Vcobyst::Const0,
            true => Vcobyst::Const1,
        }
    }
    #[doc = "Free-running / Normal Mode is entered"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == Vcobyst::Const0
    }
    #[doc = "Prescaler Mode is entered"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == Vcobyst::Const1
    }
}
#[doc = "PLL Power-saving Mode Status\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pwdstat {
    #[doc = "0: PLL Power-saving Mode was not entered"]
    Const0 = 0,
    #[doc = "1: PLL Power-saving Mode was entered"]
    Const1 = 1,
}
impl From<Pwdstat> for bool {
    #[inline(always)]
    fn from(variant: Pwdstat) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWDSTAT` reader - PLL Power-saving Mode Status"]
pub type PwdstatR = crate::BitReader<Pwdstat>;
impl PwdstatR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pwdstat {
        match self.bits {
            false => Pwdstat::Const0,
            true => Pwdstat::Const1,
        }
    }
    #[doc = "PLL Power-saving Mode was not entered"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == Pwdstat::Const0
    }
    #[doc = "PLL Power-saving Mode was entered"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == Pwdstat::Const1
    }
}
#[doc = "PLL LOCK Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Vcolock {
    #[doc = "0: PLL not locked"]
    Const0 = 0,
    #[doc = "1: PLL locked"]
    Const1 = 1,
}
impl From<Vcolock> for bool {
    #[inline(always)]
    fn from(variant: Vcolock) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VCOLOCK` reader - PLL LOCK Status"]
pub type VcolockR = crate::BitReader<Vcolock>;
impl VcolockR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Vcolock {
        match self.bits {
            false => Vcolock::Const0,
            true => Vcolock::Const1,
        }
    }
    #[doc = "PLL not locked"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == Vcolock::Const0
    }
    #[doc = "PLL locked"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == Vcolock::Const1
    }
}
#[doc = "K1 Divider Ready Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum K1rdy {
    #[doc = "0: K1-Divider does not operate with the new value"]
    Const0 = 0,
    #[doc = "1: K1-Divider operate with the new value"]
    Const1 = 1,
}
impl From<K1rdy> for bool {
    #[inline(always)]
    fn from(variant: K1rdy) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `K1RDY` reader - K1 Divider Ready Status"]
pub type K1rdyR = crate::BitReader<K1rdy>;
impl K1rdyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> K1rdy {
        match self.bits {
            false => K1rdy::Const0,
            true => K1rdy::Const1,
        }
    }
    #[doc = "K1-Divider does not operate with the new value"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == K1rdy::Const0
    }
    #[doc = "K1-Divider operate with the new value"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == K1rdy::Const1
    }
}
#[doc = "K2 Divider Ready Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum K2rdy {
    #[doc = "0: K2-Divider does not operate with the new value"]
    Const0 = 0,
    #[doc = "1: K2-Divider operate with the new value"]
    Const1 = 1,
}
impl From<K2rdy> for bool {
    #[inline(always)]
    fn from(variant: K2rdy) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `K2RDY` reader - K2 Divider Ready Status"]
pub type K2rdyR = crate::BitReader<K2rdy>;
impl K2rdyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> K2rdy {
        match self.bits {
            false => K2rdy::Const0,
            true => K2rdy::Const1,
        }
    }
    #[doc = "K2-Divider does not operate with the new value"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == K2rdy::Const0
    }
    #[doc = "K2-Divider operate with the new value"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == K2rdy::Const1
    }
}
#[doc = "Bypass Mode Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum By {
    #[doc = "0: Bypass Mode is not entered"]
    Const0 = 0,
    #[doc = "1: Bypass Mode is entered. Input fOSC is selected as output fPLL."]
    Const1 = 1,
}
impl From<By> for bool {
    #[inline(always)]
    fn from(variant: By) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BY` reader - Bypass Mode Status"]
pub type ByR = crate::BitReader<By>;
impl ByR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> By {
        match self.bits {
            false => By::Const0,
            true => By::Const1,
        }
    }
    #[doc = "Bypass Mode is not entered"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == By::Const0
    }
    #[doc = "Bypass Mode is entered. Input fOSC is selected as output fPLL."]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == By::Const1
    }
}
#[doc = "Oscillator for PLL Valid Low Status Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Plllv {
    #[doc = "0: The OSC frequency is not usable. Frequency fREF is too low."]
    Const0 = 0,
    #[doc = "1: The OSC frequency is usable"]
    Const1 = 1,
}
impl From<Plllv> for bool {
    #[inline(always)]
    fn from(variant: Plllv) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PLLLV` reader - Oscillator for PLL Valid Low Status Bit"]
pub type PlllvR = crate::BitReader<Plllv>;
impl PlllvR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Plllv {
        match self.bits {
            false => Plllv::Const0,
            true => Plllv::Const1,
        }
    }
    #[doc = "The OSC frequency is not usable. Frequency fREF is too low."]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == Plllv::Const0
    }
    #[doc = "The OSC frequency is usable"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == Plllv::Const1
    }
}
#[doc = "Oscillator for PLL Valid High Status Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pllhv {
    #[doc = "0: The OSC frequency is not usable. Frequency fOSC is too high."]
    Const0 = 0,
    #[doc = "1: The OSC frequency is usable"]
    Const1 = 1,
}
impl From<Pllhv> for bool {
    #[inline(always)]
    fn from(variant: Pllhv) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PLLHV` reader - Oscillator for PLL Valid High Status Bit"]
pub type PllhvR = crate::BitReader<Pllhv>;
impl PllhvR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pllhv {
        match self.bits {
            false => Pllhv::Const0,
            true => Pllhv::Const1,
        }
    }
    #[doc = "The OSC frequency is not usable. Frequency fOSC is too high."]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == Pllhv::Const0
    }
    #[doc = "The OSC frequency is usable"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == Pllhv::Const1
    }
}
#[doc = "Oscillator for PLL Valid Spike Status Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pllsp {
    #[doc = "0: The OSC frequency is not usable. Spikes are detected that disturb a locked operation"]
    Const0 = 0,
    #[doc = "1: The OSC frequency is usable"]
    Const1 = 1,
}
impl From<Pllsp> for bool {
    #[inline(always)]
    fn from(variant: Pllsp) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PLLSP` reader - Oscillator for PLL Valid Spike Status Bit"]
pub type PllspR = crate::BitReader<Pllsp>;
impl PllspR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pllsp {
        match self.bits {
            false => Pllsp::Const0,
            true => Pllsp::Const1,
        }
    }
    #[doc = "The OSC frequency is not usable. Spikes are detected that disturb a locked operation"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == Pllsp::Const0
    }
    #[doc = "The OSC frequency is usable"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == Pllsp::Const1
    }
}
impl R {
    #[doc = "Bit 0 - VCO Bypass Status"]
    #[inline(always)]
    pub fn vcobyst(&self) -> VcobystR {
        VcobystR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - PLL Power-saving Mode Status"]
    #[inline(always)]
    pub fn pwdstat(&self) -> PwdstatR {
        PwdstatR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - PLL LOCK Status"]
    #[inline(always)]
    pub fn vcolock(&self) -> VcolockR {
        VcolockR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - K1 Divider Ready Status"]
    #[inline(always)]
    pub fn k1rdy(&self) -> K1rdyR {
        K1rdyR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - K2 Divider Ready Status"]
    #[inline(always)]
    pub fn k2rdy(&self) -> K2rdyR {
        K2rdyR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Bypass Mode Status"]
    #[inline(always)]
    pub fn by(&self) -> ByR {
        ByR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Oscillator for PLL Valid Low Status Bit"]
    #[inline(always)]
    pub fn plllv(&self) -> PlllvR {
        PlllvR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Oscillator for PLL Valid High Status Bit"]
    #[inline(always)]
    pub fn pllhv(&self) -> PllhvR {
        PllhvR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Oscillator for PLL Valid Spike Status Bit"]
    #[inline(always)]
    pub fn pllsp(&self) -> PllspR {
        PllspR::new(((self.bits >> 9) & 1) != 0)
    }
}
#[doc = "PLL Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pllstat::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PllstatSpec;
impl crate::RegisterSpec for PllstatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pllstat::R`](R) reader structure"]
impl crate::Readable for PllstatSpec {}
#[doc = "`reset()` method sets PLLSTAT to value 0x02"]
impl crate::Resettable for PllstatSpec {
    const RESET_VALUE: u32 = 0x02;
}
