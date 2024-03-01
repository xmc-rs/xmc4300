#[doc = "Register `DC_LATCH1_CONT` reader"]
pub type R = crate::R<DcLatch1ContSpec>;
#[doc = "Register `DC_LATCH1_CONT` writer"]
pub type W = crate::W<DcLatch1ContSpec>;
#[doc = "Latch1 positive edge\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum L1Pos {
    #[doc = "0: Continuous Latch active"]
    Value1 = 0,
    #[doc = "1: Single event (only first event active)"]
    Value2 = 1,
}
impl From<L1Pos> for bool {
    #[inline(always)]
    fn from(variant: L1Pos) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `L1_POS` reader - Latch1 positive edge"]
pub type L1PosR = crate::BitReader<L1Pos>;
impl L1PosR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> L1Pos {
        match self.bits {
            false => L1Pos::Value1,
            true => L1Pos::Value2,
        }
    }
    #[doc = "Continuous Latch active"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == L1Pos::Value1
    }
    #[doc = "Single event (only first event active)"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == L1Pos::Value2
    }
}
#[doc = "Field `L1_POS` writer - Latch1 positive edge"]
pub type L1PosW<'a, REG> = crate::BitWriter<'a, REG, L1Pos>;
impl<'a, REG> L1PosW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Continuous Latch active"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(L1Pos::Value1)
    }
    #[doc = "Single event (only first event active)"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(L1Pos::Value2)
    }
}
#[doc = "Latch1 negative edge\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum L1Neg {
    #[doc = "0: Continuous Latch active"]
    Value1 = 0,
    #[doc = "1: Single event (only first event active)"]
    Value2 = 1,
}
impl From<L1Neg> for bool {
    #[inline(always)]
    fn from(variant: L1Neg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `L1_NEG` reader - Latch1 negative edge"]
pub type L1NegR = crate::BitReader<L1Neg>;
impl L1NegR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> L1Neg {
        match self.bits {
            false => L1Neg::Value1,
            true => L1Neg::Value2,
        }
    }
    #[doc = "Continuous Latch active"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == L1Neg::Value1
    }
    #[doc = "Single event (only first event active)"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == L1Neg::Value2
    }
}
#[doc = "Field `L1_NEG` writer - Latch1 negative edge"]
pub type L1NegW<'a, REG> = crate::BitWriter<'a, REG, L1Neg>;
impl<'a, REG> L1NegW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Continuous Latch active"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(L1Neg::Value1)
    }
    #[doc = "Single event (only first event active)"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(L1Neg::Value2)
    }
}
impl R {
    #[doc = "Bit 0 - Latch1 positive edge"]
    #[inline(always)]
    pub fn l1_pos(&self) -> L1PosR {
        L1PosR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Latch1 negative edge"]
    #[inline(always)]
    pub fn l1_neg(&self) -> L1NegR {
        L1NegR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Latch1 positive edge"]
    #[inline(always)]
    #[must_use]
    pub fn l1_pos(&mut self) -> L1PosW<DcLatch1ContSpec> {
        L1PosW::new(self, 0)
    }
    #[doc = "Bit 1 - Latch1 negative edge"]
    #[inline(always)]
    #[must_use]
    pub fn l1_neg(&mut self) -> L1NegW<DcLatch1ContSpec> {
        L1NegW::new(self, 1)
    }
}
#[doc = "Latch1 Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dc_latch1_cont::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dc_latch1_cont::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DcLatch1ContSpec;
impl crate::RegisterSpec for DcLatch1ContSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`dc_latch1_cont::R`](R) reader structure"]
impl crate::Readable for DcLatch1ContSpec {}
#[doc = "`write(|w| ..)` method takes [`dc_latch1_cont::W`](W) writer structure"]
impl crate::Writable for DcLatch1ContSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets DC_LATCH1_CONT to value 0"]
impl crate::Resettable for DcLatch1ContSpec {
    const RESET_VALUE: u8 = 0;
}
