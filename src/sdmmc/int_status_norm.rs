#[doc = "Register `INT_STATUS_NORM` reader"]
pub type R = crate::R<IntStatusNormSpec>;
#[doc = "Register `INT_STATUS_NORM` writer"]
pub type W = crate::W<IntStatusNormSpec>;
#[doc = "Command Complete\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CmdComplete {
    #[doc = "0: No Command Complete"]
    Value1 = 0,
    #[doc = "1: Command Complete"]
    Value2 = 1,
}
impl From<CmdComplete> for bool {
    #[inline(always)]
    fn from(variant: CmdComplete) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMD_COMPLETE` reader - Command Complete"]
pub type CmdCompleteR = crate::BitReader<CmdComplete>;
impl CmdCompleteR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CmdComplete {
        match self.bits {
            false => CmdComplete::Value1,
            true => CmdComplete::Value2,
        }
    }
    #[doc = "No Command Complete"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CmdComplete::Value1
    }
    #[doc = "Command Complete"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CmdComplete::Value2
    }
}
#[doc = "Field `CMD_COMPLETE` writer - Command Complete"]
pub type CmdCompleteW<'a, REG> = crate::BitWriter<'a, REG, CmdComplete>;
impl<'a, REG> CmdCompleteW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Command Complete"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(CmdComplete::Value1)
    }
    #[doc = "Command Complete"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(CmdComplete::Value2)
    }
}
#[doc = "Transfer Complete\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TxComplete {
    #[doc = "0: No Data Transfer Complete"]
    Value1 = 0,
    #[doc = "1: Data Transfer Complete"]
    Value2 = 1,
}
impl From<TxComplete> for bool {
    #[inline(always)]
    fn from(variant: TxComplete) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TX_COMPLETE` reader - Transfer Complete"]
pub type TxCompleteR = crate::BitReader<TxComplete>;
impl TxCompleteR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TxComplete {
        match self.bits {
            false => TxComplete::Value1,
            true => TxComplete::Value2,
        }
    }
    #[doc = "No Data Transfer Complete"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == TxComplete::Value1
    }
    #[doc = "Data Transfer Complete"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == TxComplete::Value2
    }
}
#[doc = "Field `TX_COMPLETE` writer - Transfer Complete"]
pub type TxCompleteW<'a, REG> = crate::BitWriter<'a, REG, TxComplete>;
impl<'a, REG> TxCompleteW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Data Transfer Complete"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(TxComplete::Value1)
    }
    #[doc = "Data Transfer Complete"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(TxComplete::Value2)
    }
}
#[doc = "Block Gap Event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BlockGapEvent {
    #[doc = "0: No Block Gap Event"]
    Value1 = 0,
    #[doc = "1: Transaction stopped at Block Gap"]
    Value2 = 1,
}
impl From<BlockGapEvent> for bool {
    #[inline(always)]
    fn from(variant: BlockGapEvent) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BLOCK_GAP_EVENT` reader - Block Gap Event"]
pub type BlockGapEventR = crate::BitReader<BlockGapEvent>;
impl BlockGapEventR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BlockGapEvent {
        match self.bits {
            false => BlockGapEvent::Value1,
            true => BlockGapEvent::Value2,
        }
    }
    #[doc = "No Block Gap Event"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == BlockGapEvent::Value1
    }
    #[doc = "Transaction stopped at Block Gap"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == BlockGapEvent::Value2
    }
}
#[doc = "Field `BLOCK_GAP_EVENT` writer - Block Gap Event"]
pub type BlockGapEventW<'a, REG> = crate::BitWriter<'a, REG, BlockGapEvent>;
impl<'a, REG> BlockGapEventW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Block Gap Event"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(BlockGapEvent::Value1)
    }
    #[doc = "Transaction stopped at Block Gap"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(BlockGapEvent::Value2)
    }
}
#[doc = "Buffer Write Ready\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BuffWriteReady {
    #[doc = "0: Not Ready to Write Buffer."]
    Value1 = 0,
    #[doc = "1: Ready to Write Buffer."]
    Value2 = 1,
}
impl From<BuffWriteReady> for bool {
    #[inline(always)]
    fn from(variant: BuffWriteReady) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BUFF_WRITE_READY` reader - Buffer Write Ready"]
pub type BuffWriteReadyR = crate::BitReader<BuffWriteReady>;
impl BuffWriteReadyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BuffWriteReady {
        match self.bits {
            false => BuffWriteReady::Value1,
            true => BuffWriteReady::Value2,
        }
    }
    #[doc = "Not Ready to Write Buffer."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == BuffWriteReady::Value1
    }
    #[doc = "Ready to Write Buffer."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == BuffWriteReady::Value2
    }
}
#[doc = "Field `BUFF_WRITE_READY` writer - Buffer Write Ready"]
pub type BuffWriteReadyW<'a, REG> = crate::BitWriter<'a, REG, BuffWriteReady>;
impl<'a, REG> BuffWriteReadyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Not Ready to Write Buffer."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(BuffWriteReady::Value1)
    }
    #[doc = "Ready to Write Buffer."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(BuffWriteReady::Value2)
    }
}
#[doc = "Buffer Read Ready\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BuffReadReady {
    #[doc = "0: Not Ready to read Buffer."]
    Value1 = 0,
    #[doc = "1: Ready to read Buffer."]
    Value2 = 1,
}
impl From<BuffReadReady> for bool {
    #[inline(always)]
    fn from(variant: BuffReadReady) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BUFF_READ_READY` reader - Buffer Read Ready"]
pub type BuffReadReadyR = crate::BitReader<BuffReadReady>;
impl BuffReadReadyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BuffReadReady {
        match self.bits {
            false => BuffReadReady::Value1,
            true => BuffReadReady::Value2,
        }
    }
    #[doc = "Not Ready to read Buffer."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == BuffReadReady::Value1
    }
    #[doc = "Ready to read Buffer."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == BuffReadReady::Value2
    }
}
#[doc = "Field `BUFF_READ_READY` writer - Buffer Read Ready"]
pub type BuffReadReadyW<'a, REG> = crate::BitWriter<'a, REG, BuffReadReady>;
impl<'a, REG> BuffReadReadyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Not Ready to read Buffer."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(BuffReadReady::Value1)
    }
    #[doc = "Ready to read Buffer."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(BuffReadReady::Value2)
    }
}
#[doc = "Card Insertion\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CardIns {
    #[doc = "0: Card State Stable or Debouncing"]
    Value1 = 0,
    #[doc = "1: Card Inserted"]
    Value2 = 1,
}
impl From<CardIns> for bool {
    #[inline(always)]
    fn from(variant: CardIns) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CARD_INS` reader - Card Insertion"]
pub type CardInsR = crate::BitReader<CardIns>;
impl CardInsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CardIns {
        match self.bits {
            false => CardIns::Value1,
            true => CardIns::Value2,
        }
    }
    #[doc = "Card State Stable or Debouncing"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CardIns::Value1
    }
    #[doc = "Card Inserted"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CardIns::Value2
    }
}
#[doc = "Field `CARD_INS` writer - Card Insertion"]
pub type CardInsW<'a, REG> = crate::BitWriter<'a, REG, CardIns>;
impl<'a, REG> CardInsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Card State Stable or Debouncing"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(CardIns::Value1)
    }
    #[doc = "Card Inserted"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(CardIns::Value2)
    }
}
#[doc = "Card Removal\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CardRemoval {
    #[doc = "0: Card State Stable or Debouncing"]
    Value1 = 0,
    #[doc = "1: Card Removed"]
    Value2 = 1,
}
impl From<CardRemoval> for bool {
    #[inline(always)]
    fn from(variant: CardRemoval) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CARD_REMOVAL` reader - Card Removal"]
pub type CardRemovalR = crate::BitReader<CardRemoval>;
impl CardRemovalR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CardRemoval {
        match self.bits {
            false => CardRemoval::Value1,
            true => CardRemoval::Value2,
        }
    }
    #[doc = "Card State Stable or Debouncing"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CardRemoval::Value1
    }
    #[doc = "Card Removed"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CardRemoval::Value2
    }
}
#[doc = "Field `CARD_REMOVAL` writer - Card Removal"]
pub type CardRemovalW<'a, REG> = crate::BitWriter<'a, REG, CardRemoval>;
impl<'a, REG> CardRemovalW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Card State Stable or Debouncing"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(CardRemoval::Value1)
    }
    #[doc = "Card Removed"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(CardRemoval::Value2)
    }
}
#[doc = "Card Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CardInt {
    #[doc = "0: No Card Interrupt"]
    Value1 = 0,
    #[doc = "1: Generate Card Interrupt"]
    Value2 = 1,
}
impl From<CardInt> for bool {
    #[inline(always)]
    fn from(variant: CardInt) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CARD_INT` reader - Card Interrupt"]
pub type CardIntR = crate::BitReader<CardInt>;
impl CardIntR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CardInt {
        match self.bits {
            false => CardInt::Value1,
            true => CardInt::Value2,
        }
    }
    #[doc = "No Card Interrupt"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CardInt::Value1
    }
    #[doc = "Generate Card Interrupt"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CardInt::Value2
    }
}
#[doc = "Error Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ErrInt {
    #[doc = "0: No Error."]
    Value1 = 0,
    #[doc = "1: Error."]
    Value2 = 1,
}
impl From<ErrInt> for bool {
    #[inline(always)]
    fn from(variant: ErrInt) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERR_INT` reader - Error Interrupt"]
pub type ErrIntR = crate::BitReader<ErrInt>;
impl ErrIntR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ErrInt {
        match self.bits {
            false => ErrInt::Value1,
            true => ErrInt::Value2,
        }
    }
    #[doc = "No Error."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == ErrInt::Value1
    }
    #[doc = "Error."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == ErrInt::Value2
    }
}
impl R {
    #[doc = "Bit 0 - Command Complete"]
    #[inline(always)]
    pub fn cmd_complete(&self) -> CmdCompleteR {
        CmdCompleteR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transfer Complete"]
    #[inline(always)]
    pub fn tx_complete(&self) -> TxCompleteR {
        TxCompleteR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Block Gap Event"]
    #[inline(always)]
    pub fn block_gap_event(&self) -> BlockGapEventR {
        BlockGapEventR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - Buffer Write Ready"]
    #[inline(always)]
    pub fn buff_write_ready(&self) -> BuffWriteReadyR {
        BuffWriteReadyR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Buffer Read Ready"]
    #[inline(always)]
    pub fn buff_read_ready(&self) -> BuffReadReadyR {
        BuffReadReadyR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Card Insertion"]
    #[inline(always)]
    pub fn card_ins(&self) -> CardInsR {
        CardInsR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Card Removal"]
    #[inline(always)]
    pub fn card_removal(&self) -> CardRemovalR {
        CardRemovalR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Card Interrupt"]
    #[inline(always)]
    pub fn card_int(&self) -> CardIntR {
        CardIntR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 15 - Error Interrupt"]
    #[inline(always)]
    pub fn err_int(&self) -> ErrIntR {
        ErrIntR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Command Complete"]
    #[inline(always)]
    #[must_use]
    pub fn cmd_complete(&mut self) -> CmdCompleteW<IntStatusNormSpec> {
        CmdCompleteW::new(self, 0)
    }
    #[doc = "Bit 1 - Transfer Complete"]
    #[inline(always)]
    #[must_use]
    pub fn tx_complete(&mut self) -> TxCompleteW<IntStatusNormSpec> {
        TxCompleteW::new(self, 1)
    }
    #[doc = "Bit 2 - Block Gap Event"]
    #[inline(always)]
    #[must_use]
    pub fn block_gap_event(&mut self) -> BlockGapEventW<IntStatusNormSpec> {
        BlockGapEventW::new(self, 2)
    }
    #[doc = "Bit 4 - Buffer Write Ready"]
    #[inline(always)]
    #[must_use]
    pub fn buff_write_ready(&mut self) -> BuffWriteReadyW<IntStatusNormSpec> {
        BuffWriteReadyW::new(self, 4)
    }
    #[doc = "Bit 5 - Buffer Read Ready"]
    #[inline(always)]
    #[must_use]
    pub fn buff_read_ready(&mut self) -> BuffReadReadyW<IntStatusNormSpec> {
        BuffReadReadyW::new(self, 5)
    }
    #[doc = "Bit 6 - Card Insertion"]
    #[inline(always)]
    #[must_use]
    pub fn card_ins(&mut self) -> CardInsW<IntStatusNormSpec> {
        CardInsW::new(self, 6)
    }
    #[doc = "Bit 7 - Card Removal"]
    #[inline(always)]
    #[must_use]
    pub fn card_removal(&mut self) -> CardRemovalW<IntStatusNormSpec> {
        CardRemovalW::new(self, 7)
    }
}
#[doc = "Normal Interrupt Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_status_norm::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_status_norm::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntStatusNormSpec;
impl crate::RegisterSpec for IntStatusNormSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`int_status_norm::R`](R) reader structure"]
impl crate::Readable for IntStatusNormSpec {}
#[doc = "`write(|w| ..)` method takes [`int_status_norm::W`](W) writer structure"]
impl crate::Writable for IntStatusNormSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets INT_STATUS_NORM to value 0"]
impl crate::Resettable for IntStatusNormSpec {
    const RESET_VALUE: u16 = 0;
}
