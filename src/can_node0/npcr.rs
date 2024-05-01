#[doc = "Register `NPCR` reader"]
pub type R = crate::R<NPCR_SPEC>;
#[doc = "Register `NPCR` writer"]
pub type W = crate::W<NPCR_SPEC>;
#[doc = "Field `RXSEL` reader - Receive Select"]
pub type RXSEL_R = crate::FieldReader;
#[doc = "Field `RXSEL` writer - Receive Select"]
pub type RXSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Loop-Back Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LBM_A {
    #[doc = "0: Loop-Back Mode is disabled."]
    VALUE1 = 0,
    #[doc = "1: Loop-Back Mode is enabled. This node is connected to an internal (virtual) loop-back CAN bus. All CAN nodes which are in Loop-Back Mode are connected to this virtual CAN bus so that they can communicate with each other internally. The external transmit line is forced recessive in Loop-Back Mode."]
    VALUE2 = 1,
}
impl From<LBM_A> for bool {
    #[inline(always)]
    fn from(variant: LBM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LBM` reader - Loop-Back Mode"]
pub type LBM_R = crate::BitReader<LBM_A>;
impl LBM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LBM_A {
        match self.bits {
            false => LBM_A::VALUE1,
            true => LBM_A::VALUE2,
        }
    }
    #[doc = "Loop-Back Mode is disabled."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == LBM_A::VALUE1
    }
    #[doc = "Loop-Back Mode is enabled. This node is connected to an internal (virtual) loop-back CAN bus. All CAN nodes which are in Loop-Back Mode are connected to this virtual CAN bus so that they can communicate with each other internally. The external transmit line is forced recessive in Loop-Back Mode."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == LBM_A::VALUE2
    }
}
#[doc = "Field `LBM` writer - Loop-Back Mode"]
pub type LBM_W<'a, REG> = crate::BitWriter<'a, REG, LBM_A>;
impl<'a, REG> LBM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Loop-Back Mode is disabled."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(LBM_A::VALUE1)
    }
    #[doc = "Loop-Back Mode is enabled. This node is connected to an internal (virtual) loop-back CAN bus. All CAN nodes which are in Loop-Back Mode are connected to this virtual CAN bus so that they can communicate with each other internally. The external transmit line is forced recessive in Loop-Back Mode."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(LBM_A::VALUE2)
    }
}
impl R {
    #[doc = "Bits 0:2 - Receive Select"]
    #[inline(always)]
    pub fn rxsel(&self) -> RXSEL_R {
        RXSEL_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 8 - Loop-Back Mode"]
    #[inline(always)]
    pub fn lbm(&self) -> LBM_R {
        LBM_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Receive Select"]
    #[inline(always)]
    #[must_use]
    pub fn rxsel(&mut self) -> RXSEL_W<NPCR_SPEC> {
        RXSEL_W::new(self, 0)
    }
    #[doc = "Bit 8 - Loop-Back Mode"]
    #[inline(always)]
    #[must_use]
    pub fn lbm(&mut self) -> LBM_W<NPCR_SPEC> {
        LBM_W::new(self, 8)
    }
}
#[doc = "Node Port Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`npcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`npcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NPCR_SPEC;
impl crate::RegisterSpec for NPCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`npcr::R`](R) reader structure"]
impl crate::Readable for NPCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`npcr::W`](W) writer structure"]
impl crate::Writable for NPCR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets NPCR to value 0"]
impl crate::Resettable for NPCR_SPEC {
    const RESET_VALUE: u32 = 0;
}
