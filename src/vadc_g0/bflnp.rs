#[doc = "Register `BFLNP` reader"]
pub type R = crate::R<BflnpSpec>;
#[doc = "Register `BFLNP` writer"]
pub type W = crate::W<BflnpSpec>;
#[doc = "Boundary Flag y Node Pointer\n\nValue on reset: 15"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Bfl0np {
    #[doc = "0: Select common bondary flag output 0"]
    Value1 = 0,
    #[doc = "3: Select common bondary flag output 3"]
    Value2 = 3,
    #[doc = "4: Select shared service request line 0"]
    Value3 = 4,
    #[doc = "7: Select shared service request line 3"]
    Value4 = 7,
    #[doc = "15: Disabled, no common output signal"]
    Value5 = 15,
}
impl From<Bfl0np> for u8 {
    #[inline(always)]
    fn from(variant: Bfl0np) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Bfl0np {
    type Ux = u8;
}
#[doc = "Field `BFL0NP` reader - Boundary Flag y Node Pointer"]
pub type Bfl0npR = crate::FieldReader<Bfl0np>;
impl Bfl0npR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Bfl0np> {
        match self.bits {
            0 => Some(Bfl0np::Value1),
            3 => Some(Bfl0np::Value2),
            4 => Some(Bfl0np::Value3),
            7 => Some(Bfl0np::Value4),
            15 => Some(Bfl0np::Value5),
            _ => None,
        }
    }
    #[doc = "Select common bondary flag output 0"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Bfl0np::Value1
    }
    #[doc = "Select common bondary flag output 3"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Bfl0np::Value2
    }
    #[doc = "Select shared service request line 0"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == Bfl0np::Value3
    }
    #[doc = "Select shared service request line 3"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == Bfl0np::Value4
    }
    #[doc = "Disabled, no common output signal"]
    #[inline(always)]
    pub fn is_value5(&self) -> bool {
        *self == Bfl0np::Value5
    }
}
#[doc = "Field `BFL0NP` writer - Boundary Flag y Node Pointer"]
pub type Bfl0npW<'a, REG> = crate::FieldWriter<'a, REG, 4, Bfl0np>;
impl<'a, REG> Bfl0npW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Select common bondary flag output 0"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Bfl0np::Value1)
    }
    #[doc = "Select common bondary flag output 3"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Bfl0np::Value2)
    }
    #[doc = "Select shared service request line 0"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(Bfl0np::Value3)
    }
    #[doc = "Select shared service request line 3"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(Bfl0np::Value4)
    }
    #[doc = "Disabled, no common output signal"]
    #[inline(always)]
    pub fn value5(self) -> &'a mut crate::W<REG> {
        self.variant(Bfl0np::Value5)
    }
}
#[doc = "Boundary Flag y Node Pointer\n\nValue on reset: 15"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Bfl1np {
    #[doc = "0: Select common bondary flag output 0"]
    Value1 = 0,
    #[doc = "3: Select common bondary flag output 3"]
    Value2 = 3,
    #[doc = "4: Select shared service request line 0"]
    Value3 = 4,
    #[doc = "7: Select shared service request line 3"]
    Value4 = 7,
    #[doc = "15: Disabled, no common output signal"]
    Value5 = 15,
}
impl From<Bfl1np> for u8 {
    #[inline(always)]
    fn from(variant: Bfl1np) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Bfl1np {
    type Ux = u8;
}
#[doc = "Field `BFL1NP` reader - Boundary Flag y Node Pointer"]
pub type Bfl1npR = crate::FieldReader<Bfl1np>;
impl Bfl1npR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Bfl1np> {
        match self.bits {
            0 => Some(Bfl1np::Value1),
            3 => Some(Bfl1np::Value2),
            4 => Some(Bfl1np::Value3),
            7 => Some(Bfl1np::Value4),
            15 => Some(Bfl1np::Value5),
            _ => None,
        }
    }
    #[doc = "Select common bondary flag output 0"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Bfl1np::Value1
    }
    #[doc = "Select common bondary flag output 3"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Bfl1np::Value2
    }
    #[doc = "Select shared service request line 0"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == Bfl1np::Value3
    }
    #[doc = "Select shared service request line 3"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == Bfl1np::Value4
    }
    #[doc = "Disabled, no common output signal"]
    #[inline(always)]
    pub fn is_value5(&self) -> bool {
        *self == Bfl1np::Value5
    }
}
#[doc = "Field `BFL1NP` writer - Boundary Flag y Node Pointer"]
pub type Bfl1npW<'a, REG> = crate::FieldWriter<'a, REG, 4, Bfl1np>;
impl<'a, REG> Bfl1npW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Select common bondary flag output 0"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Bfl1np::Value1)
    }
    #[doc = "Select common bondary flag output 3"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Bfl1np::Value2)
    }
    #[doc = "Select shared service request line 0"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(Bfl1np::Value3)
    }
    #[doc = "Select shared service request line 3"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(Bfl1np::Value4)
    }
    #[doc = "Disabled, no common output signal"]
    #[inline(always)]
    pub fn value5(self) -> &'a mut crate::W<REG> {
        self.variant(Bfl1np::Value5)
    }
}
#[doc = "Boundary Flag y Node Pointer\n\nValue on reset: 15"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Bfl2np {
    #[doc = "0: Select common bondary flag output 0"]
    Value1 = 0,
    #[doc = "3: Select common bondary flag output 3"]
    Value2 = 3,
    #[doc = "4: Select shared service request line 0"]
    Value3 = 4,
    #[doc = "7: Select shared service request line 3"]
    Value4 = 7,
    #[doc = "15: Disabled, no common output signal"]
    Value5 = 15,
}
impl From<Bfl2np> for u8 {
    #[inline(always)]
    fn from(variant: Bfl2np) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Bfl2np {
    type Ux = u8;
}
#[doc = "Field `BFL2NP` reader - Boundary Flag y Node Pointer"]
pub type Bfl2npR = crate::FieldReader<Bfl2np>;
impl Bfl2npR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Bfl2np> {
        match self.bits {
            0 => Some(Bfl2np::Value1),
            3 => Some(Bfl2np::Value2),
            4 => Some(Bfl2np::Value3),
            7 => Some(Bfl2np::Value4),
            15 => Some(Bfl2np::Value5),
            _ => None,
        }
    }
    #[doc = "Select common bondary flag output 0"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Bfl2np::Value1
    }
    #[doc = "Select common bondary flag output 3"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Bfl2np::Value2
    }
    #[doc = "Select shared service request line 0"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == Bfl2np::Value3
    }
    #[doc = "Select shared service request line 3"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == Bfl2np::Value4
    }
    #[doc = "Disabled, no common output signal"]
    #[inline(always)]
    pub fn is_value5(&self) -> bool {
        *self == Bfl2np::Value5
    }
}
#[doc = "Field `BFL2NP` writer - Boundary Flag y Node Pointer"]
pub type Bfl2npW<'a, REG> = crate::FieldWriter<'a, REG, 4, Bfl2np>;
impl<'a, REG> Bfl2npW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Select common bondary flag output 0"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Bfl2np::Value1)
    }
    #[doc = "Select common bondary flag output 3"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Bfl2np::Value2)
    }
    #[doc = "Select shared service request line 0"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(Bfl2np::Value3)
    }
    #[doc = "Select shared service request line 3"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(Bfl2np::Value4)
    }
    #[doc = "Disabled, no common output signal"]
    #[inline(always)]
    pub fn value5(self) -> &'a mut crate::W<REG> {
        self.variant(Bfl2np::Value5)
    }
}
#[doc = "Boundary Flag y Node Pointer\n\nValue on reset: 15"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Bfl3np {
    #[doc = "0: Select common bondary flag output 0"]
    Value1 = 0,
    #[doc = "3: Select common bondary flag output 3"]
    Value2 = 3,
    #[doc = "4: Select shared service request line 0"]
    Value3 = 4,
    #[doc = "7: Select shared service request line 3"]
    Value4 = 7,
    #[doc = "15: Disabled, no common output signal"]
    Value5 = 15,
}
impl From<Bfl3np> for u8 {
    #[inline(always)]
    fn from(variant: Bfl3np) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Bfl3np {
    type Ux = u8;
}
#[doc = "Field `BFL3NP` reader - Boundary Flag y Node Pointer"]
pub type Bfl3npR = crate::FieldReader<Bfl3np>;
impl Bfl3npR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Bfl3np> {
        match self.bits {
            0 => Some(Bfl3np::Value1),
            3 => Some(Bfl3np::Value2),
            4 => Some(Bfl3np::Value3),
            7 => Some(Bfl3np::Value4),
            15 => Some(Bfl3np::Value5),
            _ => None,
        }
    }
    #[doc = "Select common bondary flag output 0"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Bfl3np::Value1
    }
    #[doc = "Select common bondary flag output 3"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Bfl3np::Value2
    }
    #[doc = "Select shared service request line 0"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == Bfl3np::Value3
    }
    #[doc = "Select shared service request line 3"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == Bfl3np::Value4
    }
    #[doc = "Disabled, no common output signal"]
    #[inline(always)]
    pub fn is_value5(&self) -> bool {
        *self == Bfl3np::Value5
    }
}
#[doc = "Field `BFL3NP` writer - Boundary Flag y Node Pointer"]
pub type Bfl3npW<'a, REG> = crate::FieldWriter<'a, REG, 4, Bfl3np>;
impl<'a, REG> Bfl3npW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Select common bondary flag output 0"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Bfl3np::Value1)
    }
    #[doc = "Select common bondary flag output 3"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Bfl3np::Value2)
    }
    #[doc = "Select shared service request line 0"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(Bfl3np::Value3)
    }
    #[doc = "Select shared service request line 3"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(Bfl3np::Value4)
    }
    #[doc = "Disabled, no common output signal"]
    #[inline(always)]
    pub fn value5(self) -> &'a mut crate::W<REG> {
        self.variant(Bfl3np::Value5)
    }
}
impl R {
    #[doc = "Bits 0:3 - Boundary Flag y Node Pointer"]
    #[inline(always)]
    pub fn bfl0np(&self) -> Bfl0npR {
        Bfl0npR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Boundary Flag y Node Pointer"]
    #[inline(always)]
    pub fn bfl1np(&self) -> Bfl1npR {
        Bfl1npR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Boundary Flag y Node Pointer"]
    #[inline(always)]
    pub fn bfl2np(&self) -> Bfl2npR {
        Bfl2npR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Boundary Flag y Node Pointer"]
    #[inline(always)]
    pub fn bfl3np(&self) -> Bfl3npR {
        Bfl3npR::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Boundary Flag y Node Pointer"]
    #[inline(always)]
    #[must_use]
    pub fn bfl0np(&mut self) -> Bfl0npW<BflnpSpec> {
        Bfl0npW::new(self, 0)
    }
    #[doc = "Bits 4:7 - Boundary Flag y Node Pointer"]
    #[inline(always)]
    #[must_use]
    pub fn bfl1np(&mut self) -> Bfl1npW<BflnpSpec> {
        Bfl1npW::new(self, 4)
    }
    #[doc = "Bits 8:11 - Boundary Flag y Node Pointer"]
    #[inline(always)]
    #[must_use]
    pub fn bfl2np(&mut self) -> Bfl2npW<BflnpSpec> {
        Bfl2npW::new(self, 8)
    }
    #[doc = "Bits 12:15 - Boundary Flag y Node Pointer"]
    #[inline(always)]
    #[must_use]
    pub fn bfl3np(&mut self) -> Bfl3npW<BflnpSpec> {
        Bfl3npW::new(self, 12)
    }
}
#[doc = "Boundary Flag Node Pointer Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bflnp::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bflnp::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BflnpSpec;
impl crate::RegisterSpec for BflnpSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bflnp::R`](R) reader structure"]
impl crate::Readable for BflnpSpec {}
#[doc = "`write(|w| ..)` method takes [`bflnp::W`](W) writer structure"]
impl crate::Writable for BflnpSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BFLNP to value 0xffff"]
impl crate::Resettable for BflnpSpec {
    const RESET_VALUE: u32 = 0xffff;
}
