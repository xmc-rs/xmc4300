#[doc = "Register `INS` reader"]
pub type R = crate::R<InsSpec>;
#[doc = "Register `INS` writer"]
pub type W = crate::W<InsSpec>;
#[doc = "Event 0 signal selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ev0is {
    #[doc = "0: CCU8x.INyA"]
    Value1 = 0,
    #[doc = "1: CCU8x.INyB"]
    Value2 = 1,
    #[doc = "2: CCU8x.INyC"]
    Value3 = 2,
    #[doc = "3: CCU8x.INyD"]
    Value4 = 3,
    #[doc = "4: CCU8x.INyE"]
    Value5 = 4,
    #[doc = "5: CCU8x.INyF"]
    Value6 = 5,
    #[doc = "6: CCU8x.INyG"]
    Value7 = 6,
    #[doc = "7: CCU8x.INyH"]
    Value8 = 7,
    #[doc = "8: CCU8x.INyI"]
    Value9 = 8,
    #[doc = "9: CCU8x.INyJ"]
    Value10 = 9,
    #[doc = "10: CCU8x.INyK"]
    Value11 = 10,
    #[doc = "11: CCU8x.INyL"]
    Value12 = 11,
    #[doc = "12: CCU8x.INyM"]
    Value13 = 12,
    #[doc = "13: CCU8x.INyN"]
    Value14 = 13,
    #[doc = "14: CCU8x.INyO"]
    Value15 = 14,
    #[doc = "15: CCU8x.INyP"]
    Value16 = 15,
}
impl From<Ev0is> for u8 {
    #[inline(always)]
    fn from(variant: Ev0is) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ev0is {
    type Ux = u8;
}
#[doc = "Field `EV0IS` reader - Event 0 signal selection"]
pub type Ev0isR = crate::FieldReader<Ev0is>;
impl Ev0isR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ev0is {
        match self.bits {
            0 => Ev0is::Value1,
            1 => Ev0is::Value2,
            2 => Ev0is::Value3,
            3 => Ev0is::Value4,
            4 => Ev0is::Value5,
            5 => Ev0is::Value6,
            6 => Ev0is::Value7,
            7 => Ev0is::Value8,
            8 => Ev0is::Value9,
            9 => Ev0is::Value10,
            10 => Ev0is::Value11,
            11 => Ev0is::Value12,
            12 => Ev0is::Value13,
            13 => Ev0is::Value14,
            14 => Ev0is::Value15,
            15 => Ev0is::Value16,
            _ => unreachable!(),
        }
    }
    #[doc = "CCU8x.INyA"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Ev0is::Value1
    }
    #[doc = "CCU8x.INyB"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Ev0is::Value2
    }
    #[doc = "CCU8x.INyC"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == Ev0is::Value3
    }
    #[doc = "CCU8x.INyD"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == Ev0is::Value4
    }
    #[doc = "CCU8x.INyE"]
    #[inline(always)]
    pub fn is_value5(&self) -> bool {
        *self == Ev0is::Value5
    }
    #[doc = "CCU8x.INyF"]
    #[inline(always)]
    pub fn is_value6(&self) -> bool {
        *self == Ev0is::Value6
    }
    #[doc = "CCU8x.INyG"]
    #[inline(always)]
    pub fn is_value7(&self) -> bool {
        *self == Ev0is::Value7
    }
    #[doc = "CCU8x.INyH"]
    #[inline(always)]
    pub fn is_value8(&self) -> bool {
        *self == Ev0is::Value8
    }
    #[doc = "CCU8x.INyI"]
    #[inline(always)]
    pub fn is_value9(&self) -> bool {
        *self == Ev0is::Value9
    }
    #[doc = "CCU8x.INyJ"]
    #[inline(always)]
    pub fn is_value10(&self) -> bool {
        *self == Ev0is::Value10
    }
    #[doc = "CCU8x.INyK"]
    #[inline(always)]
    pub fn is_value11(&self) -> bool {
        *self == Ev0is::Value11
    }
    #[doc = "CCU8x.INyL"]
    #[inline(always)]
    pub fn is_value12(&self) -> bool {
        *self == Ev0is::Value12
    }
    #[doc = "CCU8x.INyM"]
    #[inline(always)]
    pub fn is_value13(&self) -> bool {
        *self == Ev0is::Value13
    }
    #[doc = "CCU8x.INyN"]
    #[inline(always)]
    pub fn is_value14(&self) -> bool {
        *self == Ev0is::Value14
    }
    #[doc = "CCU8x.INyO"]
    #[inline(always)]
    pub fn is_value15(&self) -> bool {
        *self == Ev0is::Value15
    }
    #[doc = "CCU8x.INyP"]
    #[inline(always)]
    pub fn is_value16(&self) -> bool {
        *self == Ev0is::Value16
    }
}
#[doc = "Field `EV0IS` writer - Event 0 signal selection"]
pub type Ev0isW<'a, REG> = crate::FieldWriterSafe<'a, REG, 4, Ev0is>;
impl<'a, REG> Ev0isW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CCU8x.INyA"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Ev0is::Value1)
    }
    #[doc = "CCU8x.INyB"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Ev0is::Value2)
    }
    #[doc = "CCU8x.INyC"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(Ev0is::Value3)
    }
    #[doc = "CCU8x.INyD"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(Ev0is::Value4)
    }
    #[doc = "CCU8x.INyE"]
    #[inline(always)]
    pub fn value5(self) -> &'a mut crate::W<REG> {
        self.variant(Ev0is::Value5)
    }
    #[doc = "CCU8x.INyF"]
    #[inline(always)]
    pub fn value6(self) -> &'a mut crate::W<REG> {
        self.variant(Ev0is::Value6)
    }
    #[doc = "CCU8x.INyG"]
    #[inline(always)]
    pub fn value7(self) -> &'a mut crate::W<REG> {
        self.variant(Ev0is::Value7)
    }
    #[doc = "CCU8x.INyH"]
    #[inline(always)]
    pub fn value8(self) -> &'a mut crate::W<REG> {
        self.variant(Ev0is::Value8)
    }
    #[doc = "CCU8x.INyI"]
    #[inline(always)]
    pub fn value9(self) -> &'a mut crate::W<REG> {
        self.variant(Ev0is::Value9)
    }
    #[doc = "CCU8x.INyJ"]
    #[inline(always)]
    pub fn value10(self) -> &'a mut crate::W<REG> {
        self.variant(Ev0is::Value10)
    }
    #[doc = "CCU8x.INyK"]
    #[inline(always)]
    pub fn value11(self) -> &'a mut crate::W<REG> {
        self.variant(Ev0is::Value11)
    }
    #[doc = "CCU8x.INyL"]
    #[inline(always)]
    pub fn value12(self) -> &'a mut crate::W<REG> {
        self.variant(Ev0is::Value12)
    }
    #[doc = "CCU8x.INyM"]
    #[inline(always)]
    pub fn value13(self) -> &'a mut crate::W<REG> {
        self.variant(Ev0is::Value13)
    }
    #[doc = "CCU8x.INyN"]
    #[inline(always)]
    pub fn value14(self) -> &'a mut crate::W<REG> {
        self.variant(Ev0is::Value14)
    }
    #[doc = "CCU8x.INyO"]
    #[inline(always)]
    pub fn value15(self) -> &'a mut crate::W<REG> {
        self.variant(Ev0is::Value15)
    }
    #[doc = "CCU8x.INyP"]
    #[inline(always)]
    pub fn value16(self) -> &'a mut crate::W<REG> {
        self.variant(Ev0is::Value16)
    }
}
#[doc = "Event 1 signal selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ev1is {
    #[doc = "0: CCU8x.INyA"]
    Value1 = 0,
    #[doc = "1: CCU8x.INyB"]
    Value2 = 1,
    #[doc = "2: CCU8x.INyC"]
    Value3 = 2,
    #[doc = "3: CCU8x.INyD"]
    Value4 = 3,
    #[doc = "4: CCU8x.INyE"]
    Value5 = 4,
    #[doc = "5: CCU8x.INyF"]
    Value6 = 5,
    #[doc = "6: CCU8x.INyG"]
    Value7 = 6,
    #[doc = "7: CCU8x.INyH"]
    Value8 = 7,
    #[doc = "8: CCU8x.INyI"]
    Value9 = 8,
    #[doc = "9: CCU8x.INyJ"]
    Value10 = 9,
    #[doc = "10: CCU8x.INyK"]
    Value11 = 10,
    #[doc = "11: CCU8x.INyL"]
    Value12 = 11,
    #[doc = "12: CCU8x.INyM"]
    Value13 = 12,
    #[doc = "13: CCU8x.INyN"]
    Value14 = 13,
    #[doc = "14: CCU8x.INyO"]
    Value15 = 14,
    #[doc = "15: CCU8x.INyP"]
    Value16 = 15,
}
impl From<Ev1is> for u8 {
    #[inline(always)]
    fn from(variant: Ev1is) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ev1is {
    type Ux = u8;
}
#[doc = "Field `EV1IS` reader - Event 1 signal selection"]
pub type Ev1isR = crate::FieldReader<Ev1is>;
impl Ev1isR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ev1is {
        match self.bits {
            0 => Ev1is::Value1,
            1 => Ev1is::Value2,
            2 => Ev1is::Value3,
            3 => Ev1is::Value4,
            4 => Ev1is::Value5,
            5 => Ev1is::Value6,
            6 => Ev1is::Value7,
            7 => Ev1is::Value8,
            8 => Ev1is::Value9,
            9 => Ev1is::Value10,
            10 => Ev1is::Value11,
            11 => Ev1is::Value12,
            12 => Ev1is::Value13,
            13 => Ev1is::Value14,
            14 => Ev1is::Value15,
            15 => Ev1is::Value16,
            _ => unreachable!(),
        }
    }
    #[doc = "CCU8x.INyA"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Ev1is::Value1
    }
    #[doc = "CCU8x.INyB"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Ev1is::Value2
    }
    #[doc = "CCU8x.INyC"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == Ev1is::Value3
    }
    #[doc = "CCU8x.INyD"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == Ev1is::Value4
    }
    #[doc = "CCU8x.INyE"]
    #[inline(always)]
    pub fn is_value5(&self) -> bool {
        *self == Ev1is::Value5
    }
    #[doc = "CCU8x.INyF"]
    #[inline(always)]
    pub fn is_value6(&self) -> bool {
        *self == Ev1is::Value6
    }
    #[doc = "CCU8x.INyG"]
    #[inline(always)]
    pub fn is_value7(&self) -> bool {
        *self == Ev1is::Value7
    }
    #[doc = "CCU8x.INyH"]
    #[inline(always)]
    pub fn is_value8(&self) -> bool {
        *self == Ev1is::Value8
    }
    #[doc = "CCU8x.INyI"]
    #[inline(always)]
    pub fn is_value9(&self) -> bool {
        *self == Ev1is::Value9
    }
    #[doc = "CCU8x.INyJ"]
    #[inline(always)]
    pub fn is_value10(&self) -> bool {
        *self == Ev1is::Value10
    }
    #[doc = "CCU8x.INyK"]
    #[inline(always)]
    pub fn is_value11(&self) -> bool {
        *self == Ev1is::Value11
    }
    #[doc = "CCU8x.INyL"]
    #[inline(always)]
    pub fn is_value12(&self) -> bool {
        *self == Ev1is::Value12
    }
    #[doc = "CCU8x.INyM"]
    #[inline(always)]
    pub fn is_value13(&self) -> bool {
        *self == Ev1is::Value13
    }
    #[doc = "CCU8x.INyN"]
    #[inline(always)]
    pub fn is_value14(&self) -> bool {
        *self == Ev1is::Value14
    }
    #[doc = "CCU8x.INyO"]
    #[inline(always)]
    pub fn is_value15(&self) -> bool {
        *self == Ev1is::Value15
    }
    #[doc = "CCU8x.INyP"]
    #[inline(always)]
    pub fn is_value16(&self) -> bool {
        *self == Ev1is::Value16
    }
}
#[doc = "Field `EV1IS` writer - Event 1 signal selection"]
pub type Ev1isW<'a, REG> = crate::FieldWriterSafe<'a, REG, 4, Ev1is>;
impl<'a, REG> Ev1isW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CCU8x.INyA"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Ev1is::Value1)
    }
    #[doc = "CCU8x.INyB"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Ev1is::Value2)
    }
    #[doc = "CCU8x.INyC"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(Ev1is::Value3)
    }
    #[doc = "CCU8x.INyD"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(Ev1is::Value4)
    }
    #[doc = "CCU8x.INyE"]
    #[inline(always)]
    pub fn value5(self) -> &'a mut crate::W<REG> {
        self.variant(Ev1is::Value5)
    }
    #[doc = "CCU8x.INyF"]
    #[inline(always)]
    pub fn value6(self) -> &'a mut crate::W<REG> {
        self.variant(Ev1is::Value6)
    }
    #[doc = "CCU8x.INyG"]
    #[inline(always)]
    pub fn value7(self) -> &'a mut crate::W<REG> {
        self.variant(Ev1is::Value7)
    }
    #[doc = "CCU8x.INyH"]
    #[inline(always)]
    pub fn value8(self) -> &'a mut crate::W<REG> {
        self.variant(Ev1is::Value8)
    }
    #[doc = "CCU8x.INyI"]
    #[inline(always)]
    pub fn value9(self) -> &'a mut crate::W<REG> {
        self.variant(Ev1is::Value9)
    }
    #[doc = "CCU8x.INyJ"]
    #[inline(always)]
    pub fn value10(self) -> &'a mut crate::W<REG> {
        self.variant(Ev1is::Value10)
    }
    #[doc = "CCU8x.INyK"]
    #[inline(always)]
    pub fn value11(self) -> &'a mut crate::W<REG> {
        self.variant(Ev1is::Value11)
    }
    #[doc = "CCU8x.INyL"]
    #[inline(always)]
    pub fn value12(self) -> &'a mut crate::W<REG> {
        self.variant(Ev1is::Value12)
    }
    #[doc = "CCU8x.INyM"]
    #[inline(always)]
    pub fn value13(self) -> &'a mut crate::W<REG> {
        self.variant(Ev1is::Value13)
    }
    #[doc = "CCU8x.INyN"]
    #[inline(always)]
    pub fn value14(self) -> &'a mut crate::W<REG> {
        self.variant(Ev1is::Value14)
    }
    #[doc = "CCU8x.INyO"]
    #[inline(always)]
    pub fn value15(self) -> &'a mut crate::W<REG> {
        self.variant(Ev1is::Value15)
    }
    #[doc = "CCU8x.INyP"]
    #[inline(always)]
    pub fn value16(self) -> &'a mut crate::W<REG> {
        self.variant(Ev1is::Value16)
    }
}
#[doc = "Event 2 signal selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ev2is {
    #[doc = "0: CCU8x.INyA"]
    Value1 = 0,
    #[doc = "1: CCU8x.INyB"]
    Value2 = 1,
    #[doc = "2: CCU8x.INyC"]
    Value3 = 2,
    #[doc = "3: CCU8x.INyD"]
    Value4 = 3,
    #[doc = "4: CCU8x.INyE"]
    Value5 = 4,
    #[doc = "5: CCU8x.INyF"]
    Value6 = 5,
    #[doc = "6: CCU8x.INyG"]
    Value7 = 6,
    #[doc = "7: CCU8x.INyH"]
    Value8 = 7,
    #[doc = "8: CCU8x.INyI"]
    Value9 = 8,
    #[doc = "9: CCU8x.INyJ"]
    Value10 = 9,
    #[doc = "10: CCU8x.INyK"]
    Value11 = 10,
    #[doc = "11: CCU8x.INyL"]
    Value12 = 11,
    #[doc = "12: CCU8x.INyM"]
    Value13 = 12,
    #[doc = "13: CCU8x.INyN"]
    Value14 = 13,
    #[doc = "14: CCU8x.INyO"]
    Value15 = 14,
    #[doc = "15: CCU8x.INyP"]
    Value16 = 15,
}
impl From<Ev2is> for u8 {
    #[inline(always)]
    fn from(variant: Ev2is) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ev2is {
    type Ux = u8;
}
#[doc = "Field `EV2IS` reader - Event 2 signal selection"]
pub type Ev2isR = crate::FieldReader<Ev2is>;
impl Ev2isR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ev2is {
        match self.bits {
            0 => Ev2is::Value1,
            1 => Ev2is::Value2,
            2 => Ev2is::Value3,
            3 => Ev2is::Value4,
            4 => Ev2is::Value5,
            5 => Ev2is::Value6,
            6 => Ev2is::Value7,
            7 => Ev2is::Value8,
            8 => Ev2is::Value9,
            9 => Ev2is::Value10,
            10 => Ev2is::Value11,
            11 => Ev2is::Value12,
            12 => Ev2is::Value13,
            13 => Ev2is::Value14,
            14 => Ev2is::Value15,
            15 => Ev2is::Value16,
            _ => unreachable!(),
        }
    }
    #[doc = "CCU8x.INyA"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Ev2is::Value1
    }
    #[doc = "CCU8x.INyB"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Ev2is::Value2
    }
    #[doc = "CCU8x.INyC"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == Ev2is::Value3
    }
    #[doc = "CCU8x.INyD"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == Ev2is::Value4
    }
    #[doc = "CCU8x.INyE"]
    #[inline(always)]
    pub fn is_value5(&self) -> bool {
        *self == Ev2is::Value5
    }
    #[doc = "CCU8x.INyF"]
    #[inline(always)]
    pub fn is_value6(&self) -> bool {
        *self == Ev2is::Value6
    }
    #[doc = "CCU8x.INyG"]
    #[inline(always)]
    pub fn is_value7(&self) -> bool {
        *self == Ev2is::Value7
    }
    #[doc = "CCU8x.INyH"]
    #[inline(always)]
    pub fn is_value8(&self) -> bool {
        *self == Ev2is::Value8
    }
    #[doc = "CCU8x.INyI"]
    #[inline(always)]
    pub fn is_value9(&self) -> bool {
        *self == Ev2is::Value9
    }
    #[doc = "CCU8x.INyJ"]
    #[inline(always)]
    pub fn is_value10(&self) -> bool {
        *self == Ev2is::Value10
    }
    #[doc = "CCU8x.INyK"]
    #[inline(always)]
    pub fn is_value11(&self) -> bool {
        *self == Ev2is::Value11
    }
    #[doc = "CCU8x.INyL"]
    #[inline(always)]
    pub fn is_value12(&self) -> bool {
        *self == Ev2is::Value12
    }
    #[doc = "CCU8x.INyM"]
    #[inline(always)]
    pub fn is_value13(&self) -> bool {
        *self == Ev2is::Value13
    }
    #[doc = "CCU8x.INyN"]
    #[inline(always)]
    pub fn is_value14(&self) -> bool {
        *self == Ev2is::Value14
    }
    #[doc = "CCU8x.INyO"]
    #[inline(always)]
    pub fn is_value15(&self) -> bool {
        *self == Ev2is::Value15
    }
    #[doc = "CCU8x.INyP"]
    #[inline(always)]
    pub fn is_value16(&self) -> bool {
        *self == Ev2is::Value16
    }
}
#[doc = "Field `EV2IS` writer - Event 2 signal selection"]
pub type Ev2isW<'a, REG> = crate::FieldWriterSafe<'a, REG, 4, Ev2is>;
impl<'a, REG> Ev2isW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CCU8x.INyA"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Ev2is::Value1)
    }
    #[doc = "CCU8x.INyB"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Ev2is::Value2)
    }
    #[doc = "CCU8x.INyC"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(Ev2is::Value3)
    }
    #[doc = "CCU8x.INyD"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(Ev2is::Value4)
    }
    #[doc = "CCU8x.INyE"]
    #[inline(always)]
    pub fn value5(self) -> &'a mut crate::W<REG> {
        self.variant(Ev2is::Value5)
    }
    #[doc = "CCU8x.INyF"]
    #[inline(always)]
    pub fn value6(self) -> &'a mut crate::W<REG> {
        self.variant(Ev2is::Value6)
    }
    #[doc = "CCU8x.INyG"]
    #[inline(always)]
    pub fn value7(self) -> &'a mut crate::W<REG> {
        self.variant(Ev2is::Value7)
    }
    #[doc = "CCU8x.INyH"]
    #[inline(always)]
    pub fn value8(self) -> &'a mut crate::W<REG> {
        self.variant(Ev2is::Value8)
    }
    #[doc = "CCU8x.INyI"]
    #[inline(always)]
    pub fn value9(self) -> &'a mut crate::W<REG> {
        self.variant(Ev2is::Value9)
    }
    #[doc = "CCU8x.INyJ"]
    #[inline(always)]
    pub fn value10(self) -> &'a mut crate::W<REG> {
        self.variant(Ev2is::Value10)
    }
    #[doc = "CCU8x.INyK"]
    #[inline(always)]
    pub fn value11(self) -> &'a mut crate::W<REG> {
        self.variant(Ev2is::Value11)
    }
    #[doc = "CCU8x.INyL"]
    #[inline(always)]
    pub fn value12(self) -> &'a mut crate::W<REG> {
        self.variant(Ev2is::Value12)
    }
    #[doc = "CCU8x.INyM"]
    #[inline(always)]
    pub fn value13(self) -> &'a mut crate::W<REG> {
        self.variant(Ev2is::Value13)
    }
    #[doc = "CCU8x.INyN"]
    #[inline(always)]
    pub fn value14(self) -> &'a mut crate::W<REG> {
        self.variant(Ev2is::Value14)
    }
    #[doc = "CCU8x.INyO"]
    #[inline(always)]
    pub fn value15(self) -> &'a mut crate::W<REG> {
        self.variant(Ev2is::Value15)
    }
    #[doc = "CCU8x.INyP"]
    #[inline(always)]
    pub fn value16(self) -> &'a mut crate::W<REG> {
        self.variant(Ev2is::Value16)
    }
}
#[doc = "Event 0 Edge Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ev0em {
    #[doc = "0: No action"]
    Value1 = 0,
    #[doc = "1: Signal active on rising edge"]
    Value2 = 1,
    #[doc = "2: Signal active on falling edge"]
    Value3 = 2,
    #[doc = "3: Signal active on both edges"]
    Value4 = 3,
}
impl From<Ev0em> for u8 {
    #[inline(always)]
    fn from(variant: Ev0em) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ev0em {
    type Ux = u8;
}
#[doc = "Field `EV0EM` reader - Event 0 Edge Selection"]
pub type Ev0emR = crate::FieldReader<Ev0em>;
impl Ev0emR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ev0em {
        match self.bits {
            0 => Ev0em::Value1,
            1 => Ev0em::Value2,
            2 => Ev0em::Value3,
            3 => Ev0em::Value4,
            _ => unreachable!(),
        }
    }
    #[doc = "No action"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Ev0em::Value1
    }
    #[doc = "Signal active on rising edge"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Ev0em::Value2
    }
    #[doc = "Signal active on falling edge"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == Ev0em::Value3
    }
    #[doc = "Signal active on both edges"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == Ev0em::Value4
    }
}
#[doc = "Field `EV0EM` writer - Event 0 Edge Selection"]
pub type Ev0emW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Ev0em>;
impl<'a, REG> Ev0emW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No action"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Ev0em::Value1)
    }
    #[doc = "Signal active on rising edge"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Ev0em::Value2)
    }
    #[doc = "Signal active on falling edge"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(Ev0em::Value3)
    }
    #[doc = "Signal active on both edges"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(Ev0em::Value4)
    }
}
#[doc = "Event 1 Edge Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ev1em {
    #[doc = "0: No action"]
    Value1 = 0,
    #[doc = "1: Signal active on rising edge"]
    Value2 = 1,
    #[doc = "2: Signal active on falling edge"]
    Value3 = 2,
    #[doc = "3: Signal active on both edges"]
    Value4 = 3,
}
impl From<Ev1em> for u8 {
    #[inline(always)]
    fn from(variant: Ev1em) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ev1em {
    type Ux = u8;
}
#[doc = "Field `EV1EM` reader - Event 1 Edge Selection"]
pub type Ev1emR = crate::FieldReader<Ev1em>;
impl Ev1emR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ev1em {
        match self.bits {
            0 => Ev1em::Value1,
            1 => Ev1em::Value2,
            2 => Ev1em::Value3,
            3 => Ev1em::Value4,
            _ => unreachable!(),
        }
    }
    #[doc = "No action"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Ev1em::Value1
    }
    #[doc = "Signal active on rising edge"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Ev1em::Value2
    }
    #[doc = "Signal active on falling edge"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == Ev1em::Value3
    }
    #[doc = "Signal active on both edges"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == Ev1em::Value4
    }
}
#[doc = "Field `EV1EM` writer - Event 1 Edge Selection"]
pub type Ev1emW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Ev1em>;
impl<'a, REG> Ev1emW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No action"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Ev1em::Value1)
    }
    #[doc = "Signal active on rising edge"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Ev1em::Value2)
    }
    #[doc = "Signal active on falling edge"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(Ev1em::Value3)
    }
    #[doc = "Signal active on both edges"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(Ev1em::Value4)
    }
}
#[doc = "Event 2 Edge Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ev2em {
    #[doc = "0: No action"]
    Value1 = 0,
    #[doc = "1: Signal active on rising edge"]
    Value2 = 1,
    #[doc = "2: Signal active on falling edge"]
    Value3 = 2,
    #[doc = "3: Signal active on both edges"]
    Value4 = 3,
}
impl From<Ev2em> for u8 {
    #[inline(always)]
    fn from(variant: Ev2em) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ev2em {
    type Ux = u8;
}
#[doc = "Field `EV2EM` reader - Event 2 Edge Selection"]
pub type Ev2emR = crate::FieldReader<Ev2em>;
impl Ev2emR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ev2em {
        match self.bits {
            0 => Ev2em::Value1,
            1 => Ev2em::Value2,
            2 => Ev2em::Value3,
            3 => Ev2em::Value4,
            _ => unreachable!(),
        }
    }
    #[doc = "No action"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Ev2em::Value1
    }
    #[doc = "Signal active on rising edge"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Ev2em::Value2
    }
    #[doc = "Signal active on falling edge"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == Ev2em::Value3
    }
    #[doc = "Signal active on both edges"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == Ev2em::Value4
    }
}
#[doc = "Field `EV2EM` writer - Event 2 Edge Selection"]
pub type Ev2emW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Ev2em>;
impl<'a, REG> Ev2emW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No action"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Ev2em::Value1)
    }
    #[doc = "Signal active on rising edge"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Ev2em::Value2)
    }
    #[doc = "Signal active on falling edge"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(Ev2em::Value3)
    }
    #[doc = "Signal active on both edges"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(Ev2em::Value4)
    }
}
#[doc = "Event 0 Level Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ev0lm {
    #[doc = "0: Active on HIGH level"]
    Value1 = 0,
    #[doc = "1: Active on LOW level"]
    Value2 = 1,
}
impl From<Ev0lm> for bool {
    #[inline(always)]
    fn from(variant: Ev0lm) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EV0LM` reader - Event 0 Level Selection"]
pub type Ev0lmR = crate::BitReader<Ev0lm>;
impl Ev0lmR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ev0lm {
        match self.bits {
            false => Ev0lm::Value1,
            true => Ev0lm::Value2,
        }
    }
    #[doc = "Active on HIGH level"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Ev0lm::Value1
    }
    #[doc = "Active on LOW level"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Ev0lm::Value2
    }
}
#[doc = "Field `EV0LM` writer - Event 0 Level Selection"]
pub type Ev0lmW<'a, REG> = crate::BitWriter<'a, REG, Ev0lm>;
impl<'a, REG> Ev0lmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Active on HIGH level"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Ev0lm::Value1)
    }
    #[doc = "Active on LOW level"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Ev0lm::Value2)
    }
}
#[doc = "Event 1 Level Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ev1lm {
    #[doc = "0: Active on HIGH level"]
    Value1 = 0,
    #[doc = "1: Active on LOW level"]
    Value2 = 1,
}
impl From<Ev1lm> for bool {
    #[inline(always)]
    fn from(variant: Ev1lm) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EV1LM` reader - Event 1 Level Selection"]
pub type Ev1lmR = crate::BitReader<Ev1lm>;
impl Ev1lmR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ev1lm {
        match self.bits {
            false => Ev1lm::Value1,
            true => Ev1lm::Value2,
        }
    }
    #[doc = "Active on HIGH level"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Ev1lm::Value1
    }
    #[doc = "Active on LOW level"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Ev1lm::Value2
    }
}
#[doc = "Field `EV1LM` writer - Event 1 Level Selection"]
pub type Ev1lmW<'a, REG> = crate::BitWriter<'a, REG, Ev1lm>;
impl<'a, REG> Ev1lmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Active on HIGH level"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Ev1lm::Value1)
    }
    #[doc = "Active on LOW level"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Ev1lm::Value2)
    }
}
#[doc = "Event 2 Level Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ev2lm {
    #[doc = "0: Active on HIGH level"]
    Value1 = 0,
    #[doc = "1: Active on LOW level"]
    Value2 = 1,
}
impl From<Ev2lm> for bool {
    #[inline(always)]
    fn from(variant: Ev2lm) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EV2LM` reader - Event 2 Level Selection"]
pub type Ev2lmR = crate::BitReader<Ev2lm>;
impl Ev2lmR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ev2lm {
        match self.bits {
            false => Ev2lm::Value1,
            true => Ev2lm::Value2,
        }
    }
    #[doc = "Active on HIGH level"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Ev2lm::Value1
    }
    #[doc = "Active on LOW level"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Ev2lm::Value2
    }
}
#[doc = "Field `EV2LM` writer - Event 2 Level Selection"]
pub type Ev2lmW<'a, REG> = crate::BitWriter<'a, REG, Ev2lm>;
impl<'a, REG> Ev2lmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Active on HIGH level"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Ev2lm::Value1)
    }
    #[doc = "Active on LOW level"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Ev2lm::Value2)
    }
}
#[doc = "Event 0 Low Pass Filter Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Lpf0m {
    #[doc = "0: LPF is disabled"]
    Value1 = 0,
    #[doc = "1: 3 clock cycles of fCCU8"]
    Value2 = 1,
    #[doc = "2: 5 clock cycles of fCCU8"]
    Value3 = 2,
    #[doc = "3: 7 clock cycles of fCCU8"]
    Value4 = 3,
}
impl From<Lpf0m> for u8 {
    #[inline(always)]
    fn from(variant: Lpf0m) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Lpf0m {
    type Ux = u8;
}
#[doc = "Field `LPF0M` reader - Event 0 Low Pass Filter Configuration"]
pub type Lpf0mR = crate::FieldReader<Lpf0m>;
impl Lpf0mR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lpf0m {
        match self.bits {
            0 => Lpf0m::Value1,
            1 => Lpf0m::Value2,
            2 => Lpf0m::Value3,
            3 => Lpf0m::Value4,
            _ => unreachable!(),
        }
    }
    #[doc = "LPF is disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Lpf0m::Value1
    }
    #[doc = "3 clock cycles of fCCU8"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Lpf0m::Value2
    }
    #[doc = "5 clock cycles of fCCU8"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == Lpf0m::Value3
    }
    #[doc = "7 clock cycles of fCCU8"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == Lpf0m::Value4
    }
}
#[doc = "Field `LPF0M` writer - Event 0 Low Pass Filter Configuration"]
pub type Lpf0mW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Lpf0m>;
impl<'a, REG> Lpf0mW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "LPF is disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Lpf0m::Value1)
    }
    #[doc = "3 clock cycles of fCCU8"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Lpf0m::Value2)
    }
    #[doc = "5 clock cycles of fCCU8"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(Lpf0m::Value3)
    }
    #[doc = "7 clock cycles of fCCU8"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(Lpf0m::Value4)
    }
}
#[doc = "Event 1 Low Pass Filter Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Lpf1m {
    #[doc = "0: LPF is disabled"]
    Value1 = 0,
    #[doc = "1: 3 clock cycles of fCCU8"]
    Value2 = 1,
    #[doc = "2: 5 clock cycles of fCCU8"]
    Value3 = 2,
    #[doc = "3: 7 clock cycles of fCCU8"]
    Value4 = 3,
}
impl From<Lpf1m> for u8 {
    #[inline(always)]
    fn from(variant: Lpf1m) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Lpf1m {
    type Ux = u8;
}
#[doc = "Field `LPF1M` reader - Event 1 Low Pass Filter Configuration"]
pub type Lpf1mR = crate::FieldReader<Lpf1m>;
impl Lpf1mR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lpf1m {
        match self.bits {
            0 => Lpf1m::Value1,
            1 => Lpf1m::Value2,
            2 => Lpf1m::Value3,
            3 => Lpf1m::Value4,
            _ => unreachable!(),
        }
    }
    #[doc = "LPF is disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Lpf1m::Value1
    }
    #[doc = "3 clock cycles of fCCU8"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Lpf1m::Value2
    }
    #[doc = "5 clock cycles of fCCU8"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == Lpf1m::Value3
    }
    #[doc = "7 clock cycles of fCCU8"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == Lpf1m::Value4
    }
}
#[doc = "Field `LPF1M` writer - Event 1 Low Pass Filter Configuration"]
pub type Lpf1mW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Lpf1m>;
impl<'a, REG> Lpf1mW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "LPF is disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Lpf1m::Value1)
    }
    #[doc = "3 clock cycles of fCCU8"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Lpf1m::Value2)
    }
    #[doc = "5 clock cycles of fCCU8"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(Lpf1m::Value3)
    }
    #[doc = "7 clock cycles of fCCU8"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(Lpf1m::Value4)
    }
}
#[doc = "Event 2 Low Pass Filter Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Lpf2m {
    #[doc = "0: LPF is disabled"]
    Value1 = 0,
    #[doc = "1: 3 clock cycles of fCCU8"]
    Value2 = 1,
    #[doc = "2: 5 clock cycles of fCCU8"]
    Value3 = 2,
    #[doc = "3: 7 clock cycles of fCCU8"]
    Value4 = 3,
}
impl From<Lpf2m> for u8 {
    #[inline(always)]
    fn from(variant: Lpf2m) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Lpf2m {
    type Ux = u8;
}
#[doc = "Field `LPF2M` reader - Event 2 Low Pass Filter Configuration"]
pub type Lpf2mR = crate::FieldReader<Lpf2m>;
impl Lpf2mR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lpf2m {
        match self.bits {
            0 => Lpf2m::Value1,
            1 => Lpf2m::Value2,
            2 => Lpf2m::Value3,
            3 => Lpf2m::Value4,
            _ => unreachable!(),
        }
    }
    #[doc = "LPF is disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Lpf2m::Value1
    }
    #[doc = "3 clock cycles of fCCU8"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Lpf2m::Value2
    }
    #[doc = "5 clock cycles of fCCU8"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == Lpf2m::Value3
    }
    #[doc = "7 clock cycles of fCCU8"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == Lpf2m::Value4
    }
}
#[doc = "Field `LPF2M` writer - Event 2 Low Pass Filter Configuration"]
pub type Lpf2mW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Lpf2m>;
impl<'a, REG> Lpf2mW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "LPF is disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Lpf2m::Value1)
    }
    #[doc = "3 clock cycles of fCCU8"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Lpf2m::Value2)
    }
    #[doc = "5 clock cycles of fCCU8"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(Lpf2m::Value3)
    }
    #[doc = "7 clock cycles of fCCU8"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(Lpf2m::Value4)
    }
}
impl R {
    #[doc = "Bits 0:3 - Event 0 signal selection"]
    #[inline(always)]
    pub fn ev0is(&self) -> Ev0isR {
        Ev0isR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Event 1 signal selection"]
    #[inline(always)]
    pub fn ev1is(&self) -> Ev1isR {
        Ev1isR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Event 2 signal selection"]
    #[inline(always)]
    pub fn ev2is(&self) -> Ev2isR {
        Ev2isR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:17 - Event 0 Edge Selection"]
    #[inline(always)]
    pub fn ev0em(&self) -> Ev0emR {
        Ev0emR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - Event 1 Edge Selection"]
    #[inline(always)]
    pub fn ev1em(&self) -> Ev1emR {
        Ev1emR::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - Event 2 Edge Selection"]
    #[inline(always)]
    pub fn ev2em(&self) -> Ev2emR {
        Ev2emR::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bit 22 - Event 0 Level Selection"]
    #[inline(always)]
    pub fn ev0lm(&self) -> Ev0lmR {
        Ev0lmR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Event 1 Level Selection"]
    #[inline(always)]
    pub fn ev1lm(&self) -> Ev1lmR {
        Ev1lmR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Event 2 Level Selection"]
    #[inline(always)]
    pub fn ev2lm(&self) -> Ev2lmR {
        Ev2lmR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 25:26 - Event 0 Low Pass Filter Configuration"]
    #[inline(always)]
    pub fn lpf0m(&self) -> Lpf0mR {
        Lpf0mR::new(((self.bits >> 25) & 3) as u8)
    }
    #[doc = "Bits 27:28 - Event 1 Low Pass Filter Configuration"]
    #[inline(always)]
    pub fn lpf1m(&self) -> Lpf1mR {
        Lpf1mR::new(((self.bits >> 27) & 3) as u8)
    }
    #[doc = "Bits 29:30 - Event 2 Low Pass Filter Configuration"]
    #[inline(always)]
    pub fn lpf2m(&self) -> Lpf2mR {
        Lpf2mR::new(((self.bits >> 29) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Event 0 signal selection"]
    #[inline(always)]
    #[must_use]
    pub fn ev0is(&mut self) -> Ev0isW<InsSpec> {
        Ev0isW::new(self, 0)
    }
    #[doc = "Bits 4:7 - Event 1 signal selection"]
    #[inline(always)]
    #[must_use]
    pub fn ev1is(&mut self) -> Ev1isW<InsSpec> {
        Ev1isW::new(self, 4)
    }
    #[doc = "Bits 8:11 - Event 2 signal selection"]
    #[inline(always)]
    #[must_use]
    pub fn ev2is(&mut self) -> Ev2isW<InsSpec> {
        Ev2isW::new(self, 8)
    }
    #[doc = "Bits 16:17 - Event 0 Edge Selection"]
    #[inline(always)]
    #[must_use]
    pub fn ev0em(&mut self) -> Ev0emW<InsSpec> {
        Ev0emW::new(self, 16)
    }
    #[doc = "Bits 18:19 - Event 1 Edge Selection"]
    #[inline(always)]
    #[must_use]
    pub fn ev1em(&mut self) -> Ev1emW<InsSpec> {
        Ev1emW::new(self, 18)
    }
    #[doc = "Bits 20:21 - Event 2 Edge Selection"]
    #[inline(always)]
    #[must_use]
    pub fn ev2em(&mut self) -> Ev2emW<InsSpec> {
        Ev2emW::new(self, 20)
    }
    #[doc = "Bit 22 - Event 0 Level Selection"]
    #[inline(always)]
    #[must_use]
    pub fn ev0lm(&mut self) -> Ev0lmW<InsSpec> {
        Ev0lmW::new(self, 22)
    }
    #[doc = "Bit 23 - Event 1 Level Selection"]
    #[inline(always)]
    #[must_use]
    pub fn ev1lm(&mut self) -> Ev1lmW<InsSpec> {
        Ev1lmW::new(self, 23)
    }
    #[doc = "Bit 24 - Event 2 Level Selection"]
    #[inline(always)]
    #[must_use]
    pub fn ev2lm(&mut self) -> Ev2lmW<InsSpec> {
        Ev2lmW::new(self, 24)
    }
    #[doc = "Bits 25:26 - Event 0 Low Pass Filter Configuration"]
    #[inline(always)]
    #[must_use]
    pub fn lpf0m(&mut self) -> Lpf0mW<InsSpec> {
        Lpf0mW::new(self, 25)
    }
    #[doc = "Bits 27:28 - Event 1 Low Pass Filter Configuration"]
    #[inline(always)]
    #[must_use]
    pub fn lpf1m(&mut self) -> Lpf1mW<InsSpec> {
        Lpf1mW::new(self, 27)
    }
    #[doc = "Bits 29:30 - Event 2 Low Pass Filter Configuration"]
    #[inline(always)]
    #[must_use]
    pub fn lpf2m(&mut self) -> Lpf2mW<InsSpec> {
        Lpf2mW::new(self, 29)
    }
}
#[doc = "Input Selector Configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ins::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ins::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct InsSpec;
impl crate::RegisterSpec for InsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ins::R`](R) reader structure"]
impl crate::Readable for InsSpec {}
#[doc = "`write(|w| ..)` method takes [`ins::W`](W) writer structure"]
impl crate::Writable for InsSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INS to value 0"]
impl crate::Resettable for InsSpec {
    const RESET_VALUE: u32 = 0;
}
