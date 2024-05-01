#[doc = "Register `STC` reader"]
pub type R = crate::R<StcSpec>;
#[doc = "Register `STC` writer"]
pub type W = crate::W<StcSpec>;
#[doc = "Cascaded shadow transfer enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cse {
    #[doc = "0: Cascaded shadow transfer disabled"]
    Value1 = 0,
    #[doc = "1: Cascaded shadow transfer enabled"]
    Value2 = 1,
}
impl From<Cse> for bool {
    #[inline(always)]
    fn from(variant: Cse) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CSE` reader - Cascaded shadow transfer enable"]
pub type CseR = crate::BitReader<Cse>;
impl CseR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cse {
        match self.bits {
            false => Cse::Value1,
            true => Cse::Value2,
        }
    }
    #[doc = "Cascaded shadow transfer disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Cse::Value1
    }
    #[doc = "Cascaded shadow transfer enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Cse::Value2
    }
}
#[doc = "Field `CSE` writer - Cascaded shadow transfer enable"]
pub type CseW<'a, REG> = crate::BitWriter<'a, REG, Cse>;
impl<'a, REG> CseW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Cascaded shadow transfer disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Cse::Value1)
    }
    #[doc = "Cascaded shadow transfer enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Cse::Value2)
    }
}
#[doc = "Shadow transfer mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Stm {
    #[doc = "0: Shadow transfer is done in Period Match and One match."]
    Value1 = 0,
    #[doc = "1: Shadow transfer is done only in Period Match."]
    Value2 = 1,
    #[doc = "2: Shadow transfer is done only in One Match."]
    Value3 = 2,
}
impl From<Stm> for u8 {
    #[inline(always)]
    fn from(variant: Stm) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Stm {
    type Ux = u8;
}
impl crate::IsEnum for Stm {}
#[doc = "Field `STM` reader - Shadow transfer mode"]
pub type StmR = crate::FieldReader<Stm>;
impl StmR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Stm> {
        match self.bits {
            0 => Some(Stm::Value1),
            1 => Some(Stm::Value2),
            2 => Some(Stm::Value3),
            _ => None,
        }
    }
    #[doc = "Shadow transfer is done in Period Match and One match."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Stm::Value1
    }
    #[doc = "Shadow transfer is done only in Period Match."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Stm::Value2
    }
    #[doc = "Shadow transfer is done only in One Match."]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == Stm::Value3
    }
}
#[doc = "Field `STM` writer - Shadow transfer mode"]
pub type StmW<'a, REG> = crate::FieldWriter<'a, REG, 2, Stm>;
impl<'a, REG> StmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Shadow transfer is done in Period Match and One match."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Stm::Value1)
    }
    #[doc = "Shadow transfer is done only in Period Match."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Stm::Value2)
    }
    #[doc = "Shadow transfer is done only in One Match."]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(Stm::Value3)
    }
}
impl R {
    #[doc = "Bit 0 - Cascaded shadow transfer enable"]
    #[inline(always)]
    pub fn cse(&self) -> CseR {
        CseR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - Shadow transfer mode"]
    #[inline(always)]
    pub fn stm(&self) -> StmR {
        StmR::new(((self.bits >> 1) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Cascaded shadow transfer enable"]
    #[inline(always)]
    #[must_use]
    pub fn cse(&mut self) -> CseW<StcSpec> {
        CseW::new(self, 0)
    }
    #[doc = "Bits 1:2 - Shadow transfer mode"]
    #[inline(always)]
    #[must_use]
    pub fn stm(&mut self) -> StmW<StcSpec> {
        StmW::new(self, 1)
    }
}
#[doc = "Shadow transfer control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StcSpec;
impl crate::RegisterSpec for StcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stc::R`](R) reader structure"]
impl crate::Readable for StcSpec {}
#[doc = "`write(|w| ..)` method takes [`stc::W`](W) writer structure"]
impl crate::Writable for StcSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets STC to value 0"]
impl crate::Resettable for StcSpec {
    const RESET_VALUE: u32 = 0;
}
