#[doc = "Register `HWSEL` reader"]
pub type R = crate::R<HWSEL_SPEC>;
#[doc = "Register `HWSEL` writer"]
pub type W = crate::W<HWSEL_SPEC>;
#[doc = "Field `HW0` reader - Port n Pin Hardware Select Bit 0"]
pub type HW0_R = crate::FieldReader<HW0_A>;
#[doc = "Port n Pin Hardware Select Bit 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum HW0_A {
    #[doc = "0: Software control only."]
    CONST_00 = 0,
    #[doc = "1: HWI0/HWO0 control path can override the software configuration."]
    CONST_01 = 1,
    #[doc = "2: HWI1/HWO1 control path can override the software configuration."]
    CONST_10 = 2,
}
impl From<HW0_A> for u8 {
    #[inline(always)]
    fn from(variant: HW0_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for HW0_A {
    type Ux = u8;
}
impl HW0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<HW0_A> {
        match self.bits {
            0 => Some(HW0_A::CONST_00),
            1 => Some(HW0_A::CONST_01),
            2 => Some(HW0_A::CONST_10),
            _ => None,
        }
    }
    #[doc = "Software control only."]
    #[inline(always)]
    pub fn is_const_00(&self) -> bool {
        *self == HW0_A::CONST_00
    }
    #[doc = "HWI0/HWO0 control path can override the software configuration."]
    #[inline(always)]
    pub fn is_const_01(&self) -> bool {
        *self == HW0_A::CONST_01
    }
    #[doc = "HWI1/HWO1 control path can override the software configuration."]
    #[inline(always)]
    pub fn is_const_10(&self) -> bool {
        *self == HW0_A::CONST_10
    }
}
#[doc = "Field `HW0` writer - Port n Pin Hardware Select Bit 0"]
pub type HW0_W<'a, REG> = crate::FieldWriter<'a, REG, 2, HW0_A>;
impl<'a, REG> HW0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Software control only."]
    #[inline(always)]
    pub fn const_00(self) -> &'a mut crate::W<REG> {
        self.variant(HW0_A::CONST_00)
    }
    #[doc = "HWI0/HWO0 control path can override the software configuration."]
    #[inline(always)]
    pub fn const_01(self) -> &'a mut crate::W<REG> {
        self.variant(HW0_A::CONST_01)
    }
    #[doc = "HWI1/HWO1 control path can override the software configuration."]
    #[inline(always)]
    pub fn const_10(self) -> &'a mut crate::W<REG> {
        self.variant(HW0_A::CONST_10)
    }
}
#[doc = "Field `HW1` reader - Port n Pin Hardware Select Bit 1"]
pub type HW1_R = crate::FieldReader<HW1_A>;
#[doc = "Port n Pin Hardware Select Bit 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum HW1_A {
    #[doc = "0: Software control only."]
    CONST_00 = 0,
    #[doc = "1: HWI0/HWO0 control path can override the software configuration."]
    CONST_01 = 1,
    #[doc = "2: HWI1/HWO1 control path can override the software configuration."]
    CONST_10 = 2,
}
impl From<HW1_A> for u8 {
    #[inline(always)]
    fn from(variant: HW1_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for HW1_A {
    type Ux = u8;
}
impl HW1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<HW1_A> {
        match self.bits {
            0 => Some(HW1_A::CONST_00),
            1 => Some(HW1_A::CONST_01),
            2 => Some(HW1_A::CONST_10),
            _ => None,
        }
    }
    #[doc = "Software control only."]
    #[inline(always)]
    pub fn is_const_00(&self) -> bool {
        *self == HW1_A::CONST_00
    }
    #[doc = "HWI0/HWO0 control path can override the software configuration."]
    #[inline(always)]
    pub fn is_const_01(&self) -> bool {
        *self == HW1_A::CONST_01
    }
    #[doc = "HWI1/HWO1 control path can override the software configuration."]
    #[inline(always)]
    pub fn is_const_10(&self) -> bool {
        *self == HW1_A::CONST_10
    }
}
#[doc = "Field `HW1` writer - Port n Pin Hardware Select Bit 1"]
pub type HW1_W<'a, REG> = crate::FieldWriter<'a, REG, 2, HW1_A>;
impl<'a, REG> HW1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Software control only."]
    #[inline(always)]
    pub fn const_00(self) -> &'a mut crate::W<REG> {
        self.variant(HW1_A::CONST_00)
    }
    #[doc = "HWI0/HWO0 control path can override the software configuration."]
    #[inline(always)]
    pub fn const_01(self) -> &'a mut crate::W<REG> {
        self.variant(HW1_A::CONST_01)
    }
    #[doc = "HWI1/HWO1 control path can override the software configuration."]
    #[inline(always)]
    pub fn const_10(self) -> &'a mut crate::W<REG> {
        self.variant(HW1_A::CONST_10)
    }
}
#[doc = "Field `HW2` reader - Port n Pin Hardware Select Bit 2"]
pub type HW2_R = crate::FieldReader<HW2_A>;
#[doc = "Port n Pin Hardware Select Bit 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum HW2_A {
    #[doc = "0: Software control only."]
    CONST_00 = 0,
    #[doc = "1: HWI0/HWO0 control path can override the software configuration."]
    CONST_01 = 1,
    #[doc = "2: HWI1/HWO1 control path can override the software configuration."]
    CONST_10 = 2,
}
impl From<HW2_A> for u8 {
    #[inline(always)]
    fn from(variant: HW2_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for HW2_A {
    type Ux = u8;
}
impl HW2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<HW2_A> {
        match self.bits {
            0 => Some(HW2_A::CONST_00),
            1 => Some(HW2_A::CONST_01),
            2 => Some(HW2_A::CONST_10),
            _ => None,
        }
    }
    #[doc = "Software control only."]
    #[inline(always)]
    pub fn is_const_00(&self) -> bool {
        *self == HW2_A::CONST_00
    }
    #[doc = "HWI0/HWO0 control path can override the software configuration."]
    #[inline(always)]
    pub fn is_const_01(&self) -> bool {
        *self == HW2_A::CONST_01
    }
    #[doc = "HWI1/HWO1 control path can override the software configuration."]
    #[inline(always)]
    pub fn is_const_10(&self) -> bool {
        *self == HW2_A::CONST_10
    }
}
#[doc = "Field `HW2` writer - Port n Pin Hardware Select Bit 2"]
pub type HW2_W<'a, REG> = crate::FieldWriter<'a, REG, 2, HW2_A>;
impl<'a, REG> HW2_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Software control only."]
    #[inline(always)]
    pub fn const_00(self) -> &'a mut crate::W<REG> {
        self.variant(HW2_A::CONST_00)
    }
    #[doc = "HWI0/HWO0 control path can override the software configuration."]
    #[inline(always)]
    pub fn const_01(self) -> &'a mut crate::W<REG> {
        self.variant(HW2_A::CONST_01)
    }
    #[doc = "HWI1/HWO1 control path can override the software configuration."]
    #[inline(always)]
    pub fn const_10(self) -> &'a mut crate::W<REG> {
        self.variant(HW2_A::CONST_10)
    }
}
#[doc = "Field `HW3` reader - Port n Pin Hardware Select Bit 3"]
pub type HW3_R = crate::FieldReader<HW3_A>;
#[doc = "Port n Pin Hardware Select Bit 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum HW3_A {
    #[doc = "0: Software control only."]
    CONST_00 = 0,
    #[doc = "1: HWI0/HWO0 control path can override the software configuration."]
    CONST_01 = 1,
    #[doc = "2: HWI1/HWO1 control path can override the software configuration."]
    CONST_10 = 2,
}
impl From<HW3_A> for u8 {
    #[inline(always)]
    fn from(variant: HW3_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for HW3_A {
    type Ux = u8;
}
impl HW3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<HW3_A> {
        match self.bits {
            0 => Some(HW3_A::CONST_00),
            1 => Some(HW3_A::CONST_01),
            2 => Some(HW3_A::CONST_10),
            _ => None,
        }
    }
    #[doc = "Software control only."]
    #[inline(always)]
    pub fn is_const_00(&self) -> bool {
        *self == HW3_A::CONST_00
    }
    #[doc = "HWI0/HWO0 control path can override the software configuration."]
    #[inline(always)]
    pub fn is_const_01(&self) -> bool {
        *self == HW3_A::CONST_01
    }
    #[doc = "HWI1/HWO1 control path can override the software configuration."]
    #[inline(always)]
    pub fn is_const_10(&self) -> bool {
        *self == HW3_A::CONST_10
    }
}
#[doc = "Field `HW3` writer - Port n Pin Hardware Select Bit 3"]
pub type HW3_W<'a, REG> = crate::FieldWriter<'a, REG, 2, HW3_A>;
impl<'a, REG> HW3_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Software control only."]
    #[inline(always)]
    pub fn const_00(self) -> &'a mut crate::W<REG> {
        self.variant(HW3_A::CONST_00)
    }
    #[doc = "HWI0/HWO0 control path can override the software configuration."]
    #[inline(always)]
    pub fn const_01(self) -> &'a mut crate::W<REG> {
        self.variant(HW3_A::CONST_01)
    }
    #[doc = "HWI1/HWO1 control path can override the software configuration."]
    #[inline(always)]
    pub fn const_10(self) -> &'a mut crate::W<REG> {
        self.variant(HW3_A::CONST_10)
    }
}
#[doc = "Field `HW4` reader - Port n Pin Hardware Select Bit 4"]
pub type HW4_R = crate::FieldReader<HW4_A>;
#[doc = "Port n Pin Hardware Select Bit 4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum HW4_A {
    #[doc = "0: Software control only."]
    CONST_00 = 0,
    #[doc = "1: HWI0/HWO0 control path can override the software configuration."]
    CONST_01 = 1,
    #[doc = "2: HWI1/HWO1 control path can override the software configuration."]
    CONST_10 = 2,
}
impl From<HW4_A> for u8 {
    #[inline(always)]
    fn from(variant: HW4_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for HW4_A {
    type Ux = u8;
}
impl HW4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<HW4_A> {
        match self.bits {
            0 => Some(HW4_A::CONST_00),
            1 => Some(HW4_A::CONST_01),
            2 => Some(HW4_A::CONST_10),
            _ => None,
        }
    }
    #[doc = "Software control only."]
    #[inline(always)]
    pub fn is_const_00(&self) -> bool {
        *self == HW4_A::CONST_00
    }
    #[doc = "HWI0/HWO0 control path can override the software configuration."]
    #[inline(always)]
    pub fn is_const_01(&self) -> bool {
        *self == HW4_A::CONST_01
    }
    #[doc = "HWI1/HWO1 control path can override the software configuration."]
    #[inline(always)]
    pub fn is_const_10(&self) -> bool {
        *self == HW4_A::CONST_10
    }
}
#[doc = "Field `HW4` writer - Port n Pin Hardware Select Bit 4"]
pub type HW4_W<'a, REG> = crate::FieldWriter<'a, REG, 2, HW4_A>;
impl<'a, REG> HW4_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Software control only."]
    #[inline(always)]
    pub fn const_00(self) -> &'a mut crate::W<REG> {
        self.variant(HW4_A::CONST_00)
    }
    #[doc = "HWI0/HWO0 control path can override the software configuration."]
    #[inline(always)]
    pub fn const_01(self) -> &'a mut crate::W<REG> {
        self.variant(HW4_A::CONST_01)
    }
    #[doc = "HWI1/HWO1 control path can override the software configuration."]
    #[inline(always)]
    pub fn const_10(self) -> &'a mut crate::W<REG> {
        self.variant(HW4_A::CONST_10)
    }
}
#[doc = "Field `HW5` reader - Port n Pin Hardware Select Bit 5"]
pub type HW5_R = crate::FieldReader<HW5_A>;
#[doc = "Port n Pin Hardware Select Bit 5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum HW5_A {
    #[doc = "0: Software control only."]
    CONST_00 = 0,
    #[doc = "1: HWI0/HWO0 control path can override the software configuration."]
    CONST_01 = 1,
    #[doc = "2: HWI1/HWO1 control path can override the software configuration."]
    CONST_10 = 2,
}
impl From<HW5_A> for u8 {
    #[inline(always)]
    fn from(variant: HW5_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for HW5_A {
    type Ux = u8;
}
impl HW5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<HW5_A> {
        match self.bits {
            0 => Some(HW5_A::CONST_00),
            1 => Some(HW5_A::CONST_01),
            2 => Some(HW5_A::CONST_10),
            _ => None,
        }
    }
    #[doc = "Software control only."]
    #[inline(always)]
    pub fn is_const_00(&self) -> bool {
        *self == HW5_A::CONST_00
    }
    #[doc = "HWI0/HWO0 control path can override the software configuration."]
    #[inline(always)]
    pub fn is_const_01(&self) -> bool {
        *self == HW5_A::CONST_01
    }
    #[doc = "HWI1/HWO1 control path can override the software configuration."]
    #[inline(always)]
    pub fn is_const_10(&self) -> bool {
        *self == HW5_A::CONST_10
    }
}
#[doc = "Field `HW5` writer - Port n Pin Hardware Select Bit 5"]
pub type HW5_W<'a, REG> = crate::FieldWriter<'a, REG, 2, HW5_A>;
impl<'a, REG> HW5_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Software control only."]
    #[inline(always)]
    pub fn const_00(self) -> &'a mut crate::W<REG> {
        self.variant(HW5_A::CONST_00)
    }
    #[doc = "HWI0/HWO0 control path can override the software configuration."]
    #[inline(always)]
    pub fn const_01(self) -> &'a mut crate::W<REG> {
        self.variant(HW5_A::CONST_01)
    }
    #[doc = "HWI1/HWO1 control path can override the software configuration."]
    #[inline(always)]
    pub fn const_10(self) -> &'a mut crate::W<REG> {
        self.variant(HW5_A::CONST_10)
    }
}
#[doc = "Field `HW6` reader - Port n Pin Hardware Select Bit 6"]
pub type HW6_R = crate::FieldReader<HW6_A>;
#[doc = "Port n Pin Hardware Select Bit 6\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum HW6_A {
    #[doc = "0: Software control only."]
    CONST_00 = 0,
    #[doc = "1: HWI0/HWO0 control path can override the software configuration."]
    CONST_01 = 1,
    #[doc = "2: HWI1/HWO1 control path can override the software configuration."]
    CONST_10 = 2,
}
impl From<HW6_A> for u8 {
    #[inline(always)]
    fn from(variant: HW6_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for HW6_A {
    type Ux = u8;
}
impl HW6_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<HW6_A> {
        match self.bits {
            0 => Some(HW6_A::CONST_00),
            1 => Some(HW6_A::CONST_01),
            2 => Some(HW6_A::CONST_10),
            _ => None,
        }
    }
    #[doc = "Software control only."]
    #[inline(always)]
    pub fn is_const_00(&self) -> bool {
        *self == HW6_A::CONST_00
    }
    #[doc = "HWI0/HWO0 control path can override the software configuration."]
    #[inline(always)]
    pub fn is_const_01(&self) -> bool {
        *self == HW6_A::CONST_01
    }
    #[doc = "HWI1/HWO1 control path can override the software configuration."]
    #[inline(always)]
    pub fn is_const_10(&self) -> bool {
        *self == HW6_A::CONST_10
    }
}
#[doc = "Field `HW6` writer - Port n Pin Hardware Select Bit 6"]
pub type HW6_W<'a, REG> = crate::FieldWriter<'a, REG, 2, HW6_A>;
impl<'a, REG> HW6_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Software control only."]
    #[inline(always)]
    pub fn const_00(self) -> &'a mut crate::W<REG> {
        self.variant(HW6_A::CONST_00)
    }
    #[doc = "HWI0/HWO0 control path can override the software configuration."]
    #[inline(always)]
    pub fn const_01(self) -> &'a mut crate::W<REG> {
        self.variant(HW6_A::CONST_01)
    }
    #[doc = "HWI1/HWO1 control path can override the software configuration."]
    #[inline(always)]
    pub fn const_10(self) -> &'a mut crate::W<REG> {
        self.variant(HW6_A::CONST_10)
    }
}
#[doc = "Field `HW7` reader - Port n Pin Hardware Select Bit 7"]
pub type HW7_R = crate::FieldReader<HW7_A>;
#[doc = "Port n Pin Hardware Select Bit 7\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum HW7_A {
    #[doc = "0: Software control only."]
    CONST_00 = 0,
    #[doc = "1: HWI0/HWO0 control path can override the software configuration."]
    CONST_01 = 1,
    #[doc = "2: HWI1/HWO1 control path can override the software configuration."]
    CONST_10 = 2,
}
impl From<HW7_A> for u8 {
    #[inline(always)]
    fn from(variant: HW7_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for HW7_A {
    type Ux = u8;
}
impl HW7_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<HW7_A> {
        match self.bits {
            0 => Some(HW7_A::CONST_00),
            1 => Some(HW7_A::CONST_01),
            2 => Some(HW7_A::CONST_10),
            _ => None,
        }
    }
    #[doc = "Software control only."]
    #[inline(always)]
    pub fn is_const_00(&self) -> bool {
        *self == HW7_A::CONST_00
    }
    #[doc = "HWI0/HWO0 control path can override the software configuration."]
    #[inline(always)]
    pub fn is_const_01(&self) -> bool {
        *self == HW7_A::CONST_01
    }
    #[doc = "HWI1/HWO1 control path can override the software configuration."]
    #[inline(always)]
    pub fn is_const_10(&self) -> bool {
        *self == HW7_A::CONST_10
    }
}
#[doc = "Field `HW7` writer - Port n Pin Hardware Select Bit 7"]
pub type HW7_W<'a, REG> = crate::FieldWriter<'a, REG, 2, HW7_A>;
impl<'a, REG> HW7_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Software control only."]
    #[inline(always)]
    pub fn const_00(self) -> &'a mut crate::W<REG> {
        self.variant(HW7_A::CONST_00)
    }
    #[doc = "HWI0/HWO0 control path can override the software configuration."]
    #[inline(always)]
    pub fn const_01(self) -> &'a mut crate::W<REG> {
        self.variant(HW7_A::CONST_01)
    }
    #[doc = "HWI1/HWO1 control path can override the software configuration."]
    #[inline(always)]
    pub fn const_10(self) -> &'a mut crate::W<REG> {
        self.variant(HW7_A::CONST_10)
    }
}
#[doc = "Field `HW8` reader - Port n Pin Hardware Select Bit 8"]
pub type HW8_R = crate::FieldReader<HW8_A>;
#[doc = "Port n Pin Hardware Select Bit 8\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum HW8_A {
    #[doc = "0: Software control only."]
    CONST_00 = 0,
    #[doc = "1: HWI0/HWO0 control path can override the software configuration."]
    CONST_01 = 1,
    #[doc = "2: HWI1/HWO1 control path can override the software configuration."]
    CONST_10 = 2,
}
impl From<HW8_A> for u8 {
    #[inline(always)]
    fn from(variant: HW8_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for HW8_A {
    type Ux = u8;
}
impl HW8_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<HW8_A> {
        match self.bits {
            0 => Some(HW8_A::CONST_00),
            1 => Some(HW8_A::CONST_01),
            2 => Some(HW8_A::CONST_10),
            _ => None,
        }
    }
    #[doc = "Software control only."]
    #[inline(always)]
    pub fn is_const_00(&self) -> bool {
        *self == HW8_A::CONST_00
    }
    #[doc = "HWI0/HWO0 control path can override the software configuration."]
    #[inline(always)]
    pub fn is_const_01(&self) -> bool {
        *self == HW8_A::CONST_01
    }
    #[doc = "HWI1/HWO1 control path can override the software configuration."]
    #[inline(always)]
    pub fn is_const_10(&self) -> bool {
        *self == HW8_A::CONST_10
    }
}
#[doc = "Field `HW8` writer - Port n Pin Hardware Select Bit 8"]
pub type HW8_W<'a, REG> = crate::FieldWriter<'a, REG, 2, HW8_A>;
impl<'a, REG> HW8_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Software control only."]
    #[inline(always)]
    pub fn const_00(self) -> &'a mut crate::W<REG> {
        self.variant(HW8_A::CONST_00)
    }
    #[doc = "HWI0/HWO0 control path can override the software configuration."]
    #[inline(always)]
    pub fn const_01(self) -> &'a mut crate::W<REG> {
        self.variant(HW8_A::CONST_01)
    }
    #[doc = "HWI1/HWO1 control path can override the software configuration."]
    #[inline(always)]
    pub fn const_10(self) -> &'a mut crate::W<REG> {
        self.variant(HW8_A::CONST_10)
    }
}
#[doc = "Field `HW9` reader - Port n Pin Hardware Select Bit 9"]
pub type HW9_R = crate::FieldReader<HW9_A>;
#[doc = "Port n Pin Hardware Select Bit 9\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum HW9_A {
    #[doc = "0: Software control only."]
    CONST_00 = 0,
    #[doc = "1: HWI0/HWO0 control path can override the software configuration."]
    CONST_01 = 1,
    #[doc = "2: HWI1/HWO1 control path can override the software configuration."]
    CONST_10 = 2,
}
impl From<HW9_A> for u8 {
    #[inline(always)]
    fn from(variant: HW9_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for HW9_A {
    type Ux = u8;
}
impl HW9_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<HW9_A> {
        match self.bits {
            0 => Some(HW9_A::CONST_00),
            1 => Some(HW9_A::CONST_01),
            2 => Some(HW9_A::CONST_10),
            _ => None,
        }
    }
    #[doc = "Software control only."]
    #[inline(always)]
    pub fn is_const_00(&self) -> bool {
        *self == HW9_A::CONST_00
    }
    #[doc = "HWI0/HWO0 control path can override the software configuration."]
    #[inline(always)]
    pub fn is_const_01(&self) -> bool {
        *self == HW9_A::CONST_01
    }
    #[doc = "HWI1/HWO1 control path can override the software configuration."]
    #[inline(always)]
    pub fn is_const_10(&self) -> bool {
        *self == HW9_A::CONST_10
    }
}
#[doc = "Field `HW9` writer - Port n Pin Hardware Select Bit 9"]
pub type HW9_W<'a, REG> = crate::FieldWriter<'a, REG, 2, HW9_A>;
impl<'a, REG> HW9_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Software control only."]
    #[inline(always)]
    pub fn const_00(self) -> &'a mut crate::W<REG> {
        self.variant(HW9_A::CONST_00)
    }
    #[doc = "HWI0/HWO0 control path can override the software configuration."]
    #[inline(always)]
    pub fn const_01(self) -> &'a mut crate::W<REG> {
        self.variant(HW9_A::CONST_01)
    }
    #[doc = "HWI1/HWO1 control path can override the software configuration."]
    #[inline(always)]
    pub fn const_10(self) -> &'a mut crate::W<REG> {
        self.variant(HW9_A::CONST_10)
    }
}
#[doc = "Field `HW10` reader - Port n Pin Hardware Select Bit 10"]
pub type HW10_R = crate::FieldReader<HW10_A>;
#[doc = "Port n Pin Hardware Select Bit 10\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum HW10_A {
    #[doc = "0: Software control only."]
    CONST_00 = 0,
    #[doc = "1: HWI0/HWO0 control path can override the software configuration."]
    CONST_01 = 1,
    #[doc = "2: HWI1/HWO1 control path can override the software configuration."]
    CONST_10 = 2,
}
impl From<HW10_A> for u8 {
    #[inline(always)]
    fn from(variant: HW10_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for HW10_A {
    type Ux = u8;
}
impl HW10_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<HW10_A> {
        match self.bits {
            0 => Some(HW10_A::CONST_00),
            1 => Some(HW10_A::CONST_01),
            2 => Some(HW10_A::CONST_10),
            _ => None,
        }
    }
    #[doc = "Software control only."]
    #[inline(always)]
    pub fn is_const_00(&self) -> bool {
        *self == HW10_A::CONST_00
    }
    #[doc = "HWI0/HWO0 control path can override the software configuration."]
    #[inline(always)]
    pub fn is_const_01(&self) -> bool {
        *self == HW10_A::CONST_01
    }
    #[doc = "HWI1/HWO1 control path can override the software configuration."]
    #[inline(always)]
    pub fn is_const_10(&self) -> bool {
        *self == HW10_A::CONST_10
    }
}
#[doc = "Field `HW10` writer - Port n Pin Hardware Select Bit 10"]
pub type HW10_W<'a, REG> = crate::FieldWriter<'a, REG, 2, HW10_A>;
impl<'a, REG> HW10_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Software control only."]
    #[inline(always)]
    pub fn const_00(self) -> &'a mut crate::W<REG> {
        self.variant(HW10_A::CONST_00)
    }
    #[doc = "HWI0/HWO0 control path can override the software configuration."]
    #[inline(always)]
    pub fn const_01(self) -> &'a mut crate::W<REG> {
        self.variant(HW10_A::CONST_01)
    }
    #[doc = "HWI1/HWO1 control path can override the software configuration."]
    #[inline(always)]
    pub fn const_10(self) -> &'a mut crate::W<REG> {
        self.variant(HW10_A::CONST_10)
    }
}
#[doc = "Field `HW11` reader - Port n Pin Hardware Select Bit 11"]
pub type HW11_R = crate::FieldReader<HW11_A>;
#[doc = "Port n Pin Hardware Select Bit 11\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum HW11_A {
    #[doc = "0: Software control only."]
    CONST_00 = 0,
    #[doc = "1: HWI0/HWO0 control path can override the software configuration."]
    CONST_01 = 1,
    #[doc = "2: HWI1/HWO1 control path can override the software configuration."]
    CONST_10 = 2,
}
impl From<HW11_A> for u8 {
    #[inline(always)]
    fn from(variant: HW11_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for HW11_A {
    type Ux = u8;
}
impl HW11_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<HW11_A> {
        match self.bits {
            0 => Some(HW11_A::CONST_00),
            1 => Some(HW11_A::CONST_01),
            2 => Some(HW11_A::CONST_10),
            _ => None,
        }
    }
    #[doc = "Software control only."]
    #[inline(always)]
    pub fn is_const_00(&self) -> bool {
        *self == HW11_A::CONST_00
    }
    #[doc = "HWI0/HWO0 control path can override the software configuration."]
    #[inline(always)]
    pub fn is_const_01(&self) -> bool {
        *self == HW11_A::CONST_01
    }
    #[doc = "HWI1/HWO1 control path can override the software configuration."]
    #[inline(always)]
    pub fn is_const_10(&self) -> bool {
        *self == HW11_A::CONST_10
    }
}
#[doc = "Field `HW11` writer - Port n Pin Hardware Select Bit 11"]
pub type HW11_W<'a, REG> = crate::FieldWriter<'a, REG, 2, HW11_A>;
impl<'a, REG> HW11_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Software control only."]
    #[inline(always)]
    pub fn const_00(self) -> &'a mut crate::W<REG> {
        self.variant(HW11_A::CONST_00)
    }
    #[doc = "HWI0/HWO0 control path can override the software configuration."]
    #[inline(always)]
    pub fn const_01(self) -> &'a mut crate::W<REG> {
        self.variant(HW11_A::CONST_01)
    }
    #[doc = "HWI1/HWO1 control path can override the software configuration."]
    #[inline(always)]
    pub fn const_10(self) -> &'a mut crate::W<REG> {
        self.variant(HW11_A::CONST_10)
    }
}
#[doc = "Field `HW12` reader - Port n Pin Hardware Select Bit 12"]
pub type HW12_R = crate::FieldReader<HW12_A>;
#[doc = "Port n Pin Hardware Select Bit 12\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum HW12_A {
    #[doc = "0: Software control only."]
    CONST_00 = 0,
    #[doc = "1: HWI0/HWO0 control path can override the software configuration."]
    CONST_01 = 1,
    #[doc = "2: HWI1/HWO1 control path can override the software configuration."]
    CONST_10 = 2,
}
impl From<HW12_A> for u8 {
    #[inline(always)]
    fn from(variant: HW12_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for HW12_A {
    type Ux = u8;
}
impl HW12_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<HW12_A> {
        match self.bits {
            0 => Some(HW12_A::CONST_00),
            1 => Some(HW12_A::CONST_01),
            2 => Some(HW12_A::CONST_10),
            _ => None,
        }
    }
    #[doc = "Software control only."]
    #[inline(always)]
    pub fn is_const_00(&self) -> bool {
        *self == HW12_A::CONST_00
    }
    #[doc = "HWI0/HWO0 control path can override the software configuration."]
    #[inline(always)]
    pub fn is_const_01(&self) -> bool {
        *self == HW12_A::CONST_01
    }
    #[doc = "HWI1/HWO1 control path can override the software configuration."]
    #[inline(always)]
    pub fn is_const_10(&self) -> bool {
        *self == HW12_A::CONST_10
    }
}
#[doc = "Field `HW12` writer - Port n Pin Hardware Select Bit 12"]
pub type HW12_W<'a, REG> = crate::FieldWriter<'a, REG, 2, HW12_A>;
impl<'a, REG> HW12_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Software control only."]
    #[inline(always)]
    pub fn const_00(self) -> &'a mut crate::W<REG> {
        self.variant(HW12_A::CONST_00)
    }
    #[doc = "HWI0/HWO0 control path can override the software configuration."]
    #[inline(always)]
    pub fn const_01(self) -> &'a mut crate::W<REG> {
        self.variant(HW12_A::CONST_01)
    }
    #[doc = "HWI1/HWO1 control path can override the software configuration."]
    #[inline(always)]
    pub fn const_10(self) -> &'a mut crate::W<REG> {
        self.variant(HW12_A::CONST_10)
    }
}
#[doc = "Field `HW13` reader - Port n Pin Hardware Select Bit 13"]
pub type HW13_R = crate::FieldReader<HW13_A>;
#[doc = "Port n Pin Hardware Select Bit 13\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum HW13_A {
    #[doc = "0: Software control only."]
    CONST_00 = 0,
    #[doc = "1: HWI0/HWO0 control path can override the software configuration."]
    CONST_01 = 1,
    #[doc = "2: HWI1/HWO1 control path can override the software configuration."]
    CONST_10 = 2,
}
impl From<HW13_A> for u8 {
    #[inline(always)]
    fn from(variant: HW13_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for HW13_A {
    type Ux = u8;
}
impl HW13_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<HW13_A> {
        match self.bits {
            0 => Some(HW13_A::CONST_00),
            1 => Some(HW13_A::CONST_01),
            2 => Some(HW13_A::CONST_10),
            _ => None,
        }
    }
    #[doc = "Software control only."]
    #[inline(always)]
    pub fn is_const_00(&self) -> bool {
        *self == HW13_A::CONST_00
    }
    #[doc = "HWI0/HWO0 control path can override the software configuration."]
    #[inline(always)]
    pub fn is_const_01(&self) -> bool {
        *self == HW13_A::CONST_01
    }
    #[doc = "HWI1/HWO1 control path can override the software configuration."]
    #[inline(always)]
    pub fn is_const_10(&self) -> bool {
        *self == HW13_A::CONST_10
    }
}
#[doc = "Field `HW13` writer - Port n Pin Hardware Select Bit 13"]
pub type HW13_W<'a, REG> = crate::FieldWriter<'a, REG, 2, HW13_A>;
impl<'a, REG> HW13_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Software control only."]
    #[inline(always)]
    pub fn const_00(self) -> &'a mut crate::W<REG> {
        self.variant(HW13_A::CONST_00)
    }
    #[doc = "HWI0/HWO0 control path can override the software configuration."]
    #[inline(always)]
    pub fn const_01(self) -> &'a mut crate::W<REG> {
        self.variant(HW13_A::CONST_01)
    }
    #[doc = "HWI1/HWO1 control path can override the software configuration."]
    #[inline(always)]
    pub fn const_10(self) -> &'a mut crate::W<REG> {
        self.variant(HW13_A::CONST_10)
    }
}
#[doc = "Field `HW14` reader - Port n Pin Hardware Select Bit 14"]
pub type HW14_R = crate::FieldReader<HW14_A>;
#[doc = "Port n Pin Hardware Select Bit 14\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum HW14_A {
    #[doc = "0: Software control only."]
    CONST_00 = 0,
    #[doc = "1: HWI0/HWO0 control path can override the software configuration."]
    CONST_01 = 1,
    #[doc = "2: HWI1/HWO1 control path can override the software configuration."]
    CONST_10 = 2,
}
impl From<HW14_A> for u8 {
    #[inline(always)]
    fn from(variant: HW14_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for HW14_A {
    type Ux = u8;
}
impl HW14_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<HW14_A> {
        match self.bits {
            0 => Some(HW14_A::CONST_00),
            1 => Some(HW14_A::CONST_01),
            2 => Some(HW14_A::CONST_10),
            _ => None,
        }
    }
    #[doc = "Software control only."]
    #[inline(always)]
    pub fn is_const_00(&self) -> bool {
        *self == HW14_A::CONST_00
    }
    #[doc = "HWI0/HWO0 control path can override the software configuration."]
    #[inline(always)]
    pub fn is_const_01(&self) -> bool {
        *self == HW14_A::CONST_01
    }
    #[doc = "HWI1/HWO1 control path can override the software configuration."]
    #[inline(always)]
    pub fn is_const_10(&self) -> bool {
        *self == HW14_A::CONST_10
    }
}
#[doc = "Field `HW14` writer - Port n Pin Hardware Select Bit 14"]
pub type HW14_W<'a, REG> = crate::FieldWriter<'a, REG, 2, HW14_A>;
impl<'a, REG> HW14_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Software control only."]
    #[inline(always)]
    pub fn const_00(self) -> &'a mut crate::W<REG> {
        self.variant(HW14_A::CONST_00)
    }
    #[doc = "HWI0/HWO0 control path can override the software configuration."]
    #[inline(always)]
    pub fn const_01(self) -> &'a mut crate::W<REG> {
        self.variant(HW14_A::CONST_01)
    }
    #[doc = "HWI1/HWO1 control path can override the software configuration."]
    #[inline(always)]
    pub fn const_10(self) -> &'a mut crate::W<REG> {
        self.variant(HW14_A::CONST_10)
    }
}
#[doc = "Field `HW15` reader - Port n Pin Hardware Select Bit 15"]
pub type HW15_R = crate::FieldReader<HW15_A>;
#[doc = "Port n Pin Hardware Select Bit 15\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum HW15_A {
    #[doc = "0: Software control only."]
    CONST_00 = 0,
    #[doc = "1: HWI0/HWO0 control path can override the software configuration."]
    CONST_01 = 1,
    #[doc = "2: HWI1/HWO1 control path can override the software configuration."]
    CONST_10 = 2,
}
impl From<HW15_A> for u8 {
    #[inline(always)]
    fn from(variant: HW15_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for HW15_A {
    type Ux = u8;
}
impl HW15_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<HW15_A> {
        match self.bits {
            0 => Some(HW15_A::CONST_00),
            1 => Some(HW15_A::CONST_01),
            2 => Some(HW15_A::CONST_10),
            _ => None,
        }
    }
    #[doc = "Software control only."]
    #[inline(always)]
    pub fn is_const_00(&self) -> bool {
        *self == HW15_A::CONST_00
    }
    #[doc = "HWI0/HWO0 control path can override the software configuration."]
    #[inline(always)]
    pub fn is_const_01(&self) -> bool {
        *self == HW15_A::CONST_01
    }
    #[doc = "HWI1/HWO1 control path can override the software configuration."]
    #[inline(always)]
    pub fn is_const_10(&self) -> bool {
        *self == HW15_A::CONST_10
    }
}
#[doc = "Field `HW15` writer - Port n Pin Hardware Select Bit 15"]
pub type HW15_W<'a, REG> = crate::FieldWriter<'a, REG, 2, HW15_A>;
impl<'a, REG> HW15_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Software control only."]
    #[inline(always)]
    pub fn const_00(self) -> &'a mut crate::W<REG> {
        self.variant(HW15_A::CONST_00)
    }
    #[doc = "HWI0/HWO0 control path can override the software configuration."]
    #[inline(always)]
    pub fn const_01(self) -> &'a mut crate::W<REG> {
        self.variant(HW15_A::CONST_01)
    }
    #[doc = "HWI1/HWO1 control path can override the software configuration."]
    #[inline(always)]
    pub fn const_10(self) -> &'a mut crate::W<REG> {
        self.variant(HW15_A::CONST_10)
    }
}
impl R {
    #[doc = "Bits 0:1 - Port n Pin Hardware Select Bit 0"]
    #[inline(always)]
    pub fn hw0(&self) -> HW0_R {
        HW0_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Port n Pin Hardware Select Bit 1"]
    #[inline(always)]
    pub fn hw1(&self) -> HW1_R {
        HW1_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Port n Pin Hardware Select Bit 2"]
    #[inline(always)]
    pub fn hw2(&self) -> HW2_R {
        HW2_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Port n Pin Hardware Select Bit 3"]
    #[inline(always)]
    pub fn hw3(&self) -> HW3_R {
        HW3_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Port n Pin Hardware Select Bit 4"]
    #[inline(always)]
    pub fn hw4(&self) -> HW4_R {
        HW4_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Port n Pin Hardware Select Bit 5"]
    #[inline(always)]
    pub fn hw5(&self) -> HW5_R {
        HW5_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Port n Pin Hardware Select Bit 6"]
    #[inline(always)]
    pub fn hw6(&self) -> HW6_R {
        HW6_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Port n Pin Hardware Select Bit 7"]
    #[inline(always)]
    pub fn hw7(&self) -> HW7_R {
        HW7_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - Port n Pin Hardware Select Bit 8"]
    #[inline(always)]
    pub fn hw8(&self) -> HW8_R {
        HW8_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - Port n Pin Hardware Select Bit 9"]
    #[inline(always)]
    pub fn hw9(&self) -> HW9_R {
        HW9_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - Port n Pin Hardware Select Bit 10"]
    #[inline(always)]
    pub fn hw10(&self) -> HW10_R {
        HW10_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - Port n Pin Hardware Select Bit 11"]
    #[inline(always)]
    pub fn hw11(&self) -> HW11_R {
        HW11_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:25 - Port n Pin Hardware Select Bit 12"]
    #[inline(always)]
    pub fn hw12(&self) -> HW12_R {
        HW12_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27 - Port n Pin Hardware Select Bit 13"]
    #[inline(always)]
    pub fn hw13(&self) -> HW13_R {
        HW13_R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:29 - Port n Pin Hardware Select Bit 14"]
    #[inline(always)]
    pub fn hw14(&self) -> HW14_R {
        HW14_R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31 - Port n Pin Hardware Select Bit 15"]
    #[inline(always)]
    pub fn hw15(&self) -> HW15_R {
        HW15_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Port n Pin Hardware Select Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn hw0(&mut self) -> HW0_W<HWSEL_SPEC> {
        HW0_W::new(self, 0)
    }
    #[doc = "Bits 2:3 - Port n Pin Hardware Select Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn hw1(&mut self) -> HW1_W<HWSEL_SPEC> {
        HW1_W::new(self, 2)
    }
    #[doc = "Bits 4:5 - Port n Pin Hardware Select Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn hw2(&mut self) -> HW2_W<HWSEL_SPEC> {
        HW2_W::new(self, 4)
    }
    #[doc = "Bits 6:7 - Port n Pin Hardware Select Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn hw3(&mut self) -> HW3_W<HWSEL_SPEC> {
        HW3_W::new(self, 6)
    }
    #[doc = "Bits 8:9 - Port n Pin Hardware Select Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn hw4(&mut self) -> HW4_W<HWSEL_SPEC> {
        HW4_W::new(self, 8)
    }
    #[doc = "Bits 10:11 - Port n Pin Hardware Select Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn hw5(&mut self) -> HW5_W<HWSEL_SPEC> {
        HW5_W::new(self, 10)
    }
    #[doc = "Bits 12:13 - Port n Pin Hardware Select Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn hw6(&mut self) -> HW6_W<HWSEL_SPEC> {
        HW6_W::new(self, 12)
    }
    #[doc = "Bits 14:15 - Port n Pin Hardware Select Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn hw7(&mut self) -> HW7_W<HWSEL_SPEC> {
        HW7_W::new(self, 14)
    }
    #[doc = "Bits 16:17 - Port n Pin Hardware Select Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn hw8(&mut self) -> HW8_W<HWSEL_SPEC> {
        HW8_W::new(self, 16)
    }
    #[doc = "Bits 18:19 - Port n Pin Hardware Select Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn hw9(&mut self) -> HW9_W<HWSEL_SPEC> {
        HW9_W::new(self, 18)
    }
    #[doc = "Bits 20:21 - Port n Pin Hardware Select Bit 10"]
    #[inline(always)]
    #[must_use]
    pub fn hw10(&mut self) -> HW10_W<HWSEL_SPEC> {
        HW10_W::new(self, 20)
    }
    #[doc = "Bits 22:23 - Port n Pin Hardware Select Bit 11"]
    #[inline(always)]
    #[must_use]
    pub fn hw11(&mut self) -> HW11_W<HWSEL_SPEC> {
        HW11_W::new(self, 22)
    }
    #[doc = "Bits 24:25 - Port n Pin Hardware Select Bit 12"]
    #[inline(always)]
    #[must_use]
    pub fn hw12(&mut self) -> HW12_W<HWSEL_SPEC> {
        HW12_W::new(self, 24)
    }
    #[doc = "Bits 26:27 - Port n Pin Hardware Select Bit 13"]
    #[inline(always)]
    #[must_use]
    pub fn hw13(&mut self) -> HW13_W<HWSEL_SPEC> {
        HW13_W::new(self, 26)
    }
    #[doc = "Bits 28:29 - Port n Pin Hardware Select Bit 14"]
    #[inline(always)]
    #[must_use]
    pub fn hw14(&mut self) -> HW14_W<HWSEL_SPEC> {
        HW14_W::new(self, 28)
    }
    #[doc = "Bits 30:31 - Port n Pin Hardware Select Bit 15"]
    #[inline(always)]
    #[must_use]
    pub fn hw15(&mut self) -> HW15_W<HWSEL_SPEC> {
        HW15_W::new(self, 30)
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
#[doc = "Port 1 Pin Hardware Select Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hwsel::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hwsel::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HWSEL_SPEC;
impl crate::RegisterSpec for HWSEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hwsel::R`](R) reader structure"]
impl crate::Readable for HWSEL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hwsel::W`](W) writer structure"]
impl crate::Writable for HWSEL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HWSEL to value 0"]
impl crate::Resettable for HWSEL_SPEC {
    const RESET_VALUE: u32 = 0;
}
