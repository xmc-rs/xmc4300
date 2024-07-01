#[doc = "Register `STC` reader"]
pub type R = crate::R<STC_SPEC>;
#[doc = "Register `STC` writer"]
pub type W = crate::W<STC_SPEC>;
#[doc = "Cascaded shadow transfer enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSE_A {
    #[doc = "0: Cascaded shadow transfer disabled"]
    VALUE1 = 0,
    #[doc = "1: Cascaded shadow transfer enabled"]
    VALUE2 = 1,
}
impl From<CSE_A> for bool {
    #[inline(always)]
    fn from(variant: CSE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CSE` reader - Cascaded shadow transfer enable"]
pub type CSE_R = crate::BitReader<CSE_A>;
impl CSE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CSE_A {
        match self.bits {
            false => CSE_A::VALUE1,
            true => CSE_A::VALUE2,
        }
    }
    #[doc = "Cascaded shadow transfer disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CSE_A::VALUE1
    }
    #[doc = "Cascaded shadow transfer enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CSE_A::VALUE2
    }
}
#[doc = "Field `CSE` writer - Cascaded shadow transfer enable"]
pub type CSE_W<'a, REG> = crate::BitWriter<'a, REG, CSE_A>;
impl<'a, REG> CSE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Cascaded shadow transfer disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(CSE_A::VALUE1)
    }
    #[doc = "Cascaded shadow transfer enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(CSE_A::VALUE2)
    }
}
#[doc = "Shadow transfer mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum STM_A {
    #[doc = "0: Shadow transfer is done in Period Match and One match."]
    VALUE1 = 0,
    #[doc = "1: Shadow transfer is done only in Period Match."]
    VALUE2 = 1,
    #[doc = "2: Shadow transfer is done only in One Match."]
    VALUE3 = 2,
}
impl From<STM_A> for u8 {
    #[inline(always)]
    fn from(variant: STM_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for STM_A {
    type Ux = u8;
}
impl crate::IsEnum for STM_A {}
#[doc = "Field `STM` reader - Shadow transfer mode"]
pub type STM_R = crate::FieldReader<STM_A>;
impl STM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<STM_A> {
        match self.bits {
            0 => Some(STM_A::VALUE1),
            1 => Some(STM_A::VALUE2),
            2 => Some(STM_A::VALUE3),
            _ => None,
        }
    }
    #[doc = "Shadow transfer is done in Period Match and One match."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == STM_A::VALUE1
    }
    #[doc = "Shadow transfer is done only in Period Match."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == STM_A::VALUE2
    }
    #[doc = "Shadow transfer is done only in One Match."]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == STM_A::VALUE3
    }
}
#[doc = "Field `STM` writer - Shadow transfer mode"]
pub type STM_W<'a, REG> = crate::FieldWriter<'a, REG, 2, STM_A>;
impl<'a, REG> STM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Shadow transfer is done in Period Match and One match."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(STM_A::VALUE1)
    }
    #[doc = "Shadow transfer is done only in Period Match."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(STM_A::VALUE2)
    }
    #[doc = "Shadow transfer is done only in One Match."]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(STM_A::VALUE3)
    }
}
impl R {
    #[doc = "Bit 0 - Cascaded shadow transfer enable"]
    #[inline(always)]
    pub fn cse(&self) -> CSE_R {
        CSE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - Shadow transfer mode"]
    #[inline(always)]
    pub fn stm(&self) -> STM_R {
        STM_R::new(((self.bits >> 1) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Cascaded shadow transfer enable"]
    #[inline(always)]
    #[must_use]
    pub fn cse(&mut self) -> CSE_W<STC_SPEC> {
        CSE_W::new(self, 0)
    }
    #[doc = "Bits 1:2 - Shadow transfer mode"]
    #[inline(always)]
    #[must_use]
    pub fn stm(&mut self) -> STM_W<STC_SPEC> {
        STM_W::new(self, 1)
    }
}
#[doc = "Shadow transfer control\n\nYou can [`read`](crate::Reg::read) this register and get [`stc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`stc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STC_SPEC;
impl crate::RegisterSpec for STC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stc::R`](R) reader structure"]
impl crate::Readable for STC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`stc::W`](W) writer structure"]
impl crate::Writable for STC_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets STC to value 0"]
impl crate::Resettable for STC_SPEC {
    const RESET_VALUE: u32 = 0;
}
