#[doc = "Register `VFR` reader"]
pub type R = crate::R<VfrSpec>;
#[doc = "Register `VFR` writer"]
pub type W = crate::W<VfrSpec>;
#[doc = "Valid Flag of Result Register x\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Vf0 {
    #[doc = "0: Read access: No new valid data available Write access: No effect"]
    Value1 = 0,
    #[doc = "1: Read access: Result register x contains valid data and has not yet been read, or bit FCR has been updated Write access: Clear this valid flag and bitfield DRC in register GxRESy (overrides a hardware set action)"]
    Value2 = 1,
}
impl From<Vf0> for bool {
    #[inline(always)]
    fn from(variant: Vf0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VF0` reader - Valid Flag of Result Register x"]
pub type Vf0R = crate::BitReader<Vf0>;
impl Vf0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Vf0 {
        match self.bits {
            false => Vf0::Value1,
            true => Vf0::Value2,
        }
    }
    #[doc = "Read access: No new valid data available Write access: No effect"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Vf0::Value1
    }
    #[doc = "Read access: Result register x contains valid data and has not yet been read, or bit FCR has been updated Write access: Clear this valid flag and bitfield DRC in register GxRESy (overrides a hardware set action)"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Vf0::Value2
    }
}
#[doc = "Field `VF0` writer - Valid Flag of Result Register x"]
pub type Vf0W<'a, REG> = crate::BitWriter<'a, REG, Vf0>;
impl<'a, REG> Vf0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Read access: No new valid data available Write access: No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Vf0::Value1)
    }
    #[doc = "Read access: Result register x contains valid data and has not yet been read, or bit FCR has been updated Write access: Clear this valid flag and bitfield DRC in register GxRESy (overrides a hardware set action)"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Vf0::Value2)
    }
}
#[doc = "Valid Flag of Result Register x\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Vf1 {
    #[doc = "0: Read access: No new valid data available Write access: No effect"]
    Value1 = 0,
    #[doc = "1: Read access: Result register x contains valid data and has not yet been read, or bit FCR has been updated Write access: Clear this valid flag and bitfield DRC in register GxRESy (overrides a hardware set action)"]
    Value2 = 1,
}
impl From<Vf1> for bool {
    #[inline(always)]
    fn from(variant: Vf1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VF1` reader - Valid Flag of Result Register x"]
pub type Vf1R = crate::BitReader<Vf1>;
impl Vf1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Vf1 {
        match self.bits {
            false => Vf1::Value1,
            true => Vf1::Value2,
        }
    }
    #[doc = "Read access: No new valid data available Write access: No effect"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Vf1::Value1
    }
    #[doc = "Read access: Result register x contains valid data and has not yet been read, or bit FCR has been updated Write access: Clear this valid flag and bitfield DRC in register GxRESy (overrides a hardware set action)"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Vf1::Value2
    }
}
#[doc = "Field `VF1` writer - Valid Flag of Result Register x"]
pub type Vf1W<'a, REG> = crate::BitWriter<'a, REG, Vf1>;
impl<'a, REG> Vf1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Read access: No new valid data available Write access: No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Vf1::Value1)
    }
    #[doc = "Read access: Result register x contains valid data and has not yet been read, or bit FCR has been updated Write access: Clear this valid flag and bitfield DRC in register GxRESy (overrides a hardware set action)"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Vf1::Value2)
    }
}
#[doc = "Valid Flag of Result Register x\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Vf2 {
    #[doc = "0: Read access: No new valid data available Write access: No effect"]
    Value1 = 0,
    #[doc = "1: Read access: Result register x contains valid data and has not yet been read, or bit FCR has been updated Write access: Clear this valid flag and bitfield DRC in register GxRESy (overrides a hardware set action)"]
    Value2 = 1,
}
impl From<Vf2> for bool {
    #[inline(always)]
    fn from(variant: Vf2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VF2` reader - Valid Flag of Result Register x"]
pub type Vf2R = crate::BitReader<Vf2>;
impl Vf2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Vf2 {
        match self.bits {
            false => Vf2::Value1,
            true => Vf2::Value2,
        }
    }
    #[doc = "Read access: No new valid data available Write access: No effect"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Vf2::Value1
    }
    #[doc = "Read access: Result register x contains valid data and has not yet been read, or bit FCR has been updated Write access: Clear this valid flag and bitfield DRC in register GxRESy (overrides a hardware set action)"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Vf2::Value2
    }
}
#[doc = "Field `VF2` writer - Valid Flag of Result Register x"]
pub type Vf2W<'a, REG> = crate::BitWriter<'a, REG, Vf2>;
impl<'a, REG> Vf2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Read access: No new valid data available Write access: No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Vf2::Value1)
    }
    #[doc = "Read access: Result register x contains valid data and has not yet been read, or bit FCR has been updated Write access: Clear this valid flag and bitfield DRC in register GxRESy (overrides a hardware set action)"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Vf2::Value2)
    }
}
#[doc = "Valid Flag of Result Register x\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Vf3 {
    #[doc = "0: Read access: No new valid data available Write access: No effect"]
    Value1 = 0,
    #[doc = "1: Read access: Result register x contains valid data and has not yet been read, or bit FCR has been updated Write access: Clear this valid flag and bitfield DRC in register GxRESy (overrides a hardware set action)"]
    Value2 = 1,
}
impl From<Vf3> for bool {
    #[inline(always)]
    fn from(variant: Vf3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VF3` reader - Valid Flag of Result Register x"]
pub type Vf3R = crate::BitReader<Vf3>;
impl Vf3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Vf3 {
        match self.bits {
            false => Vf3::Value1,
            true => Vf3::Value2,
        }
    }
    #[doc = "Read access: No new valid data available Write access: No effect"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Vf3::Value1
    }
    #[doc = "Read access: Result register x contains valid data and has not yet been read, or bit FCR has been updated Write access: Clear this valid flag and bitfield DRC in register GxRESy (overrides a hardware set action)"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Vf3::Value2
    }
}
#[doc = "Field `VF3` writer - Valid Flag of Result Register x"]
pub type Vf3W<'a, REG> = crate::BitWriter<'a, REG, Vf3>;
impl<'a, REG> Vf3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Read access: No new valid data available Write access: No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Vf3::Value1)
    }
    #[doc = "Read access: Result register x contains valid data and has not yet been read, or bit FCR has been updated Write access: Clear this valid flag and bitfield DRC in register GxRESy (overrides a hardware set action)"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Vf3::Value2)
    }
}
#[doc = "Valid Flag of Result Register x\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Vf4 {
    #[doc = "0: Read access: No new valid data available Write access: No effect"]
    Value1 = 0,
    #[doc = "1: Read access: Result register x contains valid data and has not yet been read, or bit FCR has been updated Write access: Clear this valid flag and bitfield DRC in register GxRESy (overrides a hardware set action)"]
    Value2 = 1,
}
impl From<Vf4> for bool {
    #[inline(always)]
    fn from(variant: Vf4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VF4` reader - Valid Flag of Result Register x"]
pub type Vf4R = crate::BitReader<Vf4>;
impl Vf4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Vf4 {
        match self.bits {
            false => Vf4::Value1,
            true => Vf4::Value2,
        }
    }
    #[doc = "Read access: No new valid data available Write access: No effect"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Vf4::Value1
    }
    #[doc = "Read access: Result register x contains valid data and has not yet been read, or bit FCR has been updated Write access: Clear this valid flag and bitfield DRC in register GxRESy (overrides a hardware set action)"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Vf4::Value2
    }
}
#[doc = "Field `VF4` writer - Valid Flag of Result Register x"]
pub type Vf4W<'a, REG> = crate::BitWriter<'a, REG, Vf4>;
impl<'a, REG> Vf4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Read access: No new valid data available Write access: No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Vf4::Value1)
    }
    #[doc = "Read access: Result register x contains valid data and has not yet been read, or bit FCR has been updated Write access: Clear this valid flag and bitfield DRC in register GxRESy (overrides a hardware set action)"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Vf4::Value2)
    }
}
#[doc = "Valid Flag of Result Register x\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Vf5 {
    #[doc = "0: Read access: No new valid data available Write access: No effect"]
    Value1 = 0,
    #[doc = "1: Read access: Result register x contains valid data and has not yet been read, or bit FCR has been updated Write access: Clear this valid flag and bitfield DRC in register GxRESy (overrides a hardware set action)"]
    Value2 = 1,
}
impl From<Vf5> for bool {
    #[inline(always)]
    fn from(variant: Vf5) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VF5` reader - Valid Flag of Result Register x"]
pub type Vf5R = crate::BitReader<Vf5>;
impl Vf5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Vf5 {
        match self.bits {
            false => Vf5::Value1,
            true => Vf5::Value2,
        }
    }
    #[doc = "Read access: No new valid data available Write access: No effect"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Vf5::Value1
    }
    #[doc = "Read access: Result register x contains valid data and has not yet been read, or bit FCR has been updated Write access: Clear this valid flag and bitfield DRC in register GxRESy (overrides a hardware set action)"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Vf5::Value2
    }
}
#[doc = "Field `VF5` writer - Valid Flag of Result Register x"]
pub type Vf5W<'a, REG> = crate::BitWriter<'a, REG, Vf5>;
impl<'a, REG> Vf5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Read access: No new valid data available Write access: No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Vf5::Value1)
    }
    #[doc = "Read access: Result register x contains valid data and has not yet been read, or bit FCR has been updated Write access: Clear this valid flag and bitfield DRC in register GxRESy (overrides a hardware set action)"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Vf5::Value2)
    }
}
#[doc = "Valid Flag of Result Register x\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Vf6 {
    #[doc = "0: Read access: No new valid data available Write access: No effect"]
    Value1 = 0,
    #[doc = "1: Read access: Result register x contains valid data and has not yet been read, or bit FCR has been updated Write access: Clear this valid flag and bitfield DRC in register GxRESy (overrides a hardware set action)"]
    Value2 = 1,
}
impl From<Vf6> for bool {
    #[inline(always)]
    fn from(variant: Vf6) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VF6` reader - Valid Flag of Result Register x"]
pub type Vf6R = crate::BitReader<Vf6>;
impl Vf6R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Vf6 {
        match self.bits {
            false => Vf6::Value1,
            true => Vf6::Value2,
        }
    }
    #[doc = "Read access: No new valid data available Write access: No effect"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Vf6::Value1
    }
    #[doc = "Read access: Result register x contains valid data and has not yet been read, or bit FCR has been updated Write access: Clear this valid flag and bitfield DRC in register GxRESy (overrides a hardware set action)"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Vf6::Value2
    }
}
#[doc = "Field `VF6` writer - Valid Flag of Result Register x"]
pub type Vf6W<'a, REG> = crate::BitWriter<'a, REG, Vf6>;
impl<'a, REG> Vf6W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Read access: No new valid data available Write access: No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Vf6::Value1)
    }
    #[doc = "Read access: Result register x contains valid data and has not yet been read, or bit FCR has been updated Write access: Clear this valid flag and bitfield DRC in register GxRESy (overrides a hardware set action)"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Vf6::Value2)
    }
}
#[doc = "Valid Flag of Result Register x\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Vf7 {
    #[doc = "0: Read access: No new valid data available Write access: No effect"]
    Value1 = 0,
    #[doc = "1: Read access: Result register x contains valid data and has not yet been read, or bit FCR has been updated Write access: Clear this valid flag and bitfield DRC in register GxRESy (overrides a hardware set action)"]
    Value2 = 1,
}
impl From<Vf7> for bool {
    #[inline(always)]
    fn from(variant: Vf7) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VF7` reader - Valid Flag of Result Register x"]
pub type Vf7R = crate::BitReader<Vf7>;
impl Vf7R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Vf7 {
        match self.bits {
            false => Vf7::Value1,
            true => Vf7::Value2,
        }
    }
    #[doc = "Read access: No new valid data available Write access: No effect"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Vf7::Value1
    }
    #[doc = "Read access: Result register x contains valid data and has not yet been read, or bit FCR has been updated Write access: Clear this valid flag and bitfield DRC in register GxRESy (overrides a hardware set action)"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Vf7::Value2
    }
}
#[doc = "Field `VF7` writer - Valid Flag of Result Register x"]
pub type Vf7W<'a, REG> = crate::BitWriter<'a, REG, Vf7>;
impl<'a, REG> Vf7W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Read access: No new valid data available Write access: No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Vf7::Value1)
    }
    #[doc = "Read access: Result register x contains valid data and has not yet been read, or bit FCR has been updated Write access: Clear this valid flag and bitfield DRC in register GxRESy (overrides a hardware set action)"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Vf7::Value2)
    }
}
#[doc = "Valid Flag of Result Register x\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Vf8 {
    #[doc = "0: Read access: No new valid data available Write access: No effect"]
    Value1 = 0,
    #[doc = "1: Read access: Result register x contains valid data and has not yet been read, or bit FCR has been updated Write access: Clear this valid flag and bitfield DRC in register GxRESy (overrides a hardware set action)"]
    Value2 = 1,
}
impl From<Vf8> for bool {
    #[inline(always)]
    fn from(variant: Vf8) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VF8` reader - Valid Flag of Result Register x"]
pub type Vf8R = crate::BitReader<Vf8>;
impl Vf8R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Vf8 {
        match self.bits {
            false => Vf8::Value1,
            true => Vf8::Value2,
        }
    }
    #[doc = "Read access: No new valid data available Write access: No effect"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Vf8::Value1
    }
    #[doc = "Read access: Result register x contains valid data and has not yet been read, or bit FCR has been updated Write access: Clear this valid flag and bitfield DRC in register GxRESy (overrides a hardware set action)"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Vf8::Value2
    }
}
#[doc = "Field `VF8` writer - Valid Flag of Result Register x"]
pub type Vf8W<'a, REG> = crate::BitWriter<'a, REG, Vf8>;
impl<'a, REG> Vf8W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Read access: No new valid data available Write access: No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Vf8::Value1)
    }
    #[doc = "Read access: Result register x contains valid data and has not yet been read, or bit FCR has been updated Write access: Clear this valid flag and bitfield DRC in register GxRESy (overrides a hardware set action)"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Vf8::Value2)
    }
}
#[doc = "Valid Flag of Result Register x\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Vf9 {
    #[doc = "0: Read access: No new valid data available Write access: No effect"]
    Value1 = 0,
    #[doc = "1: Read access: Result register x contains valid data and has not yet been read, or bit FCR has been updated Write access: Clear this valid flag and bitfield DRC in register GxRESy (overrides a hardware set action)"]
    Value2 = 1,
}
impl From<Vf9> for bool {
    #[inline(always)]
    fn from(variant: Vf9) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VF9` reader - Valid Flag of Result Register x"]
pub type Vf9R = crate::BitReader<Vf9>;
impl Vf9R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Vf9 {
        match self.bits {
            false => Vf9::Value1,
            true => Vf9::Value2,
        }
    }
    #[doc = "Read access: No new valid data available Write access: No effect"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Vf9::Value1
    }
    #[doc = "Read access: Result register x contains valid data and has not yet been read, or bit FCR has been updated Write access: Clear this valid flag and bitfield DRC in register GxRESy (overrides a hardware set action)"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Vf9::Value2
    }
}
#[doc = "Field `VF9` writer - Valid Flag of Result Register x"]
pub type Vf9W<'a, REG> = crate::BitWriter<'a, REG, Vf9>;
impl<'a, REG> Vf9W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Read access: No new valid data available Write access: No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Vf9::Value1)
    }
    #[doc = "Read access: Result register x contains valid data and has not yet been read, or bit FCR has been updated Write access: Clear this valid flag and bitfield DRC in register GxRESy (overrides a hardware set action)"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Vf9::Value2)
    }
}
#[doc = "Valid Flag of Result Register x\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Vf10 {
    #[doc = "0: Read access: No new valid data available Write access: No effect"]
    Value1 = 0,
    #[doc = "1: Read access: Result register x contains valid data and has not yet been read, or bit FCR has been updated Write access: Clear this valid flag and bitfield DRC in register GxRESy (overrides a hardware set action)"]
    Value2 = 1,
}
impl From<Vf10> for bool {
    #[inline(always)]
    fn from(variant: Vf10) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VF10` reader - Valid Flag of Result Register x"]
pub type Vf10R = crate::BitReader<Vf10>;
impl Vf10R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Vf10 {
        match self.bits {
            false => Vf10::Value1,
            true => Vf10::Value2,
        }
    }
    #[doc = "Read access: No new valid data available Write access: No effect"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Vf10::Value1
    }
    #[doc = "Read access: Result register x contains valid data and has not yet been read, or bit FCR has been updated Write access: Clear this valid flag and bitfield DRC in register GxRESy (overrides a hardware set action)"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Vf10::Value2
    }
}
#[doc = "Field `VF10` writer - Valid Flag of Result Register x"]
pub type Vf10W<'a, REG> = crate::BitWriter<'a, REG, Vf10>;
impl<'a, REG> Vf10W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Read access: No new valid data available Write access: No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Vf10::Value1)
    }
    #[doc = "Read access: Result register x contains valid data and has not yet been read, or bit FCR has been updated Write access: Clear this valid flag and bitfield DRC in register GxRESy (overrides a hardware set action)"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Vf10::Value2)
    }
}
#[doc = "Valid Flag of Result Register x\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Vf11 {
    #[doc = "0: Read access: No new valid data available Write access: No effect"]
    Value1 = 0,
    #[doc = "1: Read access: Result register x contains valid data and has not yet been read, or bit FCR has been updated Write access: Clear this valid flag and bitfield DRC in register GxRESy (overrides a hardware set action)"]
    Value2 = 1,
}
impl From<Vf11> for bool {
    #[inline(always)]
    fn from(variant: Vf11) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VF11` reader - Valid Flag of Result Register x"]
pub type Vf11R = crate::BitReader<Vf11>;
impl Vf11R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Vf11 {
        match self.bits {
            false => Vf11::Value1,
            true => Vf11::Value2,
        }
    }
    #[doc = "Read access: No new valid data available Write access: No effect"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Vf11::Value1
    }
    #[doc = "Read access: Result register x contains valid data and has not yet been read, or bit FCR has been updated Write access: Clear this valid flag and bitfield DRC in register GxRESy (overrides a hardware set action)"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Vf11::Value2
    }
}
#[doc = "Field `VF11` writer - Valid Flag of Result Register x"]
pub type Vf11W<'a, REG> = crate::BitWriter<'a, REG, Vf11>;
impl<'a, REG> Vf11W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Read access: No new valid data available Write access: No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Vf11::Value1)
    }
    #[doc = "Read access: Result register x contains valid data and has not yet been read, or bit FCR has been updated Write access: Clear this valid flag and bitfield DRC in register GxRESy (overrides a hardware set action)"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Vf11::Value2)
    }
}
#[doc = "Valid Flag of Result Register x\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Vf12 {
    #[doc = "0: Read access: No new valid data available Write access: No effect"]
    Value1 = 0,
    #[doc = "1: Read access: Result register x contains valid data and has not yet been read, or bit FCR has been updated Write access: Clear this valid flag and bitfield DRC in register GxRESy (overrides a hardware set action)"]
    Value2 = 1,
}
impl From<Vf12> for bool {
    #[inline(always)]
    fn from(variant: Vf12) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VF12` reader - Valid Flag of Result Register x"]
pub type Vf12R = crate::BitReader<Vf12>;
impl Vf12R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Vf12 {
        match self.bits {
            false => Vf12::Value1,
            true => Vf12::Value2,
        }
    }
    #[doc = "Read access: No new valid data available Write access: No effect"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Vf12::Value1
    }
    #[doc = "Read access: Result register x contains valid data and has not yet been read, or bit FCR has been updated Write access: Clear this valid flag and bitfield DRC in register GxRESy (overrides a hardware set action)"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Vf12::Value2
    }
}
#[doc = "Field `VF12` writer - Valid Flag of Result Register x"]
pub type Vf12W<'a, REG> = crate::BitWriter<'a, REG, Vf12>;
impl<'a, REG> Vf12W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Read access: No new valid data available Write access: No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Vf12::Value1)
    }
    #[doc = "Read access: Result register x contains valid data and has not yet been read, or bit FCR has been updated Write access: Clear this valid flag and bitfield DRC in register GxRESy (overrides a hardware set action)"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Vf12::Value2)
    }
}
#[doc = "Valid Flag of Result Register x\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Vf13 {
    #[doc = "0: Read access: No new valid data available Write access: No effect"]
    Value1 = 0,
    #[doc = "1: Read access: Result register x contains valid data and has not yet been read, or bit FCR has been updated Write access: Clear this valid flag and bitfield DRC in register GxRESy (overrides a hardware set action)"]
    Value2 = 1,
}
impl From<Vf13> for bool {
    #[inline(always)]
    fn from(variant: Vf13) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VF13` reader - Valid Flag of Result Register x"]
pub type Vf13R = crate::BitReader<Vf13>;
impl Vf13R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Vf13 {
        match self.bits {
            false => Vf13::Value1,
            true => Vf13::Value2,
        }
    }
    #[doc = "Read access: No new valid data available Write access: No effect"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Vf13::Value1
    }
    #[doc = "Read access: Result register x contains valid data and has not yet been read, or bit FCR has been updated Write access: Clear this valid flag and bitfield DRC in register GxRESy (overrides a hardware set action)"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Vf13::Value2
    }
}
#[doc = "Field `VF13` writer - Valid Flag of Result Register x"]
pub type Vf13W<'a, REG> = crate::BitWriter<'a, REG, Vf13>;
impl<'a, REG> Vf13W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Read access: No new valid data available Write access: No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Vf13::Value1)
    }
    #[doc = "Read access: Result register x contains valid data and has not yet been read, or bit FCR has been updated Write access: Clear this valid flag and bitfield DRC in register GxRESy (overrides a hardware set action)"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Vf13::Value2)
    }
}
#[doc = "Valid Flag of Result Register x\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Vf14 {
    #[doc = "0: Read access: No new valid data available Write access: No effect"]
    Value1 = 0,
    #[doc = "1: Read access: Result register x contains valid data and has not yet been read, or bit FCR has been updated Write access: Clear this valid flag and bitfield DRC in register GxRESy (overrides a hardware set action)"]
    Value2 = 1,
}
impl From<Vf14> for bool {
    #[inline(always)]
    fn from(variant: Vf14) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VF14` reader - Valid Flag of Result Register x"]
pub type Vf14R = crate::BitReader<Vf14>;
impl Vf14R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Vf14 {
        match self.bits {
            false => Vf14::Value1,
            true => Vf14::Value2,
        }
    }
    #[doc = "Read access: No new valid data available Write access: No effect"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Vf14::Value1
    }
    #[doc = "Read access: Result register x contains valid data and has not yet been read, or bit FCR has been updated Write access: Clear this valid flag and bitfield DRC in register GxRESy (overrides a hardware set action)"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Vf14::Value2
    }
}
#[doc = "Field `VF14` writer - Valid Flag of Result Register x"]
pub type Vf14W<'a, REG> = crate::BitWriter<'a, REG, Vf14>;
impl<'a, REG> Vf14W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Read access: No new valid data available Write access: No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Vf14::Value1)
    }
    #[doc = "Read access: Result register x contains valid data and has not yet been read, or bit FCR has been updated Write access: Clear this valid flag and bitfield DRC in register GxRESy (overrides a hardware set action)"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Vf14::Value2)
    }
}
#[doc = "Valid Flag of Result Register x\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Vf15 {
    #[doc = "0: Read access: No new valid data available Write access: No effect"]
    Value1 = 0,
    #[doc = "1: Read access: Result register x contains valid data and has not yet been read, or bit FCR has been updated Write access: Clear this valid flag and bitfield DRC in register GxRESy (overrides a hardware set action)"]
    Value2 = 1,
}
impl From<Vf15> for bool {
    #[inline(always)]
    fn from(variant: Vf15) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VF15` reader - Valid Flag of Result Register x"]
pub type Vf15R = crate::BitReader<Vf15>;
impl Vf15R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Vf15 {
        match self.bits {
            false => Vf15::Value1,
            true => Vf15::Value2,
        }
    }
    #[doc = "Read access: No new valid data available Write access: No effect"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Vf15::Value1
    }
    #[doc = "Read access: Result register x contains valid data and has not yet been read, or bit FCR has been updated Write access: Clear this valid flag and bitfield DRC in register GxRESy (overrides a hardware set action)"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Vf15::Value2
    }
}
#[doc = "Field `VF15` writer - Valid Flag of Result Register x"]
pub type Vf15W<'a, REG> = crate::BitWriter<'a, REG, Vf15>;
impl<'a, REG> Vf15W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Read access: No new valid data available Write access: No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Vf15::Value1)
    }
    #[doc = "Read access: Result register x contains valid data and has not yet been read, or bit FCR has been updated Write access: Clear this valid flag and bitfield DRC in register GxRESy (overrides a hardware set action)"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Vf15::Value2)
    }
}
impl R {
    #[doc = "Bit 0 - Valid Flag of Result Register x"]
    #[inline(always)]
    pub fn vf0(&self) -> Vf0R {
        Vf0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Valid Flag of Result Register x"]
    #[inline(always)]
    pub fn vf1(&self) -> Vf1R {
        Vf1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Valid Flag of Result Register x"]
    #[inline(always)]
    pub fn vf2(&self) -> Vf2R {
        Vf2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Valid Flag of Result Register x"]
    #[inline(always)]
    pub fn vf3(&self) -> Vf3R {
        Vf3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Valid Flag of Result Register x"]
    #[inline(always)]
    pub fn vf4(&self) -> Vf4R {
        Vf4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Valid Flag of Result Register x"]
    #[inline(always)]
    pub fn vf5(&self) -> Vf5R {
        Vf5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Valid Flag of Result Register x"]
    #[inline(always)]
    pub fn vf6(&self) -> Vf6R {
        Vf6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Valid Flag of Result Register x"]
    #[inline(always)]
    pub fn vf7(&self) -> Vf7R {
        Vf7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Valid Flag of Result Register x"]
    #[inline(always)]
    pub fn vf8(&self) -> Vf8R {
        Vf8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Valid Flag of Result Register x"]
    #[inline(always)]
    pub fn vf9(&self) -> Vf9R {
        Vf9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Valid Flag of Result Register x"]
    #[inline(always)]
    pub fn vf10(&self) -> Vf10R {
        Vf10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Valid Flag of Result Register x"]
    #[inline(always)]
    pub fn vf11(&self) -> Vf11R {
        Vf11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Valid Flag of Result Register x"]
    #[inline(always)]
    pub fn vf12(&self) -> Vf12R {
        Vf12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Valid Flag of Result Register x"]
    #[inline(always)]
    pub fn vf13(&self) -> Vf13R {
        Vf13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Valid Flag of Result Register x"]
    #[inline(always)]
    pub fn vf14(&self) -> Vf14R {
        Vf14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Valid Flag of Result Register x"]
    #[inline(always)]
    pub fn vf15(&self) -> Vf15R {
        Vf15R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Valid Flag of Result Register x"]
    #[inline(always)]
    #[must_use]
    pub fn vf0(&mut self) -> Vf0W<VfrSpec> {
        Vf0W::new(self, 0)
    }
    #[doc = "Bit 1 - Valid Flag of Result Register x"]
    #[inline(always)]
    #[must_use]
    pub fn vf1(&mut self) -> Vf1W<VfrSpec> {
        Vf1W::new(self, 1)
    }
    #[doc = "Bit 2 - Valid Flag of Result Register x"]
    #[inline(always)]
    #[must_use]
    pub fn vf2(&mut self) -> Vf2W<VfrSpec> {
        Vf2W::new(self, 2)
    }
    #[doc = "Bit 3 - Valid Flag of Result Register x"]
    #[inline(always)]
    #[must_use]
    pub fn vf3(&mut self) -> Vf3W<VfrSpec> {
        Vf3W::new(self, 3)
    }
    #[doc = "Bit 4 - Valid Flag of Result Register x"]
    #[inline(always)]
    #[must_use]
    pub fn vf4(&mut self) -> Vf4W<VfrSpec> {
        Vf4W::new(self, 4)
    }
    #[doc = "Bit 5 - Valid Flag of Result Register x"]
    #[inline(always)]
    #[must_use]
    pub fn vf5(&mut self) -> Vf5W<VfrSpec> {
        Vf5W::new(self, 5)
    }
    #[doc = "Bit 6 - Valid Flag of Result Register x"]
    #[inline(always)]
    #[must_use]
    pub fn vf6(&mut self) -> Vf6W<VfrSpec> {
        Vf6W::new(self, 6)
    }
    #[doc = "Bit 7 - Valid Flag of Result Register x"]
    #[inline(always)]
    #[must_use]
    pub fn vf7(&mut self) -> Vf7W<VfrSpec> {
        Vf7W::new(self, 7)
    }
    #[doc = "Bit 8 - Valid Flag of Result Register x"]
    #[inline(always)]
    #[must_use]
    pub fn vf8(&mut self) -> Vf8W<VfrSpec> {
        Vf8W::new(self, 8)
    }
    #[doc = "Bit 9 - Valid Flag of Result Register x"]
    #[inline(always)]
    #[must_use]
    pub fn vf9(&mut self) -> Vf9W<VfrSpec> {
        Vf9W::new(self, 9)
    }
    #[doc = "Bit 10 - Valid Flag of Result Register x"]
    #[inline(always)]
    #[must_use]
    pub fn vf10(&mut self) -> Vf10W<VfrSpec> {
        Vf10W::new(self, 10)
    }
    #[doc = "Bit 11 - Valid Flag of Result Register x"]
    #[inline(always)]
    #[must_use]
    pub fn vf11(&mut self) -> Vf11W<VfrSpec> {
        Vf11W::new(self, 11)
    }
    #[doc = "Bit 12 - Valid Flag of Result Register x"]
    #[inline(always)]
    #[must_use]
    pub fn vf12(&mut self) -> Vf12W<VfrSpec> {
        Vf12W::new(self, 12)
    }
    #[doc = "Bit 13 - Valid Flag of Result Register x"]
    #[inline(always)]
    #[must_use]
    pub fn vf13(&mut self) -> Vf13W<VfrSpec> {
        Vf13W::new(self, 13)
    }
    #[doc = "Bit 14 - Valid Flag of Result Register x"]
    #[inline(always)]
    #[must_use]
    pub fn vf14(&mut self) -> Vf14W<VfrSpec> {
        Vf14W::new(self, 14)
    }
    #[doc = "Bit 15 - Valid Flag of Result Register x"]
    #[inline(always)]
    #[must_use]
    pub fn vf15(&mut self) -> Vf15W<VfrSpec> {
        Vf15W::new(self, 15)
    }
}
#[doc = "Valid Flag Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vfr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vfr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VfrSpec;
impl crate::RegisterSpec for VfrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vfr::R`](R) reader structure"]
impl crate::Readable for VfrSpec {}
#[doc = "`write(|w| ..)` method takes [`vfr::W`](W) writer structure"]
impl crate::Writable for VfrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets VFR to value 0"]
impl crate::Resettable for VfrSpec {
    const RESET_VALUE: u32 = 0;
}
