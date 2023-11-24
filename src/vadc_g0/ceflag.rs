#[doc = "Register `CEFLAG` reader"]
pub type R = crate::R<CEFLAG_SPEC>;
#[doc = "Register `CEFLAG` writer"]
pub type W = crate::W<CEFLAG_SPEC>;
#[doc = "Field `CEV0` reader - Channel Event for Channel 0"]
pub type CEV0_R = crate::BitReader<CEV0_A>;
#[doc = "Channel Event for Channel 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CEV0_A {
    #[doc = "0: No channel event"]
    VALUE1 = 0,
    #[doc = "1: A channel event has occurred"]
    VALUE2 = 1,
}
impl From<CEV0_A> for bool {
    #[inline(always)]
    fn from(variant: CEV0_A) -> Self {
        variant as u8 != 0
    }
}
impl CEV0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CEV0_A {
        match self.bits {
            false => CEV0_A::VALUE1,
            true => CEV0_A::VALUE2,
        }
    }
    #[doc = "No channel event"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CEV0_A::VALUE1
    }
    #[doc = "A channel event has occurred"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CEV0_A::VALUE2
    }
}
#[doc = "Field `CEV0` writer - Channel Event for Channel 0"]
pub type CEV0_W<'a, REG> = crate::BitWriter<'a, REG, CEV0_A>;
impl<'a, REG> CEV0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No channel event"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(CEV0_A::VALUE1)
    }
    #[doc = "A channel event has occurred"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(CEV0_A::VALUE2)
    }
}
#[doc = "Field `CEV1` reader - Channel Event for Channel 1"]
pub type CEV1_R = crate::BitReader<CEV1_A>;
#[doc = "Channel Event for Channel 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CEV1_A {
    #[doc = "0: No channel event"]
    VALUE1 = 0,
    #[doc = "1: A channel event has occurred"]
    VALUE2 = 1,
}
impl From<CEV1_A> for bool {
    #[inline(always)]
    fn from(variant: CEV1_A) -> Self {
        variant as u8 != 0
    }
}
impl CEV1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CEV1_A {
        match self.bits {
            false => CEV1_A::VALUE1,
            true => CEV1_A::VALUE2,
        }
    }
    #[doc = "No channel event"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CEV1_A::VALUE1
    }
    #[doc = "A channel event has occurred"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CEV1_A::VALUE2
    }
}
#[doc = "Field `CEV1` writer - Channel Event for Channel 1"]
pub type CEV1_W<'a, REG> = crate::BitWriter<'a, REG, CEV1_A>;
impl<'a, REG> CEV1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No channel event"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(CEV1_A::VALUE1)
    }
    #[doc = "A channel event has occurred"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(CEV1_A::VALUE2)
    }
}
#[doc = "Field `CEV2` reader - Channel Event for Channel 2"]
pub type CEV2_R = crate::BitReader<CEV2_A>;
#[doc = "Channel Event for Channel 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CEV2_A {
    #[doc = "0: No channel event"]
    VALUE1 = 0,
    #[doc = "1: A channel event has occurred"]
    VALUE2 = 1,
}
impl From<CEV2_A> for bool {
    #[inline(always)]
    fn from(variant: CEV2_A) -> Self {
        variant as u8 != 0
    }
}
impl CEV2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CEV2_A {
        match self.bits {
            false => CEV2_A::VALUE1,
            true => CEV2_A::VALUE2,
        }
    }
    #[doc = "No channel event"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CEV2_A::VALUE1
    }
    #[doc = "A channel event has occurred"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CEV2_A::VALUE2
    }
}
#[doc = "Field `CEV2` writer - Channel Event for Channel 2"]
pub type CEV2_W<'a, REG> = crate::BitWriter<'a, REG, CEV2_A>;
impl<'a, REG> CEV2_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No channel event"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(CEV2_A::VALUE1)
    }
    #[doc = "A channel event has occurred"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(CEV2_A::VALUE2)
    }
}
#[doc = "Field `CEV3` reader - Channel Event for Channel 3"]
pub type CEV3_R = crate::BitReader<CEV3_A>;
#[doc = "Channel Event for Channel 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CEV3_A {
    #[doc = "0: No channel event"]
    VALUE1 = 0,
    #[doc = "1: A channel event has occurred"]
    VALUE2 = 1,
}
impl From<CEV3_A> for bool {
    #[inline(always)]
    fn from(variant: CEV3_A) -> Self {
        variant as u8 != 0
    }
}
impl CEV3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CEV3_A {
        match self.bits {
            false => CEV3_A::VALUE1,
            true => CEV3_A::VALUE2,
        }
    }
    #[doc = "No channel event"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CEV3_A::VALUE1
    }
    #[doc = "A channel event has occurred"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CEV3_A::VALUE2
    }
}
#[doc = "Field `CEV3` writer - Channel Event for Channel 3"]
pub type CEV3_W<'a, REG> = crate::BitWriter<'a, REG, CEV3_A>;
impl<'a, REG> CEV3_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No channel event"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(CEV3_A::VALUE1)
    }
    #[doc = "A channel event has occurred"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(CEV3_A::VALUE2)
    }
}
#[doc = "Field `CEV4` reader - Channel Event for Channel 4"]
pub type CEV4_R = crate::BitReader<CEV4_A>;
#[doc = "Channel Event for Channel 4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CEV4_A {
    #[doc = "0: No channel event"]
    VALUE1 = 0,
    #[doc = "1: A channel event has occurred"]
    VALUE2 = 1,
}
impl From<CEV4_A> for bool {
    #[inline(always)]
    fn from(variant: CEV4_A) -> Self {
        variant as u8 != 0
    }
}
impl CEV4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CEV4_A {
        match self.bits {
            false => CEV4_A::VALUE1,
            true => CEV4_A::VALUE2,
        }
    }
    #[doc = "No channel event"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CEV4_A::VALUE1
    }
    #[doc = "A channel event has occurred"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CEV4_A::VALUE2
    }
}
#[doc = "Field `CEV4` writer - Channel Event for Channel 4"]
pub type CEV4_W<'a, REG> = crate::BitWriter<'a, REG, CEV4_A>;
impl<'a, REG> CEV4_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No channel event"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(CEV4_A::VALUE1)
    }
    #[doc = "A channel event has occurred"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(CEV4_A::VALUE2)
    }
}
#[doc = "Field `CEV5` reader - Channel Event for Channel 5"]
pub type CEV5_R = crate::BitReader<CEV5_A>;
#[doc = "Channel Event for Channel 5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CEV5_A {
    #[doc = "0: No channel event"]
    VALUE1 = 0,
    #[doc = "1: A channel event has occurred"]
    VALUE2 = 1,
}
impl From<CEV5_A> for bool {
    #[inline(always)]
    fn from(variant: CEV5_A) -> Self {
        variant as u8 != 0
    }
}
impl CEV5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CEV5_A {
        match self.bits {
            false => CEV5_A::VALUE1,
            true => CEV5_A::VALUE2,
        }
    }
    #[doc = "No channel event"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CEV5_A::VALUE1
    }
    #[doc = "A channel event has occurred"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CEV5_A::VALUE2
    }
}
#[doc = "Field `CEV5` writer - Channel Event for Channel 5"]
pub type CEV5_W<'a, REG> = crate::BitWriter<'a, REG, CEV5_A>;
impl<'a, REG> CEV5_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No channel event"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(CEV5_A::VALUE1)
    }
    #[doc = "A channel event has occurred"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(CEV5_A::VALUE2)
    }
}
#[doc = "Field `CEV6` reader - Channel Event for Channel 6"]
pub type CEV6_R = crate::BitReader<CEV6_A>;
#[doc = "Channel Event for Channel 6\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CEV6_A {
    #[doc = "0: No channel event"]
    VALUE1 = 0,
    #[doc = "1: A channel event has occurred"]
    VALUE2 = 1,
}
impl From<CEV6_A> for bool {
    #[inline(always)]
    fn from(variant: CEV6_A) -> Self {
        variant as u8 != 0
    }
}
impl CEV6_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CEV6_A {
        match self.bits {
            false => CEV6_A::VALUE1,
            true => CEV6_A::VALUE2,
        }
    }
    #[doc = "No channel event"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CEV6_A::VALUE1
    }
    #[doc = "A channel event has occurred"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CEV6_A::VALUE2
    }
}
#[doc = "Field `CEV6` writer - Channel Event for Channel 6"]
pub type CEV6_W<'a, REG> = crate::BitWriter<'a, REG, CEV6_A>;
impl<'a, REG> CEV6_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No channel event"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(CEV6_A::VALUE1)
    }
    #[doc = "A channel event has occurred"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(CEV6_A::VALUE2)
    }
}
#[doc = "Field `CEV7` reader - Channel Event for Channel 7"]
pub type CEV7_R = crate::BitReader<CEV7_A>;
#[doc = "Channel Event for Channel 7\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CEV7_A {
    #[doc = "0: No channel event"]
    VALUE1 = 0,
    #[doc = "1: A channel event has occurred"]
    VALUE2 = 1,
}
impl From<CEV7_A> for bool {
    #[inline(always)]
    fn from(variant: CEV7_A) -> Self {
        variant as u8 != 0
    }
}
impl CEV7_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CEV7_A {
        match self.bits {
            false => CEV7_A::VALUE1,
            true => CEV7_A::VALUE2,
        }
    }
    #[doc = "No channel event"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CEV7_A::VALUE1
    }
    #[doc = "A channel event has occurred"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CEV7_A::VALUE2
    }
}
#[doc = "Field `CEV7` writer - Channel Event for Channel 7"]
pub type CEV7_W<'a, REG> = crate::BitWriter<'a, REG, CEV7_A>;
impl<'a, REG> CEV7_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No channel event"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(CEV7_A::VALUE1)
    }
    #[doc = "A channel event has occurred"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(CEV7_A::VALUE2)
    }
}
impl R {
    #[doc = "Bit 0 - Channel Event for Channel 0"]
    #[inline(always)]
    pub fn cev0(&self) -> CEV0_R {
        CEV0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Channel Event for Channel 1"]
    #[inline(always)]
    pub fn cev1(&self) -> CEV1_R {
        CEV1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Channel Event for Channel 2"]
    #[inline(always)]
    pub fn cev2(&self) -> CEV2_R {
        CEV2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Channel Event for Channel 3"]
    #[inline(always)]
    pub fn cev3(&self) -> CEV3_R {
        CEV3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Channel Event for Channel 4"]
    #[inline(always)]
    pub fn cev4(&self) -> CEV4_R {
        CEV4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Channel Event for Channel 5"]
    #[inline(always)]
    pub fn cev5(&self) -> CEV5_R {
        CEV5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Channel Event for Channel 6"]
    #[inline(always)]
    pub fn cev6(&self) -> CEV6_R {
        CEV6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Channel Event for Channel 7"]
    #[inline(always)]
    pub fn cev7(&self) -> CEV7_R {
        CEV7_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Channel Event for Channel 0"]
    #[inline(always)]
    #[must_use]
    pub fn cev0(&mut self) -> CEV0_W<CEFLAG_SPEC> {
        CEV0_W::new(self, 0)
    }
    #[doc = "Bit 1 - Channel Event for Channel 1"]
    #[inline(always)]
    #[must_use]
    pub fn cev1(&mut self) -> CEV1_W<CEFLAG_SPEC> {
        CEV1_W::new(self, 1)
    }
    #[doc = "Bit 2 - Channel Event for Channel 2"]
    #[inline(always)]
    #[must_use]
    pub fn cev2(&mut self) -> CEV2_W<CEFLAG_SPEC> {
        CEV2_W::new(self, 2)
    }
    #[doc = "Bit 3 - Channel Event for Channel 3"]
    #[inline(always)]
    #[must_use]
    pub fn cev3(&mut self) -> CEV3_W<CEFLAG_SPEC> {
        CEV3_W::new(self, 3)
    }
    #[doc = "Bit 4 - Channel Event for Channel 4"]
    #[inline(always)]
    #[must_use]
    pub fn cev4(&mut self) -> CEV4_W<CEFLAG_SPEC> {
        CEV4_W::new(self, 4)
    }
    #[doc = "Bit 5 - Channel Event for Channel 5"]
    #[inline(always)]
    #[must_use]
    pub fn cev5(&mut self) -> CEV5_W<CEFLAG_SPEC> {
        CEV5_W::new(self, 5)
    }
    #[doc = "Bit 6 - Channel Event for Channel 6"]
    #[inline(always)]
    #[must_use]
    pub fn cev6(&mut self) -> CEV6_W<CEFLAG_SPEC> {
        CEV6_W::new(self, 6)
    }
    #[doc = "Bit 7 - Channel Event for Channel 7"]
    #[inline(always)]
    #[must_use]
    pub fn cev7(&mut self) -> CEV7_W<CEFLAG_SPEC> {
        CEV7_W::new(self, 7)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Channel Event Flag Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ceflag::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ceflag::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CEFLAG_SPEC;
impl crate::RegisterSpec for CEFLAG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ceflag::R`](R) reader structure"]
impl crate::Readable for CEFLAG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ceflag::W`](W) writer structure"]
impl crate::Writable for CEFLAG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CEFLAG to value 0"]
impl crate::Resettable for CEFLAG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
