#[doc = "Register `CEVNP0` reader"]
pub type R = crate::R<Cevnp0Spec>;
#[doc = "Register `CEVNP0` writer"]
pub type W = crate::W<Cevnp0Spec>;
#[doc = "Service Request Node Pointer Channel Event i\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cev0np {
    #[doc = "0: Select service request line 0 of group x"]
    Value1 = 0,
    #[doc = "3: Select service request line 3 of group x"]
    Value2 = 3,
    #[doc = "4: Select shared service request line 0"]
    Value3 = 4,
    #[doc = "7: Select shared service request line 3"]
    Value4 = 7,
}
impl From<Cev0np> for u8 {
    #[inline(always)]
    fn from(variant: Cev0np) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cev0np {
    type Ux = u8;
}
#[doc = "Field `CEV0NP` reader - Service Request Node Pointer Channel Event i"]
pub type Cev0npR = crate::FieldReader<Cev0np>;
impl Cev0npR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Cev0np> {
        match self.bits {
            0 => Some(Cev0np::Value1),
            3 => Some(Cev0np::Value2),
            4 => Some(Cev0np::Value3),
            7 => Some(Cev0np::Value4),
            _ => None,
        }
    }
    #[doc = "Select service request line 0 of group x"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Cev0np::Value1
    }
    #[doc = "Select service request line 3 of group x"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Cev0np::Value2
    }
    #[doc = "Select shared service request line 0"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == Cev0np::Value3
    }
    #[doc = "Select shared service request line 3"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == Cev0np::Value4
    }
}
#[doc = "Field `CEV0NP` writer - Service Request Node Pointer Channel Event i"]
pub type Cev0npW<'a, REG> = crate::FieldWriter<'a, REG, 4, Cev0np>;
impl<'a, REG> Cev0npW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Select service request line 0 of group x"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Cev0np::Value1)
    }
    #[doc = "Select service request line 3 of group x"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Cev0np::Value2)
    }
    #[doc = "Select shared service request line 0"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(Cev0np::Value3)
    }
    #[doc = "Select shared service request line 3"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(Cev0np::Value4)
    }
}
#[doc = "Service Request Node Pointer Channel Event i\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cev1np {
    #[doc = "0: Select service request line 0 of group x"]
    Value1 = 0,
    #[doc = "3: Select service request line 3 of group x"]
    Value2 = 3,
    #[doc = "4: Select shared service request line 0"]
    Value3 = 4,
    #[doc = "7: Select shared service request line 3"]
    Value4 = 7,
}
impl From<Cev1np> for u8 {
    #[inline(always)]
    fn from(variant: Cev1np) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cev1np {
    type Ux = u8;
}
#[doc = "Field `CEV1NP` reader - Service Request Node Pointer Channel Event i"]
pub type Cev1npR = crate::FieldReader<Cev1np>;
impl Cev1npR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Cev1np> {
        match self.bits {
            0 => Some(Cev1np::Value1),
            3 => Some(Cev1np::Value2),
            4 => Some(Cev1np::Value3),
            7 => Some(Cev1np::Value4),
            _ => None,
        }
    }
    #[doc = "Select service request line 0 of group x"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Cev1np::Value1
    }
    #[doc = "Select service request line 3 of group x"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Cev1np::Value2
    }
    #[doc = "Select shared service request line 0"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == Cev1np::Value3
    }
    #[doc = "Select shared service request line 3"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == Cev1np::Value4
    }
}
#[doc = "Field `CEV1NP` writer - Service Request Node Pointer Channel Event i"]
pub type Cev1npW<'a, REG> = crate::FieldWriter<'a, REG, 4, Cev1np>;
impl<'a, REG> Cev1npW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Select service request line 0 of group x"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Cev1np::Value1)
    }
    #[doc = "Select service request line 3 of group x"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Cev1np::Value2)
    }
    #[doc = "Select shared service request line 0"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(Cev1np::Value3)
    }
    #[doc = "Select shared service request line 3"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(Cev1np::Value4)
    }
}
#[doc = "Service Request Node Pointer Channel Event i\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cev2np {
    #[doc = "0: Select service request line 0 of group x"]
    Value1 = 0,
    #[doc = "3: Select service request line 3 of group x"]
    Value2 = 3,
    #[doc = "4: Select shared service request line 0"]
    Value3 = 4,
    #[doc = "7: Select shared service request line 3"]
    Value4 = 7,
}
impl From<Cev2np> for u8 {
    #[inline(always)]
    fn from(variant: Cev2np) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cev2np {
    type Ux = u8;
}
#[doc = "Field `CEV2NP` reader - Service Request Node Pointer Channel Event i"]
pub type Cev2npR = crate::FieldReader<Cev2np>;
impl Cev2npR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Cev2np> {
        match self.bits {
            0 => Some(Cev2np::Value1),
            3 => Some(Cev2np::Value2),
            4 => Some(Cev2np::Value3),
            7 => Some(Cev2np::Value4),
            _ => None,
        }
    }
    #[doc = "Select service request line 0 of group x"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Cev2np::Value1
    }
    #[doc = "Select service request line 3 of group x"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Cev2np::Value2
    }
    #[doc = "Select shared service request line 0"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == Cev2np::Value3
    }
    #[doc = "Select shared service request line 3"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == Cev2np::Value4
    }
}
#[doc = "Field `CEV2NP` writer - Service Request Node Pointer Channel Event i"]
pub type Cev2npW<'a, REG> = crate::FieldWriter<'a, REG, 4, Cev2np>;
impl<'a, REG> Cev2npW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Select service request line 0 of group x"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Cev2np::Value1)
    }
    #[doc = "Select service request line 3 of group x"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Cev2np::Value2)
    }
    #[doc = "Select shared service request line 0"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(Cev2np::Value3)
    }
    #[doc = "Select shared service request line 3"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(Cev2np::Value4)
    }
}
#[doc = "Service Request Node Pointer Channel Event i\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cev3np {
    #[doc = "0: Select service request line 0 of group x"]
    Value1 = 0,
    #[doc = "3: Select service request line 3 of group x"]
    Value2 = 3,
    #[doc = "4: Select shared service request line 0"]
    Value3 = 4,
    #[doc = "7: Select shared service request line 3"]
    Value4 = 7,
}
impl From<Cev3np> for u8 {
    #[inline(always)]
    fn from(variant: Cev3np) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cev3np {
    type Ux = u8;
}
#[doc = "Field `CEV3NP` reader - Service Request Node Pointer Channel Event i"]
pub type Cev3npR = crate::FieldReader<Cev3np>;
impl Cev3npR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Cev3np> {
        match self.bits {
            0 => Some(Cev3np::Value1),
            3 => Some(Cev3np::Value2),
            4 => Some(Cev3np::Value3),
            7 => Some(Cev3np::Value4),
            _ => None,
        }
    }
    #[doc = "Select service request line 0 of group x"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Cev3np::Value1
    }
    #[doc = "Select service request line 3 of group x"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Cev3np::Value2
    }
    #[doc = "Select shared service request line 0"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == Cev3np::Value3
    }
    #[doc = "Select shared service request line 3"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == Cev3np::Value4
    }
}
#[doc = "Field `CEV3NP` writer - Service Request Node Pointer Channel Event i"]
pub type Cev3npW<'a, REG> = crate::FieldWriter<'a, REG, 4, Cev3np>;
impl<'a, REG> Cev3npW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Select service request line 0 of group x"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Cev3np::Value1)
    }
    #[doc = "Select service request line 3 of group x"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Cev3np::Value2)
    }
    #[doc = "Select shared service request line 0"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(Cev3np::Value3)
    }
    #[doc = "Select shared service request line 3"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(Cev3np::Value4)
    }
}
#[doc = "Service Request Node Pointer Channel Event i\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cev4np {
    #[doc = "0: Select service request line 0 of group x"]
    Value1 = 0,
    #[doc = "3: Select service request line 3 of group x"]
    Value2 = 3,
    #[doc = "4: Select shared service request line 0"]
    Value3 = 4,
    #[doc = "7: Select shared service request line 3"]
    Value4 = 7,
}
impl From<Cev4np> for u8 {
    #[inline(always)]
    fn from(variant: Cev4np) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cev4np {
    type Ux = u8;
}
#[doc = "Field `CEV4NP` reader - Service Request Node Pointer Channel Event i"]
pub type Cev4npR = crate::FieldReader<Cev4np>;
impl Cev4npR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Cev4np> {
        match self.bits {
            0 => Some(Cev4np::Value1),
            3 => Some(Cev4np::Value2),
            4 => Some(Cev4np::Value3),
            7 => Some(Cev4np::Value4),
            _ => None,
        }
    }
    #[doc = "Select service request line 0 of group x"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Cev4np::Value1
    }
    #[doc = "Select service request line 3 of group x"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Cev4np::Value2
    }
    #[doc = "Select shared service request line 0"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == Cev4np::Value3
    }
    #[doc = "Select shared service request line 3"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == Cev4np::Value4
    }
}
#[doc = "Field `CEV4NP` writer - Service Request Node Pointer Channel Event i"]
pub type Cev4npW<'a, REG> = crate::FieldWriter<'a, REG, 4, Cev4np>;
impl<'a, REG> Cev4npW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Select service request line 0 of group x"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Cev4np::Value1)
    }
    #[doc = "Select service request line 3 of group x"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Cev4np::Value2)
    }
    #[doc = "Select shared service request line 0"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(Cev4np::Value3)
    }
    #[doc = "Select shared service request line 3"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(Cev4np::Value4)
    }
}
#[doc = "Service Request Node Pointer Channel Event i\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cev5np {
    #[doc = "0: Select service request line 0 of group x"]
    Value1 = 0,
    #[doc = "3: Select service request line 3 of group x"]
    Value2 = 3,
    #[doc = "4: Select shared service request line 0"]
    Value3 = 4,
    #[doc = "7: Select shared service request line 3"]
    Value4 = 7,
}
impl From<Cev5np> for u8 {
    #[inline(always)]
    fn from(variant: Cev5np) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cev5np {
    type Ux = u8;
}
#[doc = "Field `CEV5NP` reader - Service Request Node Pointer Channel Event i"]
pub type Cev5npR = crate::FieldReader<Cev5np>;
impl Cev5npR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Cev5np> {
        match self.bits {
            0 => Some(Cev5np::Value1),
            3 => Some(Cev5np::Value2),
            4 => Some(Cev5np::Value3),
            7 => Some(Cev5np::Value4),
            _ => None,
        }
    }
    #[doc = "Select service request line 0 of group x"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Cev5np::Value1
    }
    #[doc = "Select service request line 3 of group x"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Cev5np::Value2
    }
    #[doc = "Select shared service request line 0"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == Cev5np::Value3
    }
    #[doc = "Select shared service request line 3"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == Cev5np::Value4
    }
}
#[doc = "Field `CEV5NP` writer - Service Request Node Pointer Channel Event i"]
pub type Cev5npW<'a, REG> = crate::FieldWriter<'a, REG, 4, Cev5np>;
impl<'a, REG> Cev5npW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Select service request line 0 of group x"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Cev5np::Value1)
    }
    #[doc = "Select service request line 3 of group x"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Cev5np::Value2)
    }
    #[doc = "Select shared service request line 0"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(Cev5np::Value3)
    }
    #[doc = "Select shared service request line 3"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(Cev5np::Value4)
    }
}
#[doc = "Service Request Node Pointer Channel Event i\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cev6np {
    #[doc = "0: Select service request line 0 of group x"]
    Value1 = 0,
    #[doc = "3: Select service request line 3 of group x"]
    Value2 = 3,
    #[doc = "4: Select shared service request line 0"]
    Value3 = 4,
    #[doc = "7: Select shared service request line 3"]
    Value4 = 7,
}
impl From<Cev6np> for u8 {
    #[inline(always)]
    fn from(variant: Cev6np) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cev6np {
    type Ux = u8;
}
#[doc = "Field `CEV6NP` reader - Service Request Node Pointer Channel Event i"]
pub type Cev6npR = crate::FieldReader<Cev6np>;
impl Cev6npR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Cev6np> {
        match self.bits {
            0 => Some(Cev6np::Value1),
            3 => Some(Cev6np::Value2),
            4 => Some(Cev6np::Value3),
            7 => Some(Cev6np::Value4),
            _ => None,
        }
    }
    #[doc = "Select service request line 0 of group x"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Cev6np::Value1
    }
    #[doc = "Select service request line 3 of group x"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Cev6np::Value2
    }
    #[doc = "Select shared service request line 0"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == Cev6np::Value3
    }
    #[doc = "Select shared service request line 3"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == Cev6np::Value4
    }
}
#[doc = "Field `CEV6NP` writer - Service Request Node Pointer Channel Event i"]
pub type Cev6npW<'a, REG> = crate::FieldWriter<'a, REG, 4, Cev6np>;
impl<'a, REG> Cev6npW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Select service request line 0 of group x"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Cev6np::Value1)
    }
    #[doc = "Select service request line 3 of group x"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Cev6np::Value2)
    }
    #[doc = "Select shared service request line 0"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(Cev6np::Value3)
    }
    #[doc = "Select shared service request line 3"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(Cev6np::Value4)
    }
}
#[doc = "Service Request Node Pointer Channel Event i\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cev7np {
    #[doc = "0: Select service request line 0 of group x"]
    Value1 = 0,
    #[doc = "3: Select service request line 3 of group x"]
    Value2 = 3,
    #[doc = "4: Select shared service request line 0"]
    Value3 = 4,
    #[doc = "7: Select shared service request line 3"]
    Value4 = 7,
}
impl From<Cev7np> for u8 {
    #[inline(always)]
    fn from(variant: Cev7np) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cev7np {
    type Ux = u8;
}
#[doc = "Field `CEV7NP` reader - Service Request Node Pointer Channel Event i"]
pub type Cev7npR = crate::FieldReader<Cev7np>;
impl Cev7npR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Cev7np> {
        match self.bits {
            0 => Some(Cev7np::Value1),
            3 => Some(Cev7np::Value2),
            4 => Some(Cev7np::Value3),
            7 => Some(Cev7np::Value4),
            _ => None,
        }
    }
    #[doc = "Select service request line 0 of group x"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Cev7np::Value1
    }
    #[doc = "Select service request line 3 of group x"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Cev7np::Value2
    }
    #[doc = "Select shared service request line 0"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == Cev7np::Value3
    }
    #[doc = "Select shared service request line 3"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == Cev7np::Value4
    }
}
#[doc = "Field `CEV7NP` writer - Service Request Node Pointer Channel Event i"]
pub type Cev7npW<'a, REG> = crate::FieldWriter<'a, REG, 4, Cev7np>;
impl<'a, REG> Cev7npW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Select service request line 0 of group x"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Cev7np::Value1)
    }
    #[doc = "Select service request line 3 of group x"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Cev7np::Value2)
    }
    #[doc = "Select shared service request line 0"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(Cev7np::Value3)
    }
    #[doc = "Select shared service request line 3"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(Cev7np::Value4)
    }
}
impl R {
    #[doc = "Bits 0:3 - Service Request Node Pointer Channel Event i"]
    #[inline(always)]
    pub fn cev0np(&self) -> Cev0npR {
        Cev0npR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Service Request Node Pointer Channel Event i"]
    #[inline(always)]
    pub fn cev1np(&self) -> Cev1npR {
        Cev1npR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Service Request Node Pointer Channel Event i"]
    #[inline(always)]
    pub fn cev2np(&self) -> Cev2npR {
        Cev2npR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Service Request Node Pointer Channel Event i"]
    #[inline(always)]
    pub fn cev3np(&self) -> Cev3npR {
        Cev3npR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Service Request Node Pointer Channel Event i"]
    #[inline(always)]
    pub fn cev4np(&self) -> Cev4npR {
        Cev4npR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Service Request Node Pointer Channel Event i"]
    #[inline(always)]
    pub fn cev5np(&self) -> Cev5npR {
        Cev5npR::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Service Request Node Pointer Channel Event i"]
    #[inline(always)]
    pub fn cev6np(&self) -> Cev6npR {
        Cev6npR::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - Service Request Node Pointer Channel Event i"]
    #[inline(always)]
    pub fn cev7np(&self) -> Cev7npR {
        Cev7npR::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Service Request Node Pointer Channel Event i"]
    #[inline(always)]
    #[must_use]
    pub fn cev0np(&mut self) -> Cev0npW<Cevnp0Spec> {
        Cev0npW::new(self, 0)
    }
    #[doc = "Bits 4:7 - Service Request Node Pointer Channel Event i"]
    #[inline(always)]
    #[must_use]
    pub fn cev1np(&mut self) -> Cev1npW<Cevnp0Spec> {
        Cev1npW::new(self, 4)
    }
    #[doc = "Bits 8:11 - Service Request Node Pointer Channel Event i"]
    #[inline(always)]
    #[must_use]
    pub fn cev2np(&mut self) -> Cev2npW<Cevnp0Spec> {
        Cev2npW::new(self, 8)
    }
    #[doc = "Bits 12:15 - Service Request Node Pointer Channel Event i"]
    #[inline(always)]
    #[must_use]
    pub fn cev3np(&mut self) -> Cev3npW<Cevnp0Spec> {
        Cev3npW::new(self, 12)
    }
    #[doc = "Bits 16:19 - Service Request Node Pointer Channel Event i"]
    #[inline(always)]
    #[must_use]
    pub fn cev4np(&mut self) -> Cev4npW<Cevnp0Spec> {
        Cev4npW::new(self, 16)
    }
    #[doc = "Bits 20:23 - Service Request Node Pointer Channel Event i"]
    #[inline(always)]
    #[must_use]
    pub fn cev5np(&mut self) -> Cev5npW<Cevnp0Spec> {
        Cev5npW::new(self, 20)
    }
    #[doc = "Bits 24:27 - Service Request Node Pointer Channel Event i"]
    #[inline(always)]
    #[must_use]
    pub fn cev6np(&mut self) -> Cev6npW<Cevnp0Spec> {
        Cev6npW::new(self, 24)
    }
    #[doc = "Bits 28:31 - Service Request Node Pointer Channel Event i"]
    #[inline(always)]
    #[must_use]
    pub fn cev7np(&mut self) -> Cev7npW<Cevnp0Spec> {
        Cev7npW::new(self, 28)
    }
}
#[doc = "Channel Event Node Pointer Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cevnp0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cevnp0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cevnp0Spec;
impl crate::RegisterSpec for Cevnp0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cevnp0::R`](R) reader structure"]
impl crate::Readable for Cevnp0Spec {}
#[doc = "`write(|w| ..)` method takes [`cevnp0::W`](W) writer structure"]
impl crate::Writable for Cevnp0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CEVNP0 to value 0"]
impl crate::Resettable for Cevnp0Spec {
    const RESET_VALUE: u32 = 0;
}
