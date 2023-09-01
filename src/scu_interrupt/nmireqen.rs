#[doc = "Register `NMIREQEN` reader"]
pub type R = crate::R<NMIREQEN_SPEC>;
#[doc = "Register `NMIREQEN` writer"]
pub type W = crate::W<NMIREQEN_SPEC>;
#[doc = "Field `PRWARN` reader - Promote Pre-Warning Interrupt Request to NMI Request"]
pub type PRWARN_R = crate::BitReader<PRWARN_A>;
#[doc = "Promote Pre-Warning Interrupt Request to NMI Request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PRWARN_A {
    #[doc = "0: Disabled"]
    CONST_0 = 0,
    #[doc = "1: Enabled"]
    CONST_1 = 1,
}
impl From<PRWARN_A> for bool {
    #[inline(always)]
    fn from(variant: PRWARN_A) -> Self {
        variant as u8 != 0
    }
}
impl PRWARN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PRWARN_A {
        match self.bits {
            false => PRWARN_A::CONST_0,
            true => PRWARN_A::CONST_1,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == PRWARN_A::CONST_0
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == PRWARN_A::CONST_1
    }
}
#[doc = "Field `PRWARN` writer - Promote Pre-Warning Interrupt Request to NMI Request"]
pub type PRWARN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, PRWARN_A>;
impl<'a, REG, const O: u8> PRWARN_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(PRWARN_A::CONST_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(PRWARN_A::CONST_1)
    }
}
#[doc = "Field `PI` reader - Promote RTC Periodic Interrupt request to NMI Request"]
pub type PI_R = crate::BitReader<PI_A>;
#[doc = "Promote RTC Periodic Interrupt request to NMI Request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PI_A {
    #[doc = "0: Disabled"]
    CONST_0 = 0,
    #[doc = "1: Enabled"]
    CONST_1 = 1,
}
impl From<PI_A> for bool {
    #[inline(always)]
    fn from(variant: PI_A) -> Self {
        variant as u8 != 0
    }
}
impl PI_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PI_A {
        match self.bits {
            false => PI_A::CONST_0,
            true => PI_A::CONST_1,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == PI_A::CONST_0
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == PI_A::CONST_1
    }
}
#[doc = "Field `PI` writer - Promote RTC Periodic Interrupt request to NMI Request"]
pub type PI_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, PI_A>;
impl<'a, REG, const O: u8> PI_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(PI_A::CONST_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(PI_A::CONST_1)
    }
}
#[doc = "Field `AI` reader - Promote RTC Alarm Interrupt Request to NMI Request"]
pub type AI_R = crate::BitReader<AI_A>;
#[doc = "Promote RTC Alarm Interrupt Request to NMI Request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AI_A {
    #[doc = "0: Disabled"]
    CONST_0 = 0,
    #[doc = "1: Enabled"]
    CONST_1 = 1,
}
impl From<AI_A> for bool {
    #[inline(always)]
    fn from(variant: AI_A) -> Self {
        variant as u8 != 0
    }
}
impl AI_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AI_A {
        match self.bits {
            false => AI_A::CONST_0,
            true => AI_A::CONST_1,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == AI_A::CONST_0
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == AI_A::CONST_1
    }
}
#[doc = "Field `AI` writer - Promote RTC Alarm Interrupt Request to NMI Request"]
pub type AI_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, AI_A>;
impl<'a, REG, const O: u8> AI_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(AI_A::CONST_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(AI_A::CONST_1)
    }
}
#[doc = "Field `ERU00` reader - Promote Channel 0 Interrupt of ERU0 Request to NMI Request"]
pub type ERU00_R = crate::BitReader<ERU00_A>;
#[doc = "Promote Channel 0 Interrupt of ERU0 Request to NMI Request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ERU00_A {
    #[doc = "0: Disabled"]
    CONST_0 = 0,
    #[doc = "1: Enabled"]
    CONST_1 = 1,
}
impl From<ERU00_A> for bool {
    #[inline(always)]
    fn from(variant: ERU00_A) -> Self {
        variant as u8 != 0
    }
}
impl ERU00_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERU00_A {
        match self.bits {
            false => ERU00_A::CONST_0,
            true => ERU00_A::CONST_1,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == ERU00_A::CONST_0
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == ERU00_A::CONST_1
    }
}
#[doc = "Field `ERU00` writer - Promote Channel 0 Interrupt of ERU0 Request to NMI Request"]
pub type ERU00_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, ERU00_A>;
impl<'a, REG, const O: u8> ERU00_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(ERU00_A::CONST_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(ERU00_A::CONST_1)
    }
}
#[doc = "Field `ERU01` reader - Promote Channel 1 Interrupt of ERU0 Request to NMI Request"]
pub type ERU01_R = crate::BitReader<ERU01_A>;
#[doc = "Promote Channel 1 Interrupt of ERU0 Request to NMI Request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ERU01_A {
    #[doc = "0: Disabled"]
    CONST_0 = 0,
    #[doc = "1: Enabled"]
    CONST_1 = 1,
}
impl From<ERU01_A> for bool {
    #[inline(always)]
    fn from(variant: ERU01_A) -> Self {
        variant as u8 != 0
    }
}
impl ERU01_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERU01_A {
        match self.bits {
            false => ERU01_A::CONST_0,
            true => ERU01_A::CONST_1,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == ERU01_A::CONST_0
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == ERU01_A::CONST_1
    }
}
#[doc = "Field `ERU01` writer - Promote Channel 1 Interrupt of ERU0 Request to NMI Request"]
pub type ERU01_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, ERU01_A>;
impl<'a, REG, const O: u8> ERU01_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(ERU01_A::CONST_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(ERU01_A::CONST_1)
    }
}
#[doc = "Field `ERU02` reader - Promote Channel 2 Interrupt of ERU0 Request to NMI Request"]
pub type ERU02_R = crate::BitReader<ERU02_A>;
#[doc = "Promote Channel 2 Interrupt of ERU0 Request to NMI Request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ERU02_A {
    #[doc = "0: Disabled"]
    CONST_0 = 0,
    #[doc = "1: Enabled"]
    CONST_1 = 1,
}
impl From<ERU02_A> for bool {
    #[inline(always)]
    fn from(variant: ERU02_A) -> Self {
        variant as u8 != 0
    }
}
impl ERU02_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERU02_A {
        match self.bits {
            false => ERU02_A::CONST_0,
            true => ERU02_A::CONST_1,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == ERU02_A::CONST_0
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == ERU02_A::CONST_1
    }
}
#[doc = "Field `ERU02` writer - Promote Channel 2 Interrupt of ERU0 Request to NMI Request"]
pub type ERU02_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, ERU02_A>;
impl<'a, REG, const O: u8> ERU02_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(ERU02_A::CONST_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(ERU02_A::CONST_1)
    }
}
#[doc = "Field `ERU03` reader - Promote Channel 3 Interrupt of ERU0 Request to NMI Request"]
pub type ERU03_R = crate::BitReader<ERU03_A>;
#[doc = "Promote Channel 3 Interrupt of ERU0 Request to NMI Request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ERU03_A {
    #[doc = "0: Disabled"]
    CONST_0 = 0,
    #[doc = "1: Enabled"]
    CONST_1 = 1,
}
impl From<ERU03_A> for bool {
    #[inline(always)]
    fn from(variant: ERU03_A) -> Self {
        variant as u8 != 0
    }
}
impl ERU03_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERU03_A {
        match self.bits {
            false => ERU03_A::CONST_0,
            true => ERU03_A::CONST_1,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == ERU03_A::CONST_0
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == ERU03_A::CONST_1
    }
}
#[doc = "Field `ERU03` writer - Promote Channel 3 Interrupt of ERU0 Request to NMI Request"]
pub type ERU03_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, ERU03_A>;
impl<'a, REG, const O: u8> ERU03_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(ERU03_A::CONST_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(ERU03_A::CONST_1)
    }
}
impl R {
    #[doc = "Bit 0 - Promote Pre-Warning Interrupt Request to NMI Request"]
    #[inline(always)]
    pub fn prwarn(&self) -> PRWARN_R {
        PRWARN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Promote RTC Periodic Interrupt request to NMI Request"]
    #[inline(always)]
    pub fn pi(&self) -> PI_R {
        PI_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Promote RTC Alarm Interrupt Request to NMI Request"]
    #[inline(always)]
    pub fn ai(&self) -> AI_R {
        AI_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 16 - Promote Channel 0 Interrupt of ERU0 Request to NMI Request"]
    #[inline(always)]
    pub fn eru00(&self) -> ERU00_R {
        ERU00_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Promote Channel 1 Interrupt of ERU0 Request to NMI Request"]
    #[inline(always)]
    pub fn eru01(&self) -> ERU01_R {
        ERU01_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Promote Channel 2 Interrupt of ERU0 Request to NMI Request"]
    #[inline(always)]
    pub fn eru02(&self) -> ERU02_R {
        ERU02_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Promote Channel 3 Interrupt of ERU0 Request to NMI Request"]
    #[inline(always)]
    pub fn eru03(&self) -> ERU03_R {
        ERU03_R::new(((self.bits >> 19) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Promote Pre-Warning Interrupt Request to NMI Request"]
    #[inline(always)]
    #[must_use]
    pub fn prwarn(&mut self) -> PRWARN_W<NMIREQEN_SPEC, 0> {
        PRWARN_W::new(self)
    }
    #[doc = "Bit 1 - Promote RTC Periodic Interrupt request to NMI Request"]
    #[inline(always)]
    #[must_use]
    pub fn pi(&mut self) -> PI_W<NMIREQEN_SPEC, 1> {
        PI_W::new(self)
    }
    #[doc = "Bit 2 - Promote RTC Alarm Interrupt Request to NMI Request"]
    #[inline(always)]
    #[must_use]
    pub fn ai(&mut self) -> AI_W<NMIREQEN_SPEC, 2> {
        AI_W::new(self)
    }
    #[doc = "Bit 16 - Promote Channel 0 Interrupt of ERU0 Request to NMI Request"]
    #[inline(always)]
    #[must_use]
    pub fn eru00(&mut self) -> ERU00_W<NMIREQEN_SPEC, 16> {
        ERU00_W::new(self)
    }
    #[doc = "Bit 17 - Promote Channel 1 Interrupt of ERU0 Request to NMI Request"]
    #[inline(always)]
    #[must_use]
    pub fn eru01(&mut self) -> ERU01_W<NMIREQEN_SPEC, 17> {
        ERU01_W::new(self)
    }
    #[doc = "Bit 18 - Promote Channel 2 Interrupt of ERU0 Request to NMI Request"]
    #[inline(always)]
    #[must_use]
    pub fn eru02(&mut self) -> ERU02_W<NMIREQEN_SPEC, 18> {
        ERU02_W::new(self)
    }
    #[doc = "Bit 19 - Promote Channel 3 Interrupt of ERU0 Request to NMI Request"]
    #[inline(always)]
    #[must_use]
    pub fn eru03(&mut self) -> ERU03_W<NMIREQEN_SPEC, 19> {
        ERU03_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "SCU Service Request Mask\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`nmireqen::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`nmireqen::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NMIREQEN_SPEC;
impl crate::RegisterSpec for NMIREQEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`nmireqen::R`](R) reader structure"]
impl crate::Readable for NMIREQEN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`nmireqen::W`](W) writer structure"]
impl crate::Writable for NMIREQEN_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets NMIREQEN to value 0"]
impl crate::Resettable for NMIREQEN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
