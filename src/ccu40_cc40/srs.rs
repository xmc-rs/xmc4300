#[doc = "Register `SRS` reader"]
pub type R = crate::R<SRS_SPEC>;
#[doc = "Register `SRS` writer"]
pub type W = crate::W<SRS_SPEC>;
#[doc = "Period/One match Service request selector\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum POSR_A {
    #[doc = "0: Forward to CC4ySR0"]
    VALUE1 = 0,
    #[doc = "1: Forward to CC4ySR1"]
    VALUE2 = 1,
    #[doc = "2: Forward to CC4ySR2"]
    VALUE3 = 2,
    #[doc = "3: Forward to CC4ySR3"]
    VALUE4 = 3,
}
impl From<POSR_A> for u8 {
    #[inline(always)]
    fn from(variant: POSR_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for POSR_A {
    type Ux = u8;
}
impl crate::IsEnum for POSR_A {}
#[doc = "Field `POSR` reader - Period/One match Service request selector"]
pub type POSR_R = crate::FieldReader<POSR_A>;
impl POSR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> POSR_A {
        match self.bits {
            0 => POSR_A::VALUE1,
            1 => POSR_A::VALUE2,
            2 => POSR_A::VALUE3,
            3 => POSR_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Forward to CC4ySR0"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == POSR_A::VALUE1
    }
    #[doc = "Forward to CC4ySR1"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == POSR_A::VALUE2
    }
    #[doc = "Forward to CC4ySR2"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == POSR_A::VALUE3
    }
    #[doc = "Forward to CC4ySR3"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == POSR_A::VALUE4
    }
}
#[doc = "Field `POSR` writer - Period/One match Service request selector"]
pub type POSR_W<'a, REG> = crate::FieldWriter<'a, REG, 2, POSR_A, crate::Safe>;
impl<'a, REG> POSR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Forward to CC4ySR0"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(POSR_A::VALUE1)
    }
    #[doc = "Forward to CC4ySR1"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(POSR_A::VALUE2)
    }
    #[doc = "Forward to CC4ySR2"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(POSR_A::VALUE3)
    }
    #[doc = "Forward to CC4ySR3"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(POSR_A::VALUE4)
    }
}
#[doc = "Compare match Service request selector\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CMSR_A {
    #[doc = "0: Forward to CC4ySR0"]
    VALUE1 = 0,
    #[doc = "1: Forward to CC4ySR1"]
    VALUE2 = 1,
    #[doc = "2: Forward to CC4ySR2"]
    VALUE3 = 2,
    #[doc = "3: Forward to CC4ySR3"]
    VALUE4 = 3,
}
impl From<CMSR_A> for u8 {
    #[inline(always)]
    fn from(variant: CMSR_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CMSR_A {
    type Ux = u8;
}
impl crate::IsEnum for CMSR_A {}
#[doc = "Field `CMSR` reader - Compare match Service request selector"]
pub type CMSR_R = crate::FieldReader<CMSR_A>;
impl CMSR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CMSR_A {
        match self.bits {
            0 => CMSR_A::VALUE1,
            1 => CMSR_A::VALUE2,
            2 => CMSR_A::VALUE3,
            3 => CMSR_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Forward to CC4ySR0"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CMSR_A::VALUE1
    }
    #[doc = "Forward to CC4ySR1"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CMSR_A::VALUE2
    }
    #[doc = "Forward to CC4ySR2"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == CMSR_A::VALUE3
    }
    #[doc = "Forward to CC4ySR3"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == CMSR_A::VALUE4
    }
}
#[doc = "Field `CMSR` writer - Compare match Service request selector"]
pub type CMSR_W<'a, REG> = crate::FieldWriter<'a, REG, 2, CMSR_A, crate::Safe>;
impl<'a, REG> CMSR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Forward to CC4ySR0"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(CMSR_A::VALUE1)
    }
    #[doc = "Forward to CC4ySR1"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(CMSR_A::VALUE2)
    }
    #[doc = "Forward to CC4ySR2"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(CMSR_A::VALUE3)
    }
    #[doc = "Forward to CC4ySR3"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(CMSR_A::VALUE4)
    }
}
#[doc = "Event 0 Service request selector\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum E0SR_A {
    #[doc = "0: Forward to CC4ySR0"]
    VALUE1 = 0,
    #[doc = "1: Forward to CC4ySR1"]
    VALUE2 = 1,
    #[doc = "2: Forward to CC4ySR2"]
    VALUE3 = 2,
    #[doc = "3: Forward to CC4ySR3"]
    VALUE4 = 3,
}
impl From<E0SR_A> for u8 {
    #[inline(always)]
    fn from(variant: E0SR_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for E0SR_A {
    type Ux = u8;
}
impl crate::IsEnum for E0SR_A {}
#[doc = "Field `E0SR` reader - Event 0 Service request selector"]
pub type E0SR_R = crate::FieldReader<E0SR_A>;
impl E0SR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> E0SR_A {
        match self.bits {
            0 => E0SR_A::VALUE1,
            1 => E0SR_A::VALUE2,
            2 => E0SR_A::VALUE3,
            3 => E0SR_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Forward to CC4ySR0"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == E0SR_A::VALUE1
    }
    #[doc = "Forward to CC4ySR1"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == E0SR_A::VALUE2
    }
    #[doc = "Forward to CC4ySR2"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == E0SR_A::VALUE3
    }
    #[doc = "Forward to CC4ySR3"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == E0SR_A::VALUE4
    }
}
#[doc = "Field `E0SR` writer - Event 0 Service request selector"]
pub type E0SR_W<'a, REG> = crate::FieldWriter<'a, REG, 2, E0SR_A, crate::Safe>;
impl<'a, REG> E0SR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Forward to CC4ySR0"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(E0SR_A::VALUE1)
    }
    #[doc = "Forward to CC4ySR1"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(E0SR_A::VALUE2)
    }
    #[doc = "Forward to CC4ySR2"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(E0SR_A::VALUE3)
    }
    #[doc = "Forward to CC4ySR3"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(E0SR_A::VALUE4)
    }
}
#[doc = "Event 1 Service request selector\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum E1SR_A {
    #[doc = "0: Forward to CC4ySR0"]
    VALUE1 = 0,
    #[doc = "1: Forward to CC4ySR1"]
    VALUE2 = 1,
    #[doc = "2: Forward to CC4ySR2"]
    VALUE3 = 2,
    #[doc = "3: Forward to CC4ySR3"]
    VALUE4 = 3,
}
impl From<E1SR_A> for u8 {
    #[inline(always)]
    fn from(variant: E1SR_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for E1SR_A {
    type Ux = u8;
}
impl crate::IsEnum for E1SR_A {}
#[doc = "Field `E1SR` reader - Event 1 Service request selector"]
pub type E1SR_R = crate::FieldReader<E1SR_A>;
impl E1SR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> E1SR_A {
        match self.bits {
            0 => E1SR_A::VALUE1,
            1 => E1SR_A::VALUE2,
            2 => E1SR_A::VALUE3,
            3 => E1SR_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Forward to CC4ySR0"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == E1SR_A::VALUE1
    }
    #[doc = "Forward to CC4ySR1"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == E1SR_A::VALUE2
    }
    #[doc = "Forward to CC4ySR2"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == E1SR_A::VALUE3
    }
    #[doc = "Forward to CC4ySR3"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == E1SR_A::VALUE4
    }
}
#[doc = "Field `E1SR` writer - Event 1 Service request selector"]
pub type E1SR_W<'a, REG> = crate::FieldWriter<'a, REG, 2, E1SR_A, crate::Safe>;
impl<'a, REG> E1SR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Forward to CC4ySR0"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(E1SR_A::VALUE1)
    }
    #[doc = "Forward to CC4ySR1"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(E1SR_A::VALUE2)
    }
    #[doc = "Forward to CC4ySR2"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(E1SR_A::VALUE3)
    }
    #[doc = "Forward to CC4ySR3"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(E1SR_A::VALUE4)
    }
}
#[doc = "Event 2 Service request selector\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum E2SR_A {
    #[doc = "0: Forward to CC4ySR0"]
    VALUE1 = 0,
    #[doc = "1: Forward to CC4ySR1"]
    VALUE2 = 1,
    #[doc = "2: Forward to CC4ySR2"]
    VALUE3 = 2,
    #[doc = "3: Forward to CC4ySR3"]
    VALUE4 = 3,
}
impl From<E2SR_A> for u8 {
    #[inline(always)]
    fn from(variant: E2SR_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for E2SR_A {
    type Ux = u8;
}
impl crate::IsEnum for E2SR_A {}
#[doc = "Field `E2SR` reader - Event 2 Service request selector"]
pub type E2SR_R = crate::FieldReader<E2SR_A>;
impl E2SR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> E2SR_A {
        match self.bits {
            0 => E2SR_A::VALUE1,
            1 => E2SR_A::VALUE2,
            2 => E2SR_A::VALUE3,
            3 => E2SR_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Forward to CC4ySR0"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == E2SR_A::VALUE1
    }
    #[doc = "Forward to CC4ySR1"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == E2SR_A::VALUE2
    }
    #[doc = "Forward to CC4ySR2"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == E2SR_A::VALUE3
    }
    #[doc = "Forward to CC4ySR3"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == E2SR_A::VALUE4
    }
}
#[doc = "Field `E2SR` writer - Event 2 Service request selector"]
pub type E2SR_W<'a, REG> = crate::FieldWriter<'a, REG, 2, E2SR_A, crate::Safe>;
impl<'a, REG> E2SR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Forward to CC4ySR0"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(E2SR_A::VALUE1)
    }
    #[doc = "Forward to CC4ySR1"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(E2SR_A::VALUE2)
    }
    #[doc = "Forward to CC4ySR2"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(E2SR_A::VALUE3)
    }
    #[doc = "Forward to CC4ySR3"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(E2SR_A::VALUE4)
    }
}
impl R {
    #[doc = "Bits 0:1 - Period/One match Service request selector"]
    #[inline(always)]
    pub fn posr(&self) -> POSR_R {
        POSR_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Compare match Service request selector"]
    #[inline(always)]
    pub fn cmsr(&self) -> CMSR_R {
        CMSR_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Event 0 Service request selector"]
    #[inline(always)]
    pub fn e0sr(&self) -> E0SR_R {
        E0SR_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Event 1 Service request selector"]
    #[inline(always)]
    pub fn e1sr(&self) -> E1SR_R {
        E1SR_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Event 2 Service request selector"]
    #[inline(always)]
    pub fn e2sr(&self) -> E2SR_R {
        E2SR_R::new(((self.bits >> 12) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Period/One match Service request selector"]
    #[inline(always)]
    pub fn posr(&mut self) -> POSR_W<SRS_SPEC> {
        POSR_W::new(self, 0)
    }
    #[doc = "Bits 2:3 - Compare match Service request selector"]
    #[inline(always)]
    pub fn cmsr(&mut self) -> CMSR_W<SRS_SPEC> {
        CMSR_W::new(self, 2)
    }
    #[doc = "Bits 8:9 - Event 0 Service request selector"]
    #[inline(always)]
    pub fn e0sr(&mut self) -> E0SR_W<SRS_SPEC> {
        E0SR_W::new(self, 8)
    }
    #[doc = "Bits 10:11 - Event 1 Service request selector"]
    #[inline(always)]
    pub fn e1sr(&mut self) -> E1SR_W<SRS_SPEC> {
        E1SR_W::new(self, 10)
    }
    #[doc = "Bits 12:13 - Event 2 Service request selector"]
    #[inline(always)]
    pub fn e2sr(&mut self) -> E2SR_W<SRS_SPEC> {
        E2SR_W::new(self, 12)
    }
}
#[doc = "Service Request Selector\n\nYou can [`read`](crate::Reg::read) this register and get [`srs::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`srs::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SRS_SPEC;
impl crate::RegisterSpec for SRS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`srs::R`](R) reader structure"]
impl crate::Readable for SRS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`srs::W`](W) writer structure"]
impl crate::Writable for SRS_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SRS to value 0"]
impl crate::Resettable for SRS_SPEC {
    const RESET_VALUE: u32 = 0;
}
