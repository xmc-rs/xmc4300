#[doc = "Register `BFL` reader"]
pub type R = crate::R<BflSpec>;
#[doc = "Register `BFL` writer"]
pub type W = crate::W<BflSpec>;
#[doc = "Boundary Flag 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bfl0 {
    #[doc = "0: Passive state: result has not yet crossed the activation boundary (see bitfield BFAy), or selected gate signal is inactive, or this boundary flag is disabled"]
    Value1 = 0,
    #[doc = "1: Active state: result has crossed the activation boundary"]
    Value2 = 1,
}
impl From<Bfl0> for bool {
    #[inline(always)]
    fn from(variant: Bfl0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BFL0` reader - Boundary Flag 0"]
pub type Bfl0R = crate::BitReader<Bfl0>;
impl Bfl0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bfl0 {
        match self.bits {
            false => Bfl0::Value1,
            true => Bfl0::Value2,
        }
    }
    #[doc = "Passive state: result has not yet crossed the activation boundary (see bitfield BFAy), or selected gate signal is inactive, or this boundary flag is disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Bfl0::Value1
    }
    #[doc = "Active state: result has crossed the activation boundary"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Bfl0::Value2
    }
}
#[doc = "Boundary Flag 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bfl1 {
    #[doc = "0: Passive state: result has not yet crossed the activation boundary (see bitfield BFAy), or selected gate signal is inactive, or this boundary flag is disabled"]
    Value1 = 0,
    #[doc = "1: Active state: result has crossed the activation boundary"]
    Value2 = 1,
}
impl From<Bfl1> for bool {
    #[inline(always)]
    fn from(variant: Bfl1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BFL1` reader - Boundary Flag 1"]
pub type Bfl1R = crate::BitReader<Bfl1>;
impl Bfl1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bfl1 {
        match self.bits {
            false => Bfl1::Value1,
            true => Bfl1::Value2,
        }
    }
    #[doc = "Passive state: result has not yet crossed the activation boundary (see bitfield BFAy), or selected gate signal is inactive, or this boundary flag is disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Bfl1::Value1
    }
    #[doc = "Active state: result has crossed the activation boundary"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Bfl1::Value2
    }
}
#[doc = "Boundary Flag 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bfl2 {
    #[doc = "0: Passive state: result has not yet crossed the activation boundary (see bitfield BFAy), or selected gate signal is inactive, or this boundary flag is disabled"]
    Value1 = 0,
    #[doc = "1: Active state: result has crossed the activation boundary"]
    Value2 = 1,
}
impl From<Bfl2> for bool {
    #[inline(always)]
    fn from(variant: Bfl2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BFL2` reader - Boundary Flag 2"]
pub type Bfl2R = crate::BitReader<Bfl2>;
impl Bfl2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bfl2 {
        match self.bits {
            false => Bfl2::Value1,
            true => Bfl2::Value2,
        }
    }
    #[doc = "Passive state: result has not yet crossed the activation boundary (see bitfield BFAy), or selected gate signal is inactive, or this boundary flag is disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Bfl2::Value1
    }
    #[doc = "Active state: result has crossed the activation boundary"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Bfl2::Value2
    }
}
#[doc = "Boundary Flag 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bfl3 {
    #[doc = "0: Passive state: result has not yet crossed the activation boundary (see bitfield BFAy), or selected gate signal is inactive, or this boundary flag is disabled"]
    Value1 = 0,
    #[doc = "1: Active state: result has crossed the activation boundary"]
    Value2 = 1,
}
impl From<Bfl3> for bool {
    #[inline(always)]
    fn from(variant: Bfl3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BFL3` reader - Boundary Flag 3"]
pub type Bfl3R = crate::BitReader<Bfl3>;
impl Bfl3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bfl3 {
        match self.bits {
            false => Bfl3::Value1,
            true => Bfl3::Value2,
        }
    }
    #[doc = "Passive state: result has not yet crossed the activation boundary (see bitfield BFAy), or selected gate signal is inactive, or this boundary flag is disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Bfl3::Value1
    }
    #[doc = "Active state: result has crossed the activation boundary"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Bfl3::Value2
    }
}
#[doc = "Boundary Flag 0 Activation Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bfa0 {
    #[doc = "0: Set boundary flag BFLy if result is above the defined band or compare value, clear if below"]
    Value1 = 0,
    #[doc = "1: Set boundary flag BFLy if result is below the defined band or compare value, clear if above"]
    Value2 = 1,
}
impl From<Bfa0> for bool {
    #[inline(always)]
    fn from(variant: Bfa0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BFA0` reader - Boundary Flag 0 Activation Select"]
pub type Bfa0R = crate::BitReader<Bfa0>;
impl Bfa0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bfa0 {
        match self.bits {
            false => Bfa0::Value1,
            true => Bfa0::Value2,
        }
    }
    #[doc = "Set boundary flag BFLy if result is above the defined band or compare value, clear if below"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Bfa0::Value1
    }
    #[doc = "Set boundary flag BFLy if result is below the defined band or compare value, clear if above"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Bfa0::Value2
    }
}
#[doc = "Field `BFA0` writer - Boundary Flag 0 Activation Select"]
pub type Bfa0W<'a, REG> = crate::BitWriter<'a, REG, Bfa0>;
impl<'a, REG> Bfa0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Set boundary flag BFLy if result is above the defined band or compare value, clear if below"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Bfa0::Value1)
    }
    #[doc = "Set boundary flag BFLy if result is below the defined band or compare value, clear if above"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Bfa0::Value2)
    }
}
#[doc = "Boundary Flag 1 Activation Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bfa1 {
    #[doc = "0: Set boundary flag BFLy if result is above the defined band or compare value, clear if below"]
    Value1 = 0,
    #[doc = "1: Set boundary flag BFLy if result is below the defined band or compare value, clear if above"]
    Value2 = 1,
}
impl From<Bfa1> for bool {
    #[inline(always)]
    fn from(variant: Bfa1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BFA1` reader - Boundary Flag 1 Activation Select"]
pub type Bfa1R = crate::BitReader<Bfa1>;
impl Bfa1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bfa1 {
        match self.bits {
            false => Bfa1::Value1,
            true => Bfa1::Value2,
        }
    }
    #[doc = "Set boundary flag BFLy if result is above the defined band or compare value, clear if below"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Bfa1::Value1
    }
    #[doc = "Set boundary flag BFLy if result is below the defined band or compare value, clear if above"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Bfa1::Value2
    }
}
#[doc = "Field `BFA1` writer - Boundary Flag 1 Activation Select"]
pub type Bfa1W<'a, REG> = crate::BitWriter<'a, REG, Bfa1>;
impl<'a, REG> Bfa1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Set boundary flag BFLy if result is above the defined band or compare value, clear if below"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Bfa1::Value1)
    }
    #[doc = "Set boundary flag BFLy if result is below the defined band or compare value, clear if above"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Bfa1::Value2)
    }
}
#[doc = "Boundary Flag 2 Activation Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bfa2 {
    #[doc = "0: Set boundary flag BFLy if result is above the defined band or compare value, clear if below"]
    Value1 = 0,
    #[doc = "1: Set boundary flag BFLy if result is below the defined band or compare value, clear if above"]
    Value2 = 1,
}
impl From<Bfa2> for bool {
    #[inline(always)]
    fn from(variant: Bfa2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BFA2` reader - Boundary Flag 2 Activation Select"]
pub type Bfa2R = crate::BitReader<Bfa2>;
impl Bfa2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bfa2 {
        match self.bits {
            false => Bfa2::Value1,
            true => Bfa2::Value2,
        }
    }
    #[doc = "Set boundary flag BFLy if result is above the defined band or compare value, clear if below"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Bfa2::Value1
    }
    #[doc = "Set boundary flag BFLy if result is below the defined band or compare value, clear if above"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Bfa2::Value2
    }
}
#[doc = "Field `BFA2` writer - Boundary Flag 2 Activation Select"]
pub type Bfa2W<'a, REG> = crate::BitWriter<'a, REG, Bfa2>;
impl<'a, REG> Bfa2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Set boundary flag BFLy if result is above the defined band or compare value, clear if below"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Bfa2::Value1)
    }
    #[doc = "Set boundary flag BFLy if result is below the defined band or compare value, clear if above"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Bfa2::Value2)
    }
}
#[doc = "Boundary Flag 3 Activation Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bfa3 {
    #[doc = "0: Set boundary flag BFLy if result is above the defined band or compare value, clear if below"]
    Value1 = 0,
    #[doc = "1: Set boundary flag BFLy if result is below the defined band or compare value, clear if above"]
    Value2 = 1,
}
impl From<Bfa3> for bool {
    #[inline(always)]
    fn from(variant: Bfa3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BFA3` reader - Boundary Flag 3 Activation Select"]
pub type Bfa3R = crate::BitReader<Bfa3>;
impl Bfa3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bfa3 {
        match self.bits {
            false => Bfa3::Value1,
            true => Bfa3::Value2,
        }
    }
    #[doc = "Set boundary flag BFLy if result is above the defined band or compare value, clear if below"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Bfa3::Value1
    }
    #[doc = "Set boundary flag BFLy if result is below the defined band or compare value, clear if above"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Bfa3::Value2
    }
}
#[doc = "Field `BFA3` writer - Boundary Flag 3 Activation Select"]
pub type Bfa3W<'a, REG> = crate::BitWriter<'a, REG, Bfa3>;
impl<'a, REG> Bfa3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Set boundary flag BFLy if result is above the defined band or compare value, clear if below"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Bfa3::Value1)
    }
    #[doc = "Set boundary flag BFLy if result is below the defined band or compare value, clear if above"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Bfa3::Value2)
    }
}
#[doc = "Boundary Flag 0 Inversion Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bfi0 {
    #[doc = "0: Use BFLy directly"]
    Value1 = 0,
    #[doc = "1: Invert value and use BFLy"]
    Value2 = 1,
}
impl From<Bfi0> for bool {
    #[inline(always)]
    fn from(variant: Bfi0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BFI0` reader - Boundary Flag 0 Inversion Control"]
pub type Bfi0R = crate::BitReader<Bfi0>;
impl Bfi0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bfi0 {
        match self.bits {
            false => Bfi0::Value1,
            true => Bfi0::Value2,
        }
    }
    #[doc = "Use BFLy directly"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Bfi0::Value1
    }
    #[doc = "Invert value and use BFLy"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Bfi0::Value2
    }
}
#[doc = "Field `BFI0` writer - Boundary Flag 0 Inversion Control"]
pub type Bfi0W<'a, REG> = crate::BitWriter<'a, REG, Bfi0>;
impl<'a, REG> Bfi0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Use BFLy directly"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Bfi0::Value1)
    }
    #[doc = "Invert value and use BFLy"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Bfi0::Value2)
    }
}
#[doc = "Boundary Flag 1 Inversion Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bfi1 {
    #[doc = "0: Use BFLy directly"]
    Value1 = 0,
    #[doc = "1: Invert value and use BFLy"]
    Value2 = 1,
}
impl From<Bfi1> for bool {
    #[inline(always)]
    fn from(variant: Bfi1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BFI1` reader - Boundary Flag 1 Inversion Control"]
pub type Bfi1R = crate::BitReader<Bfi1>;
impl Bfi1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bfi1 {
        match self.bits {
            false => Bfi1::Value1,
            true => Bfi1::Value2,
        }
    }
    #[doc = "Use BFLy directly"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Bfi1::Value1
    }
    #[doc = "Invert value and use BFLy"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Bfi1::Value2
    }
}
#[doc = "Field `BFI1` writer - Boundary Flag 1 Inversion Control"]
pub type Bfi1W<'a, REG> = crate::BitWriter<'a, REG, Bfi1>;
impl<'a, REG> Bfi1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Use BFLy directly"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Bfi1::Value1)
    }
    #[doc = "Invert value and use BFLy"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Bfi1::Value2)
    }
}
#[doc = "Boundary Flag 2 Inversion Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bfi2 {
    #[doc = "0: Use BFLy directly"]
    Value1 = 0,
    #[doc = "1: Invert value and use BFLy"]
    Value2 = 1,
}
impl From<Bfi2> for bool {
    #[inline(always)]
    fn from(variant: Bfi2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BFI2` reader - Boundary Flag 2 Inversion Control"]
pub type Bfi2R = crate::BitReader<Bfi2>;
impl Bfi2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bfi2 {
        match self.bits {
            false => Bfi2::Value1,
            true => Bfi2::Value2,
        }
    }
    #[doc = "Use BFLy directly"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Bfi2::Value1
    }
    #[doc = "Invert value and use BFLy"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Bfi2::Value2
    }
}
#[doc = "Field `BFI2` writer - Boundary Flag 2 Inversion Control"]
pub type Bfi2W<'a, REG> = crate::BitWriter<'a, REG, Bfi2>;
impl<'a, REG> Bfi2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Use BFLy directly"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Bfi2::Value1)
    }
    #[doc = "Invert value and use BFLy"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Bfi2::Value2)
    }
}
#[doc = "Boundary Flag 3 Inversion Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bfi3 {
    #[doc = "0: Use BFLy directly"]
    Value1 = 0,
    #[doc = "1: Invert value and use BFLy"]
    Value2 = 1,
}
impl From<Bfi3> for bool {
    #[inline(always)]
    fn from(variant: Bfi3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BFI3` reader - Boundary Flag 3 Inversion Control"]
pub type Bfi3R = crate::BitReader<Bfi3>;
impl Bfi3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bfi3 {
        match self.bits {
            false => Bfi3::Value1,
            true => Bfi3::Value2,
        }
    }
    #[doc = "Use BFLy directly"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Bfi3::Value1
    }
    #[doc = "Invert value and use BFLy"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Bfi3::Value2
    }
}
#[doc = "Field `BFI3` writer - Boundary Flag 3 Inversion Control"]
pub type Bfi3W<'a, REG> = crate::BitWriter<'a, REG, Bfi3>;
impl<'a, REG> Bfi3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Use BFLy directly"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Bfi3::Value1)
    }
    #[doc = "Invert value and use BFLy"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Bfi3::Value2)
    }
}
impl R {
    #[doc = "Bit 0 - Boundary Flag 0"]
    #[inline(always)]
    pub fn bfl0(&self) -> Bfl0R {
        Bfl0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Boundary Flag 1"]
    #[inline(always)]
    pub fn bfl1(&self) -> Bfl1R {
        Bfl1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Boundary Flag 2"]
    #[inline(always)]
    pub fn bfl2(&self) -> Bfl2R {
        Bfl2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Boundary Flag 3"]
    #[inline(always)]
    pub fn bfl3(&self) -> Bfl3R {
        Bfl3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 8 - Boundary Flag 0 Activation Select"]
    #[inline(always)]
    pub fn bfa0(&self) -> Bfa0R {
        Bfa0R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Boundary Flag 1 Activation Select"]
    #[inline(always)]
    pub fn bfa1(&self) -> Bfa1R {
        Bfa1R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Boundary Flag 2 Activation Select"]
    #[inline(always)]
    pub fn bfa2(&self) -> Bfa2R {
        Bfa2R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Boundary Flag 3 Activation Select"]
    #[inline(always)]
    pub fn bfa3(&self) -> Bfa3R {
        Bfa3R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 16 - Boundary Flag 0 Inversion Control"]
    #[inline(always)]
    pub fn bfi0(&self) -> Bfi0R {
        Bfi0R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Boundary Flag 1 Inversion Control"]
    #[inline(always)]
    pub fn bfi1(&self) -> Bfi1R {
        Bfi1R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Boundary Flag 2 Inversion Control"]
    #[inline(always)]
    pub fn bfi2(&self) -> Bfi2R {
        Bfi2R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Boundary Flag 3 Inversion Control"]
    #[inline(always)]
    pub fn bfi3(&self) -> Bfi3R {
        Bfi3R::new(((self.bits >> 19) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 8 - Boundary Flag 0 Activation Select"]
    #[inline(always)]
    #[must_use]
    pub fn bfa0(&mut self) -> Bfa0W<BflSpec> {
        Bfa0W::new(self, 8)
    }
    #[doc = "Bit 9 - Boundary Flag 1 Activation Select"]
    #[inline(always)]
    #[must_use]
    pub fn bfa1(&mut self) -> Bfa1W<BflSpec> {
        Bfa1W::new(self, 9)
    }
    #[doc = "Bit 10 - Boundary Flag 2 Activation Select"]
    #[inline(always)]
    #[must_use]
    pub fn bfa2(&mut self) -> Bfa2W<BflSpec> {
        Bfa2W::new(self, 10)
    }
    #[doc = "Bit 11 - Boundary Flag 3 Activation Select"]
    #[inline(always)]
    #[must_use]
    pub fn bfa3(&mut self) -> Bfa3W<BflSpec> {
        Bfa3W::new(self, 11)
    }
    #[doc = "Bit 16 - Boundary Flag 0 Inversion Control"]
    #[inline(always)]
    #[must_use]
    pub fn bfi0(&mut self) -> Bfi0W<BflSpec> {
        Bfi0W::new(self, 16)
    }
    #[doc = "Bit 17 - Boundary Flag 1 Inversion Control"]
    #[inline(always)]
    #[must_use]
    pub fn bfi1(&mut self) -> Bfi1W<BflSpec> {
        Bfi1W::new(self, 17)
    }
    #[doc = "Bit 18 - Boundary Flag 2 Inversion Control"]
    #[inline(always)]
    #[must_use]
    pub fn bfi2(&mut self) -> Bfi2W<BflSpec> {
        Bfi2W::new(self, 18)
    }
    #[doc = "Bit 19 - Boundary Flag 3 Inversion Control"]
    #[inline(always)]
    #[must_use]
    pub fn bfi3(&mut self) -> Bfi3W<BflSpec> {
        Bfi3W::new(self, 19)
    }
}
#[doc = "Boundary Flag Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bfl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bfl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BflSpec;
impl crate::RegisterSpec for BflSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bfl::R`](R) reader structure"]
impl crate::Readable for BflSpec {}
#[doc = "`write(|w| ..)` method takes [`bfl::W`](W) writer structure"]
impl crate::Writable for BflSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BFL to value 0"]
impl crate::Resettable for BflSpec {
    const RESET_VALUE: u32 = 0;
}
