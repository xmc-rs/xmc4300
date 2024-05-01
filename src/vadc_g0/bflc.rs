#[doc = "Register `BFLC` reader"]
pub type R = crate::R<BFLC_SPEC>;
#[doc = "Register `BFLC` writer"]
pub type W = crate::W<BFLC_SPEC>;
#[doc = "Boundary Flag y Mode Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum BFM0_A {
    #[doc = "0: Disable boundary flag, BFLy is not changed"]
    VALUE1 = 0,
    #[doc = "1: Always enable boundary flag (follow compare results)"]
    VALUE2 = 1,
    #[doc = "2: Enable boundary flag while gate of source 0 is active, clear BFLy while gate is inactive"]
    VALUE3 = 2,
    #[doc = "3: Enable boundary flag while gate of source 1 is active, clear BFLy while gate is inactive"]
    VALUE4 = 3,
}
impl From<BFM0_A> for u8 {
    #[inline(always)]
    fn from(variant: BFM0_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for BFM0_A {
    type Ux = u8;
}
impl crate::IsEnum for BFM0_A {}
#[doc = "Field `BFM0` reader - Boundary Flag y Mode Control"]
pub type BFM0_R = crate::FieldReader<BFM0_A>;
impl BFM0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<BFM0_A> {
        match self.bits {
            0 => Some(BFM0_A::VALUE1),
            1 => Some(BFM0_A::VALUE2),
            2 => Some(BFM0_A::VALUE3),
            3 => Some(BFM0_A::VALUE4),
            _ => None,
        }
    }
    #[doc = "Disable boundary flag, BFLy is not changed"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == BFM0_A::VALUE1
    }
    #[doc = "Always enable boundary flag (follow compare results)"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == BFM0_A::VALUE2
    }
    #[doc = "Enable boundary flag while gate of source 0 is active, clear BFLy while gate is inactive"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == BFM0_A::VALUE3
    }
    #[doc = "Enable boundary flag while gate of source 1 is active, clear BFLy while gate is inactive"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == BFM0_A::VALUE4
    }
}
#[doc = "Field `BFM0` writer - Boundary Flag y Mode Control"]
pub type BFM0_W<'a, REG> = crate::FieldWriter<'a, REG, 4, BFM0_A>;
impl<'a, REG> BFM0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Disable boundary flag, BFLy is not changed"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(BFM0_A::VALUE1)
    }
    #[doc = "Always enable boundary flag (follow compare results)"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(BFM0_A::VALUE2)
    }
    #[doc = "Enable boundary flag while gate of source 0 is active, clear BFLy while gate is inactive"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(BFM0_A::VALUE3)
    }
    #[doc = "Enable boundary flag while gate of source 1 is active, clear BFLy while gate is inactive"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(BFM0_A::VALUE4)
    }
}
#[doc = "Boundary Flag y Mode Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum BFM1_A {
    #[doc = "0: Disable boundary flag, BFLy is not changed"]
    VALUE1 = 0,
    #[doc = "1: Always enable boundary flag (follow compare results)"]
    VALUE2 = 1,
    #[doc = "2: Enable boundary flag while gate of source 0 is active, clear BFLy while gate is inactive"]
    VALUE3 = 2,
    #[doc = "3: Enable boundary flag while gate of source 1 is active, clear BFLy while gate is inactive"]
    VALUE4 = 3,
}
impl From<BFM1_A> for u8 {
    #[inline(always)]
    fn from(variant: BFM1_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for BFM1_A {
    type Ux = u8;
}
impl crate::IsEnum for BFM1_A {}
#[doc = "Field `BFM1` reader - Boundary Flag y Mode Control"]
pub type BFM1_R = crate::FieldReader<BFM1_A>;
impl BFM1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<BFM1_A> {
        match self.bits {
            0 => Some(BFM1_A::VALUE1),
            1 => Some(BFM1_A::VALUE2),
            2 => Some(BFM1_A::VALUE3),
            3 => Some(BFM1_A::VALUE4),
            _ => None,
        }
    }
    #[doc = "Disable boundary flag, BFLy is not changed"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == BFM1_A::VALUE1
    }
    #[doc = "Always enable boundary flag (follow compare results)"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == BFM1_A::VALUE2
    }
    #[doc = "Enable boundary flag while gate of source 0 is active, clear BFLy while gate is inactive"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == BFM1_A::VALUE3
    }
    #[doc = "Enable boundary flag while gate of source 1 is active, clear BFLy while gate is inactive"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == BFM1_A::VALUE4
    }
}
#[doc = "Field `BFM1` writer - Boundary Flag y Mode Control"]
pub type BFM1_W<'a, REG> = crate::FieldWriter<'a, REG, 4, BFM1_A>;
impl<'a, REG> BFM1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Disable boundary flag, BFLy is not changed"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(BFM1_A::VALUE1)
    }
    #[doc = "Always enable boundary flag (follow compare results)"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(BFM1_A::VALUE2)
    }
    #[doc = "Enable boundary flag while gate of source 0 is active, clear BFLy while gate is inactive"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(BFM1_A::VALUE3)
    }
    #[doc = "Enable boundary flag while gate of source 1 is active, clear BFLy while gate is inactive"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(BFM1_A::VALUE4)
    }
}
#[doc = "Boundary Flag y Mode Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum BFM2_A {
    #[doc = "0: Disable boundary flag, BFLy is not changed"]
    VALUE1 = 0,
    #[doc = "1: Always enable boundary flag (follow compare results)"]
    VALUE2 = 1,
    #[doc = "2: Enable boundary flag while gate of source 0 is active, clear BFLy while gate is inactive"]
    VALUE3 = 2,
    #[doc = "3: Enable boundary flag while gate of source 1 is active, clear BFLy while gate is inactive"]
    VALUE4 = 3,
}
impl From<BFM2_A> for u8 {
    #[inline(always)]
    fn from(variant: BFM2_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for BFM2_A {
    type Ux = u8;
}
impl crate::IsEnum for BFM2_A {}
#[doc = "Field `BFM2` reader - Boundary Flag y Mode Control"]
pub type BFM2_R = crate::FieldReader<BFM2_A>;
impl BFM2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<BFM2_A> {
        match self.bits {
            0 => Some(BFM2_A::VALUE1),
            1 => Some(BFM2_A::VALUE2),
            2 => Some(BFM2_A::VALUE3),
            3 => Some(BFM2_A::VALUE4),
            _ => None,
        }
    }
    #[doc = "Disable boundary flag, BFLy is not changed"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == BFM2_A::VALUE1
    }
    #[doc = "Always enable boundary flag (follow compare results)"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == BFM2_A::VALUE2
    }
    #[doc = "Enable boundary flag while gate of source 0 is active, clear BFLy while gate is inactive"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == BFM2_A::VALUE3
    }
    #[doc = "Enable boundary flag while gate of source 1 is active, clear BFLy while gate is inactive"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == BFM2_A::VALUE4
    }
}
#[doc = "Field `BFM2` writer - Boundary Flag y Mode Control"]
pub type BFM2_W<'a, REG> = crate::FieldWriter<'a, REG, 4, BFM2_A>;
impl<'a, REG> BFM2_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Disable boundary flag, BFLy is not changed"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(BFM2_A::VALUE1)
    }
    #[doc = "Always enable boundary flag (follow compare results)"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(BFM2_A::VALUE2)
    }
    #[doc = "Enable boundary flag while gate of source 0 is active, clear BFLy while gate is inactive"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(BFM2_A::VALUE3)
    }
    #[doc = "Enable boundary flag while gate of source 1 is active, clear BFLy while gate is inactive"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(BFM2_A::VALUE4)
    }
}
#[doc = "Boundary Flag y Mode Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum BFM3_A {
    #[doc = "0: Disable boundary flag, BFLy is not changed"]
    VALUE1 = 0,
    #[doc = "1: Always enable boundary flag (follow compare results)"]
    VALUE2 = 1,
    #[doc = "2: Enable boundary flag while gate of source 0 is active, clear BFLy while gate is inactive"]
    VALUE3 = 2,
    #[doc = "3: Enable boundary flag while gate of source 1 is active, clear BFLy while gate is inactive"]
    VALUE4 = 3,
}
impl From<BFM3_A> for u8 {
    #[inline(always)]
    fn from(variant: BFM3_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for BFM3_A {
    type Ux = u8;
}
impl crate::IsEnum for BFM3_A {}
#[doc = "Field `BFM3` reader - Boundary Flag y Mode Control"]
pub type BFM3_R = crate::FieldReader<BFM3_A>;
impl BFM3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<BFM3_A> {
        match self.bits {
            0 => Some(BFM3_A::VALUE1),
            1 => Some(BFM3_A::VALUE2),
            2 => Some(BFM3_A::VALUE3),
            3 => Some(BFM3_A::VALUE4),
            _ => None,
        }
    }
    #[doc = "Disable boundary flag, BFLy is not changed"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == BFM3_A::VALUE1
    }
    #[doc = "Always enable boundary flag (follow compare results)"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == BFM3_A::VALUE2
    }
    #[doc = "Enable boundary flag while gate of source 0 is active, clear BFLy while gate is inactive"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == BFM3_A::VALUE3
    }
    #[doc = "Enable boundary flag while gate of source 1 is active, clear BFLy while gate is inactive"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == BFM3_A::VALUE4
    }
}
#[doc = "Field `BFM3` writer - Boundary Flag y Mode Control"]
pub type BFM3_W<'a, REG> = crate::FieldWriter<'a, REG, 4, BFM3_A>;
impl<'a, REG> BFM3_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Disable boundary flag, BFLy is not changed"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(BFM3_A::VALUE1)
    }
    #[doc = "Always enable boundary flag (follow compare results)"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(BFM3_A::VALUE2)
    }
    #[doc = "Enable boundary flag while gate of source 0 is active, clear BFLy while gate is inactive"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(BFM3_A::VALUE3)
    }
    #[doc = "Enable boundary flag while gate of source 1 is active, clear BFLy while gate is inactive"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(BFM3_A::VALUE4)
    }
}
impl R {
    #[doc = "Bits 0:3 - Boundary Flag y Mode Control"]
    #[inline(always)]
    pub fn bfm0(&self) -> BFM0_R {
        BFM0_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Boundary Flag y Mode Control"]
    #[inline(always)]
    pub fn bfm1(&self) -> BFM1_R {
        BFM1_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Boundary Flag y Mode Control"]
    #[inline(always)]
    pub fn bfm2(&self) -> BFM2_R {
        BFM2_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Boundary Flag y Mode Control"]
    #[inline(always)]
    pub fn bfm3(&self) -> BFM3_R {
        BFM3_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Boundary Flag y Mode Control"]
    #[inline(always)]
    #[must_use]
    pub fn bfm0(&mut self) -> BFM0_W<BFLC_SPEC> {
        BFM0_W::new(self, 0)
    }
    #[doc = "Bits 4:7 - Boundary Flag y Mode Control"]
    #[inline(always)]
    #[must_use]
    pub fn bfm1(&mut self) -> BFM1_W<BFLC_SPEC> {
        BFM1_W::new(self, 4)
    }
    #[doc = "Bits 8:11 - Boundary Flag y Mode Control"]
    #[inline(always)]
    #[must_use]
    pub fn bfm2(&mut self) -> BFM2_W<BFLC_SPEC> {
        BFM2_W::new(self, 8)
    }
    #[doc = "Bits 12:15 - Boundary Flag y Mode Control"]
    #[inline(always)]
    #[must_use]
    pub fn bfm3(&mut self) -> BFM3_W<BFLC_SPEC> {
        BFM3_W::new(self, 12)
    }
}
#[doc = "Boundary Flag Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bflc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bflc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BFLC_SPEC;
impl crate::RegisterSpec for BFLC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bflc::R`](R) reader structure"]
impl crate::Readable for BFLC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`bflc::W`](W) writer structure"]
impl crate::Writable for BFLC_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BFLC to value 0"]
impl crate::Resettable for BFLC_SPEC {
    const RESET_VALUE: u32 = 0;
}
