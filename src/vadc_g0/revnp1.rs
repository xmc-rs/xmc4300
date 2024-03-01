#[doc = "Register `REVNP1` reader"]
pub type R = crate::R<Revnp1Spec>;
#[doc = "Register `REVNP1` writer"]
pub type W = crate::W<Revnp1Spec>;
#[doc = "Service Request Node Pointer Result Event i\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Rev8np {
    #[doc = "0: Select service request line 0 of group x"]
    Value1 = 0,
    #[doc = "3: Select service request line 3 of group x"]
    Value2 = 3,
    #[doc = "4: Select shared service request line 0"]
    Value3 = 4,
    #[doc = "7: Select shared service request line 3"]
    Value4 = 7,
}
impl From<Rev8np> for u8 {
    #[inline(always)]
    fn from(variant: Rev8np) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Rev8np {
    type Ux = u8;
}
#[doc = "Field `REV8NP` reader - Service Request Node Pointer Result Event i"]
pub type Rev8npR = crate::FieldReader<Rev8np>;
impl Rev8npR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Rev8np> {
        match self.bits {
            0 => Some(Rev8np::Value1),
            3 => Some(Rev8np::Value2),
            4 => Some(Rev8np::Value3),
            7 => Some(Rev8np::Value4),
            _ => None,
        }
    }
    #[doc = "Select service request line 0 of group x"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Rev8np::Value1
    }
    #[doc = "Select service request line 3 of group x"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Rev8np::Value2
    }
    #[doc = "Select shared service request line 0"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == Rev8np::Value3
    }
    #[doc = "Select shared service request line 3"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == Rev8np::Value4
    }
}
#[doc = "Field `REV8NP` writer - Service Request Node Pointer Result Event i"]
pub type Rev8npW<'a, REG> = crate::FieldWriter<'a, REG, 4, Rev8np>;
impl<'a, REG> Rev8npW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Select service request line 0 of group x"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Rev8np::Value1)
    }
    #[doc = "Select service request line 3 of group x"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Rev8np::Value2)
    }
    #[doc = "Select shared service request line 0"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(Rev8np::Value3)
    }
    #[doc = "Select shared service request line 3"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(Rev8np::Value4)
    }
}
#[doc = "Service Request Node Pointer Result Event i\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Rev9np {
    #[doc = "0: Select service request line 0 of group x"]
    Value1 = 0,
    #[doc = "3: Select service request line 3 of group x"]
    Value2 = 3,
    #[doc = "4: Select shared service request line 0"]
    Value3 = 4,
    #[doc = "7: Select shared service request line 3"]
    Value4 = 7,
}
impl From<Rev9np> for u8 {
    #[inline(always)]
    fn from(variant: Rev9np) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Rev9np {
    type Ux = u8;
}
#[doc = "Field `REV9NP` reader - Service Request Node Pointer Result Event i"]
pub type Rev9npR = crate::FieldReader<Rev9np>;
impl Rev9npR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Rev9np> {
        match self.bits {
            0 => Some(Rev9np::Value1),
            3 => Some(Rev9np::Value2),
            4 => Some(Rev9np::Value3),
            7 => Some(Rev9np::Value4),
            _ => None,
        }
    }
    #[doc = "Select service request line 0 of group x"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Rev9np::Value1
    }
    #[doc = "Select service request line 3 of group x"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Rev9np::Value2
    }
    #[doc = "Select shared service request line 0"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == Rev9np::Value3
    }
    #[doc = "Select shared service request line 3"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == Rev9np::Value4
    }
}
#[doc = "Field `REV9NP` writer - Service Request Node Pointer Result Event i"]
pub type Rev9npW<'a, REG> = crate::FieldWriter<'a, REG, 4, Rev9np>;
impl<'a, REG> Rev9npW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Select service request line 0 of group x"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Rev9np::Value1)
    }
    #[doc = "Select service request line 3 of group x"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Rev9np::Value2)
    }
    #[doc = "Select shared service request line 0"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(Rev9np::Value3)
    }
    #[doc = "Select shared service request line 3"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(Rev9np::Value4)
    }
}
#[doc = "Service Request Node Pointer Result Event i\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Rev10np {
    #[doc = "0: Select service request line 0 of group x"]
    Value1 = 0,
    #[doc = "3: Select service request line 3 of group x"]
    Value2 = 3,
    #[doc = "4: Select shared service request line 0"]
    Value3 = 4,
    #[doc = "7: Select shared service request line 3"]
    Value4 = 7,
}
impl From<Rev10np> for u8 {
    #[inline(always)]
    fn from(variant: Rev10np) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Rev10np {
    type Ux = u8;
}
#[doc = "Field `REV10NP` reader - Service Request Node Pointer Result Event i"]
pub type Rev10npR = crate::FieldReader<Rev10np>;
impl Rev10npR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Rev10np> {
        match self.bits {
            0 => Some(Rev10np::Value1),
            3 => Some(Rev10np::Value2),
            4 => Some(Rev10np::Value3),
            7 => Some(Rev10np::Value4),
            _ => None,
        }
    }
    #[doc = "Select service request line 0 of group x"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Rev10np::Value1
    }
    #[doc = "Select service request line 3 of group x"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Rev10np::Value2
    }
    #[doc = "Select shared service request line 0"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == Rev10np::Value3
    }
    #[doc = "Select shared service request line 3"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == Rev10np::Value4
    }
}
#[doc = "Field `REV10NP` writer - Service Request Node Pointer Result Event i"]
pub type Rev10npW<'a, REG> = crate::FieldWriter<'a, REG, 4, Rev10np>;
impl<'a, REG> Rev10npW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Select service request line 0 of group x"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Rev10np::Value1)
    }
    #[doc = "Select service request line 3 of group x"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Rev10np::Value2)
    }
    #[doc = "Select shared service request line 0"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(Rev10np::Value3)
    }
    #[doc = "Select shared service request line 3"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(Rev10np::Value4)
    }
}
#[doc = "Service Request Node Pointer Result Event i\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Rev11np {
    #[doc = "0: Select service request line 0 of group x"]
    Value1 = 0,
    #[doc = "3: Select service request line 3 of group x"]
    Value2 = 3,
    #[doc = "4: Select shared service request line 0"]
    Value3 = 4,
    #[doc = "7: Select shared service request line 3"]
    Value4 = 7,
}
impl From<Rev11np> for u8 {
    #[inline(always)]
    fn from(variant: Rev11np) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Rev11np {
    type Ux = u8;
}
#[doc = "Field `REV11NP` reader - Service Request Node Pointer Result Event i"]
pub type Rev11npR = crate::FieldReader<Rev11np>;
impl Rev11npR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Rev11np> {
        match self.bits {
            0 => Some(Rev11np::Value1),
            3 => Some(Rev11np::Value2),
            4 => Some(Rev11np::Value3),
            7 => Some(Rev11np::Value4),
            _ => None,
        }
    }
    #[doc = "Select service request line 0 of group x"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Rev11np::Value1
    }
    #[doc = "Select service request line 3 of group x"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Rev11np::Value2
    }
    #[doc = "Select shared service request line 0"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == Rev11np::Value3
    }
    #[doc = "Select shared service request line 3"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == Rev11np::Value4
    }
}
#[doc = "Field `REV11NP` writer - Service Request Node Pointer Result Event i"]
pub type Rev11npW<'a, REG> = crate::FieldWriter<'a, REG, 4, Rev11np>;
impl<'a, REG> Rev11npW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Select service request line 0 of group x"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Rev11np::Value1)
    }
    #[doc = "Select service request line 3 of group x"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Rev11np::Value2)
    }
    #[doc = "Select shared service request line 0"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(Rev11np::Value3)
    }
    #[doc = "Select shared service request line 3"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(Rev11np::Value4)
    }
}
#[doc = "Service Request Node Pointer Result Event i\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Rev12np {
    #[doc = "0: Select service request line 0 of group x"]
    Value1 = 0,
    #[doc = "3: Select service request line 3 of group x"]
    Value2 = 3,
    #[doc = "4: Select shared service request line 0"]
    Value3 = 4,
    #[doc = "7: Select shared service request line 3"]
    Value4 = 7,
}
impl From<Rev12np> for u8 {
    #[inline(always)]
    fn from(variant: Rev12np) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Rev12np {
    type Ux = u8;
}
#[doc = "Field `REV12NP` reader - Service Request Node Pointer Result Event i"]
pub type Rev12npR = crate::FieldReader<Rev12np>;
impl Rev12npR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Rev12np> {
        match self.bits {
            0 => Some(Rev12np::Value1),
            3 => Some(Rev12np::Value2),
            4 => Some(Rev12np::Value3),
            7 => Some(Rev12np::Value4),
            _ => None,
        }
    }
    #[doc = "Select service request line 0 of group x"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Rev12np::Value1
    }
    #[doc = "Select service request line 3 of group x"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Rev12np::Value2
    }
    #[doc = "Select shared service request line 0"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == Rev12np::Value3
    }
    #[doc = "Select shared service request line 3"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == Rev12np::Value4
    }
}
#[doc = "Field `REV12NP` writer - Service Request Node Pointer Result Event i"]
pub type Rev12npW<'a, REG> = crate::FieldWriter<'a, REG, 4, Rev12np>;
impl<'a, REG> Rev12npW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Select service request line 0 of group x"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Rev12np::Value1)
    }
    #[doc = "Select service request line 3 of group x"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Rev12np::Value2)
    }
    #[doc = "Select shared service request line 0"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(Rev12np::Value3)
    }
    #[doc = "Select shared service request line 3"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(Rev12np::Value4)
    }
}
#[doc = "Service Request Node Pointer Result Event i\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Rev13np {
    #[doc = "0: Select service request line 0 of group x"]
    Value1 = 0,
    #[doc = "3: Select service request line 3 of group x"]
    Value2 = 3,
    #[doc = "4: Select shared service request line 0"]
    Value3 = 4,
    #[doc = "7: Select shared service request line 3"]
    Value4 = 7,
}
impl From<Rev13np> for u8 {
    #[inline(always)]
    fn from(variant: Rev13np) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Rev13np {
    type Ux = u8;
}
#[doc = "Field `REV13NP` reader - Service Request Node Pointer Result Event i"]
pub type Rev13npR = crate::FieldReader<Rev13np>;
impl Rev13npR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Rev13np> {
        match self.bits {
            0 => Some(Rev13np::Value1),
            3 => Some(Rev13np::Value2),
            4 => Some(Rev13np::Value3),
            7 => Some(Rev13np::Value4),
            _ => None,
        }
    }
    #[doc = "Select service request line 0 of group x"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Rev13np::Value1
    }
    #[doc = "Select service request line 3 of group x"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Rev13np::Value2
    }
    #[doc = "Select shared service request line 0"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == Rev13np::Value3
    }
    #[doc = "Select shared service request line 3"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == Rev13np::Value4
    }
}
#[doc = "Field `REV13NP` writer - Service Request Node Pointer Result Event i"]
pub type Rev13npW<'a, REG> = crate::FieldWriter<'a, REG, 4, Rev13np>;
impl<'a, REG> Rev13npW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Select service request line 0 of group x"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Rev13np::Value1)
    }
    #[doc = "Select service request line 3 of group x"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Rev13np::Value2)
    }
    #[doc = "Select shared service request line 0"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(Rev13np::Value3)
    }
    #[doc = "Select shared service request line 3"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(Rev13np::Value4)
    }
}
#[doc = "Service Request Node Pointer Result Event i\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Rev14np {
    #[doc = "0: Select service request line 0 of group x"]
    Value1 = 0,
    #[doc = "3: Select service request line 3 of group x"]
    Value2 = 3,
    #[doc = "4: Select shared service request line 0"]
    Value3 = 4,
    #[doc = "7: Select shared service request line 3"]
    Value4 = 7,
}
impl From<Rev14np> for u8 {
    #[inline(always)]
    fn from(variant: Rev14np) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Rev14np {
    type Ux = u8;
}
#[doc = "Field `REV14NP` reader - Service Request Node Pointer Result Event i"]
pub type Rev14npR = crate::FieldReader<Rev14np>;
impl Rev14npR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Rev14np> {
        match self.bits {
            0 => Some(Rev14np::Value1),
            3 => Some(Rev14np::Value2),
            4 => Some(Rev14np::Value3),
            7 => Some(Rev14np::Value4),
            _ => None,
        }
    }
    #[doc = "Select service request line 0 of group x"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Rev14np::Value1
    }
    #[doc = "Select service request line 3 of group x"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Rev14np::Value2
    }
    #[doc = "Select shared service request line 0"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == Rev14np::Value3
    }
    #[doc = "Select shared service request line 3"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == Rev14np::Value4
    }
}
#[doc = "Field `REV14NP` writer - Service Request Node Pointer Result Event i"]
pub type Rev14npW<'a, REG> = crate::FieldWriter<'a, REG, 4, Rev14np>;
impl<'a, REG> Rev14npW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Select service request line 0 of group x"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Rev14np::Value1)
    }
    #[doc = "Select service request line 3 of group x"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Rev14np::Value2)
    }
    #[doc = "Select shared service request line 0"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(Rev14np::Value3)
    }
    #[doc = "Select shared service request line 3"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(Rev14np::Value4)
    }
}
#[doc = "Service Request Node Pointer Result Event i\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Rev15np {
    #[doc = "0: Select service request line 0 of group x"]
    Value1 = 0,
    #[doc = "3: Select service request line 3 of group x"]
    Value2 = 3,
    #[doc = "4: Select shared service request line 0"]
    Value3 = 4,
    #[doc = "7: Select shared service request line 3"]
    Value4 = 7,
}
impl From<Rev15np> for u8 {
    #[inline(always)]
    fn from(variant: Rev15np) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Rev15np {
    type Ux = u8;
}
#[doc = "Field `REV15NP` reader - Service Request Node Pointer Result Event i"]
pub type Rev15npR = crate::FieldReader<Rev15np>;
impl Rev15npR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Rev15np> {
        match self.bits {
            0 => Some(Rev15np::Value1),
            3 => Some(Rev15np::Value2),
            4 => Some(Rev15np::Value3),
            7 => Some(Rev15np::Value4),
            _ => None,
        }
    }
    #[doc = "Select service request line 0 of group x"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Rev15np::Value1
    }
    #[doc = "Select service request line 3 of group x"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Rev15np::Value2
    }
    #[doc = "Select shared service request line 0"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == Rev15np::Value3
    }
    #[doc = "Select shared service request line 3"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == Rev15np::Value4
    }
}
#[doc = "Field `REV15NP` writer - Service Request Node Pointer Result Event i"]
pub type Rev15npW<'a, REG> = crate::FieldWriter<'a, REG, 4, Rev15np>;
impl<'a, REG> Rev15npW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Select service request line 0 of group x"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Rev15np::Value1)
    }
    #[doc = "Select service request line 3 of group x"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Rev15np::Value2)
    }
    #[doc = "Select shared service request line 0"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(Rev15np::Value3)
    }
    #[doc = "Select shared service request line 3"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(Rev15np::Value4)
    }
}
impl R {
    #[doc = "Bits 0:3 - Service Request Node Pointer Result Event i"]
    #[inline(always)]
    pub fn rev8np(&self) -> Rev8npR {
        Rev8npR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Service Request Node Pointer Result Event i"]
    #[inline(always)]
    pub fn rev9np(&self) -> Rev9npR {
        Rev9npR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Service Request Node Pointer Result Event i"]
    #[inline(always)]
    pub fn rev10np(&self) -> Rev10npR {
        Rev10npR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Service Request Node Pointer Result Event i"]
    #[inline(always)]
    pub fn rev11np(&self) -> Rev11npR {
        Rev11npR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Service Request Node Pointer Result Event i"]
    #[inline(always)]
    pub fn rev12np(&self) -> Rev12npR {
        Rev12npR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Service Request Node Pointer Result Event i"]
    #[inline(always)]
    pub fn rev13np(&self) -> Rev13npR {
        Rev13npR::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Service Request Node Pointer Result Event i"]
    #[inline(always)]
    pub fn rev14np(&self) -> Rev14npR {
        Rev14npR::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - Service Request Node Pointer Result Event i"]
    #[inline(always)]
    pub fn rev15np(&self) -> Rev15npR {
        Rev15npR::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Service Request Node Pointer Result Event i"]
    #[inline(always)]
    #[must_use]
    pub fn rev8np(&mut self) -> Rev8npW<Revnp1Spec> {
        Rev8npW::new(self, 0)
    }
    #[doc = "Bits 4:7 - Service Request Node Pointer Result Event i"]
    #[inline(always)]
    #[must_use]
    pub fn rev9np(&mut self) -> Rev9npW<Revnp1Spec> {
        Rev9npW::new(self, 4)
    }
    #[doc = "Bits 8:11 - Service Request Node Pointer Result Event i"]
    #[inline(always)]
    #[must_use]
    pub fn rev10np(&mut self) -> Rev10npW<Revnp1Spec> {
        Rev10npW::new(self, 8)
    }
    #[doc = "Bits 12:15 - Service Request Node Pointer Result Event i"]
    #[inline(always)]
    #[must_use]
    pub fn rev11np(&mut self) -> Rev11npW<Revnp1Spec> {
        Rev11npW::new(self, 12)
    }
    #[doc = "Bits 16:19 - Service Request Node Pointer Result Event i"]
    #[inline(always)]
    #[must_use]
    pub fn rev12np(&mut self) -> Rev12npW<Revnp1Spec> {
        Rev12npW::new(self, 16)
    }
    #[doc = "Bits 20:23 - Service Request Node Pointer Result Event i"]
    #[inline(always)]
    #[must_use]
    pub fn rev13np(&mut self) -> Rev13npW<Revnp1Spec> {
        Rev13npW::new(self, 20)
    }
    #[doc = "Bits 24:27 - Service Request Node Pointer Result Event i"]
    #[inline(always)]
    #[must_use]
    pub fn rev14np(&mut self) -> Rev14npW<Revnp1Spec> {
        Rev14npW::new(self, 24)
    }
    #[doc = "Bits 28:31 - Service Request Node Pointer Result Event i"]
    #[inline(always)]
    #[must_use]
    pub fn rev15np(&mut self) -> Rev15npW<Revnp1Spec> {
        Rev15npW::new(self, 28)
    }
}
#[doc = "Result Event Node Pointer Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`revnp1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`revnp1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Revnp1Spec;
impl crate::RegisterSpec for Revnp1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`revnp1::R`](R) reader structure"]
impl crate::Readable for Revnp1Spec {}
#[doc = "`write(|w| ..)` method takes [`revnp1::W`](W) writer structure"]
impl crate::Writable for Revnp1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets REVNP1 to value 0"]
impl crate::Resettable for Revnp1Spec {
    const RESET_VALUE: u32 = 0;
}
