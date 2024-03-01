#[doc = "Register `HDCR` reader"]
pub type R = crate::R<HdcrSpec>;
#[doc = "Register `HDCR` writer"]
pub type W = crate::W<HdcrSpec>;
#[doc = "Wake-Up on Pin Event Positive Edge Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wkpep {
    #[doc = "0: Wake-up event disabled"]
    Const0 = 0,
    #[doc = "1: Wake-up event enabled"]
    Const1 = 1,
}
impl From<Wkpep> for bool {
    #[inline(always)]
    fn from(variant: Wkpep) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WKPEP` reader - Wake-Up on Pin Event Positive Edge Enable"]
pub type WkpepR = crate::BitReader<Wkpep>;
impl WkpepR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wkpep {
        match self.bits {
            false => Wkpep::Const0,
            true => Wkpep::Const1,
        }
    }
    #[doc = "Wake-up event disabled"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == Wkpep::Const0
    }
    #[doc = "Wake-up event enabled"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == Wkpep::Const1
    }
}
#[doc = "Field `WKPEP` writer - Wake-Up on Pin Event Positive Edge Enable"]
pub type WkpepW<'a, REG> = crate::BitWriter<'a, REG, Wkpep>;
impl<'a, REG> WkpepW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Wake-up event disabled"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(Wkpep::Const0)
    }
    #[doc = "Wake-up event enabled"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(Wkpep::Const1)
    }
}
#[doc = "Wake-up on Pin Event Negative Edge Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wkpen {
    #[doc = "0: Wake-up event disabled"]
    Const0 = 0,
    #[doc = "1: Wake-up event enabled"]
    Const1 = 1,
}
impl From<Wkpen> for bool {
    #[inline(always)]
    fn from(variant: Wkpen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WKPEN` reader - Wake-up on Pin Event Negative Edge Enable"]
pub type WkpenR = crate::BitReader<Wkpen>;
impl WkpenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wkpen {
        match self.bits {
            false => Wkpen::Const0,
            true => Wkpen::Const1,
        }
    }
    #[doc = "Wake-up event disabled"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == Wkpen::Const0
    }
    #[doc = "Wake-up event enabled"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == Wkpen::Const1
    }
}
#[doc = "Field `WKPEN` writer - Wake-up on Pin Event Negative Edge Enable"]
pub type WkpenW<'a, REG> = crate::BitWriter<'a, REG, Wkpen>;
impl<'a, REG> WkpenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Wake-up event disabled"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(Wkpen::Const0)
    }
    #[doc = "Wake-up event enabled"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(Wkpen::Const1)
    }
}
#[doc = "Wake-up on RTC Event Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rtce {
    #[doc = "0: Wake-up event disabled"]
    Const0 = 0,
    #[doc = "1: Wake-up event enabled"]
    Const1 = 1,
}
impl From<Rtce> for bool {
    #[inline(always)]
    fn from(variant: Rtce) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RTCE` reader - Wake-up on RTC Event Enable"]
pub type RtceR = crate::BitReader<Rtce>;
impl RtceR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rtce {
        match self.bits {
            false => Rtce::Const0,
            true => Rtce::Const1,
        }
    }
    #[doc = "Wake-up event disabled"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == Rtce::Const0
    }
    #[doc = "Wake-up event enabled"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == Rtce::Const1
    }
}
#[doc = "Field `RTCE` writer - Wake-up on RTC Event Enable"]
pub type RtceW<'a, REG> = crate::BitWriter<'a, REG, Rtce>;
impl<'a, REG> RtceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Wake-up event disabled"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(Rtce::Const0)
    }
    #[doc = "Wake-up event enabled"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(Rtce::Const1)
    }
}
#[doc = "ULP WDG Alarm Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ulpwdgen {
    #[doc = "0: Wake-up event disabled"]
    Const0 = 0,
    #[doc = "1: Wake-up event enabled"]
    Const1 = 1,
}
impl From<Ulpwdgen> for bool {
    #[inline(always)]
    fn from(variant: Ulpwdgen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ULPWDGEN` reader - ULP WDG Alarm Enable"]
pub type UlpwdgenR = crate::BitReader<Ulpwdgen>;
impl UlpwdgenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ulpwdgen {
        match self.bits {
            false => Ulpwdgen::Const0,
            true => Ulpwdgen::Const1,
        }
    }
    #[doc = "Wake-up event disabled"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == Ulpwdgen::Const0
    }
    #[doc = "Wake-up event enabled"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == Ulpwdgen::Const1
    }
}
#[doc = "Field `ULPWDGEN` writer - ULP WDG Alarm Enable"]
pub type UlpwdgenW<'a, REG> = crate::BitWriter<'a, REG, Ulpwdgen>;
impl<'a, REG> UlpwdgenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Wake-up event disabled"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(Ulpwdgen::Const0)
    }
    #[doc = "Wake-up event enabled"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(Ulpwdgen::Const1)
    }
}
#[doc = "Hibernate Request Value Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hib {
    #[doc = "0: External hibernate request inactive"]
    Const0 = 0,
    #[doc = "1: External hibernate request active"]
    Const1 = 1,
}
impl From<Hib> for bool {
    #[inline(always)]
    fn from(variant: Hib) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HIB` reader - Hibernate Request Value Set"]
pub type HibR = crate::BitReader<Hib>;
impl HibR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hib {
        match self.bits {
            false => Hib::Const0,
            true => Hib::Const1,
        }
    }
    #[doc = "External hibernate request inactive"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == Hib::Const0
    }
    #[doc = "External hibernate request active"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == Hib::Const1
    }
}
#[doc = "Field `HIB` writer - Hibernate Request Value Set"]
pub type HibW<'a, REG> = crate::BitWriter<'a, REG, Hib>;
impl<'a, REG> HibW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "External hibernate request inactive"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(Hib::Const0)
    }
    #[doc = "External hibernate request active"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(Hib::Const1)
    }
}
#[doc = "fRTC Clock Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rcs {
    #[doc = "0: fOSI selected"]
    Const0 = 0,
    #[doc = "1: fULP selected"]
    Const1 = 1,
}
impl From<Rcs> for bool {
    #[inline(always)]
    fn from(variant: Rcs) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RCS` reader - fRTC Clock Selection"]
pub type RcsR = crate::BitReader<Rcs>;
impl RcsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rcs {
        match self.bits {
            false => Rcs::Const0,
            true => Rcs::Const1,
        }
    }
    #[doc = "fOSI selected"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == Rcs::Const0
    }
    #[doc = "fULP selected"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == Rcs::Const1
    }
}
#[doc = "Field `RCS` writer - fRTC Clock Selection"]
pub type RcsW<'a, REG> = crate::BitWriter<'a, REG, Rcs>;
impl<'a, REG> RcsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "fOSI selected"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(Rcs::Const0)
    }
    #[doc = "fULP selected"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(Rcs::Const1)
    }
}
#[doc = "fSTDBY Clock Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Stdbysel {
    #[doc = "0: fOSI selected"]
    Const0 = 0,
    #[doc = "1: fULP selected"]
    Const1 = 1,
}
impl From<Stdbysel> for bool {
    #[inline(always)]
    fn from(variant: Stdbysel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STDBYSEL` reader - fSTDBY Clock Selection"]
pub type StdbyselR = crate::BitReader<Stdbysel>;
impl StdbyselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Stdbysel {
        match self.bits {
            false => Stdbysel::Const0,
            true => Stdbysel::Const1,
        }
    }
    #[doc = "fOSI selected"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == Stdbysel::Const0
    }
    #[doc = "fULP selected"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == Stdbysel::Const1
    }
}
#[doc = "Field `STDBYSEL` writer - fSTDBY Clock Selection"]
pub type StdbyselW<'a, REG> = crate::BitWriter<'a, REG, Stdbysel>;
impl<'a, REG> StdbyselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "fOSI selected"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(Stdbysel::Const0)
    }
    #[doc = "fULP selected"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(Stdbysel::Const1)
    }
}
#[doc = "Wake-Up from Hibernate Trigger Input Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wkupsel {
    #[doc = "0: HIB_IO_1 pin selected"]
    Const0 = 0,
    #[doc = "1: HIB_IO_0 pin selected"]
    Const1 = 1,
}
impl From<Wkupsel> for bool {
    #[inline(always)]
    fn from(variant: Wkupsel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WKUPSEL` reader - Wake-Up from Hibernate Trigger Input Selection"]
pub type WkupselR = crate::BitReader<Wkupsel>;
impl WkupselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wkupsel {
        match self.bits {
            false => Wkupsel::Const0,
            true => Wkupsel::Const1,
        }
    }
    #[doc = "HIB_IO_1 pin selected"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == Wkupsel::Const0
    }
    #[doc = "HIB_IO_0 pin selected"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == Wkupsel::Const1
    }
}
#[doc = "Field `WKUPSEL` writer - Wake-Up from Hibernate Trigger Input Selection"]
pub type WkupselW<'a, REG> = crate::BitWriter<'a, REG, Wkupsel>;
impl<'a, REG> WkupselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "HIB_IO_1 pin selected"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(Wkupsel::Const0)
    }
    #[doc = "HIB_IO_0 pin selected"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(Wkupsel::Const1)
    }
}
#[doc = "General Purpose Input 0 Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpi0sel {
    #[doc = "0: #0"]
    Const0 = 0,
    #[doc = "1: HIB_IO_0 pin selected"]
    Const1 = 1,
}
impl From<Gpi0sel> for bool {
    #[inline(always)]
    fn from(variant: Gpi0sel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPI0SEL` reader - General Purpose Input 0 Selection"]
pub type Gpi0selR = crate::BitReader<Gpi0sel>;
impl Gpi0selR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpi0sel {
        match self.bits {
            false => Gpi0sel::Const0,
            true => Gpi0sel::Const1,
        }
    }
    #[doc = "#0"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == Gpi0sel::Const0
    }
    #[doc = "HIB_IO_0 pin selected"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == Gpi0sel::Const1
    }
}
#[doc = "Field `GPI0SEL` writer - General Purpose Input 0 Selection"]
pub type Gpi0selW<'a, REG> = crate::BitWriter<'a, REG, Gpi0sel>;
impl<'a, REG> Gpi0selW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "#0"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(Gpi0sel::Const0)
    }
    #[doc = "HIB_IO_0 pin selected"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(Gpi0sel::Const1)
    }
}
#[doc = "HIBIO0 Polarity Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hibio0pol {
    #[doc = "0: Direct value"]
    Const0 = 0,
    #[doc = "1: Inverted value"]
    Const1 = 1,
}
impl From<Hibio0pol> for bool {
    #[inline(always)]
    fn from(variant: Hibio0pol) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HIBIO0POL` reader - HIBIO0 Polarity Set"]
pub type Hibio0polR = crate::BitReader<Hibio0pol>;
impl Hibio0polR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hibio0pol {
        match self.bits {
            false => Hibio0pol::Const0,
            true => Hibio0pol::Const1,
        }
    }
    #[doc = "Direct value"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == Hibio0pol::Const0
    }
    #[doc = "Inverted value"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == Hibio0pol::Const1
    }
}
#[doc = "Field `HIBIO0POL` writer - HIBIO0 Polarity Set"]
pub type Hibio0polW<'a, REG> = crate::BitWriter<'a, REG, Hibio0pol>;
impl<'a, REG> Hibio0polW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Direct value"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(Hibio0pol::Const0)
    }
    #[doc = "Inverted value"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(Hibio0pol::Const1)
    }
}
#[doc = "HIBIO1 Polarity Set\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hibio1pol {
    #[doc = "0: Direct value"]
    Const0 = 0,
    #[doc = "1: Inverted value"]
    Const1 = 1,
}
impl From<Hibio1pol> for bool {
    #[inline(always)]
    fn from(variant: Hibio1pol) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HIBIO1POL` reader - HIBIO1 Polarity Set"]
pub type Hibio1polR = crate::BitReader<Hibio1pol>;
impl Hibio1polR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hibio1pol {
        match self.bits {
            false => Hibio1pol::Const0,
            true => Hibio1pol::Const1,
        }
    }
    #[doc = "Direct value"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == Hibio1pol::Const0
    }
    #[doc = "Inverted value"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == Hibio1pol::Const1
    }
}
#[doc = "Field `HIBIO1POL` writer - HIBIO1 Polarity Set"]
pub type Hibio1polW<'a, REG> = crate::BitWriter<'a, REG, Hibio1pol>;
impl<'a, REG> Hibio1polW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Direct value"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(Hibio1pol::Const0)
    }
    #[doc = "Inverted value"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(Hibio1pol::Const1)
    }
}
#[doc = "HIB_IO_0 Pin I/O Control (default HIBOUT)\n\nValue on reset: 12"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Hibio0sel {
    #[doc = "0: Direct input, No input pull device connected"]
    Const0000 = 0,
    #[doc = "1: Direct input, Input pull-down device connected"]
    Const0001 = 1,
    #[doc = "2: Direct input, Input pull-up device connected"]
    Const0010 = 2,
    #[doc = "8: Push-pull HIB Control output"]
    Const1000 = 8,
    #[doc = "9: Push-pull WDT service output"]
    Const1001 = 9,
    #[doc = "10: Push-pull GPIO output"]
    Const1010 = 10,
    #[doc = "12: Open-drain HIB Control output"]
    Const1100 = 12,
    #[doc = "13: Open-drain WDT service output"]
    Const1101 = 13,
    #[doc = "14: Open-drain GPIO output"]
    Const1110 = 14,
    #[doc = "15: #1111"]
    Const1111 = 15,
}
impl From<Hibio0sel> for u8 {
    #[inline(always)]
    fn from(variant: Hibio0sel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Hibio0sel {
    type Ux = u8;
}
#[doc = "Field `HIBIO0SEL` reader - HIB_IO_0 Pin I/O Control (default HIBOUT)"]
pub type Hibio0selR = crate::FieldReader<Hibio0sel>;
impl Hibio0selR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Hibio0sel> {
        match self.bits {
            0 => Some(Hibio0sel::Const0000),
            1 => Some(Hibio0sel::Const0001),
            2 => Some(Hibio0sel::Const0010),
            8 => Some(Hibio0sel::Const1000),
            9 => Some(Hibio0sel::Const1001),
            10 => Some(Hibio0sel::Const1010),
            12 => Some(Hibio0sel::Const1100),
            13 => Some(Hibio0sel::Const1101),
            14 => Some(Hibio0sel::Const1110),
            15 => Some(Hibio0sel::Const1111),
            _ => None,
        }
    }
    #[doc = "Direct input, No input pull device connected"]
    #[inline(always)]
    pub fn is_const_0000(&self) -> bool {
        *self == Hibio0sel::Const0000
    }
    #[doc = "Direct input, Input pull-down device connected"]
    #[inline(always)]
    pub fn is_const_0001(&self) -> bool {
        *self == Hibio0sel::Const0001
    }
    #[doc = "Direct input, Input pull-up device connected"]
    #[inline(always)]
    pub fn is_const_0010(&self) -> bool {
        *self == Hibio0sel::Const0010
    }
    #[doc = "Push-pull HIB Control output"]
    #[inline(always)]
    pub fn is_const_1000(&self) -> bool {
        *self == Hibio0sel::Const1000
    }
    #[doc = "Push-pull WDT service output"]
    #[inline(always)]
    pub fn is_const_1001(&self) -> bool {
        *self == Hibio0sel::Const1001
    }
    #[doc = "Push-pull GPIO output"]
    #[inline(always)]
    pub fn is_const_1010(&self) -> bool {
        *self == Hibio0sel::Const1010
    }
    #[doc = "Open-drain HIB Control output"]
    #[inline(always)]
    pub fn is_const_1100(&self) -> bool {
        *self == Hibio0sel::Const1100
    }
    #[doc = "Open-drain WDT service output"]
    #[inline(always)]
    pub fn is_const_1101(&self) -> bool {
        *self == Hibio0sel::Const1101
    }
    #[doc = "Open-drain GPIO output"]
    #[inline(always)]
    pub fn is_const_1110(&self) -> bool {
        *self == Hibio0sel::Const1110
    }
    #[doc = "#1111"]
    #[inline(always)]
    pub fn is_const_1111(&self) -> bool {
        *self == Hibio0sel::Const1111
    }
}
#[doc = "Field `HIBIO0SEL` writer - HIB_IO_0 Pin I/O Control (default HIBOUT)"]
pub type Hibio0selW<'a, REG> = crate::FieldWriter<'a, REG, 4, Hibio0sel>;
impl<'a, REG> Hibio0selW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Direct input, No input pull device connected"]
    #[inline(always)]
    pub fn const_0000(self) -> &'a mut crate::W<REG> {
        self.variant(Hibio0sel::Const0000)
    }
    #[doc = "Direct input, Input pull-down device connected"]
    #[inline(always)]
    pub fn const_0001(self) -> &'a mut crate::W<REG> {
        self.variant(Hibio0sel::Const0001)
    }
    #[doc = "Direct input, Input pull-up device connected"]
    #[inline(always)]
    pub fn const_0010(self) -> &'a mut crate::W<REG> {
        self.variant(Hibio0sel::Const0010)
    }
    #[doc = "Push-pull HIB Control output"]
    #[inline(always)]
    pub fn const_1000(self) -> &'a mut crate::W<REG> {
        self.variant(Hibio0sel::Const1000)
    }
    #[doc = "Push-pull WDT service output"]
    #[inline(always)]
    pub fn const_1001(self) -> &'a mut crate::W<REG> {
        self.variant(Hibio0sel::Const1001)
    }
    #[doc = "Push-pull GPIO output"]
    #[inline(always)]
    pub fn const_1010(self) -> &'a mut crate::W<REG> {
        self.variant(Hibio0sel::Const1010)
    }
    #[doc = "Open-drain HIB Control output"]
    #[inline(always)]
    pub fn const_1100(self) -> &'a mut crate::W<REG> {
        self.variant(Hibio0sel::Const1100)
    }
    #[doc = "Open-drain WDT service output"]
    #[inline(always)]
    pub fn const_1101(self) -> &'a mut crate::W<REG> {
        self.variant(Hibio0sel::Const1101)
    }
    #[doc = "Open-drain GPIO output"]
    #[inline(always)]
    pub fn const_1110(self) -> &'a mut crate::W<REG> {
        self.variant(Hibio0sel::Const1110)
    }
    #[doc = "#1111"]
    #[inline(always)]
    pub fn const_1111(self) -> &'a mut crate::W<REG> {
        self.variant(Hibio0sel::Const1111)
    }
}
#[doc = "HIB_IO_1 Pin I/O Control (Default WKUP)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Hibio1sel {
    #[doc = "0: Direct input, No input pull device connected"]
    Const0000 = 0,
    #[doc = "1: Direct input, Input pull-down device connected"]
    Const0001 = 1,
    #[doc = "2: Direct input, Input pull-up device connected"]
    Const0010 = 2,
    #[doc = "8: Push-pull HIB Control output"]
    Const1000 = 8,
    #[doc = "9: Push-pull WDT service output"]
    Const1001 = 9,
    #[doc = "10: Push-pull GPIO output"]
    Const1010 = 10,
    #[doc = "12: Open-drain HIB Control output"]
    Const1100 = 12,
    #[doc = "13: Open-drain WDT service output"]
    Const1101 = 13,
    #[doc = "14: Open-drain GPIO output"]
    Const1110 = 14,
    #[doc = "15: #1111"]
    Const1111 = 15,
}
impl From<Hibio1sel> for u8 {
    #[inline(always)]
    fn from(variant: Hibio1sel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Hibio1sel {
    type Ux = u8;
}
#[doc = "Field `HIBIO1SEL` reader - HIB_IO_1 Pin I/O Control (Default WKUP)"]
pub type Hibio1selR = crate::FieldReader<Hibio1sel>;
impl Hibio1selR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Hibio1sel> {
        match self.bits {
            0 => Some(Hibio1sel::Const0000),
            1 => Some(Hibio1sel::Const0001),
            2 => Some(Hibio1sel::Const0010),
            8 => Some(Hibio1sel::Const1000),
            9 => Some(Hibio1sel::Const1001),
            10 => Some(Hibio1sel::Const1010),
            12 => Some(Hibio1sel::Const1100),
            13 => Some(Hibio1sel::Const1101),
            14 => Some(Hibio1sel::Const1110),
            15 => Some(Hibio1sel::Const1111),
            _ => None,
        }
    }
    #[doc = "Direct input, No input pull device connected"]
    #[inline(always)]
    pub fn is_const_0000(&self) -> bool {
        *self == Hibio1sel::Const0000
    }
    #[doc = "Direct input, Input pull-down device connected"]
    #[inline(always)]
    pub fn is_const_0001(&self) -> bool {
        *self == Hibio1sel::Const0001
    }
    #[doc = "Direct input, Input pull-up device connected"]
    #[inline(always)]
    pub fn is_const_0010(&self) -> bool {
        *self == Hibio1sel::Const0010
    }
    #[doc = "Push-pull HIB Control output"]
    #[inline(always)]
    pub fn is_const_1000(&self) -> bool {
        *self == Hibio1sel::Const1000
    }
    #[doc = "Push-pull WDT service output"]
    #[inline(always)]
    pub fn is_const_1001(&self) -> bool {
        *self == Hibio1sel::Const1001
    }
    #[doc = "Push-pull GPIO output"]
    #[inline(always)]
    pub fn is_const_1010(&self) -> bool {
        *self == Hibio1sel::Const1010
    }
    #[doc = "Open-drain HIB Control output"]
    #[inline(always)]
    pub fn is_const_1100(&self) -> bool {
        *self == Hibio1sel::Const1100
    }
    #[doc = "Open-drain WDT service output"]
    #[inline(always)]
    pub fn is_const_1101(&self) -> bool {
        *self == Hibio1sel::Const1101
    }
    #[doc = "Open-drain GPIO output"]
    #[inline(always)]
    pub fn is_const_1110(&self) -> bool {
        *self == Hibio1sel::Const1110
    }
    #[doc = "#1111"]
    #[inline(always)]
    pub fn is_const_1111(&self) -> bool {
        *self == Hibio1sel::Const1111
    }
}
#[doc = "Field `HIBIO1SEL` writer - HIB_IO_1 Pin I/O Control (Default WKUP)"]
pub type Hibio1selW<'a, REG> = crate::FieldWriter<'a, REG, 4, Hibio1sel>;
impl<'a, REG> Hibio1selW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Direct input, No input pull device connected"]
    #[inline(always)]
    pub fn const_0000(self) -> &'a mut crate::W<REG> {
        self.variant(Hibio1sel::Const0000)
    }
    #[doc = "Direct input, Input pull-down device connected"]
    #[inline(always)]
    pub fn const_0001(self) -> &'a mut crate::W<REG> {
        self.variant(Hibio1sel::Const0001)
    }
    #[doc = "Direct input, Input pull-up device connected"]
    #[inline(always)]
    pub fn const_0010(self) -> &'a mut crate::W<REG> {
        self.variant(Hibio1sel::Const0010)
    }
    #[doc = "Push-pull HIB Control output"]
    #[inline(always)]
    pub fn const_1000(self) -> &'a mut crate::W<REG> {
        self.variant(Hibio1sel::Const1000)
    }
    #[doc = "Push-pull WDT service output"]
    #[inline(always)]
    pub fn const_1001(self) -> &'a mut crate::W<REG> {
        self.variant(Hibio1sel::Const1001)
    }
    #[doc = "Push-pull GPIO output"]
    #[inline(always)]
    pub fn const_1010(self) -> &'a mut crate::W<REG> {
        self.variant(Hibio1sel::Const1010)
    }
    #[doc = "Open-drain HIB Control output"]
    #[inline(always)]
    pub fn const_1100(self) -> &'a mut crate::W<REG> {
        self.variant(Hibio1sel::Const1100)
    }
    #[doc = "Open-drain WDT service output"]
    #[inline(always)]
    pub fn const_1101(self) -> &'a mut crate::W<REG> {
        self.variant(Hibio1sel::Const1101)
    }
    #[doc = "Open-drain GPIO output"]
    #[inline(always)]
    pub fn const_1110(self) -> &'a mut crate::W<REG> {
        self.variant(Hibio1sel::Const1110)
    }
    #[doc = "#1111"]
    #[inline(always)]
    pub fn const_1111(self) -> &'a mut crate::W<REG> {
        self.variant(Hibio1sel::Const1111)
    }
}
impl R {
    #[doc = "Bit 0 - Wake-Up on Pin Event Positive Edge Enable"]
    #[inline(always)]
    pub fn wkpep(&self) -> WkpepR {
        WkpepR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Wake-up on Pin Event Negative Edge Enable"]
    #[inline(always)]
    pub fn wkpen(&self) -> WkpenR {
        WkpenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Wake-up on RTC Event Enable"]
    #[inline(always)]
    pub fn rtce(&self) -> RtceR {
        RtceR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - ULP WDG Alarm Enable"]
    #[inline(always)]
    pub fn ulpwdgen(&self) -> UlpwdgenR {
        UlpwdgenR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Hibernate Request Value Set"]
    #[inline(always)]
    pub fn hib(&self) -> HibR {
        HibR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - fRTC Clock Selection"]
    #[inline(always)]
    pub fn rcs(&self) -> RcsR {
        RcsR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - fSTDBY Clock Selection"]
    #[inline(always)]
    pub fn stdbysel(&self) -> StdbyselR {
        StdbyselR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Wake-Up from Hibernate Trigger Input Selection"]
    #[inline(always)]
    pub fn wkupsel(&self) -> WkupselR {
        WkupselR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 10 - General Purpose Input 0 Selection"]
    #[inline(always)]
    pub fn gpi0sel(&self) -> Gpi0selR {
        Gpi0selR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 12 - HIBIO0 Polarity Set"]
    #[inline(always)]
    pub fn hibio0pol(&self) -> Hibio0polR {
        Hibio0polR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - HIBIO1 Polarity Set"]
    #[inline(always)]
    pub fn hibio1pol(&self) -> Hibio1polR {
        Hibio1polR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 16:19 - HIB_IO_0 Pin I/O Control (default HIBOUT)"]
    #[inline(always)]
    pub fn hibio0sel(&self) -> Hibio0selR {
        Hibio0selR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - HIB_IO_1 Pin I/O Control (Default WKUP)"]
    #[inline(always)]
    pub fn hibio1sel(&self) -> Hibio1selR {
        Hibio1selR::new(((self.bits >> 20) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Wake-Up on Pin Event Positive Edge Enable"]
    #[inline(always)]
    #[must_use]
    pub fn wkpep(&mut self) -> WkpepW<HdcrSpec> {
        WkpepW::new(self, 0)
    }
    #[doc = "Bit 1 - Wake-up on Pin Event Negative Edge Enable"]
    #[inline(always)]
    #[must_use]
    pub fn wkpen(&mut self) -> WkpenW<HdcrSpec> {
        WkpenW::new(self, 1)
    }
    #[doc = "Bit 2 - Wake-up on RTC Event Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rtce(&mut self) -> RtceW<HdcrSpec> {
        RtceW::new(self, 2)
    }
    #[doc = "Bit 3 - ULP WDG Alarm Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ulpwdgen(&mut self) -> UlpwdgenW<HdcrSpec> {
        UlpwdgenW::new(self, 3)
    }
    #[doc = "Bit 4 - Hibernate Request Value Set"]
    #[inline(always)]
    #[must_use]
    pub fn hib(&mut self) -> HibW<HdcrSpec> {
        HibW::new(self, 4)
    }
    #[doc = "Bit 6 - fRTC Clock Selection"]
    #[inline(always)]
    #[must_use]
    pub fn rcs(&mut self) -> RcsW<HdcrSpec> {
        RcsW::new(self, 6)
    }
    #[doc = "Bit 7 - fSTDBY Clock Selection"]
    #[inline(always)]
    #[must_use]
    pub fn stdbysel(&mut self) -> StdbyselW<HdcrSpec> {
        StdbyselW::new(self, 7)
    }
    #[doc = "Bit 8 - Wake-Up from Hibernate Trigger Input Selection"]
    #[inline(always)]
    #[must_use]
    pub fn wkupsel(&mut self) -> WkupselW<HdcrSpec> {
        WkupselW::new(self, 8)
    }
    #[doc = "Bit 10 - General Purpose Input 0 Selection"]
    #[inline(always)]
    #[must_use]
    pub fn gpi0sel(&mut self) -> Gpi0selW<HdcrSpec> {
        Gpi0selW::new(self, 10)
    }
    #[doc = "Bit 12 - HIBIO0 Polarity Set"]
    #[inline(always)]
    #[must_use]
    pub fn hibio0pol(&mut self) -> Hibio0polW<HdcrSpec> {
        Hibio0polW::new(self, 12)
    }
    #[doc = "Bit 13 - HIBIO1 Polarity Set"]
    #[inline(always)]
    #[must_use]
    pub fn hibio1pol(&mut self) -> Hibio1polW<HdcrSpec> {
        Hibio1polW::new(self, 13)
    }
    #[doc = "Bits 16:19 - HIB_IO_0 Pin I/O Control (default HIBOUT)"]
    #[inline(always)]
    #[must_use]
    pub fn hibio0sel(&mut self) -> Hibio0selW<HdcrSpec> {
        Hibio0selW::new(self, 16)
    }
    #[doc = "Bits 20:23 - HIB_IO_1 Pin I/O Control (Default WKUP)"]
    #[inline(always)]
    #[must_use]
    pub fn hibio1sel(&mut self) -> Hibio1selW<HdcrSpec> {
        Hibio1selW::new(self, 20)
    }
}
#[doc = "Hibernate Domain Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hdcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hdcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HdcrSpec;
impl crate::RegisterSpec for HdcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hdcr::R`](R) reader structure"]
impl crate::Readable for HdcrSpec {}
#[doc = "`write(|w| ..)` method takes [`hdcr::W`](W) writer structure"]
impl crate::Writable for HdcrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HDCR to value 0x000c_2000"]
impl crate::Resettable for HdcrSpec {
    const RESET_VALUE: u32 = 0x000c_2000;
}
