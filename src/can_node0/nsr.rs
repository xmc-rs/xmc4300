#[doc = "Register `NSR` reader"]
pub type R = crate::R<NsrSpec>;
#[doc = "Register `NSR` writer"]
pub type W = crate::W<NsrSpec>;
#[doc = "Field `LEC` reader - Last Error Code"]
pub type LecR = crate::FieldReader;
#[doc = "Field `LEC` writer - Last Error Code"]
pub type LecW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Message Transmitted Successfully\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Txok {
    #[doc = "0: No successful transmission since last (most recent) flag reset."]
    Value1 = 0,
    #[doc = "1: A message has been transmitted successfully (error-free and acknowledged by at least another node)."]
    Value2 = 1,
}
impl From<Txok> for bool {
    #[inline(always)]
    fn from(variant: Txok) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXOK` reader - Message Transmitted Successfully"]
pub type TxokR = crate::BitReader<Txok>;
impl TxokR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Txok {
        match self.bits {
            false => Txok::Value1,
            true => Txok::Value2,
        }
    }
    #[doc = "No successful transmission since last (most recent) flag reset."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Txok::Value1
    }
    #[doc = "A message has been transmitted successfully (error-free and acknowledged by at least another node)."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Txok::Value2
    }
}
#[doc = "Field `TXOK` writer - Message Transmitted Successfully"]
pub type TxokW<'a, REG> = crate::BitWriter<'a, REG, Txok>;
impl<'a, REG> TxokW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No successful transmission since last (most recent) flag reset."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Txok::Value1)
    }
    #[doc = "A message has been transmitted successfully (error-free and acknowledged by at least another node)."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Txok::Value2)
    }
}
#[doc = "Message Received Successfully\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rxok {
    #[doc = "0: No successful reception since last (most recent) flag reset."]
    Value1 = 0,
    #[doc = "1: A message has been received successfully."]
    Value2 = 1,
}
impl From<Rxok> for bool {
    #[inline(always)]
    fn from(variant: Rxok) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXOK` reader - Message Received Successfully"]
pub type RxokR = crate::BitReader<Rxok>;
impl RxokR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rxok {
        match self.bits {
            false => Rxok::Value1,
            true => Rxok::Value2,
        }
    }
    #[doc = "No successful reception since last (most recent) flag reset."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Rxok::Value1
    }
    #[doc = "A message has been received successfully."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Rxok::Value2
    }
}
#[doc = "Field `RXOK` writer - Message Received Successfully"]
pub type RxokW<'a, REG> = crate::BitWriter<'a, REG, Rxok>;
impl<'a, REG> RxokW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No successful reception since last (most recent) flag reset."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Rxok::Value1)
    }
    #[doc = "A message has been received successfully."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Rxok::Value2)
    }
}
#[doc = "Field `ALERT` reader - Alert Warning"]
pub type AlertR = crate::BitReader;
#[doc = "Field `ALERT` writer - Alert Warning"]
pub type AlertW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Error Warning Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ewrn {
    #[doc = "0: No warning limit exceeded."]
    Value1 = 0,
    #[doc = "1: One of the error counters REC or TEC reached the warning limit EWRNLVL."]
    Value2 = 1,
}
impl From<Ewrn> for bool {
    #[inline(always)]
    fn from(variant: Ewrn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EWRN` reader - Error Warning Status"]
pub type EwrnR = crate::BitReader<Ewrn>;
impl EwrnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ewrn {
        match self.bits {
            false => Ewrn::Value1,
            true => Ewrn::Value2,
        }
    }
    #[doc = "No warning limit exceeded."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Ewrn::Value1
    }
    #[doc = "One of the error counters REC or TEC reached the warning limit EWRNLVL."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Ewrn::Value2
    }
}
#[doc = "Bus-off Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Boff {
    #[doc = "0: CAN controller is not in the bus-off state."]
    Value1 = 0,
    #[doc = "1: CAN controller is in the bus-off state."]
    Value2 = 1,
}
impl From<Boff> for bool {
    #[inline(always)]
    fn from(variant: Boff) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BOFF` reader - Bus-off Status"]
pub type BoffR = crate::BitReader<Boff>;
impl BoffR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Boff {
        match self.bits {
            false => Boff::Value1,
            true => Boff::Value2,
        }
    }
    #[doc = "CAN controller is not in the bus-off state."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Boff::Value1
    }
    #[doc = "CAN controller is in the bus-off state."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Boff::Value2
    }
}
#[doc = "List Length Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lle {
    #[doc = "0: No List Length Error since last (most recent) flag reset."]
    Value1 = 0,
    #[doc = "1: A List Length Error has been detected during message acceptance filtering. The number of elements in the list that belongs to this CAN node differs from the list SIZE given in the list termination pointer."]
    Value2 = 1,
}
impl From<Lle> for bool {
    #[inline(always)]
    fn from(variant: Lle) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LLE` reader - List Length Error"]
pub type LleR = crate::BitReader<Lle>;
impl LleR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lle {
        match self.bits {
            false => Lle::Value1,
            true => Lle::Value2,
        }
    }
    #[doc = "No List Length Error since last (most recent) flag reset."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Lle::Value1
    }
    #[doc = "A List Length Error has been detected during message acceptance filtering. The number of elements in the list that belongs to this CAN node differs from the list SIZE given in the list termination pointer."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Lle::Value2
    }
}
#[doc = "Field `LLE` writer - List Length Error"]
pub type LleW<'a, REG> = crate::BitWriter<'a, REG, Lle>;
impl<'a, REG> LleW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No List Length Error since last (most recent) flag reset."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Lle::Value1)
    }
    #[doc = "A List Length Error has been detected during message acceptance filtering. The number of elements in the list that belongs to this CAN node differs from the list SIZE given in the list termination pointer."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Lle::Value2)
    }
}
#[doc = "List Object Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Loe {
    #[doc = "0: No List Object Error since last (most recent) flag reset."]
    Value1 = 0,
    #[doc = "1: A List Object Error has been detected during message acceptance filtering. A message object with wrong LIST index entry in the Message Object Status Register has been detected."]
    Value2 = 1,
}
impl From<Loe> for bool {
    #[inline(always)]
    fn from(variant: Loe) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LOE` reader - List Object Error"]
pub type LoeR = crate::BitReader<Loe>;
impl LoeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Loe {
        match self.bits {
            false => Loe::Value1,
            true => Loe::Value2,
        }
    }
    #[doc = "No List Object Error since last (most recent) flag reset."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Loe::Value1
    }
    #[doc = "A List Object Error has been detected during message acceptance filtering. A message object with wrong LIST index entry in the Message Object Status Register has been detected."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Loe::Value2
    }
}
#[doc = "Field `LOE` writer - List Object Error"]
pub type LoeW<'a, REG> = crate::BitWriter<'a, REG, Loe>;
impl<'a, REG> LoeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No List Object Error since last (most recent) flag reset."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Loe::Value1)
    }
    #[doc = "A List Object Error has been detected during message acceptance filtering. A message object with wrong LIST index entry in the Message Object Status Register has been detected."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Loe::Value2)
    }
}
impl R {
    #[doc = "Bits 0:2 - Last Error Code"]
    #[inline(always)]
    pub fn lec(&self) -> LecR {
        LecR::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - Message Transmitted Successfully"]
    #[inline(always)]
    pub fn txok(&self) -> TxokR {
        TxokR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Message Received Successfully"]
    #[inline(always)]
    pub fn rxok(&self) -> RxokR {
        RxokR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Alert Warning"]
    #[inline(always)]
    pub fn alert(&self) -> AlertR {
        AlertR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Error Warning Status"]
    #[inline(always)]
    pub fn ewrn(&self) -> EwrnR {
        EwrnR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Bus-off Status"]
    #[inline(always)]
    pub fn boff(&self) -> BoffR {
        BoffR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - List Length Error"]
    #[inline(always)]
    pub fn lle(&self) -> LleR {
        LleR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - List Object Error"]
    #[inline(always)]
    pub fn loe(&self) -> LoeR {
        LoeR::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Last Error Code"]
    #[inline(always)]
    #[must_use]
    pub fn lec(&mut self) -> LecW<NsrSpec> {
        LecW::new(self, 0)
    }
    #[doc = "Bit 3 - Message Transmitted Successfully"]
    #[inline(always)]
    #[must_use]
    pub fn txok(&mut self) -> TxokW<NsrSpec> {
        TxokW::new(self, 3)
    }
    #[doc = "Bit 4 - Message Received Successfully"]
    #[inline(always)]
    #[must_use]
    pub fn rxok(&mut self) -> RxokW<NsrSpec> {
        RxokW::new(self, 4)
    }
    #[doc = "Bit 5 - Alert Warning"]
    #[inline(always)]
    #[must_use]
    pub fn alert(&mut self) -> AlertW<NsrSpec> {
        AlertW::new(self, 5)
    }
    #[doc = "Bit 8 - List Length Error"]
    #[inline(always)]
    #[must_use]
    pub fn lle(&mut self) -> LleW<NsrSpec> {
        LleW::new(self, 8)
    }
    #[doc = "Bit 9 - List Object Error"]
    #[inline(always)]
    #[must_use]
    pub fn loe(&mut self) -> LoeW<NsrSpec> {
        LoeW::new(self, 9)
    }
}
#[doc = "Node Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`nsr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`nsr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NsrSpec;
impl crate::RegisterSpec for NsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`nsr::R`](R) reader structure"]
impl crate::Readable for NsrSpec {}
#[doc = "`write(|w| ..)` method takes [`nsr::W`](W) writer structure"]
impl crate::Writable for NsrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets NSR to value 0"]
impl crate::Resettable for NsrSpec {
    const RESET_VALUE: u32 = 0;
}
