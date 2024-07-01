#[doc = "Register `EXISEL` reader"]
pub type R = crate::R<EXISEL_SPEC>;
#[doc = "Register `EXISEL` writer"]
pub type W = crate::W<EXISEL_SPEC>;
#[doc = "Event Source Select for A0 (ERS0)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EXS0A_A {
    #[doc = "0: Input ERU_0A0 is selected"]
    VALUE1 = 0,
    #[doc = "1: Input ERU_0A1 is selected"]
    VALUE2 = 1,
    #[doc = "2: Input ERU_0A2 is selected"]
    VALUE3 = 2,
    #[doc = "3: Input ERU_0A3 is selected"]
    VALUE4 = 3,
}
impl From<EXS0A_A> for u8 {
    #[inline(always)]
    fn from(variant: EXS0A_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for EXS0A_A {
    type Ux = u8;
}
impl crate::IsEnum for EXS0A_A {}
#[doc = "Field `EXS0A` reader - Event Source Select for A0 (ERS0)"]
pub type EXS0A_R = crate::FieldReader<EXS0A_A>;
impl EXS0A_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EXS0A_A {
        match self.bits {
            0 => EXS0A_A::VALUE1,
            1 => EXS0A_A::VALUE2,
            2 => EXS0A_A::VALUE3,
            3 => EXS0A_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Input ERU_0A0 is selected"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == EXS0A_A::VALUE1
    }
    #[doc = "Input ERU_0A1 is selected"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == EXS0A_A::VALUE2
    }
    #[doc = "Input ERU_0A2 is selected"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == EXS0A_A::VALUE3
    }
    #[doc = "Input ERU_0A3 is selected"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == EXS0A_A::VALUE4
    }
}
#[doc = "Field `EXS0A` writer - Event Source Select for A0 (ERS0)"]
pub type EXS0A_W<'a, REG> = crate::FieldWriter<'a, REG, 2, EXS0A_A, crate::Safe>;
impl<'a, REG> EXS0A_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Input ERU_0A0 is selected"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(EXS0A_A::VALUE1)
    }
    #[doc = "Input ERU_0A1 is selected"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(EXS0A_A::VALUE2)
    }
    #[doc = "Input ERU_0A2 is selected"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(EXS0A_A::VALUE3)
    }
    #[doc = "Input ERU_0A3 is selected"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(EXS0A_A::VALUE4)
    }
}
#[doc = "Event Source Select for B0 (ERS0)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EXS0B_A {
    #[doc = "0: Input ERU_0B0 is selected"]
    VALUE1 = 0,
    #[doc = "1: Input ERU_0B1 is selected"]
    VALUE2 = 1,
    #[doc = "2: Input ERU_0B2 is selected"]
    VALUE3 = 2,
    #[doc = "3: Input ERU_0B3 is selected"]
    VALUE4 = 3,
}
impl From<EXS0B_A> for u8 {
    #[inline(always)]
    fn from(variant: EXS0B_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for EXS0B_A {
    type Ux = u8;
}
impl crate::IsEnum for EXS0B_A {}
#[doc = "Field `EXS0B` reader - Event Source Select for B0 (ERS0)"]
pub type EXS0B_R = crate::FieldReader<EXS0B_A>;
impl EXS0B_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EXS0B_A {
        match self.bits {
            0 => EXS0B_A::VALUE1,
            1 => EXS0B_A::VALUE2,
            2 => EXS0B_A::VALUE3,
            3 => EXS0B_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Input ERU_0B0 is selected"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == EXS0B_A::VALUE1
    }
    #[doc = "Input ERU_0B1 is selected"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == EXS0B_A::VALUE2
    }
    #[doc = "Input ERU_0B2 is selected"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == EXS0B_A::VALUE3
    }
    #[doc = "Input ERU_0B3 is selected"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == EXS0B_A::VALUE4
    }
}
#[doc = "Field `EXS0B` writer - Event Source Select for B0 (ERS0)"]
pub type EXS0B_W<'a, REG> = crate::FieldWriter<'a, REG, 2, EXS0B_A, crate::Safe>;
impl<'a, REG> EXS0B_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Input ERU_0B0 is selected"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(EXS0B_A::VALUE1)
    }
    #[doc = "Input ERU_0B1 is selected"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(EXS0B_A::VALUE2)
    }
    #[doc = "Input ERU_0B2 is selected"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(EXS0B_A::VALUE3)
    }
    #[doc = "Input ERU_0B3 is selected"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(EXS0B_A::VALUE4)
    }
}
#[doc = "Event Source Select for A1 (ERS1)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EXS1A_A {
    #[doc = "0: Input ERU_1A0 is selected"]
    VALUE1 = 0,
    #[doc = "1: Input ERU_1A1 is selected"]
    VALUE2 = 1,
    #[doc = "2: Input ERU_1A2 is selected"]
    VALUE3 = 2,
    #[doc = "3: Input ERU_1A3 is selected"]
    VALUE4 = 3,
}
impl From<EXS1A_A> for u8 {
    #[inline(always)]
    fn from(variant: EXS1A_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for EXS1A_A {
    type Ux = u8;
}
impl crate::IsEnum for EXS1A_A {}
#[doc = "Field `EXS1A` reader - Event Source Select for A1 (ERS1)"]
pub type EXS1A_R = crate::FieldReader<EXS1A_A>;
impl EXS1A_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EXS1A_A {
        match self.bits {
            0 => EXS1A_A::VALUE1,
            1 => EXS1A_A::VALUE2,
            2 => EXS1A_A::VALUE3,
            3 => EXS1A_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Input ERU_1A0 is selected"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == EXS1A_A::VALUE1
    }
    #[doc = "Input ERU_1A1 is selected"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == EXS1A_A::VALUE2
    }
    #[doc = "Input ERU_1A2 is selected"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == EXS1A_A::VALUE3
    }
    #[doc = "Input ERU_1A3 is selected"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == EXS1A_A::VALUE4
    }
}
#[doc = "Field `EXS1A` writer - Event Source Select for A1 (ERS1)"]
pub type EXS1A_W<'a, REG> = crate::FieldWriter<'a, REG, 2, EXS1A_A, crate::Safe>;
impl<'a, REG> EXS1A_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Input ERU_1A0 is selected"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(EXS1A_A::VALUE1)
    }
    #[doc = "Input ERU_1A1 is selected"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(EXS1A_A::VALUE2)
    }
    #[doc = "Input ERU_1A2 is selected"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(EXS1A_A::VALUE3)
    }
    #[doc = "Input ERU_1A3 is selected"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(EXS1A_A::VALUE4)
    }
}
#[doc = "Event Source Select for B1 (ERS1)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EXS1B_A {
    #[doc = "0: Input ERU_1B0 is selected"]
    VALUE1 = 0,
    #[doc = "1: Input ERU_1B1 is selected"]
    VALUE2 = 1,
    #[doc = "2: Input ERU_1B2 is selected"]
    VALUE3 = 2,
    #[doc = "3: Input ERU_1B3 is selected"]
    VALUE4 = 3,
}
impl From<EXS1B_A> for u8 {
    #[inline(always)]
    fn from(variant: EXS1B_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for EXS1B_A {
    type Ux = u8;
}
impl crate::IsEnum for EXS1B_A {}
#[doc = "Field `EXS1B` reader - Event Source Select for B1 (ERS1)"]
pub type EXS1B_R = crate::FieldReader<EXS1B_A>;
impl EXS1B_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EXS1B_A {
        match self.bits {
            0 => EXS1B_A::VALUE1,
            1 => EXS1B_A::VALUE2,
            2 => EXS1B_A::VALUE3,
            3 => EXS1B_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Input ERU_1B0 is selected"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == EXS1B_A::VALUE1
    }
    #[doc = "Input ERU_1B1 is selected"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == EXS1B_A::VALUE2
    }
    #[doc = "Input ERU_1B2 is selected"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == EXS1B_A::VALUE3
    }
    #[doc = "Input ERU_1B3 is selected"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == EXS1B_A::VALUE4
    }
}
#[doc = "Field `EXS1B` writer - Event Source Select for B1 (ERS1)"]
pub type EXS1B_W<'a, REG> = crate::FieldWriter<'a, REG, 2, EXS1B_A, crate::Safe>;
impl<'a, REG> EXS1B_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Input ERU_1B0 is selected"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(EXS1B_A::VALUE1)
    }
    #[doc = "Input ERU_1B1 is selected"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(EXS1B_A::VALUE2)
    }
    #[doc = "Input ERU_1B2 is selected"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(EXS1B_A::VALUE3)
    }
    #[doc = "Input ERU_1B3 is selected"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(EXS1B_A::VALUE4)
    }
}
#[doc = "Event Source Select for A2 (ERS2)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EXS2A_A {
    #[doc = "0: Input ERU_2A0 is selected"]
    VALUE1 = 0,
    #[doc = "1: Input ERU_2A1 is selected"]
    VALUE2 = 1,
    #[doc = "2: Input ERU_2A2 is selected"]
    VALUE3 = 2,
    #[doc = "3: Input ERU_2A3 is selected"]
    VALUE4 = 3,
}
impl From<EXS2A_A> for u8 {
    #[inline(always)]
    fn from(variant: EXS2A_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for EXS2A_A {
    type Ux = u8;
}
impl crate::IsEnum for EXS2A_A {}
#[doc = "Field `EXS2A` reader - Event Source Select for A2 (ERS2)"]
pub type EXS2A_R = crate::FieldReader<EXS2A_A>;
impl EXS2A_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EXS2A_A {
        match self.bits {
            0 => EXS2A_A::VALUE1,
            1 => EXS2A_A::VALUE2,
            2 => EXS2A_A::VALUE3,
            3 => EXS2A_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Input ERU_2A0 is selected"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == EXS2A_A::VALUE1
    }
    #[doc = "Input ERU_2A1 is selected"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == EXS2A_A::VALUE2
    }
    #[doc = "Input ERU_2A2 is selected"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == EXS2A_A::VALUE3
    }
    #[doc = "Input ERU_2A3 is selected"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == EXS2A_A::VALUE4
    }
}
#[doc = "Field `EXS2A` writer - Event Source Select for A2 (ERS2)"]
pub type EXS2A_W<'a, REG> = crate::FieldWriter<'a, REG, 2, EXS2A_A, crate::Safe>;
impl<'a, REG> EXS2A_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Input ERU_2A0 is selected"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(EXS2A_A::VALUE1)
    }
    #[doc = "Input ERU_2A1 is selected"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(EXS2A_A::VALUE2)
    }
    #[doc = "Input ERU_2A2 is selected"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(EXS2A_A::VALUE3)
    }
    #[doc = "Input ERU_2A3 is selected"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(EXS2A_A::VALUE4)
    }
}
#[doc = "Event Source Select for B2 (ERS2)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EXS2B_A {
    #[doc = "0: Input ERU_2B0 is selected"]
    VALUE1 = 0,
    #[doc = "1: Input ERU_2B1 is selected"]
    VALUE2 = 1,
    #[doc = "2: Input ERU_2B2 is selected"]
    VALUE3 = 2,
    #[doc = "3: Input ERU_2B3 is selected"]
    VALUE4 = 3,
}
impl From<EXS2B_A> for u8 {
    #[inline(always)]
    fn from(variant: EXS2B_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for EXS2B_A {
    type Ux = u8;
}
impl crate::IsEnum for EXS2B_A {}
#[doc = "Field `EXS2B` reader - Event Source Select for B2 (ERS2)"]
pub type EXS2B_R = crate::FieldReader<EXS2B_A>;
impl EXS2B_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EXS2B_A {
        match self.bits {
            0 => EXS2B_A::VALUE1,
            1 => EXS2B_A::VALUE2,
            2 => EXS2B_A::VALUE3,
            3 => EXS2B_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Input ERU_2B0 is selected"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == EXS2B_A::VALUE1
    }
    #[doc = "Input ERU_2B1 is selected"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == EXS2B_A::VALUE2
    }
    #[doc = "Input ERU_2B2 is selected"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == EXS2B_A::VALUE3
    }
    #[doc = "Input ERU_2B3 is selected"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == EXS2B_A::VALUE4
    }
}
#[doc = "Field `EXS2B` writer - Event Source Select for B2 (ERS2)"]
pub type EXS2B_W<'a, REG> = crate::FieldWriter<'a, REG, 2, EXS2B_A, crate::Safe>;
impl<'a, REG> EXS2B_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Input ERU_2B0 is selected"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(EXS2B_A::VALUE1)
    }
    #[doc = "Input ERU_2B1 is selected"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(EXS2B_A::VALUE2)
    }
    #[doc = "Input ERU_2B2 is selected"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(EXS2B_A::VALUE3)
    }
    #[doc = "Input ERU_2B3 is selected"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(EXS2B_A::VALUE4)
    }
}
#[doc = "Event Source Select for A3 (ERS3)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EXS3A_A {
    #[doc = "0: Input ERU_3A0 is selected"]
    VALUE1 = 0,
    #[doc = "1: Input ERU_3A1 is selected"]
    VALUE2 = 1,
    #[doc = "2: Input ERU_3A2 is selected"]
    VALUE3 = 2,
    #[doc = "3: Input ERU_3A3 is selected"]
    VALUE4 = 3,
}
impl From<EXS3A_A> for u8 {
    #[inline(always)]
    fn from(variant: EXS3A_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for EXS3A_A {
    type Ux = u8;
}
impl crate::IsEnum for EXS3A_A {}
#[doc = "Field `EXS3A` reader - Event Source Select for A3 (ERS3)"]
pub type EXS3A_R = crate::FieldReader<EXS3A_A>;
impl EXS3A_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EXS3A_A {
        match self.bits {
            0 => EXS3A_A::VALUE1,
            1 => EXS3A_A::VALUE2,
            2 => EXS3A_A::VALUE3,
            3 => EXS3A_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Input ERU_3A0 is selected"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == EXS3A_A::VALUE1
    }
    #[doc = "Input ERU_3A1 is selected"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == EXS3A_A::VALUE2
    }
    #[doc = "Input ERU_3A2 is selected"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == EXS3A_A::VALUE3
    }
    #[doc = "Input ERU_3A3 is selected"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == EXS3A_A::VALUE4
    }
}
#[doc = "Field `EXS3A` writer - Event Source Select for A3 (ERS3)"]
pub type EXS3A_W<'a, REG> = crate::FieldWriter<'a, REG, 2, EXS3A_A, crate::Safe>;
impl<'a, REG> EXS3A_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Input ERU_3A0 is selected"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(EXS3A_A::VALUE1)
    }
    #[doc = "Input ERU_3A1 is selected"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(EXS3A_A::VALUE2)
    }
    #[doc = "Input ERU_3A2 is selected"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(EXS3A_A::VALUE3)
    }
    #[doc = "Input ERU_3A3 is selected"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(EXS3A_A::VALUE4)
    }
}
#[doc = "Event Source Select for B3 (ERS3)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EXS3B_A {
    #[doc = "0: Input ERU_3B0 is selected"]
    VALUE1 = 0,
    #[doc = "1: Input ERU_3B1 is selected"]
    VALUE2 = 1,
    #[doc = "2: Input ERU_3B2 is selected"]
    VALUE3 = 2,
    #[doc = "3: Input ERU_3B3 is selected"]
    VALUE4 = 3,
}
impl From<EXS3B_A> for u8 {
    #[inline(always)]
    fn from(variant: EXS3B_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for EXS3B_A {
    type Ux = u8;
}
impl crate::IsEnum for EXS3B_A {}
#[doc = "Field `EXS3B` reader - Event Source Select for B3 (ERS3)"]
pub type EXS3B_R = crate::FieldReader<EXS3B_A>;
impl EXS3B_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EXS3B_A {
        match self.bits {
            0 => EXS3B_A::VALUE1,
            1 => EXS3B_A::VALUE2,
            2 => EXS3B_A::VALUE3,
            3 => EXS3B_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Input ERU_3B0 is selected"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == EXS3B_A::VALUE1
    }
    #[doc = "Input ERU_3B1 is selected"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == EXS3B_A::VALUE2
    }
    #[doc = "Input ERU_3B2 is selected"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == EXS3B_A::VALUE3
    }
    #[doc = "Input ERU_3B3 is selected"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == EXS3B_A::VALUE4
    }
}
#[doc = "Field `EXS3B` writer - Event Source Select for B3 (ERS3)"]
pub type EXS3B_W<'a, REG> = crate::FieldWriter<'a, REG, 2, EXS3B_A, crate::Safe>;
impl<'a, REG> EXS3B_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Input ERU_3B0 is selected"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(EXS3B_A::VALUE1)
    }
    #[doc = "Input ERU_3B1 is selected"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(EXS3B_A::VALUE2)
    }
    #[doc = "Input ERU_3B2 is selected"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(EXS3B_A::VALUE3)
    }
    #[doc = "Input ERU_3B3 is selected"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(EXS3B_A::VALUE4)
    }
}
impl R {
    #[doc = "Bits 0:1 - Event Source Select for A0 (ERS0)"]
    #[inline(always)]
    pub fn exs0a(&self) -> EXS0A_R {
        EXS0A_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Event Source Select for B0 (ERS0)"]
    #[inline(always)]
    pub fn exs0b(&self) -> EXS0B_R {
        EXS0B_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Event Source Select for A1 (ERS1)"]
    #[inline(always)]
    pub fn exs1a(&self) -> EXS1A_R {
        EXS1A_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Event Source Select for B1 (ERS1)"]
    #[inline(always)]
    pub fn exs1b(&self) -> EXS1B_R {
        EXS1B_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Event Source Select for A2 (ERS2)"]
    #[inline(always)]
    pub fn exs2a(&self) -> EXS2A_R {
        EXS2A_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Event Source Select for B2 (ERS2)"]
    #[inline(always)]
    pub fn exs2b(&self) -> EXS2B_R {
        EXS2B_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Event Source Select for A3 (ERS3)"]
    #[inline(always)]
    pub fn exs3a(&self) -> EXS3A_R {
        EXS3A_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Event Source Select for B3 (ERS3)"]
    #[inline(always)]
    pub fn exs3b(&self) -> EXS3B_R {
        EXS3B_R::new(((self.bits >> 14) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Event Source Select for A0 (ERS0)"]
    #[inline(always)]
    #[must_use]
    pub fn exs0a(&mut self) -> EXS0A_W<EXISEL_SPEC> {
        EXS0A_W::new(self, 0)
    }
    #[doc = "Bits 2:3 - Event Source Select for B0 (ERS0)"]
    #[inline(always)]
    #[must_use]
    pub fn exs0b(&mut self) -> EXS0B_W<EXISEL_SPEC> {
        EXS0B_W::new(self, 2)
    }
    #[doc = "Bits 4:5 - Event Source Select for A1 (ERS1)"]
    #[inline(always)]
    #[must_use]
    pub fn exs1a(&mut self) -> EXS1A_W<EXISEL_SPEC> {
        EXS1A_W::new(self, 4)
    }
    #[doc = "Bits 6:7 - Event Source Select for B1 (ERS1)"]
    #[inline(always)]
    #[must_use]
    pub fn exs1b(&mut self) -> EXS1B_W<EXISEL_SPEC> {
        EXS1B_W::new(self, 6)
    }
    #[doc = "Bits 8:9 - Event Source Select for A2 (ERS2)"]
    #[inline(always)]
    #[must_use]
    pub fn exs2a(&mut self) -> EXS2A_W<EXISEL_SPEC> {
        EXS2A_W::new(self, 8)
    }
    #[doc = "Bits 10:11 - Event Source Select for B2 (ERS2)"]
    #[inline(always)]
    #[must_use]
    pub fn exs2b(&mut self) -> EXS2B_W<EXISEL_SPEC> {
        EXS2B_W::new(self, 10)
    }
    #[doc = "Bits 12:13 - Event Source Select for A3 (ERS3)"]
    #[inline(always)]
    #[must_use]
    pub fn exs3a(&mut self) -> EXS3A_W<EXISEL_SPEC> {
        EXS3A_W::new(self, 12)
    }
    #[doc = "Bits 14:15 - Event Source Select for B3 (ERS3)"]
    #[inline(always)]
    #[must_use]
    pub fn exs3b(&mut self) -> EXS3B_W<EXISEL_SPEC> {
        EXS3B_W::new(self, 14)
    }
}
#[doc = "Event Input Select\n\nYou can [`read`](crate::Reg::read) this register and get [`exisel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`exisel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EXISEL_SPEC;
impl crate::RegisterSpec for EXISEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`exisel::R`](R) reader structure"]
impl crate::Readable for EXISEL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`exisel::W`](W) writer structure"]
impl crate::Writable for EXISEL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EXISEL to value 0"]
impl crate::Resettable for EXISEL_SPEC {
    const RESET_VALUE: u32 = 0;
}
