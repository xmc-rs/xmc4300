#[doc = "Register `CHC` reader"]
pub type R = crate::R<ChcSpec>;
#[doc = "Register `CHC` writer"]
pub type W = crate::W<ChcSpec>;
#[doc = "Asymmetric PWM mode Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ase {
    #[doc = "0: Asymmetric PWM is disabled"]
    Value1 = 0,
    #[doc = "1: Asymmetric PWM is enabled"]
    Value2 = 1,
}
impl From<Ase> for bool {
    #[inline(always)]
    fn from(variant: Ase) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ASE` reader - Asymmetric PWM mode Enable"]
pub type AseR = crate::BitReader<Ase>;
impl AseR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ase {
        match self.bits {
            false => Ase::Value1,
            true => Ase::Value2,
        }
    }
    #[doc = "Asymmetric PWM is disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Ase::Value1
    }
    #[doc = "Asymmetric PWM is enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Ase::Value2
    }
}
#[doc = "Field `ASE` writer - Asymmetric PWM mode Enable"]
pub type AseW<'a, REG> = crate::BitWriter<'a, REG, Ase>;
impl<'a, REG> AseW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Asymmetric PWM is disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Ase::Value1)
    }
    #[doc = "Asymmetric PWM is enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Ase::Value2)
    }
}
#[doc = "Output selector for CCU8x.OUTy0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ocs1 {
    #[doc = "0: CC8yST1 signal path is connected to the CCU8x.OUTy0"]
    Value1 = 0,
    #[doc = "1: Inverted CC8yST1 signal path is connected to the CCU8x.OUTy0"]
    Value2 = 1,
}
impl From<Ocs1> for bool {
    #[inline(always)]
    fn from(variant: Ocs1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OCS1` reader - Output selector for CCU8x.OUTy0"]
pub type Ocs1R = crate::BitReader<Ocs1>;
impl Ocs1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ocs1 {
        match self.bits {
            false => Ocs1::Value1,
            true => Ocs1::Value2,
        }
    }
    #[doc = "CC8yST1 signal path is connected to the CCU8x.OUTy0"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Ocs1::Value1
    }
    #[doc = "Inverted CC8yST1 signal path is connected to the CCU8x.OUTy0"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Ocs1::Value2
    }
}
#[doc = "Field `OCS1` writer - Output selector for CCU8x.OUTy0"]
pub type Ocs1W<'a, REG> = crate::BitWriter<'a, REG, Ocs1>;
impl<'a, REG> Ocs1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CC8yST1 signal path is connected to the CCU8x.OUTy0"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Ocs1::Value1)
    }
    #[doc = "Inverted CC8yST1 signal path is connected to the CCU8x.OUTy0"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Ocs1::Value2)
    }
}
#[doc = "Output selector for CCU8x.OUTy1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ocs2 {
    #[doc = "0: Inverted CC8yST1 signal path is connected to the CCU8x.OUTy1"]
    Value1 = 0,
    #[doc = "1: CC8yST1 signal path is connected to the CCU8x.OUTy1"]
    Value2 = 1,
}
impl From<Ocs2> for bool {
    #[inline(always)]
    fn from(variant: Ocs2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OCS2` reader - Output selector for CCU8x.OUTy1"]
pub type Ocs2R = crate::BitReader<Ocs2>;
impl Ocs2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ocs2 {
        match self.bits {
            false => Ocs2::Value1,
            true => Ocs2::Value2,
        }
    }
    #[doc = "Inverted CC8yST1 signal path is connected to the CCU8x.OUTy1"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Ocs2::Value1
    }
    #[doc = "CC8yST1 signal path is connected to the CCU8x.OUTy1"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Ocs2::Value2
    }
}
#[doc = "Field `OCS2` writer - Output selector for CCU8x.OUTy1"]
pub type Ocs2W<'a, REG> = crate::BitWriter<'a, REG, Ocs2>;
impl<'a, REG> Ocs2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Inverted CC8yST1 signal path is connected to the CCU8x.OUTy1"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Ocs2::Value1)
    }
    #[doc = "CC8yST1 signal path is connected to the CCU8x.OUTy1"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Ocs2::Value2)
    }
}
#[doc = "Output selector for CCU8x.OUTy2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ocs3 {
    #[doc = "0: CC8yST2 signal path is connected to the CCU8x.OUTy2"]
    Value1 = 0,
    #[doc = "1: Inverted CCST2 signal path is connected to the CCU8x.OUTy2"]
    Value2 = 1,
}
impl From<Ocs3> for bool {
    #[inline(always)]
    fn from(variant: Ocs3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OCS3` reader - Output selector for CCU8x.OUTy2"]
pub type Ocs3R = crate::BitReader<Ocs3>;
impl Ocs3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ocs3 {
        match self.bits {
            false => Ocs3::Value1,
            true => Ocs3::Value2,
        }
    }
    #[doc = "CC8yST2 signal path is connected to the CCU8x.OUTy2"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Ocs3::Value1
    }
    #[doc = "Inverted CCST2 signal path is connected to the CCU8x.OUTy2"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Ocs3::Value2
    }
}
#[doc = "Field `OCS3` writer - Output selector for CCU8x.OUTy2"]
pub type Ocs3W<'a, REG> = crate::BitWriter<'a, REG, Ocs3>;
impl<'a, REG> Ocs3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CC8yST2 signal path is connected to the CCU8x.OUTy2"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Ocs3::Value1)
    }
    #[doc = "Inverted CCST2 signal path is connected to the CCU8x.OUTy2"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Ocs3::Value2)
    }
}
#[doc = "Output selector for CCU8x.OUTy3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ocs4 {
    #[doc = "0: Inverted CC8yST2 signal path is connected to the CCU8x.OUTy3"]
    Value1 = 0,
    #[doc = "1: CC8yST2 signal path is connected to the CCU8x.OUTy3"]
    Value2 = 1,
}
impl From<Ocs4> for bool {
    #[inline(always)]
    fn from(variant: Ocs4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OCS4` reader - Output selector for CCU8x.OUTy3"]
pub type Ocs4R = crate::BitReader<Ocs4>;
impl Ocs4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ocs4 {
        match self.bits {
            false => Ocs4::Value1,
            true => Ocs4::Value2,
        }
    }
    #[doc = "Inverted CC8yST2 signal path is connected to the CCU8x.OUTy3"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Ocs4::Value1
    }
    #[doc = "CC8yST2 signal path is connected to the CCU8x.OUTy3"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Ocs4::Value2
    }
}
#[doc = "Field `OCS4` writer - Output selector for CCU8x.OUTy3"]
pub type Ocs4W<'a, REG> = crate::BitWriter<'a, REG, Ocs4>;
impl<'a, REG> Ocs4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Inverted CC8yST2 signal path is connected to the CCU8x.OUTy3"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Ocs4::Value1)
    }
    #[doc = "CC8yST2 signal path is connected to the CCU8x.OUTy3"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Ocs4::Value2)
    }
}
impl R {
    #[doc = "Bit 0 - Asymmetric PWM mode Enable"]
    #[inline(always)]
    pub fn ase(&self) -> AseR {
        AseR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Output selector for CCU8x.OUTy0"]
    #[inline(always)]
    pub fn ocs1(&self) -> Ocs1R {
        Ocs1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Output selector for CCU8x.OUTy1"]
    #[inline(always)]
    pub fn ocs2(&self) -> Ocs2R {
        Ocs2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Output selector for CCU8x.OUTy2"]
    #[inline(always)]
    pub fn ocs3(&self) -> Ocs3R {
        Ocs3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Output selector for CCU8x.OUTy3"]
    #[inline(always)]
    pub fn ocs4(&self) -> Ocs4R {
        Ocs4R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Asymmetric PWM mode Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ase(&mut self) -> AseW<ChcSpec> {
        AseW::new(self, 0)
    }
    #[doc = "Bit 1 - Output selector for CCU8x.OUTy0"]
    #[inline(always)]
    #[must_use]
    pub fn ocs1(&mut self) -> Ocs1W<ChcSpec> {
        Ocs1W::new(self, 1)
    }
    #[doc = "Bit 2 - Output selector for CCU8x.OUTy1"]
    #[inline(always)]
    #[must_use]
    pub fn ocs2(&mut self) -> Ocs2W<ChcSpec> {
        Ocs2W::new(self, 2)
    }
    #[doc = "Bit 3 - Output selector for CCU8x.OUTy2"]
    #[inline(always)]
    #[must_use]
    pub fn ocs3(&mut self) -> Ocs3W<ChcSpec> {
        Ocs3W::new(self, 3)
    }
    #[doc = "Bit 4 - Output selector for CCU8x.OUTy3"]
    #[inline(always)]
    #[must_use]
    pub fn ocs4(&mut self) -> Ocs4W<ChcSpec> {
        Ocs4W::new(self, 4)
    }
}
#[doc = "Channel Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`chc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`chc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ChcSpec;
impl crate::RegisterSpec for ChcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`chc::R`](R) reader structure"]
impl crate::Readable for ChcSpec {}
#[doc = "`write(|w| ..)` method takes [`chc::W`](W) writer structure"]
impl crate::Writable for ChcSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CHC to value 0"]
impl crate::Resettable for ChcSpec {
    const RESET_VALUE: u32 = 0;
}
