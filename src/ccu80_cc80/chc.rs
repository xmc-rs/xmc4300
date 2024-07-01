#[doc = "Register `CHC` reader"]
pub type R = crate::R<CHC_SPEC>;
#[doc = "Register `CHC` writer"]
pub type W = crate::W<CHC_SPEC>;
#[doc = "Asymmetric PWM mode Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ASE_A {
    #[doc = "0: Asymmetric PWM is disabled"]
    VALUE1 = 0,
    #[doc = "1: Asymmetric PWM is enabled"]
    VALUE2 = 1,
}
impl From<ASE_A> for bool {
    #[inline(always)]
    fn from(variant: ASE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ASE` reader - Asymmetric PWM mode Enable"]
pub type ASE_R = crate::BitReader<ASE_A>;
impl ASE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ASE_A {
        match self.bits {
            false => ASE_A::VALUE1,
            true => ASE_A::VALUE2,
        }
    }
    #[doc = "Asymmetric PWM is disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == ASE_A::VALUE1
    }
    #[doc = "Asymmetric PWM is enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == ASE_A::VALUE2
    }
}
#[doc = "Field `ASE` writer - Asymmetric PWM mode Enable"]
pub type ASE_W<'a, REG> = crate::BitWriter<'a, REG, ASE_A>;
impl<'a, REG> ASE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Asymmetric PWM is disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(ASE_A::VALUE1)
    }
    #[doc = "Asymmetric PWM is enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(ASE_A::VALUE2)
    }
}
#[doc = "Output selector for CCU8x.OUTy0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OCS1_A {
    #[doc = "0: CC8yST1 signal path is connected to the CCU8x.OUTy0"]
    VALUE1 = 0,
    #[doc = "1: Inverted CC8yST1 signal path is connected to the CCU8x.OUTy0"]
    VALUE2 = 1,
}
impl From<OCS1_A> for bool {
    #[inline(always)]
    fn from(variant: OCS1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OCS1` reader - Output selector for CCU8x.OUTy0"]
pub type OCS1_R = crate::BitReader<OCS1_A>;
impl OCS1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> OCS1_A {
        match self.bits {
            false => OCS1_A::VALUE1,
            true => OCS1_A::VALUE2,
        }
    }
    #[doc = "CC8yST1 signal path is connected to the CCU8x.OUTy0"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == OCS1_A::VALUE1
    }
    #[doc = "Inverted CC8yST1 signal path is connected to the CCU8x.OUTy0"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == OCS1_A::VALUE2
    }
}
#[doc = "Field `OCS1` writer - Output selector for CCU8x.OUTy0"]
pub type OCS1_W<'a, REG> = crate::BitWriter<'a, REG, OCS1_A>;
impl<'a, REG> OCS1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CC8yST1 signal path is connected to the CCU8x.OUTy0"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(OCS1_A::VALUE1)
    }
    #[doc = "Inverted CC8yST1 signal path is connected to the CCU8x.OUTy0"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(OCS1_A::VALUE2)
    }
}
#[doc = "Output selector for CCU8x.OUTy1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OCS2_A {
    #[doc = "0: Inverted CC8yST1 signal path is connected to the CCU8x.OUTy1"]
    VALUE1 = 0,
    #[doc = "1: CC8yST1 signal path is connected to the CCU8x.OUTy1"]
    VALUE2 = 1,
}
impl From<OCS2_A> for bool {
    #[inline(always)]
    fn from(variant: OCS2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OCS2` reader - Output selector for CCU8x.OUTy1"]
pub type OCS2_R = crate::BitReader<OCS2_A>;
impl OCS2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> OCS2_A {
        match self.bits {
            false => OCS2_A::VALUE1,
            true => OCS2_A::VALUE2,
        }
    }
    #[doc = "Inverted CC8yST1 signal path is connected to the CCU8x.OUTy1"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == OCS2_A::VALUE1
    }
    #[doc = "CC8yST1 signal path is connected to the CCU8x.OUTy1"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == OCS2_A::VALUE2
    }
}
#[doc = "Field `OCS2` writer - Output selector for CCU8x.OUTy1"]
pub type OCS2_W<'a, REG> = crate::BitWriter<'a, REG, OCS2_A>;
impl<'a, REG> OCS2_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Inverted CC8yST1 signal path is connected to the CCU8x.OUTy1"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(OCS2_A::VALUE1)
    }
    #[doc = "CC8yST1 signal path is connected to the CCU8x.OUTy1"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(OCS2_A::VALUE2)
    }
}
#[doc = "Output selector for CCU8x.OUTy2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OCS3_A {
    #[doc = "0: CC8yST2 signal path is connected to the CCU8x.OUTy2"]
    VALUE1 = 0,
    #[doc = "1: Inverted CCST2 signal path is connected to the CCU8x.OUTy2"]
    VALUE2 = 1,
}
impl From<OCS3_A> for bool {
    #[inline(always)]
    fn from(variant: OCS3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OCS3` reader - Output selector for CCU8x.OUTy2"]
pub type OCS3_R = crate::BitReader<OCS3_A>;
impl OCS3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> OCS3_A {
        match self.bits {
            false => OCS3_A::VALUE1,
            true => OCS3_A::VALUE2,
        }
    }
    #[doc = "CC8yST2 signal path is connected to the CCU8x.OUTy2"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == OCS3_A::VALUE1
    }
    #[doc = "Inverted CCST2 signal path is connected to the CCU8x.OUTy2"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == OCS3_A::VALUE2
    }
}
#[doc = "Field `OCS3` writer - Output selector for CCU8x.OUTy2"]
pub type OCS3_W<'a, REG> = crate::BitWriter<'a, REG, OCS3_A>;
impl<'a, REG> OCS3_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CC8yST2 signal path is connected to the CCU8x.OUTy2"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(OCS3_A::VALUE1)
    }
    #[doc = "Inverted CCST2 signal path is connected to the CCU8x.OUTy2"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(OCS3_A::VALUE2)
    }
}
#[doc = "Output selector for CCU8x.OUTy3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OCS4_A {
    #[doc = "0: Inverted CC8yST2 signal path is connected to the CCU8x.OUTy3"]
    VALUE1 = 0,
    #[doc = "1: CC8yST2 signal path is connected to the CCU8x.OUTy3"]
    VALUE2 = 1,
}
impl From<OCS4_A> for bool {
    #[inline(always)]
    fn from(variant: OCS4_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OCS4` reader - Output selector for CCU8x.OUTy3"]
pub type OCS4_R = crate::BitReader<OCS4_A>;
impl OCS4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> OCS4_A {
        match self.bits {
            false => OCS4_A::VALUE1,
            true => OCS4_A::VALUE2,
        }
    }
    #[doc = "Inverted CC8yST2 signal path is connected to the CCU8x.OUTy3"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == OCS4_A::VALUE1
    }
    #[doc = "CC8yST2 signal path is connected to the CCU8x.OUTy3"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == OCS4_A::VALUE2
    }
}
#[doc = "Field `OCS4` writer - Output selector for CCU8x.OUTy3"]
pub type OCS4_W<'a, REG> = crate::BitWriter<'a, REG, OCS4_A>;
impl<'a, REG> OCS4_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Inverted CC8yST2 signal path is connected to the CCU8x.OUTy3"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(OCS4_A::VALUE1)
    }
    #[doc = "CC8yST2 signal path is connected to the CCU8x.OUTy3"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(OCS4_A::VALUE2)
    }
}
impl R {
    #[doc = "Bit 0 - Asymmetric PWM mode Enable"]
    #[inline(always)]
    pub fn ase(&self) -> ASE_R {
        ASE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Output selector for CCU8x.OUTy0"]
    #[inline(always)]
    pub fn ocs1(&self) -> OCS1_R {
        OCS1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Output selector for CCU8x.OUTy1"]
    #[inline(always)]
    pub fn ocs2(&self) -> OCS2_R {
        OCS2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Output selector for CCU8x.OUTy2"]
    #[inline(always)]
    pub fn ocs3(&self) -> OCS3_R {
        OCS3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Output selector for CCU8x.OUTy3"]
    #[inline(always)]
    pub fn ocs4(&self) -> OCS4_R {
        OCS4_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Asymmetric PWM mode Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ase(&mut self) -> ASE_W<CHC_SPEC> {
        ASE_W::new(self, 0)
    }
    #[doc = "Bit 1 - Output selector for CCU8x.OUTy0"]
    #[inline(always)]
    #[must_use]
    pub fn ocs1(&mut self) -> OCS1_W<CHC_SPEC> {
        OCS1_W::new(self, 1)
    }
    #[doc = "Bit 2 - Output selector for CCU8x.OUTy1"]
    #[inline(always)]
    #[must_use]
    pub fn ocs2(&mut self) -> OCS2_W<CHC_SPEC> {
        OCS2_W::new(self, 2)
    }
    #[doc = "Bit 3 - Output selector for CCU8x.OUTy2"]
    #[inline(always)]
    #[must_use]
    pub fn ocs3(&mut self) -> OCS3_W<CHC_SPEC> {
        OCS3_W::new(self, 3)
    }
    #[doc = "Bit 4 - Output selector for CCU8x.OUTy3"]
    #[inline(always)]
    #[must_use]
    pub fn ocs4(&mut self) -> OCS4_W<CHC_SPEC> {
        OCS4_W::new(self, 4)
    }
}
#[doc = "Channel Control\n\nYou can [`read`](crate::Reg::read) this register and get [`chc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CHC_SPEC;
impl crate::RegisterSpec for CHC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`chc::R`](R) reader structure"]
impl crate::Readable for CHC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`chc::W`](W) writer structure"]
impl crate::Writable for CHC_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CHC to value 0"]
impl crate::Resettable for CHC_SPEC {
    const RESET_VALUE: u32 = 0;
}
