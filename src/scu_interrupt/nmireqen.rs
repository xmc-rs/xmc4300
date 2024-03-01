#[doc = "Register `NMIREQEN` reader"]
pub type R = crate::R<NmireqenSpec>;
#[doc = "Register `NMIREQEN` writer"]
pub type W = crate::W<NmireqenSpec>;
#[doc = "Promote Pre-Warning Interrupt Request to NMI Request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Prwarn {
    #[doc = "0: Disabled"]
    Const0 = 0,
    #[doc = "1: Enabled"]
    Const1 = 1,
}
impl From<Prwarn> for bool {
    #[inline(always)]
    fn from(variant: Prwarn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PRWARN` reader - Promote Pre-Warning Interrupt Request to NMI Request"]
pub type PrwarnR = crate::BitReader<Prwarn>;
impl PrwarnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Prwarn {
        match self.bits {
            false => Prwarn::Const0,
            true => Prwarn::Const1,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == Prwarn::Const0
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == Prwarn::Const1
    }
}
#[doc = "Field `PRWARN` writer - Promote Pre-Warning Interrupt Request to NMI Request"]
pub type PrwarnW<'a, REG> = crate::BitWriter<'a, REG, Prwarn>;
impl<'a, REG> PrwarnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(Prwarn::Const0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(Prwarn::Const1)
    }
}
#[doc = "Promote RTC Periodic Interrupt request to NMI Request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pi {
    #[doc = "0: Disabled"]
    Const0 = 0,
    #[doc = "1: Enabled"]
    Const1 = 1,
}
impl From<Pi> for bool {
    #[inline(always)]
    fn from(variant: Pi) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PI` reader - Promote RTC Periodic Interrupt request to NMI Request"]
pub type PiR = crate::BitReader<Pi>;
impl PiR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pi {
        match self.bits {
            false => Pi::Const0,
            true => Pi::Const1,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == Pi::Const0
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == Pi::Const1
    }
}
#[doc = "Field `PI` writer - Promote RTC Periodic Interrupt request to NMI Request"]
pub type PiW<'a, REG> = crate::BitWriter<'a, REG, Pi>;
impl<'a, REG> PiW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(Pi::Const0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(Pi::Const1)
    }
}
#[doc = "Promote RTC Alarm Interrupt Request to NMI Request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ai {
    #[doc = "0: Disabled"]
    Const0 = 0,
    #[doc = "1: Enabled"]
    Const1 = 1,
}
impl From<Ai> for bool {
    #[inline(always)]
    fn from(variant: Ai) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AI` reader - Promote RTC Alarm Interrupt Request to NMI Request"]
pub type AiR = crate::BitReader<Ai>;
impl AiR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ai {
        match self.bits {
            false => Ai::Const0,
            true => Ai::Const1,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == Ai::Const0
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == Ai::Const1
    }
}
#[doc = "Field `AI` writer - Promote RTC Alarm Interrupt Request to NMI Request"]
pub type AiW<'a, REG> = crate::BitWriter<'a, REG, Ai>;
impl<'a, REG> AiW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(Ai::Const0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(Ai::Const1)
    }
}
#[doc = "Promote Channel 0 Interrupt of ERU0 Request to NMI Request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Eru00 {
    #[doc = "0: Disabled"]
    Const0 = 0,
    #[doc = "1: Enabled"]
    Const1 = 1,
}
impl From<Eru00> for bool {
    #[inline(always)]
    fn from(variant: Eru00) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERU00` reader - Promote Channel 0 Interrupt of ERU0 Request to NMI Request"]
pub type Eru00R = crate::BitReader<Eru00>;
impl Eru00R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Eru00 {
        match self.bits {
            false => Eru00::Const0,
            true => Eru00::Const1,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == Eru00::Const0
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == Eru00::Const1
    }
}
#[doc = "Field `ERU00` writer - Promote Channel 0 Interrupt of ERU0 Request to NMI Request"]
pub type Eru00W<'a, REG> = crate::BitWriter<'a, REG, Eru00>;
impl<'a, REG> Eru00W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(Eru00::Const0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(Eru00::Const1)
    }
}
#[doc = "Promote Channel 1 Interrupt of ERU0 Request to NMI Request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Eru01 {
    #[doc = "0: Disabled"]
    Const0 = 0,
    #[doc = "1: Enabled"]
    Const1 = 1,
}
impl From<Eru01> for bool {
    #[inline(always)]
    fn from(variant: Eru01) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERU01` reader - Promote Channel 1 Interrupt of ERU0 Request to NMI Request"]
pub type Eru01R = crate::BitReader<Eru01>;
impl Eru01R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Eru01 {
        match self.bits {
            false => Eru01::Const0,
            true => Eru01::Const1,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == Eru01::Const0
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == Eru01::Const1
    }
}
#[doc = "Field `ERU01` writer - Promote Channel 1 Interrupt of ERU0 Request to NMI Request"]
pub type Eru01W<'a, REG> = crate::BitWriter<'a, REG, Eru01>;
impl<'a, REG> Eru01W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(Eru01::Const0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(Eru01::Const1)
    }
}
#[doc = "Promote Channel 2 Interrupt of ERU0 Request to NMI Request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Eru02 {
    #[doc = "0: Disabled"]
    Const0 = 0,
    #[doc = "1: Enabled"]
    Const1 = 1,
}
impl From<Eru02> for bool {
    #[inline(always)]
    fn from(variant: Eru02) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERU02` reader - Promote Channel 2 Interrupt of ERU0 Request to NMI Request"]
pub type Eru02R = crate::BitReader<Eru02>;
impl Eru02R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Eru02 {
        match self.bits {
            false => Eru02::Const0,
            true => Eru02::Const1,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == Eru02::Const0
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == Eru02::Const1
    }
}
#[doc = "Field `ERU02` writer - Promote Channel 2 Interrupt of ERU0 Request to NMI Request"]
pub type Eru02W<'a, REG> = crate::BitWriter<'a, REG, Eru02>;
impl<'a, REG> Eru02W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(Eru02::Const0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(Eru02::Const1)
    }
}
#[doc = "Promote Channel 3 Interrupt of ERU0 Request to NMI Request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Eru03 {
    #[doc = "0: Disabled"]
    Const0 = 0,
    #[doc = "1: Enabled"]
    Const1 = 1,
}
impl From<Eru03> for bool {
    #[inline(always)]
    fn from(variant: Eru03) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERU03` reader - Promote Channel 3 Interrupt of ERU0 Request to NMI Request"]
pub type Eru03R = crate::BitReader<Eru03>;
impl Eru03R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Eru03 {
        match self.bits {
            false => Eru03::Const0,
            true => Eru03::Const1,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == Eru03::Const0
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == Eru03::Const1
    }
}
#[doc = "Field `ERU03` writer - Promote Channel 3 Interrupt of ERU0 Request to NMI Request"]
pub type Eru03W<'a, REG> = crate::BitWriter<'a, REG, Eru03>;
impl<'a, REG> Eru03W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(Eru03::Const0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(Eru03::Const1)
    }
}
impl R {
    #[doc = "Bit 0 - Promote Pre-Warning Interrupt Request to NMI Request"]
    #[inline(always)]
    pub fn prwarn(&self) -> PrwarnR {
        PrwarnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Promote RTC Periodic Interrupt request to NMI Request"]
    #[inline(always)]
    pub fn pi(&self) -> PiR {
        PiR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Promote RTC Alarm Interrupt Request to NMI Request"]
    #[inline(always)]
    pub fn ai(&self) -> AiR {
        AiR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 16 - Promote Channel 0 Interrupt of ERU0 Request to NMI Request"]
    #[inline(always)]
    pub fn eru00(&self) -> Eru00R {
        Eru00R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Promote Channel 1 Interrupt of ERU0 Request to NMI Request"]
    #[inline(always)]
    pub fn eru01(&self) -> Eru01R {
        Eru01R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Promote Channel 2 Interrupt of ERU0 Request to NMI Request"]
    #[inline(always)]
    pub fn eru02(&self) -> Eru02R {
        Eru02R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Promote Channel 3 Interrupt of ERU0 Request to NMI Request"]
    #[inline(always)]
    pub fn eru03(&self) -> Eru03R {
        Eru03R::new(((self.bits >> 19) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Promote Pre-Warning Interrupt Request to NMI Request"]
    #[inline(always)]
    #[must_use]
    pub fn prwarn(&mut self) -> PrwarnW<NmireqenSpec> {
        PrwarnW::new(self, 0)
    }
    #[doc = "Bit 1 - Promote RTC Periodic Interrupt request to NMI Request"]
    #[inline(always)]
    #[must_use]
    pub fn pi(&mut self) -> PiW<NmireqenSpec> {
        PiW::new(self, 1)
    }
    #[doc = "Bit 2 - Promote RTC Alarm Interrupt Request to NMI Request"]
    #[inline(always)]
    #[must_use]
    pub fn ai(&mut self) -> AiW<NmireqenSpec> {
        AiW::new(self, 2)
    }
    #[doc = "Bit 16 - Promote Channel 0 Interrupt of ERU0 Request to NMI Request"]
    #[inline(always)]
    #[must_use]
    pub fn eru00(&mut self) -> Eru00W<NmireqenSpec> {
        Eru00W::new(self, 16)
    }
    #[doc = "Bit 17 - Promote Channel 1 Interrupt of ERU0 Request to NMI Request"]
    #[inline(always)]
    #[must_use]
    pub fn eru01(&mut self) -> Eru01W<NmireqenSpec> {
        Eru01W::new(self, 17)
    }
    #[doc = "Bit 18 - Promote Channel 2 Interrupt of ERU0 Request to NMI Request"]
    #[inline(always)]
    #[must_use]
    pub fn eru02(&mut self) -> Eru02W<NmireqenSpec> {
        Eru02W::new(self, 18)
    }
    #[doc = "Bit 19 - Promote Channel 3 Interrupt of ERU0 Request to NMI Request"]
    #[inline(always)]
    #[must_use]
    pub fn eru03(&mut self) -> Eru03W<NmireqenSpec> {
        Eru03W::new(self, 19)
    }
}
#[doc = "SCU Service Request Mask\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`nmireqen::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`nmireqen::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NmireqenSpec;
impl crate::RegisterSpec for NmireqenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`nmireqen::R`](R) reader structure"]
impl crate::Readable for NmireqenSpec {}
#[doc = "`write(|w| ..)` method takes [`nmireqen::W`](W) writer structure"]
impl crate::Writable for NmireqenSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets NMIREQEN to value 0"]
impl crate::Resettable for NmireqenSpec {
    const RESET_VALUE: u32 = 0;
}
