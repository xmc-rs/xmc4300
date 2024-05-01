#[doc = "Register `TRBSR` reader"]
pub type R = crate::R<TrbsrSpec>;
#[doc = "Register `TRBSR` writer"]
pub type W = crate::W<TrbsrSpec>;
#[doc = "Standard Receive Buffer Event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Srbi {
    #[doc = "0: A standard receive buffer event has not been detected."]
    Value1 = 0,
    #[doc = "1: A standard receive buffer event has been detected."]
    Value2 = 1,
}
impl From<Srbi> for bool {
    #[inline(always)]
    fn from(variant: Srbi) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRBI` reader - Standard Receive Buffer Event"]
pub type SrbiR = crate::BitReader<Srbi>;
impl SrbiR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Srbi {
        match self.bits {
            false => Srbi::Value1,
            true => Srbi::Value2,
        }
    }
    #[doc = "A standard receive buffer event has not been detected."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Srbi::Value1
    }
    #[doc = "A standard receive buffer event has been detected."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Srbi::Value2
    }
}
#[doc = "Field `SRBI` writer - Standard Receive Buffer Event"]
pub type SrbiW<'a, REG> = crate::BitWriter<'a, REG, Srbi>;
impl<'a, REG> SrbiW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "A standard receive buffer event has not been detected."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Srbi::Value1)
    }
    #[doc = "A standard receive buffer event has been detected."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Srbi::Value2)
    }
}
#[doc = "Receive Buffer Error Event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rberi {
    #[doc = "0: A receive buffer error event has not been detected."]
    Value1 = 0,
    #[doc = "1: A receive buffer error event has been detected."]
    Value2 = 1,
}
impl From<Rberi> for bool {
    #[inline(always)]
    fn from(variant: Rberi) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RBERI` reader - Receive Buffer Error Event"]
pub type RberiR = crate::BitReader<Rberi>;
impl RberiR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rberi {
        match self.bits {
            false => Rberi::Value1,
            true => Rberi::Value2,
        }
    }
    #[doc = "A receive buffer error event has not been detected."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Rberi::Value1
    }
    #[doc = "A receive buffer error event has been detected."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Rberi::Value2
    }
}
#[doc = "Field `RBERI` writer - Receive Buffer Error Event"]
pub type RberiW<'a, REG> = crate::BitWriter<'a, REG, Rberi>;
impl<'a, REG> RberiW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "A receive buffer error event has not been detected."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Rberi::Value1)
    }
    #[doc = "A receive buffer error event has been detected."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Rberi::Value2)
    }
}
#[doc = "Alternative Receive Buffer Event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Arbi {
    #[doc = "0: An alternative receive buffer event has not been detected."]
    Value1 = 0,
    #[doc = "1: An alternative receive buffer event has been detected."]
    Value2 = 1,
}
impl From<Arbi> for bool {
    #[inline(always)]
    fn from(variant: Arbi) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ARBI` reader - Alternative Receive Buffer Event"]
pub type ArbiR = crate::BitReader<Arbi>;
impl ArbiR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Arbi {
        match self.bits {
            false => Arbi::Value1,
            true => Arbi::Value2,
        }
    }
    #[doc = "An alternative receive buffer event has not been detected."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Arbi::Value1
    }
    #[doc = "An alternative receive buffer event has been detected."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Arbi::Value2
    }
}
#[doc = "Field `ARBI` writer - Alternative Receive Buffer Event"]
pub type ArbiW<'a, REG> = crate::BitWriter<'a, REG, Arbi>;
impl<'a, REG> ArbiW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "An alternative receive buffer event has not been detected."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Arbi::Value1)
    }
    #[doc = "An alternative receive buffer event has been detected."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Arbi::Value2)
    }
}
#[doc = "Receive Buffer Empty\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rempty {
    #[doc = "0: The receive buffer is not empty."]
    Value1 = 0,
    #[doc = "1: The receive buffer is empty."]
    Value2 = 1,
}
impl From<Rempty> for bool {
    #[inline(always)]
    fn from(variant: Rempty) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REMPTY` reader - Receive Buffer Empty"]
pub type RemptyR = crate::BitReader<Rempty>;
impl RemptyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rempty {
        match self.bits {
            false => Rempty::Value1,
            true => Rempty::Value2,
        }
    }
    #[doc = "The receive buffer is not empty."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Rempty::Value1
    }
    #[doc = "The receive buffer is empty."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Rempty::Value2
    }
}
#[doc = "Receive Buffer Full\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rfull {
    #[doc = "0: The receive buffer is not full."]
    Value1 = 0,
    #[doc = "1: The receive buffer is full."]
    Value2 = 1,
}
impl From<Rfull> for bool {
    #[inline(always)]
    fn from(variant: Rfull) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RFULL` reader - Receive Buffer Full"]
pub type RfullR = crate::BitReader<Rfull>;
impl RfullR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rfull {
        match self.bits {
            false => Rfull::Value1,
            true => Rfull::Value2,
        }
    }
    #[doc = "The receive buffer is not full."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Rfull::Value1
    }
    #[doc = "The receive buffer is full."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Rfull::Value2
    }
}
#[doc = "Receive Buffer Busy\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rbus {
    #[doc = "0: The receive buffer information has been completely updated."]
    Value1 = 0,
    #[doc = "1: The OUTR update from the FIFO memory is ongoing. A read from OUTR will be delayed. FIFO pointers from the previous read are not yet updated."]
    Value2 = 1,
}
impl From<Rbus> for bool {
    #[inline(always)]
    fn from(variant: Rbus) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RBUS` reader - Receive Buffer Busy"]
pub type RbusR = crate::BitReader<Rbus>;
impl RbusR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rbus {
        match self.bits {
            false => Rbus::Value1,
            true => Rbus::Value2,
        }
    }
    #[doc = "The receive buffer information has been completely updated."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Rbus::Value1
    }
    #[doc = "The OUTR update from the FIFO memory is ongoing. A read from OUTR will be delayed. FIFO pointers from the previous read are not yet updated."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Rbus::Value2
    }
}
#[doc = "Standard Receive Buffer Event Trigger\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Srbt {
    #[doc = "0: A standard receive buffer event is not triggered using this bit."]
    Value1 = 0,
    #[doc = "1: A standard receive buffer event is triggered using this bit."]
    Value2 = 1,
}
impl From<Srbt> for bool {
    #[inline(always)]
    fn from(variant: Srbt) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRBT` reader - Standard Receive Buffer Event Trigger"]
pub type SrbtR = crate::BitReader<Srbt>;
impl SrbtR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Srbt {
        match self.bits {
            false => Srbt::Value1,
            true => Srbt::Value2,
        }
    }
    #[doc = "A standard receive buffer event is not triggered using this bit."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Srbt::Value1
    }
    #[doc = "A standard receive buffer event is triggered using this bit."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Srbt::Value2
    }
}
#[doc = "Standard Transmit Buffer Event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Stbi {
    #[doc = "0: A standard transmit buffer event has not been detected."]
    Value1 = 0,
    #[doc = "1: A standard transmit buffer event has been detected."]
    Value2 = 1,
}
impl From<Stbi> for bool {
    #[inline(always)]
    fn from(variant: Stbi) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STBI` reader - Standard Transmit Buffer Event"]
pub type StbiR = crate::BitReader<Stbi>;
impl StbiR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Stbi {
        match self.bits {
            false => Stbi::Value1,
            true => Stbi::Value2,
        }
    }
    #[doc = "A standard transmit buffer event has not been detected."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Stbi::Value1
    }
    #[doc = "A standard transmit buffer event has been detected."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Stbi::Value2
    }
}
#[doc = "Field `STBI` writer - Standard Transmit Buffer Event"]
pub type StbiW<'a, REG> = crate::BitWriter<'a, REG, Stbi>;
impl<'a, REG> StbiW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "A standard transmit buffer event has not been detected."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Stbi::Value1)
    }
    #[doc = "A standard transmit buffer event has been detected."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Stbi::Value2)
    }
}
#[doc = "Transmit Buffer Error Event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tberi {
    #[doc = "0: A transmit buffer error event has not been detected."]
    Value1 = 0,
    #[doc = "1: A transmit buffer error event has been detected."]
    Value2 = 1,
}
impl From<Tberi> for bool {
    #[inline(always)]
    fn from(variant: Tberi) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TBERI` reader - Transmit Buffer Error Event"]
pub type TberiR = crate::BitReader<Tberi>;
impl TberiR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tberi {
        match self.bits {
            false => Tberi::Value1,
            true => Tberi::Value2,
        }
    }
    #[doc = "A transmit buffer error event has not been detected."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Tberi::Value1
    }
    #[doc = "A transmit buffer error event has been detected."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Tberi::Value2
    }
}
#[doc = "Field `TBERI` writer - Transmit Buffer Error Event"]
pub type TberiW<'a, REG> = crate::BitWriter<'a, REG, Tberi>;
impl<'a, REG> TberiW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "A transmit buffer error event has not been detected."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Tberi::Value1)
    }
    #[doc = "A transmit buffer error event has been detected."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Tberi::Value2)
    }
}
#[doc = "Transmit Buffer Empty\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tempty {
    #[doc = "0: The transmit buffer is not empty."]
    Value1 = 0,
    #[doc = "1: The transmit buffer is empty."]
    Value2 = 1,
}
impl From<Tempty> for bool {
    #[inline(always)]
    fn from(variant: Tempty) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TEMPTY` reader - Transmit Buffer Empty"]
pub type TemptyR = crate::BitReader<Tempty>;
impl TemptyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tempty {
        match self.bits {
            false => Tempty::Value1,
            true => Tempty::Value2,
        }
    }
    #[doc = "The transmit buffer is not empty."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Tempty::Value1
    }
    #[doc = "The transmit buffer is empty."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Tempty::Value2
    }
}
#[doc = "Transmit Buffer Full\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tfull {
    #[doc = "0: The transmit buffer is not full."]
    Value1 = 0,
    #[doc = "1: The transmit buffer is full."]
    Value2 = 1,
}
impl From<Tfull> for bool {
    #[inline(always)]
    fn from(variant: Tfull) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TFULL` reader - Transmit Buffer Full"]
pub type TfullR = crate::BitReader<Tfull>;
impl TfullR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tfull {
        match self.bits {
            false => Tfull::Value1,
            true => Tfull::Value2,
        }
    }
    #[doc = "The transmit buffer is not full."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Tfull::Value1
    }
    #[doc = "The transmit buffer is full."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Tfull::Value2
    }
}
#[doc = "Transmit Buffer Busy\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tbus {
    #[doc = "0: The transmit buffer information has been completely updated."]
    Value1 = 0,
    #[doc = "1: The FIFO memory update after write to INx is ongoing. A write to INx will be delayed. FIFO pointers from the previous INx write are not yet updated."]
    Value2 = 1,
}
impl From<Tbus> for bool {
    #[inline(always)]
    fn from(variant: Tbus) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TBUS` reader - Transmit Buffer Busy"]
pub type TbusR = crate::BitReader<Tbus>;
impl TbusR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tbus {
        match self.bits {
            false => Tbus::Value1,
            true => Tbus::Value2,
        }
    }
    #[doc = "The transmit buffer information has been completely updated."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Tbus::Value1
    }
    #[doc = "The FIFO memory update after write to INx is ongoing. A write to INx will be delayed. FIFO pointers from the previous INx write are not yet updated."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Tbus::Value2
    }
}
#[doc = "Standard Transmit Buffer Event Trigger\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Stbt {
    #[doc = "0: A standard transmit buffer event is not triggered using this bit."]
    Value1 = 0,
    #[doc = "1: A standard transmit buffer event is triggered using this bit."]
    Value2 = 1,
}
impl From<Stbt> for bool {
    #[inline(always)]
    fn from(variant: Stbt) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STBT` reader - Standard Transmit Buffer Event Trigger"]
pub type StbtR = crate::BitReader<Stbt>;
impl StbtR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Stbt {
        match self.bits {
            false => Stbt::Value1,
            true => Stbt::Value2,
        }
    }
    #[doc = "A standard transmit buffer event is not triggered using this bit."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Stbt::Value1
    }
    #[doc = "A standard transmit buffer event is triggered using this bit."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Stbt::Value2
    }
}
#[doc = "Field `RBFLVL` reader - Receive Buffer Filling Level"]
pub type RbflvlR = crate::FieldReader;
#[doc = "Field `TBFLVL` reader - Transmit Buffer Filling Level"]
pub type TbflvlR = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - Standard Receive Buffer Event"]
    #[inline(always)]
    pub fn srbi(&self) -> SrbiR {
        SrbiR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Receive Buffer Error Event"]
    #[inline(always)]
    pub fn rberi(&self) -> RberiR {
        RberiR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Alternative Receive Buffer Event"]
    #[inline(always)]
    pub fn arbi(&self) -> ArbiR {
        ArbiR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Receive Buffer Empty"]
    #[inline(always)]
    pub fn rempty(&self) -> RemptyR {
        RemptyR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Receive Buffer Full"]
    #[inline(always)]
    pub fn rfull(&self) -> RfullR {
        RfullR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Receive Buffer Busy"]
    #[inline(always)]
    pub fn rbus(&self) -> RbusR {
        RbusR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Standard Receive Buffer Event Trigger"]
    #[inline(always)]
    pub fn srbt(&self) -> SrbtR {
        SrbtR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - Standard Transmit Buffer Event"]
    #[inline(always)]
    pub fn stbi(&self) -> StbiR {
        StbiR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Transmit Buffer Error Event"]
    #[inline(always)]
    pub fn tberi(&self) -> TberiR {
        TberiR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 11 - Transmit Buffer Empty"]
    #[inline(always)]
    pub fn tempty(&self) -> TemptyR {
        TemptyR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Transmit Buffer Full"]
    #[inline(always)]
    pub fn tfull(&self) -> TfullR {
        TfullR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Transmit Buffer Busy"]
    #[inline(always)]
    pub fn tbus(&self) -> TbusR {
        TbusR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Standard Transmit Buffer Event Trigger"]
    #[inline(always)]
    pub fn stbt(&self) -> StbtR {
        StbtR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bits 16:22 - Receive Buffer Filling Level"]
    #[inline(always)]
    pub fn rbflvl(&self) -> RbflvlR {
        RbflvlR::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 24:30 - Transmit Buffer Filling Level"]
    #[inline(always)]
    pub fn tbflvl(&self) -> TbflvlR {
        TbflvlR::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Standard Receive Buffer Event"]
    #[inline(always)]
    #[must_use]
    pub fn srbi(&mut self) -> SrbiW<TrbsrSpec> {
        SrbiW::new(self, 0)
    }
    #[doc = "Bit 1 - Receive Buffer Error Event"]
    #[inline(always)]
    #[must_use]
    pub fn rberi(&mut self) -> RberiW<TrbsrSpec> {
        RberiW::new(self, 1)
    }
    #[doc = "Bit 2 - Alternative Receive Buffer Event"]
    #[inline(always)]
    #[must_use]
    pub fn arbi(&mut self) -> ArbiW<TrbsrSpec> {
        ArbiW::new(self, 2)
    }
    #[doc = "Bit 8 - Standard Transmit Buffer Event"]
    #[inline(always)]
    #[must_use]
    pub fn stbi(&mut self) -> StbiW<TrbsrSpec> {
        StbiW::new(self, 8)
    }
    #[doc = "Bit 9 - Transmit Buffer Error Event"]
    #[inline(always)]
    #[must_use]
    pub fn tberi(&mut self) -> TberiW<TrbsrSpec> {
        TberiW::new(self, 9)
    }
}
#[doc = "Transmit/Receive Buffer Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`trbsr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`trbsr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TrbsrSpec;
impl crate::RegisterSpec for TrbsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`trbsr::R`](R) reader structure"]
impl crate::Readable for TrbsrSpec {}
#[doc = "`write(|w| ..)` method takes [`trbsr::W`](W) writer structure"]
impl crate::Writable for TrbsrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TRBSR to value 0x0808"]
impl crate::Resettable for TrbsrSpec {
    const RESET_VALUE: u32 = 0x0808;
}
