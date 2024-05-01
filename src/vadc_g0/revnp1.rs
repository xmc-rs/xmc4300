#[doc = "Register `REVNP1` reader"]
pub type R = crate::R<REVNP1_SPEC>;
#[doc = "Register `REVNP1` writer"]
pub type W = crate::W<REVNP1_SPEC>;
#[doc = "Service Request Node Pointer Result Event i\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum REV8NP_A {
    #[doc = "0: Select service request line 0 of group x"]
    VALUE1 = 0,
    #[doc = "3: Select service request line 3 of group x"]
    VALUE2 = 3,
    #[doc = "4: Select shared service request line 0"]
    VALUE3 = 4,
    #[doc = "7: Select shared service request line 3"]
    VALUE4 = 7,
}
impl From<REV8NP_A> for u8 {
    #[inline(always)]
    fn from(variant: REV8NP_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for REV8NP_A {
    type Ux = u8;
}
impl crate::IsEnum for REV8NP_A {}
#[doc = "Field `REV8NP` reader - Service Request Node Pointer Result Event i"]
pub type REV8NP_R = crate::FieldReader<REV8NP_A>;
impl REV8NP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<REV8NP_A> {
        match self.bits {
            0 => Some(REV8NP_A::VALUE1),
            3 => Some(REV8NP_A::VALUE2),
            4 => Some(REV8NP_A::VALUE3),
            7 => Some(REV8NP_A::VALUE4),
            _ => None,
        }
    }
    #[doc = "Select service request line 0 of group x"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == REV8NP_A::VALUE1
    }
    #[doc = "Select service request line 3 of group x"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == REV8NP_A::VALUE2
    }
    #[doc = "Select shared service request line 0"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == REV8NP_A::VALUE3
    }
    #[doc = "Select shared service request line 3"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == REV8NP_A::VALUE4
    }
}
#[doc = "Field `REV8NP` writer - Service Request Node Pointer Result Event i"]
pub type REV8NP_W<'a, REG> = crate::FieldWriter<'a, REG, 4, REV8NP_A>;
impl<'a, REG> REV8NP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Select service request line 0 of group x"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(REV8NP_A::VALUE1)
    }
    #[doc = "Select service request line 3 of group x"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(REV8NP_A::VALUE2)
    }
    #[doc = "Select shared service request line 0"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(REV8NP_A::VALUE3)
    }
    #[doc = "Select shared service request line 3"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(REV8NP_A::VALUE4)
    }
}
#[doc = "Service Request Node Pointer Result Event i\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum REV9NP_A {
    #[doc = "0: Select service request line 0 of group x"]
    VALUE1 = 0,
    #[doc = "3: Select service request line 3 of group x"]
    VALUE2 = 3,
    #[doc = "4: Select shared service request line 0"]
    VALUE3 = 4,
    #[doc = "7: Select shared service request line 3"]
    VALUE4 = 7,
}
impl From<REV9NP_A> for u8 {
    #[inline(always)]
    fn from(variant: REV9NP_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for REV9NP_A {
    type Ux = u8;
}
impl crate::IsEnum for REV9NP_A {}
#[doc = "Field `REV9NP` reader - Service Request Node Pointer Result Event i"]
pub type REV9NP_R = crate::FieldReader<REV9NP_A>;
impl REV9NP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<REV9NP_A> {
        match self.bits {
            0 => Some(REV9NP_A::VALUE1),
            3 => Some(REV9NP_A::VALUE2),
            4 => Some(REV9NP_A::VALUE3),
            7 => Some(REV9NP_A::VALUE4),
            _ => None,
        }
    }
    #[doc = "Select service request line 0 of group x"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == REV9NP_A::VALUE1
    }
    #[doc = "Select service request line 3 of group x"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == REV9NP_A::VALUE2
    }
    #[doc = "Select shared service request line 0"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == REV9NP_A::VALUE3
    }
    #[doc = "Select shared service request line 3"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == REV9NP_A::VALUE4
    }
}
#[doc = "Field `REV9NP` writer - Service Request Node Pointer Result Event i"]
pub type REV9NP_W<'a, REG> = crate::FieldWriter<'a, REG, 4, REV9NP_A>;
impl<'a, REG> REV9NP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Select service request line 0 of group x"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(REV9NP_A::VALUE1)
    }
    #[doc = "Select service request line 3 of group x"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(REV9NP_A::VALUE2)
    }
    #[doc = "Select shared service request line 0"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(REV9NP_A::VALUE3)
    }
    #[doc = "Select shared service request line 3"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(REV9NP_A::VALUE4)
    }
}
#[doc = "Service Request Node Pointer Result Event i\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum REV10NP_A {
    #[doc = "0: Select service request line 0 of group x"]
    VALUE1 = 0,
    #[doc = "3: Select service request line 3 of group x"]
    VALUE2 = 3,
    #[doc = "4: Select shared service request line 0"]
    VALUE3 = 4,
    #[doc = "7: Select shared service request line 3"]
    VALUE4 = 7,
}
impl From<REV10NP_A> for u8 {
    #[inline(always)]
    fn from(variant: REV10NP_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for REV10NP_A {
    type Ux = u8;
}
impl crate::IsEnum for REV10NP_A {}
#[doc = "Field `REV10NP` reader - Service Request Node Pointer Result Event i"]
pub type REV10NP_R = crate::FieldReader<REV10NP_A>;
impl REV10NP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<REV10NP_A> {
        match self.bits {
            0 => Some(REV10NP_A::VALUE1),
            3 => Some(REV10NP_A::VALUE2),
            4 => Some(REV10NP_A::VALUE3),
            7 => Some(REV10NP_A::VALUE4),
            _ => None,
        }
    }
    #[doc = "Select service request line 0 of group x"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == REV10NP_A::VALUE1
    }
    #[doc = "Select service request line 3 of group x"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == REV10NP_A::VALUE2
    }
    #[doc = "Select shared service request line 0"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == REV10NP_A::VALUE3
    }
    #[doc = "Select shared service request line 3"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == REV10NP_A::VALUE4
    }
}
#[doc = "Field `REV10NP` writer - Service Request Node Pointer Result Event i"]
pub type REV10NP_W<'a, REG> = crate::FieldWriter<'a, REG, 4, REV10NP_A>;
impl<'a, REG> REV10NP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Select service request line 0 of group x"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(REV10NP_A::VALUE1)
    }
    #[doc = "Select service request line 3 of group x"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(REV10NP_A::VALUE2)
    }
    #[doc = "Select shared service request line 0"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(REV10NP_A::VALUE3)
    }
    #[doc = "Select shared service request line 3"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(REV10NP_A::VALUE4)
    }
}
#[doc = "Service Request Node Pointer Result Event i\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum REV11NP_A {
    #[doc = "0: Select service request line 0 of group x"]
    VALUE1 = 0,
    #[doc = "3: Select service request line 3 of group x"]
    VALUE2 = 3,
    #[doc = "4: Select shared service request line 0"]
    VALUE3 = 4,
    #[doc = "7: Select shared service request line 3"]
    VALUE4 = 7,
}
impl From<REV11NP_A> for u8 {
    #[inline(always)]
    fn from(variant: REV11NP_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for REV11NP_A {
    type Ux = u8;
}
impl crate::IsEnum for REV11NP_A {}
#[doc = "Field `REV11NP` reader - Service Request Node Pointer Result Event i"]
pub type REV11NP_R = crate::FieldReader<REV11NP_A>;
impl REV11NP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<REV11NP_A> {
        match self.bits {
            0 => Some(REV11NP_A::VALUE1),
            3 => Some(REV11NP_A::VALUE2),
            4 => Some(REV11NP_A::VALUE3),
            7 => Some(REV11NP_A::VALUE4),
            _ => None,
        }
    }
    #[doc = "Select service request line 0 of group x"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == REV11NP_A::VALUE1
    }
    #[doc = "Select service request line 3 of group x"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == REV11NP_A::VALUE2
    }
    #[doc = "Select shared service request line 0"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == REV11NP_A::VALUE3
    }
    #[doc = "Select shared service request line 3"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == REV11NP_A::VALUE4
    }
}
#[doc = "Field `REV11NP` writer - Service Request Node Pointer Result Event i"]
pub type REV11NP_W<'a, REG> = crate::FieldWriter<'a, REG, 4, REV11NP_A>;
impl<'a, REG> REV11NP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Select service request line 0 of group x"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(REV11NP_A::VALUE1)
    }
    #[doc = "Select service request line 3 of group x"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(REV11NP_A::VALUE2)
    }
    #[doc = "Select shared service request line 0"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(REV11NP_A::VALUE3)
    }
    #[doc = "Select shared service request line 3"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(REV11NP_A::VALUE4)
    }
}
#[doc = "Service Request Node Pointer Result Event i\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum REV12NP_A {
    #[doc = "0: Select service request line 0 of group x"]
    VALUE1 = 0,
    #[doc = "3: Select service request line 3 of group x"]
    VALUE2 = 3,
    #[doc = "4: Select shared service request line 0"]
    VALUE3 = 4,
    #[doc = "7: Select shared service request line 3"]
    VALUE4 = 7,
}
impl From<REV12NP_A> for u8 {
    #[inline(always)]
    fn from(variant: REV12NP_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for REV12NP_A {
    type Ux = u8;
}
impl crate::IsEnum for REV12NP_A {}
#[doc = "Field `REV12NP` reader - Service Request Node Pointer Result Event i"]
pub type REV12NP_R = crate::FieldReader<REV12NP_A>;
impl REV12NP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<REV12NP_A> {
        match self.bits {
            0 => Some(REV12NP_A::VALUE1),
            3 => Some(REV12NP_A::VALUE2),
            4 => Some(REV12NP_A::VALUE3),
            7 => Some(REV12NP_A::VALUE4),
            _ => None,
        }
    }
    #[doc = "Select service request line 0 of group x"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == REV12NP_A::VALUE1
    }
    #[doc = "Select service request line 3 of group x"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == REV12NP_A::VALUE2
    }
    #[doc = "Select shared service request line 0"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == REV12NP_A::VALUE3
    }
    #[doc = "Select shared service request line 3"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == REV12NP_A::VALUE4
    }
}
#[doc = "Field `REV12NP` writer - Service Request Node Pointer Result Event i"]
pub type REV12NP_W<'a, REG> = crate::FieldWriter<'a, REG, 4, REV12NP_A>;
impl<'a, REG> REV12NP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Select service request line 0 of group x"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(REV12NP_A::VALUE1)
    }
    #[doc = "Select service request line 3 of group x"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(REV12NP_A::VALUE2)
    }
    #[doc = "Select shared service request line 0"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(REV12NP_A::VALUE3)
    }
    #[doc = "Select shared service request line 3"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(REV12NP_A::VALUE4)
    }
}
#[doc = "Service Request Node Pointer Result Event i\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum REV13NP_A {
    #[doc = "0: Select service request line 0 of group x"]
    VALUE1 = 0,
    #[doc = "3: Select service request line 3 of group x"]
    VALUE2 = 3,
    #[doc = "4: Select shared service request line 0"]
    VALUE3 = 4,
    #[doc = "7: Select shared service request line 3"]
    VALUE4 = 7,
}
impl From<REV13NP_A> for u8 {
    #[inline(always)]
    fn from(variant: REV13NP_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for REV13NP_A {
    type Ux = u8;
}
impl crate::IsEnum for REV13NP_A {}
#[doc = "Field `REV13NP` reader - Service Request Node Pointer Result Event i"]
pub type REV13NP_R = crate::FieldReader<REV13NP_A>;
impl REV13NP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<REV13NP_A> {
        match self.bits {
            0 => Some(REV13NP_A::VALUE1),
            3 => Some(REV13NP_A::VALUE2),
            4 => Some(REV13NP_A::VALUE3),
            7 => Some(REV13NP_A::VALUE4),
            _ => None,
        }
    }
    #[doc = "Select service request line 0 of group x"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == REV13NP_A::VALUE1
    }
    #[doc = "Select service request line 3 of group x"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == REV13NP_A::VALUE2
    }
    #[doc = "Select shared service request line 0"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == REV13NP_A::VALUE3
    }
    #[doc = "Select shared service request line 3"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == REV13NP_A::VALUE4
    }
}
#[doc = "Field `REV13NP` writer - Service Request Node Pointer Result Event i"]
pub type REV13NP_W<'a, REG> = crate::FieldWriter<'a, REG, 4, REV13NP_A>;
impl<'a, REG> REV13NP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Select service request line 0 of group x"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(REV13NP_A::VALUE1)
    }
    #[doc = "Select service request line 3 of group x"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(REV13NP_A::VALUE2)
    }
    #[doc = "Select shared service request line 0"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(REV13NP_A::VALUE3)
    }
    #[doc = "Select shared service request line 3"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(REV13NP_A::VALUE4)
    }
}
#[doc = "Service Request Node Pointer Result Event i\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum REV14NP_A {
    #[doc = "0: Select service request line 0 of group x"]
    VALUE1 = 0,
    #[doc = "3: Select service request line 3 of group x"]
    VALUE2 = 3,
    #[doc = "4: Select shared service request line 0"]
    VALUE3 = 4,
    #[doc = "7: Select shared service request line 3"]
    VALUE4 = 7,
}
impl From<REV14NP_A> for u8 {
    #[inline(always)]
    fn from(variant: REV14NP_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for REV14NP_A {
    type Ux = u8;
}
impl crate::IsEnum for REV14NP_A {}
#[doc = "Field `REV14NP` reader - Service Request Node Pointer Result Event i"]
pub type REV14NP_R = crate::FieldReader<REV14NP_A>;
impl REV14NP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<REV14NP_A> {
        match self.bits {
            0 => Some(REV14NP_A::VALUE1),
            3 => Some(REV14NP_A::VALUE2),
            4 => Some(REV14NP_A::VALUE3),
            7 => Some(REV14NP_A::VALUE4),
            _ => None,
        }
    }
    #[doc = "Select service request line 0 of group x"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == REV14NP_A::VALUE1
    }
    #[doc = "Select service request line 3 of group x"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == REV14NP_A::VALUE2
    }
    #[doc = "Select shared service request line 0"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == REV14NP_A::VALUE3
    }
    #[doc = "Select shared service request line 3"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == REV14NP_A::VALUE4
    }
}
#[doc = "Field `REV14NP` writer - Service Request Node Pointer Result Event i"]
pub type REV14NP_W<'a, REG> = crate::FieldWriter<'a, REG, 4, REV14NP_A>;
impl<'a, REG> REV14NP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Select service request line 0 of group x"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(REV14NP_A::VALUE1)
    }
    #[doc = "Select service request line 3 of group x"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(REV14NP_A::VALUE2)
    }
    #[doc = "Select shared service request line 0"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(REV14NP_A::VALUE3)
    }
    #[doc = "Select shared service request line 3"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(REV14NP_A::VALUE4)
    }
}
#[doc = "Service Request Node Pointer Result Event i\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum REV15NP_A {
    #[doc = "0: Select service request line 0 of group x"]
    VALUE1 = 0,
    #[doc = "3: Select service request line 3 of group x"]
    VALUE2 = 3,
    #[doc = "4: Select shared service request line 0"]
    VALUE3 = 4,
    #[doc = "7: Select shared service request line 3"]
    VALUE4 = 7,
}
impl From<REV15NP_A> for u8 {
    #[inline(always)]
    fn from(variant: REV15NP_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for REV15NP_A {
    type Ux = u8;
}
impl crate::IsEnum for REV15NP_A {}
#[doc = "Field `REV15NP` reader - Service Request Node Pointer Result Event i"]
pub type REV15NP_R = crate::FieldReader<REV15NP_A>;
impl REV15NP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<REV15NP_A> {
        match self.bits {
            0 => Some(REV15NP_A::VALUE1),
            3 => Some(REV15NP_A::VALUE2),
            4 => Some(REV15NP_A::VALUE3),
            7 => Some(REV15NP_A::VALUE4),
            _ => None,
        }
    }
    #[doc = "Select service request line 0 of group x"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == REV15NP_A::VALUE1
    }
    #[doc = "Select service request line 3 of group x"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == REV15NP_A::VALUE2
    }
    #[doc = "Select shared service request line 0"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == REV15NP_A::VALUE3
    }
    #[doc = "Select shared service request line 3"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == REV15NP_A::VALUE4
    }
}
#[doc = "Field `REV15NP` writer - Service Request Node Pointer Result Event i"]
pub type REV15NP_W<'a, REG> = crate::FieldWriter<'a, REG, 4, REV15NP_A>;
impl<'a, REG> REV15NP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Select service request line 0 of group x"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(REV15NP_A::VALUE1)
    }
    #[doc = "Select service request line 3 of group x"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(REV15NP_A::VALUE2)
    }
    #[doc = "Select shared service request line 0"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(REV15NP_A::VALUE3)
    }
    #[doc = "Select shared service request line 3"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(REV15NP_A::VALUE4)
    }
}
impl R {
    #[doc = "Bits 0:3 - Service Request Node Pointer Result Event i"]
    #[inline(always)]
    pub fn rev8np(&self) -> REV8NP_R {
        REV8NP_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Service Request Node Pointer Result Event i"]
    #[inline(always)]
    pub fn rev9np(&self) -> REV9NP_R {
        REV9NP_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Service Request Node Pointer Result Event i"]
    #[inline(always)]
    pub fn rev10np(&self) -> REV10NP_R {
        REV10NP_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Service Request Node Pointer Result Event i"]
    #[inline(always)]
    pub fn rev11np(&self) -> REV11NP_R {
        REV11NP_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Service Request Node Pointer Result Event i"]
    #[inline(always)]
    pub fn rev12np(&self) -> REV12NP_R {
        REV12NP_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Service Request Node Pointer Result Event i"]
    #[inline(always)]
    pub fn rev13np(&self) -> REV13NP_R {
        REV13NP_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Service Request Node Pointer Result Event i"]
    #[inline(always)]
    pub fn rev14np(&self) -> REV14NP_R {
        REV14NP_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - Service Request Node Pointer Result Event i"]
    #[inline(always)]
    pub fn rev15np(&self) -> REV15NP_R {
        REV15NP_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Service Request Node Pointer Result Event i"]
    #[inline(always)]
    #[must_use]
    pub fn rev8np(&mut self) -> REV8NP_W<REVNP1_SPEC> {
        REV8NP_W::new(self, 0)
    }
    #[doc = "Bits 4:7 - Service Request Node Pointer Result Event i"]
    #[inline(always)]
    #[must_use]
    pub fn rev9np(&mut self) -> REV9NP_W<REVNP1_SPEC> {
        REV9NP_W::new(self, 4)
    }
    #[doc = "Bits 8:11 - Service Request Node Pointer Result Event i"]
    #[inline(always)]
    #[must_use]
    pub fn rev10np(&mut self) -> REV10NP_W<REVNP1_SPEC> {
        REV10NP_W::new(self, 8)
    }
    #[doc = "Bits 12:15 - Service Request Node Pointer Result Event i"]
    #[inline(always)]
    #[must_use]
    pub fn rev11np(&mut self) -> REV11NP_W<REVNP1_SPEC> {
        REV11NP_W::new(self, 12)
    }
    #[doc = "Bits 16:19 - Service Request Node Pointer Result Event i"]
    #[inline(always)]
    #[must_use]
    pub fn rev12np(&mut self) -> REV12NP_W<REVNP1_SPEC> {
        REV12NP_W::new(self, 16)
    }
    #[doc = "Bits 20:23 - Service Request Node Pointer Result Event i"]
    #[inline(always)]
    #[must_use]
    pub fn rev13np(&mut self) -> REV13NP_W<REVNP1_SPEC> {
        REV13NP_W::new(self, 20)
    }
    #[doc = "Bits 24:27 - Service Request Node Pointer Result Event i"]
    #[inline(always)]
    #[must_use]
    pub fn rev14np(&mut self) -> REV14NP_W<REVNP1_SPEC> {
        REV14NP_W::new(self, 24)
    }
    #[doc = "Bits 28:31 - Service Request Node Pointer Result Event i"]
    #[inline(always)]
    #[must_use]
    pub fn rev15np(&mut self) -> REV15NP_W<REVNP1_SPEC> {
        REV15NP_W::new(self, 28)
    }
}
#[doc = "Result Event Node Pointer Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`revnp1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`revnp1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct REVNP1_SPEC;
impl crate::RegisterSpec for REVNP1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`revnp1::R`](R) reader structure"]
impl crate::Readable for REVNP1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`revnp1::W`](W) writer structure"]
impl crate::Writable for REVNP1_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets REVNP1 to value 0"]
impl crate::Resettable for REVNP1_SPEC {
    const RESET_VALUE: u32 = 0;
}
