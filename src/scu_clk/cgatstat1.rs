#[doc = "Register `CGATSTAT1` reader"]
pub type R = crate::R<Cgatstat1Spec>;
#[doc = "LEDTS Gating Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ledtscu0 {
    #[doc = "0: Gating de-asserted"]
    Value1 = 0,
    #[doc = "1: Gating asserted"]
    Value2 = 1,
}
impl From<Ledtscu0> for bool {
    #[inline(always)]
    fn from(variant: Ledtscu0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LEDTSCU0` reader - LEDTS Gating Status"]
pub type Ledtscu0R = crate::BitReader<Ledtscu0>;
impl Ledtscu0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ledtscu0 {
        match self.bits {
            false => Ledtscu0::Value1,
            true => Ledtscu0::Value2,
        }
    }
    #[doc = "Gating de-asserted"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Ledtscu0::Value1
    }
    #[doc = "Gating asserted"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Ledtscu0::Value2
    }
}
#[doc = "MultiCAN Gating Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mcan0 {
    #[doc = "0: Gating de-asserted"]
    Value1 = 0,
    #[doc = "1: Gating asserted"]
    Value2 = 1,
}
impl From<Mcan0> for bool {
    #[inline(always)]
    fn from(variant: Mcan0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MCAN0` reader - MultiCAN Gating Status"]
pub type Mcan0R = crate::BitReader<Mcan0>;
impl Mcan0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mcan0 {
        match self.bits {
            false => Mcan0::Value1,
            true => Mcan0::Value2,
        }
    }
    #[doc = "Gating de-asserted"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Mcan0::Value1
    }
    #[doc = "Gating asserted"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Mcan0::Value2
    }
}
#[doc = "DAC Gating Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dac {
    #[doc = "0: Gating de-asserted"]
    Value1 = 0,
    #[doc = "1: Gating asserted"]
    Value2 = 1,
}
impl From<Dac> for bool {
    #[inline(always)]
    fn from(variant: Dac) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DAC` reader - DAC Gating Status"]
pub type DacR = crate::BitReader<Dac>;
impl DacR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dac {
        match self.bits {
            false => Dac::Value1,
            true => Dac::Value2,
        }
    }
    #[doc = "Gating de-asserted"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Dac::Value1
    }
    #[doc = "Gating asserted"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Dac::Value2
    }
}
#[doc = "MMC Interface Gating Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mmci {
    #[doc = "0: Gating de-asserted"]
    Value1 = 0,
    #[doc = "1: Gating asserted"]
    Value2 = 1,
}
impl From<Mmci> for bool {
    #[inline(always)]
    fn from(variant: Mmci) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MMCI` reader - MMC Interface Gating Status"]
pub type MmciR = crate::BitReader<Mmci>;
impl MmciR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mmci {
        match self.bits {
            false => Mmci::Value1,
            true => Mmci::Value2,
        }
    }
    #[doc = "Gating de-asserted"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Mmci::Value1
    }
    #[doc = "Gating asserted"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Mmci::Value2
    }
}
#[doc = "USIC1 Gating Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Usic1 {
    #[doc = "0: Gating de-asserted"]
    Value1 = 0,
    #[doc = "1: Gating asserted"]
    Value2 = 1,
}
impl From<Usic1> for bool {
    #[inline(always)]
    fn from(variant: Usic1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USIC1` reader - USIC1 Gating Status"]
pub type Usic1R = crate::BitReader<Usic1>;
impl Usic1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Usic1 {
        match self.bits {
            false => Usic1::Value1,
            true => Usic1::Value2,
        }
    }
    #[doc = "Gating de-asserted"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Usic1::Value1
    }
    #[doc = "Gating asserted"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Usic1::Value2
    }
}
#[doc = "PORTS Gating Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pports {
    #[doc = "0: Gating de-asserted"]
    Value1 = 0,
    #[doc = "1: Gating asserted"]
    Value2 = 1,
}
impl From<Pports> for bool {
    #[inline(always)]
    fn from(variant: Pports) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PPORTS` reader - PORTS Gating Status"]
pub type PportsR = crate::BitReader<Pports>;
impl PportsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pports {
        match self.bits {
            false => Pports::Value1,
            true => Pports::Value2,
        }
    }
    #[doc = "Gating de-asserted"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Pports::Value1
    }
    #[doc = "Gating asserted"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Pports::Value2
    }
}
impl R {
    #[doc = "Bit 3 - LEDTS Gating Status"]
    #[inline(always)]
    pub fn ledtscu0(&self) -> Ledtscu0R {
        Ledtscu0R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - MultiCAN Gating Status"]
    #[inline(always)]
    pub fn mcan0(&self) -> Mcan0R {
        Mcan0R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - DAC Gating Status"]
    #[inline(always)]
    pub fn dac(&self) -> DacR {
        DacR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - MMC Interface Gating Status"]
    #[inline(always)]
    pub fn mmci(&self) -> MmciR {
        MmciR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - USIC1 Gating Status"]
    #[inline(always)]
    pub fn usic1(&self) -> Usic1R {
        Usic1R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 9 - PORTS Gating Status"]
    #[inline(always)]
    pub fn pports(&self) -> PportsR {
        PportsR::new(((self.bits >> 9) & 1) != 0)
    }
}
#[doc = "Peripheral 1 Clock Gating Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cgatstat1::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cgatstat1Spec;
impl crate::RegisterSpec for Cgatstat1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cgatstat1::R`](R) reader structure"]
impl crate::Readable for Cgatstat1Spec {}
#[doc = "`reset()` method sets CGATSTAT1 to value 0"]
impl crate::Resettable for Cgatstat1Spec {
    const RESET_VALUE: u32 = 0;
}
