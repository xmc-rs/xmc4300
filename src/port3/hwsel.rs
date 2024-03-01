#[doc = "Register `HWSEL` reader"]
pub type R = crate::R<HwselSpec>;
#[doc = "Register `HWSEL` writer"]
pub type W = crate::W<HwselSpec>;
#[doc = "Port n Pin Hardware Select Bit 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Hw0 {
    #[doc = "0: Software control only."]
    Const00 = 0,
    #[doc = "1: HWI0/HWO0 control path can override the software configuration."]
    Const01 = 1,
    #[doc = "2: HWI1/HWO1 control path can override the software configuration."]
    Const10 = 2,
}
impl From<Hw0> for u8 {
    #[inline(always)]
    fn from(variant: Hw0) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Hw0 {
    type Ux = u8;
}
#[doc = "Field `HW0` reader - Port n Pin Hardware Select Bit 0"]
pub type Hw0R = crate::FieldReader<Hw0>;
impl Hw0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Hw0> {
        match self.bits {
            0 => Some(Hw0::Const00),
            1 => Some(Hw0::Const01),
            2 => Some(Hw0::Const10),
            _ => None,
        }
    }
    #[doc = "Software control only."]
    #[inline(always)]
    pub fn is_const_00(&self) -> bool {
        *self == Hw0::Const00
    }
    #[doc = "HWI0/HWO0 control path can override the software configuration."]
    #[inline(always)]
    pub fn is_const_01(&self) -> bool {
        *self == Hw0::Const01
    }
    #[doc = "HWI1/HWO1 control path can override the software configuration."]
    #[inline(always)]
    pub fn is_const_10(&self) -> bool {
        *self == Hw0::Const10
    }
}
#[doc = "Field `HW0` writer - Port n Pin Hardware Select Bit 0"]
pub type Hw0W<'a, REG> = crate::FieldWriter<'a, REG, 2, Hw0>;
impl<'a, REG> Hw0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Software control only."]
    #[inline(always)]
    pub fn const_00(self) -> &'a mut crate::W<REG> {
        self.variant(Hw0::Const00)
    }
    #[doc = "HWI0/HWO0 control path can override the software configuration."]
    #[inline(always)]
    pub fn const_01(self) -> &'a mut crate::W<REG> {
        self.variant(Hw0::Const01)
    }
    #[doc = "HWI1/HWO1 control path can override the software configuration."]
    #[inline(always)]
    pub fn const_10(self) -> &'a mut crate::W<REG> {
        self.variant(Hw0::Const10)
    }
}
#[doc = "Port n Pin Hardware Select Bit 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Hw1 {
    #[doc = "0: Software control only."]
    Const00 = 0,
    #[doc = "1: HWI0/HWO0 control path can override the software configuration."]
    Const01 = 1,
    #[doc = "2: HWI1/HWO1 control path can override the software configuration."]
    Const10 = 2,
}
impl From<Hw1> for u8 {
    #[inline(always)]
    fn from(variant: Hw1) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Hw1 {
    type Ux = u8;
}
#[doc = "Field `HW1` reader - Port n Pin Hardware Select Bit 1"]
pub type Hw1R = crate::FieldReader<Hw1>;
impl Hw1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Hw1> {
        match self.bits {
            0 => Some(Hw1::Const00),
            1 => Some(Hw1::Const01),
            2 => Some(Hw1::Const10),
            _ => None,
        }
    }
    #[doc = "Software control only."]
    #[inline(always)]
    pub fn is_const_00(&self) -> bool {
        *self == Hw1::Const00
    }
    #[doc = "HWI0/HWO0 control path can override the software configuration."]
    #[inline(always)]
    pub fn is_const_01(&self) -> bool {
        *self == Hw1::Const01
    }
    #[doc = "HWI1/HWO1 control path can override the software configuration."]
    #[inline(always)]
    pub fn is_const_10(&self) -> bool {
        *self == Hw1::Const10
    }
}
#[doc = "Field `HW1` writer - Port n Pin Hardware Select Bit 1"]
pub type Hw1W<'a, REG> = crate::FieldWriter<'a, REG, 2, Hw1>;
impl<'a, REG> Hw1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Software control only."]
    #[inline(always)]
    pub fn const_00(self) -> &'a mut crate::W<REG> {
        self.variant(Hw1::Const00)
    }
    #[doc = "HWI0/HWO0 control path can override the software configuration."]
    #[inline(always)]
    pub fn const_01(self) -> &'a mut crate::W<REG> {
        self.variant(Hw1::Const01)
    }
    #[doc = "HWI1/HWO1 control path can override the software configuration."]
    #[inline(always)]
    pub fn const_10(self) -> &'a mut crate::W<REG> {
        self.variant(Hw1::Const10)
    }
}
#[doc = "Port n Pin Hardware Select Bit 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Hw2 {
    #[doc = "0: Software control only."]
    Const00 = 0,
    #[doc = "1: HWI0/HWO0 control path can override the software configuration."]
    Const01 = 1,
    #[doc = "2: HWI1/HWO1 control path can override the software configuration."]
    Const10 = 2,
}
impl From<Hw2> for u8 {
    #[inline(always)]
    fn from(variant: Hw2) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Hw2 {
    type Ux = u8;
}
#[doc = "Field `HW2` reader - Port n Pin Hardware Select Bit 2"]
pub type Hw2R = crate::FieldReader<Hw2>;
impl Hw2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Hw2> {
        match self.bits {
            0 => Some(Hw2::Const00),
            1 => Some(Hw2::Const01),
            2 => Some(Hw2::Const10),
            _ => None,
        }
    }
    #[doc = "Software control only."]
    #[inline(always)]
    pub fn is_const_00(&self) -> bool {
        *self == Hw2::Const00
    }
    #[doc = "HWI0/HWO0 control path can override the software configuration."]
    #[inline(always)]
    pub fn is_const_01(&self) -> bool {
        *self == Hw2::Const01
    }
    #[doc = "HWI1/HWO1 control path can override the software configuration."]
    #[inline(always)]
    pub fn is_const_10(&self) -> bool {
        *self == Hw2::Const10
    }
}
#[doc = "Field `HW2` writer - Port n Pin Hardware Select Bit 2"]
pub type Hw2W<'a, REG> = crate::FieldWriter<'a, REG, 2, Hw2>;
impl<'a, REG> Hw2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Software control only."]
    #[inline(always)]
    pub fn const_00(self) -> &'a mut crate::W<REG> {
        self.variant(Hw2::Const00)
    }
    #[doc = "HWI0/HWO0 control path can override the software configuration."]
    #[inline(always)]
    pub fn const_01(self) -> &'a mut crate::W<REG> {
        self.variant(Hw2::Const01)
    }
    #[doc = "HWI1/HWO1 control path can override the software configuration."]
    #[inline(always)]
    pub fn const_10(self) -> &'a mut crate::W<REG> {
        self.variant(Hw2::Const10)
    }
}
#[doc = "Port n Pin Hardware Select Bit 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Hw3 {
    #[doc = "0: Software control only."]
    Const00 = 0,
    #[doc = "1: HWI0/HWO0 control path can override the software configuration."]
    Const01 = 1,
    #[doc = "2: HWI1/HWO1 control path can override the software configuration."]
    Const10 = 2,
}
impl From<Hw3> for u8 {
    #[inline(always)]
    fn from(variant: Hw3) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Hw3 {
    type Ux = u8;
}
#[doc = "Field `HW3` reader - Port n Pin Hardware Select Bit 3"]
pub type Hw3R = crate::FieldReader<Hw3>;
impl Hw3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Hw3> {
        match self.bits {
            0 => Some(Hw3::Const00),
            1 => Some(Hw3::Const01),
            2 => Some(Hw3::Const10),
            _ => None,
        }
    }
    #[doc = "Software control only."]
    #[inline(always)]
    pub fn is_const_00(&self) -> bool {
        *self == Hw3::Const00
    }
    #[doc = "HWI0/HWO0 control path can override the software configuration."]
    #[inline(always)]
    pub fn is_const_01(&self) -> bool {
        *self == Hw3::Const01
    }
    #[doc = "HWI1/HWO1 control path can override the software configuration."]
    #[inline(always)]
    pub fn is_const_10(&self) -> bool {
        *self == Hw3::Const10
    }
}
#[doc = "Field `HW3` writer - Port n Pin Hardware Select Bit 3"]
pub type Hw3W<'a, REG> = crate::FieldWriter<'a, REG, 2, Hw3>;
impl<'a, REG> Hw3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Software control only."]
    #[inline(always)]
    pub fn const_00(self) -> &'a mut crate::W<REG> {
        self.variant(Hw3::Const00)
    }
    #[doc = "HWI0/HWO0 control path can override the software configuration."]
    #[inline(always)]
    pub fn const_01(self) -> &'a mut crate::W<REG> {
        self.variant(Hw3::Const01)
    }
    #[doc = "HWI1/HWO1 control path can override the software configuration."]
    #[inline(always)]
    pub fn const_10(self) -> &'a mut crate::W<REG> {
        self.variant(Hw3::Const10)
    }
}
#[doc = "Port n Pin Hardware Select Bit 4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Hw4 {
    #[doc = "0: Software control only."]
    Const00 = 0,
    #[doc = "1: HWI0/HWO0 control path can override the software configuration."]
    Const01 = 1,
    #[doc = "2: HWI1/HWO1 control path can override the software configuration."]
    Const10 = 2,
}
impl From<Hw4> for u8 {
    #[inline(always)]
    fn from(variant: Hw4) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Hw4 {
    type Ux = u8;
}
#[doc = "Field `HW4` reader - Port n Pin Hardware Select Bit 4"]
pub type Hw4R = crate::FieldReader<Hw4>;
impl Hw4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Hw4> {
        match self.bits {
            0 => Some(Hw4::Const00),
            1 => Some(Hw4::Const01),
            2 => Some(Hw4::Const10),
            _ => None,
        }
    }
    #[doc = "Software control only."]
    #[inline(always)]
    pub fn is_const_00(&self) -> bool {
        *self == Hw4::Const00
    }
    #[doc = "HWI0/HWO0 control path can override the software configuration."]
    #[inline(always)]
    pub fn is_const_01(&self) -> bool {
        *self == Hw4::Const01
    }
    #[doc = "HWI1/HWO1 control path can override the software configuration."]
    #[inline(always)]
    pub fn is_const_10(&self) -> bool {
        *self == Hw4::Const10
    }
}
#[doc = "Field `HW4` writer - Port n Pin Hardware Select Bit 4"]
pub type Hw4W<'a, REG> = crate::FieldWriter<'a, REG, 2, Hw4>;
impl<'a, REG> Hw4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Software control only."]
    #[inline(always)]
    pub fn const_00(self) -> &'a mut crate::W<REG> {
        self.variant(Hw4::Const00)
    }
    #[doc = "HWI0/HWO0 control path can override the software configuration."]
    #[inline(always)]
    pub fn const_01(self) -> &'a mut crate::W<REG> {
        self.variant(Hw4::Const01)
    }
    #[doc = "HWI1/HWO1 control path can override the software configuration."]
    #[inline(always)]
    pub fn const_10(self) -> &'a mut crate::W<REG> {
        self.variant(Hw4::Const10)
    }
}
#[doc = "Port n Pin Hardware Select Bit 5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Hw5 {
    #[doc = "0: Software control only."]
    Const00 = 0,
    #[doc = "1: HWI0/HWO0 control path can override the software configuration."]
    Const01 = 1,
    #[doc = "2: HWI1/HWO1 control path can override the software configuration."]
    Const10 = 2,
}
impl From<Hw5> for u8 {
    #[inline(always)]
    fn from(variant: Hw5) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Hw5 {
    type Ux = u8;
}
#[doc = "Field `HW5` reader - Port n Pin Hardware Select Bit 5"]
pub type Hw5R = crate::FieldReader<Hw5>;
impl Hw5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Hw5> {
        match self.bits {
            0 => Some(Hw5::Const00),
            1 => Some(Hw5::Const01),
            2 => Some(Hw5::Const10),
            _ => None,
        }
    }
    #[doc = "Software control only."]
    #[inline(always)]
    pub fn is_const_00(&self) -> bool {
        *self == Hw5::Const00
    }
    #[doc = "HWI0/HWO0 control path can override the software configuration."]
    #[inline(always)]
    pub fn is_const_01(&self) -> bool {
        *self == Hw5::Const01
    }
    #[doc = "HWI1/HWO1 control path can override the software configuration."]
    #[inline(always)]
    pub fn is_const_10(&self) -> bool {
        *self == Hw5::Const10
    }
}
#[doc = "Field `HW5` writer - Port n Pin Hardware Select Bit 5"]
pub type Hw5W<'a, REG> = crate::FieldWriter<'a, REG, 2, Hw5>;
impl<'a, REG> Hw5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Software control only."]
    #[inline(always)]
    pub fn const_00(self) -> &'a mut crate::W<REG> {
        self.variant(Hw5::Const00)
    }
    #[doc = "HWI0/HWO0 control path can override the software configuration."]
    #[inline(always)]
    pub fn const_01(self) -> &'a mut crate::W<REG> {
        self.variant(Hw5::Const01)
    }
    #[doc = "HWI1/HWO1 control path can override the software configuration."]
    #[inline(always)]
    pub fn const_10(self) -> &'a mut crate::W<REG> {
        self.variant(Hw5::Const10)
    }
}
#[doc = "Port n Pin Hardware Select Bit 6\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Hw6 {
    #[doc = "0: Software control only."]
    Const00 = 0,
    #[doc = "1: HWI0/HWO0 control path can override the software configuration."]
    Const01 = 1,
    #[doc = "2: HWI1/HWO1 control path can override the software configuration."]
    Const10 = 2,
}
impl From<Hw6> for u8 {
    #[inline(always)]
    fn from(variant: Hw6) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Hw6 {
    type Ux = u8;
}
#[doc = "Field `HW6` reader - Port n Pin Hardware Select Bit 6"]
pub type Hw6R = crate::FieldReader<Hw6>;
impl Hw6R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Hw6> {
        match self.bits {
            0 => Some(Hw6::Const00),
            1 => Some(Hw6::Const01),
            2 => Some(Hw6::Const10),
            _ => None,
        }
    }
    #[doc = "Software control only."]
    #[inline(always)]
    pub fn is_const_00(&self) -> bool {
        *self == Hw6::Const00
    }
    #[doc = "HWI0/HWO0 control path can override the software configuration."]
    #[inline(always)]
    pub fn is_const_01(&self) -> bool {
        *self == Hw6::Const01
    }
    #[doc = "HWI1/HWO1 control path can override the software configuration."]
    #[inline(always)]
    pub fn is_const_10(&self) -> bool {
        *self == Hw6::Const10
    }
}
#[doc = "Field `HW6` writer - Port n Pin Hardware Select Bit 6"]
pub type Hw6W<'a, REG> = crate::FieldWriter<'a, REG, 2, Hw6>;
impl<'a, REG> Hw6W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Software control only."]
    #[inline(always)]
    pub fn const_00(self) -> &'a mut crate::W<REG> {
        self.variant(Hw6::Const00)
    }
    #[doc = "HWI0/HWO0 control path can override the software configuration."]
    #[inline(always)]
    pub fn const_01(self) -> &'a mut crate::W<REG> {
        self.variant(Hw6::Const01)
    }
    #[doc = "HWI1/HWO1 control path can override the software configuration."]
    #[inline(always)]
    pub fn const_10(self) -> &'a mut crate::W<REG> {
        self.variant(Hw6::Const10)
    }
}
#[doc = "Port n Pin Hardware Select Bit 7\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Hw7 {
    #[doc = "0: Software control only."]
    Const00 = 0,
    #[doc = "1: HWI0/HWO0 control path can override the software configuration."]
    Const01 = 1,
    #[doc = "2: HWI1/HWO1 control path can override the software configuration."]
    Const10 = 2,
}
impl From<Hw7> for u8 {
    #[inline(always)]
    fn from(variant: Hw7) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Hw7 {
    type Ux = u8;
}
#[doc = "Field `HW7` reader - Port n Pin Hardware Select Bit 7"]
pub type Hw7R = crate::FieldReader<Hw7>;
impl Hw7R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Hw7> {
        match self.bits {
            0 => Some(Hw7::Const00),
            1 => Some(Hw7::Const01),
            2 => Some(Hw7::Const10),
            _ => None,
        }
    }
    #[doc = "Software control only."]
    #[inline(always)]
    pub fn is_const_00(&self) -> bool {
        *self == Hw7::Const00
    }
    #[doc = "HWI0/HWO0 control path can override the software configuration."]
    #[inline(always)]
    pub fn is_const_01(&self) -> bool {
        *self == Hw7::Const01
    }
    #[doc = "HWI1/HWO1 control path can override the software configuration."]
    #[inline(always)]
    pub fn is_const_10(&self) -> bool {
        *self == Hw7::Const10
    }
}
#[doc = "Field `HW7` writer - Port n Pin Hardware Select Bit 7"]
pub type Hw7W<'a, REG> = crate::FieldWriter<'a, REG, 2, Hw7>;
impl<'a, REG> Hw7W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Software control only."]
    #[inline(always)]
    pub fn const_00(self) -> &'a mut crate::W<REG> {
        self.variant(Hw7::Const00)
    }
    #[doc = "HWI0/HWO0 control path can override the software configuration."]
    #[inline(always)]
    pub fn const_01(self) -> &'a mut crate::W<REG> {
        self.variant(Hw7::Const01)
    }
    #[doc = "HWI1/HWO1 control path can override the software configuration."]
    #[inline(always)]
    pub fn const_10(self) -> &'a mut crate::W<REG> {
        self.variant(Hw7::Const10)
    }
}
#[doc = "Port n Pin Hardware Select Bit 8\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Hw8 {
    #[doc = "0: Software control only."]
    Const00 = 0,
    #[doc = "1: HWI0/HWO0 control path can override the software configuration."]
    Const01 = 1,
    #[doc = "2: HWI1/HWO1 control path can override the software configuration."]
    Const10 = 2,
}
impl From<Hw8> for u8 {
    #[inline(always)]
    fn from(variant: Hw8) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Hw8 {
    type Ux = u8;
}
#[doc = "Field `HW8` reader - Port n Pin Hardware Select Bit 8"]
pub type Hw8R = crate::FieldReader<Hw8>;
impl Hw8R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Hw8> {
        match self.bits {
            0 => Some(Hw8::Const00),
            1 => Some(Hw8::Const01),
            2 => Some(Hw8::Const10),
            _ => None,
        }
    }
    #[doc = "Software control only."]
    #[inline(always)]
    pub fn is_const_00(&self) -> bool {
        *self == Hw8::Const00
    }
    #[doc = "HWI0/HWO0 control path can override the software configuration."]
    #[inline(always)]
    pub fn is_const_01(&self) -> bool {
        *self == Hw8::Const01
    }
    #[doc = "HWI1/HWO1 control path can override the software configuration."]
    #[inline(always)]
    pub fn is_const_10(&self) -> bool {
        *self == Hw8::Const10
    }
}
#[doc = "Field `HW8` writer - Port n Pin Hardware Select Bit 8"]
pub type Hw8W<'a, REG> = crate::FieldWriter<'a, REG, 2, Hw8>;
impl<'a, REG> Hw8W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Software control only."]
    #[inline(always)]
    pub fn const_00(self) -> &'a mut crate::W<REG> {
        self.variant(Hw8::Const00)
    }
    #[doc = "HWI0/HWO0 control path can override the software configuration."]
    #[inline(always)]
    pub fn const_01(self) -> &'a mut crate::W<REG> {
        self.variant(Hw8::Const01)
    }
    #[doc = "HWI1/HWO1 control path can override the software configuration."]
    #[inline(always)]
    pub fn const_10(self) -> &'a mut crate::W<REG> {
        self.variant(Hw8::Const10)
    }
}
#[doc = "Port n Pin Hardware Select Bit 9\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Hw9 {
    #[doc = "0: Software control only."]
    Const00 = 0,
    #[doc = "1: HWI0/HWO0 control path can override the software configuration."]
    Const01 = 1,
    #[doc = "2: HWI1/HWO1 control path can override the software configuration."]
    Const10 = 2,
}
impl From<Hw9> for u8 {
    #[inline(always)]
    fn from(variant: Hw9) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Hw9 {
    type Ux = u8;
}
#[doc = "Field `HW9` reader - Port n Pin Hardware Select Bit 9"]
pub type Hw9R = crate::FieldReader<Hw9>;
impl Hw9R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Hw9> {
        match self.bits {
            0 => Some(Hw9::Const00),
            1 => Some(Hw9::Const01),
            2 => Some(Hw9::Const10),
            _ => None,
        }
    }
    #[doc = "Software control only."]
    #[inline(always)]
    pub fn is_const_00(&self) -> bool {
        *self == Hw9::Const00
    }
    #[doc = "HWI0/HWO0 control path can override the software configuration."]
    #[inline(always)]
    pub fn is_const_01(&self) -> bool {
        *self == Hw9::Const01
    }
    #[doc = "HWI1/HWO1 control path can override the software configuration."]
    #[inline(always)]
    pub fn is_const_10(&self) -> bool {
        *self == Hw9::Const10
    }
}
#[doc = "Field `HW9` writer - Port n Pin Hardware Select Bit 9"]
pub type Hw9W<'a, REG> = crate::FieldWriter<'a, REG, 2, Hw9>;
impl<'a, REG> Hw9W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Software control only."]
    #[inline(always)]
    pub fn const_00(self) -> &'a mut crate::W<REG> {
        self.variant(Hw9::Const00)
    }
    #[doc = "HWI0/HWO0 control path can override the software configuration."]
    #[inline(always)]
    pub fn const_01(self) -> &'a mut crate::W<REG> {
        self.variant(Hw9::Const01)
    }
    #[doc = "HWI1/HWO1 control path can override the software configuration."]
    #[inline(always)]
    pub fn const_10(self) -> &'a mut crate::W<REG> {
        self.variant(Hw9::Const10)
    }
}
#[doc = "Port n Pin Hardware Select Bit 10\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Hw10 {
    #[doc = "0: Software control only."]
    Const00 = 0,
    #[doc = "1: HWI0/HWO0 control path can override the software configuration."]
    Const01 = 1,
    #[doc = "2: HWI1/HWO1 control path can override the software configuration."]
    Const10 = 2,
}
impl From<Hw10> for u8 {
    #[inline(always)]
    fn from(variant: Hw10) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Hw10 {
    type Ux = u8;
}
#[doc = "Field `HW10` reader - Port n Pin Hardware Select Bit 10"]
pub type Hw10R = crate::FieldReader<Hw10>;
impl Hw10R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Hw10> {
        match self.bits {
            0 => Some(Hw10::Const00),
            1 => Some(Hw10::Const01),
            2 => Some(Hw10::Const10),
            _ => None,
        }
    }
    #[doc = "Software control only."]
    #[inline(always)]
    pub fn is_const_00(&self) -> bool {
        *self == Hw10::Const00
    }
    #[doc = "HWI0/HWO0 control path can override the software configuration."]
    #[inline(always)]
    pub fn is_const_01(&self) -> bool {
        *self == Hw10::Const01
    }
    #[doc = "HWI1/HWO1 control path can override the software configuration."]
    #[inline(always)]
    pub fn is_const_10(&self) -> bool {
        *self == Hw10::Const10
    }
}
#[doc = "Field `HW10` writer - Port n Pin Hardware Select Bit 10"]
pub type Hw10W<'a, REG> = crate::FieldWriter<'a, REG, 2, Hw10>;
impl<'a, REG> Hw10W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Software control only."]
    #[inline(always)]
    pub fn const_00(self) -> &'a mut crate::W<REG> {
        self.variant(Hw10::Const00)
    }
    #[doc = "HWI0/HWO0 control path can override the software configuration."]
    #[inline(always)]
    pub fn const_01(self) -> &'a mut crate::W<REG> {
        self.variant(Hw10::Const01)
    }
    #[doc = "HWI1/HWO1 control path can override the software configuration."]
    #[inline(always)]
    pub fn const_10(self) -> &'a mut crate::W<REG> {
        self.variant(Hw10::Const10)
    }
}
#[doc = "Port n Pin Hardware Select Bit 11\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Hw11 {
    #[doc = "0: Software control only."]
    Const00 = 0,
    #[doc = "1: HWI0/HWO0 control path can override the software configuration."]
    Const01 = 1,
    #[doc = "2: HWI1/HWO1 control path can override the software configuration."]
    Const10 = 2,
}
impl From<Hw11> for u8 {
    #[inline(always)]
    fn from(variant: Hw11) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Hw11 {
    type Ux = u8;
}
#[doc = "Field `HW11` reader - Port n Pin Hardware Select Bit 11"]
pub type Hw11R = crate::FieldReader<Hw11>;
impl Hw11R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Hw11> {
        match self.bits {
            0 => Some(Hw11::Const00),
            1 => Some(Hw11::Const01),
            2 => Some(Hw11::Const10),
            _ => None,
        }
    }
    #[doc = "Software control only."]
    #[inline(always)]
    pub fn is_const_00(&self) -> bool {
        *self == Hw11::Const00
    }
    #[doc = "HWI0/HWO0 control path can override the software configuration."]
    #[inline(always)]
    pub fn is_const_01(&self) -> bool {
        *self == Hw11::Const01
    }
    #[doc = "HWI1/HWO1 control path can override the software configuration."]
    #[inline(always)]
    pub fn is_const_10(&self) -> bool {
        *self == Hw11::Const10
    }
}
#[doc = "Field `HW11` writer - Port n Pin Hardware Select Bit 11"]
pub type Hw11W<'a, REG> = crate::FieldWriter<'a, REG, 2, Hw11>;
impl<'a, REG> Hw11W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Software control only."]
    #[inline(always)]
    pub fn const_00(self) -> &'a mut crate::W<REG> {
        self.variant(Hw11::Const00)
    }
    #[doc = "HWI0/HWO0 control path can override the software configuration."]
    #[inline(always)]
    pub fn const_01(self) -> &'a mut crate::W<REG> {
        self.variant(Hw11::Const01)
    }
    #[doc = "HWI1/HWO1 control path can override the software configuration."]
    #[inline(always)]
    pub fn const_10(self) -> &'a mut crate::W<REG> {
        self.variant(Hw11::Const10)
    }
}
#[doc = "Port n Pin Hardware Select Bit 12\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Hw12 {
    #[doc = "0: Software control only."]
    Const00 = 0,
    #[doc = "1: HWI0/HWO0 control path can override the software configuration."]
    Const01 = 1,
    #[doc = "2: HWI1/HWO1 control path can override the software configuration."]
    Const10 = 2,
}
impl From<Hw12> for u8 {
    #[inline(always)]
    fn from(variant: Hw12) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Hw12 {
    type Ux = u8;
}
#[doc = "Field `HW12` reader - Port n Pin Hardware Select Bit 12"]
pub type Hw12R = crate::FieldReader<Hw12>;
impl Hw12R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Hw12> {
        match self.bits {
            0 => Some(Hw12::Const00),
            1 => Some(Hw12::Const01),
            2 => Some(Hw12::Const10),
            _ => None,
        }
    }
    #[doc = "Software control only."]
    #[inline(always)]
    pub fn is_const_00(&self) -> bool {
        *self == Hw12::Const00
    }
    #[doc = "HWI0/HWO0 control path can override the software configuration."]
    #[inline(always)]
    pub fn is_const_01(&self) -> bool {
        *self == Hw12::Const01
    }
    #[doc = "HWI1/HWO1 control path can override the software configuration."]
    #[inline(always)]
    pub fn is_const_10(&self) -> bool {
        *self == Hw12::Const10
    }
}
#[doc = "Field `HW12` writer - Port n Pin Hardware Select Bit 12"]
pub type Hw12W<'a, REG> = crate::FieldWriter<'a, REG, 2, Hw12>;
impl<'a, REG> Hw12W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Software control only."]
    #[inline(always)]
    pub fn const_00(self) -> &'a mut crate::W<REG> {
        self.variant(Hw12::Const00)
    }
    #[doc = "HWI0/HWO0 control path can override the software configuration."]
    #[inline(always)]
    pub fn const_01(self) -> &'a mut crate::W<REG> {
        self.variant(Hw12::Const01)
    }
    #[doc = "HWI1/HWO1 control path can override the software configuration."]
    #[inline(always)]
    pub fn const_10(self) -> &'a mut crate::W<REG> {
        self.variant(Hw12::Const10)
    }
}
#[doc = "Port n Pin Hardware Select Bit 13\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Hw13 {
    #[doc = "0: Software control only."]
    Const00 = 0,
    #[doc = "1: HWI0/HWO0 control path can override the software configuration."]
    Const01 = 1,
    #[doc = "2: HWI1/HWO1 control path can override the software configuration."]
    Const10 = 2,
}
impl From<Hw13> for u8 {
    #[inline(always)]
    fn from(variant: Hw13) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Hw13 {
    type Ux = u8;
}
#[doc = "Field `HW13` reader - Port n Pin Hardware Select Bit 13"]
pub type Hw13R = crate::FieldReader<Hw13>;
impl Hw13R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Hw13> {
        match self.bits {
            0 => Some(Hw13::Const00),
            1 => Some(Hw13::Const01),
            2 => Some(Hw13::Const10),
            _ => None,
        }
    }
    #[doc = "Software control only."]
    #[inline(always)]
    pub fn is_const_00(&self) -> bool {
        *self == Hw13::Const00
    }
    #[doc = "HWI0/HWO0 control path can override the software configuration."]
    #[inline(always)]
    pub fn is_const_01(&self) -> bool {
        *self == Hw13::Const01
    }
    #[doc = "HWI1/HWO1 control path can override the software configuration."]
    #[inline(always)]
    pub fn is_const_10(&self) -> bool {
        *self == Hw13::Const10
    }
}
#[doc = "Field `HW13` writer - Port n Pin Hardware Select Bit 13"]
pub type Hw13W<'a, REG> = crate::FieldWriter<'a, REG, 2, Hw13>;
impl<'a, REG> Hw13W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Software control only."]
    #[inline(always)]
    pub fn const_00(self) -> &'a mut crate::W<REG> {
        self.variant(Hw13::Const00)
    }
    #[doc = "HWI0/HWO0 control path can override the software configuration."]
    #[inline(always)]
    pub fn const_01(self) -> &'a mut crate::W<REG> {
        self.variant(Hw13::Const01)
    }
    #[doc = "HWI1/HWO1 control path can override the software configuration."]
    #[inline(always)]
    pub fn const_10(self) -> &'a mut crate::W<REG> {
        self.variant(Hw13::Const10)
    }
}
#[doc = "Port n Pin Hardware Select Bit 14\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Hw14 {
    #[doc = "0: Software control only."]
    Const00 = 0,
    #[doc = "1: HWI0/HWO0 control path can override the software configuration."]
    Const01 = 1,
    #[doc = "2: HWI1/HWO1 control path can override the software configuration."]
    Const10 = 2,
}
impl From<Hw14> for u8 {
    #[inline(always)]
    fn from(variant: Hw14) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Hw14 {
    type Ux = u8;
}
#[doc = "Field `HW14` reader - Port n Pin Hardware Select Bit 14"]
pub type Hw14R = crate::FieldReader<Hw14>;
impl Hw14R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Hw14> {
        match self.bits {
            0 => Some(Hw14::Const00),
            1 => Some(Hw14::Const01),
            2 => Some(Hw14::Const10),
            _ => None,
        }
    }
    #[doc = "Software control only."]
    #[inline(always)]
    pub fn is_const_00(&self) -> bool {
        *self == Hw14::Const00
    }
    #[doc = "HWI0/HWO0 control path can override the software configuration."]
    #[inline(always)]
    pub fn is_const_01(&self) -> bool {
        *self == Hw14::Const01
    }
    #[doc = "HWI1/HWO1 control path can override the software configuration."]
    #[inline(always)]
    pub fn is_const_10(&self) -> bool {
        *self == Hw14::Const10
    }
}
#[doc = "Field `HW14` writer - Port n Pin Hardware Select Bit 14"]
pub type Hw14W<'a, REG> = crate::FieldWriter<'a, REG, 2, Hw14>;
impl<'a, REG> Hw14W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Software control only."]
    #[inline(always)]
    pub fn const_00(self) -> &'a mut crate::W<REG> {
        self.variant(Hw14::Const00)
    }
    #[doc = "HWI0/HWO0 control path can override the software configuration."]
    #[inline(always)]
    pub fn const_01(self) -> &'a mut crate::W<REG> {
        self.variant(Hw14::Const01)
    }
    #[doc = "HWI1/HWO1 control path can override the software configuration."]
    #[inline(always)]
    pub fn const_10(self) -> &'a mut crate::W<REG> {
        self.variant(Hw14::Const10)
    }
}
#[doc = "Port n Pin Hardware Select Bit 15\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Hw15 {
    #[doc = "0: Software control only."]
    Const00 = 0,
    #[doc = "1: HWI0/HWO0 control path can override the software configuration."]
    Const01 = 1,
    #[doc = "2: HWI1/HWO1 control path can override the software configuration."]
    Const10 = 2,
}
impl From<Hw15> for u8 {
    #[inline(always)]
    fn from(variant: Hw15) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Hw15 {
    type Ux = u8;
}
#[doc = "Field `HW15` reader - Port n Pin Hardware Select Bit 15"]
pub type Hw15R = crate::FieldReader<Hw15>;
impl Hw15R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Hw15> {
        match self.bits {
            0 => Some(Hw15::Const00),
            1 => Some(Hw15::Const01),
            2 => Some(Hw15::Const10),
            _ => None,
        }
    }
    #[doc = "Software control only."]
    #[inline(always)]
    pub fn is_const_00(&self) -> bool {
        *self == Hw15::Const00
    }
    #[doc = "HWI0/HWO0 control path can override the software configuration."]
    #[inline(always)]
    pub fn is_const_01(&self) -> bool {
        *self == Hw15::Const01
    }
    #[doc = "HWI1/HWO1 control path can override the software configuration."]
    #[inline(always)]
    pub fn is_const_10(&self) -> bool {
        *self == Hw15::Const10
    }
}
#[doc = "Field `HW15` writer - Port n Pin Hardware Select Bit 15"]
pub type Hw15W<'a, REG> = crate::FieldWriter<'a, REG, 2, Hw15>;
impl<'a, REG> Hw15W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Software control only."]
    #[inline(always)]
    pub fn const_00(self) -> &'a mut crate::W<REG> {
        self.variant(Hw15::Const00)
    }
    #[doc = "HWI0/HWO0 control path can override the software configuration."]
    #[inline(always)]
    pub fn const_01(self) -> &'a mut crate::W<REG> {
        self.variant(Hw15::Const01)
    }
    #[doc = "HWI1/HWO1 control path can override the software configuration."]
    #[inline(always)]
    pub fn const_10(self) -> &'a mut crate::W<REG> {
        self.variant(Hw15::Const10)
    }
}
impl R {
    #[doc = "Bits 0:1 - Port n Pin Hardware Select Bit 0"]
    #[inline(always)]
    pub fn hw0(&self) -> Hw0R {
        Hw0R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Port n Pin Hardware Select Bit 1"]
    #[inline(always)]
    pub fn hw1(&self) -> Hw1R {
        Hw1R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Port n Pin Hardware Select Bit 2"]
    #[inline(always)]
    pub fn hw2(&self) -> Hw2R {
        Hw2R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Port n Pin Hardware Select Bit 3"]
    #[inline(always)]
    pub fn hw3(&self) -> Hw3R {
        Hw3R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Port n Pin Hardware Select Bit 4"]
    #[inline(always)]
    pub fn hw4(&self) -> Hw4R {
        Hw4R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Port n Pin Hardware Select Bit 5"]
    #[inline(always)]
    pub fn hw5(&self) -> Hw5R {
        Hw5R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Port n Pin Hardware Select Bit 6"]
    #[inline(always)]
    pub fn hw6(&self) -> Hw6R {
        Hw6R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Port n Pin Hardware Select Bit 7"]
    #[inline(always)]
    pub fn hw7(&self) -> Hw7R {
        Hw7R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - Port n Pin Hardware Select Bit 8"]
    #[inline(always)]
    pub fn hw8(&self) -> Hw8R {
        Hw8R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - Port n Pin Hardware Select Bit 9"]
    #[inline(always)]
    pub fn hw9(&self) -> Hw9R {
        Hw9R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - Port n Pin Hardware Select Bit 10"]
    #[inline(always)]
    pub fn hw10(&self) -> Hw10R {
        Hw10R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - Port n Pin Hardware Select Bit 11"]
    #[inline(always)]
    pub fn hw11(&self) -> Hw11R {
        Hw11R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:25 - Port n Pin Hardware Select Bit 12"]
    #[inline(always)]
    pub fn hw12(&self) -> Hw12R {
        Hw12R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27 - Port n Pin Hardware Select Bit 13"]
    #[inline(always)]
    pub fn hw13(&self) -> Hw13R {
        Hw13R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:29 - Port n Pin Hardware Select Bit 14"]
    #[inline(always)]
    pub fn hw14(&self) -> Hw14R {
        Hw14R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31 - Port n Pin Hardware Select Bit 15"]
    #[inline(always)]
    pub fn hw15(&self) -> Hw15R {
        Hw15R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Port n Pin Hardware Select Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn hw0(&mut self) -> Hw0W<HwselSpec> {
        Hw0W::new(self, 0)
    }
    #[doc = "Bits 2:3 - Port n Pin Hardware Select Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn hw1(&mut self) -> Hw1W<HwselSpec> {
        Hw1W::new(self, 2)
    }
    #[doc = "Bits 4:5 - Port n Pin Hardware Select Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn hw2(&mut self) -> Hw2W<HwselSpec> {
        Hw2W::new(self, 4)
    }
    #[doc = "Bits 6:7 - Port n Pin Hardware Select Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn hw3(&mut self) -> Hw3W<HwselSpec> {
        Hw3W::new(self, 6)
    }
    #[doc = "Bits 8:9 - Port n Pin Hardware Select Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn hw4(&mut self) -> Hw4W<HwselSpec> {
        Hw4W::new(self, 8)
    }
    #[doc = "Bits 10:11 - Port n Pin Hardware Select Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn hw5(&mut self) -> Hw5W<HwselSpec> {
        Hw5W::new(self, 10)
    }
    #[doc = "Bits 12:13 - Port n Pin Hardware Select Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn hw6(&mut self) -> Hw6W<HwselSpec> {
        Hw6W::new(self, 12)
    }
    #[doc = "Bits 14:15 - Port n Pin Hardware Select Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn hw7(&mut self) -> Hw7W<HwselSpec> {
        Hw7W::new(self, 14)
    }
    #[doc = "Bits 16:17 - Port n Pin Hardware Select Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn hw8(&mut self) -> Hw8W<HwselSpec> {
        Hw8W::new(self, 16)
    }
    #[doc = "Bits 18:19 - Port n Pin Hardware Select Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn hw9(&mut self) -> Hw9W<HwselSpec> {
        Hw9W::new(self, 18)
    }
    #[doc = "Bits 20:21 - Port n Pin Hardware Select Bit 10"]
    #[inline(always)]
    #[must_use]
    pub fn hw10(&mut self) -> Hw10W<HwselSpec> {
        Hw10W::new(self, 20)
    }
    #[doc = "Bits 22:23 - Port n Pin Hardware Select Bit 11"]
    #[inline(always)]
    #[must_use]
    pub fn hw11(&mut self) -> Hw11W<HwselSpec> {
        Hw11W::new(self, 22)
    }
    #[doc = "Bits 24:25 - Port n Pin Hardware Select Bit 12"]
    #[inline(always)]
    #[must_use]
    pub fn hw12(&mut self) -> Hw12W<HwselSpec> {
        Hw12W::new(self, 24)
    }
    #[doc = "Bits 26:27 - Port n Pin Hardware Select Bit 13"]
    #[inline(always)]
    #[must_use]
    pub fn hw13(&mut self) -> Hw13W<HwselSpec> {
        Hw13W::new(self, 26)
    }
    #[doc = "Bits 28:29 - Port n Pin Hardware Select Bit 14"]
    #[inline(always)]
    #[must_use]
    pub fn hw14(&mut self) -> Hw14W<HwselSpec> {
        Hw14W::new(self, 28)
    }
    #[doc = "Bits 30:31 - Port n Pin Hardware Select Bit 15"]
    #[inline(always)]
    #[must_use]
    pub fn hw15(&mut self) -> Hw15W<HwselSpec> {
        Hw15W::new(self, 30)
    }
}
#[doc = "Port 3 Pin Hardware Select Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hwsel::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hwsel::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HwselSpec;
impl crate::RegisterSpec for HwselSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hwsel::R`](R) reader structure"]
impl crate::Readable for HwselSpec {}
#[doc = "`write(|w| ..)` method takes [`hwsel::W`](W) writer structure"]
impl crate::Writable for HwselSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HWSEL to value 0"]
impl crate::Resettable for HwselSpec {
    const RESET_VALUE: u32 = 0;
}
