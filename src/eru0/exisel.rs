#[doc = "Register `EXISEL` reader"]
pub type R = crate::R<ExiselSpec>;
#[doc = "Register `EXISEL` writer"]
pub type W = crate::W<ExiselSpec>;
#[doc = "Event Source Select for A0 (ERS0)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Exs0a {
    #[doc = "0: Input ERU_0A0 is selected"]
    Value1 = 0,
    #[doc = "1: Input ERU_0A1 is selected"]
    Value2 = 1,
    #[doc = "2: Input ERU_0A2 is selected"]
    Value3 = 2,
    #[doc = "3: Input ERU_0A3 is selected"]
    Value4 = 3,
}
impl From<Exs0a> for u8 {
    #[inline(always)]
    fn from(variant: Exs0a) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Exs0a {
    type Ux = u8;
}
#[doc = "Field `EXS0A` reader - Event Source Select for A0 (ERS0)"]
pub type Exs0aR = crate::FieldReader<Exs0a>;
impl Exs0aR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Exs0a {
        match self.bits {
            0 => Exs0a::Value1,
            1 => Exs0a::Value2,
            2 => Exs0a::Value3,
            3 => Exs0a::Value4,
            _ => unreachable!(),
        }
    }
    #[doc = "Input ERU_0A0 is selected"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Exs0a::Value1
    }
    #[doc = "Input ERU_0A1 is selected"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Exs0a::Value2
    }
    #[doc = "Input ERU_0A2 is selected"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == Exs0a::Value3
    }
    #[doc = "Input ERU_0A3 is selected"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == Exs0a::Value4
    }
}
#[doc = "Field `EXS0A` writer - Event Source Select for A0 (ERS0)"]
pub type Exs0aW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Exs0a>;
impl<'a, REG> Exs0aW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Input ERU_0A0 is selected"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Exs0a::Value1)
    }
    #[doc = "Input ERU_0A1 is selected"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Exs0a::Value2)
    }
    #[doc = "Input ERU_0A2 is selected"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(Exs0a::Value3)
    }
    #[doc = "Input ERU_0A3 is selected"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(Exs0a::Value4)
    }
}
#[doc = "Event Source Select for B0 (ERS0)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Exs0b {
    #[doc = "0: Input ERU_0B0 is selected"]
    Value1 = 0,
    #[doc = "1: Input ERU_0B1 is selected"]
    Value2 = 1,
    #[doc = "2: Input ERU_0B2 is selected"]
    Value3 = 2,
    #[doc = "3: Input ERU_0B3 is selected"]
    Value4 = 3,
}
impl From<Exs0b> for u8 {
    #[inline(always)]
    fn from(variant: Exs0b) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Exs0b {
    type Ux = u8;
}
#[doc = "Field `EXS0B` reader - Event Source Select for B0 (ERS0)"]
pub type Exs0bR = crate::FieldReader<Exs0b>;
impl Exs0bR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Exs0b {
        match self.bits {
            0 => Exs0b::Value1,
            1 => Exs0b::Value2,
            2 => Exs0b::Value3,
            3 => Exs0b::Value4,
            _ => unreachable!(),
        }
    }
    #[doc = "Input ERU_0B0 is selected"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Exs0b::Value1
    }
    #[doc = "Input ERU_0B1 is selected"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Exs0b::Value2
    }
    #[doc = "Input ERU_0B2 is selected"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == Exs0b::Value3
    }
    #[doc = "Input ERU_0B3 is selected"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == Exs0b::Value4
    }
}
#[doc = "Field `EXS0B` writer - Event Source Select for B0 (ERS0)"]
pub type Exs0bW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Exs0b>;
impl<'a, REG> Exs0bW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Input ERU_0B0 is selected"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Exs0b::Value1)
    }
    #[doc = "Input ERU_0B1 is selected"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Exs0b::Value2)
    }
    #[doc = "Input ERU_0B2 is selected"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(Exs0b::Value3)
    }
    #[doc = "Input ERU_0B3 is selected"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(Exs0b::Value4)
    }
}
#[doc = "Event Source Select for A1 (ERS1)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Exs1a {
    #[doc = "0: Input ERU_1A0 is selected"]
    Value1 = 0,
    #[doc = "1: Input ERU_1A1 is selected"]
    Value2 = 1,
    #[doc = "2: Input ERU_1A2 is selected"]
    Value3 = 2,
    #[doc = "3: Input ERU_1A3 is selected"]
    Value4 = 3,
}
impl From<Exs1a> for u8 {
    #[inline(always)]
    fn from(variant: Exs1a) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Exs1a {
    type Ux = u8;
}
#[doc = "Field `EXS1A` reader - Event Source Select for A1 (ERS1)"]
pub type Exs1aR = crate::FieldReader<Exs1a>;
impl Exs1aR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Exs1a {
        match self.bits {
            0 => Exs1a::Value1,
            1 => Exs1a::Value2,
            2 => Exs1a::Value3,
            3 => Exs1a::Value4,
            _ => unreachable!(),
        }
    }
    #[doc = "Input ERU_1A0 is selected"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Exs1a::Value1
    }
    #[doc = "Input ERU_1A1 is selected"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Exs1a::Value2
    }
    #[doc = "Input ERU_1A2 is selected"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == Exs1a::Value3
    }
    #[doc = "Input ERU_1A3 is selected"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == Exs1a::Value4
    }
}
#[doc = "Field `EXS1A` writer - Event Source Select for A1 (ERS1)"]
pub type Exs1aW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Exs1a>;
impl<'a, REG> Exs1aW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Input ERU_1A0 is selected"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Exs1a::Value1)
    }
    #[doc = "Input ERU_1A1 is selected"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Exs1a::Value2)
    }
    #[doc = "Input ERU_1A2 is selected"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(Exs1a::Value3)
    }
    #[doc = "Input ERU_1A3 is selected"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(Exs1a::Value4)
    }
}
#[doc = "Event Source Select for B1 (ERS1)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Exs1b {
    #[doc = "0: Input ERU_1B0 is selected"]
    Value1 = 0,
    #[doc = "1: Input ERU_1B1 is selected"]
    Value2 = 1,
    #[doc = "2: Input ERU_1B2 is selected"]
    Value3 = 2,
    #[doc = "3: Input ERU_1B3 is selected"]
    Value4 = 3,
}
impl From<Exs1b> for u8 {
    #[inline(always)]
    fn from(variant: Exs1b) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Exs1b {
    type Ux = u8;
}
#[doc = "Field `EXS1B` reader - Event Source Select for B1 (ERS1)"]
pub type Exs1bR = crate::FieldReader<Exs1b>;
impl Exs1bR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Exs1b {
        match self.bits {
            0 => Exs1b::Value1,
            1 => Exs1b::Value2,
            2 => Exs1b::Value3,
            3 => Exs1b::Value4,
            _ => unreachable!(),
        }
    }
    #[doc = "Input ERU_1B0 is selected"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Exs1b::Value1
    }
    #[doc = "Input ERU_1B1 is selected"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Exs1b::Value2
    }
    #[doc = "Input ERU_1B2 is selected"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == Exs1b::Value3
    }
    #[doc = "Input ERU_1B3 is selected"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == Exs1b::Value4
    }
}
#[doc = "Field `EXS1B` writer - Event Source Select for B1 (ERS1)"]
pub type Exs1bW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Exs1b>;
impl<'a, REG> Exs1bW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Input ERU_1B0 is selected"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Exs1b::Value1)
    }
    #[doc = "Input ERU_1B1 is selected"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Exs1b::Value2)
    }
    #[doc = "Input ERU_1B2 is selected"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(Exs1b::Value3)
    }
    #[doc = "Input ERU_1B3 is selected"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(Exs1b::Value4)
    }
}
#[doc = "Event Source Select for A2 (ERS2)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Exs2a {
    #[doc = "0: Input ERU_2A0 is selected"]
    Value1 = 0,
    #[doc = "1: Input ERU_2A1 is selected"]
    Value2 = 1,
    #[doc = "2: Input ERU_2A2 is selected"]
    Value3 = 2,
    #[doc = "3: Input ERU_2A3 is selected"]
    Value4 = 3,
}
impl From<Exs2a> for u8 {
    #[inline(always)]
    fn from(variant: Exs2a) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Exs2a {
    type Ux = u8;
}
#[doc = "Field `EXS2A` reader - Event Source Select for A2 (ERS2)"]
pub type Exs2aR = crate::FieldReader<Exs2a>;
impl Exs2aR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Exs2a {
        match self.bits {
            0 => Exs2a::Value1,
            1 => Exs2a::Value2,
            2 => Exs2a::Value3,
            3 => Exs2a::Value4,
            _ => unreachable!(),
        }
    }
    #[doc = "Input ERU_2A0 is selected"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Exs2a::Value1
    }
    #[doc = "Input ERU_2A1 is selected"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Exs2a::Value2
    }
    #[doc = "Input ERU_2A2 is selected"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == Exs2a::Value3
    }
    #[doc = "Input ERU_2A3 is selected"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == Exs2a::Value4
    }
}
#[doc = "Field `EXS2A` writer - Event Source Select for A2 (ERS2)"]
pub type Exs2aW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Exs2a>;
impl<'a, REG> Exs2aW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Input ERU_2A0 is selected"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Exs2a::Value1)
    }
    #[doc = "Input ERU_2A1 is selected"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Exs2a::Value2)
    }
    #[doc = "Input ERU_2A2 is selected"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(Exs2a::Value3)
    }
    #[doc = "Input ERU_2A3 is selected"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(Exs2a::Value4)
    }
}
#[doc = "Event Source Select for B2 (ERS2)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Exs2b {
    #[doc = "0: Input ERU_2B0 is selected"]
    Value1 = 0,
    #[doc = "1: Input ERU_2B1 is selected"]
    Value2 = 1,
    #[doc = "2: Input ERU_2B2 is selected"]
    Value3 = 2,
    #[doc = "3: Input ERU_2B3 is selected"]
    Value4 = 3,
}
impl From<Exs2b> for u8 {
    #[inline(always)]
    fn from(variant: Exs2b) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Exs2b {
    type Ux = u8;
}
#[doc = "Field `EXS2B` reader - Event Source Select for B2 (ERS2)"]
pub type Exs2bR = crate::FieldReader<Exs2b>;
impl Exs2bR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Exs2b {
        match self.bits {
            0 => Exs2b::Value1,
            1 => Exs2b::Value2,
            2 => Exs2b::Value3,
            3 => Exs2b::Value4,
            _ => unreachable!(),
        }
    }
    #[doc = "Input ERU_2B0 is selected"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Exs2b::Value1
    }
    #[doc = "Input ERU_2B1 is selected"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Exs2b::Value2
    }
    #[doc = "Input ERU_2B2 is selected"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == Exs2b::Value3
    }
    #[doc = "Input ERU_2B3 is selected"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == Exs2b::Value4
    }
}
#[doc = "Field `EXS2B` writer - Event Source Select for B2 (ERS2)"]
pub type Exs2bW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Exs2b>;
impl<'a, REG> Exs2bW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Input ERU_2B0 is selected"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Exs2b::Value1)
    }
    #[doc = "Input ERU_2B1 is selected"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Exs2b::Value2)
    }
    #[doc = "Input ERU_2B2 is selected"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(Exs2b::Value3)
    }
    #[doc = "Input ERU_2B3 is selected"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(Exs2b::Value4)
    }
}
#[doc = "Event Source Select for A3 (ERS3)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Exs3a {
    #[doc = "0: Input ERU_3A0 is selected"]
    Value1 = 0,
    #[doc = "1: Input ERU_3A1 is selected"]
    Value2 = 1,
    #[doc = "2: Input ERU_3A2 is selected"]
    Value3 = 2,
    #[doc = "3: Input ERU_3A3 is selected"]
    Value4 = 3,
}
impl From<Exs3a> for u8 {
    #[inline(always)]
    fn from(variant: Exs3a) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Exs3a {
    type Ux = u8;
}
#[doc = "Field `EXS3A` reader - Event Source Select for A3 (ERS3)"]
pub type Exs3aR = crate::FieldReader<Exs3a>;
impl Exs3aR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Exs3a {
        match self.bits {
            0 => Exs3a::Value1,
            1 => Exs3a::Value2,
            2 => Exs3a::Value3,
            3 => Exs3a::Value4,
            _ => unreachable!(),
        }
    }
    #[doc = "Input ERU_3A0 is selected"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Exs3a::Value1
    }
    #[doc = "Input ERU_3A1 is selected"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Exs3a::Value2
    }
    #[doc = "Input ERU_3A2 is selected"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == Exs3a::Value3
    }
    #[doc = "Input ERU_3A3 is selected"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == Exs3a::Value4
    }
}
#[doc = "Field `EXS3A` writer - Event Source Select for A3 (ERS3)"]
pub type Exs3aW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Exs3a>;
impl<'a, REG> Exs3aW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Input ERU_3A0 is selected"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Exs3a::Value1)
    }
    #[doc = "Input ERU_3A1 is selected"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Exs3a::Value2)
    }
    #[doc = "Input ERU_3A2 is selected"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(Exs3a::Value3)
    }
    #[doc = "Input ERU_3A3 is selected"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(Exs3a::Value4)
    }
}
#[doc = "Event Source Select for B3 (ERS3)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Exs3b {
    #[doc = "0: Input ERU_3B0 is selected"]
    Value1 = 0,
    #[doc = "1: Input ERU_3B1 is selected"]
    Value2 = 1,
    #[doc = "2: Input ERU_3B2 is selected"]
    Value3 = 2,
    #[doc = "3: Input ERU_3B3 is selected"]
    Value4 = 3,
}
impl From<Exs3b> for u8 {
    #[inline(always)]
    fn from(variant: Exs3b) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Exs3b {
    type Ux = u8;
}
#[doc = "Field `EXS3B` reader - Event Source Select for B3 (ERS3)"]
pub type Exs3bR = crate::FieldReader<Exs3b>;
impl Exs3bR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Exs3b {
        match self.bits {
            0 => Exs3b::Value1,
            1 => Exs3b::Value2,
            2 => Exs3b::Value3,
            3 => Exs3b::Value4,
            _ => unreachable!(),
        }
    }
    #[doc = "Input ERU_3B0 is selected"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Exs3b::Value1
    }
    #[doc = "Input ERU_3B1 is selected"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Exs3b::Value2
    }
    #[doc = "Input ERU_3B2 is selected"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == Exs3b::Value3
    }
    #[doc = "Input ERU_3B3 is selected"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == Exs3b::Value4
    }
}
#[doc = "Field `EXS3B` writer - Event Source Select for B3 (ERS3)"]
pub type Exs3bW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Exs3b>;
impl<'a, REG> Exs3bW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Input ERU_3B0 is selected"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Exs3b::Value1)
    }
    #[doc = "Input ERU_3B1 is selected"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Exs3b::Value2)
    }
    #[doc = "Input ERU_3B2 is selected"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(Exs3b::Value3)
    }
    #[doc = "Input ERU_3B3 is selected"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(Exs3b::Value4)
    }
}
impl R {
    #[doc = "Bits 0:1 - Event Source Select for A0 (ERS0)"]
    #[inline(always)]
    pub fn exs0a(&self) -> Exs0aR {
        Exs0aR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Event Source Select for B0 (ERS0)"]
    #[inline(always)]
    pub fn exs0b(&self) -> Exs0bR {
        Exs0bR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Event Source Select for A1 (ERS1)"]
    #[inline(always)]
    pub fn exs1a(&self) -> Exs1aR {
        Exs1aR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Event Source Select for B1 (ERS1)"]
    #[inline(always)]
    pub fn exs1b(&self) -> Exs1bR {
        Exs1bR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Event Source Select for A2 (ERS2)"]
    #[inline(always)]
    pub fn exs2a(&self) -> Exs2aR {
        Exs2aR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Event Source Select for B2 (ERS2)"]
    #[inline(always)]
    pub fn exs2b(&self) -> Exs2bR {
        Exs2bR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Event Source Select for A3 (ERS3)"]
    #[inline(always)]
    pub fn exs3a(&self) -> Exs3aR {
        Exs3aR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Event Source Select for B3 (ERS3)"]
    #[inline(always)]
    pub fn exs3b(&self) -> Exs3bR {
        Exs3bR::new(((self.bits >> 14) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Event Source Select for A0 (ERS0)"]
    #[inline(always)]
    #[must_use]
    pub fn exs0a(&mut self) -> Exs0aW<ExiselSpec> {
        Exs0aW::new(self, 0)
    }
    #[doc = "Bits 2:3 - Event Source Select for B0 (ERS0)"]
    #[inline(always)]
    #[must_use]
    pub fn exs0b(&mut self) -> Exs0bW<ExiselSpec> {
        Exs0bW::new(self, 2)
    }
    #[doc = "Bits 4:5 - Event Source Select for A1 (ERS1)"]
    #[inline(always)]
    #[must_use]
    pub fn exs1a(&mut self) -> Exs1aW<ExiselSpec> {
        Exs1aW::new(self, 4)
    }
    #[doc = "Bits 6:7 - Event Source Select for B1 (ERS1)"]
    #[inline(always)]
    #[must_use]
    pub fn exs1b(&mut self) -> Exs1bW<ExiselSpec> {
        Exs1bW::new(self, 6)
    }
    #[doc = "Bits 8:9 - Event Source Select for A2 (ERS2)"]
    #[inline(always)]
    #[must_use]
    pub fn exs2a(&mut self) -> Exs2aW<ExiselSpec> {
        Exs2aW::new(self, 8)
    }
    #[doc = "Bits 10:11 - Event Source Select for B2 (ERS2)"]
    #[inline(always)]
    #[must_use]
    pub fn exs2b(&mut self) -> Exs2bW<ExiselSpec> {
        Exs2bW::new(self, 10)
    }
    #[doc = "Bits 12:13 - Event Source Select for A3 (ERS3)"]
    #[inline(always)]
    #[must_use]
    pub fn exs3a(&mut self) -> Exs3aW<ExiselSpec> {
        Exs3aW::new(self, 12)
    }
    #[doc = "Bits 14:15 - Event Source Select for B3 (ERS3)"]
    #[inline(always)]
    #[must_use]
    pub fn exs3b(&mut self) -> Exs3bW<ExiselSpec> {
        Exs3bW::new(self, 14)
    }
}
#[doc = "Event Input Select\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`exisel::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`exisel::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ExiselSpec;
impl crate::RegisterSpec for ExiselSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`exisel::R`](R) reader structure"]
impl crate::Readable for ExiselSpec {}
#[doc = "`write(|w| ..)` method takes [`exisel::W`](W) writer structure"]
impl crate::Writable for ExiselSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EXISEL to value 0"]
impl crate::Resettable for ExiselSpec {
    const RESET_VALUE: u32 = 0;
}
