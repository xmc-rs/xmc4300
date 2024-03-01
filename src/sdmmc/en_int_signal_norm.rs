#[doc = "Register `EN_INT_SIGNAL_NORM` reader"]
pub type R = crate::R<EnIntSignalNormSpec>;
#[doc = "Register `EN_INT_SIGNAL_NORM` writer"]
pub type W = crate::W<EnIntSignalNormSpec>;
#[doc = "Command Complete Signal Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CmdCompleteEn {
    #[doc = "0: Masked"]
    Value1 = 0,
    #[doc = "1: Enabled"]
    Value2 = 1,
}
impl From<CmdCompleteEn> for bool {
    #[inline(always)]
    fn from(variant: CmdCompleteEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMD_COMPLETE_EN` reader - Command Complete Signal Enable"]
pub type CmdCompleteEnR = crate::BitReader<CmdCompleteEn>;
impl CmdCompleteEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CmdCompleteEn {
        match self.bits {
            false => CmdCompleteEn::Value1,
            true => CmdCompleteEn::Value2,
        }
    }
    #[doc = "Masked"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CmdCompleteEn::Value1
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CmdCompleteEn::Value2
    }
}
#[doc = "Field `CMD_COMPLETE_EN` writer - Command Complete Signal Enable"]
pub type CmdCompleteEnW<'a, REG> = crate::BitWriter<'a, REG, CmdCompleteEn>;
impl<'a, REG> CmdCompleteEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Masked"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(CmdCompleteEn::Value1)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(CmdCompleteEn::Value2)
    }
}
#[doc = "Transfer Complete Signal Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TxCompleteEn {
    #[doc = "0: Masked"]
    Value1 = 0,
    #[doc = "1: Enabled"]
    Value2 = 1,
}
impl From<TxCompleteEn> for bool {
    #[inline(always)]
    fn from(variant: TxCompleteEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TX_COMPLETE_EN` reader - Transfer Complete Signal Enable"]
pub type TxCompleteEnR = crate::BitReader<TxCompleteEn>;
impl TxCompleteEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TxCompleteEn {
        match self.bits {
            false => TxCompleteEn::Value1,
            true => TxCompleteEn::Value2,
        }
    }
    #[doc = "Masked"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == TxCompleteEn::Value1
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == TxCompleteEn::Value2
    }
}
#[doc = "Field `TX_COMPLETE_EN` writer - Transfer Complete Signal Enable"]
pub type TxCompleteEnW<'a, REG> = crate::BitWriter<'a, REG, TxCompleteEn>;
impl<'a, REG> TxCompleteEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Masked"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(TxCompleteEn::Value1)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(TxCompleteEn::Value2)
    }
}
#[doc = "Block Gap Event Signal Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BlockGapEventEn {
    #[doc = "0: Masked"]
    Value1 = 0,
    #[doc = "1: Enabled"]
    Value2 = 1,
}
impl From<BlockGapEventEn> for bool {
    #[inline(always)]
    fn from(variant: BlockGapEventEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BLOCK_GAP_EVENT_EN` reader - Block Gap Event Signal Enable"]
pub type BlockGapEventEnR = crate::BitReader<BlockGapEventEn>;
impl BlockGapEventEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BlockGapEventEn {
        match self.bits {
            false => BlockGapEventEn::Value1,
            true => BlockGapEventEn::Value2,
        }
    }
    #[doc = "Masked"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == BlockGapEventEn::Value1
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == BlockGapEventEn::Value2
    }
}
#[doc = "Field `BLOCK_GAP_EVENT_EN` writer - Block Gap Event Signal Enable"]
pub type BlockGapEventEnW<'a, REG> = crate::BitWriter<'a, REG, BlockGapEventEn>;
impl<'a, REG> BlockGapEventEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Masked"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(BlockGapEventEn::Value1)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(BlockGapEventEn::Value2)
    }
}
#[doc = "Buffer Write Ready Signal Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BuffWriteReadyEn {
    #[doc = "0: Masked"]
    Value1 = 0,
    #[doc = "1: Enabled"]
    Value2 = 1,
}
impl From<BuffWriteReadyEn> for bool {
    #[inline(always)]
    fn from(variant: BuffWriteReadyEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BUFF_WRITE_READY_EN` reader - Buffer Write Ready Signal Enable"]
pub type BuffWriteReadyEnR = crate::BitReader<BuffWriteReadyEn>;
impl BuffWriteReadyEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BuffWriteReadyEn {
        match self.bits {
            false => BuffWriteReadyEn::Value1,
            true => BuffWriteReadyEn::Value2,
        }
    }
    #[doc = "Masked"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == BuffWriteReadyEn::Value1
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == BuffWriteReadyEn::Value2
    }
}
#[doc = "Field `BUFF_WRITE_READY_EN` writer - Buffer Write Ready Signal Enable"]
pub type BuffWriteReadyEnW<'a, REG> = crate::BitWriter<'a, REG, BuffWriteReadyEn>;
impl<'a, REG> BuffWriteReadyEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Masked"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(BuffWriteReadyEn::Value1)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(BuffWriteReadyEn::Value2)
    }
}
#[doc = "Buffer Read Ready Signal Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BuffReadReadyEn {
    #[doc = "0: Masked"]
    Value1 = 0,
    #[doc = "1: Enabled"]
    Value2 = 1,
}
impl From<BuffReadReadyEn> for bool {
    #[inline(always)]
    fn from(variant: BuffReadReadyEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BUFF_READ_READY_EN` reader - Buffer Read Ready Signal Enable"]
pub type BuffReadReadyEnR = crate::BitReader<BuffReadReadyEn>;
impl BuffReadReadyEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BuffReadReadyEn {
        match self.bits {
            false => BuffReadReadyEn::Value1,
            true => BuffReadReadyEn::Value2,
        }
    }
    #[doc = "Masked"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == BuffReadReadyEn::Value1
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == BuffReadReadyEn::Value2
    }
}
#[doc = "Field `BUFF_READ_READY_EN` writer - Buffer Read Ready Signal Enable"]
pub type BuffReadReadyEnW<'a, REG> = crate::BitWriter<'a, REG, BuffReadReadyEn>;
impl<'a, REG> BuffReadReadyEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Masked"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(BuffReadReadyEn::Value1)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(BuffReadReadyEn::Value2)
    }
}
#[doc = "Card Insertion Signal Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CardInsEn {
    #[doc = "0: Masked"]
    Value1 = 0,
    #[doc = "1: Enabled"]
    Value2 = 1,
}
impl From<CardInsEn> for bool {
    #[inline(always)]
    fn from(variant: CardInsEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CARD_INS_EN` reader - Card Insertion Signal Enable"]
pub type CardInsEnR = crate::BitReader<CardInsEn>;
impl CardInsEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CardInsEn {
        match self.bits {
            false => CardInsEn::Value1,
            true => CardInsEn::Value2,
        }
    }
    #[doc = "Masked"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CardInsEn::Value1
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CardInsEn::Value2
    }
}
#[doc = "Field `CARD_INS_EN` writer - Card Insertion Signal Enable"]
pub type CardInsEnW<'a, REG> = crate::BitWriter<'a, REG, CardInsEn>;
impl<'a, REG> CardInsEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Masked"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(CardInsEn::Value1)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(CardInsEn::Value2)
    }
}
#[doc = "Card Removal Signal Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CardRemovalEn {
    #[doc = "0: Masked"]
    Value1 = 0,
    #[doc = "1: Enabled"]
    Value2 = 1,
}
impl From<CardRemovalEn> for bool {
    #[inline(always)]
    fn from(variant: CardRemovalEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CARD_REMOVAL_EN` reader - Card Removal Signal Enable"]
pub type CardRemovalEnR = crate::BitReader<CardRemovalEn>;
impl CardRemovalEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CardRemovalEn {
        match self.bits {
            false => CardRemovalEn::Value1,
            true => CardRemovalEn::Value2,
        }
    }
    #[doc = "Masked"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CardRemovalEn::Value1
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CardRemovalEn::Value2
    }
}
#[doc = "Field `CARD_REMOVAL_EN` writer - Card Removal Signal Enable"]
pub type CardRemovalEnW<'a, REG> = crate::BitWriter<'a, REG, CardRemovalEn>;
impl<'a, REG> CardRemovalEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Masked"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(CardRemovalEn::Value1)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(CardRemovalEn::Value2)
    }
}
#[doc = "Card Interrupt Signal Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CardIntEn {
    #[doc = "0: Masked"]
    Value1 = 0,
    #[doc = "1: Enabled"]
    Value2 = 1,
}
impl From<CardIntEn> for bool {
    #[inline(always)]
    fn from(variant: CardIntEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CARD_INT_EN` reader - Card Interrupt Signal Enable"]
pub type CardIntEnR = crate::BitReader<CardIntEn>;
impl CardIntEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CardIntEn {
        match self.bits {
            false => CardIntEn::Value1,
            true => CardIntEn::Value2,
        }
    }
    #[doc = "Masked"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CardIntEn::Value1
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CardIntEn::Value2
    }
}
#[doc = "Field `CARD_INT_EN` writer - Card Interrupt Signal Enable"]
pub type CardIntEnW<'a, REG> = crate::BitWriter<'a, REG, CardIntEn>;
impl<'a, REG> CardIntEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Masked"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(CardIntEn::Value1)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(CardIntEn::Value2)
    }
}
#[doc = "Field `FIXED_TO_0` reader - Fixed to 0"]
pub type FixedTo0R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Command Complete Signal Enable"]
    #[inline(always)]
    pub fn cmd_complete_en(&self) -> CmdCompleteEnR {
        CmdCompleteEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transfer Complete Signal Enable"]
    #[inline(always)]
    pub fn tx_complete_en(&self) -> TxCompleteEnR {
        TxCompleteEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Block Gap Event Signal Enable"]
    #[inline(always)]
    pub fn block_gap_event_en(&self) -> BlockGapEventEnR {
        BlockGapEventEnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - Buffer Write Ready Signal Enable"]
    #[inline(always)]
    pub fn buff_write_ready_en(&self) -> BuffWriteReadyEnR {
        BuffWriteReadyEnR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Buffer Read Ready Signal Enable"]
    #[inline(always)]
    pub fn buff_read_ready_en(&self) -> BuffReadReadyEnR {
        BuffReadReadyEnR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Card Insertion Signal Enable"]
    #[inline(always)]
    pub fn card_ins_en(&self) -> CardInsEnR {
        CardInsEnR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Card Removal Signal Enable"]
    #[inline(always)]
    pub fn card_removal_en(&self) -> CardRemovalEnR {
        CardRemovalEnR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Card Interrupt Signal Enable"]
    #[inline(always)]
    pub fn card_int_en(&self) -> CardIntEnR {
        CardIntEnR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 15 - Fixed to 0"]
    #[inline(always)]
    pub fn fixed_to_0(&self) -> FixedTo0R {
        FixedTo0R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Command Complete Signal Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cmd_complete_en(&mut self) -> CmdCompleteEnW<EnIntSignalNormSpec> {
        CmdCompleteEnW::new(self, 0)
    }
    #[doc = "Bit 1 - Transfer Complete Signal Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tx_complete_en(&mut self) -> TxCompleteEnW<EnIntSignalNormSpec> {
        TxCompleteEnW::new(self, 1)
    }
    #[doc = "Bit 2 - Block Gap Event Signal Enable"]
    #[inline(always)]
    #[must_use]
    pub fn block_gap_event_en(&mut self) -> BlockGapEventEnW<EnIntSignalNormSpec> {
        BlockGapEventEnW::new(self, 2)
    }
    #[doc = "Bit 4 - Buffer Write Ready Signal Enable"]
    #[inline(always)]
    #[must_use]
    pub fn buff_write_ready_en(&mut self) -> BuffWriteReadyEnW<EnIntSignalNormSpec> {
        BuffWriteReadyEnW::new(self, 4)
    }
    #[doc = "Bit 5 - Buffer Read Ready Signal Enable"]
    #[inline(always)]
    #[must_use]
    pub fn buff_read_ready_en(&mut self) -> BuffReadReadyEnW<EnIntSignalNormSpec> {
        BuffReadReadyEnW::new(self, 5)
    }
    #[doc = "Bit 6 - Card Insertion Signal Enable"]
    #[inline(always)]
    #[must_use]
    pub fn card_ins_en(&mut self) -> CardInsEnW<EnIntSignalNormSpec> {
        CardInsEnW::new(self, 6)
    }
    #[doc = "Bit 7 - Card Removal Signal Enable"]
    #[inline(always)]
    #[must_use]
    pub fn card_removal_en(&mut self) -> CardRemovalEnW<EnIntSignalNormSpec> {
        CardRemovalEnW::new(self, 7)
    }
    #[doc = "Bit 8 - Card Interrupt Signal Enable"]
    #[inline(always)]
    #[must_use]
    pub fn card_int_en(&mut self) -> CardIntEnW<EnIntSignalNormSpec> {
        CardIntEnW::new(self, 8)
    }
}
#[doc = "Normal Interrupt Signal Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`en_int_signal_norm::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`en_int_signal_norm::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EnIntSignalNormSpec;
impl crate::RegisterSpec for EnIntSignalNormSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`en_int_signal_norm::R`](R) reader structure"]
impl crate::Readable for EnIntSignalNormSpec {}
#[doc = "`write(|w| ..)` method takes [`en_int_signal_norm::W`](W) writer structure"]
impl crate::Writable for EnIntSignalNormSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets EN_INT_SIGNAL_NORM to value 0"]
impl crate::Resettable for EnIntSignalNormSpec {
    const RESET_VALUE: u16 = 0;
}
