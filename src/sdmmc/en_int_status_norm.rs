#[doc = "Register `EN_INT_STATUS_NORM` reader"]
pub type R = crate::R<EN_INT_STATUS_NORM_SPEC>;
#[doc = "Register `EN_INT_STATUS_NORM` writer"]
pub type W = crate::W<EN_INT_STATUS_NORM_SPEC>;
#[doc = "Command Complete Status Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMD_COMPLETE_EN_A {
    #[doc = "0: Masked"]
    VALUE1 = 0,
    #[doc = "1: Enabled"]
    VALUE2 = 1,
}
impl From<CMD_COMPLETE_EN_A> for bool {
    #[inline(always)]
    fn from(variant: CMD_COMPLETE_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMD_COMPLETE_EN` reader - Command Complete Status Enable"]
pub type CMD_COMPLETE_EN_R = crate::BitReader<CMD_COMPLETE_EN_A>;
impl CMD_COMPLETE_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CMD_COMPLETE_EN_A {
        match self.bits {
            false => CMD_COMPLETE_EN_A::VALUE1,
            true => CMD_COMPLETE_EN_A::VALUE2,
        }
    }
    #[doc = "Masked"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CMD_COMPLETE_EN_A::VALUE1
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CMD_COMPLETE_EN_A::VALUE2
    }
}
#[doc = "Field `CMD_COMPLETE_EN` writer - Command Complete Status Enable"]
pub type CMD_COMPLETE_EN_W<'a, REG> = crate::BitWriter<'a, REG, CMD_COMPLETE_EN_A>;
impl<'a, REG> CMD_COMPLETE_EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Masked"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(CMD_COMPLETE_EN_A::VALUE1)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(CMD_COMPLETE_EN_A::VALUE2)
    }
}
#[doc = "Transfer Complete Status Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TX_COMPLETE_EN_A {
    #[doc = "0: Masked"]
    VALUE1 = 0,
    #[doc = "1: Enabled"]
    VALUE2 = 1,
}
impl From<TX_COMPLETE_EN_A> for bool {
    #[inline(always)]
    fn from(variant: TX_COMPLETE_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TX_COMPLETE_EN` reader - Transfer Complete Status Enable"]
pub type TX_COMPLETE_EN_R = crate::BitReader<TX_COMPLETE_EN_A>;
impl TX_COMPLETE_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TX_COMPLETE_EN_A {
        match self.bits {
            false => TX_COMPLETE_EN_A::VALUE1,
            true => TX_COMPLETE_EN_A::VALUE2,
        }
    }
    #[doc = "Masked"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == TX_COMPLETE_EN_A::VALUE1
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == TX_COMPLETE_EN_A::VALUE2
    }
}
#[doc = "Field `TX_COMPLETE_EN` writer - Transfer Complete Status Enable"]
pub type TX_COMPLETE_EN_W<'a, REG> = crate::BitWriter<'a, REG, TX_COMPLETE_EN_A>;
impl<'a, REG> TX_COMPLETE_EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Masked"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(TX_COMPLETE_EN_A::VALUE1)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(TX_COMPLETE_EN_A::VALUE2)
    }
}
#[doc = "Block Gap Event Status Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BLOCK_GAP_EVENT_EN_A {
    #[doc = "0: Masked"]
    VALUE1 = 0,
    #[doc = "1: Enabled"]
    VALUE2 = 1,
}
impl From<BLOCK_GAP_EVENT_EN_A> for bool {
    #[inline(always)]
    fn from(variant: BLOCK_GAP_EVENT_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BLOCK_GAP_EVENT_EN` reader - Block Gap Event Status Enable"]
pub type BLOCK_GAP_EVENT_EN_R = crate::BitReader<BLOCK_GAP_EVENT_EN_A>;
impl BLOCK_GAP_EVENT_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BLOCK_GAP_EVENT_EN_A {
        match self.bits {
            false => BLOCK_GAP_EVENT_EN_A::VALUE1,
            true => BLOCK_GAP_EVENT_EN_A::VALUE2,
        }
    }
    #[doc = "Masked"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == BLOCK_GAP_EVENT_EN_A::VALUE1
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == BLOCK_GAP_EVENT_EN_A::VALUE2
    }
}
#[doc = "Field `BLOCK_GAP_EVENT_EN` writer - Block Gap Event Status Enable"]
pub type BLOCK_GAP_EVENT_EN_W<'a, REG> = crate::BitWriter<'a, REG, BLOCK_GAP_EVENT_EN_A>;
impl<'a, REG> BLOCK_GAP_EVENT_EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Masked"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(BLOCK_GAP_EVENT_EN_A::VALUE1)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(BLOCK_GAP_EVENT_EN_A::VALUE2)
    }
}
#[doc = "Buffer Write Ready Status Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BUFF_WRITE_READY_EN_A {
    #[doc = "0: Masked"]
    VALUE1 = 0,
    #[doc = "1: Enabled"]
    VALUE2 = 1,
}
impl From<BUFF_WRITE_READY_EN_A> for bool {
    #[inline(always)]
    fn from(variant: BUFF_WRITE_READY_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BUFF_WRITE_READY_EN` reader - Buffer Write Ready Status Enable"]
pub type BUFF_WRITE_READY_EN_R = crate::BitReader<BUFF_WRITE_READY_EN_A>;
impl BUFF_WRITE_READY_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BUFF_WRITE_READY_EN_A {
        match self.bits {
            false => BUFF_WRITE_READY_EN_A::VALUE1,
            true => BUFF_WRITE_READY_EN_A::VALUE2,
        }
    }
    #[doc = "Masked"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == BUFF_WRITE_READY_EN_A::VALUE1
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == BUFF_WRITE_READY_EN_A::VALUE2
    }
}
#[doc = "Field `BUFF_WRITE_READY_EN` writer - Buffer Write Ready Status Enable"]
pub type BUFF_WRITE_READY_EN_W<'a, REG> = crate::BitWriter<'a, REG, BUFF_WRITE_READY_EN_A>;
impl<'a, REG> BUFF_WRITE_READY_EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Masked"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(BUFF_WRITE_READY_EN_A::VALUE1)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(BUFF_WRITE_READY_EN_A::VALUE2)
    }
}
#[doc = "Buffer Read Ready Status Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BUFF_READ_READY_EN_A {
    #[doc = "0: Masked"]
    VALUE1 = 0,
    #[doc = "1: Enabled"]
    VALUE2 = 1,
}
impl From<BUFF_READ_READY_EN_A> for bool {
    #[inline(always)]
    fn from(variant: BUFF_READ_READY_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BUFF_READ_READY_EN` reader - Buffer Read Ready Status Enable"]
pub type BUFF_READ_READY_EN_R = crate::BitReader<BUFF_READ_READY_EN_A>;
impl BUFF_READ_READY_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BUFF_READ_READY_EN_A {
        match self.bits {
            false => BUFF_READ_READY_EN_A::VALUE1,
            true => BUFF_READ_READY_EN_A::VALUE2,
        }
    }
    #[doc = "Masked"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == BUFF_READ_READY_EN_A::VALUE1
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == BUFF_READ_READY_EN_A::VALUE2
    }
}
#[doc = "Field `BUFF_READ_READY_EN` writer - Buffer Read Ready Status Enable"]
pub type BUFF_READ_READY_EN_W<'a, REG> = crate::BitWriter<'a, REG, BUFF_READ_READY_EN_A>;
impl<'a, REG> BUFF_READ_READY_EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Masked"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(BUFF_READ_READY_EN_A::VALUE1)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(BUFF_READ_READY_EN_A::VALUE2)
    }
}
#[doc = "Card Insertion Status Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CARD_INS_EN_A {
    #[doc = "0: Masked"]
    VALUE1 = 0,
    #[doc = "1: Enabled"]
    VALUE2 = 1,
}
impl From<CARD_INS_EN_A> for bool {
    #[inline(always)]
    fn from(variant: CARD_INS_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CARD_INS_EN` reader - Card Insertion Status Enable"]
pub type CARD_INS_EN_R = crate::BitReader<CARD_INS_EN_A>;
impl CARD_INS_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CARD_INS_EN_A {
        match self.bits {
            false => CARD_INS_EN_A::VALUE1,
            true => CARD_INS_EN_A::VALUE2,
        }
    }
    #[doc = "Masked"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CARD_INS_EN_A::VALUE1
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CARD_INS_EN_A::VALUE2
    }
}
#[doc = "Field `CARD_INS_EN` writer - Card Insertion Status Enable"]
pub type CARD_INS_EN_W<'a, REG> = crate::BitWriter<'a, REG, CARD_INS_EN_A>;
impl<'a, REG> CARD_INS_EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Masked"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(CARD_INS_EN_A::VALUE1)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(CARD_INS_EN_A::VALUE2)
    }
}
#[doc = "Card Removal Status Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CARD_REMOVAL_EN_A {
    #[doc = "0: Masked"]
    VALUE1 = 0,
    #[doc = "1: Enabled"]
    VALUE2 = 1,
}
impl From<CARD_REMOVAL_EN_A> for bool {
    #[inline(always)]
    fn from(variant: CARD_REMOVAL_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CARD_REMOVAL_EN` reader - Card Removal Status Enable"]
pub type CARD_REMOVAL_EN_R = crate::BitReader<CARD_REMOVAL_EN_A>;
impl CARD_REMOVAL_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CARD_REMOVAL_EN_A {
        match self.bits {
            false => CARD_REMOVAL_EN_A::VALUE1,
            true => CARD_REMOVAL_EN_A::VALUE2,
        }
    }
    #[doc = "Masked"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CARD_REMOVAL_EN_A::VALUE1
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CARD_REMOVAL_EN_A::VALUE2
    }
}
#[doc = "Field `CARD_REMOVAL_EN` writer - Card Removal Status Enable"]
pub type CARD_REMOVAL_EN_W<'a, REG> = crate::BitWriter<'a, REG, CARD_REMOVAL_EN_A>;
impl<'a, REG> CARD_REMOVAL_EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Masked"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(CARD_REMOVAL_EN_A::VALUE1)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(CARD_REMOVAL_EN_A::VALUE2)
    }
}
#[doc = "Card Interrupt Status Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CARD_INT_EN_A {
    #[doc = "0: Masked"]
    VALUE1 = 0,
    #[doc = "1: Enabled"]
    VALUE2 = 1,
}
impl From<CARD_INT_EN_A> for bool {
    #[inline(always)]
    fn from(variant: CARD_INT_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CARD_INT_EN` reader - Card Interrupt Status Enable"]
pub type CARD_INT_EN_R = crate::BitReader<CARD_INT_EN_A>;
impl CARD_INT_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CARD_INT_EN_A {
        match self.bits {
            false => CARD_INT_EN_A::VALUE1,
            true => CARD_INT_EN_A::VALUE2,
        }
    }
    #[doc = "Masked"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CARD_INT_EN_A::VALUE1
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CARD_INT_EN_A::VALUE2
    }
}
#[doc = "Field `CARD_INT_EN` writer - Card Interrupt Status Enable"]
pub type CARD_INT_EN_W<'a, REG> = crate::BitWriter<'a, REG, CARD_INT_EN_A>;
impl<'a, REG> CARD_INT_EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Masked"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(CARD_INT_EN_A::VALUE1)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(CARD_INT_EN_A::VALUE2)
    }
}
#[doc = "Field `FIXED_TO_0` reader - Fixed to 0"]
pub type FIXED_TO_0_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Command Complete Status Enable"]
    #[inline(always)]
    pub fn cmd_complete_en(&self) -> CMD_COMPLETE_EN_R {
        CMD_COMPLETE_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transfer Complete Status Enable"]
    #[inline(always)]
    pub fn tx_complete_en(&self) -> TX_COMPLETE_EN_R {
        TX_COMPLETE_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Block Gap Event Status Enable"]
    #[inline(always)]
    pub fn block_gap_event_en(&self) -> BLOCK_GAP_EVENT_EN_R {
        BLOCK_GAP_EVENT_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - Buffer Write Ready Status Enable"]
    #[inline(always)]
    pub fn buff_write_ready_en(&self) -> BUFF_WRITE_READY_EN_R {
        BUFF_WRITE_READY_EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Buffer Read Ready Status Enable"]
    #[inline(always)]
    pub fn buff_read_ready_en(&self) -> BUFF_READ_READY_EN_R {
        BUFF_READ_READY_EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Card Insertion Status Enable"]
    #[inline(always)]
    pub fn card_ins_en(&self) -> CARD_INS_EN_R {
        CARD_INS_EN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Card Removal Status Enable"]
    #[inline(always)]
    pub fn card_removal_en(&self) -> CARD_REMOVAL_EN_R {
        CARD_REMOVAL_EN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Card Interrupt Status Enable"]
    #[inline(always)]
    pub fn card_int_en(&self) -> CARD_INT_EN_R {
        CARD_INT_EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 15 - Fixed to 0"]
    #[inline(always)]
    pub fn fixed_to_0(&self) -> FIXED_TO_0_R {
        FIXED_TO_0_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Command Complete Status Enable"]
    #[inline(always)]
    pub fn cmd_complete_en(&mut self) -> CMD_COMPLETE_EN_W<EN_INT_STATUS_NORM_SPEC> {
        CMD_COMPLETE_EN_W::new(self, 0)
    }
    #[doc = "Bit 1 - Transfer Complete Status Enable"]
    #[inline(always)]
    pub fn tx_complete_en(&mut self) -> TX_COMPLETE_EN_W<EN_INT_STATUS_NORM_SPEC> {
        TX_COMPLETE_EN_W::new(self, 1)
    }
    #[doc = "Bit 2 - Block Gap Event Status Enable"]
    #[inline(always)]
    pub fn block_gap_event_en(&mut self) -> BLOCK_GAP_EVENT_EN_W<EN_INT_STATUS_NORM_SPEC> {
        BLOCK_GAP_EVENT_EN_W::new(self, 2)
    }
    #[doc = "Bit 4 - Buffer Write Ready Status Enable"]
    #[inline(always)]
    pub fn buff_write_ready_en(&mut self) -> BUFF_WRITE_READY_EN_W<EN_INT_STATUS_NORM_SPEC> {
        BUFF_WRITE_READY_EN_W::new(self, 4)
    }
    #[doc = "Bit 5 - Buffer Read Ready Status Enable"]
    #[inline(always)]
    pub fn buff_read_ready_en(&mut self) -> BUFF_READ_READY_EN_W<EN_INT_STATUS_NORM_SPEC> {
        BUFF_READ_READY_EN_W::new(self, 5)
    }
    #[doc = "Bit 6 - Card Insertion Status Enable"]
    #[inline(always)]
    pub fn card_ins_en(&mut self) -> CARD_INS_EN_W<EN_INT_STATUS_NORM_SPEC> {
        CARD_INS_EN_W::new(self, 6)
    }
    #[doc = "Bit 7 - Card Removal Status Enable"]
    #[inline(always)]
    pub fn card_removal_en(&mut self) -> CARD_REMOVAL_EN_W<EN_INT_STATUS_NORM_SPEC> {
        CARD_REMOVAL_EN_W::new(self, 7)
    }
    #[doc = "Bit 8 - Card Interrupt Status Enable"]
    #[inline(always)]
    pub fn card_int_en(&mut self) -> CARD_INT_EN_W<EN_INT_STATUS_NORM_SPEC> {
        CARD_INT_EN_W::new(self, 8)
    }
}
#[doc = "Normal Interrupt Status Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`en_int_status_norm::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`en_int_status_norm::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EN_INT_STATUS_NORM_SPEC;
impl crate::RegisterSpec for EN_INT_STATUS_NORM_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`en_int_status_norm::R`](R) reader structure"]
impl crate::Readable for EN_INT_STATUS_NORM_SPEC {}
#[doc = "`write(|w| ..)` method takes [`en_int_status_norm::W`](W) writer structure"]
impl crate::Writable for EN_INT_STATUS_NORM_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets EN_INT_STATUS_NORM to value 0"]
impl crate::Resettable for EN_INT_STATUS_NORM_SPEC {
    const RESET_VALUE: u16 = 0;
}
