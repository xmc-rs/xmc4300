#[doc = "Register `BFL` reader"]
pub type R = crate::R<BFL_SPEC>;
#[doc = "Register `BFL` writer"]
pub type W = crate::W<BFL_SPEC>;
#[doc = "Boundary Flag 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BFL0_A {
    #[doc = "0: Passive state: result has not yet crossed the activation boundary (see bitfield BFAy), or selected gate signal is inactive, or this boundary flag is disabled"]
    VALUE1 = 0,
    #[doc = "1: Active state: result has crossed the activation boundary"]
    VALUE2 = 1,
}
impl From<BFL0_A> for bool {
    #[inline(always)]
    fn from(variant: BFL0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BFL0` reader - Boundary Flag 0"]
pub type BFL0_R = crate::BitReader<BFL0_A>;
impl BFL0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BFL0_A {
        match self.bits {
            false => BFL0_A::VALUE1,
            true => BFL0_A::VALUE2,
        }
    }
    #[doc = "Passive state: result has not yet crossed the activation boundary (see bitfield BFAy), or selected gate signal is inactive, or this boundary flag is disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == BFL0_A::VALUE1
    }
    #[doc = "Active state: result has crossed the activation boundary"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == BFL0_A::VALUE2
    }
}
#[doc = "Boundary Flag 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BFL1_A {
    #[doc = "0: Passive state: result has not yet crossed the activation boundary (see bitfield BFAy), or selected gate signal is inactive, or this boundary flag is disabled"]
    VALUE1 = 0,
    #[doc = "1: Active state: result has crossed the activation boundary"]
    VALUE2 = 1,
}
impl From<BFL1_A> for bool {
    #[inline(always)]
    fn from(variant: BFL1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BFL1` reader - Boundary Flag 1"]
pub type BFL1_R = crate::BitReader<BFL1_A>;
impl BFL1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BFL1_A {
        match self.bits {
            false => BFL1_A::VALUE1,
            true => BFL1_A::VALUE2,
        }
    }
    #[doc = "Passive state: result has not yet crossed the activation boundary (see bitfield BFAy), or selected gate signal is inactive, or this boundary flag is disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == BFL1_A::VALUE1
    }
    #[doc = "Active state: result has crossed the activation boundary"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == BFL1_A::VALUE2
    }
}
#[doc = "Boundary Flag 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BFL2_A {
    #[doc = "0: Passive state: result has not yet crossed the activation boundary (see bitfield BFAy), or selected gate signal is inactive, or this boundary flag is disabled"]
    VALUE1 = 0,
    #[doc = "1: Active state: result has crossed the activation boundary"]
    VALUE2 = 1,
}
impl From<BFL2_A> for bool {
    #[inline(always)]
    fn from(variant: BFL2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BFL2` reader - Boundary Flag 2"]
pub type BFL2_R = crate::BitReader<BFL2_A>;
impl BFL2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BFL2_A {
        match self.bits {
            false => BFL2_A::VALUE1,
            true => BFL2_A::VALUE2,
        }
    }
    #[doc = "Passive state: result has not yet crossed the activation boundary (see bitfield BFAy), or selected gate signal is inactive, or this boundary flag is disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == BFL2_A::VALUE1
    }
    #[doc = "Active state: result has crossed the activation boundary"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == BFL2_A::VALUE2
    }
}
#[doc = "Boundary Flag 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BFL3_A {
    #[doc = "0: Passive state: result has not yet crossed the activation boundary (see bitfield BFAy), or selected gate signal is inactive, or this boundary flag is disabled"]
    VALUE1 = 0,
    #[doc = "1: Active state: result has crossed the activation boundary"]
    VALUE2 = 1,
}
impl From<BFL3_A> for bool {
    #[inline(always)]
    fn from(variant: BFL3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BFL3` reader - Boundary Flag 3"]
pub type BFL3_R = crate::BitReader<BFL3_A>;
impl BFL3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BFL3_A {
        match self.bits {
            false => BFL3_A::VALUE1,
            true => BFL3_A::VALUE2,
        }
    }
    #[doc = "Passive state: result has not yet crossed the activation boundary (see bitfield BFAy), or selected gate signal is inactive, or this boundary flag is disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == BFL3_A::VALUE1
    }
    #[doc = "Active state: result has crossed the activation boundary"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == BFL3_A::VALUE2
    }
}
#[doc = "Boundary Flag 0 Activation Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BFA0_A {
    #[doc = "0: Set boundary flag BFLy if result is above the defined band or compare value, clear if below"]
    VALUE1 = 0,
    #[doc = "1: Set boundary flag BFLy if result is below the defined band or compare value, clear if above"]
    VALUE2 = 1,
}
impl From<BFA0_A> for bool {
    #[inline(always)]
    fn from(variant: BFA0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BFA0` reader - Boundary Flag 0 Activation Select"]
pub type BFA0_R = crate::BitReader<BFA0_A>;
impl BFA0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BFA0_A {
        match self.bits {
            false => BFA0_A::VALUE1,
            true => BFA0_A::VALUE2,
        }
    }
    #[doc = "Set boundary flag BFLy if result is above the defined band or compare value, clear if below"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == BFA0_A::VALUE1
    }
    #[doc = "Set boundary flag BFLy if result is below the defined band or compare value, clear if above"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == BFA0_A::VALUE2
    }
}
#[doc = "Field `BFA0` writer - Boundary Flag 0 Activation Select"]
pub type BFA0_W<'a, REG> = crate::BitWriter<'a, REG, BFA0_A>;
impl<'a, REG> BFA0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Set boundary flag BFLy if result is above the defined band or compare value, clear if below"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(BFA0_A::VALUE1)
    }
    #[doc = "Set boundary flag BFLy if result is below the defined band or compare value, clear if above"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(BFA0_A::VALUE2)
    }
}
#[doc = "Boundary Flag 1 Activation Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BFA1_A {
    #[doc = "0: Set boundary flag BFLy if result is above the defined band or compare value, clear if below"]
    VALUE1 = 0,
    #[doc = "1: Set boundary flag BFLy if result is below the defined band or compare value, clear if above"]
    VALUE2 = 1,
}
impl From<BFA1_A> for bool {
    #[inline(always)]
    fn from(variant: BFA1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BFA1` reader - Boundary Flag 1 Activation Select"]
pub type BFA1_R = crate::BitReader<BFA1_A>;
impl BFA1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BFA1_A {
        match self.bits {
            false => BFA1_A::VALUE1,
            true => BFA1_A::VALUE2,
        }
    }
    #[doc = "Set boundary flag BFLy if result is above the defined band or compare value, clear if below"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == BFA1_A::VALUE1
    }
    #[doc = "Set boundary flag BFLy if result is below the defined band or compare value, clear if above"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == BFA1_A::VALUE2
    }
}
#[doc = "Field `BFA1` writer - Boundary Flag 1 Activation Select"]
pub type BFA1_W<'a, REG> = crate::BitWriter<'a, REG, BFA1_A>;
impl<'a, REG> BFA1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Set boundary flag BFLy if result is above the defined band or compare value, clear if below"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(BFA1_A::VALUE1)
    }
    #[doc = "Set boundary flag BFLy if result is below the defined band or compare value, clear if above"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(BFA1_A::VALUE2)
    }
}
#[doc = "Boundary Flag 2 Activation Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BFA2_A {
    #[doc = "0: Set boundary flag BFLy if result is above the defined band or compare value, clear if below"]
    VALUE1 = 0,
    #[doc = "1: Set boundary flag BFLy if result is below the defined band or compare value, clear if above"]
    VALUE2 = 1,
}
impl From<BFA2_A> for bool {
    #[inline(always)]
    fn from(variant: BFA2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BFA2` reader - Boundary Flag 2 Activation Select"]
pub type BFA2_R = crate::BitReader<BFA2_A>;
impl BFA2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BFA2_A {
        match self.bits {
            false => BFA2_A::VALUE1,
            true => BFA2_A::VALUE2,
        }
    }
    #[doc = "Set boundary flag BFLy if result is above the defined band or compare value, clear if below"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == BFA2_A::VALUE1
    }
    #[doc = "Set boundary flag BFLy if result is below the defined band or compare value, clear if above"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == BFA2_A::VALUE2
    }
}
#[doc = "Field `BFA2` writer - Boundary Flag 2 Activation Select"]
pub type BFA2_W<'a, REG> = crate::BitWriter<'a, REG, BFA2_A>;
impl<'a, REG> BFA2_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Set boundary flag BFLy if result is above the defined band or compare value, clear if below"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(BFA2_A::VALUE1)
    }
    #[doc = "Set boundary flag BFLy if result is below the defined band or compare value, clear if above"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(BFA2_A::VALUE2)
    }
}
#[doc = "Boundary Flag 3 Activation Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BFA3_A {
    #[doc = "0: Set boundary flag BFLy if result is above the defined band or compare value, clear if below"]
    VALUE1 = 0,
    #[doc = "1: Set boundary flag BFLy if result is below the defined band or compare value, clear if above"]
    VALUE2 = 1,
}
impl From<BFA3_A> for bool {
    #[inline(always)]
    fn from(variant: BFA3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BFA3` reader - Boundary Flag 3 Activation Select"]
pub type BFA3_R = crate::BitReader<BFA3_A>;
impl BFA3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BFA3_A {
        match self.bits {
            false => BFA3_A::VALUE1,
            true => BFA3_A::VALUE2,
        }
    }
    #[doc = "Set boundary flag BFLy if result is above the defined band or compare value, clear if below"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == BFA3_A::VALUE1
    }
    #[doc = "Set boundary flag BFLy if result is below the defined band or compare value, clear if above"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == BFA3_A::VALUE2
    }
}
#[doc = "Field `BFA3` writer - Boundary Flag 3 Activation Select"]
pub type BFA3_W<'a, REG> = crate::BitWriter<'a, REG, BFA3_A>;
impl<'a, REG> BFA3_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Set boundary flag BFLy if result is above the defined band or compare value, clear if below"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(BFA3_A::VALUE1)
    }
    #[doc = "Set boundary flag BFLy if result is below the defined band or compare value, clear if above"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(BFA3_A::VALUE2)
    }
}
#[doc = "Boundary Flag 0 Inversion Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BFI0_A {
    #[doc = "0: Use BFLy directly"]
    VALUE1 = 0,
    #[doc = "1: Invert value and use BFLy"]
    VALUE2 = 1,
}
impl From<BFI0_A> for bool {
    #[inline(always)]
    fn from(variant: BFI0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BFI0` reader - Boundary Flag 0 Inversion Control"]
pub type BFI0_R = crate::BitReader<BFI0_A>;
impl BFI0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BFI0_A {
        match self.bits {
            false => BFI0_A::VALUE1,
            true => BFI0_A::VALUE2,
        }
    }
    #[doc = "Use BFLy directly"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == BFI0_A::VALUE1
    }
    #[doc = "Invert value and use BFLy"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == BFI0_A::VALUE2
    }
}
#[doc = "Field `BFI0` writer - Boundary Flag 0 Inversion Control"]
pub type BFI0_W<'a, REG> = crate::BitWriter<'a, REG, BFI0_A>;
impl<'a, REG> BFI0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Use BFLy directly"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(BFI0_A::VALUE1)
    }
    #[doc = "Invert value and use BFLy"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(BFI0_A::VALUE2)
    }
}
#[doc = "Boundary Flag 1 Inversion Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BFI1_A {
    #[doc = "0: Use BFLy directly"]
    VALUE1 = 0,
    #[doc = "1: Invert value and use BFLy"]
    VALUE2 = 1,
}
impl From<BFI1_A> for bool {
    #[inline(always)]
    fn from(variant: BFI1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BFI1` reader - Boundary Flag 1 Inversion Control"]
pub type BFI1_R = crate::BitReader<BFI1_A>;
impl BFI1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BFI1_A {
        match self.bits {
            false => BFI1_A::VALUE1,
            true => BFI1_A::VALUE2,
        }
    }
    #[doc = "Use BFLy directly"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == BFI1_A::VALUE1
    }
    #[doc = "Invert value and use BFLy"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == BFI1_A::VALUE2
    }
}
#[doc = "Field `BFI1` writer - Boundary Flag 1 Inversion Control"]
pub type BFI1_W<'a, REG> = crate::BitWriter<'a, REG, BFI1_A>;
impl<'a, REG> BFI1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Use BFLy directly"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(BFI1_A::VALUE1)
    }
    #[doc = "Invert value and use BFLy"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(BFI1_A::VALUE2)
    }
}
#[doc = "Boundary Flag 2 Inversion Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BFI2_A {
    #[doc = "0: Use BFLy directly"]
    VALUE1 = 0,
    #[doc = "1: Invert value and use BFLy"]
    VALUE2 = 1,
}
impl From<BFI2_A> for bool {
    #[inline(always)]
    fn from(variant: BFI2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BFI2` reader - Boundary Flag 2 Inversion Control"]
pub type BFI2_R = crate::BitReader<BFI2_A>;
impl BFI2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BFI2_A {
        match self.bits {
            false => BFI2_A::VALUE1,
            true => BFI2_A::VALUE2,
        }
    }
    #[doc = "Use BFLy directly"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == BFI2_A::VALUE1
    }
    #[doc = "Invert value and use BFLy"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == BFI2_A::VALUE2
    }
}
#[doc = "Field `BFI2` writer - Boundary Flag 2 Inversion Control"]
pub type BFI2_W<'a, REG> = crate::BitWriter<'a, REG, BFI2_A>;
impl<'a, REG> BFI2_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Use BFLy directly"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(BFI2_A::VALUE1)
    }
    #[doc = "Invert value and use BFLy"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(BFI2_A::VALUE2)
    }
}
#[doc = "Boundary Flag 3 Inversion Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BFI3_A {
    #[doc = "0: Use BFLy directly"]
    VALUE1 = 0,
    #[doc = "1: Invert value and use BFLy"]
    VALUE2 = 1,
}
impl From<BFI3_A> for bool {
    #[inline(always)]
    fn from(variant: BFI3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BFI3` reader - Boundary Flag 3 Inversion Control"]
pub type BFI3_R = crate::BitReader<BFI3_A>;
impl BFI3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BFI3_A {
        match self.bits {
            false => BFI3_A::VALUE1,
            true => BFI3_A::VALUE2,
        }
    }
    #[doc = "Use BFLy directly"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == BFI3_A::VALUE1
    }
    #[doc = "Invert value and use BFLy"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == BFI3_A::VALUE2
    }
}
#[doc = "Field `BFI3` writer - Boundary Flag 3 Inversion Control"]
pub type BFI3_W<'a, REG> = crate::BitWriter<'a, REG, BFI3_A>;
impl<'a, REG> BFI3_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Use BFLy directly"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(BFI3_A::VALUE1)
    }
    #[doc = "Invert value and use BFLy"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(BFI3_A::VALUE2)
    }
}
impl R {
    #[doc = "Bit 0 - Boundary Flag 0"]
    #[inline(always)]
    pub fn bfl0(&self) -> BFL0_R {
        BFL0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Boundary Flag 1"]
    #[inline(always)]
    pub fn bfl1(&self) -> BFL1_R {
        BFL1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Boundary Flag 2"]
    #[inline(always)]
    pub fn bfl2(&self) -> BFL2_R {
        BFL2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Boundary Flag 3"]
    #[inline(always)]
    pub fn bfl3(&self) -> BFL3_R {
        BFL3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 8 - Boundary Flag 0 Activation Select"]
    #[inline(always)]
    pub fn bfa0(&self) -> BFA0_R {
        BFA0_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Boundary Flag 1 Activation Select"]
    #[inline(always)]
    pub fn bfa1(&self) -> BFA1_R {
        BFA1_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Boundary Flag 2 Activation Select"]
    #[inline(always)]
    pub fn bfa2(&self) -> BFA2_R {
        BFA2_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Boundary Flag 3 Activation Select"]
    #[inline(always)]
    pub fn bfa3(&self) -> BFA3_R {
        BFA3_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 16 - Boundary Flag 0 Inversion Control"]
    #[inline(always)]
    pub fn bfi0(&self) -> BFI0_R {
        BFI0_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Boundary Flag 1 Inversion Control"]
    #[inline(always)]
    pub fn bfi1(&self) -> BFI1_R {
        BFI1_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Boundary Flag 2 Inversion Control"]
    #[inline(always)]
    pub fn bfi2(&self) -> BFI2_R {
        BFI2_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Boundary Flag 3 Inversion Control"]
    #[inline(always)]
    pub fn bfi3(&self) -> BFI3_R {
        BFI3_R::new(((self.bits >> 19) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 8 - Boundary Flag 0 Activation Select"]
    #[inline(always)]
    pub fn bfa0(&mut self) -> BFA0_W<BFL_SPEC> {
        BFA0_W::new(self, 8)
    }
    #[doc = "Bit 9 - Boundary Flag 1 Activation Select"]
    #[inline(always)]
    pub fn bfa1(&mut self) -> BFA1_W<BFL_SPEC> {
        BFA1_W::new(self, 9)
    }
    #[doc = "Bit 10 - Boundary Flag 2 Activation Select"]
    #[inline(always)]
    pub fn bfa2(&mut self) -> BFA2_W<BFL_SPEC> {
        BFA2_W::new(self, 10)
    }
    #[doc = "Bit 11 - Boundary Flag 3 Activation Select"]
    #[inline(always)]
    pub fn bfa3(&mut self) -> BFA3_W<BFL_SPEC> {
        BFA3_W::new(self, 11)
    }
    #[doc = "Bit 16 - Boundary Flag 0 Inversion Control"]
    #[inline(always)]
    pub fn bfi0(&mut self) -> BFI0_W<BFL_SPEC> {
        BFI0_W::new(self, 16)
    }
    #[doc = "Bit 17 - Boundary Flag 1 Inversion Control"]
    #[inline(always)]
    pub fn bfi1(&mut self) -> BFI1_W<BFL_SPEC> {
        BFI1_W::new(self, 17)
    }
    #[doc = "Bit 18 - Boundary Flag 2 Inversion Control"]
    #[inline(always)]
    pub fn bfi2(&mut self) -> BFI2_W<BFL_SPEC> {
        BFI2_W::new(self, 18)
    }
    #[doc = "Bit 19 - Boundary Flag 3 Inversion Control"]
    #[inline(always)]
    pub fn bfi3(&mut self) -> BFI3_W<BFL_SPEC> {
        BFI3_W::new(self, 19)
    }
}
#[doc = "Boundary Flag Register\n\nYou can [`read`](crate::Reg::read) this register and get [`bfl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bfl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BFL_SPEC;
impl crate::RegisterSpec for BFL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bfl::R`](R) reader structure"]
impl crate::Readable for BFL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`bfl::W`](W) writer structure"]
impl crate::Writable for BFL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BFL to value 0"]
impl crate::Resettable for BFL_SPEC {
    const RESET_VALUE: u32 = 0;
}
