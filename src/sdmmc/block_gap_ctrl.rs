#[doc = "Register `BLOCK_GAP_CTRL` reader"]
pub type R = crate::R<BlockGapCtrlSpec>;
#[doc = "Register `BLOCK_GAP_CTRL` writer"]
pub type W = crate::W<BlockGapCtrlSpec>;
#[doc = "Stop At Block Gap Request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StopAtBlockGap {
    #[doc = "0: Transfer"]
    Value1 = 0,
    #[doc = "1: Stop"]
    Value2 = 1,
}
impl From<StopAtBlockGap> for bool {
    #[inline(always)]
    fn from(variant: StopAtBlockGap) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STOP_AT_BLOCK_GAP` reader - Stop At Block Gap Request"]
pub type StopAtBlockGapR = crate::BitReader<StopAtBlockGap>;
impl StopAtBlockGapR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> StopAtBlockGap {
        match self.bits {
            false => StopAtBlockGap::Value1,
            true => StopAtBlockGap::Value2,
        }
    }
    #[doc = "Transfer"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == StopAtBlockGap::Value1
    }
    #[doc = "Stop"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == StopAtBlockGap::Value2
    }
}
#[doc = "Field `STOP_AT_BLOCK_GAP` writer - Stop At Block Gap Request"]
pub type StopAtBlockGapW<'a, REG> = crate::BitWriter<'a, REG, StopAtBlockGap>;
impl<'a, REG> StopAtBlockGapW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Transfer"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(StopAtBlockGap::Value1)
    }
    #[doc = "Stop"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(StopAtBlockGap::Value2)
    }
}
#[doc = "Continue Request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ContinueReq {
    #[doc = "0: Ignored"]
    Value1 = 0,
    #[doc = "1: Restart"]
    Value2 = 1,
}
impl From<ContinueReq> for bool {
    #[inline(always)]
    fn from(variant: ContinueReq) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CONTINUE_REQ` reader - Continue Request"]
pub type ContinueReqR = crate::BitReader<ContinueReq>;
impl ContinueReqR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ContinueReq {
        match self.bits {
            false => ContinueReq::Value1,
            true => ContinueReq::Value2,
        }
    }
    #[doc = "Ignored"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == ContinueReq::Value1
    }
    #[doc = "Restart"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == ContinueReq::Value2
    }
}
#[doc = "Field `CONTINUE_REQ` writer - Continue Request"]
pub type ContinueReqW<'a, REG> = crate::BitWriter<'a, REG, ContinueReq>;
impl<'a, REG> ContinueReqW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Ignored"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(ContinueReq::Value1)
    }
    #[doc = "Restart"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(ContinueReq::Value2)
    }
}
#[doc = "Read Wait Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ReadWaitCtrl {
    #[doc = "0: Disable Read Wait Control"]
    Value1 = 0,
    #[doc = "1: Enable Read Wait Control"]
    Value2 = 1,
}
impl From<ReadWaitCtrl> for bool {
    #[inline(always)]
    fn from(variant: ReadWaitCtrl) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `READ_WAIT_CTRL` reader - Read Wait Control"]
pub type ReadWaitCtrlR = crate::BitReader<ReadWaitCtrl>;
impl ReadWaitCtrlR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ReadWaitCtrl {
        match self.bits {
            false => ReadWaitCtrl::Value1,
            true => ReadWaitCtrl::Value2,
        }
    }
    #[doc = "Disable Read Wait Control"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == ReadWaitCtrl::Value1
    }
    #[doc = "Enable Read Wait Control"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == ReadWaitCtrl::Value2
    }
}
#[doc = "Field `READ_WAIT_CTRL` writer - Read Wait Control"]
pub type ReadWaitCtrlW<'a, REG> = crate::BitWriter<'a, REG, ReadWaitCtrl>;
impl<'a, REG> ReadWaitCtrlW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable Read Wait Control"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(ReadWaitCtrl::Value1)
    }
    #[doc = "Enable Read Wait Control"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(ReadWaitCtrl::Value2)
    }
}
#[doc = "Field `INT_AT_BLOCK_GAP` reader - Interrupt At Block Gap"]
pub type IntAtBlockGapR = crate::BitReader;
#[doc = "Field `INT_AT_BLOCK_GAP` writer - Interrupt At Block Gap"]
pub type IntAtBlockGapW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Stop At Block Gap Request"]
    #[inline(always)]
    pub fn stop_at_block_gap(&self) -> StopAtBlockGapR {
        StopAtBlockGapR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Continue Request"]
    #[inline(always)]
    pub fn continue_req(&self) -> ContinueReqR {
        ContinueReqR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Read Wait Control"]
    #[inline(always)]
    pub fn read_wait_ctrl(&self) -> ReadWaitCtrlR {
        ReadWaitCtrlR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Interrupt At Block Gap"]
    #[inline(always)]
    pub fn int_at_block_gap(&self) -> IntAtBlockGapR {
        IntAtBlockGapR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Stop At Block Gap Request"]
    #[inline(always)]
    #[must_use]
    pub fn stop_at_block_gap(&mut self) -> StopAtBlockGapW<BlockGapCtrlSpec> {
        StopAtBlockGapW::new(self, 0)
    }
    #[doc = "Bit 1 - Continue Request"]
    #[inline(always)]
    #[must_use]
    pub fn continue_req(&mut self) -> ContinueReqW<BlockGapCtrlSpec> {
        ContinueReqW::new(self, 1)
    }
    #[doc = "Bit 2 - Read Wait Control"]
    #[inline(always)]
    #[must_use]
    pub fn read_wait_ctrl(&mut self) -> ReadWaitCtrlW<BlockGapCtrlSpec> {
        ReadWaitCtrlW::new(self, 2)
    }
    #[doc = "Bit 3 - Interrupt At Block Gap"]
    #[inline(always)]
    #[must_use]
    pub fn int_at_block_gap(&mut self) -> IntAtBlockGapW<BlockGapCtrlSpec> {
        IntAtBlockGapW::new(self, 3)
    }
}
#[doc = "Block Gap Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`block_gap_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`block_gap_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BlockGapCtrlSpec;
impl crate::RegisterSpec for BlockGapCtrlSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`block_gap_ctrl::R`](R) reader structure"]
impl crate::Readable for BlockGapCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`block_gap_ctrl::W`](W) writer structure"]
impl crate::Writable for BlockGapCtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets BLOCK_GAP_CTRL to value 0"]
impl crate::Resettable for BlockGapCtrlSpec {
    const RESET_VALUE: u8 = 0;
}
