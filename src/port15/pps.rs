#[doc = "Register `PPS` reader"]
pub type R = crate::R<PPS_SPEC>;
#[doc = "Register `PPS` writer"]
pub type W = crate::W<PPS_SPEC>;
#[doc = "Field `PPS0` reader - Port n Pin Power Save Bit 0"]
pub type PPS0_R = crate::BitReader<PPS0_A>;
#[doc = "Port n Pin Power Save Bit 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PPS0_A {
    #[doc = "0: Pin Power Save of Pn.x is disabled."]
    CONST_0 = 0,
    #[doc = "1: Pin Power Save of Pn.x is enabled."]
    CONST_1 = 1,
}
impl From<PPS0_A> for bool {
    #[inline(always)]
    fn from(variant: PPS0_A) -> Self {
        variant as u8 != 0
    }
}
impl PPS0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PPS0_A {
        match self.bits {
            false => PPS0_A::CONST_0,
            true => PPS0_A::CONST_1,
        }
    }
    #[doc = "Pin Power Save of Pn.x is disabled."]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == PPS0_A::CONST_0
    }
    #[doc = "Pin Power Save of Pn.x is enabled."]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == PPS0_A::CONST_1
    }
}
#[doc = "Field `PPS0` writer - Port n Pin Power Save Bit 0"]
pub type PPS0_W<'a, REG> = crate::BitWriter<'a, REG, PPS0_A>;
impl<'a, REG> PPS0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Pin Power Save of Pn.x is disabled."]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(PPS0_A::CONST_0)
    }
    #[doc = "Pin Power Save of Pn.x is enabled."]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(PPS0_A::CONST_1)
    }
}
#[doc = "Field `PPS1` reader - Port n Pin Power Save Bit 1"]
pub type PPS1_R = crate::BitReader<PPS1_A>;
#[doc = "Port n Pin Power Save Bit 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PPS1_A {
    #[doc = "0: Pin Power Save of Pn.x is disabled."]
    CONST_0 = 0,
    #[doc = "1: Pin Power Save of Pn.x is enabled."]
    CONST_1 = 1,
}
impl From<PPS1_A> for bool {
    #[inline(always)]
    fn from(variant: PPS1_A) -> Self {
        variant as u8 != 0
    }
}
impl PPS1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PPS1_A {
        match self.bits {
            false => PPS1_A::CONST_0,
            true => PPS1_A::CONST_1,
        }
    }
    #[doc = "Pin Power Save of Pn.x is disabled."]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == PPS1_A::CONST_0
    }
    #[doc = "Pin Power Save of Pn.x is enabled."]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == PPS1_A::CONST_1
    }
}
#[doc = "Field `PPS1` writer - Port n Pin Power Save Bit 1"]
pub type PPS1_W<'a, REG> = crate::BitWriter<'a, REG, PPS1_A>;
impl<'a, REG> PPS1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Pin Power Save of Pn.x is disabled."]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(PPS1_A::CONST_0)
    }
    #[doc = "Pin Power Save of Pn.x is enabled."]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(PPS1_A::CONST_1)
    }
}
#[doc = "Field `PPS2` reader - Port n Pin Power Save Bit 2"]
pub type PPS2_R = crate::BitReader<PPS2_A>;
#[doc = "Port n Pin Power Save Bit 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PPS2_A {
    #[doc = "0: Pin Power Save of Pn.x is disabled."]
    CONST_0 = 0,
    #[doc = "1: Pin Power Save of Pn.x is enabled."]
    CONST_1 = 1,
}
impl From<PPS2_A> for bool {
    #[inline(always)]
    fn from(variant: PPS2_A) -> Self {
        variant as u8 != 0
    }
}
impl PPS2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PPS2_A {
        match self.bits {
            false => PPS2_A::CONST_0,
            true => PPS2_A::CONST_1,
        }
    }
    #[doc = "Pin Power Save of Pn.x is disabled."]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == PPS2_A::CONST_0
    }
    #[doc = "Pin Power Save of Pn.x is enabled."]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == PPS2_A::CONST_1
    }
}
#[doc = "Field `PPS2` writer - Port n Pin Power Save Bit 2"]
pub type PPS2_W<'a, REG> = crate::BitWriter<'a, REG, PPS2_A>;
impl<'a, REG> PPS2_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Pin Power Save of Pn.x is disabled."]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(PPS2_A::CONST_0)
    }
    #[doc = "Pin Power Save of Pn.x is enabled."]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(PPS2_A::CONST_1)
    }
}
#[doc = "Field `PPS3` reader - Port n Pin Power Save Bit 3"]
pub type PPS3_R = crate::BitReader<PPS3_A>;
#[doc = "Port n Pin Power Save Bit 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PPS3_A {
    #[doc = "0: Pin Power Save of Pn.x is disabled."]
    CONST_0 = 0,
    #[doc = "1: Pin Power Save of Pn.x is enabled."]
    CONST_1 = 1,
}
impl From<PPS3_A> for bool {
    #[inline(always)]
    fn from(variant: PPS3_A) -> Self {
        variant as u8 != 0
    }
}
impl PPS3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PPS3_A {
        match self.bits {
            false => PPS3_A::CONST_0,
            true => PPS3_A::CONST_1,
        }
    }
    #[doc = "Pin Power Save of Pn.x is disabled."]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == PPS3_A::CONST_0
    }
    #[doc = "Pin Power Save of Pn.x is enabled."]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == PPS3_A::CONST_1
    }
}
#[doc = "Field `PPS3` writer - Port n Pin Power Save Bit 3"]
pub type PPS3_W<'a, REG> = crate::BitWriter<'a, REG, PPS3_A>;
impl<'a, REG> PPS3_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Pin Power Save of Pn.x is disabled."]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(PPS3_A::CONST_0)
    }
    #[doc = "Pin Power Save of Pn.x is enabled."]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(PPS3_A::CONST_1)
    }
}
#[doc = "Field `PPS4` reader - Port n Pin Power Save Bit 4"]
pub type PPS4_R = crate::BitReader<PPS4_A>;
#[doc = "Port n Pin Power Save Bit 4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PPS4_A {
    #[doc = "0: Pin Power Save of Pn.x is disabled."]
    CONST_0 = 0,
    #[doc = "1: Pin Power Save of Pn.x is enabled."]
    CONST_1 = 1,
}
impl From<PPS4_A> for bool {
    #[inline(always)]
    fn from(variant: PPS4_A) -> Self {
        variant as u8 != 0
    }
}
impl PPS4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PPS4_A {
        match self.bits {
            false => PPS4_A::CONST_0,
            true => PPS4_A::CONST_1,
        }
    }
    #[doc = "Pin Power Save of Pn.x is disabled."]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == PPS4_A::CONST_0
    }
    #[doc = "Pin Power Save of Pn.x is enabled."]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == PPS4_A::CONST_1
    }
}
#[doc = "Field `PPS4` writer - Port n Pin Power Save Bit 4"]
pub type PPS4_W<'a, REG> = crate::BitWriter<'a, REG, PPS4_A>;
impl<'a, REG> PPS4_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Pin Power Save of Pn.x is disabled."]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(PPS4_A::CONST_0)
    }
    #[doc = "Pin Power Save of Pn.x is enabled."]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(PPS4_A::CONST_1)
    }
}
#[doc = "Field `PPS5` reader - Port n Pin Power Save Bit 5"]
pub type PPS5_R = crate::BitReader<PPS5_A>;
#[doc = "Port n Pin Power Save Bit 5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PPS5_A {
    #[doc = "0: Pin Power Save of Pn.x is disabled."]
    CONST_0 = 0,
    #[doc = "1: Pin Power Save of Pn.x is enabled."]
    CONST_1 = 1,
}
impl From<PPS5_A> for bool {
    #[inline(always)]
    fn from(variant: PPS5_A) -> Self {
        variant as u8 != 0
    }
}
impl PPS5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PPS5_A {
        match self.bits {
            false => PPS5_A::CONST_0,
            true => PPS5_A::CONST_1,
        }
    }
    #[doc = "Pin Power Save of Pn.x is disabled."]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == PPS5_A::CONST_0
    }
    #[doc = "Pin Power Save of Pn.x is enabled."]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == PPS5_A::CONST_1
    }
}
#[doc = "Field `PPS5` writer - Port n Pin Power Save Bit 5"]
pub type PPS5_W<'a, REG> = crate::BitWriter<'a, REG, PPS5_A>;
impl<'a, REG> PPS5_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Pin Power Save of Pn.x is disabled."]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(PPS5_A::CONST_0)
    }
    #[doc = "Pin Power Save of Pn.x is enabled."]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(PPS5_A::CONST_1)
    }
}
#[doc = "Field `PPS6` reader - Port n Pin Power Save Bit 6"]
pub type PPS6_R = crate::BitReader<PPS6_A>;
#[doc = "Port n Pin Power Save Bit 6\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PPS6_A {
    #[doc = "0: Pin Power Save of Pn.x is disabled."]
    CONST_0 = 0,
    #[doc = "1: Pin Power Save of Pn.x is enabled."]
    CONST_1 = 1,
}
impl From<PPS6_A> for bool {
    #[inline(always)]
    fn from(variant: PPS6_A) -> Self {
        variant as u8 != 0
    }
}
impl PPS6_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PPS6_A {
        match self.bits {
            false => PPS6_A::CONST_0,
            true => PPS6_A::CONST_1,
        }
    }
    #[doc = "Pin Power Save of Pn.x is disabled."]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == PPS6_A::CONST_0
    }
    #[doc = "Pin Power Save of Pn.x is enabled."]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == PPS6_A::CONST_1
    }
}
#[doc = "Field `PPS6` writer - Port n Pin Power Save Bit 6"]
pub type PPS6_W<'a, REG> = crate::BitWriter<'a, REG, PPS6_A>;
impl<'a, REG> PPS6_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Pin Power Save of Pn.x is disabled."]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(PPS6_A::CONST_0)
    }
    #[doc = "Pin Power Save of Pn.x is enabled."]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(PPS6_A::CONST_1)
    }
}
#[doc = "Field `PPS7` reader - Port n Pin Power Save Bit 7"]
pub type PPS7_R = crate::BitReader<PPS7_A>;
#[doc = "Port n Pin Power Save Bit 7\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PPS7_A {
    #[doc = "0: Pin Power Save of Pn.x is disabled."]
    CONST_0 = 0,
    #[doc = "1: Pin Power Save of Pn.x is enabled."]
    CONST_1 = 1,
}
impl From<PPS7_A> for bool {
    #[inline(always)]
    fn from(variant: PPS7_A) -> Self {
        variant as u8 != 0
    }
}
impl PPS7_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PPS7_A {
        match self.bits {
            false => PPS7_A::CONST_0,
            true => PPS7_A::CONST_1,
        }
    }
    #[doc = "Pin Power Save of Pn.x is disabled."]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == PPS7_A::CONST_0
    }
    #[doc = "Pin Power Save of Pn.x is enabled."]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == PPS7_A::CONST_1
    }
}
#[doc = "Field `PPS7` writer - Port n Pin Power Save Bit 7"]
pub type PPS7_W<'a, REG> = crate::BitWriter<'a, REG, PPS7_A>;
impl<'a, REG> PPS7_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Pin Power Save of Pn.x is disabled."]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(PPS7_A::CONST_0)
    }
    #[doc = "Pin Power Save of Pn.x is enabled."]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(PPS7_A::CONST_1)
    }
}
#[doc = "Field `PPS8` reader - Port n Pin Power Save Bit 8"]
pub type PPS8_R = crate::BitReader<PPS8_A>;
#[doc = "Port n Pin Power Save Bit 8\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PPS8_A {
    #[doc = "0: Pin Power Save of Pn.x is disabled."]
    CONST_0 = 0,
    #[doc = "1: Pin Power Save of Pn.x is enabled."]
    CONST_1 = 1,
}
impl From<PPS8_A> for bool {
    #[inline(always)]
    fn from(variant: PPS8_A) -> Self {
        variant as u8 != 0
    }
}
impl PPS8_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PPS8_A {
        match self.bits {
            false => PPS8_A::CONST_0,
            true => PPS8_A::CONST_1,
        }
    }
    #[doc = "Pin Power Save of Pn.x is disabled."]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == PPS8_A::CONST_0
    }
    #[doc = "Pin Power Save of Pn.x is enabled."]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == PPS8_A::CONST_1
    }
}
#[doc = "Field `PPS8` writer - Port n Pin Power Save Bit 8"]
pub type PPS8_W<'a, REG> = crate::BitWriter<'a, REG, PPS8_A>;
impl<'a, REG> PPS8_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Pin Power Save of Pn.x is disabled."]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(PPS8_A::CONST_0)
    }
    #[doc = "Pin Power Save of Pn.x is enabled."]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(PPS8_A::CONST_1)
    }
}
#[doc = "Field `PPS9` reader - Port n Pin Power Save Bit 9"]
pub type PPS9_R = crate::BitReader<PPS9_A>;
#[doc = "Port n Pin Power Save Bit 9\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PPS9_A {
    #[doc = "0: Pin Power Save of Pn.x is disabled."]
    CONST_0 = 0,
    #[doc = "1: Pin Power Save of Pn.x is enabled."]
    CONST_1 = 1,
}
impl From<PPS9_A> for bool {
    #[inline(always)]
    fn from(variant: PPS9_A) -> Self {
        variant as u8 != 0
    }
}
impl PPS9_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PPS9_A {
        match self.bits {
            false => PPS9_A::CONST_0,
            true => PPS9_A::CONST_1,
        }
    }
    #[doc = "Pin Power Save of Pn.x is disabled."]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == PPS9_A::CONST_0
    }
    #[doc = "Pin Power Save of Pn.x is enabled."]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == PPS9_A::CONST_1
    }
}
#[doc = "Field `PPS9` writer - Port n Pin Power Save Bit 9"]
pub type PPS9_W<'a, REG> = crate::BitWriter<'a, REG, PPS9_A>;
impl<'a, REG> PPS9_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Pin Power Save of Pn.x is disabled."]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(PPS9_A::CONST_0)
    }
    #[doc = "Pin Power Save of Pn.x is enabled."]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(PPS9_A::CONST_1)
    }
}
#[doc = "Field `PPS10` reader - Port n Pin Power Save Bit 10"]
pub type PPS10_R = crate::BitReader<PPS10_A>;
#[doc = "Port n Pin Power Save Bit 10\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PPS10_A {
    #[doc = "0: Pin Power Save of Pn.x is disabled."]
    CONST_0 = 0,
    #[doc = "1: Pin Power Save of Pn.x is enabled."]
    CONST_1 = 1,
}
impl From<PPS10_A> for bool {
    #[inline(always)]
    fn from(variant: PPS10_A) -> Self {
        variant as u8 != 0
    }
}
impl PPS10_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PPS10_A {
        match self.bits {
            false => PPS10_A::CONST_0,
            true => PPS10_A::CONST_1,
        }
    }
    #[doc = "Pin Power Save of Pn.x is disabled."]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == PPS10_A::CONST_0
    }
    #[doc = "Pin Power Save of Pn.x is enabled."]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == PPS10_A::CONST_1
    }
}
#[doc = "Field `PPS10` writer - Port n Pin Power Save Bit 10"]
pub type PPS10_W<'a, REG> = crate::BitWriter<'a, REG, PPS10_A>;
impl<'a, REG> PPS10_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Pin Power Save of Pn.x is disabled."]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(PPS10_A::CONST_0)
    }
    #[doc = "Pin Power Save of Pn.x is enabled."]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(PPS10_A::CONST_1)
    }
}
#[doc = "Field `PPS11` reader - Port n Pin Power Save Bit 11"]
pub type PPS11_R = crate::BitReader<PPS11_A>;
#[doc = "Port n Pin Power Save Bit 11\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PPS11_A {
    #[doc = "0: Pin Power Save of Pn.x is disabled."]
    CONST_0 = 0,
    #[doc = "1: Pin Power Save of Pn.x is enabled."]
    CONST_1 = 1,
}
impl From<PPS11_A> for bool {
    #[inline(always)]
    fn from(variant: PPS11_A) -> Self {
        variant as u8 != 0
    }
}
impl PPS11_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PPS11_A {
        match self.bits {
            false => PPS11_A::CONST_0,
            true => PPS11_A::CONST_1,
        }
    }
    #[doc = "Pin Power Save of Pn.x is disabled."]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == PPS11_A::CONST_0
    }
    #[doc = "Pin Power Save of Pn.x is enabled."]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == PPS11_A::CONST_1
    }
}
#[doc = "Field `PPS11` writer - Port n Pin Power Save Bit 11"]
pub type PPS11_W<'a, REG> = crate::BitWriter<'a, REG, PPS11_A>;
impl<'a, REG> PPS11_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Pin Power Save of Pn.x is disabled."]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(PPS11_A::CONST_0)
    }
    #[doc = "Pin Power Save of Pn.x is enabled."]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(PPS11_A::CONST_1)
    }
}
#[doc = "Field `PPS12` reader - Port n Pin Power Save Bit 12"]
pub type PPS12_R = crate::BitReader<PPS12_A>;
#[doc = "Port n Pin Power Save Bit 12\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PPS12_A {
    #[doc = "0: Pin Power Save of Pn.x is disabled."]
    CONST_0 = 0,
    #[doc = "1: Pin Power Save of Pn.x is enabled."]
    CONST_1 = 1,
}
impl From<PPS12_A> for bool {
    #[inline(always)]
    fn from(variant: PPS12_A) -> Self {
        variant as u8 != 0
    }
}
impl PPS12_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PPS12_A {
        match self.bits {
            false => PPS12_A::CONST_0,
            true => PPS12_A::CONST_1,
        }
    }
    #[doc = "Pin Power Save of Pn.x is disabled."]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == PPS12_A::CONST_0
    }
    #[doc = "Pin Power Save of Pn.x is enabled."]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == PPS12_A::CONST_1
    }
}
#[doc = "Field `PPS12` writer - Port n Pin Power Save Bit 12"]
pub type PPS12_W<'a, REG> = crate::BitWriter<'a, REG, PPS12_A>;
impl<'a, REG> PPS12_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Pin Power Save of Pn.x is disabled."]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(PPS12_A::CONST_0)
    }
    #[doc = "Pin Power Save of Pn.x is enabled."]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(PPS12_A::CONST_1)
    }
}
#[doc = "Field `PPS13` reader - Port n Pin Power Save Bit 13"]
pub type PPS13_R = crate::BitReader<PPS13_A>;
#[doc = "Port n Pin Power Save Bit 13\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PPS13_A {
    #[doc = "0: Pin Power Save of Pn.x is disabled."]
    CONST_0 = 0,
    #[doc = "1: Pin Power Save of Pn.x is enabled."]
    CONST_1 = 1,
}
impl From<PPS13_A> for bool {
    #[inline(always)]
    fn from(variant: PPS13_A) -> Self {
        variant as u8 != 0
    }
}
impl PPS13_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PPS13_A {
        match self.bits {
            false => PPS13_A::CONST_0,
            true => PPS13_A::CONST_1,
        }
    }
    #[doc = "Pin Power Save of Pn.x is disabled."]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == PPS13_A::CONST_0
    }
    #[doc = "Pin Power Save of Pn.x is enabled."]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == PPS13_A::CONST_1
    }
}
#[doc = "Field `PPS13` writer - Port n Pin Power Save Bit 13"]
pub type PPS13_W<'a, REG> = crate::BitWriter<'a, REG, PPS13_A>;
impl<'a, REG> PPS13_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Pin Power Save of Pn.x is disabled."]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(PPS13_A::CONST_0)
    }
    #[doc = "Pin Power Save of Pn.x is enabled."]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(PPS13_A::CONST_1)
    }
}
#[doc = "Field `PPS14` reader - Port n Pin Power Save Bit 14"]
pub type PPS14_R = crate::BitReader<PPS14_A>;
#[doc = "Port n Pin Power Save Bit 14\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PPS14_A {
    #[doc = "0: Pin Power Save of Pn.x is disabled."]
    CONST_0 = 0,
    #[doc = "1: Pin Power Save of Pn.x is enabled."]
    CONST_1 = 1,
}
impl From<PPS14_A> for bool {
    #[inline(always)]
    fn from(variant: PPS14_A) -> Self {
        variant as u8 != 0
    }
}
impl PPS14_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PPS14_A {
        match self.bits {
            false => PPS14_A::CONST_0,
            true => PPS14_A::CONST_1,
        }
    }
    #[doc = "Pin Power Save of Pn.x is disabled."]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == PPS14_A::CONST_0
    }
    #[doc = "Pin Power Save of Pn.x is enabled."]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == PPS14_A::CONST_1
    }
}
#[doc = "Field `PPS14` writer - Port n Pin Power Save Bit 14"]
pub type PPS14_W<'a, REG> = crate::BitWriter<'a, REG, PPS14_A>;
impl<'a, REG> PPS14_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Pin Power Save of Pn.x is disabled."]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(PPS14_A::CONST_0)
    }
    #[doc = "Pin Power Save of Pn.x is enabled."]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(PPS14_A::CONST_1)
    }
}
#[doc = "Field `PPS15` reader - Port n Pin Power Save Bit 15"]
pub type PPS15_R = crate::BitReader<PPS15_A>;
#[doc = "Port n Pin Power Save Bit 15\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PPS15_A {
    #[doc = "0: Pin Power Save of Pn.x is disabled."]
    CONST_0 = 0,
    #[doc = "1: Pin Power Save of Pn.x is enabled."]
    CONST_1 = 1,
}
impl From<PPS15_A> for bool {
    #[inline(always)]
    fn from(variant: PPS15_A) -> Self {
        variant as u8 != 0
    }
}
impl PPS15_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PPS15_A {
        match self.bits {
            false => PPS15_A::CONST_0,
            true => PPS15_A::CONST_1,
        }
    }
    #[doc = "Pin Power Save of Pn.x is disabled."]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == PPS15_A::CONST_0
    }
    #[doc = "Pin Power Save of Pn.x is enabled."]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == PPS15_A::CONST_1
    }
}
#[doc = "Field `PPS15` writer - Port n Pin Power Save Bit 15"]
pub type PPS15_W<'a, REG> = crate::BitWriter<'a, REG, PPS15_A>;
impl<'a, REG> PPS15_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Pin Power Save of Pn.x is disabled."]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(PPS15_A::CONST_0)
    }
    #[doc = "Pin Power Save of Pn.x is enabled."]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(PPS15_A::CONST_1)
    }
}
impl R {
    #[doc = "Bit 0 - Port n Pin Power Save Bit 0"]
    #[inline(always)]
    pub fn pps0(&self) -> PPS0_R {
        PPS0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Port n Pin Power Save Bit 1"]
    #[inline(always)]
    pub fn pps1(&self) -> PPS1_R {
        PPS1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Port n Pin Power Save Bit 2"]
    #[inline(always)]
    pub fn pps2(&self) -> PPS2_R {
        PPS2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Port n Pin Power Save Bit 3"]
    #[inline(always)]
    pub fn pps3(&self) -> PPS3_R {
        PPS3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Port n Pin Power Save Bit 4"]
    #[inline(always)]
    pub fn pps4(&self) -> PPS4_R {
        PPS4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Port n Pin Power Save Bit 5"]
    #[inline(always)]
    pub fn pps5(&self) -> PPS5_R {
        PPS5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Port n Pin Power Save Bit 6"]
    #[inline(always)]
    pub fn pps6(&self) -> PPS6_R {
        PPS6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Port n Pin Power Save Bit 7"]
    #[inline(always)]
    pub fn pps7(&self) -> PPS7_R {
        PPS7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Port n Pin Power Save Bit 8"]
    #[inline(always)]
    pub fn pps8(&self) -> PPS8_R {
        PPS8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Port n Pin Power Save Bit 9"]
    #[inline(always)]
    pub fn pps9(&self) -> PPS9_R {
        PPS9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Port n Pin Power Save Bit 10"]
    #[inline(always)]
    pub fn pps10(&self) -> PPS10_R {
        PPS10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Port n Pin Power Save Bit 11"]
    #[inline(always)]
    pub fn pps11(&self) -> PPS11_R {
        PPS11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Port n Pin Power Save Bit 12"]
    #[inline(always)]
    pub fn pps12(&self) -> PPS12_R {
        PPS12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Port n Pin Power Save Bit 13"]
    #[inline(always)]
    pub fn pps13(&self) -> PPS13_R {
        PPS13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Port n Pin Power Save Bit 14"]
    #[inline(always)]
    pub fn pps14(&self) -> PPS14_R {
        PPS14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Port n Pin Power Save Bit 15"]
    #[inline(always)]
    pub fn pps15(&self) -> PPS15_R {
        PPS15_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Port n Pin Power Save Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn pps0(&mut self) -> PPS0_W<PPS_SPEC> {
        PPS0_W::new(self, 0)
    }
    #[doc = "Bit 1 - Port n Pin Power Save Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn pps1(&mut self) -> PPS1_W<PPS_SPEC> {
        PPS1_W::new(self, 1)
    }
    #[doc = "Bit 2 - Port n Pin Power Save Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn pps2(&mut self) -> PPS2_W<PPS_SPEC> {
        PPS2_W::new(self, 2)
    }
    #[doc = "Bit 3 - Port n Pin Power Save Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn pps3(&mut self) -> PPS3_W<PPS_SPEC> {
        PPS3_W::new(self, 3)
    }
    #[doc = "Bit 4 - Port n Pin Power Save Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn pps4(&mut self) -> PPS4_W<PPS_SPEC> {
        PPS4_W::new(self, 4)
    }
    #[doc = "Bit 5 - Port n Pin Power Save Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn pps5(&mut self) -> PPS5_W<PPS_SPEC> {
        PPS5_W::new(self, 5)
    }
    #[doc = "Bit 6 - Port n Pin Power Save Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn pps6(&mut self) -> PPS6_W<PPS_SPEC> {
        PPS6_W::new(self, 6)
    }
    #[doc = "Bit 7 - Port n Pin Power Save Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn pps7(&mut self) -> PPS7_W<PPS_SPEC> {
        PPS7_W::new(self, 7)
    }
    #[doc = "Bit 8 - Port n Pin Power Save Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn pps8(&mut self) -> PPS8_W<PPS_SPEC> {
        PPS8_W::new(self, 8)
    }
    #[doc = "Bit 9 - Port n Pin Power Save Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn pps9(&mut self) -> PPS9_W<PPS_SPEC> {
        PPS9_W::new(self, 9)
    }
    #[doc = "Bit 10 - Port n Pin Power Save Bit 10"]
    #[inline(always)]
    #[must_use]
    pub fn pps10(&mut self) -> PPS10_W<PPS_SPEC> {
        PPS10_W::new(self, 10)
    }
    #[doc = "Bit 11 - Port n Pin Power Save Bit 11"]
    #[inline(always)]
    #[must_use]
    pub fn pps11(&mut self) -> PPS11_W<PPS_SPEC> {
        PPS11_W::new(self, 11)
    }
    #[doc = "Bit 12 - Port n Pin Power Save Bit 12"]
    #[inline(always)]
    #[must_use]
    pub fn pps12(&mut self) -> PPS12_W<PPS_SPEC> {
        PPS12_W::new(self, 12)
    }
    #[doc = "Bit 13 - Port n Pin Power Save Bit 13"]
    #[inline(always)]
    #[must_use]
    pub fn pps13(&mut self) -> PPS13_W<PPS_SPEC> {
        PPS13_W::new(self, 13)
    }
    #[doc = "Bit 14 - Port n Pin Power Save Bit 14"]
    #[inline(always)]
    #[must_use]
    pub fn pps14(&mut self) -> PPS14_W<PPS_SPEC> {
        PPS14_W::new(self, 14)
    }
    #[doc = "Bit 15 - Port n Pin Power Save Bit 15"]
    #[inline(always)]
    #[must_use]
    pub fn pps15(&mut self) -> PPS15_W<PPS_SPEC> {
        PPS15_W::new(self, 15)
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
#[doc = "Port 15 Pin Power Save Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pps::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pps::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PPS_SPEC;
impl crate::RegisterSpec for PPS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pps::R`](R) reader structure"]
impl crate::Readable for PPS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pps::W`](W) writer structure"]
impl crate::Writable for PPS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PPS to value 0"]
impl crate::Resettable for PPS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
