#[doc = "Register `SRS` reader"]
pub type R = crate::R<SrsSpec>;
#[doc = "Register `SRS` writer"]
pub type W = crate::W<SrsSpec>;
#[doc = "Period/One match Service request selector\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Posr {
    #[doc = "0: Forward to CC8ySR0"]
    Value1 = 0,
    #[doc = "1: Forward to CC8ySR1"]
    Value2 = 1,
    #[doc = "2: Forward to CC8ySR2"]
    Value3 = 2,
    #[doc = "3: Forward to CC8ySR3"]
    Value4 = 3,
}
impl From<Posr> for u8 {
    #[inline(always)]
    fn from(variant: Posr) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Posr {
    type Ux = u8;
}
impl crate::IsEnum for Posr {}
#[doc = "Field `POSR` reader - Period/One match Service request selector"]
pub type PosrR = crate::FieldReader<Posr>;
impl PosrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Posr {
        match self.bits {
            0 => Posr::Value1,
            1 => Posr::Value2,
            2 => Posr::Value3,
            3 => Posr::Value4,
            _ => unreachable!(),
        }
    }
    #[doc = "Forward to CC8ySR0"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Posr::Value1
    }
    #[doc = "Forward to CC8ySR1"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Posr::Value2
    }
    #[doc = "Forward to CC8ySR2"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == Posr::Value3
    }
    #[doc = "Forward to CC8ySR3"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == Posr::Value4
    }
}
#[doc = "Field `POSR` writer - Period/One match Service request selector"]
pub type PosrW<'a, REG> = crate::FieldWriter<'a, REG, 2, Posr, crate::Safe>;
impl<'a, REG> PosrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Forward to CC8ySR0"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Posr::Value1)
    }
    #[doc = "Forward to CC8ySR1"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Posr::Value2)
    }
    #[doc = "Forward to CC8ySR2"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(Posr::Value3)
    }
    #[doc = "Forward to CC8ySR3"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(Posr::Value4)
    }
}
#[doc = "Channel 1 Compare match Service request selector\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cm1sr {
    #[doc = "0: Forward to CC8ySR0"]
    Value1 = 0,
    #[doc = "1: Forward to CC8ySR1"]
    Value2 = 1,
    #[doc = "2: Forward to CC8ySR2"]
    Value3 = 2,
    #[doc = "3: Forward to CC8ySR3"]
    Value4 = 3,
}
impl From<Cm1sr> for u8 {
    #[inline(always)]
    fn from(variant: Cm1sr) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cm1sr {
    type Ux = u8;
}
impl crate::IsEnum for Cm1sr {}
#[doc = "Field `CM1SR` reader - Channel 1 Compare match Service request selector"]
pub type Cm1srR = crate::FieldReader<Cm1sr>;
impl Cm1srR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cm1sr {
        match self.bits {
            0 => Cm1sr::Value1,
            1 => Cm1sr::Value2,
            2 => Cm1sr::Value3,
            3 => Cm1sr::Value4,
            _ => unreachable!(),
        }
    }
    #[doc = "Forward to CC8ySR0"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Cm1sr::Value1
    }
    #[doc = "Forward to CC8ySR1"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Cm1sr::Value2
    }
    #[doc = "Forward to CC8ySR2"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == Cm1sr::Value3
    }
    #[doc = "Forward to CC8ySR3"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == Cm1sr::Value4
    }
}
#[doc = "Field `CM1SR` writer - Channel 1 Compare match Service request selector"]
pub type Cm1srW<'a, REG> = crate::FieldWriter<'a, REG, 2, Cm1sr, crate::Safe>;
impl<'a, REG> Cm1srW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Forward to CC8ySR0"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Cm1sr::Value1)
    }
    #[doc = "Forward to CC8ySR1"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Cm1sr::Value2)
    }
    #[doc = "Forward to CC8ySR2"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(Cm1sr::Value3)
    }
    #[doc = "Forward to CC8ySR3"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(Cm1sr::Value4)
    }
}
#[doc = "Channel 2 Compare match Service request selector\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cm2sr {
    #[doc = "0: Forward to CC8ySR0"]
    Value1 = 0,
    #[doc = "1: Forward to CC8ySR1"]
    Value2 = 1,
    #[doc = "2: Forward to CC8ySR2"]
    Value3 = 2,
    #[doc = "3: Forward to CC8ySR3"]
    Value4 = 3,
}
impl From<Cm2sr> for u8 {
    #[inline(always)]
    fn from(variant: Cm2sr) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cm2sr {
    type Ux = u8;
}
impl crate::IsEnum for Cm2sr {}
#[doc = "Field `CM2SR` reader - Channel 2 Compare match Service request selector"]
pub type Cm2srR = crate::FieldReader<Cm2sr>;
impl Cm2srR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cm2sr {
        match self.bits {
            0 => Cm2sr::Value1,
            1 => Cm2sr::Value2,
            2 => Cm2sr::Value3,
            3 => Cm2sr::Value4,
            _ => unreachable!(),
        }
    }
    #[doc = "Forward to CC8ySR0"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Cm2sr::Value1
    }
    #[doc = "Forward to CC8ySR1"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Cm2sr::Value2
    }
    #[doc = "Forward to CC8ySR2"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == Cm2sr::Value3
    }
    #[doc = "Forward to CC8ySR3"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == Cm2sr::Value4
    }
}
#[doc = "Field `CM2SR` writer - Channel 2 Compare match Service request selector"]
pub type Cm2srW<'a, REG> = crate::FieldWriter<'a, REG, 2, Cm2sr, crate::Safe>;
impl<'a, REG> Cm2srW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Forward to CC8ySR0"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Cm2sr::Value1)
    }
    #[doc = "Forward to CC8ySR1"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Cm2sr::Value2)
    }
    #[doc = "Forward to CC8ySR2"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(Cm2sr::Value3)
    }
    #[doc = "Forward to CC8ySR3"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(Cm2sr::Value4)
    }
}
#[doc = "Event 0 Service request selector\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum E0sr {
    #[doc = "0: Forward to CCvySR0"]
    Value1 = 0,
    #[doc = "1: Forward to CC8ySR1"]
    Value2 = 1,
    #[doc = "2: Forward to CC8ySR2"]
    Value3 = 2,
    #[doc = "3: Forward to CC8ySR3"]
    Value4 = 3,
}
impl From<E0sr> for u8 {
    #[inline(always)]
    fn from(variant: E0sr) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for E0sr {
    type Ux = u8;
}
impl crate::IsEnum for E0sr {}
#[doc = "Field `E0SR` reader - Event 0 Service request selector"]
pub type E0srR = crate::FieldReader<E0sr>;
impl E0srR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> E0sr {
        match self.bits {
            0 => E0sr::Value1,
            1 => E0sr::Value2,
            2 => E0sr::Value3,
            3 => E0sr::Value4,
            _ => unreachable!(),
        }
    }
    #[doc = "Forward to CCvySR0"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == E0sr::Value1
    }
    #[doc = "Forward to CC8ySR1"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == E0sr::Value2
    }
    #[doc = "Forward to CC8ySR2"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == E0sr::Value3
    }
    #[doc = "Forward to CC8ySR3"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == E0sr::Value4
    }
}
#[doc = "Field `E0SR` writer - Event 0 Service request selector"]
pub type E0srW<'a, REG> = crate::FieldWriter<'a, REG, 2, E0sr, crate::Safe>;
impl<'a, REG> E0srW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Forward to CCvySR0"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(E0sr::Value1)
    }
    #[doc = "Forward to CC8ySR1"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(E0sr::Value2)
    }
    #[doc = "Forward to CC8ySR2"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(E0sr::Value3)
    }
    #[doc = "Forward to CC8ySR3"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(E0sr::Value4)
    }
}
#[doc = "Event 1 Service request selector\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum E1sr {
    #[doc = "0: Forward to CC8ySR0"]
    Value1 = 0,
    #[doc = "1: Forward to CC8ySR1"]
    Value2 = 1,
    #[doc = "2: Forward to CC8ySR2"]
    Value3 = 2,
    #[doc = "3: Forward to CC8ySR3"]
    Value4 = 3,
}
impl From<E1sr> for u8 {
    #[inline(always)]
    fn from(variant: E1sr) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for E1sr {
    type Ux = u8;
}
impl crate::IsEnum for E1sr {}
#[doc = "Field `E1SR` reader - Event 1 Service request selector"]
pub type E1srR = crate::FieldReader<E1sr>;
impl E1srR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> E1sr {
        match self.bits {
            0 => E1sr::Value1,
            1 => E1sr::Value2,
            2 => E1sr::Value3,
            3 => E1sr::Value4,
            _ => unreachable!(),
        }
    }
    #[doc = "Forward to CC8ySR0"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == E1sr::Value1
    }
    #[doc = "Forward to CC8ySR1"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == E1sr::Value2
    }
    #[doc = "Forward to CC8ySR2"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == E1sr::Value3
    }
    #[doc = "Forward to CC8ySR3"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == E1sr::Value4
    }
}
#[doc = "Field `E1SR` writer - Event 1 Service request selector"]
pub type E1srW<'a, REG> = crate::FieldWriter<'a, REG, 2, E1sr, crate::Safe>;
impl<'a, REG> E1srW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Forward to CC8ySR0"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(E1sr::Value1)
    }
    #[doc = "Forward to CC8ySR1"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(E1sr::Value2)
    }
    #[doc = "Forward to CC8ySR2"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(E1sr::Value3)
    }
    #[doc = "Forward to CC8ySR3"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(E1sr::Value4)
    }
}
#[doc = "Event 2 Service request selector\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum E2sr {
    #[doc = "0: Forward to CC8ySR0"]
    Value1 = 0,
    #[doc = "1: Forward to CCvySR1"]
    Value2 = 1,
    #[doc = "2: Forward to CC8ySR2"]
    Value3 = 2,
    #[doc = "3: Forward to CC8ySR3"]
    Value4 = 3,
}
impl From<E2sr> for u8 {
    #[inline(always)]
    fn from(variant: E2sr) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for E2sr {
    type Ux = u8;
}
impl crate::IsEnum for E2sr {}
#[doc = "Field `E2SR` reader - Event 2 Service request selector"]
pub type E2srR = crate::FieldReader<E2sr>;
impl E2srR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> E2sr {
        match self.bits {
            0 => E2sr::Value1,
            1 => E2sr::Value2,
            2 => E2sr::Value3,
            3 => E2sr::Value4,
            _ => unreachable!(),
        }
    }
    #[doc = "Forward to CC8ySR0"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == E2sr::Value1
    }
    #[doc = "Forward to CCvySR1"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == E2sr::Value2
    }
    #[doc = "Forward to CC8ySR2"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == E2sr::Value3
    }
    #[doc = "Forward to CC8ySR3"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == E2sr::Value4
    }
}
#[doc = "Field `E2SR` writer - Event 2 Service request selector"]
pub type E2srW<'a, REG> = crate::FieldWriter<'a, REG, 2, E2sr, crate::Safe>;
impl<'a, REG> E2srW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Forward to CC8ySR0"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(E2sr::Value1)
    }
    #[doc = "Forward to CCvySR1"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(E2sr::Value2)
    }
    #[doc = "Forward to CC8ySR2"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(E2sr::Value3)
    }
    #[doc = "Forward to CC8ySR3"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(E2sr::Value4)
    }
}
impl R {
    #[doc = "Bits 0:1 - Period/One match Service request selector"]
    #[inline(always)]
    pub fn posr(&self) -> PosrR {
        PosrR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Channel 1 Compare match Service request selector"]
    #[inline(always)]
    pub fn cm1sr(&self) -> Cm1srR {
        Cm1srR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Channel 2 Compare match Service request selector"]
    #[inline(always)]
    pub fn cm2sr(&self) -> Cm2srR {
        Cm2srR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Event 0 Service request selector"]
    #[inline(always)]
    pub fn e0sr(&self) -> E0srR {
        E0srR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Event 1 Service request selector"]
    #[inline(always)]
    pub fn e1sr(&self) -> E1srR {
        E1srR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Event 2 Service request selector"]
    #[inline(always)]
    pub fn e2sr(&self) -> E2srR {
        E2srR::new(((self.bits >> 12) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Period/One match Service request selector"]
    #[inline(always)]
    #[must_use]
    pub fn posr(&mut self) -> PosrW<SrsSpec> {
        PosrW::new(self, 0)
    }
    #[doc = "Bits 2:3 - Channel 1 Compare match Service request selector"]
    #[inline(always)]
    #[must_use]
    pub fn cm1sr(&mut self) -> Cm1srW<SrsSpec> {
        Cm1srW::new(self, 2)
    }
    #[doc = "Bits 4:5 - Channel 2 Compare match Service request selector"]
    #[inline(always)]
    #[must_use]
    pub fn cm2sr(&mut self) -> Cm2srW<SrsSpec> {
        Cm2srW::new(self, 4)
    }
    #[doc = "Bits 8:9 - Event 0 Service request selector"]
    #[inline(always)]
    #[must_use]
    pub fn e0sr(&mut self) -> E0srW<SrsSpec> {
        E0srW::new(self, 8)
    }
    #[doc = "Bits 10:11 - Event 1 Service request selector"]
    #[inline(always)]
    #[must_use]
    pub fn e1sr(&mut self) -> E1srW<SrsSpec> {
        E1srW::new(self, 10)
    }
    #[doc = "Bits 12:13 - Event 2 Service request selector"]
    #[inline(always)]
    #[must_use]
    pub fn e2sr(&mut self) -> E2srW<SrsSpec> {
        E2srW::new(self, 12)
    }
}
#[doc = "Service Request Selector\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`srs::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`srs::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SrsSpec;
impl crate::RegisterSpec for SrsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`srs::R`](R) reader structure"]
impl crate::Readable for SrsSpec {}
#[doc = "`write(|w| ..)` method takes [`srs::W`](W) writer structure"]
impl crate::Writable for SrsSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SRS to value 0"]
impl crate::Resettable for SrsSpec {
    const RESET_VALUE: u32 = 0;
}
