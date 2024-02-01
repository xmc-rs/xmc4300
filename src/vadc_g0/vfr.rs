#[doc = "Register `VFR` reader"]
pub type R = crate::R<VFR_SPEC>;
#[doc = "Register `VFR` writer"]
pub type W = crate::W<VFR_SPEC>;
#[doc = "Field `VF0` reader - Valid Flag of Result Register x"]
pub type VF0_R = crate::BitReader<VF0_A>;
#[doc = "Valid Flag of Result Register x\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VF0_A {
    #[doc = "0: Read access: No new valid data available Write access: No effect"]
    VALUE1 = 0,
    #[doc = "1: Read access: Result register x contains valid data and has not yet been read, or bit FCR has been updated Write access: Clear this valid flag and bitfield DRC in register GxRESy (overrides a hardware set action)"]
    VALUE2 = 1,
}
impl From<VF0_A> for bool {
    #[inline(always)]
    fn from(variant: VF0_A) -> Self {
        variant as u8 != 0
    }
}
impl VF0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> VF0_A {
        match self.bits {
            false => VF0_A::VALUE1,
            true => VF0_A::VALUE2,
        }
    }
    #[doc = "Read access: No new valid data available Write access: No effect"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == VF0_A::VALUE1
    }
    #[doc = "Read access: Result register x contains valid data and has not yet been read, or bit FCR has been updated Write access: Clear this valid flag and bitfield DRC in register GxRESy (overrides a hardware set action)"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == VF0_A::VALUE2
    }
}
#[doc = "Field `VF0` writer - Valid Flag of Result Register x"]
pub type VF0_W<'a, REG> = crate::BitWriter<'a, REG, VF0_A>;
impl<'a, REG> VF0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Read access: No new valid data available Write access: No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(VF0_A::VALUE1)
    }
    #[doc = "Read access: Result register x contains valid data and has not yet been read, or bit FCR has been updated Write access: Clear this valid flag and bitfield DRC in register GxRESy (overrides a hardware set action)"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(VF0_A::VALUE2)
    }
}
#[doc = "Field `VF1` reader - Valid Flag of Result Register x"]
pub type VF1_R = crate::BitReader<VF1_A>;
#[doc = "Valid Flag of Result Register x\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VF1_A {
    #[doc = "0: Read access: No new valid data available Write access: No effect"]
    VALUE1 = 0,
    #[doc = "1: Read access: Result register x contains valid data and has not yet been read, or bit FCR has been updated Write access: Clear this valid flag and bitfield DRC in register GxRESy (overrides a hardware set action)"]
    VALUE2 = 1,
}
impl From<VF1_A> for bool {
    #[inline(always)]
    fn from(variant: VF1_A) -> Self {
        variant as u8 != 0
    }
}
impl VF1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> VF1_A {
        match self.bits {
            false => VF1_A::VALUE1,
            true => VF1_A::VALUE2,
        }
    }
    #[doc = "Read access: No new valid data available Write access: No effect"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == VF1_A::VALUE1
    }
    #[doc = "Read access: Result register x contains valid data and has not yet been read, or bit FCR has been updated Write access: Clear this valid flag and bitfield DRC in register GxRESy (overrides a hardware set action)"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == VF1_A::VALUE2
    }
}
#[doc = "Field `VF1` writer - Valid Flag of Result Register x"]
pub type VF1_W<'a, REG> = crate::BitWriter<'a, REG, VF1_A>;
impl<'a, REG> VF1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Read access: No new valid data available Write access: No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(VF1_A::VALUE1)
    }
    #[doc = "Read access: Result register x contains valid data and has not yet been read, or bit FCR has been updated Write access: Clear this valid flag and bitfield DRC in register GxRESy (overrides a hardware set action)"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(VF1_A::VALUE2)
    }
}
#[doc = "Field `VF2` reader - Valid Flag of Result Register x"]
pub type VF2_R = crate::BitReader<VF2_A>;
#[doc = "Valid Flag of Result Register x\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VF2_A {
    #[doc = "0: Read access: No new valid data available Write access: No effect"]
    VALUE1 = 0,
    #[doc = "1: Read access: Result register x contains valid data and has not yet been read, or bit FCR has been updated Write access: Clear this valid flag and bitfield DRC in register GxRESy (overrides a hardware set action)"]
    VALUE2 = 1,
}
impl From<VF2_A> for bool {
    #[inline(always)]
    fn from(variant: VF2_A) -> Self {
        variant as u8 != 0
    }
}
impl VF2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> VF2_A {
        match self.bits {
            false => VF2_A::VALUE1,
            true => VF2_A::VALUE2,
        }
    }
    #[doc = "Read access: No new valid data available Write access: No effect"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == VF2_A::VALUE1
    }
    #[doc = "Read access: Result register x contains valid data and has not yet been read, or bit FCR has been updated Write access: Clear this valid flag and bitfield DRC in register GxRESy (overrides a hardware set action)"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == VF2_A::VALUE2
    }
}
#[doc = "Field `VF2` writer - Valid Flag of Result Register x"]
pub type VF2_W<'a, REG> = crate::BitWriter<'a, REG, VF2_A>;
impl<'a, REG> VF2_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Read access: No new valid data available Write access: No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(VF2_A::VALUE1)
    }
    #[doc = "Read access: Result register x contains valid data and has not yet been read, or bit FCR has been updated Write access: Clear this valid flag and bitfield DRC in register GxRESy (overrides a hardware set action)"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(VF2_A::VALUE2)
    }
}
#[doc = "Field `VF3` reader - Valid Flag of Result Register x"]
pub type VF3_R = crate::BitReader<VF3_A>;
#[doc = "Valid Flag of Result Register x\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VF3_A {
    #[doc = "0: Read access: No new valid data available Write access: No effect"]
    VALUE1 = 0,
    #[doc = "1: Read access: Result register x contains valid data and has not yet been read, or bit FCR has been updated Write access: Clear this valid flag and bitfield DRC in register GxRESy (overrides a hardware set action)"]
    VALUE2 = 1,
}
impl From<VF3_A> for bool {
    #[inline(always)]
    fn from(variant: VF3_A) -> Self {
        variant as u8 != 0
    }
}
impl VF3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> VF3_A {
        match self.bits {
            false => VF3_A::VALUE1,
            true => VF3_A::VALUE2,
        }
    }
    #[doc = "Read access: No new valid data available Write access: No effect"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == VF3_A::VALUE1
    }
    #[doc = "Read access: Result register x contains valid data and has not yet been read, or bit FCR has been updated Write access: Clear this valid flag and bitfield DRC in register GxRESy (overrides a hardware set action)"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == VF3_A::VALUE2
    }
}
#[doc = "Field `VF3` writer - Valid Flag of Result Register x"]
pub type VF3_W<'a, REG> = crate::BitWriter<'a, REG, VF3_A>;
impl<'a, REG> VF3_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Read access: No new valid data available Write access: No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(VF3_A::VALUE1)
    }
    #[doc = "Read access: Result register x contains valid data and has not yet been read, or bit FCR has been updated Write access: Clear this valid flag and bitfield DRC in register GxRESy (overrides a hardware set action)"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(VF3_A::VALUE2)
    }
}
#[doc = "Field `VF4` reader - Valid Flag of Result Register x"]
pub type VF4_R = crate::BitReader<VF4_A>;
#[doc = "Valid Flag of Result Register x\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VF4_A {
    #[doc = "0: Read access: No new valid data available Write access: No effect"]
    VALUE1 = 0,
    #[doc = "1: Read access: Result register x contains valid data and has not yet been read, or bit FCR has been updated Write access: Clear this valid flag and bitfield DRC in register GxRESy (overrides a hardware set action)"]
    VALUE2 = 1,
}
impl From<VF4_A> for bool {
    #[inline(always)]
    fn from(variant: VF4_A) -> Self {
        variant as u8 != 0
    }
}
impl VF4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> VF4_A {
        match self.bits {
            false => VF4_A::VALUE1,
            true => VF4_A::VALUE2,
        }
    }
    #[doc = "Read access: No new valid data available Write access: No effect"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == VF4_A::VALUE1
    }
    #[doc = "Read access: Result register x contains valid data and has not yet been read, or bit FCR has been updated Write access: Clear this valid flag and bitfield DRC in register GxRESy (overrides a hardware set action)"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == VF4_A::VALUE2
    }
}
#[doc = "Field `VF4` writer - Valid Flag of Result Register x"]
pub type VF4_W<'a, REG> = crate::BitWriter<'a, REG, VF4_A>;
impl<'a, REG> VF4_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Read access: No new valid data available Write access: No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(VF4_A::VALUE1)
    }
    #[doc = "Read access: Result register x contains valid data and has not yet been read, or bit FCR has been updated Write access: Clear this valid flag and bitfield DRC in register GxRESy (overrides a hardware set action)"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(VF4_A::VALUE2)
    }
}
#[doc = "Field `VF5` reader - Valid Flag of Result Register x"]
pub type VF5_R = crate::BitReader<VF5_A>;
#[doc = "Valid Flag of Result Register x\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VF5_A {
    #[doc = "0: Read access: No new valid data available Write access: No effect"]
    VALUE1 = 0,
    #[doc = "1: Read access: Result register x contains valid data and has not yet been read, or bit FCR has been updated Write access: Clear this valid flag and bitfield DRC in register GxRESy (overrides a hardware set action)"]
    VALUE2 = 1,
}
impl From<VF5_A> for bool {
    #[inline(always)]
    fn from(variant: VF5_A) -> Self {
        variant as u8 != 0
    }
}
impl VF5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> VF5_A {
        match self.bits {
            false => VF5_A::VALUE1,
            true => VF5_A::VALUE2,
        }
    }
    #[doc = "Read access: No new valid data available Write access: No effect"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == VF5_A::VALUE1
    }
    #[doc = "Read access: Result register x contains valid data and has not yet been read, or bit FCR has been updated Write access: Clear this valid flag and bitfield DRC in register GxRESy (overrides a hardware set action)"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == VF5_A::VALUE2
    }
}
#[doc = "Field `VF5` writer - Valid Flag of Result Register x"]
pub type VF5_W<'a, REG> = crate::BitWriter<'a, REG, VF5_A>;
impl<'a, REG> VF5_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Read access: No new valid data available Write access: No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(VF5_A::VALUE1)
    }
    #[doc = "Read access: Result register x contains valid data and has not yet been read, or bit FCR has been updated Write access: Clear this valid flag and bitfield DRC in register GxRESy (overrides a hardware set action)"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(VF5_A::VALUE2)
    }
}
#[doc = "Field `VF6` reader - Valid Flag of Result Register x"]
pub type VF6_R = crate::BitReader<VF6_A>;
#[doc = "Valid Flag of Result Register x\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VF6_A {
    #[doc = "0: Read access: No new valid data available Write access: No effect"]
    VALUE1 = 0,
    #[doc = "1: Read access: Result register x contains valid data and has not yet been read, or bit FCR has been updated Write access: Clear this valid flag and bitfield DRC in register GxRESy (overrides a hardware set action)"]
    VALUE2 = 1,
}
impl From<VF6_A> for bool {
    #[inline(always)]
    fn from(variant: VF6_A) -> Self {
        variant as u8 != 0
    }
}
impl VF6_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> VF6_A {
        match self.bits {
            false => VF6_A::VALUE1,
            true => VF6_A::VALUE2,
        }
    }
    #[doc = "Read access: No new valid data available Write access: No effect"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == VF6_A::VALUE1
    }
    #[doc = "Read access: Result register x contains valid data and has not yet been read, or bit FCR has been updated Write access: Clear this valid flag and bitfield DRC in register GxRESy (overrides a hardware set action)"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == VF6_A::VALUE2
    }
}
#[doc = "Field `VF6` writer - Valid Flag of Result Register x"]
pub type VF6_W<'a, REG> = crate::BitWriter<'a, REG, VF6_A>;
impl<'a, REG> VF6_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Read access: No new valid data available Write access: No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(VF6_A::VALUE1)
    }
    #[doc = "Read access: Result register x contains valid data and has not yet been read, or bit FCR has been updated Write access: Clear this valid flag and bitfield DRC in register GxRESy (overrides a hardware set action)"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(VF6_A::VALUE2)
    }
}
#[doc = "Field `VF7` reader - Valid Flag of Result Register x"]
pub type VF7_R = crate::BitReader<VF7_A>;
#[doc = "Valid Flag of Result Register x\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VF7_A {
    #[doc = "0: Read access: No new valid data available Write access: No effect"]
    VALUE1 = 0,
    #[doc = "1: Read access: Result register x contains valid data and has not yet been read, or bit FCR has been updated Write access: Clear this valid flag and bitfield DRC in register GxRESy (overrides a hardware set action)"]
    VALUE2 = 1,
}
impl From<VF7_A> for bool {
    #[inline(always)]
    fn from(variant: VF7_A) -> Self {
        variant as u8 != 0
    }
}
impl VF7_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> VF7_A {
        match self.bits {
            false => VF7_A::VALUE1,
            true => VF7_A::VALUE2,
        }
    }
    #[doc = "Read access: No new valid data available Write access: No effect"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == VF7_A::VALUE1
    }
    #[doc = "Read access: Result register x contains valid data and has not yet been read, or bit FCR has been updated Write access: Clear this valid flag and bitfield DRC in register GxRESy (overrides a hardware set action)"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == VF7_A::VALUE2
    }
}
#[doc = "Field `VF7` writer - Valid Flag of Result Register x"]
pub type VF7_W<'a, REG> = crate::BitWriter<'a, REG, VF7_A>;
impl<'a, REG> VF7_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Read access: No new valid data available Write access: No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(VF7_A::VALUE1)
    }
    #[doc = "Read access: Result register x contains valid data and has not yet been read, or bit FCR has been updated Write access: Clear this valid flag and bitfield DRC in register GxRESy (overrides a hardware set action)"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(VF7_A::VALUE2)
    }
}
#[doc = "Field `VF8` reader - Valid Flag of Result Register x"]
pub type VF8_R = crate::BitReader<VF8_A>;
#[doc = "Valid Flag of Result Register x\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VF8_A {
    #[doc = "0: Read access: No new valid data available Write access: No effect"]
    VALUE1 = 0,
    #[doc = "1: Read access: Result register x contains valid data and has not yet been read, or bit FCR has been updated Write access: Clear this valid flag and bitfield DRC in register GxRESy (overrides a hardware set action)"]
    VALUE2 = 1,
}
impl From<VF8_A> for bool {
    #[inline(always)]
    fn from(variant: VF8_A) -> Self {
        variant as u8 != 0
    }
}
impl VF8_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> VF8_A {
        match self.bits {
            false => VF8_A::VALUE1,
            true => VF8_A::VALUE2,
        }
    }
    #[doc = "Read access: No new valid data available Write access: No effect"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == VF8_A::VALUE1
    }
    #[doc = "Read access: Result register x contains valid data and has not yet been read, or bit FCR has been updated Write access: Clear this valid flag and bitfield DRC in register GxRESy (overrides a hardware set action)"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == VF8_A::VALUE2
    }
}
#[doc = "Field `VF8` writer - Valid Flag of Result Register x"]
pub type VF8_W<'a, REG> = crate::BitWriter<'a, REG, VF8_A>;
impl<'a, REG> VF8_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Read access: No new valid data available Write access: No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(VF8_A::VALUE1)
    }
    #[doc = "Read access: Result register x contains valid data and has not yet been read, or bit FCR has been updated Write access: Clear this valid flag and bitfield DRC in register GxRESy (overrides a hardware set action)"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(VF8_A::VALUE2)
    }
}
#[doc = "Field `VF9` reader - Valid Flag of Result Register x"]
pub type VF9_R = crate::BitReader<VF9_A>;
#[doc = "Valid Flag of Result Register x\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VF9_A {
    #[doc = "0: Read access: No new valid data available Write access: No effect"]
    VALUE1 = 0,
    #[doc = "1: Read access: Result register x contains valid data and has not yet been read, or bit FCR has been updated Write access: Clear this valid flag and bitfield DRC in register GxRESy (overrides a hardware set action)"]
    VALUE2 = 1,
}
impl From<VF9_A> for bool {
    #[inline(always)]
    fn from(variant: VF9_A) -> Self {
        variant as u8 != 0
    }
}
impl VF9_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> VF9_A {
        match self.bits {
            false => VF9_A::VALUE1,
            true => VF9_A::VALUE2,
        }
    }
    #[doc = "Read access: No new valid data available Write access: No effect"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == VF9_A::VALUE1
    }
    #[doc = "Read access: Result register x contains valid data and has not yet been read, or bit FCR has been updated Write access: Clear this valid flag and bitfield DRC in register GxRESy (overrides a hardware set action)"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == VF9_A::VALUE2
    }
}
#[doc = "Field `VF9` writer - Valid Flag of Result Register x"]
pub type VF9_W<'a, REG> = crate::BitWriter<'a, REG, VF9_A>;
impl<'a, REG> VF9_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Read access: No new valid data available Write access: No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(VF9_A::VALUE1)
    }
    #[doc = "Read access: Result register x contains valid data and has not yet been read, or bit FCR has been updated Write access: Clear this valid flag and bitfield DRC in register GxRESy (overrides a hardware set action)"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(VF9_A::VALUE2)
    }
}
#[doc = "Field `VF10` reader - Valid Flag of Result Register x"]
pub type VF10_R = crate::BitReader<VF10_A>;
#[doc = "Valid Flag of Result Register x\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VF10_A {
    #[doc = "0: Read access: No new valid data available Write access: No effect"]
    VALUE1 = 0,
    #[doc = "1: Read access: Result register x contains valid data and has not yet been read, or bit FCR has been updated Write access: Clear this valid flag and bitfield DRC in register GxRESy (overrides a hardware set action)"]
    VALUE2 = 1,
}
impl From<VF10_A> for bool {
    #[inline(always)]
    fn from(variant: VF10_A) -> Self {
        variant as u8 != 0
    }
}
impl VF10_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> VF10_A {
        match self.bits {
            false => VF10_A::VALUE1,
            true => VF10_A::VALUE2,
        }
    }
    #[doc = "Read access: No new valid data available Write access: No effect"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == VF10_A::VALUE1
    }
    #[doc = "Read access: Result register x contains valid data and has not yet been read, or bit FCR has been updated Write access: Clear this valid flag and bitfield DRC in register GxRESy (overrides a hardware set action)"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == VF10_A::VALUE2
    }
}
#[doc = "Field `VF10` writer - Valid Flag of Result Register x"]
pub type VF10_W<'a, REG> = crate::BitWriter<'a, REG, VF10_A>;
impl<'a, REG> VF10_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Read access: No new valid data available Write access: No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(VF10_A::VALUE1)
    }
    #[doc = "Read access: Result register x contains valid data and has not yet been read, or bit FCR has been updated Write access: Clear this valid flag and bitfield DRC in register GxRESy (overrides a hardware set action)"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(VF10_A::VALUE2)
    }
}
#[doc = "Field `VF11` reader - Valid Flag of Result Register x"]
pub type VF11_R = crate::BitReader<VF11_A>;
#[doc = "Valid Flag of Result Register x\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VF11_A {
    #[doc = "0: Read access: No new valid data available Write access: No effect"]
    VALUE1 = 0,
    #[doc = "1: Read access: Result register x contains valid data and has not yet been read, or bit FCR has been updated Write access: Clear this valid flag and bitfield DRC in register GxRESy (overrides a hardware set action)"]
    VALUE2 = 1,
}
impl From<VF11_A> for bool {
    #[inline(always)]
    fn from(variant: VF11_A) -> Self {
        variant as u8 != 0
    }
}
impl VF11_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> VF11_A {
        match self.bits {
            false => VF11_A::VALUE1,
            true => VF11_A::VALUE2,
        }
    }
    #[doc = "Read access: No new valid data available Write access: No effect"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == VF11_A::VALUE1
    }
    #[doc = "Read access: Result register x contains valid data and has not yet been read, or bit FCR has been updated Write access: Clear this valid flag and bitfield DRC in register GxRESy (overrides a hardware set action)"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == VF11_A::VALUE2
    }
}
#[doc = "Field `VF11` writer - Valid Flag of Result Register x"]
pub type VF11_W<'a, REG> = crate::BitWriter<'a, REG, VF11_A>;
impl<'a, REG> VF11_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Read access: No new valid data available Write access: No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(VF11_A::VALUE1)
    }
    #[doc = "Read access: Result register x contains valid data and has not yet been read, or bit FCR has been updated Write access: Clear this valid flag and bitfield DRC in register GxRESy (overrides a hardware set action)"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(VF11_A::VALUE2)
    }
}
#[doc = "Field `VF12` reader - Valid Flag of Result Register x"]
pub type VF12_R = crate::BitReader<VF12_A>;
#[doc = "Valid Flag of Result Register x\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VF12_A {
    #[doc = "0: Read access: No new valid data available Write access: No effect"]
    VALUE1 = 0,
    #[doc = "1: Read access: Result register x contains valid data and has not yet been read, or bit FCR has been updated Write access: Clear this valid flag and bitfield DRC in register GxRESy (overrides a hardware set action)"]
    VALUE2 = 1,
}
impl From<VF12_A> for bool {
    #[inline(always)]
    fn from(variant: VF12_A) -> Self {
        variant as u8 != 0
    }
}
impl VF12_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> VF12_A {
        match self.bits {
            false => VF12_A::VALUE1,
            true => VF12_A::VALUE2,
        }
    }
    #[doc = "Read access: No new valid data available Write access: No effect"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == VF12_A::VALUE1
    }
    #[doc = "Read access: Result register x contains valid data and has not yet been read, or bit FCR has been updated Write access: Clear this valid flag and bitfield DRC in register GxRESy (overrides a hardware set action)"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == VF12_A::VALUE2
    }
}
#[doc = "Field `VF12` writer - Valid Flag of Result Register x"]
pub type VF12_W<'a, REG> = crate::BitWriter<'a, REG, VF12_A>;
impl<'a, REG> VF12_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Read access: No new valid data available Write access: No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(VF12_A::VALUE1)
    }
    #[doc = "Read access: Result register x contains valid data and has not yet been read, or bit FCR has been updated Write access: Clear this valid flag and bitfield DRC in register GxRESy (overrides a hardware set action)"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(VF12_A::VALUE2)
    }
}
#[doc = "Field `VF13` reader - Valid Flag of Result Register x"]
pub type VF13_R = crate::BitReader<VF13_A>;
#[doc = "Valid Flag of Result Register x\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VF13_A {
    #[doc = "0: Read access: No new valid data available Write access: No effect"]
    VALUE1 = 0,
    #[doc = "1: Read access: Result register x contains valid data and has not yet been read, or bit FCR has been updated Write access: Clear this valid flag and bitfield DRC in register GxRESy (overrides a hardware set action)"]
    VALUE2 = 1,
}
impl From<VF13_A> for bool {
    #[inline(always)]
    fn from(variant: VF13_A) -> Self {
        variant as u8 != 0
    }
}
impl VF13_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> VF13_A {
        match self.bits {
            false => VF13_A::VALUE1,
            true => VF13_A::VALUE2,
        }
    }
    #[doc = "Read access: No new valid data available Write access: No effect"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == VF13_A::VALUE1
    }
    #[doc = "Read access: Result register x contains valid data and has not yet been read, or bit FCR has been updated Write access: Clear this valid flag and bitfield DRC in register GxRESy (overrides a hardware set action)"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == VF13_A::VALUE2
    }
}
#[doc = "Field `VF13` writer - Valid Flag of Result Register x"]
pub type VF13_W<'a, REG> = crate::BitWriter<'a, REG, VF13_A>;
impl<'a, REG> VF13_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Read access: No new valid data available Write access: No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(VF13_A::VALUE1)
    }
    #[doc = "Read access: Result register x contains valid data and has not yet been read, or bit FCR has been updated Write access: Clear this valid flag and bitfield DRC in register GxRESy (overrides a hardware set action)"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(VF13_A::VALUE2)
    }
}
#[doc = "Field `VF14` reader - Valid Flag of Result Register x"]
pub type VF14_R = crate::BitReader<VF14_A>;
#[doc = "Valid Flag of Result Register x\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VF14_A {
    #[doc = "0: Read access: No new valid data available Write access: No effect"]
    VALUE1 = 0,
    #[doc = "1: Read access: Result register x contains valid data and has not yet been read, or bit FCR has been updated Write access: Clear this valid flag and bitfield DRC in register GxRESy (overrides a hardware set action)"]
    VALUE2 = 1,
}
impl From<VF14_A> for bool {
    #[inline(always)]
    fn from(variant: VF14_A) -> Self {
        variant as u8 != 0
    }
}
impl VF14_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> VF14_A {
        match self.bits {
            false => VF14_A::VALUE1,
            true => VF14_A::VALUE2,
        }
    }
    #[doc = "Read access: No new valid data available Write access: No effect"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == VF14_A::VALUE1
    }
    #[doc = "Read access: Result register x contains valid data and has not yet been read, or bit FCR has been updated Write access: Clear this valid flag and bitfield DRC in register GxRESy (overrides a hardware set action)"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == VF14_A::VALUE2
    }
}
#[doc = "Field `VF14` writer - Valid Flag of Result Register x"]
pub type VF14_W<'a, REG> = crate::BitWriter<'a, REG, VF14_A>;
impl<'a, REG> VF14_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Read access: No new valid data available Write access: No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(VF14_A::VALUE1)
    }
    #[doc = "Read access: Result register x contains valid data and has not yet been read, or bit FCR has been updated Write access: Clear this valid flag and bitfield DRC in register GxRESy (overrides a hardware set action)"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(VF14_A::VALUE2)
    }
}
#[doc = "Field `VF15` reader - Valid Flag of Result Register x"]
pub type VF15_R = crate::BitReader<VF15_A>;
#[doc = "Valid Flag of Result Register x\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VF15_A {
    #[doc = "0: Read access: No new valid data available Write access: No effect"]
    VALUE1 = 0,
    #[doc = "1: Read access: Result register x contains valid data and has not yet been read, or bit FCR has been updated Write access: Clear this valid flag and bitfield DRC in register GxRESy (overrides a hardware set action)"]
    VALUE2 = 1,
}
impl From<VF15_A> for bool {
    #[inline(always)]
    fn from(variant: VF15_A) -> Self {
        variant as u8 != 0
    }
}
impl VF15_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> VF15_A {
        match self.bits {
            false => VF15_A::VALUE1,
            true => VF15_A::VALUE2,
        }
    }
    #[doc = "Read access: No new valid data available Write access: No effect"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == VF15_A::VALUE1
    }
    #[doc = "Read access: Result register x contains valid data and has not yet been read, or bit FCR has been updated Write access: Clear this valid flag and bitfield DRC in register GxRESy (overrides a hardware set action)"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == VF15_A::VALUE2
    }
}
#[doc = "Field `VF15` writer - Valid Flag of Result Register x"]
pub type VF15_W<'a, REG> = crate::BitWriter<'a, REG, VF15_A>;
impl<'a, REG> VF15_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Read access: No new valid data available Write access: No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(VF15_A::VALUE1)
    }
    #[doc = "Read access: Result register x contains valid data and has not yet been read, or bit FCR has been updated Write access: Clear this valid flag and bitfield DRC in register GxRESy (overrides a hardware set action)"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(VF15_A::VALUE2)
    }
}
impl R {
    #[doc = "Bit 0 - Valid Flag of Result Register x"]
    #[inline(always)]
    pub fn vf0(&self) -> VF0_R {
        VF0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Valid Flag of Result Register x"]
    #[inline(always)]
    pub fn vf1(&self) -> VF1_R {
        VF1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Valid Flag of Result Register x"]
    #[inline(always)]
    pub fn vf2(&self) -> VF2_R {
        VF2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Valid Flag of Result Register x"]
    #[inline(always)]
    pub fn vf3(&self) -> VF3_R {
        VF3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Valid Flag of Result Register x"]
    #[inline(always)]
    pub fn vf4(&self) -> VF4_R {
        VF4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Valid Flag of Result Register x"]
    #[inline(always)]
    pub fn vf5(&self) -> VF5_R {
        VF5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Valid Flag of Result Register x"]
    #[inline(always)]
    pub fn vf6(&self) -> VF6_R {
        VF6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Valid Flag of Result Register x"]
    #[inline(always)]
    pub fn vf7(&self) -> VF7_R {
        VF7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Valid Flag of Result Register x"]
    #[inline(always)]
    pub fn vf8(&self) -> VF8_R {
        VF8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Valid Flag of Result Register x"]
    #[inline(always)]
    pub fn vf9(&self) -> VF9_R {
        VF9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Valid Flag of Result Register x"]
    #[inline(always)]
    pub fn vf10(&self) -> VF10_R {
        VF10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Valid Flag of Result Register x"]
    #[inline(always)]
    pub fn vf11(&self) -> VF11_R {
        VF11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Valid Flag of Result Register x"]
    #[inline(always)]
    pub fn vf12(&self) -> VF12_R {
        VF12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Valid Flag of Result Register x"]
    #[inline(always)]
    pub fn vf13(&self) -> VF13_R {
        VF13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Valid Flag of Result Register x"]
    #[inline(always)]
    pub fn vf14(&self) -> VF14_R {
        VF14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Valid Flag of Result Register x"]
    #[inline(always)]
    pub fn vf15(&self) -> VF15_R {
        VF15_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Valid Flag of Result Register x"]
    #[inline(always)]
    #[must_use]
    pub fn vf0(&mut self) -> VF0_W<VFR_SPEC> {
        VF0_W::new(self, 0)
    }
    #[doc = "Bit 1 - Valid Flag of Result Register x"]
    #[inline(always)]
    #[must_use]
    pub fn vf1(&mut self) -> VF1_W<VFR_SPEC> {
        VF1_W::new(self, 1)
    }
    #[doc = "Bit 2 - Valid Flag of Result Register x"]
    #[inline(always)]
    #[must_use]
    pub fn vf2(&mut self) -> VF2_W<VFR_SPEC> {
        VF2_W::new(self, 2)
    }
    #[doc = "Bit 3 - Valid Flag of Result Register x"]
    #[inline(always)]
    #[must_use]
    pub fn vf3(&mut self) -> VF3_W<VFR_SPEC> {
        VF3_W::new(self, 3)
    }
    #[doc = "Bit 4 - Valid Flag of Result Register x"]
    #[inline(always)]
    #[must_use]
    pub fn vf4(&mut self) -> VF4_W<VFR_SPEC> {
        VF4_W::new(self, 4)
    }
    #[doc = "Bit 5 - Valid Flag of Result Register x"]
    #[inline(always)]
    #[must_use]
    pub fn vf5(&mut self) -> VF5_W<VFR_SPEC> {
        VF5_W::new(self, 5)
    }
    #[doc = "Bit 6 - Valid Flag of Result Register x"]
    #[inline(always)]
    #[must_use]
    pub fn vf6(&mut self) -> VF6_W<VFR_SPEC> {
        VF6_W::new(self, 6)
    }
    #[doc = "Bit 7 - Valid Flag of Result Register x"]
    #[inline(always)]
    #[must_use]
    pub fn vf7(&mut self) -> VF7_W<VFR_SPEC> {
        VF7_W::new(self, 7)
    }
    #[doc = "Bit 8 - Valid Flag of Result Register x"]
    #[inline(always)]
    #[must_use]
    pub fn vf8(&mut self) -> VF8_W<VFR_SPEC> {
        VF8_W::new(self, 8)
    }
    #[doc = "Bit 9 - Valid Flag of Result Register x"]
    #[inline(always)]
    #[must_use]
    pub fn vf9(&mut self) -> VF9_W<VFR_SPEC> {
        VF9_W::new(self, 9)
    }
    #[doc = "Bit 10 - Valid Flag of Result Register x"]
    #[inline(always)]
    #[must_use]
    pub fn vf10(&mut self) -> VF10_W<VFR_SPEC> {
        VF10_W::new(self, 10)
    }
    #[doc = "Bit 11 - Valid Flag of Result Register x"]
    #[inline(always)]
    #[must_use]
    pub fn vf11(&mut self) -> VF11_W<VFR_SPEC> {
        VF11_W::new(self, 11)
    }
    #[doc = "Bit 12 - Valid Flag of Result Register x"]
    #[inline(always)]
    #[must_use]
    pub fn vf12(&mut self) -> VF12_W<VFR_SPEC> {
        VF12_W::new(self, 12)
    }
    #[doc = "Bit 13 - Valid Flag of Result Register x"]
    #[inline(always)]
    #[must_use]
    pub fn vf13(&mut self) -> VF13_W<VFR_SPEC> {
        VF13_W::new(self, 13)
    }
    #[doc = "Bit 14 - Valid Flag of Result Register x"]
    #[inline(always)]
    #[must_use]
    pub fn vf14(&mut self) -> VF14_W<VFR_SPEC> {
        VF14_W::new(self, 14)
    }
    #[doc = "Bit 15 - Valid Flag of Result Register x"]
    #[inline(always)]
    #[must_use]
    pub fn vf15(&mut self) -> VF15_W<VFR_SPEC> {
        VF15_W::new(self, 15)
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
#[doc = "Valid Flag Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vfr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vfr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VFR_SPEC;
impl crate::RegisterSpec for VFR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vfr::R`](R) reader structure"]
impl crate::Readable for VFR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`vfr::W`](W) writer structure"]
impl crate::Writable for VFR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets VFR to value 0"]
impl crate::Resettable for VFR_SPEC {
    const RESET_VALUE: u32 = 0;
}
