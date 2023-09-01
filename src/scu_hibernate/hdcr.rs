#[doc = "Register `HDCR` reader"]
pub type R = crate::R<HDCR_SPEC>;
#[doc = "Register `HDCR` writer"]
pub type W = crate::W<HDCR_SPEC>;
#[doc = "Field `WKPEP` reader - Wake-Up on Pin Event Positive Edge Enable"]
pub type WKPEP_R = crate::BitReader<WKPEP_A>;
#[doc = "Wake-Up on Pin Event Positive Edge Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WKPEP_A {
    #[doc = "0: Wake-up event disabled"]
    CONST_0 = 0,
    #[doc = "1: Wake-up event enabled"]
    CONST_1 = 1,
}
impl From<WKPEP_A> for bool {
    #[inline(always)]
    fn from(variant: WKPEP_A) -> Self {
        variant as u8 != 0
    }
}
impl WKPEP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WKPEP_A {
        match self.bits {
            false => WKPEP_A::CONST_0,
            true => WKPEP_A::CONST_1,
        }
    }
    #[doc = "Wake-up event disabled"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == WKPEP_A::CONST_0
    }
    #[doc = "Wake-up event enabled"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == WKPEP_A::CONST_1
    }
}
#[doc = "Field `WKPEP` writer - Wake-Up on Pin Event Positive Edge Enable"]
pub type WKPEP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, WKPEP_A>;
impl<'a, REG, const O: u8> WKPEP_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Wake-up event disabled"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(WKPEP_A::CONST_0)
    }
    #[doc = "Wake-up event enabled"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(WKPEP_A::CONST_1)
    }
}
#[doc = "Field `WKPEN` reader - Wake-up on Pin Event Negative Edge Enable"]
pub type WKPEN_R = crate::BitReader<WKPEN_A>;
#[doc = "Wake-up on Pin Event Negative Edge Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WKPEN_A {
    #[doc = "0: Wake-up event disabled"]
    CONST_0 = 0,
    #[doc = "1: Wake-up event enabled"]
    CONST_1 = 1,
}
impl From<WKPEN_A> for bool {
    #[inline(always)]
    fn from(variant: WKPEN_A) -> Self {
        variant as u8 != 0
    }
}
impl WKPEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WKPEN_A {
        match self.bits {
            false => WKPEN_A::CONST_0,
            true => WKPEN_A::CONST_1,
        }
    }
    #[doc = "Wake-up event disabled"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == WKPEN_A::CONST_0
    }
    #[doc = "Wake-up event enabled"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == WKPEN_A::CONST_1
    }
}
#[doc = "Field `WKPEN` writer - Wake-up on Pin Event Negative Edge Enable"]
pub type WKPEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, WKPEN_A>;
impl<'a, REG, const O: u8> WKPEN_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Wake-up event disabled"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(WKPEN_A::CONST_0)
    }
    #[doc = "Wake-up event enabled"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(WKPEN_A::CONST_1)
    }
}
#[doc = "Field `RTCE` reader - Wake-up on RTC Event Enable"]
pub type RTCE_R = crate::BitReader<RTCE_A>;
#[doc = "Wake-up on RTC Event Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RTCE_A {
    #[doc = "0: Wake-up event disabled"]
    CONST_0 = 0,
    #[doc = "1: Wake-up event enabled"]
    CONST_1 = 1,
}
impl From<RTCE_A> for bool {
    #[inline(always)]
    fn from(variant: RTCE_A) -> Self {
        variant as u8 != 0
    }
}
impl RTCE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RTCE_A {
        match self.bits {
            false => RTCE_A::CONST_0,
            true => RTCE_A::CONST_1,
        }
    }
    #[doc = "Wake-up event disabled"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == RTCE_A::CONST_0
    }
    #[doc = "Wake-up event enabled"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == RTCE_A::CONST_1
    }
}
#[doc = "Field `RTCE` writer - Wake-up on RTC Event Enable"]
pub type RTCE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, RTCE_A>;
impl<'a, REG, const O: u8> RTCE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Wake-up event disabled"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(RTCE_A::CONST_0)
    }
    #[doc = "Wake-up event enabled"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(RTCE_A::CONST_1)
    }
}
#[doc = "Field `ULPWDGEN` reader - ULP WDG Alarm Enable"]
pub type ULPWDGEN_R = crate::BitReader<ULPWDGEN_A>;
#[doc = "ULP WDG Alarm Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ULPWDGEN_A {
    #[doc = "0: Wake-up event disabled"]
    CONST_0 = 0,
    #[doc = "1: Wake-up event enabled"]
    CONST_1 = 1,
}
impl From<ULPWDGEN_A> for bool {
    #[inline(always)]
    fn from(variant: ULPWDGEN_A) -> Self {
        variant as u8 != 0
    }
}
impl ULPWDGEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ULPWDGEN_A {
        match self.bits {
            false => ULPWDGEN_A::CONST_0,
            true => ULPWDGEN_A::CONST_1,
        }
    }
    #[doc = "Wake-up event disabled"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == ULPWDGEN_A::CONST_0
    }
    #[doc = "Wake-up event enabled"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == ULPWDGEN_A::CONST_1
    }
}
#[doc = "Field `ULPWDGEN` writer - ULP WDG Alarm Enable"]
pub type ULPWDGEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, ULPWDGEN_A>;
impl<'a, REG, const O: u8> ULPWDGEN_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Wake-up event disabled"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(ULPWDGEN_A::CONST_0)
    }
    #[doc = "Wake-up event enabled"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(ULPWDGEN_A::CONST_1)
    }
}
#[doc = "Field `HIB` reader - Hibernate Request Value Set"]
pub type HIB_R = crate::BitReader<HIB_A>;
#[doc = "Hibernate Request Value Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HIB_A {
    #[doc = "0: External hibernate request inactive"]
    CONST_0 = 0,
    #[doc = "1: External hibernate request active"]
    CONST_1 = 1,
}
impl From<HIB_A> for bool {
    #[inline(always)]
    fn from(variant: HIB_A) -> Self {
        variant as u8 != 0
    }
}
impl HIB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HIB_A {
        match self.bits {
            false => HIB_A::CONST_0,
            true => HIB_A::CONST_1,
        }
    }
    #[doc = "External hibernate request inactive"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == HIB_A::CONST_0
    }
    #[doc = "External hibernate request active"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == HIB_A::CONST_1
    }
}
#[doc = "Field `HIB` writer - Hibernate Request Value Set"]
pub type HIB_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, HIB_A>;
impl<'a, REG, const O: u8> HIB_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "External hibernate request inactive"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(HIB_A::CONST_0)
    }
    #[doc = "External hibernate request active"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(HIB_A::CONST_1)
    }
}
#[doc = "Field `RCS` reader - fRTC Clock Selection"]
pub type RCS_R = crate::BitReader<RCS_A>;
#[doc = "fRTC Clock Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RCS_A {
    #[doc = "0: fOSI selected"]
    CONST_0 = 0,
    #[doc = "1: fULP selected"]
    CONST_1 = 1,
}
impl From<RCS_A> for bool {
    #[inline(always)]
    fn from(variant: RCS_A) -> Self {
        variant as u8 != 0
    }
}
impl RCS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RCS_A {
        match self.bits {
            false => RCS_A::CONST_0,
            true => RCS_A::CONST_1,
        }
    }
    #[doc = "fOSI selected"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == RCS_A::CONST_0
    }
    #[doc = "fULP selected"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == RCS_A::CONST_1
    }
}
#[doc = "Field `RCS` writer - fRTC Clock Selection"]
pub type RCS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, RCS_A>;
impl<'a, REG, const O: u8> RCS_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "fOSI selected"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(RCS_A::CONST_0)
    }
    #[doc = "fULP selected"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(RCS_A::CONST_1)
    }
}
#[doc = "Field `STDBYSEL` reader - fSTDBY Clock Selection"]
pub type STDBYSEL_R = crate::BitReader<STDBYSEL_A>;
#[doc = "fSTDBY Clock Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum STDBYSEL_A {
    #[doc = "0: fOSI selected"]
    CONST_0 = 0,
    #[doc = "1: fULP selected"]
    CONST_1 = 1,
}
impl From<STDBYSEL_A> for bool {
    #[inline(always)]
    fn from(variant: STDBYSEL_A) -> Self {
        variant as u8 != 0
    }
}
impl STDBYSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STDBYSEL_A {
        match self.bits {
            false => STDBYSEL_A::CONST_0,
            true => STDBYSEL_A::CONST_1,
        }
    }
    #[doc = "fOSI selected"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == STDBYSEL_A::CONST_0
    }
    #[doc = "fULP selected"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == STDBYSEL_A::CONST_1
    }
}
#[doc = "Field `STDBYSEL` writer - fSTDBY Clock Selection"]
pub type STDBYSEL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, STDBYSEL_A>;
impl<'a, REG, const O: u8> STDBYSEL_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "fOSI selected"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(STDBYSEL_A::CONST_0)
    }
    #[doc = "fULP selected"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(STDBYSEL_A::CONST_1)
    }
}
#[doc = "Field `WKUPSEL` reader - Wake-Up from Hibernate Trigger Input Selection"]
pub type WKUPSEL_R = crate::BitReader<WKUPSEL_A>;
#[doc = "Wake-Up from Hibernate Trigger Input Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WKUPSEL_A {
    #[doc = "0: HIB_IO_1 pin selected"]
    CONST_0 = 0,
    #[doc = "1: HIB_IO_0 pin selected"]
    CONST_1 = 1,
}
impl From<WKUPSEL_A> for bool {
    #[inline(always)]
    fn from(variant: WKUPSEL_A) -> Self {
        variant as u8 != 0
    }
}
impl WKUPSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WKUPSEL_A {
        match self.bits {
            false => WKUPSEL_A::CONST_0,
            true => WKUPSEL_A::CONST_1,
        }
    }
    #[doc = "HIB_IO_1 pin selected"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == WKUPSEL_A::CONST_0
    }
    #[doc = "HIB_IO_0 pin selected"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == WKUPSEL_A::CONST_1
    }
}
#[doc = "Field `WKUPSEL` writer - Wake-Up from Hibernate Trigger Input Selection"]
pub type WKUPSEL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, WKUPSEL_A>;
impl<'a, REG, const O: u8> WKUPSEL_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "HIB_IO_1 pin selected"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(WKUPSEL_A::CONST_0)
    }
    #[doc = "HIB_IO_0 pin selected"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(WKUPSEL_A::CONST_1)
    }
}
#[doc = "Field `GPI0SEL` reader - General Purpose Input 0 Selection"]
pub type GPI0SEL_R = crate::BitReader<GPI0SEL_A>;
#[doc = "General Purpose Input 0 Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPI0SEL_A {
    #[doc = "0: #0"]
    CONST_0 = 0,
    #[doc = "1: HIB_IO_0 pin selected"]
    CONST_1 = 1,
}
impl From<GPI0SEL_A> for bool {
    #[inline(always)]
    fn from(variant: GPI0SEL_A) -> Self {
        variant as u8 != 0
    }
}
impl GPI0SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPI0SEL_A {
        match self.bits {
            false => GPI0SEL_A::CONST_0,
            true => GPI0SEL_A::CONST_1,
        }
    }
    #[doc = "#0"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == GPI0SEL_A::CONST_0
    }
    #[doc = "HIB_IO_0 pin selected"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == GPI0SEL_A::CONST_1
    }
}
#[doc = "Field `GPI0SEL` writer - General Purpose Input 0 Selection"]
pub type GPI0SEL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, GPI0SEL_A>;
impl<'a, REG, const O: u8> GPI0SEL_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "#0"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(GPI0SEL_A::CONST_0)
    }
    #[doc = "HIB_IO_0 pin selected"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(GPI0SEL_A::CONST_1)
    }
}
#[doc = "Field `HIBIO0POL` reader - HIBIO0 Polarity Set"]
pub type HIBIO0POL_R = crate::BitReader<HIBIO0POL_A>;
#[doc = "HIBIO0 Polarity Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HIBIO0POL_A {
    #[doc = "0: Direct value"]
    CONST_0 = 0,
    #[doc = "1: Inverted value"]
    CONST_1 = 1,
}
impl From<HIBIO0POL_A> for bool {
    #[inline(always)]
    fn from(variant: HIBIO0POL_A) -> Self {
        variant as u8 != 0
    }
}
impl HIBIO0POL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HIBIO0POL_A {
        match self.bits {
            false => HIBIO0POL_A::CONST_0,
            true => HIBIO0POL_A::CONST_1,
        }
    }
    #[doc = "Direct value"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == HIBIO0POL_A::CONST_0
    }
    #[doc = "Inverted value"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == HIBIO0POL_A::CONST_1
    }
}
#[doc = "Field `HIBIO0POL` writer - HIBIO0 Polarity Set"]
pub type HIBIO0POL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, HIBIO0POL_A>;
impl<'a, REG, const O: u8> HIBIO0POL_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Direct value"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(HIBIO0POL_A::CONST_0)
    }
    #[doc = "Inverted value"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(HIBIO0POL_A::CONST_1)
    }
}
#[doc = "Field `HIBIO1POL` reader - HIBIO1 Polarity Set"]
pub type HIBIO1POL_R = crate::BitReader<HIBIO1POL_A>;
#[doc = "HIBIO1 Polarity Set\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HIBIO1POL_A {
    #[doc = "0: Direct value"]
    CONST_0 = 0,
    #[doc = "1: Inverted value"]
    CONST_1 = 1,
}
impl From<HIBIO1POL_A> for bool {
    #[inline(always)]
    fn from(variant: HIBIO1POL_A) -> Self {
        variant as u8 != 0
    }
}
impl HIBIO1POL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HIBIO1POL_A {
        match self.bits {
            false => HIBIO1POL_A::CONST_0,
            true => HIBIO1POL_A::CONST_1,
        }
    }
    #[doc = "Direct value"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == HIBIO1POL_A::CONST_0
    }
    #[doc = "Inverted value"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == HIBIO1POL_A::CONST_1
    }
}
#[doc = "Field `HIBIO1POL` writer - HIBIO1 Polarity Set"]
pub type HIBIO1POL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, HIBIO1POL_A>;
impl<'a, REG, const O: u8> HIBIO1POL_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Direct value"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(HIBIO1POL_A::CONST_0)
    }
    #[doc = "Inverted value"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(HIBIO1POL_A::CONST_1)
    }
}
#[doc = "Field `HIBIO0SEL` reader - HIB_IO_0 Pin I/O Control (default HIBOUT)"]
pub type HIBIO0SEL_R = crate::FieldReader<HIBIO0SEL_A>;
#[doc = "HIB_IO_0 Pin I/O Control (default HIBOUT)\n\nValue on reset: 12"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum HIBIO0SEL_A {
    #[doc = "0: Direct input, No input pull device connected"]
    CONST_0000 = 0,
    #[doc = "1: Direct input, Input pull-down device connected"]
    CONST_0001 = 1,
    #[doc = "2: Direct input, Input pull-up device connected"]
    CONST_0010 = 2,
    #[doc = "8: Push-pull HIB Control output"]
    CONST_1000 = 8,
    #[doc = "9: Push-pull WDT service output"]
    CONST_1001 = 9,
    #[doc = "10: Push-pull GPIO output"]
    CONST_1010 = 10,
    #[doc = "12: Open-drain HIB Control output"]
    CONST_1100 = 12,
    #[doc = "13: Open-drain WDT service output"]
    CONST_1101 = 13,
    #[doc = "14: Open-drain GPIO output"]
    CONST_1110 = 14,
    #[doc = "15: #1111"]
    CONST_1111 = 15,
}
impl From<HIBIO0SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: HIBIO0SEL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for HIBIO0SEL_A {
    type Ux = u8;
}
impl HIBIO0SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<HIBIO0SEL_A> {
        match self.bits {
            0 => Some(HIBIO0SEL_A::CONST_0000),
            1 => Some(HIBIO0SEL_A::CONST_0001),
            2 => Some(HIBIO0SEL_A::CONST_0010),
            8 => Some(HIBIO0SEL_A::CONST_1000),
            9 => Some(HIBIO0SEL_A::CONST_1001),
            10 => Some(HIBIO0SEL_A::CONST_1010),
            12 => Some(HIBIO0SEL_A::CONST_1100),
            13 => Some(HIBIO0SEL_A::CONST_1101),
            14 => Some(HIBIO0SEL_A::CONST_1110),
            15 => Some(HIBIO0SEL_A::CONST_1111),
            _ => None,
        }
    }
    #[doc = "Direct input, No input pull device connected"]
    #[inline(always)]
    pub fn is_const_0000(&self) -> bool {
        *self == HIBIO0SEL_A::CONST_0000
    }
    #[doc = "Direct input, Input pull-down device connected"]
    #[inline(always)]
    pub fn is_const_0001(&self) -> bool {
        *self == HIBIO0SEL_A::CONST_0001
    }
    #[doc = "Direct input, Input pull-up device connected"]
    #[inline(always)]
    pub fn is_const_0010(&self) -> bool {
        *self == HIBIO0SEL_A::CONST_0010
    }
    #[doc = "Push-pull HIB Control output"]
    #[inline(always)]
    pub fn is_const_1000(&self) -> bool {
        *self == HIBIO0SEL_A::CONST_1000
    }
    #[doc = "Push-pull WDT service output"]
    #[inline(always)]
    pub fn is_const_1001(&self) -> bool {
        *self == HIBIO0SEL_A::CONST_1001
    }
    #[doc = "Push-pull GPIO output"]
    #[inline(always)]
    pub fn is_const_1010(&self) -> bool {
        *self == HIBIO0SEL_A::CONST_1010
    }
    #[doc = "Open-drain HIB Control output"]
    #[inline(always)]
    pub fn is_const_1100(&self) -> bool {
        *self == HIBIO0SEL_A::CONST_1100
    }
    #[doc = "Open-drain WDT service output"]
    #[inline(always)]
    pub fn is_const_1101(&self) -> bool {
        *self == HIBIO0SEL_A::CONST_1101
    }
    #[doc = "Open-drain GPIO output"]
    #[inline(always)]
    pub fn is_const_1110(&self) -> bool {
        *self == HIBIO0SEL_A::CONST_1110
    }
    #[doc = "#1111"]
    #[inline(always)]
    pub fn is_const_1111(&self) -> bool {
        *self == HIBIO0SEL_A::CONST_1111
    }
}
#[doc = "Field `HIBIO0SEL` writer - HIB_IO_0 Pin I/O Control (default HIBOUT)"]
pub type HIBIO0SEL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O, HIBIO0SEL_A>;
impl<'a, REG, const O: u8> HIBIO0SEL_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Direct input, No input pull device connected"]
    #[inline(always)]
    pub fn const_0000(self) -> &'a mut crate::W<REG> {
        self.variant(HIBIO0SEL_A::CONST_0000)
    }
    #[doc = "Direct input, Input pull-down device connected"]
    #[inline(always)]
    pub fn const_0001(self) -> &'a mut crate::W<REG> {
        self.variant(HIBIO0SEL_A::CONST_0001)
    }
    #[doc = "Direct input, Input pull-up device connected"]
    #[inline(always)]
    pub fn const_0010(self) -> &'a mut crate::W<REG> {
        self.variant(HIBIO0SEL_A::CONST_0010)
    }
    #[doc = "Push-pull HIB Control output"]
    #[inline(always)]
    pub fn const_1000(self) -> &'a mut crate::W<REG> {
        self.variant(HIBIO0SEL_A::CONST_1000)
    }
    #[doc = "Push-pull WDT service output"]
    #[inline(always)]
    pub fn const_1001(self) -> &'a mut crate::W<REG> {
        self.variant(HIBIO0SEL_A::CONST_1001)
    }
    #[doc = "Push-pull GPIO output"]
    #[inline(always)]
    pub fn const_1010(self) -> &'a mut crate::W<REG> {
        self.variant(HIBIO0SEL_A::CONST_1010)
    }
    #[doc = "Open-drain HIB Control output"]
    #[inline(always)]
    pub fn const_1100(self) -> &'a mut crate::W<REG> {
        self.variant(HIBIO0SEL_A::CONST_1100)
    }
    #[doc = "Open-drain WDT service output"]
    #[inline(always)]
    pub fn const_1101(self) -> &'a mut crate::W<REG> {
        self.variant(HIBIO0SEL_A::CONST_1101)
    }
    #[doc = "Open-drain GPIO output"]
    #[inline(always)]
    pub fn const_1110(self) -> &'a mut crate::W<REG> {
        self.variant(HIBIO0SEL_A::CONST_1110)
    }
    #[doc = "#1111"]
    #[inline(always)]
    pub fn const_1111(self) -> &'a mut crate::W<REG> {
        self.variant(HIBIO0SEL_A::CONST_1111)
    }
}
#[doc = "Field `HIBIO1SEL` reader - HIB_IO_1 Pin I/O Control (Default WKUP)"]
pub type HIBIO1SEL_R = crate::FieldReader<HIBIO1SEL_A>;
#[doc = "HIB_IO_1 Pin I/O Control (Default WKUP)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum HIBIO1SEL_A {
    #[doc = "0: Direct input, No input pull device connected"]
    CONST_0000 = 0,
    #[doc = "1: Direct input, Input pull-down device connected"]
    CONST_0001 = 1,
    #[doc = "2: Direct input, Input pull-up device connected"]
    CONST_0010 = 2,
    #[doc = "8: Push-pull HIB Control output"]
    CONST_1000 = 8,
    #[doc = "9: Push-pull WDT service output"]
    CONST_1001 = 9,
    #[doc = "10: Push-pull GPIO output"]
    CONST_1010 = 10,
    #[doc = "12: Open-drain HIB Control output"]
    CONST_1100 = 12,
    #[doc = "13: Open-drain WDT service output"]
    CONST_1101 = 13,
    #[doc = "14: Open-drain GPIO output"]
    CONST_1110 = 14,
    #[doc = "15: #1111"]
    CONST_1111 = 15,
}
impl From<HIBIO1SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: HIBIO1SEL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for HIBIO1SEL_A {
    type Ux = u8;
}
impl HIBIO1SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<HIBIO1SEL_A> {
        match self.bits {
            0 => Some(HIBIO1SEL_A::CONST_0000),
            1 => Some(HIBIO1SEL_A::CONST_0001),
            2 => Some(HIBIO1SEL_A::CONST_0010),
            8 => Some(HIBIO1SEL_A::CONST_1000),
            9 => Some(HIBIO1SEL_A::CONST_1001),
            10 => Some(HIBIO1SEL_A::CONST_1010),
            12 => Some(HIBIO1SEL_A::CONST_1100),
            13 => Some(HIBIO1SEL_A::CONST_1101),
            14 => Some(HIBIO1SEL_A::CONST_1110),
            15 => Some(HIBIO1SEL_A::CONST_1111),
            _ => None,
        }
    }
    #[doc = "Direct input, No input pull device connected"]
    #[inline(always)]
    pub fn is_const_0000(&self) -> bool {
        *self == HIBIO1SEL_A::CONST_0000
    }
    #[doc = "Direct input, Input pull-down device connected"]
    #[inline(always)]
    pub fn is_const_0001(&self) -> bool {
        *self == HIBIO1SEL_A::CONST_0001
    }
    #[doc = "Direct input, Input pull-up device connected"]
    #[inline(always)]
    pub fn is_const_0010(&self) -> bool {
        *self == HIBIO1SEL_A::CONST_0010
    }
    #[doc = "Push-pull HIB Control output"]
    #[inline(always)]
    pub fn is_const_1000(&self) -> bool {
        *self == HIBIO1SEL_A::CONST_1000
    }
    #[doc = "Push-pull WDT service output"]
    #[inline(always)]
    pub fn is_const_1001(&self) -> bool {
        *self == HIBIO1SEL_A::CONST_1001
    }
    #[doc = "Push-pull GPIO output"]
    #[inline(always)]
    pub fn is_const_1010(&self) -> bool {
        *self == HIBIO1SEL_A::CONST_1010
    }
    #[doc = "Open-drain HIB Control output"]
    #[inline(always)]
    pub fn is_const_1100(&self) -> bool {
        *self == HIBIO1SEL_A::CONST_1100
    }
    #[doc = "Open-drain WDT service output"]
    #[inline(always)]
    pub fn is_const_1101(&self) -> bool {
        *self == HIBIO1SEL_A::CONST_1101
    }
    #[doc = "Open-drain GPIO output"]
    #[inline(always)]
    pub fn is_const_1110(&self) -> bool {
        *self == HIBIO1SEL_A::CONST_1110
    }
    #[doc = "#1111"]
    #[inline(always)]
    pub fn is_const_1111(&self) -> bool {
        *self == HIBIO1SEL_A::CONST_1111
    }
}
#[doc = "Field `HIBIO1SEL` writer - HIB_IO_1 Pin I/O Control (Default WKUP)"]
pub type HIBIO1SEL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O, HIBIO1SEL_A>;
impl<'a, REG, const O: u8> HIBIO1SEL_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Direct input, No input pull device connected"]
    #[inline(always)]
    pub fn const_0000(self) -> &'a mut crate::W<REG> {
        self.variant(HIBIO1SEL_A::CONST_0000)
    }
    #[doc = "Direct input, Input pull-down device connected"]
    #[inline(always)]
    pub fn const_0001(self) -> &'a mut crate::W<REG> {
        self.variant(HIBIO1SEL_A::CONST_0001)
    }
    #[doc = "Direct input, Input pull-up device connected"]
    #[inline(always)]
    pub fn const_0010(self) -> &'a mut crate::W<REG> {
        self.variant(HIBIO1SEL_A::CONST_0010)
    }
    #[doc = "Push-pull HIB Control output"]
    #[inline(always)]
    pub fn const_1000(self) -> &'a mut crate::W<REG> {
        self.variant(HIBIO1SEL_A::CONST_1000)
    }
    #[doc = "Push-pull WDT service output"]
    #[inline(always)]
    pub fn const_1001(self) -> &'a mut crate::W<REG> {
        self.variant(HIBIO1SEL_A::CONST_1001)
    }
    #[doc = "Push-pull GPIO output"]
    #[inline(always)]
    pub fn const_1010(self) -> &'a mut crate::W<REG> {
        self.variant(HIBIO1SEL_A::CONST_1010)
    }
    #[doc = "Open-drain HIB Control output"]
    #[inline(always)]
    pub fn const_1100(self) -> &'a mut crate::W<REG> {
        self.variant(HIBIO1SEL_A::CONST_1100)
    }
    #[doc = "Open-drain WDT service output"]
    #[inline(always)]
    pub fn const_1101(self) -> &'a mut crate::W<REG> {
        self.variant(HIBIO1SEL_A::CONST_1101)
    }
    #[doc = "Open-drain GPIO output"]
    #[inline(always)]
    pub fn const_1110(self) -> &'a mut crate::W<REG> {
        self.variant(HIBIO1SEL_A::CONST_1110)
    }
    #[doc = "#1111"]
    #[inline(always)]
    pub fn const_1111(self) -> &'a mut crate::W<REG> {
        self.variant(HIBIO1SEL_A::CONST_1111)
    }
}
impl R {
    #[doc = "Bit 0 - Wake-Up on Pin Event Positive Edge Enable"]
    #[inline(always)]
    pub fn wkpep(&self) -> WKPEP_R {
        WKPEP_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Wake-up on Pin Event Negative Edge Enable"]
    #[inline(always)]
    pub fn wkpen(&self) -> WKPEN_R {
        WKPEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Wake-up on RTC Event Enable"]
    #[inline(always)]
    pub fn rtce(&self) -> RTCE_R {
        RTCE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - ULP WDG Alarm Enable"]
    #[inline(always)]
    pub fn ulpwdgen(&self) -> ULPWDGEN_R {
        ULPWDGEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Hibernate Request Value Set"]
    #[inline(always)]
    pub fn hib(&self) -> HIB_R {
        HIB_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - fRTC Clock Selection"]
    #[inline(always)]
    pub fn rcs(&self) -> RCS_R {
        RCS_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - fSTDBY Clock Selection"]
    #[inline(always)]
    pub fn stdbysel(&self) -> STDBYSEL_R {
        STDBYSEL_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Wake-Up from Hibernate Trigger Input Selection"]
    #[inline(always)]
    pub fn wkupsel(&self) -> WKUPSEL_R {
        WKUPSEL_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 10 - General Purpose Input 0 Selection"]
    #[inline(always)]
    pub fn gpi0sel(&self) -> GPI0SEL_R {
        GPI0SEL_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 12 - HIBIO0 Polarity Set"]
    #[inline(always)]
    pub fn hibio0pol(&self) -> HIBIO0POL_R {
        HIBIO0POL_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - HIBIO1 Polarity Set"]
    #[inline(always)]
    pub fn hibio1pol(&self) -> HIBIO1POL_R {
        HIBIO1POL_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 16:19 - HIB_IO_0 Pin I/O Control (default HIBOUT)"]
    #[inline(always)]
    pub fn hibio0sel(&self) -> HIBIO0SEL_R {
        HIBIO0SEL_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - HIB_IO_1 Pin I/O Control (Default WKUP)"]
    #[inline(always)]
    pub fn hibio1sel(&self) -> HIBIO1SEL_R {
        HIBIO1SEL_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Wake-Up on Pin Event Positive Edge Enable"]
    #[inline(always)]
    #[must_use]
    pub fn wkpep(&mut self) -> WKPEP_W<HDCR_SPEC, 0> {
        WKPEP_W::new(self)
    }
    #[doc = "Bit 1 - Wake-up on Pin Event Negative Edge Enable"]
    #[inline(always)]
    #[must_use]
    pub fn wkpen(&mut self) -> WKPEN_W<HDCR_SPEC, 1> {
        WKPEN_W::new(self)
    }
    #[doc = "Bit 2 - Wake-up on RTC Event Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rtce(&mut self) -> RTCE_W<HDCR_SPEC, 2> {
        RTCE_W::new(self)
    }
    #[doc = "Bit 3 - ULP WDG Alarm Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ulpwdgen(&mut self) -> ULPWDGEN_W<HDCR_SPEC, 3> {
        ULPWDGEN_W::new(self)
    }
    #[doc = "Bit 4 - Hibernate Request Value Set"]
    #[inline(always)]
    #[must_use]
    pub fn hib(&mut self) -> HIB_W<HDCR_SPEC, 4> {
        HIB_W::new(self)
    }
    #[doc = "Bit 6 - fRTC Clock Selection"]
    #[inline(always)]
    #[must_use]
    pub fn rcs(&mut self) -> RCS_W<HDCR_SPEC, 6> {
        RCS_W::new(self)
    }
    #[doc = "Bit 7 - fSTDBY Clock Selection"]
    #[inline(always)]
    #[must_use]
    pub fn stdbysel(&mut self) -> STDBYSEL_W<HDCR_SPEC, 7> {
        STDBYSEL_W::new(self)
    }
    #[doc = "Bit 8 - Wake-Up from Hibernate Trigger Input Selection"]
    #[inline(always)]
    #[must_use]
    pub fn wkupsel(&mut self) -> WKUPSEL_W<HDCR_SPEC, 8> {
        WKUPSEL_W::new(self)
    }
    #[doc = "Bit 10 - General Purpose Input 0 Selection"]
    #[inline(always)]
    #[must_use]
    pub fn gpi0sel(&mut self) -> GPI0SEL_W<HDCR_SPEC, 10> {
        GPI0SEL_W::new(self)
    }
    #[doc = "Bit 12 - HIBIO0 Polarity Set"]
    #[inline(always)]
    #[must_use]
    pub fn hibio0pol(&mut self) -> HIBIO0POL_W<HDCR_SPEC, 12> {
        HIBIO0POL_W::new(self)
    }
    #[doc = "Bit 13 - HIBIO1 Polarity Set"]
    #[inline(always)]
    #[must_use]
    pub fn hibio1pol(&mut self) -> HIBIO1POL_W<HDCR_SPEC, 13> {
        HIBIO1POL_W::new(self)
    }
    #[doc = "Bits 16:19 - HIB_IO_0 Pin I/O Control (default HIBOUT)"]
    #[inline(always)]
    #[must_use]
    pub fn hibio0sel(&mut self) -> HIBIO0SEL_W<HDCR_SPEC, 16> {
        HIBIO0SEL_W::new(self)
    }
    #[doc = "Bits 20:23 - HIB_IO_1 Pin I/O Control (Default WKUP)"]
    #[inline(always)]
    #[must_use]
    pub fn hibio1sel(&mut self) -> HIBIO1SEL_W<HDCR_SPEC, 20> {
        HIBIO1SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Hibernate Domain Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hdcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hdcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HDCR_SPEC;
impl crate::RegisterSpec for HDCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hdcr::R`](R) reader structure"]
impl crate::Readable for HDCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hdcr::W`](W) writer structure"]
impl crate::Writable for HDCR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HDCR to value 0x000c_2000"]
impl crate::Resettable for HDCR_SPEC {
    const RESET_VALUE: Self::Ux = 0x000c_2000;
}
