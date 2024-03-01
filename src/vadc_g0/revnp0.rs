#[doc = "Register `REVNP0` reader"]
pub type R = crate::R<Revnp0Spec>;
#[doc = "Register `REVNP0` writer"]
pub type W = crate::W<Revnp0Spec>;
#[doc = "Service Request Node Pointer Result Event i\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Rev0np {
    #[doc = "0: Select service request line 0 of group x"]
    Value1 = 0,
    #[doc = "3: Select service request line 3 of group x"]
    Value2 = 3,
    #[doc = "4: Select shared service request line 0"]
    Value3 = 4,
    #[doc = "7: Select shared service request line 3"]
    Value4 = 7,
}
impl From<Rev0np> for u8 {
    #[inline(always)]
    fn from(variant: Rev0np) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Rev0np {
    type Ux = u8;
}
#[doc = "Field `REV0NP` reader - Service Request Node Pointer Result Event i"]
pub type Rev0npR = crate::FieldReader<Rev0np>;
impl Rev0npR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Rev0np> {
        match self.bits {
            0 => Some(Rev0np::Value1),
            3 => Some(Rev0np::Value2),
            4 => Some(Rev0np::Value3),
            7 => Some(Rev0np::Value4),
            _ => None,
        }
    }
    #[doc = "Select service request line 0 of group x"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Rev0np::Value1
    }
    #[doc = "Select service request line 3 of group x"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Rev0np::Value2
    }
    #[doc = "Select shared service request line 0"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == Rev0np::Value3
    }
    #[doc = "Select shared service request line 3"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == Rev0np::Value4
    }
}
#[doc = "Field `REV0NP` writer - Service Request Node Pointer Result Event i"]
pub type Rev0npW<'a, REG> = crate::FieldWriter<'a, REG, 4, Rev0np>;
impl<'a, REG> Rev0npW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Select service request line 0 of group x"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Rev0np::Value1)
    }
    #[doc = "Select service request line 3 of group x"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Rev0np::Value2)
    }
    #[doc = "Select shared service request line 0"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(Rev0np::Value3)
    }
    #[doc = "Select shared service request line 3"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(Rev0np::Value4)
    }
}
#[doc = "Service Request Node Pointer Result Event i\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Rev1np {
    #[doc = "0: Select service request line 0 of group x"]
    Value1 = 0,
    #[doc = "3: Select service request line 3 of group x"]
    Value2 = 3,
    #[doc = "4: Select shared service request line 0"]
    Value3 = 4,
    #[doc = "7: Select shared service request line 3"]
    Value4 = 7,
}
impl From<Rev1np> for u8 {
    #[inline(always)]
    fn from(variant: Rev1np) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Rev1np {
    type Ux = u8;
}
#[doc = "Field `REV1NP` reader - Service Request Node Pointer Result Event i"]
pub type Rev1npR = crate::FieldReader<Rev1np>;
impl Rev1npR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Rev1np> {
        match self.bits {
            0 => Some(Rev1np::Value1),
            3 => Some(Rev1np::Value2),
            4 => Some(Rev1np::Value3),
            7 => Some(Rev1np::Value4),
            _ => None,
        }
    }
    #[doc = "Select service request line 0 of group x"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Rev1np::Value1
    }
    #[doc = "Select service request line 3 of group x"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Rev1np::Value2
    }
    #[doc = "Select shared service request line 0"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == Rev1np::Value3
    }
    #[doc = "Select shared service request line 3"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == Rev1np::Value4
    }
}
#[doc = "Field `REV1NP` writer - Service Request Node Pointer Result Event i"]
pub type Rev1npW<'a, REG> = crate::FieldWriter<'a, REG, 4, Rev1np>;
impl<'a, REG> Rev1npW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Select service request line 0 of group x"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Rev1np::Value1)
    }
    #[doc = "Select service request line 3 of group x"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Rev1np::Value2)
    }
    #[doc = "Select shared service request line 0"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(Rev1np::Value3)
    }
    #[doc = "Select shared service request line 3"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(Rev1np::Value4)
    }
}
#[doc = "Service Request Node Pointer Result Event i\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Rev2np {
    #[doc = "0: Select service request line 0 of group x"]
    Value1 = 0,
    #[doc = "3: Select service request line 3 of group x"]
    Value2 = 3,
    #[doc = "4: Select shared service request line 0"]
    Value3 = 4,
    #[doc = "7: Select shared service request line 3"]
    Value4 = 7,
}
impl From<Rev2np> for u8 {
    #[inline(always)]
    fn from(variant: Rev2np) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Rev2np {
    type Ux = u8;
}
#[doc = "Field `REV2NP` reader - Service Request Node Pointer Result Event i"]
pub type Rev2npR = crate::FieldReader<Rev2np>;
impl Rev2npR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Rev2np> {
        match self.bits {
            0 => Some(Rev2np::Value1),
            3 => Some(Rev2np::Value2),
            4 => Some(Rev2np::Value3),
            7 => Some(Rev2np::Value4),
            _ => None,
        }
    }
    #[doc = "Select service request line 0 of group x"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Rev2np::Value1
    }
    #[doc = "Select service request line 3 of group x"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Rev2np::Value2
    }
    #[doc = "Select shared service request line 0"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == Rev2np::Value3
    }
    #[doc = "Select shared service request line 3"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == Rev2np::Value4
    }
}
#[doc = "Field `REV2NP` writer - Service Request Node Pointer Result Event i"]
pub type Rev2npW<'a, REG> = crate::FieldWriter<'a, REG, 4, Rev2np>;
impl<'a, REG> Rev2npW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Select service request line 0 of group x"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Rev2np::Value1)
    }
    #[doc = "Select service request line 3 of group x"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Rev2np::Value2)
    }
    #[doc = "Select shared service request line 0"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(Rev2np::Value3)
    }
    #[doc = "Select shared service request line 3"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(Rev2np::Value4)
    }
}
#[doc = "Service Request Node Pointer Result Event i\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Rev3np {
    #[doc = "0: Select service request line 0 of group x"]
    Value1 = 0,
    #[doc = "3: Select service request line 3 of group x"]
    Value2 = 3,
    #[doc = "4: Select shared service request line 0"]
    Value3 = 4,
    #[doc = "7: Select shared service request line 3"]
    Value4 = 7,
}
impl From<Rev3np> for u8 {
    #[inline(always)]
    fn from(variant: Rev3np) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Rev3np {
    type Ux = u8;
}
#[doc = "Field `REV3NP` reader - Service Request Node Pointer Result Event i"]
pub type Rev3npR = crate::FieldReader<Rev3np>;
impl Rev3npR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Rev3np> {
        match self.bits {
            0 => Some(Rev3np::Value1),
            3 => Some(Rev3np::Value2),
            4 => Some(Rev3np::Value3),
            7 => Some(Rev3np::Value4),
            _ => None,
        }
    }
    #[doc = "Select service request line 0 of group x"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Rev3np::Value1
    }
    #[doc = "Select service request line 3 of group x"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Rev3np::Value2
    }
    #[doc = "Select shared service request line 0"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == Rev3np::Value3
    }
    #[doc = "Select shared service request line 3"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == Rev3np::Value4
    }
}
#[doc = "Field `REV3NP` writer - Service Request Node Pointer Result Event i"]
pub type Rev3npW<'a, REG> = crate::FieldWriter<'a, REG, 4, Rev3np>;
impl<'a, REG> Rev3npW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Select service request line 0 of group x"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Rev3np::Value1)
    }
    #[doc = "Select service request line 3 of group x"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Rev3np::Value2)
    }
    #[doc = "Select shared service request line 0"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(Rev3np::Value3)
    }
    #[doc = "Select shared service request line 3"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(Rev3np::Value4)
    }
}
#[doc = "Service Request Node Pointer Result Event i\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Rev4np {
    #[doc = "0: Select service request line 0 of group x"]
    Value1 = 0,
    #[doc = "3: Select service request line 3 of group x"]
    Value2 = 3,
    #[doc = "4: Select shared service request line 0"]
    Value3 = 4,
    #[doc = "7: Select shared service request line 3"]
    Value4 = 7,
}
impl From<Rev4np> for u8 {
    #[inline(always)]
    fn from(variant: Rev4np) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Rev4np {
    type Ux = u8;
}
#[doc = "Field `REV4NP` reader - Service Request Node Pointer Result Event i"]
pub type Rev4npR = crate::FieldReader<Rev4np>;
impl Rev4npR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Rev4np> {
        match self.bits {
            0 => Some(Rev4np::Value1),
            3 => Some(Rev4np::Value2),
            4 => Some(Rev4np::Value3),
            7 => Some(Rev4np::Value4),
            _ => None,
        }
    }
    #[doc = "Select service request line 0 of group x"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Rev4np::Value1
    }
    #[doc = "Select service request line 3 of group x"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Rev4np::Value2
    }
    #[doc = "Select shared service request line 0"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == Rev4np::Value3
    }
    #[doc = "Select shared service request line 3"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == Rev4np::Value4
    }
}
#[doc = "Field `REV4NP` writer - Service Request Node Pointer Result Event i"]
pub type Rev4npW<'a, REG> = crate::FieldWriter<'a, REG, 4, Rev4np>;
impl<'a, REG> Rev4npW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Select service request line 0 of group x"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Rev4np::Value1)
    }
    #[doc = "Select service request line 3 of group x"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Rev4np::Value2)
    }
    #[doc = "Select shared service request line 0"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(Rev4np::Value3)
    }
    #[doc = "Select shared service request line 3"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(Rev4np::Value4)
    }
}
#[doc = "Service Request Node Pointer Result Event i\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Rev5np {
    #[doc = "0: Select service request line 0 of group x"]
    Value1 = 0,
    #[doc = "3: Select service request line 3 of group x"]
    Value2 = 3,
    #[doc = "4: Select shared service request line 0"]
    Value3 = 4,
    #[doc = "7: Select shared service request line 3"]
    Value4 = 7,
}
impl From<Rev5np> for u8 {
    #[inline(always)]
    fn from(variant: Rev5np) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Rev5np {
    type Ux = u8;
}
#[doc = "Field `REV5NP` reader - Service Request Node Pointer Result Event i"]
pub type Rev5npR = crate::FieldReader<Rev5np>;
impl Rev5npR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Rev5np> {
        match self.bits {
            0 => Some(Rev5np::Value1),
            3 => Some(Rev5np::Value2),
            4 => Some(Rev5np::Value3),
            7 => Some(Rev5np::Value4),
            _ => None,
        }
    }
    #[doc = "Select service request line 0 of group x"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Rev5np::Value1
    }
    #[doc = "Select service request line 3 of group x"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Rev5np::Value2
    }
    #[doc = "Select shared service request line 0"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == Rev5np::Value3
    }
    #[doc = "Select shared service request line 3"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == Rev5np::Value4
    }
}
#[doc = "Field `REV5NP` writer - Service Request Node Pointer Result Event i"]
pub type Rev5npW<'a, REG> = crate::FieldWriter<'a, REG, 4, Rev5np>;
impl<'a, REG> Rev5npW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Select service request line 0 of group x"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Rev5np::Value1)
    }
    #[doc = "Select service request line 3 of group x"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Rev5np::Value2)
    }
    #[doc = "Select shared service request line 0"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(Rev5np::Value3)
    }
    #[doc = "Select shared service request line 3"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(Rev5np::Value4)
    }
}
#[doc = "Service Request Node Pointer Result Event i\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Rev6np {
    #[doc = "0: Select service request line 0 of group x"]
    Value1 = 0,
    #[doc = "3: Select service request line 3 of group x"]
    Value2 = 3,
    #[doc = "4: Select shared service request line 0"]
    Value3 = 4,
    #[doc = "7: Select shared service request line 3"]
    Value4 = 7,
}
impl From<Rev6np> for u8 {
    #[inline(always)]
    fn from(variant: Rev6np) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Rev6np {
    type Ux = u8;
}
#[doc = "Field `REV6NP` reader - Service Request Node Pointer Result Event i"]
pub type Rev6npR = crate::FieldReader<Rev6np>;
impl Rev6npR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Rev6np> {
        match self.bits {
            0 => Some(Rev6np::Value1),
            3 => Some(Rev6np::Value2),
            4 => Some(Rev6np::Value3),
            7 => Some(Rev6np::Value4),
            _ => None,
        }
    }
    #[doc = "Select service request line 0 of group x"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Rev6np::Value1
    }
    #[doc = "Select service request line 3 of group x"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Rev6np::Value2
    }
    #[doc = "Select shared service request line 0"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == Rev6np::Value3
    }
    #[doc = "Select shared service request line 3"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == Rev6np::Value4
    }
}
#[doc = "Field `REV6NP` writer - Service Request Node Pointer Result Event i"]
pub type Rev6npW<'a, REG> = crate::FieldWriter<'a, REG, 4, Rev6np>;
impl<'a, REG> Rev6npW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Select service request line 0 of group x"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Rev6np::Value1)
    }
    #[doc = "Select service request line 3 of group x"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Rev6np::Value2)
    }
    #[doc = "Select shared service request line 0"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(Rev6np::Value3)
    }
    #[doc = "Select shared service request line 3"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(Rev6np::Value4)
    }
}
#[doc = "Service Request Node Pointer Result Event i\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Rev7np {
    #[doc = "0: Select service request line 0 of group x"]
    Value1 = 0,
    #[doc = "3: Select service request line 3 of group x"]
    Value2 = 3,
    #[doc = "4: Select shared service request line 0"]
    Value3 = 4,
    #[doc = "7: Select shared service request line 3"]
    Value4 = 7,
}
impl From<Rev7np> for u8 {
    #[inline(always)]
    fn from(variant: Rev7np) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Rev7np {
    type Ux = u8;
}
#[doc = "Field `REV7NP` reader - Service Request Node Pointer Result Event i"]
pub type Rev7npR = crate::FieldReader<Rev7np>;
impl Rev7npR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Rev7np> {
        match self.bits {
            0 => Some(Rev7np::Value1),
            3 => Some(Rev7np::Value2),
            4 => Some(Rev7np::Value3),
            7 => Some(Rev7np::Value4),
            _ => None,
        }
    }
    #[doc = "Select service request line 0 of group x"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Rev7np::Value1
    }
    #[doc = "Select service request line 3 of group x"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Rev7np::Value2
    }
    #[doc = "Select shared service request line 0"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == Rev7np::Value3
    }
    #[doc = "Select shared service request line 3"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == Rev7np::Value4
    }
}
#[doc = "Field `REV7NP` writer - Service Request Node Pointer Result Event i"]
pub type Rev7npW<'a, REG> = crate::FieldWriter<'a, REG, 4, Rev7np>;
impl<'a, REG> Rev7npW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Select service request line 0 of group x"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Rev7np::Value1)
    }
    #[doc = "Select service request line 3 of group x"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Rev7np::Value2)
    }
    #[doc = "Select shared service request line 0"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(Rev7np::Value3)
    }
    #[doc = "Select shared service request line 3"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(Rev7np::Value4)
    }
}
impl R {
    #[doc = "Bits 0:3 - Service Request Node Pointer Result Event i"]
    #[inline(always)]
    pub fn rev0np(&self) -> Rev0npR {
        Rev0npR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Service Request Node Pointer Result Event i"]
    #[inline(always)]
    pub fn rev1np(&self) -> Rev1npR {
        Rev1npR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Service Request Node Pointer Result Event i"]
    #[inline(always)]
    pub fn rev2np(&self) -> Rev2npR {
        Rev2npR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Service Request Node Pointer Result Event i"]
    #[inline(always)]
    pub fn rev3np(&self) -> Rev3npR {
        Rev3npR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Service Request Node Pointer Result Event i"]
    #[inline(always)]
    pub fn rev4np(&self) -> Rev4npR {
        Rev4npR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Service Request Node Pointer Result Event i"]
    #[inline(always)]
    pub fn rev5np(&self) -> Rev5npR {
        Rev5npR::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Service Request Node Pointer Result Event i"]
    #[inline(always)]
    pub fn rev6np(&self) -> Rev6npR {
        Rev6npR::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - Service Request Node Pointer Result Event i"]
    #[inline(always)]
    pub fn rev7np(&self) -> Rev7npR {
        Rev7npR::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Service Request Node Pointer Result Event i"]
    #[inline(always)]
    #[must_use]
    pub fn rev0np(&mut self) -> Rev0npW<Revnp0Spec> {
        Rev0npW::new(self, 0)
    }
    #[doc = "Bits 4:7 - Service Request Node Pointer Result Event i"]
    #[inline(always)]
    #[must_use]
    pub fn rev1np(&mut self) -> Rev1npW<Revnp0Spec> {
        Rev1npW::new(self, 4)
    }
    #[doc = "Bits 8:11 - Service Request Node Pointer Result Event i"]
    #[inline(always)]
    #[must_use]
    pub fn rev2np(&mut self) -> Rev2npW<Revnp0Spec> {
        Rev2npW::new(self, 8)
    }
    #[doc = "Bits 12:15 - Service Request Node Pointer Result Event i"]
    #[inline(always)]
    #[must_use]
    pub fn rev3np(&mut self) -> Rev3npW<Revnp0Spec> {
        Rev3npW::new(self, 12)
    }
    #[doc = "Bits 16:19 - Service Request Node Pointer Result Event i"]
    #[inline(always)]
    #[must_use]
    pub fn rev4np(&mut self) -> Rev4npW<Revnp0Spec> {
        Rev4npW::new(self, 16)
    }
    #[doc = "Bits 20:23 - Service Request Node Pointer Result Event i"]
    #[inline(always)]
    #[must_use]
    pub fn rev5np(&mut self) -> Rev5npW<Revnp0Spec> {
        Rev5npW::new(self, 20)
    }
    #[doc = "Bits 24:27 - Service Request Node Pointer Result Event i"]
    #[inline(always)]
    #[must_use]
    pub fn rev6np(&mut self) -> Rev6npW<Revnp0Spec> {
        Rev6npW::new(self, 24)
    }
    #[doc = "Bits 28:31 - Service Request Node Pointer Result Event i"]
    #[inline(always)]
    #[must_use]
    pub fn rev7np(&mut self) -> Rev7npW<Revnp0Spec> {
        Rev7npW::new(self, 28)
    }
}
#[doc = "Result Event Node Pointer Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`revnp0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`revnp0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Revnp0Spec;
impl crate::RegisterSpec for Revnp0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`revnp0::R`](R) reader structure"]
impl crate::Readable for Revnp0Spec {}
#[doc = "`write(|w| ..)` method takes [`revnp0::W`](W) writer structure"]
impl crate::Writable for Revnp0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets REVNP0 to value 0"]
impl crate::Resettable for Revnp0Spec {
    const RESET_VALUE: u32 = 0;
}
