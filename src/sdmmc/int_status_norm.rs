#[doc = "Register `INT_STATUS_NORM` reader"]
pub type R = crate::R<INT_STATUS_NORM_SPEC>;
#[doc = "Register `INT_STATUS_NORM` writer"]
pub type W = crate::W<INT_STATUS_NORM_SPEC>;
#[doc = "Field `CMD_COMPLETE` reader - Command Complete"]
pub type CMD_COMPLETE_R = crate::BitReader<CMD_COMPLETE_A>;
#[doc = "Command Complete\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMD_COMPLETE_A {
    #[doc = "0: No Command Complete"]
    VALUE1 = 0,
    #[doc = "1: Command Complete"]
    VALUE2 = 1,
}
impl From<CMD_COMPLETE_A> for bool {
    #[inline(always)]
    fn from(variant: CMD_COMPLETE_A) -> Self {
        variant as u8 != 0
    }
}
impl CMD_COMPLETE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CMD_COMPLETE_A {
        match self.bits {
            false => CMD_COMPLETE_A::VALUE1,
            true => CMD_COMPLETE_A::VALUE2,
        }
    }
    #[doc = "No Command Complete"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CMD_COMPLETE_A::VALUE1
    }
    #[doc = "Command Complete"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CMD_COMPLETE_A::VALUE2
    }
}
#[doc = "Field `CMD_COMPLETE` writer - Command Complete"]
pub type CMD_COMPLETE_W<'a, REG> = crate::BitWriter<'a, REG, CMD_COMPLETE_A>;
impl<'a, REG> CMD_COMPLETE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Command Complete"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(CMD_COMPLETE_A::VALUE1)
    }
    #[doc = "Command Complete"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(CMD_COMPLETE_A::VALUE2)
    }
}
#[doc = "Field `TX_COMPLETE` reader - Transfer Complete"]
pub type TX_COMPLETE_R = crate::BitReader<TX_COMPLETE_A>;
#[doc = "Transfer Complete\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TX_COMPLETE_A {
    #[doc = "0: No Data Transfer Complete"]
    VALUE1 = 0,
    #[doc = "1: Data Transfer Complete"]
    VALUE2 = 1,
}
impl From<TX_COMPLETE_A> for bool {
    #[inline(always)]
    fn from(variant: TX_COMPLETE_A) -> Self {
        variant as u8 != 0
    }
}
impl TX_COMPLETE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TX_COMPLETE_A {
        match self.bits {
            false => TX_COMPLETE_A::VALUE1,
            true => TX_COMPLETE_A::VALUE2,
        }
    }
    #[doc = "No Data Transfer Complete"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == TX_COMPLETE_A::VALUE1
    }
    #[doc = "Data Transfer Complete"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == TX_COMPLETE_A::VALUE2
    }
}
#[doc = "Field `TX_COMPLETE` writer - Transfer Complete"]
pub type TX_COMPLETE_W<'a, REG> = crate::BitWriter<'a, REG, TX_COMPLETE_A>;
impl<'a, REG> TX_COMPLETE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Data Transfer Complete"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(TX_COMPLETE_A::VALUE1)
    }
    #[doc = "Data Transfer Complete"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(TX_COMPLETE_A::VALUE2)
    }
}
#[doc = "Field `BLOCK_GAP_EVENT` reader - Block Gap Event"]
pub type BLOCK_GAP_EVENT_R = crate::BitReader<BLOCK_GAP_EVENT_A>;
#[doc = "Block Gap Event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BLOCK_GAP_EVENT_A {
    #[doc = "0: No Block Gap Event"]
    VALUE1 = 0,
    #[doc = "1: Transaction stopped at Block Gap"]
    VALUE2 = 1,
}
impl From<BLOCK_GAP_EVENT_A> for bool {
    #[inline(always)]
    fn from(variant: BLOCK_GAP_EVENT_A) -> Self {
        variant as u8 != 0
    }
}
impl BLOCK_GAP_EVENT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BLOCK_GAP_EVENT_A {
        match self.bits {
            false => BLOCK_GAP_EVENT_A::VALUE1,
            true => BLOCK_GAP_EVENT_A::VALUE2,
        }
    }
    #[doc = "No Block Gap Event"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == BLOCK_GAP_EVENT_A::VALUE1
    }
    #[doc = "Transaction stopped at Block Gap"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == BLOCK_GAP_EVENT_A::VALUE2
    }
}
#[doc = "Field `BLOCK_GAP_EVENT` writer - Block Gap Event"]
pub type BLOCK_GAP_EVENT_W<'a, REG> = crate::BitWriter<'a, REG, BLOCK_GAP_EVENT_A>;
impl<'a, REG> BLOCK_GAP_EVENT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Block Gap Event"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(BLOCK_GAP_EVENT_A::VALUE1)
    }
    #[doc = "Transaction stopped at Block Gap"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(BLOCK_GAP_EVENT_A::VALUE2)
    }
}
#[doc = "Field `BUFF_WRITE_READY` reader - Buffer Write Ready"]
pub type BUFF_WRITE_READY_R = crate::BitReader<BUFF_WRITE_READY_A>;
#[doc = "Buffer Write Ready\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BUFF_WRITE_READY_A {
    #[doc = "0: Not Ready to Write Buffer."]
    VALUE1 = 0,
    #[doc = "1: Ready to Write Buffer."]
    VALUE2 = 1,
}
impl From<BUFF_WRITE_READY_A> for bool {
    #[inline(always)]
    fn from(variant: BUFF_WRITE_READY_A) -> Self {
        variant as u8 != 0
    }
}
impl BUFF_WRITE_READY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BUFF_WRITE_READY_A {
        match self.bits {
            false => BUFF_WRITE_READY_A::VALUE1,
            true => BUFF_WRITE_READY_A::VALUE2,
        }
    }
    #[doc = "Not Ready to Write Buffer."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == BUFF_WRITE_READY_A::VALUE1
    }
    #[doc = "Ready to Write Buffer."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == BUFF_WRITE_READY_A::VALUE2
    }
}
#[doc = "Field `BUFF_WRITE_READY` writer - Buffer Write Ready"]
pub type BUFF_WRITE_READY_W<'a, REG> = crate::BitWriter<'a, REG, BUFF_WRITE_READY_A>;
impl<'a, REG> BUFF_WRITE_READY_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Not Ready to Write Buffer."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(BUFF_WRITE_READY_A::VALUE1)
    }
    #[doc = "Ready to Write Buffer."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(BUFF_WRITE_READY_A::VALUE2)
    }
}
#[doc = "Field `BUFF_READ_READY` reader - Buffer Read Ready"]
pub type BUFF_READ_READY_R = crate::BitReader<BUFF_READ_READY_A>;
#[doc = "Buffer Read Ready\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BUFF_READ_READY_A {
    #[doc = "0: Not Ready to read Buffer."]
    VALUE1 = 0,
    #[doc = "1: Ready to read Buffer."]
    VALUE2 = 1,
}
impl From<BUFF_READ_READY_A> for bool {
    #[inline(always)]
    fn from(variant: BUFF_READ_READY_A) -> Self {
        variant as u8 != 0
    }
}
impl BUFF_READ_READY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BUFF_READ_READY_A {
        match self.bits {
            false => BUFF_READ_READY_A::VALUE1,
            true => BUFF_READ_READY_A::VALUE2,
        }
    }
    #[doc = "Not Ready to read Buffer."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == BUFF_READ_READY_A::VALUE1
    }
    #[doc = "Ready to read Buffer."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == BUFF_READ_READY_A::VALUE2
    }
}
#[doc = "Field `BUFF_READ_READY` writer - Buffer Read Ready"]
pub type BUFF_READ_READY_W<'a, REG> = crate::BitWriter<'a, REG, BUFF_READ_READY_A>;
impl<'a, REG> BUFF_READ_READY_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Not Ready to read Buffer."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(BUFF_READ_READY_A::VALUE1)
    }
    #[doc = "Ready to read Buffer."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(BUFF_READ_READY_A::VALUE2)
    }
}
#[doc = "Field `CARD_INS` reader - Card Insertion"]
pub type CARD_INS_R = crate::BitReader<CARD_INS_A>;
#[doc = "Card Insertion\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CARD_INS_A {
    #[doc = "0: Card State Stable or Debouncing"]
    VALUE1 = 0,
    #[doc = "1: Card Inserted"]
    VALUE2 = 1,
}
impl From<CARD_INS_A> for bool {
    #[inline(always)]
    fn from(variant: CARD_INS_A) -> Self {
        variant as u8 != 0
    }
}
impl CARD_INS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CARD_INS_A {
        match self.bits {
            false => CARD_INS_A::VALUE1,
            true => CARD_INS_A::VALUE2,
        }
    }
    #[doc = "Card State Stable or Debouncing"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CARD_INS_A::VALUE1
    }
    #[doc = "Card Inserted"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CARD_INS_A::VALUE2
    }
}
#[doc = "Field `CARD_INS` writer - Card Insertion"]
pub type CARD_INS_W<'a, REG> = crate::BitWriter<'a, REG, CARD_INS_A>;
impl<'a, REG> CARD_INS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Card State Stable or Debouncing"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(CARD_INS_A::VALUE1)
    }
    #[doc = "Card Inserted"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(CARD_INS_A::VALUE2)
    }
}
#[doc = "Field `CARD_REMOVAL` reader - Card Removal"]
pub type CARD_REMOVAL_R = crate::BitReader<CARD_REMOVAL_A>;
#[doc = "Card Removal\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CARD_REMOVAL_A {
    #[doc = "0: Card State Stable or Debouncing"]
    VALUE1 = 0,
    #[doc = "1: Card Removed"]
    VALUE2 = 1,
}
impl From<CARD_REMOVAL_A> for bool {
    #[inline(always)]
    fn from(variant: CARD_REMOVAL_A) -> Self {
        variant as u8 != 0
    }
}
impl CARD_REMOVAL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CARD_REMOVAL_A {
        match self.bits {
            false => CARD_REMOVAL_A::VALUE1,
            true => CARD_REMOVAL_A::VALUE2,
        }
    }
    #[doc = "Card State Stable or Debouncing"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CARD_REMOVAL_A::VALUE1
    }
    #[doc = "Card Removed"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CARD_REMOVAL_A::VALUE2
    }
}
#[doc = "Field `CARD_REMOVAL` writer - Card Removal"]
pub type CARD_REMOVAL_W<'a, REG> = crate::BitWriter<'a, REG, CARD_REMOVAL_A>;
impl<'a, REG> CARD_REMOVAL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Card State Stable or Debouncing"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(CARD_REMOVAL_A::VALUE1)
    }
    #[doc = "Card Removed"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(CARD_REMOVAL_A::VALUE2)
    }
}
#[doc = "Field `CARD_INT` reader - Card Interrupt"]
pub type CARD_INT_R = crate::BitReader<CARD_INT_A>;
#[doc = "Card Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CARD_INT_A {
    #[doc = "0: No Card Interrupt"]
    VALUE1 = 0,
    #[doc = "1: Generate Card Interrupt"]
    VALUE2 = 1,
}
impl From<CARD_INT_A> for bool {
    #[inline(always)]
    fn from(variant: CARD_INT_A) -> Self {
        variant as u8 != 0
    }
}
impl CARD_INT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CARD_INT_A {
        match self.bits {
            false => CARD_INT_A::VALUE1,
            true => CARD_INT_A::VALUE2,
        }
    }
    #[doc = "No Card Interrupt"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CARD_INT_A::VALUE1
    }
    #[doc = "Generate Card Interrupt"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CARD_INT_A::VALUE2
    }
}
#[doc = "Field `ERR_INT` reader - Error Interrupt"]
pub type ERR_INT_R = crate::BitReader<ERR_INT_A>;
#[doc = "Error Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ERR_INT_A {
    #[doc = "0: No Error."]
    VALUE1 = 0,
    #[doc = "1: Error."]
    VALUE2 = 1,
}
impl From<ERR_INT_A> for bool {
    #[inline(always)]
    fn from(variant: ERR_INT_A) -> Self {
        variant as u8 != 0
    }
}
impl ERR_INT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ERR_INT_A {
        match self.bits {
            false => ERR_INT_A::VALUE1,
            true => ERR_INT_A::VALUE2,
        }
    }
    #[doc = "No Error."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == ERR_INT_A::VALUE1
    }
    #[doc = "Error."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == ERR_INT_A::VALUE2
    }
}
impl R {
    #[doc = "Bit 0 - Command Complete"]
    #[inline(always)]
    pub fn cmd_complete(&self) -> CMD_COMPLETE_R {
        CMD_COMPLETE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transfer Complete"]
    #[inline(always)]
    pub fn tx_complete(&self) -> TX_COMPLETE_R {
        TX_COMPLETE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Block Gap Event"]
    #[inline(always)]
    pub fn block_gap_event(&self) -> BLOCK_GAP_EVENT_R {
        BLOCK_GAP_EVENT_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - Buffer Write Ready"]
    #[inline(always)]
    pub fn buff_write_ready(&self) -> BUFF_WRITE_READY_R {
        BUFF_WRITE_READY_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Buffer Read Ready"]
    #[inline(always)]
    pub fn buff_read_ready(&self) -> BUFF_READ_READY_R {
        BUFF_READ_READY_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Card Insertion"]
    #[inline(always)]
    pub fn card_ins(&self) -> CARD_INS_R {
        CARD_INS_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Card Removal"]
    #[inline(always)]
    pub fn card_removal(&self) -> CARD_REMOVAL_R {
        CARD_REMOVAL_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Card Interrupt"]
    #[inline(always)]
    pub fn card_int(&self) -> CARD_INT_R {
        CARD_INT_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 15 - Error Interrupt"]
    #[inline(always)]
    pub fn err_int(&self) -> ERR_INT_R {
        ERR_INT_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Command Complete"]
    #[inline(always)]
    #[must_use]
    pub fn cmd_complete(&mut self) -> CMD_COMPLETE_W<INT_STATUS_NORM_SPEC> {
        CMD_COMPLETE_W::new(self, 0)
    }
    #[doc = "Bit 1 - Transfer Complete"]
    #[inline(always)]
    #[must_use]
    pub fn tx_complete(&mut self) -> TX_COMPLETE_W<INT_STATUS_NORM_SPEC> {
        TX_COMPLETE_W::new(self, 1)
    }
    #[doc = "Bit 2 - Block Gap Event"]
    #[inline(always)]
    #[must_use]
    pub fn block_gap_event(&mut self) -> BLOCK_GAP_EVENT_W<INT_STATUS_NORM_SPEC> {
        BLOCK_GAP_EVENT_W::new(self, 2)
    }
    #[doc = "Bit 4 - Buffer Write Ready"]
    #[inline(always)]
    #[must_use]
    pub fn buff_write_ready(&mut self) -> BUFF_WRITE_READY_W<INT_STATUS_NORM_SPEC> {
        BUFF_WRITE_READY_W::new(self, 4)
    }
    #[doc = "Bit 5 - Buffer Read Ready"]
    #[inline(always)]
    #[must_use]
    pub fn buff_read_ready(&mut self) -> BUFF_READ_READY_W<INT_STATUS_NORM_SPEC> {
        BUFF_READ_READY_W::new(self, 5)
    }
    #[doc = "Bit 6 - Card Insertion"]
    #[inline(always)]
    #[must_use]
    pub fn card_ins(&mut self) -> CARD_INS_W<INT_STATUS_NORM_SPEC> {
        CARD_INS_W::new(self, 6)
    }
    #[doc = "Bit 7 - Card Removal"]
    #[inline(always)]
    #[must_use]
    pub fn card_removal(&mut self) -> CARD_REMOVAL_W<INT_STATUS_NORM_SPEC> {
        CARD_REMOVAL_W::new(self, 7)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Normal Interrupt Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_status_norm::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_status_norm::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_STATUS_NORM_SPEC;
impl crate::RegisterSpec for INT_STATUS_NORM_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`int_status_norm::R`](R) reader structure"]
impl crate::Readable for INT_STATUS_NORM_SPEC {}
#[doc = "`write(|w| ..)` method takes [`int_status_norm::W`](W) writer structure"]
impl crate::Writable for INT_STATUS_NORM_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets INT_STATUS_NORM to value 0"]
impl crate::Resettable for INT_STATUS_NORM_SPEC {
    const RESET_VALUE: u16 = 0;
}
