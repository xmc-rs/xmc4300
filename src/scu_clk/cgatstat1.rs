#[doc = "Register `CGATSTAT1` reader"]
pub type R = crate::R<CGATSTAT1_SPEC>;
#[doc = "LEDTS Gating Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LEDTSCU0_A {
    #[doc = "0: Gating de-asserted"]
    VALUE1 = 0,
    #[doc = "1: Gating asserted"]
    VALUE2 = 1,
}
impl From<LEDTSCU0_A> for bool {
    #[inline(always)]
    fn from(variant: LEDTSCU0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LEDTSCU0` reader - LEDTS Gating Status"]
pub type LEDTSCU0_R = crate::BitReader<LEDTSCU0_A>;
impl LEDTSCU0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LEDTSCU0_A {
        match self.bits {
            false => LEDTSCU0_A::VALUE1,
            true => LEDTSCU0_A::VALUE2,
        }
    }
    #[doc = "Gating de-asserted"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == LEDTSCU0_A::VALUE1
    }
    #[doc = "Gating asserted"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == LEDTSCU0_A::VALUE2
    }
}
#[doc = "MultiCAN Gating Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MCAN0_A {
    #[doc = "0: Gating de-asserted"]
    VALUE1 = 0,
    #[doc = "1: Gating asserted"]
    VALUE2 = 1,
}
impl From<MCAN0_A> for bool {
    #[inline(always)]
    fn from(variant: MCAN0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MCAN0` reader - MultiCAN Gating Status"]
pub type MCAN0_R = crate::BitReader<MCAN0_A>;
impl MCAN0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MCAN0_A {
        match self.bits {
            false => MCAN0_A::VALUE1,
            true => MCAN0_A::VALUE2,
        }
    }
    #[doc = "Gating de-asserted"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == MCAN0_A::VALUE1
    }
    #[doc = "Gating asserted"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == MCAN0_A::VALUE2
    }
}
#[doc = "DAC Gating Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DAC_A {
    #[doc = "0: Gating de-asserted"]
    VALUE1 = 0,
    #[doc = "1: Gating asserted"]
    VALUE2 = 1,
}
impl From<DAC_A> for bool {
    #[inline(always)]
    fn from(variant: DAC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DAC` reader - DAC Gating Status"]
pub type DAC_R = crate::BitReader<DAC_A>;
impl DAC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DAC_A {
        match self.bits {
            false => DAC_A::VALUE1,
            true => DAC_A::VALUE2,
        }
    }
    #[doc = "Gating de-asserted"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == DAC_A::VALUE1
    }
    #[doc = "Gating asserted"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == DAC_A::VALUE2
    }
}
#[doc = "MMC Interface Gating Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MMCI_A {
    #[doc = "0: Gating de-asserted"]
    VALUE1 = 0,
    #[doc = "1: Gating asserted"]
    VALUE2 = 1,
}
impl From<MMCI_A> for bool {
    #[inline(always)]
    fn from(variant: MMCI_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MMCI` reader - MMC Interface Gating Status"]
pub type MMCI_R = crate::BitReader<MMCI_A>;
impl MMCI_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MMCI_A {
        match self.bits {
            false => MMCI_A::VALUE1,
            true => MMCI_A::VALUE2,
        }
    }
    #[doc = "Gating de-asserted"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == MMCI_A::VALUE1
    }
    #[doc = "Gating asserted"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == MMCI_A::VALUE2
    }
}
#[doc = "USIC1 Gating Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USIC1_A {
    #[doc = "0: Gating de-asserted"]
    VALUE1 = 0,
    #[doc = "1: Gating asserted"]
    VALUE2 = 1,
}
impl From<USIC1_A> for bool {
    #[inline(always)]
    fn from(variant: USIC1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USIC1` reader - USIC1 Gating Status"]
pub type USIC1_R = crate::BitReader<USIC1_A>;
impl USIC1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> USIC1_A {
        match self.bits {
            false => USIC1_A::VALUE1,
            true => USIC1_A::VALUE2,
        }
    }
    #[doc = "Gating de-asserted"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == USIC1_A::VALUE1
    }
    #[doc = "Gating asserted"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == USIC1_A::VALUE2
    }
}
#[doc = "PORTS Gating Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PPORTS_A {
    #[doc = "0: Gating de-asserted"]
    VALUE1 = 0,
    #[doc = "1: Gating asserted"]
    VALUE2 = 1,
}
impl From<PPORTS_A> for bool {
    #[inline(always)]
    fn from(variant: PPORTS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PPORTS` reader - PORTS Gating Status"]
pub type PPORTS_R = crate::BitReader<PPORTS_A>;
impl PPORTS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PPORTS_A {
        match self.bits {
            false => PPORTS_A::VALUE1,
            true => PPORTS_A::VALUE2,
        }
    }
    #[doc = "Gating de-asserted"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PPORTS_A::VALUE1
    }
    #[doc = "Gating asserted"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PPORTS_A::VALUE2
    }
}
impl R {
    #[doc = "Bit 3 - LEDTS Gating Status"]
    #[inline(always)]
    pub fn ledtscu0(&self) -> LEDTSCU0_R {
        LEDTSCU0_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - MultiCAN Gating Status"]
    #[inline(always)]
    pub fn mcan0(&self) -> MCAN0_R {
        MCAN0_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - DAC Gating Status"]
    #[inline(always)]
    pub fn dac(&self) -> DAC_R {
        DAC_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - MMC Interface Gating Status"]
    #[inline(always)]
    pub fn mmci(&self) -> MMCI_R {
        MMCI_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - USIC1 Gating Status"]
    #[inline(always)]
    pub fn usic1(&self) -> USIC1_R {
        USIC1_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 9 - PORTS Gating Status"]
    #[inline(always)]
    pub fn pports(&self) -> PPORTS_R {
        PPORTS_R::new(((self.bits >> 9) & 1) != 0)
    }
}
#[doc = "Peripheral 1 Clock Gating Status\n\nYou can [`read`](crate::Reg::read) this register and get [`cgatstat1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CGATSTAT1_SPEC;
impl crate::RegisterSpec for CGATSTAT1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cgatstat1::R`](R) reader structure"]
impl crate::Readable for CGATSTAT1_SPEC {}
#[doc = "`reset()` method sets CGATSTAT1 to value 0"]
impl crate::Resettable for CGATSTAT1_SPEC {
    const RESET_VALUE: u32 = 0;
}
