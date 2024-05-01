#[doc = "Register `NPCR` reader"]
pub type R = crate::R<NpcrSpec>;
#[doc = "Register `NPCR` writer"]
pub type W = crate::W<NpcrSpec>;
#[doc = "Field `RXSEL` reader - Receive Select"]
pub type RxselR = crate::FieldReader;
#[doc = "Field `RXSEL` writer - Receive Select"]
pub type RxselW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Loop-Back Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lbm {
    #[doc = "0: Loop-Back Mode is disabled."]
    Value1 = 0,
    #[doc = "1: Loop-Back Mode is enabled. This node is connected to an internal (virtual) loop-back CAN bus. All CAN nodes which are in Loop-Back Mode are connected to this virtual CAN bus so that they can communicate with each other internally. The external transmit line is forced recessive in Loop-Back Mode."]
    Value2 = 1,
}
impl From<Lbm> for bool {
    #[inline(always)]
    fn from(variant: Lbm) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LBM` reader - Loop-Back Mode"]
pub type LbmR = crate::BitReader<Lbm>;
impl LbmR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lbm {
        match self.bits {
            false => Lbm::Value1,
            true => Lbm::Value2,
        }
    }
    #[doc = "Loop-Back Mode is disabled."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Lbm::Value1
    }
    #[doc = "Loop-Back Mode is enabled. This node is connected to an internal (virtual) loop-back CAN bus. All CAN nodes which are in Loop-Back Mode are connected to this virtual CAN bus so that they can communicate with each other internally. The external transmit line is forced recessive in Loop-Back Mode."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Lbm::Value2
    }
}
#[doc = "Field `LBM` writer - Loop-Back Mode"]
pub type LbmW<'a, REG> = crate::BitWriter<'a, REG, Lbm>;
impl<'a, REG> LbmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Loop-Back Mode is disabled."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Lbm::Value1)
    }
    #[doc = "Loop-Back Mode is enabled. This node is connected to an internal (virtual) loop-back CAN bus. All CAN nodes which are in Loop-Back Mode are connected to this virtual CAN bus so that they can communicate with each other internally. The external transmit line is forced recessive in Loop-Back Mode."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Lbm::Value2)
    }
}
impl R {
    #[doc = "Bits 0:2 - Receive Select"]
    #[inline(always)]
    pub fn rxsel(&self) -> RxselR {
        RxselR::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 8 - Loop-Back Mode"]
    #[inline(always)]
    pub fn lbm(&self) -> LbmR {
        LbmR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Receive Select"]
    #[inline(always)]
    #[must_use]
    pub fn rxsel(&mut self) -> RxselW<NpcrSpec> {
        RxselW::new(self, 0)
    }
    #[doc = "Bit 8 - Loop-Back Mode"]
    #[inline(always)]
    #[must_use]
    pub fn lbm(&mut self) -> LbmW<NpcrSpec> {
        LbmW::new(self, 8)
    }
}
#[doc = "Node Port Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`npcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`npcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NpcrSpec;
impl crate::RegisterSpec for NpcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`npcr::R`](R) reader structure"]
impl crate::Readable for NpcrSpec {}
#[doc = "`write(|w| ..)` method takes [`npcr::W`](W) writer structure"]
impl crate::Writable for NpcrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets NPCR to value 0"]
impl crate::Resettable for NpcrSpec {
    const RESET_VALUE: u32 = 0;
}
