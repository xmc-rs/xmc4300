#[doc = "Register `DC_LATCH0_CONT` reader"]
pub type R = crate::R<DcLatch0ContSpec>;
#[doc = "Register `DC_LATCH0_CONT` writer"]
pub type W = crate::W<DcLatch0ContSpec>;
#[doc = "Latch0 positive edge\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum L0Pos {
    #[doc = "0: Continuous Latch active"]
    Value1 = 0,
    #[doc = "1: Single event (only first event active)"]
    Value2 = 1,
}
impl From<L0Pos> for bool {
    #[inline(always)]
    fn from(variant: L0Pos) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `L0_POS` reader - Latch0 positive edge"]
pub type L0PosR = crate::BitReader<L0Pos>;
impl L0PosR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> L0Pos {
        match self.bits {
            false => L0Pos::Value1,
            true => L0Pos::Value2,
        }
    }
    #[doc = "Continuous Latch active"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == L0Pos::Value1
    }
    #[doc = "Single event (only first event active)"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == L0Pos::Value2
    }
}
#[doc = "Field `L0_POS` writer - Latch0 positive edge"]
pub type L0PosW<'a, REG> = crate::BitWriter<'a, REG, L0Pos>;
impl<'a, REG> L0PosW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Continuous Latch active"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(L0Pos::Value1)
    }
    #[doc = "Single event (only first event active)"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(L0Pos::Value2)
    }
}
#[doc = "Latch0 negative edge\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum L0Neg {
    #[doc = "0: Continuous Latch active"]
    Value1 = 0,
    #[doc = "1: Single event (only first event active)"]
    Value2 = 1,
}
impl From<L0Neg> for bool {
    #[inline(always)]
    fn from(variant: L0Neg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `L0_NEG` reader - Latch0 negative edge"]
pub type L0NegR = crate::BitReader<L0Neg>;
impl L0NegR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> L0Neg {
        match self.bits {
            false => L0Neg::Value1,
            true => L0Neg::Value2,
        }
    }
    #[doc = "Continuous Latch active"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == L0Neg::Value1
    }
    #[doc = "Single event (only first event active)"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == L0Neg::Value2
    }
}
#[doc = "Field `L0_NEG` writer - Latch0 negative edge"]
pub type L0NegW<'a, REG> = crate::BitWriter<'a, REG, L0Neg>;
impl<'a, REG> L0NegW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Continuous Latch active"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(L0Neg::Value1)
    }
    #[doc = "Single event (only first event active)"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(L0Neg::Value2)
    }
}
impl R {
    #[doc = "Bit 0 - Latch0 positive edge"]
    #[inline(always)]
    pub fn l0_pos(&self) -> L0PosR {
        L0PosR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Latch0 negative edge"]
    #[inline(always)]
    pub fn l0_neg(&self) -> L0NegR {
        L0NegR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Latch0 positive edge"]
    #[inline(always)]
    #[must_use]
    pub fn l0_pos(&mut self) -> L0PosW<DcLatch0ContSpec> {
        L0PosW::new(self, 0)
    }
    #[doc = "Bit 1 - Latch0 negative edge"]
    #[inline(always)]
    #[must_use]
    pub fn l0_neg(&mut self) -> L0NegW<DcLatch0ContSpec> {
        L0NegW::new(self, 1)
    }
}
#[doc = "Latch0 Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dc_latch0_cont::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dc_latch0_cont::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DcLatch0ContSpec;
impl crate::RegisterSpec for DcLatch0ContSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`dc_latch0_cont::R`](R) reader structure"]
impl crate::Readable for DcLatch0ContSpec {}
#[doc = "`write(|w| ..)` method takes [`dc_latch0_cont::W`](W) writer structure"]
impl crate::Writable for DcLatch0ContSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets DC_LATCH0_CONT to value 0"]
impl crate::Resettable for DcLatch0ContSpec {
    const RESET_VALUE: u8 = 0;
}
