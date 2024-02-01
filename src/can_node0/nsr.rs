#[doc = "Register `NSR` reader"]
pub type R = crate::R<NSR_SPEC>;
#[doc = "Register `NSR` writer"]
pub type W = crate::W<NSR_SPEC>;
#[doc = "Field `LEC` reader - Last Error Code"]
pub type LEC_R = crate::FieldReader;
#[doc = "Field `LEC` writer - Last Error Code"]
pub type LEC_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `TXOK` reader - Message Transmitted Successfully"]
pub type TXOK_R = crate::BitReader<TXOK_A>;
#[doc = "Message Transmitted Successfully\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXOK_A {
    #[doc = "0: No successful transmission since last (most recent) flag reset."]
    VALUE1 = 0,
    #[doc = "1: A message has been transmitted successfully (error-free and acknowledged by at least another node)."]
    VALUE2 = 1,
}
impl From<TXOK_A> for bool {
    #[inline(always)]
    fn from(variant: TXOK_A) -> Self {
        variant as u8 != 0
    }
}
impl TXOK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TXOK_A {
        match self.bits {
            false => TXOK_A::VALUE1,
            true => TXOK_A::VALUE2,
        }
    }
    #[doc = "No successful transmission since last (most recent) flag reset."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == TXOK_A::VALUE1
    }
    #[doc = "A message has been transmitted successfully (error-free and acknowledged by at least another node)."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == TXOK_A::VALUE2
    }
}
#[doc = "Field `TXOK` writer - Message Transmitted Successfully"]
pub type TXOK_W<'a, REG> = crate::BitWriter<'a, REG, TXOK_A>;
impl<'a, REG> TXOK_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No successful transmission since last (most recent) flag reset."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(TXOK_A::VALUE1)
    }
    #[doc = "A message has been transmitted successfully (error-free and acknowledged by at least another node)."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(TXOK_A::VALUE2)
    }
}
#[doc = "Field `RXOK` reader - Message Received Successfully"]
pub type RXOK_R = crate::BitReader<RXOK_A>;
#[doc = "Message Received Successfully\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXOK_A {
    #[doc = "0: No successful reception since last (most recent) flag reset."]
    VALUE1 = 0,
    #[doc = "1: A message has been received successfully."]
    VALUE2 = 1,
}
impl From<RXOK_A> for bool {
    #[inline(always)]
    fn from(variant: RXOK_A) -> Self {
        variant as u8 != 0
    }
}
impl RXOK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RXOK_A {
        match self.bits {
            false => RXOK_A::VALUE1,
            true => RXOK_A::VALUE2,
        }
    }
    #[doc = "No successful reception since last (most recent) flag reset."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == RXOK_A::VALUE1
    }
    #[doc = "A message has been received successfully."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == RXOK_A::VALUE2
    }
}
#[doc = "Field `RXOK` writer - Message Received Successfully"]
pub type RXOK_W<'a, REG> = crate::BitWriter<'a, REG, RXOK_A>;
impl<'a, REG> RXOK_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No successful reception since last (most recent) flag reset."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(RXOK_A::VALUE1)
    }
    #[doc = "A message has been received successfully."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(RXOK_A::VALUE2)
    }
}
#[doc = "Field `ALERT` reader - Alert Warning"]
pub type ALERT_R = crate::BitReader;
#[doc = "Field `ALERT` writer - Alert Warning"]
pub type ALERT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EWRN` reader - Error Warning Status"]
pub type EWRN_R = crate::BitReader<EWRN_A>;
#[doc = "Error Warning Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EWRN_A {
    #[doc = "0: No warning limit exceeded."]
    VALUE1 = 0,
    #[doc = "1: One of the error counters REC or TEC reached the warning limit EWRNLVL."]
    VALUE2 = 1,
}
impl From<EWRN_A> for bool {
    #[inline(always)]
    fn from(variant: EWRN_A) -> Self {
        variant as u8 != 0
    }
}
impl EWRN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EWRN_A {
        match self.bits {
            false => EWRN_A::VALUE1,
            true => EWRN_A::VALUE2,
        }
    }
    #[doc = "No warning limit exceeded."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == EWRN_A::VALUE1
    }
    #[doc = "One of the error counters REC or TEC reached the warning limit EWRNLVL."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == EWRN_A::VALUE2
    }
}
#[doc = "Field `BOFF` reader - Bus-off Status"]
pub type BOFF_R = crate::BitReader<BOFF_A>;
#[doc = "Bus-off Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BOFF_A {
    #[doc = "0: CAN controller is not in the bus-off state."]
    VALUE1 = 0,
    #[doc = "1: CAN controller is in the bus-off state."]
    VALUE2 = 1,
}
impl From<BOFF_A> for bool {
    #[inline(always)]
    fn from(variant: BOFF_A) -> Self {
        variant as u8 != 0
    }
}
impl BOFF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BOFF_A {
        match self.bits {
            false => BOFF_A::VALUE1,
            true => BOFF_A::VALUE2,
        }
    }
    #[doc = "CAN controller is not in the bus-off state."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == BOFF_A::VALUE1
    }
    #[doc = "CAN controller is in the bus-off state."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == BOFF_A::VALUE2
    }
}
#[doc = "Field `LLE` reader - List Length Error"]
pub type LLE_R = crate::BitReader<LLE_A>;
#[doc = "List Length Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LLE_A {
    #[doc = "0: No List Length Error since last (most recent) flag reset."]
    VALUE1 = 0,
    #[doc = "1: A List Length Error has been detected during message acceptance filtering. The number of elements in the list that belongs to this CAN node differs from the list SIZE given in the list termination pointer."]
    VALUE2 = 1,
}
impl From<LLE_A> for bool {
    #[inline(always)]
    fn from(variant: LLE_A) -> Self {
        variant as u8 != 0
    }
}
impl LLE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LLE_A {
        match self.bits {
            false => LLE_A::VALUE1,
            true => LLE_A::VALUE2,
        }
    }
    #[doc = "No List Length Error since last (most recent) flag reset."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == LLE_A::VALUE1
    }
    #[doc = "A List Length Error has been detected during message acceptance filtering. The number of elements in the list that belongs to this CAN node differs from the list SIZE given in the list termination pointer."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == LLE_A::VALUE2
    }
}
#[doc = "Field `LLE` writer - List Length Error"]
pub type LLE_W<'a, REG> = crate::BitWriter<'a, REG, LLE_A>;
impl<'a, REG> LLE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No List Length Error since last (most recent) flag reset."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(LLE_A::VALUE1)
    }
    #[doc = "A List Length Error has been detected during message acceptance filtering. The number of elements in the list that belongs to this CAN node differs from the list SIZE given in the list termination pointer."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(LLE_A::VALUE2)
    }
}
#[doc = "Field `LOE` reader - List Object Error"]
pub type LOE_R = crate::BitReader<LOE_A>;
#[doc = "List Object Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LOE_A {
    #[doc = "0: No List Object Error since last (most recent) flag reset."]
    VALUE1 = 0,
    #[doc = "1: A List Object Error has been detected during message acceptance filtering. A message object with wrong LIST index entry in the Message Object Status Register has been detected."]
    VALUE2 = 1,
}
impl From<LOE_A> for bool {
    #[inline(always)]
    fn from(variant: LOE_A) -> Self {
        variant as u8 != 0
    }
}
impl LOE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LOE_A {
        match self.bits {
            false => LOE_A::VALUE1,
            true => LOE_A::VALUE2,
        }
    }
    #[doc = "No List Object Error since last (most recent) flag reset."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == LOE_A::VALUE1
    }
    #[doc = "A List Object Error has been detected during message acceptance filtering. A message object with wrong LIST index entry in the Message Object Status Register has been detected."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == LOE_A::VALUE2
    }
}
#[doc = "Field `LOE` writer - List Object Error"]
pub type LOE_W<'a, REG> = crate::BitWriter<'a, REG, LOE_A>;
impl<'a, REG> LOE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No List Object Error since last (most recent) flag reset."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(LOE_A::VALUE1)
    }
    #[doc = "A List Object Error has been detected during message acceptance filtering. A message object with wrong LIST index entry in the Message Object Status Register has been detected."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(LOE_A::VALUE2)
    }
}
impl R {
    #[doc = "Bits 0:2 - Last Error Code"]
    #[inline(always)]
    pub fn lec(&self) -> LEC_R {
        LEC_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - Message Transmitted Successfully"]
    #[inline(always)]
    pub fn txok(&self) -> TXOK_R {
        TXOK_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Message Received Successfully"]
    #[inline(always)]
    pub fn rxok(&self) -> RXOK_R {
        RXOK_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Alert Warning"]
    #[inline(always)]
    pub fn alert(&self) -> ALERT_R {
        ALERT_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Error Warning Status"]
    #[inline(always)]
    pub fn ewrn(&self) -> EWRN_R {
        EWRN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Bus-off Status"]
    #[inline(always)]
    pub fn boff(&self) -> BOFF_R {
        BOFF_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - List Length Error"]
    #[inline(always)]
    pub fn lle(&self) -> LLE_R {
        LLE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - List Object Error"]
    #[inline(always)]
    pub fn loe(&self) -> LOE_R {
        LOE_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Last Error Code"]
    #[inline(always)]
    #[must_use]
    pub fn lec(&mut self) -> LEC_W<NSR_SPEC> {
        LEC_W::new(self, 0)
    }
    #[doc = "Bit 3 - Message Transmitted Successfully"]
    #[inline(always)]
    #[must_use]
    pub fn txok(&mut self) -> TXOK_W<NSR_SPEC> {
        TXOK_W::new(self, 3)
    }
    #[doc = "Bit 4 - Message Received Successfully"]
    #[inline(always)]
    #[must_use]
    pub fn rxok(&mut self) -> RXOK_W<NSR_SPEC> {
        RXOK_W::new(self, 4)
    }
    #[doc = "Bit 5 - Alert Warning"]
    #[inline(always)]
    #[must_use]
    pub fn alert(&mut self) -> ALERT_W<NSR_SPEC> {
        ALERT_W::new(self, 5)
    }
    #[doc = "Bit 8 - List Length Error"]
    #[inline(always)]
    #[must_use]
    pub fn lle(&mut self) -> LLE_W<NSR_SPEC> {
        LLE_W::new(self, 8)
    }
    #[doc = "Bit 9 - List Object Error"]
    #[inline(always)]
    #[must_use]
    pub fn loe(&mut self) -> LOE_W<NSR_SPEC> {
        LOE_W::new(self, 9)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Node Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`nsr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`nsr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NSR_SPEC;
impl crate::RegisterSpec for NSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`nsr::R`](R) reader structure"]
impl crate::Readable for NSR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`nsr::W`](W) writer structure"]
impl crate::Writable for NSR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets NSR to value 0"]
impl crate::Resettable for NSR_SPEC {
    const RESET_VALUE: u32 = 0;
}
