#[doc = "Register `CEVNP0` reader"]
pub type R = crate::R<CEVNP0_SPEC>;
#[doc = "Register `CEVNP0` writer"]
pub type W = crate::W<CEVNP0_SPEC>;
#[doc = "Service Request Node Pointer Channel Event i\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CEV0NP_A {
    #[doc = "0: Select service request line 0 of group x"]
    VALUE1 = 0,
    #[doc = "3: Select service request line 3 of group x"]
    VALUE2 = 3,
    #[doc = "4: Select shared service request line 0"]
    VALUE3 = 4,
    #[doc = "7: Select shared service request line 3"]
    VALUE4 = 7,
}
impl From<CEV0NP_A> for u8 {
    #[inline(always)]
    fn from(variant: CEV0NP_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CEV0NP_A {
    type Ux = u8;
}
impl crate::IsEnum for CEV0NP_A {}
#[doc = "Field `CEV0NP` reader - Service Request Node Pointer Channel Event i"]
pub type CEV0NP_R = crate::FieldReader<CEV0NP_A>;
impl CEV0NP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<CEV0NP_A> {
        match self.bits {
            0 => Some(CEV0NP_A::VALUE1),
            3 => Some(CEV0NP_A::VALUE2),
            4 => Some(CEV0NP_A::VALUE3),
            7 => Some(CEV0NP_A::VALUE4),
            _ => None,
        }
    }
    #[doc = "Select service request line 0 of group x"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CEV0NP_A::VALUE1
    }
    #[doc = "Select service request line 3 of group x"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CEV0NP_A::VALUE2
    }
    #[doc = "Select shared service request line 0"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == CEV0NP_A::VALUE3
    }
    #[doc = "Select shared service request line 3"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == CEV0NP_A::VALUE4
    }
}
#[doc = "Field `CEV0NP` writer - Service Request Node Pointer Channel Event i"]
pub type CEV0NP_W<'a, REG> = crate::FieldWriter<'a, REG, 4, CEV0NP_A>;
impl<'a, REG> CEV0NP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Select service request line 0 of group x"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(CEV0NP_A::VALUE1)
    }
    #[doc = "Select service request line 3 of group x"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(CEV0NP_A::VALUE2)
    }
    #[doc = "Select shared service request line 0"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(CEV0NP_A::VALUE3)
    }
    #[doc = "Select shared service request line 3"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(CEV0NP_A::VALUE4)
    }
}
#[doc = "Service Request Node Pointer Channel Event i\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CEV1NP_A {
    #[doc = "0: Select service request line 0 of group x"]
    VALUE1 = 0,
    #[doc = "3: Select service request line 3 of group x"]
    VALUE2 = 3,
    #[doc = "4: Select shared service request line 0"]
    VALUE3 = 4,
    #[doc = "7: Select shared service request line 3"]
    VALUE4 = 7,
}
impl From<CEV1NP_A> for u8 {
    #[inline(always)]
    fn from(variant: CEV1NP_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CEV1NP_A {
    type Ux = u8;
}
impl crate::IsEnum for CEV1NP_A {}
#[doc = "Field `CEV1NP` reader - Service Request Node Pointer Channel Event i"]
pub type CEV1NP_R = crate::FieldReader<CEV1NP_A>;
impl CEV1NP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<CEV1NP_A> {
        match self.bits {
            0 => Some(CEV1NP_A::VALUE1),
            3 => Some(CEV1NP_A::VALUE2),
            4 => Some(CEV1NP_A::VALUE3),
            7 => Some(CEV1NP_A::VALUE4),
            _ => None,
        }
    }
    #[doc = "Select service request line 0 of group x"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CEV1NP_A::VALUE1
    }
    #[doc = "Select service request line 3 of group x"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CEV1NP_A::VALUE2
    }
    #[doc = "Select shared service request line 0"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == CEV1NP_A::VALUE3
    }
    #[doc = "Select shared service request line 3"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == CEV1NP_A::VALUE4
    }
}
#[doc = "Field `CEV1NP` writer - Service Request Node Pointer Channel Event i"]
pub type CEV1NP_W<'a, REG> = crate::FieldWriter<'a, REG, 4, CEV1NP_A>;
impl<'a, REG> CEV1NP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Select service request line 0 of group x"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(CEV1NP_A::VALUE1)
    }
    #[doc = "Select service request line 3 of group x"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(CEV1NP_A::VALUE2)
    }
    #[doc = "Select shared service request line 0"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(CEV1NP_A::VALUE3)
    }
    #[doc = "Select shared service request line 3"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(CEV1NP_A::VALUE4)
    }
}
#[doc = "Service Request Node Pointer Channel Event i\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CEV2NP_A {
    #[doc = "0: Select service request line 0 of group x"]
    VALUE1 = 0,
    #[doc = "3: Select service request line 3 of group x"]
    VALUE2 = 3,
    #[doc = "4: Select shared service request line 0"]
    VALUE3 = 4,
    #[doc = "7: Select shared service request line 3"]
    VALUE4 = 7,
}
impl From<CEV2NP_A> for u8 {
    #[inline(always)]
    fn from(variant: CEV2NP_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CEV2NP_A {
    type Ux = u8;
}
impl crate::IsEnum for CEV2NP_A {}
#[doc = "Field `CEV2NP` reader - Service Request Node Pointer Channel Event i"]
pub type CEV2NP_R = crate::FieldReader<CEV2NP_A>;
impl CEV2NP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<CEV2NP_A> {
        match self.bits {
            0 => Some(CEV2NP_A::VALUE1),
            3 => Some(CEV2NP_A::VALUE2),
            4 => Some(CEV2NP_A::VALUE3),
            7 => Some(CEV2NP_A::VALUE4),
            _ => None,
        }
    }
    #[doc = "Select service request line 0 of group x"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CEV2NP_A::VALUE1
    }
    #[doc = "Select service request line 3 of group x"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CEV2NP_A::VALUE2
    }
    #[doc = "Select shared service request line 0"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == CEV2NP_A::VALUE3
    }
    #[doc = "Select shared service request line 3"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == CEV2NP_A::VALUE4
    }
}
#[doc = "Field `CEV2NP` writer - Service Request Node Pointer Channel Event i"]
pub type CEV2NP_W<'a, REG> = crate::FieldWriter<'a, REG, 4, CEV2NP_A>;
impl<'a, REG> CEV2NP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Select service request line 0 of group x"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(CEV2NP_A::VALUE1)
    }
    #[doc = "Select service request line 3 of group x"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(CEV2NP_A::VALUE2)
    }
    #[doc = "Select shared service request line 0"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(CEV2NP_A::VALUE3)
    }
    #[doc = "Select shared service request line 3"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(CEV2NP_A::VALUE4)
    }
}
#[doc = "Service Request Node Pointer Channel Event i\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CEV3NP_A {
    #[doc = "0: Select service request line 0 of group x"]
    VALUE1 = 0,
    #[doc = "3: Select service request line 3 of group x"]
    VALUE2 = 3,
    #[doc = "4: Select shared service request line 0"]
    VALUE3 = 4,
    #[doc = "7: Select shared service request line 3"]
    VALUE4 = 7,
}
impl From<CEV3NP_A> for u8 {
    #[inline(always)]
    fn from(variant: CEV3NP_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CEV3NP_A {
    type Ux = u8;
}
impl crate::IsEnum for CEV3NP_A {}
#[doc = "Field `CEV3NP` reader - Service Request Node Pointer Channel Event i"]
pub type CEV3NP_R = crate::FieldReader<CEV3NP_A>;
impl CEV3NP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<CEV3NP_A> {
        match self.bits {
            0 => Some(CEV3NP_A::VALUE1),
            3 => Some(CEV3NP_A::VALUE2),
            4 => Some(CEV3NP_A::VALUE3),
            7 => Some(CEV3NP_A::VALUE4),
            _ => None,
        }
    }
    #[doc = "Select service request line 0 of group x"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CEV3NP_A::VALUE1
    }
    #[doc = "Select service request line 3 of group x"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CEV3NP_A::VALUE2
    }
    #[doc = "Select shared service request line 0"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == CEV3NP_A::VALUE3
    }
    #[doc = "Select shared service request line 3"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == CEV3NP_A::VALUE4
    }
}
#[doc = "Field `CEV3NP` writer - Service Request Node Pointer Channel Event i"]
pub type CEV3NP_W<'a, REG> = crate::FieldWriter<'a, REG, 4, CEV3NP_A>;
impl<'a, REG> CEV3NP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Select service request line 0 of group x"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(CEV3NP_A::VALUE1)
    }
    #[doc = "Select service request line 3 of group x"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(CEV3NP_A::VALUE2)
    }
    #[doc = "Select shared service request line 0"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(CEV3NP_A::VALUE3)
    }
    #[doc = "Select shared service request line 3"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(CEV3NP_A::VALUE4)
    }
}
#[doc = "Service Request Node Pointer Channel Event i\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CEV4NP_A {
    #[doc = "0: Select service request line 0 of group x"]
    VALUE1 = 0,
    #[doc = "3: Select service request line 3 of group x"]
    VALUE2 = 3,
    #[doc = "4: Select shared service request line 0"]
    VALUE3 = 4,
    #[doc = "7: Select shared service request line 3"]
    VALUE4 = 7,
}
impl From<CEV4NP_A> for u8 {
    #[inline(always)]
    fn from(variant: CEV4NP_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CEV4NP_A {
    type Ux = u8;
}
impl crate::IsEnum for CEV4NP_A {}
#[doc = "Field `CEV4NP` reader - Service Request Node Pointer Channel Event i"]
pub type CEV4NP_R = crate::FieldReader<CEV4NP_A>;
impl CEV4NP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<CEV4NP_A> {
        match self.bits {
            0 => Some(CEV4NP_A::VALUE1),
            3 => Some(CEV4NP_A::VALUE2),
            4 => Some(CEV4NP_A::VALUE3),
            7 => Some(CEV4NP_A::VALUE4),
            _ => None,
        }
    }
    #[doc = "Select service request line 0 of group x"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CEV4NP_A::VALUE1
    }
    #[doc = "Select service request line 3 of group x"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CEV4NP_A::VALUE2
    }
    #[doc = "Select shared service request line 0"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == CEV4NP_A::VALUE3
    }
    #[doc = "Select shared service request line 3"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == CEV4NP_A::VALUE4
    }
}
#[doc = "Field `CEV4NP` writer - Service Request Node Pointer Channel Event i"]
pub type CEV4NP_W<'a, REG> = crate::FieldWriter<'a, REG, 4, CEV4NP_A>;
impl<'a, REG> CEV4NP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Select service request line 0 of group x"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(CEV4NP_A::VALUE1)
    }
    #[doc = "Select service request line 3 of group x"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(CEV4NP_A::VALUE2)
    }
    #[doc = "Select shared service request line 0"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(CEV4NP_A::VALUE3)
    }
    #[doc = "Select shared service request line 3"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(CEV4NP_A::VALUE4)
    }
}
#[doc = "Service Request Node Pointer Channel Event i\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CEV5NP_A {
    #[doc = "0: Select service request line 0 of group x"]
    VALUE1 = 0,
    #[doc = "3: Select service request line 3 of group x"]
    VALUE2 = 3,
    #[doc = "4: Select shared service request line 0"]
    VALUE3 = 4,
    #[doc = "7: Select shared service request line 3"]
    VALUE4 = 7,
}
impl From<CEV5NP_A> for u8 {
    #[inline(always)]
    fn from(variant: CEV5NP_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CEV5NP_A {
    type Ux = u8;
}
impl crate::IsEnum for CEV5NP_A {}
#[doc = "Field `CEV5NP` reader - Service Request Node Pointer Channel Event i"]
pub type CEV5NP_R = crate::FieldReader<CEV5NP_A>;
impl CEV5NP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<CEV5NP_A> {
        match self.bits {
            0 => Some(CEV5NP_A::VALUE1),
            3 => Some(CEV5NP_A::VALUE2),
            4 => Some(CEV5NP_A::VALUE3),
            7 => Some(CEV5NP_A::VALUE4),
            _ => None,
        }
    }
    #[doc = "Select service request line 0 of group x"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CEV5NP_A::VALUE1
    }
    #[doc = "Select service request line 3 of group x"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CEV5NP_A::VALUE2
    }
    #[doc = "Select shared service request line 0"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == CEV5NP_A::VALUE3
    }
    #[doc = "Select shared service request line 3"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == CEV5NP_A::VALUE4
    }
}
#[doc = "Field `CEV5NP` writer - Service Request Node Pointer Channel Event i"]
pub type CEV5NP_W<'a, REG> = crate::FieldWriter<'a, REG, 4, CEV5NP_A>;
impl<'a, REG> CEV5NP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Select service request line 0 of group x"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(CEV5NP_A::VALUE1)
    }
    #[doc = "Select service request line 3 of group x"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(CEV5NP_A::VALUE2)
    }
    #[doc = "Select shared service request line 0"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(CEV5NP_A::VALUE3)
    }
    #[doc = "Select shared service request line 3"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(CEV5NP_A::VALUE4)
    }
}
#[doc = "Service Request Node Pointer Channel Event i\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CEV6NP_A {
    #[doc = "0: Select service request line 0 of group x"]
    VALUE1 = 0,
    #[doc = "3: Select service request line 3 of group x"]
    VALUE2 = 3,
    #[doc = "4: Select shared service request line 0"]
    VALUE3 = 4,
    #[doc = "7: Select shared service request line 3"]
    VALUE4 = 7,
}
impl From<CEV6NP_A> for u8 {
    #[inline(always)]
    fn from(variant: CEV6NP_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CEV6NP_A {
    type Ux = u8;
}
impl crate::IsEnum for CEV6NP_A {}
#[doc = "Field `CEV6NP` reader - Service Request Node Pointer Channel Event i"]
pub type CEV6NP_R = crate::FieldReader<CEV6NP_A>;
impl CEV6NP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<CEV6NP_A> {
        match self.bits {
            0 => Some(CEV6NP_A::VALUE1),
            3 => Some(CEV6NP_A::VALUE2),
            4 => Some(CEV6NP_A::VALUE3),
            7 => Some(CEV6NP_A::VALUE4),
            _ => None,
        }
    }
    #[doc = "Select service request line 0 of group x"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CEV6NP_A::VALUE1
    }
    #[doc = "Select service request line 3 of group x"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CEV6NP_A::VALUE2
    }
    #[doc = "Select shared service request line 0"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == CEV6NP_A::VALUE3
    }
    #[doc = "Select shared service request line 3"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == CEV6NP_A::VALUE4
    }
}
#[doc = "Field `CEV6NP` writer - Service Request Node Pointer Channel Event i"]
pub type CEV6NP_W<'a, REG> = crate::FieldWriter<'a, REG, 4, CEV6NP_A>;
impl<'a, REG> CEV6NP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Select service request line 0 of group x"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(CEV6NP_A::VALUE1)
    }
    #[doc = "Select service request line 3 of group x"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(CEV6NP_A::VALUE2)
    }
    #[doc = "Select shared service request line 0"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(CEV6NP_A::VALUE3)
    }
    #[doc = "Select shared service request line 3"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(CEV6NP_A::VALUE4)
    }
}
#[doc = "Service Request Node Pointer Channel Event i\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CEV7NP_A {
    #[doc = "0: Select service request line 0 of group x"]
    VALUE1 = 0,
    #[doc = "3: Select service request line 3 of group x"]
    VALUE2 = 3,
    #[doc = "4: Select shared service request line 0"]
    VALUE3 = 4,
    #[doc = "7: Select shared service request line 3"]
    VALUE4 = 7,
}
impl From<CEV7NP_A> for u8 {
    #[inline(always)]
    fn from(variant: CEV7NP_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CEV7NP_A {
    type Ux = u8;
}
impl crate::IsEnum for CEV7NP_A {}
#[doc = "Field `CEV7NP` reader - Service Request Node Pointer Channel Event i"]
pub type CEV7NP_R = crate::FieldReader<CEV7NP_A>;
impl CEV7NP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<CEV7NP_A> {
        match self.bits {
            0 => Some(CEV7NP_A::VALUE1),
            3 => Some(CEV7NP_A::VALUE2),
            4 => Some(CEV7NP_A::VALUE3),
            7 => Some(CEV7NP_A::VALUE4),
            _ => None,
        }
    }
    #[doc = "Select service request line 0 of group x"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CEV7NP_A::VALUE1
    }
    #[doc = "Select service request line 3 of group x"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CEV7NP_A::VALUE2
    }
    #[doc = "Select shared service request line 0"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == CEV7NP_A::VALUE3
    }
    #[doc = "Select shared service request line 3"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == CEV7NP_A::VALUE4
    }
}
#[doc = "Field `CEV7NP` writer - Service Request Node Pointer Channel Event i"]
pub type CEV7NP_W<'a, REG> = crate::FieldWriter<'a, REG, 4, CEV7NP_A>;
impl<'a, REG> CEV7NP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Select service request line 0 of group x"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(CEV7NP_A::VALUE1)
    }
    #[doc = "Select service request line 3 of group x"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(CEV7NP_A::VALUE2)
    }
    #[doc = "Select shared service request line 0"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(CEV7NP_A::VALUE3)
    }
    #[doc = "Select shared service request line 3"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(CEV7NP_A::VALUE4)
    }
}
impl R {
    #[doc = "Bits 0:3 - Service Request Node Pointer Channel Event i"]
    #[inline(always)]
    pub fn cev0np(&self) -> CEV0NP_R {
        CEV0NP_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Service Request Node Pointer Channel Event i"]
    #[inline(always)]
    pub fn cev1np(&self) -> CEV1NP_R {
        CEV1NP_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Service Request Node Pointer Channel Event i"]
    #[inline(always)]
    pub fn cev2np(&self) -> CEV2NP_R {
        CEV2NP_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Service Request Node Pointer Channel Event i"]
    #[inline(always)]
    pub fn cev3np(&self) -> CEV3NP_R {
        CEV3NP_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Service Request Node Pointer Channel Event i"]
    #[inline(always)]
    pub fn cev4np(&self) -> CEV4NP_R {
        CEV4NP_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Service Request Node Pointer Channel Event i"]
    #[inline(always)]
    pub fn cev5np(&self) -> CEV5NP_R {
        CEV5NP_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Service Request Node Pointer Channel Event i"]
    #[inline(always)]
    pub fn cev6np(&self) -> CEV6NP_R {
        CEV6NP_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - Service Request Node Pointer Channel Event i"]
    #[inline(always)]
    pub fn cev7np(&self) -> CEV7NP_R {
        CEV7NP_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Service Request Node Pointer Channel Event i"]
    #[inline(always)]
    pub fn cev0np(&mut self) -> CEV0NP_W<CEVNP0_SPEC> {
        CEV0NP_W::new(self, 0)
    }
    #[doc = "Bits 4:7 - Service Request Node Pointer Channel Event i"]
    #[inline(always)]
    pub fn cev1np(&mut self) -> CEV1NP_W<CEVNP0_SPEC> {
        CEV1NP_W::new(self, 4)
    }
    #[doc = "Bits 8:11 - Service Request Node Pointer Channel Event i"]
    #[inline(always)]
    pub fn cev2np(&mut self) -> CEV2NP_W<CEVNP0_SPEC> {
        CEV2NP_W::new(self, 8)
    }
    #[doc = "Bits 12:15 - Service Request Node Pointer Channel Event i"]
    #[inline(always)]
    pub fn cev3np(&mut self) -> CEV3NP_W<CEVNP0_SPEC> {
        CEV3NP_W::new(self, 12)
    }
    #[doc = "Bits 16:19 - Service Request Node Pointer Channel Event i"]
    #[inline(always)]
    pub fn cev4np(&mut self) -> CEV4NP_W<CEVNP0_SPEC> {
        CEV4NP_W::new(self, 16)
    }
    #[doc = "Bits 20:23 - Service Request Node Pointer Channel Event i"]
    #[inline(always)]
    pub fn cev5np(&mut self) -> CEV5NP_W<CEVNP0_SPEC> {
        CEV5NP_W::new(self, 20)
    }
    #[doc = "Bits 24:27 - Service Request Node Pointer Channel Event i"]
    #[inline(always)]
    pub fn cev6np(&mut self) -> CEV6NP_W<CEVNP0_SPEC> {
        CEV6NP_W::new(self, 24)
    }
    #[doc = "Bits 28:31 - Service Request Node Pointer Channel Event i"]
    #[inline(always)]
    pub fn cev7np(&mut self) -> CEV7NP_W<CEVNP0_SPEC> {
        CEV7NP_W::new(self, 28)
    }
}
#[doc = "Channel Event Node Pointer Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`cevnp0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cevnp0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CEVNP0_SPEC;
impl crate::RegisterSpec for CEVNP0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cevnp0::R`](R) reader structure"]
impl crate::Readable for CEVNP0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cevnp0::W`](W) writer structure"]
impl crate::Writable for CEVNP0_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CEVNP0 to value 0"]
impl crate::Resettable for CEVNP0_SPEC {
    const RESET_VALUE: u32 = 0;
}
