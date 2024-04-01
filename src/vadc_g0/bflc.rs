#[doc = "Register `BFLC` reader"]
pub type R = crate::R<BflcSpec>;
#[doc = "Register `BFLC` writer"]
pub type W = crate::W<BflcSpec>;
#[doc = "Boundary Flag y Mode Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Bfm0 {
    #[doc = "0: Disable boundary flag, BFLy is not changed"]
    Value1 = 0,
    #[doc = "1: Always enable boundary flag (follow compare results)"]
    Value2 = 1,
    #[doc = "2: Enable boundary flag while gate of source 0 is active, clear BFLy while gate is inactive"]
    Value3 = 2,
    #[doc = "3: Enable boundary flag while gate of source 1 is active, clear BFLy while gate is inactive"]
    Value4 = 3,
}
impl From<Bfm0> for u8 {
    #[inline(always)]
    fn from(variant: Bfm0) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Bfm0 {
    type Ux = u8;
}
impl crate::IsEnum for Bfm0 {}
#[doc = "Field `BFM0` reader - Boundary Flag y Mode Control"]
pub type Bfm0R = crate::FieldReader<Bfm0>;
impl Bfm0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Bfm0> {
        match self.bits {
            0 => Some(Bfm0::Value1),
            1 => Some(Bfm0::Value2),
            2 => Some(Bfm0::Value3),
            3 => Some(Bfm0::Value4),
            _ => None,
        }
    }
    #[doc = "Disable boundary flag, BFLy is not changed"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Bfm0::Value1
    }
    #[doc = "Always enable boundary flag (follow compare results)"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Bfm0::Value2
    }
    #[doc = "Enable boundary flag while gate of source 0 is active, clear BFLy while gate is inactive"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == Bfm0::Value3
    }
    #[doc = "Enable boundary flag while gate of source 1 is active, clear BFLy while gate is inactive"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == Bfm0::Value4
    }
}
#[doc = "Field `BFM0` writer - Boundary Flag y Mode Control"]
pub type Bfm0W<'a, REG> = crate::FieldWriter<'a, REG, 4, Bfm0>;
impl<'a, REG> Bfm0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Disable boundary flag, BFLy is not changed"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Bfm0::Value1)
    }
    #[doc = "Always enable boundary flag (follow compare results)"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Bfm0::Value2)
    }
    #[doc = "Enable boundary flag while gate of source 0 is active, clear BFLy while gate is inactive"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(Bfm0::Value3)
    }
    #[doc = "Enable boundary flag while gate of source 1 is active, clear BFLy while gate is inactive"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(Bfm0::Value4)
    }
}
#[doc = "Boundary Flag y Mode Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Bfm1 {
    #[doc = "0: Disable boundary flag, BFLy is not changed"]
    Value1 = 0,
    #[doc = "1: Always enable boundary flag (follow compare results)"]
    Value2 = 1,
    #[doc = "2: Enable boundary flag while gate of source 0 is active, clear BFLy while gate is inactive"]
    Value3 = 2,
    #[doc = "3: Enable boundary flag while gate of source 1 is active, clear BFLy while gate is inactive"]
    Value4 = 3,
}
impl From<Bfm1> for u8 {
    #[inline(always)]
    fn from(variant: Bfm1) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Bfm1 {
    type Ux = u8;
}
impl crate::IsEnum for Bfm1 {}
#[doc = "Field `BFM1` reader - Boundary Flag y Mode Control"]
pub type Bfm1R = crate::FieldReader<Bfm1>;
impl Bfm1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Bfm1> {
        match self.bits {
            0 => Some(Bfm1::Value1),
            1 => Some(Bfm1::Value2),
            2 => Some(Bfm1::Value3),
            3 => Some(Bfm1::Value4),
            _ => None,
        }
    }
    #[doc = "Disable boundary flag, BFLy is not changed"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Bfm1::Value1
    }
    #[doc = "Always enable boundary flag (follow compare results)"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Bfm1::Value2
    }
    #[doc = "Enable boundary flag while gate of source 0 is active, clear BFLy while gate is inactive"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == Bfm1::Value3
    }
    #[doc = "Enable boundary flag while gate of source 1 is active, clear BFLy while gate is inactive"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == Bfm1::Value4
    }
}
#[doc = "Field `BFM1` writer - Boundary Flag y Mode Control"]
pub type Bfm1W<'a, REG> = crate::FieldWriter<'a, REG, 4, Bfm1>;
impl<'a, REG> Bfm1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Disable boundary flag, BFLy is not changed"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Bfm1::Value1)
    }
    #[doc = "Always enable boundary flag (follow compare results)"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Bfm1::Value2)
    }
    #[doc = "Enable boundary flag while gate of source 0 is active, clear BFLy while gate is inactive"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(Bfm1::Value3)
    }
    #[doc = "Enable boundary flag while gate of source 1 is active, clear BFLy while gate is inactive"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(Bfm1::Value4)
    }
}
#[doc = "Boundary Flag y Mode Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Bfm2 {
    #[doc = "0: Disable boundary flag, BFLy is not changed"]
    Value1 = 0,
    #[doc = "1: Always enable boundary flag (follow compare results)"]
    Value2 = 1,
    #[doc = "2: Enable boundary flag while gate of source 0 is active, clear BFLy while gate is inactive"]
    Value3 = 2,
    #[doc = "3: Enable boundary flag while gate of source 1 is active, clear BFLy while gate is inactive"]
    Value4 = 3,
}
impl From<Bfm2> for u8 {
    #[inline(always)]
    fn from(variant: Bfm2) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Bfm2 {
    type Ux = u8;
}
impl crate::IsEnum for Bfm2 {}
#[doc = "Field `BFM2` reader - Boundary Flag y Mode Control"]
pub type Bfm2R = crate::FieldReader<Bfm2>;
impl Bfm2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Bfm2> {
        match self.bits {
            0 => Some(Bfm2::Value1),
            1 => Some(Bfm2::Value2),
            2 => Some(Bfm2::Value3),
            3 => Some(Bfm2::Value4),
            _ => None,
        }
    }
    #[doc = "Disable boundary flag, BFLy is not changed"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Bfm2::Value1
    }
    #[doc = "Always enable boundary flag (follow compare results)"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Bfm2::Value2
    }
    #[doc = "Enable boundary flag while gate of source 0 is active, clear BFLy while gate is inactive"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == Bfm2::Value3
    }
    #[doc = "Enable boundary flag while gate of source 1 is active, clear BFLy while gate is inactive"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == Bfm2::Value4
    }
}
#[doc = "Field `BFM2` writer - Boundary Flag y Mode Control"]
pub type Bfm2W<'a, REG> = crate::FieldWriter<'a, REG, 4, Bfm2>;
impl<'a, REG> Bfm2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Disable boundary flag, BFLy is not changed"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Bfm2::Value1)
    }
    #[doc = "Always enable boundary flag (follow compare results)"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Bfm2::Value2)
    }
    #[doc = "Enable boundary flag while gate of source 0 is active, clear BFLy while gate is inactive"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(Bfm2::Value3)
    }
    #[doc = "Enable boundary flag while gate of source 1 is active, clear BFLy while gate is inactive"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(Bfm2::Value4)
    }
}
#[doc = "Boundary Flag y Mode Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Bfm3 {
    #[doc = "0: Disable boundary flag, BFLy is not changed"]
    Value1 = 0,
    #[doc = "1: Always enable boundary flag (follow compare results)"]
    Value2 = 1,
    #[doc = "2: Enable boundary flag while gate of source 0 is active, clear BFLy while gate is inactive"]
    Value3 = 2,
    #[doc = "3: Enable boundary flag while gate of source 1 is active, clear BFLy while gate is inactive"]
    Value4 = 3,
}
impl From<Bfm3> for u8 {
    #[inline(always)]
    fn from(variant: Bfm3) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Bfm3 {
    type Ux = u8;
}
impl crate::IsEnum for Bfm3 {}
#[doc = "Field `BFM3` reader - Boundary Flag y Mode Control"]
pub type Bfm3R = crate::FieldReader<Bfm3>;
impl Bfm3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Bfm3> {
        match self.bits {
            0 => Some(Bfm3::Value1),
            1 => Some(Bfm3::Value2),
            2 => Some(Bfm3::Value3),
            3 => Some(Bfm3::Value4),
            _ => None,
        }
    }
    #[doc = "Disable boundary flag, BFLy is not changed"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Bfm3::Value1
    }
    #[doc = "Always enable boundary flag (follow compare results)"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Bfm3::Value2
    }
    #[doc = "Enable boundary flag while gate of source 0 is active, clear BFLy while gate is inactive"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == Bfm3::Value3
    }
    #[doc = "Enable boundary flag while gate of source 1 is active, clear BFLy while gate is inactive"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == Bfm3::Value4
    }
}
#[doc = "Field `BFM3` writer - Boundary Flag y Mode Control"]
pub type Bfm3W<'a, REG> = crate::FieldWriter<'a, REG, 4, Bfm3>;
impl<'a, REG> Bfm3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Disable boundary flag, BFLy is not changed"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Bfm3::Value1)
    }
    #[doc = "Always enable boundary flag (follow compare results)"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Bfm3::Value2)
    }
    #[doc = "Enable boundary flag while gate of source 0 is active, clear BFLy while gate is inactive"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(Bfm3::Value3)
    }
    #[doc = "Enable boundary flag while gate of source 1 is active, clear BFLy while gate is inactive"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(Bfm3::Value4)
    }
}
impl R {
    #[doc = "Bits 0:3 - Boundary Flag y Mode Control"]
    #[inline(always)]
    pub fn bfm0(&self) -> Bfm0R {
        Bfm0R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Boundary Flag y Mode Control"]
    #[inline(always)]
    pub fn bfm1(&self) -> Bfm1R {
        Bfm1R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Boundary Flag y Mode Control"]
    #[inline(always)]
    pub fn bfm2(&self) -> Bfm2R {
        Bfm2R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Boundary Flag y Mode Control"]
    #[inline(always)]
    pub fn bfm3(&self) -> Bfm3R {
        Bfm3R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Boundary Flag y Mode Control"]
    #[inline(always)]
    #[must_use]
    pub fn bfm0(&mut self) -> Bfm0W<BflcSpec> {
        Bfm0W::new(self, 0)
    }
    #[doc = "Bits 4:7 - Boundary Flag y Mode Control"]
    #[inline(always)]
    #[must_use]
    pub fn bfm1(&mut self) -> Bfm1W<BflcSpec> {
        Bfm1W::new(self, 4)
    }
    #[doc = "Bits 8:11 - Boundary Flag y Mode Control"]
    #[inline(always)]
    #[must_use]
    pub fn bfm2(&mut self) -> Bfm2W<BflcSpec> {
        Bfm2W::new(self, 8)
    }
    #[doc = "Bits 12:15 - Boundary Flag y Mode Control"]
    #[inline(always)]
    #[must_use]
    pub fn bfm3(&mut self) -> Bfm3W<BflcSpec> {
        Bfm3W::new(self, 12)
    }
}
#[doc = "Boundary Flag Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bflc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bflc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BflcSpec;
impl crate::RegisterSpec for BflcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bflc::R`](R) reader structure"]
impl crate::Readable for BflcSpec {}
#[doc = "`write(|w| ..)` method takes [`bflc::W`](W) writer structure"]
impl crate::Writable for BflcSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BFLC to value 0"]
impl crate::Resettable for BflcSpec {
    const RESET_VALUE: u32 = 0;
}
