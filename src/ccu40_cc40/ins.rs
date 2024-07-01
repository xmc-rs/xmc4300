#[doc = "Register `INS` reader"]
pub type R = crate::R<INS_SPEC>;
#[doc = "Register `INS` writer"]
pub type W = crate::W<INS_SPEC>;
#[doc = "Event 0 signal selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EV0IS_A {
    #[doc = "0: CCU4x.INyA"]
    VALUE1 = 0,
    #[doc = "1: CCU4x.INyB"]
    VALUE2 = 1,
    #[doc = "2: CCU4x.INyC"]
    VALUE3 = 2,
    #[doc = "3: CCU4x.INyD"]
    VALUE4 = 3,
    #[doc = "4: CCU4x.INyE"]
    VALUE5 = 4,
    #[doc = "5: CCU4x.INyF"]
    VALUE6 = 5,
    #[doc = "6: CCU4x.INyG"]
    VALUE7 = 6,
    #[doc = "7: CCU4x.INyH"]
    VALUE8 = 7,
    #[doc = "8: CCU4x.INyI"]
    VALUE9 = 8,
    #[doc = "9: CCU4x.INyJ"]
    VALUE10 = 9,
    #[doc = "10: CCU4x.INyK"]
    VALUE11 = 10,
    #[doc = "11: CCU4x.INyL"]
    VALUE12 = 11,
    #[doc = "12: CCU4x.INyM"]
    VALUE13 = 12,
    #[doc = "13: CCU4x.INyN"]
    VALUE14 = 13,
    #[doc = "14: CCU4x.INyO"]
    VALUE15 = 14,
    #[doc = "15: CCU4x.INyP"]
    VALUE16 = 15,
}
impl From<EV0IS_A> for u8 {
    #[inline(always)]
    fn from(variant: EV0IS_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for EV0IS_A {
    type Ux = u8;
}
impl crate::IsEnum for EV0IS_A {}
#[doc = "Field `EV0IS` reader - Event 0 signal selection"]
pub type EV0IS_R = crate::FieldReader<EV0IS_A>;
impl EV0IS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EV0IS_A {
        match self.bits {
            0 => EV0IS_A::VALUE1,
            1 => EV0IS_A::VALUE2,
            2 => EV0IS_A::VALUE3,
            3 => EV0IS_A::VALUE4,
            4 => EV0IS_A::VALUE5,
            5 => EV0IS_A::VALUE6,
            6 => EV0IS_A::VALUE7,
            7 => EV0IS_A::VALUE8,
            8 => EV0IS_A::VALUE9,
            9 => EV0IS_A::VALUE10,
            10 => EV0IS_A::VALUE11,
            11 => EV0IS_A::VALUE12,
            12 => EV0IS_A::VALUE13,
            13 => EV0IS_A::VALUE14,
            14 => EV0IS_A::VALUE15,
            15 => EV0IS_A::VALUE16,
            _ => unreachable!(),
        }
    }
    #[doc = "CCU4x.INyA"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == EV0IS_A::VALUE1
    }
    #[doc = "CCU4x.INyB"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == EV0IS_A::VALUE2
    }
    #[doc = "CCU4x.INyC"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == EV0IS_A::VALUE3
    }
    #[doc = "CCU4x.INyD"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == EV0IS_A::VALUE4
    }
    #[doc = "CCU4x.INyE"]
    #[inline(always)]
    pub fn is_value5(&self) -> bool {
        *self == EV0IS_A::VALUE5
    }
    #[doc = "CCU4x.INyF"]
    #[inline(always)]
    pub fn is_value6(&self) -> bool {
        *self == EV0IS_A::VALUE6
    }
    #[doc = "CCU4x.INyG"]
    #[inline(always)]
    pub fn is_value7(&self) -> bool {
        *self == EV0IS_A::VALUE7
    }
    #[doc = "CCU4x.INyH"]
    #[inline(always)]
    pub fn is_value8(&self) -> bool {
        *self == EV0IS_A::VALUE8
    }
    #[doc = "CCU4x.INyI"]
    #[inline(always)]
    pub fn is_value9(&self) -> bool {
        *self == EV0IS_A::VALUE9
    }
    #[doc = "CCU4x.INyJ"]
    #[inline(always)]
    pub fn is_value10(&self) -> bool {
        *self == EV0IS_A::VALUE10
    }
    #[doc = "CCU4x.INyK"]
    #[inline(always)]
    pub fn is_value11(&self) -> bool {
        *self == EV0IS_A::VALUE11
    }
    #[doc = "CCU4x.INyL"]
    #[inline(always)]
    pub fn is_value12(&self) -> bool {
        *self == EV0IS_A::VALUE12
    }
    #[doc = "CCU4x.INyM"]
    #[inline(always)]
    pub fn is_value13(&self) -> bool {
        *self == EV0IS_A::VALUE13
    }
    #[doc = "CCU4x.INyN"]
    #[inline(always)]
    pub fn is_value14(&self) -> bool {
        *self == EV0IS_A::VALUE14
    }
    #[doc = "CCU4x.INyO"]
    #[inline(always)]
    pub fn is_value15(&self) -> bool {
        *self == EV0IS_A::VALUE15
    }
    #[doc = "CCU4x.INyP"]
    #[inline(always)]
    pub fn is_value16(&self) -> bool {
        *self == EV0IS_A::VALUE16
    }
}
#[doc = "Field `EV0IS` writer - Event 0 signal selection"]
pub type EV0IS_W<'a, REG> = crate::FieldWriter<'a, REG, 4, EV0IS_A, crate::Safe>;
impl<'a, REG> EV0IS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CCU4x.INyA"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(EV0IS_A::VALUE1)
    }
    #[doc = "CCU4x.INyB"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(EV0IS_A::VALUE2)
    }
    #[doc = "CCU4x.INyC"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(EV0IS_A::VALUE3)
    }
    #[doc = "CCU4x.INyD"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(EV0IS_A::VALUE4)
    }
    #[doc = "CCU4x.INyE"]
    #[inline(always)]
    pub fn value5(self) -> &'a mut crate::W<REG> {
        self.variant(EV0IS_A::VALUE5)
    }
    #[doc = "CCU4x.INyF"]
    #[inline(always)]
    pub fn value6(self) -> &'a mut crate::W<REG> {
        self.variant(EV0IS_A::VALUE6)
    }
    #[doc = "CCU4x.INyG"]
    #[inline(always)]
    pub fn value7(self) -> &'a mut crate::W<REG> {
        self.variant(EV0IS_A::VALUE7)
    }
    #[doc = "CCU4x.INyH"]
    #[inline(always)]
    pub fn value8(self) -> &'a mut crate::W<REG> {
        self.variant(EV0IS_A::VALUE8)
    }
    #[doc = "CCU4x.INyI"]
    #[inline(always)]
    pub fn value9(self) -> &'a mut crate::W<REG> {
        self.variant(EV0IS_A::VALUE9)
    }
    #[doc = "CCU4x.INyJ"]
    #[inline(always)]
    pub fn value10(self) -> &'a mut crate::W<REG> {
        self.variant(EV0IS_A::VALUE10)
    }
    #[doc = "CCU4x.INyK"]
    #[inline(always)]
    pub fn value11(self) -> &'a mut crate::W<REG> {
        self.variant(EV0IS_A::VALUE11)
    }
    #[doc = "CCU4x.INyL"]
    #[inline(always)]
    pub fn value12(self) -> &'a mut crate::W<REG> {
        self.variant(EV0IS_A::VALUE12)
    }
    #[doc = "CCU4x.INyM"]
    #[inline(always)]
    pub fn value13(self) -> &'a mut crate::W<REG> {
        self.variant(EV0IS_A::VALUE13)
    }
    #[doc = "CCU4x.INyN"]
    #[inline(always)]
    pub fn value14(self) -> &'a mut crate::W<REG> {
        self.variant(EV0IS_A::VALUE14)
    }
    #[doc = "CCU4x.INyO"]
    #[inline(always)]
    pub fn value15(self) -> &'a mut crate::W<REG> {
        self.variant(EV0IS_A::VALUE15)
    }
    #[doc = "CCU4x.INyP"]
    #[inline(always)]
    pub fn value16(self) -> &'a mut crate::W<REG> {
        self.variant(EV0IS_A::VALUE16)
    }
}
#[doc = "Event 1 signal selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EV1IS_A {
    #[doc = "0: CCU4x.INyA"]
    VALUE1 = 0,
    #[doc = "1: CCU4x.INyB"]
    VALUE2 = 1,
    #[doc = "2: CCU4x.INyC"]
    VALUE3 = 2,
    #[doc = "3: CCU4x.INyD"]
    VALUE4 = 3,
    #[doc = "4: CCU4x.INyE"]
    VALUE5 = 4,
    #[doc = "5: CCU4x.INyF"]
    VALUE6 = 5,
    #[doc = "6: CCU4x.INyG"]
    VALUE7 = 6,
    #[doc = "7: CCU4x.INyH"]
    VALUE8 = 7,
    #[doc = "8: CCU4x.INyI"]
    VALUE9 = 8,
    #[doc = "9: CCU4x.INyJ"]
    VALUE10 = 9,
    #[doc = "10: CCU4x.INyK"]
    VALUE11 = 10,
    #[doc = "11: CCU4x.INyL"]
    VALUE12 = 11,
    #[doc = "12: CCU4x.INyM"]
    VALUE13 = 12,
    #[doc = "13: CCU4x.INyN"]
    VALUE14 = 13,
    #[doc = "14: CCU4x.INyO"]
    VALUE15 = 14,
    #[doc = "15: CCU4x.INyP"]
    VALUE16 = 15,
}
impl From<EV1IS_A> for u8 {
    #[inline(always)]
    fn from(variant: EV1IS_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for EV1IS_A {
    type Ux = u8;
}
impl crate::IsEnum for EV1IS_A {}
#[doc = "Field `EV1IS` reader - Event 1 signal selection"]
pub type EV1IS_R = crate::FieldReader<EV1IS_A>;
impl EV1IS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EV1IS_A {
        match self.bits {
            0 => EV1IS_A::VALUE1,
            1 => EV1IS_A::VALUE2,
            2 => EV1IS_A::VALUE3,
            3 => EV1IS_A::VALUE4,
            4 => EV1IS_A::VALUE5,
            5 => EV1IS_A::VALUE6,
            6 => EV1IS_A::VALUE7,
            7 => EV1IS_A::VALUE8,
            8 => EV1IS_A::VALUE9,
            9 => EV1IS_A::VALUE10,
            10 => EV1IS_A::VALUE11,
            11 => EV1IS_A::VALUE12,
            12 => EV1IS_A::VALUE13,
            13 => EV1IS_A::VALUE14,
            14 => EV1IS_A::VALUE15,
            15 => EV1IS_A::VALUE16,
            _ => unreachable!(),
        }
    }
    #[doc = "CCU4x.INyA"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == EV1IS_A::VALUE1
    }
    #[doc = "CCU4x.INyB"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == EV1IS_A::VALUE2
    }
    #[doc = "CCU4x.INyC"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == EV1IS_A::VALUE3
    }
    #[doc = "CCU4x.INyD"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == EV1IS_A::VALUE4
    }
    #[doc = "CCU4x.INyE"]
    #[inline(always)]
    pub fn is_value5(&self) -> bool {
        *self == EV1IS_A::VALUE5
    }
    #[doc = "CCU4x.INyF"]
    #[inline(always)]
    pub fn is_value6(&self) -> bool {
        *self == EV1IS_A::VALUE6
    }
    #[doc = "CCU4x.INyG"]
    #[inline(always)]
    pub fn is_value7(&self) -> bool {
        *self == EV1IS_A::VALUE7
    }
    #[doc = "CCU4x.INyH"]
    #[inline(always)]
    pub fn is_value8(&self) -> bool {
        *self == EV1IS_A::VALUE8
    }
    #[doc = "CCU4x.INyI"]
    #[inline(always)]
    pub fn is_value9(&self) -> bool {
        *self == EV1IS_A::VALUE9
    }
    #[doc = "CCU4x.INyJ"]
    #[inline(always)]
    pub fn is_value10(&self) -> bool {
        *self == EV1IS_A::VALUE10
    }
    #[doc = "CCU4x.INyK"]
    #[inline(always)]
    pub fn is_value11(&self) -> bool {
        *self == EV1IS_A::VALUE11
    }
    #[doc = "CCU4x.INyL"]
    #[inline(always)]
    pub fn is_value12(&self) -> bool {
        *self == EV1IS_A::VALUE12
    }
    #[doc = "CCU4x.INyM"]
    #[inline(always)]
    pub fn is_value13(&self) -> bool {
        *self == EV1IS_A::VALUE13
    }
    #[doc = "CCU4x.INyN"]
    #[inline(always)]
    pub fn is_value14(&self) -> bool {
        *self == EV1IS_A::VALUE14
    }
    #[doc = "CCU4x.INyO"]
    #[inline(always)]
    pub fn is_value15(&self) -> bool {
        *self == EV1IS_A::VALUE15
    }
    #[doc = "CCU4x.INyP"]
    #[inline(always)]
    pub fn is_value16(&self) -> bool {
        *self == EV1IS_A::VALUE16
    }
}
#[doc = "Field `EV1IS` writer - Event 1 signal selection"]
pub type EV1IS_W<'a, REG> = crate::FieldWriter<'a, REG, 4, EV1IS_A, crate::Safe>;
impl<'a, REG> EV1IS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CCU4x.INyA"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(EV1IS_A::VALUE1)
    }
    #[doc = "CCU4x.INyB"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(EV1IS_A::VALUE2)
    }
    #[doc = "CCU4x.INyC"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(EV1IS_A::VALUE3)
    }
    #[doc = "CCU4x.INyD"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(EV1IS_A::VALUE4)
    }
    #[doc = "CCU4x.INyE"]
    #[inline(always)]
    pub fn value5(self) -> &'a mut crate::W<REG> {
        self.variant(EV1IS_A::VALUE5)
    }
    #[doc = "CCU4x.INyF"]
    #[inline(always)]
    pub fn value6(self) -> &'a mut crate::W<REG> {
        self.variant(EV1IS_A::VALUE6)
    }
    #[doc = "CCU4x.INyG"]
    #[inline(always)]
    pub fn value7(self) -> &'a mut crate::W<REG> {
        self.variant(EV1IS_A::VALUE7)
    }
    #[doc = "CCU4x.INyH"]
    #[inline(always)]
    pub fn value8(self) -> &'a mut crate::W<REG> {
        self.variant(EV1IS_A::VALUE8)
    }
    #[doc = "CCU4x.INyI"]
    #[inline(always)]
    pub fn value9(self) -> &'a mut crate::W<REG> {
        self.variant(EV1IS_A::VALUE9)
    }
    #[doc = "CCU4x.INyJ"]
    #[inline(always)]
    pub fn value10(self) -> &'a mut crate::W<REG> {
        self.variant(EV1IS_A::VALUE10)
    }
    #[doc = "CCU4x.INyK"]
    #[inline(always)]
    pub fn value11(self) -> &'a mut crate::W<REG> {
        self.variant(EV1IS_A::VALUE11)
    }
    #[doc = "CCU4x.INyL"]
    #[inline(always)]
    pub fn value12(self) -> &'a mut crate::W<REG> {
        self.variant(EV1IS_A::VALUE12)
    }
    #[doc = "CCU4x.INyM"]
    #[inline(always)]
    pub fn value13(self) -> &'a mut crate::W<REG> {
        self.variant(EV1IS_A::VALUE13)
    }
    #[doc = "CCU4x.INyN"]
    #[inline(always)]
    pub fn value14(self) -> &'a mut crate::W<REG> {
        self.variant(EV1IS_A::VALUE14)
    }
    #[doc = "CCU4x.INyO"]
    #[inline(always)]
    pub fn value15(self) -> &'a mut crate::W<REG> {
        self.variant(EV1IS_A::VALUE15)
    }
    #[doc = "CCU4x.INyP"]
    #[inline(always)]
    pub fn value16(self) -> &'a mut crate::W<REG> {
        self.variant(EV1IS_A::VALUE16)
    }
}
#[doc = "Event 2 signal selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EV2IS_A {
    #[doc = "0: CCU4x.INyA"]
    VALUE1 = 0,
    #[doc = "1: CCU4x.INyB"]
    VALUE2 = 1,
    #[doc = "2: CCU4x.INyC"]
    VALUE3 = 2,
    #[doc = "3: CCU4x.INyD"]
    VALUE4 = 3,
    #[doc = "4: CCU4x.INyE"]
    VALUE5 = 4,
    #[doc = "5: CCU4x.INyF"]
    VALUE6 = 5,
    #[doc = "6: CCU4x.INyG"]
    VALUE7 = 6,
    #[doc = "7: CCU4x.INyH"]
    VALUE8 = 7,
    #[doc = "8: CCU4x.INyI"]
    VALUE9 = 8,
    #[doc = "9: CCU4x.INyJ"]
    VALUE10 = 9,
    #[doc = "10: CCU4x.INyK"]
    VALUE11 = 10,
    #[doc = "11: CCU4x.INyL"]
    VALUE12 = 11,
    #[doc = "12: CCU4x.INyM"]
    VALUE13 = 12,
    #[doc = "13: CCU4x.INyN"]
    VALUE14 = 13,
    #[doc = "14: CCU4x.INyO"]
    VALUE15 = 14,
    #[doc = "15: CCU4x.INyP"]
    VALUE16 = 15,
}
impl From<EV2IS_A> for u8 {
    #[inline(always)]
    fn from(variant: EV2IS_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for EV2IS_A {
    type Ux = u8;
}
impl crate::IsEnum for EV2IS_A {}
#[doc = "Field `EV2IS` reader - Event 2 signal selection"]
pub type EV2IS_R = crate::FieldReader<EV2IS_A>;
impl EV2IS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EV2IS_A {
        match self.bits {
            0 => EV2IS_A::VALUE1,
            1 => EV2IS_A::VALUE2,
            2 => EV2IS_A::VALUE3,
            3 => EV2IS_A::VALUE4,
            4 => EV2IS_A::VALUE5,
            5 => EV2IS_A::VALUE6,
            6 => EV2IS_A::VALUE7,
            7 => EV2IS_A::VALUE8,
            8 => EV2IS_A::VALUE9,
            9 => EV2IS_A::VALUE10,
            10 => EV2IS_A::VALUE11,
            11 => EV2IS_A::VALUE12,
            12 => EV2IS_A::VALUE13,
            13 => EV2IS_A::VALUE14,
            14 => EV2IS_A::VALUE15,
            15 => EV2IS_A::VALUE16,
            _ => unreachable!(),
        }
    }
    #[doc = "CCU4x.INyA"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == EV2IS_A::VALUE1
    }
    #[doc = "CCU4x.INyB"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == EV2IS_A::VALUE2
    }
    #[doc = "CCU4x.INyC"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == EV2IS_A::VALUE3
    }
    #[doc = "CCU4x.INyD"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == EV2IS_A::VALUE4
    }
    #[doc = "CCU4x.INyE"]
    #[inline(always)]
    pub fn is_value5(&self) -> bool {
        *self == EV2IS_A::VALUE5
    }
    #[doc = "CCU4x.INyF"]
    #[inline(always)]
    pub fn is_value6(&self) -> bool {
        *self == EV2IS_A::VALUE6
    }
    #[doc = "CCU4x.INyG"]
    #[inline(always)]
    pub fn is_value7(&self) -> bool {
        *self == EV2IS_A::VALUE7
    }
    #[doc = "CCU4x.INyH"]
    #[inline(always)]
    pub fn is_value8(&self) -> bool {
        *self == EV2IS_A::VALUE8
    }
    #[doc = "CCU4x.INyI"]
    #[inline(always)]
    pub fn is_value9(&self) -> bool {
        *self == EV2IS_A::VALUE9
    }
    #[doc = "CCU4x.INyJ"]
    #[inline(always)]
    pub fn is_value10(&self) -> bool {
        *self == EV2IS_A::VALUE10
    }
    #[doc = "CCU4x.INyK"]
    #[inline(always)]
    pub fn is_value11(&self) -> bool {
        *self == EV2IS_A::VALUE11
    }
    #[doc = "CCU4x.INyL"]
    #[inline(always)]
    pub fn is_value12(&self) -> bool {
        *self == EV2IS_A::VALUE12
    }
    #[doc = "CCU4x.INyM"]
    #[inline(always)]
    pub fn is_value13(&self) -> bool {
        *self == EV2IS_A::VALUE13
    }
    #[doc = "CCU4x.INyN"]
    #[inline(always)]
    pub fn is_value14(&self) -> bool {
        *self == EV2IS_A::VALUE14
    }
    #[doc = "CCU4x.INyO"]
    #[inline(always)]
    pub fn is_value15(&self) -> bool {
        *self == EV2IS_A::VALUE15
    }
    #[doc = "CCU4x.INyP"]
    #[inline(always)]
    pub fn is_value16(&self) -> bool {
        *self == EV2IS_A::VALUE16
    }
}
#[doc = "Field `EV2IS` writer - Event 2 signal selection"]
pub type EV2IS_W<'a, REG> = crate::FieldWriter<'a, REG, 4, EV2IS_A, crate::Safe>;
impl<'a, REG> EV2IS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CCU4x.INyA"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(EV2IS_A::VALUE1)
    }
    #[doc = "CCU4x.INyB"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(EV2IS_A::VALUE2)
    }
    #[doc = "CCU4x.INyC"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(EV2IS_A::VALUE3)
    }
    #[doc = "CCU4x.INyD"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(EV2IS_A::VALUE4)
    }
    #[doc = "CCU4x.INyE"]
    #[inline(always)]
    pub fn value5(self) -> &'a mut crate::W<REG> {
        self.variant(EV2IS_A::VALUE5)
    }
    #[doc = "CCU4x.INyF"]
    #[inline(always)]
    pub fn value6(self) -> &'a mut crate::W<REG> {
        self.variant(EV2IS_A::VALUE6)
    }
    #[doc = "CCU4x.INyG"]
    #[inline(always)]
    pub fn value7(self) -> &'a mut crate::W<REG> {
        self.variant(EV2IS_A::VALUE7)
    }
    #[doc = "CCU4x.INyH"]
    #[inline(always)]
    pub fn value8(self) -> &'a mut crate::W<REG> {
        self.variant(EV2IS_A::VALUE8)
    }
    #[doc = "CCU4x.INyI"]
    #[inline(always)]
    pub fn value9(self) -> &'a mut crate::W<REG> {
        self.variant(EV2IS_A::VALUE9)
    }
    #[doc = "CCU4x.INyJ"]
    #[inline(always)]
    pub fn value10(self) -> &'a mut crate::W<REG> {
        self.variant(EV2IS_A::VALUE10)
    }
    #[doc = "CCU4x.INyK"]
    #[inline(always)]
    pub fn value11(self) -> &'a mut crate::W<REG> {
        self.variant(EV2IS_A::VALUE11)
    }
    #[doc = "CCU4x.INyL"]
    #[inline(always)]
    pub fn value12(self) -> &'a mut crate::W<REG> {
        self.variant(EV2IS_A::VALUE12)
    }
    #[doc = "CCU4x.INyM"]
    #[inline(always)]
    pub fn value13(self) -> &'a mut crate::W<REG> {
        self.variant(EV2IS_A::VALUE13)
    }
    #[doc = "CCU4x.INyN"]
    #[inline(always)]
    pub fn value14(self) -> &'a mut crate::W<REG> {
        self.variant(EV2IS_A::VALUE14)
    }
    #[doc = "CCU4x.INyO"]
    #[inline(always)]
    pub fn value15(self) -> &'a mut crate::W<REG> {
        self.variant(EV2IS_A::VALUE15)
    }
    #[doc = "CCU4x.INyP"]
    #[inline(always)]
    pub fn value16(self) -> &'a mut crate::W<REG> {
        self.variant(EV2IS_A::VALUE16)
    }
}
#[doc = "Event 0 Edge Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EV0EM_A {
    #[doc = "0: No action"]
    VALUE1 = 0,
    #[doc = "1: Signal active on rising edge"]
    VALUE2 = 1,
    #[doc = "2: Signal active on falling edge"]
    VALUE3 = 2,
    #[doc = "3: Signal active on both edges"]
    VALUE4 = 3,
}
impl From<EV0EM_A> for u8 {
    #[inline(always)]
    fn from(variant: EV0EM_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for EV0EM_A {
    type Ux = u8;
}
impl crate::IsEnum for EV0EM_A {}
#[doc = "Field `EV0EM` reader - Event 0 Edge Selection"]
pub type EV0EM_R = crate::FieldReader<EV0EM_A>;
impl EV0EM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EV0EM_A {
        match self.bits {
            0 => EV0EM_A::VALUE1,
            1 => EV0EM_A::VALUE2,
            2 => EV0EM_A::VALUE3,
            3 => EV0EM_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "No action"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == EV0EM_A::VALUE1
    }
    #[doc = "Signal active on rising edge"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == EV0EM_A::VALUE2
    }
    #[doc = "Signal active on falling edge"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == EV0EM_A::VALUE3
    }
    #[doc = "Signal active on both edges"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == EV0EM_A::VALUE4
    }
}
#[doc = "Field `EV0EM` writer - Event 0 Edge Selection"]
pub type EV0EM_W<'a, REG> = crate::FieldWriter<'a, REG, 2, EV0EM_A, crate::Safe>;
impl<'a, REG> EV0EM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No action"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(EV0EM_A::VALUE1)
    }
    #[doc = "Signal active on rising edge"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(EV0EM_A::VALUE2)
    }
    #[doc = "Signal active on falling edge"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(EV0EM_A::VALUE3)
    }
    #[doc = "Signal active on both edges"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(EV0EM_A::VALUE4)
    }
}
#[doc = "Event 1 Edge Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EV1EM_A {
    #[doc = "0: No action"]
    VALUE1 = 0,
    #[doc = "1: Signal active on rising edge"]
    VALUE2 = 1,
    #[doc = "2: Signal active on falling edge"]
    VALUE3 = 2,
    #[doc = "3: Signal active on both edges"]
    VALUE4 = 3,
}
impl From<EV1EM_A> for u8 {
    #[inline(always)]
    fn from(variant: EV1EM_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for EV1EM_A {
    type Ux = u8;
}
impl crate::IsEnum for EV1EM_A {}
#[doc = "Field `EV1EM` reader - Event 1 Edge Selection"]
pub type EV1EM_R = crate::FieldReader<EV1EM_A>;
impl EV1EM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EV1EM_A {
        match self.bits {
            0 => EV1EM_A::VALUE1,
            1 => EV1EM_A::VALUE2,
            2 => EV1EM_A::VALUE3,
            3 => EV1EM_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "No action"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == EV1EM_A::VALUE1
    }
    #[doc = "Signal active on rising edge"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == EV1EM_A::VALUE2
    }
    #[doc = "Signal active on falling edge"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == EV1EM_A::VALUE3
    }
    #[doc = "Signal active on both edges"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == EV1EM_A::VALUE4
    }
}
#[doc = "Field `EV1EM` writer - Event 1 Edge Selection"]
pub type EV1EM_W<'a, REG> = crate::FieldWriter<'a, REG, 2, EV1EM_A, crate::Safe>;
impl<'a, REG> EV1EM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No action"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(EV1EM_A::VALUE1)
    }
    #[doc = "Signal active on rising edge"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(EV1EM_A::VALUE2)
    }
    #[doc = "Signal active on falling edge"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(EV1EM_A::VALUE3)
    }
    #[doc = "Signal active on both edges"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(EV1EM_A::VALUE4)
    }
}
#[doc = "Event 2 Edge Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EV2EM_A {
    #[doc = "0: No action"]
    VALUE1 = 0,
    #[doc = "1: Signal active on rising edge"]
    VALUE2 = 1,
    #[doc = "2: Signal active on falling edge"]
    VALUE3 = 2,
    #[doc = "3: Signal active on both edges"]
    VALUE4 = 3,
}
impl From<EV2EM_A> for u8 {
    #[inline(always)]
    fn from(variant: EV2EM_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for EV2EM_A {
    type Ux = u8;
}
impl crate::IsEnum for EV2EM_A {}
#[doc = "Field `EV2EM` reader - Event 2 Edge Selection"]
pub type EV2EM_R = crate::FieldReader<EV2EM_A>;
impl EV2EM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EV2EM_A {
        match self.bits {
            0 => EV2EM_A::VALUE1,
            1 => EV2EM_A::VALUE2,
            2 => EV2EM_A::VALUE3,
            3 => EV2EM_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "No action"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == EV2EM_A::VALUE1
    }
    #[doc = "Signal active on rising edge"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == EV2EM_A::VALUE2
    }
    #[doc = "Signal active on falling edge"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == EV2EM_A::VALUE3
    }
    #[doc = "Signal active on both edges"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == EV2EM_A::VALUE4
    }
}
#[doc = "Field `EV2EM` writer - Event 2 Edge Selection"]
pub type EV2EM_W<'a, REG> = crate::FieldWriter<'a, REG, 2, EV2EM_A, crate::Safe>;
impl<'a, REG> EV2EM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No action"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(EV2EM_A::VALUE1)
    }
    #[doc = "Signal active on rising edge"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(EV2EM_A::VALUE2)
    }
    #[doc = "Signal active on falling edge"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(EV2EM_A::VALUE3)
    }
    #[doc = "Signal active on both edges"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(EV2EM_A::VALUE4)
    }
}
#[doc = "Event 0 Level Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EV0LM_A {
    #[doc = "0: Active on HIGH level"]
    VALUE1 = 0,
    #[doc = "1: Active on LOW level"]
    VALUE2 = 1,
}
impl From<EV0LM_A> for bool {
    #[inline(always)]
    fn from(variant: EV0LM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EV0LM` reader - Event 0 Level Selection"]
pub type EV0LM_R = crate::BitReader<EV0LM_A>;
impl EV0LM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EV0LM_A {
        match self.bits {
            false => EV0LM_A::VALUE1,
            true => EV0LM_A::VALUE2,
        }
    }
    #[doc = "Active on HIGH level"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == EV0LM_A::VALUE1
    }
    #[doc = "Active on LOW level"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == EV0LM_A::VALUE2
    }
}
#[doc = "Field `EV0LM` writer - Event 0 Level Selection"]
pub type EV0LM_W<'a, REG> = crate::BitWriter<'a, REG, EV0LM_A>;
impl<'a, REG> EV0LM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Active on HIGH level"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(EV0LM_A::VALUE1)
    }
    #[doc = "Active on LOW level"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(EV0LM_A::VALUE2)
    }
}
#[doc = "Event 1 Level Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EV1LM_A {
    #[doc = "0: Active on HIGH level"]
    VALUE1 = 0,
    #[doc = "1: Active on LOW level"]
    VALUE2 = 1,
}
impl From<EV1LM_A> for bool {
    #[inline(always)]
    fn from(variant: EV1LM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EV1LM` reader - Event 1 Level Selection"]
pub type EV1LM_R = crate::BitReader<EV1LM_A>;
impl EV1LM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EV1LM_A {
        match self.bits {
            false => EV1LM_A::VALUE1,
            true => EV1LM_A::VALUE2,
        }
    }
    #[doc = "Active on HIGH level"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == EV1LM_A::VALUE1
    }
    #[doc = "Active on LOW level"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == EV1LM_A::VALUE2
    }
}
#[doc = "Field `EV1LM` writer - Event 1 Level Selection"]
pub type EV1LM_W<'a, REG> = crate::BitWriter<'a, REG, EV1LM_A>;
impl<'a, REG> EV1LM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Active on HIGH level"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(EV1LM_A::VALUE1)
    }
    #[doc = "Active on LOW level"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(EV1LM_A::VALUE2)
    }
}
#[doc = "Event 2 Level Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EV2LM_A {
    #[doc = "0: Active on HIGH level"]
    VALUE1 = 0,
    #[doc = "1: Active on LOW level"]
    VALUE2 = 1,
}
impl From<EV2LM_A> for bool {
    #[inline(always)]
    fn from(variant: EV2LM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EV2LM` reader - Event 2 Level Selection"]
pub type EV2LM_R = crate::BitReader<EV2LM_A>;
impl EV2LM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EV2LM_A {
        match self.bits {
            false => EV2LM_A::VALUE1,
            true => EV2LM_A::VALUE2,
        }
    }
    #[doc = "Active on HIGH level"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == EV2LM_A::VALUE1
    }
    #[doc = "Active on LOW level"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == EV2LM_A::VALUE2
    }
}
#[doc = "Field `EV2LM` writer - Event 2 Level Selection"]
pub type EV2LM_W<'a, REG> = crate::BitWriter<'a, REG, EV2LM_A>;
impl<'a, REG> EV2LM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Active on HIGH level"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(EV2LM_A::VALUE1)
    }
    #[doc = "Active on LOW level"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(EV2LM_A::VALUE2)
    }
}
#[doc = "Event 0 Low Pass Filter Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LPF0M_A {
    #[doc = "0: LPF is disabled"]
    VALUE1 = 0,
    #[doc = "1: 3 clock cycles of fCCU4"]
    VALUE2 = 1,
    #[doc = "2: 5 clock cycles of fCCU4"]
    VALUE3 = 2,
    #[doc = "3: 7 clock cycles of fCCU4"]
    VALUE4 = 3,
}
impl From<LPF0M_A> for u8 {
    #[inline(always)]
    fn from(variant: LPF0M_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for LPF0M_A {
    type Ux = u8;
}
impl crate::IsEnum for LPF0M_A {}
#[doc = "Field `LPF0M` reader - Event 0 Low Pass Filter Configuration"]
pub type LPF0M_R = crate::FieldReader<LPF0M_A>;
impl LPF0M_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LPF0M_A {
        match self.bits {
            0 => LPF0M_A::VALUE1,
            1 => LPF0M_A::VALUE2,
            2 => LPF0M_A::VALUE3,
            3 => LPF0M_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "LPF is disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == LPF0M_A::VALUE1
    }
    #[doc = "3 clock cycles of fCCU4"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == LPF0M_A::VALUE2
    }
    #[doc = "5 clock cycles of fCCU4"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == LPF0M_A::VALUE3
    }
    #[doc = "7 clock cycles of fCCU4"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == LPF0M_A::VALUE4
    }
}
#[doc = "Field `LPF0M` writer - Event 0 Low Pass Filter Configuration"]
pub type LPF0M_W<'a, REG> = crate::FieldWriter<'a, REG, 2, LPF0M_A, crate::Safe>;
impl<'a, REG> LPF0M_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "LPF is disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(LPF0M_A::VALUE1)
    }
    #[doc = "3 clock cycles of fCCU4"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(LPF0M_A::VALUE2)
    }
    #[doc = "5 clock cycles of fCCU4"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(LPF0M_A::VALUE3)
    }
    #[doc = "7 clock cycles of fCCU4"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(LPF0M_A::VALUE4)
    }
}
#[doc = "Event 1 Low Pass Filter Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LPF1M_A {
    #[doc = "0: LPF is disabled"]
    VALUE1 = 0,
    #[doc = "1: 3 clock cycles of fCCU4"]
    VALUE2 = 1,
    #[doc = "2: 5 clock cycles of fCCU4"]
    VALUE3 = 2,
    #[doc = "3: 7 clock cycles of fCCU4"]
    VALUE4 = 3,
}
impl From<LPF1M_A> for u8 {
    #[inline(always)]
    fn from(variant: LPF1M_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for LPF1M_A {
    type Ux = u8;
}
impl crate::IsEnum for LPF1M_A {}
#[doc = "Field `LPF1M` reader - Event 1 Low Pass Filter Configuration"]
pub type LPF1M_R = crate::FieldReader<LPF1M_A>;
impl LPF1M_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LPF1M_A {
        match self.bits {
            0 => LPF1M_A::VALUE1,
            1 => LPF1M_A::VALUE2,
            2 => LPF1M_A::VALUE3,
            3 => LPF1M_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "LPF is disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == LPF1M_A::VALUE1
    }
    #[doc = "3 clock cycles of fCCU4"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == LPF1M_A::VALUE2
    }
    #[doc = "5 clock cycles of fCCU4"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == LPF1M_A::VALUE3
    }
    #[doc = "7 clock cycles of fCCU4"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == LPF1M_A::VALUE4
    }
}
#[doc = "Field `LPF1M` writer - Event 1 Low Pass Filter Configuration"]
pub type LPF1M_W<'a, REG> = crate::FieldWriter<'a, REG, 2, LPF1M_A, crate::Safe>;
impl<'a, REG> LPF1M_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "LPF is disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(LPF1M_A::VALUE1)
    }
    #[doc = "3 clock cycles of fCCU4"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(LPF1M_A::VALUE2)
    }
    #[doc = "5 clock cycles of fCCU4"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(LPF1M_A::VALUE3)
    }
    #[doc = "7 clock cycles of fCCU4"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(LPF1M_A::VALUE4)
    }
}
#[doc = "Event 2 Low Pass Filter Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LPF2M_A {
    #[doc = "0: LPF is disabled"]
    VALUE1 = 0,
    #[doc = "1: 3 clock cycles of fCCU4"]
    VALUE2 = 1,
    #[doc = "2: 5 clock cycles of fCCU4"]
    VALUE3 = 2,
    #[doc = "3: 7 clock cycles of fCCU4"]
    VALUE4 = 3,
}
impl From<LPF2M_A> for u8 {
    #[inline(always)]
    fn from(variant: LPF2M_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for LPF2M_A {
    type Ux = u8;
}
impl crate::IsEnum for LPF2M_A {}
#[doc = "Field `LPF2M` reader - Event 2 Low Pass Filter Configuration"]
pub type LPF2M_R = crate::FieldReader<LPF2M_A>;
impl LPF2M_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LPF2M_A {
        match self.bits {
            0 => LPF2M_A::VALUE1,
            1 => LPF2M_A::VALUE2,
            2 => LPF2M_A::VALUE3,
            3 => LPF2M_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "LPF is disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == LPF2M_A::VALUE1
    }
    #[doc = "3 clock cycles of fCCU4"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == LPF2M_A::VALUE2
    }
    #[doc = "5 clock cycles of fCCU4"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == LPF2M_A::VALUE3
    }
    #[doc = "7 clock cycles of fCCU4"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == LPF2M_A::VALUE4
    }
}
#[doc = "Field `LPF2M` writer - Event 2 Low Pass Filter Configuration"]
pub type LPF2M_W<'a, REG> = crate::FieldWriter<'a, REG, 2, LPF2M_A, crate::Safe>;
impl<'a, REG> LPF2M_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "LPF is disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(LPF2M_A::VALUE1)
    }
    #[doc = "3 clock cycles of fCCU4"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(LPF2M_A::VALUE2)
    }
    #[doc = "5 clock cycles of fCCU4"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(LPF2M_A::VALUE3)
    }
    #[doc = "7 clock cycles of fCCU4"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(LPF2M_A::VALUE4)
    }
}
impl R {
    #[doc = "Bits 0:3 - Event 0 signal selection"]
    #[inline(always)]
    pub fn ev0is(&self) -> EV0IS_R {
        EV0IS_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Event 1 signal selection"]
    #[inline(always)]
    pub fn ev1is(&self) -> EV1IS_R {
        EV1IS_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Event 2 signal selection"]
    #[inline(always)]
    pub fn ev2is(&self) -> EV2IS_R {
        EV2IS_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:17 - Event 0 Edge Selection"]
    #[inline(always)]
    pub fn ev0em(&self) -> EV0EM_R {
        EV0EM_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - Event 1 Edge Selection"]
    #[inline(always)]
    pub fn ev1em(&self) -> EV1EM_R {
        EV1EM_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - Event 2 Edge Selection"]
    #[inline(always)]
    pub fn ev2em(&self) -> EV2EM_R {
        EV2EM_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bit 22 - Event 0 Level Selection"]
    #[inline(always)]
    pub fn ev0lm(&self) -> EV0LM_R {
        EV0LM_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Event 1 Level Selection"]
    #[inline(always)]
    pub fn ev1lm(&self) -> EV1LM_R {
        EV1LM_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Event 2 Level Selection"]
    #[inline(always)]
    pub fn ev2lm(&self) -> EV2LM_R {
        EV2LM_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 25:26 - Event 0 Low Pass Filter Configuration"]
    #[inline(always)]
    pub fn lpf0m(&self) -> LPF0M_R {
        LPF0M_R::new(((self.bits >> 25) & 3) as u8)
    }
    #[doc = "Bits 27:28 - Event 1 Low Pass Filter Configuration"]
    #[inline(always)]
    pub fn lpf1m(&self) -> LPF1M_R {
        LPF1M_R::new(((self.bits >> 27) & 3) as u8)
    }
    #[doc = "Bits 29:30 - Event 2 Low Pass Filter Configuration"]
    #[inline(always)]
    pub fn lpf2m(&self) -> LPF2M_R {
        LPF2M_R::new(((self.bits >> 29) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Event 0 signal selection"]
    #[inline(always)]
    #[must_use]
    pub fn ev0is(&mut self) -> EV0IS_W<INS_SPEC> {
        EV0IS_W::new(self, 0)
    }
    #[doc = "Bits 4:7 - Event 1 signal selection"]
    #[inline(always)]
    #[must_use]
    pub fn ev1is(&mut self) -> EV1IS_W<INS_SPEC> {
        EV1IS_W::new(self, 4)
    }
    #[doc = "Bits 8:11 - Event 2 signal selection"]
    #[inline(always)]
    #[must_use]
    pub fn ev2is(&mut self) -> EV2IS_W<INS_SPEC> {
        EV2IS_W::new(self, 8)
    }
    #[doc = "Bits 16:17 - Event 0 Edge Selection"]
    #[inline(always)]
    #[must_use]
    pub fn ev0em(&mut self) -> EV0EM_W<INS_SPEC> {
        EV0EM_W::new(self, 16)
    }
    #[doc = "Bits 18:19 - Event 1 Edge Selection"]
    #[inline(always)]
    #[must_use]
    pub fn ev1em(&mut self) -> EV1EM_W<INS_SPEC> {
        EV1EM_W::new(self, 18)
    }
    #[doc = "Bits 20:21 - Event 2 Edge Selection"]
    #[inline(always)]
    #[must_use]
    pub fn ev2em(&mut self) -> EV2EM_W<INS_SPEC> {
        EV2EM_W::new(self, 20)
    }
    #[doc = "Bit 22 - Event 0 Level Selection"]
    #[inline(always)]
    #[must_use]
    pub fn ev0lm(&mut self) -> EV0LM_W<INS_SPEC> {
        EV0LM_W::new(self, 22)
    }
    #[doc = "Bit 23 - Event 1 Level Selection"]
    #[inline(always)]
    #[must_use]
    pub fn ev1lm(&mut self) -> EV1LM_W<INS_SPEC> {
        EV1LM_W::new(self, 23)
    }
    #[doc = "Bit 24 - Event 2 Level Selection"]
    #[inline(always)]
    #[must_use]
    pub fn ev2lm(&mut self) -> EV2LM_W<INS_SPEC> {
        EV2LM_W::new(self, 24)
    }
    #[doc = "Bits 25:26 - Event 0 Low Pass Filter Configuration"]
    #[inline(always)]
    #[must_use]
    pub fn lpf0m(&mut self) -> LPF0M_W<INS_SPEC> {
        LPF0M_W::new(self, 25)
    }
    #[doc = "Bits 27:28 - Event 1 Low Pass Filter Configuration"]
    #[inline(always)]
    #[must_use]
    pub fn lpf1m(&mut self) -> LPF1M_W<INS_SPEC> {
        LPF1M_W::new(self, 27)
    }
    #[doc = "Bits 29:30 - Event 2 Low Pass Filter Configuration"]
    #[inline(always)]
    #[must_use]
    pub fn lpf2m(&mut self) -> LPF2M_W<INS_SPEC> {
        LPF2M_W::new(self, 29)
    }
}
#[doc = "Input Selector Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`ins::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ins::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INS_SPEC;
impl crate::RegisterSpec for INS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ins::R`](R) reader structure"]
impl crate::Readable for INS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ins::W`](W) writer structure"]
impl crate::Writable for INS_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INS to value 0"]
impl crate::Resettable for INS_SPEC {
    const RESET_VALUE: u32 = 0;
}
