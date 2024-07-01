#[doc = "Register `TRBSR` reader"]
pub type R = crate::R<TRBSR_SPEC>;
#[doc = "Register `TRBSR` writer"]
pub type W = crate::W<TRBSR_SPEC>;
#[doc = "Standard Receive Buffer Event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SRBI_A {
    #[doc = "0: A standard receive buffer event has not been detected."]
    VALUE1 = 0,
    #[doc = "1: A standard receive buffer event has been detected."]
    VALUE2 = 1,
}
impl From<SRBI_A> for bool {
    #[inline(always)]
    fn from(variant: SRBI_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRBI` reader - Standard Receive Buffer Event"]
pub type SRBI_R = crate::BitReader<SRBI_A>;
impl SRBI_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SRBI_A {
        match self.bits {
            false => SRBI_A::VALUE1,
            true => SRBI_A::VALUE2,
        }
    }
    #[doc = "A standard receive buffer event has not been detected."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SRBI_A::VALUE1
    }
    #[doc = "A standard receive buffer event has been detected."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SRBI_A::VALUE2
    }
}
#[doc = "Field `SRBI` writer - Standard Receive Buffer Event"]
pub type SRBI_W<'a, REG> = crate::BitWriter<'a, REG, SRBI_A>;
impl<'a, REG> SRBI_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "A standard receive buffer event has not been detected."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(SRBI_A::VALUE1)
    }
    #[doc = "A standard receive buffer event has been detected."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(SRBI_A::VALUE2)
    }
}
#[doc = "Receive Buffer Error Event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RBERI_A {
    #[doc = "0: A receive buffer error event has not been detected."]
    VALUE1 = 0,
    #[doc = "1: A receive buffer error event has been detected."]
    VALUE2 = 1,
}
impl From<RBERI_A> for bool {
    #[inline(always)]
    fn from(variant: RBERI_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RBERI` reader - Receive Buffer Error Event"]
pub type RBERI_R = crate::BitReader<RBERI_A>;
impl RBERI_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RBERI_A {
        match self.bits {
            false => RBERI_A::VALUE1,
            true => RBERI_A::VALUE2,
        }
    }
    #[doc = "A receive buffer error event has not been detected."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == RBERI_A::VALUE1
    }
    #[doc = "A receive buffer error event has been detected."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == RBERI_A::VALUE2
    }
}
#[doc = "Field `RBERI` writer - Receive Buffer Error Event"]
pub type RBERI_W<'a, REG> = crate::BitWriter<'a, REG, RBERI_A>;
impl<'a, REG> RBERI_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "A receive buffer error event has not been detected."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(RBERI_A::VALUE1)
    }
    #[doc = "A receive buffer error event has been detected."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(RBERI_A::VALUE2)
    }
}
#[doc = "Alternative Receive Buffer Event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ARBI_A {
    #[doc = "0: An alternative receive buffer event has not been detected."]
    VALUE1 = 0,
    #[doc = "1: An alternative receive buffer event has been detected."]
    VALUE2 = 1,
}
impl From<ARBI_A> for bool {
    #[inline(always)]
    fn from(variant: ARBI_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ARBI` reader - Alternative Receive Buffer Event"]
pub type ARBI_R = crate::BitReader<ARBI_A>;
impl ARBI_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ARBI_A {
        match self.bits {
            false => ARBI_A::VALUE1,
            true => ARBI_A::VALUE2,
        }
    }
    #[doc = "An alternative receive buffer event has not been detected."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == ARBI_A::VALUE1
    }
    #[doc = "An alternative receive buffer event has been detected."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == ARBI_A::VALUE2
    }
}
#[doc = "Field `ARBI` writer - Alternative Receive Buffer Event"]
pub type ARBI_W<'a, REG> = crate::BitWriter<'a, REG, ARBI_A>;
impl<'a, REG> ARBI_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "An alternative receive buffer event has not been detected."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(ARBI_A::VALUE1)
    }
    #[doc = "An alternative receive buffer event has been detected."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(ARBI_A::VALUE2)
    }
}
#[doc = "Receive Buffer Empty\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REMPTY_A {
    #[doc = "0: The receive buffer is not empty."]
    VALUE1 = 0,
    #[doc = "1: The receive buffer is empty."]
    VALUE2 = 1,
}
impl From<REMPTY_A> for bool {
    #[inline(always)]
    fn from(variant: REMPTY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REMPTY` reader - Receive Buffer Empty"]
pub type REMPTY_R = crate::BitReader<REMPTY_A>;
impl REMPTY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> REMPTY_A {
        match self.bits {
            false => REMPTY_A::VALUE1,
            true => REMPTY_A::VALUE2,
        }
    }
    #[doc = "The receive buffer is not empty."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == REMPTY_A::VALUE1
    }
    #[doc = "The receive buffer is empty."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == REMPTY_A::VALUE2
    }
}
#[doc = "Receive Buffer Full\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RFULL_A {
    #[doc = "0: The receive buffer is not full."]
    VALUE1 = 0,
    #[doc = "1: The receive buffer is full."]
    VALUE2 = 1,
}
impl From<RFULL_A> for bool {
    #[inline(always)]
    fn from(variant: RFULL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RFULL` reader - Receive Buffer Full"]
pub type RFULL_R = crate::BitReader<RFULL_A>;
impl RFULL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RFULL_A {
        match self.bits {
            false => RFULL_A::VALUE1,
            true => RFULL_A::VALUE2,
        }
    }
    #[doc = "The receive buffer is not full."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == RFULL_A::VALUE1
    }
    #[doc = "The receive buffer is full."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == RFULL_A::VALUE2
    }
}
#[doc = "Receive Buffer Busy\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RBUS_A {
    #[doc = "0: The receive buffer information has been completely updated."]
    VALUE1 = 0,
    #[doc = "1: The OUTR update from the FIFO memory is ongoing. A read from OUTR will be delayed. FIFO pointers from the previous read are not yet updated."]
    VALUE2 = 1,
}
impl From<RBUS_A> for bool {
    #[inline(always)]
    fn from(variant: RBUS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RBUS` reader - Receive Buffer Busy"]
pub type RBUS_R = crate::BitReader<RBUS_A>;
impl RBUS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RBUS_A {
        match self.bits {
            false => RBUS_A::VALUE1,
            true => RBUS_A::VALUE2,
        }
    }
    #[doc = "The receive buffer information has been completely updated."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == RBUS_A::VALUE1
    }
    #[doc = "The OUTR update from the FIFO memory is ongoing. A read from OUTR will be delayed. FIFO pointers from the previous read are not yet updated."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == RBUS_A::VALUE2
    }
}
#[doc = "Standard Receive Buffer Event Trigger\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SRBT_A {
    #[doc = "0: A standard receive buffer event is not triggered using this bit."]
    VALUE1 = 0,
    #[doc = "1: A standard receive buffer event is triggered using this bit."]
    VALUE2 = 1,
}
impl From<SRBT_A> for bool {
    #[inline(always)]
    fn from(variant: SRBT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRBT` reader - Standard Receive Buffer Event Trigger"]
pub type SRBT_R = crate::BitReader<SRBT_A>;
impl SRBT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SRBT_A {
        match self.bits {
            false => SRBT_A::VALUE1,
            true => SRBT_A::VALUE2,
        }
    }
    #[doc = "A standard receive buffer event is not triggered using this bit."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SRBT_A::VALUE1
    }
    #[doc = "A standard receive buffer event is triggered using this bit."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SRBT_A::VALUE2
    }
}
#[doc = "Standard Transmit Buffer Event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum STBI_A {
    #[doc = "0: A standard transmit buffer event has not been detected."]
    VALUE1 = 0,
    #[doc = "1: A standard transmit buffer event has been detected."]
    VALUE2 = 1,
}
impl From<STBI_A> for bool {
    #[inline(always)]
    fn from(variant: STBI_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STBI` reader - Standard Transmit Buffer Event"]
pub type STBI_R = crate::BitReader<STBI_A>;
impl STBI_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> STBI_A {
        match self.bits {
            false => STBI_A::VALUE1,
            true => STBI_A::VALUE2,
        }
    }
    #[doc = "A standard transmit buffer event has not been detected."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == STBI_A::VALUE1
    }
    #[doc = "A standard transmit buffer event has been detected."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == STBI_A::VALUE2
    }
}
#[doc = "Field `STBI` writer - Standard Transmit Buffer Event"]
pub type STBI_W<'a, REG> = crate::BitWriter<'a, REG, STBI_A>;
impl<'a, REG> STBI_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "A standard transmit buffer event has not been detected."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(STBI_A::VALUE1)
    }
    #[doc = "A standard transmit buffer event has been detected."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(STBI_A::VALUE2)
    }
}
#[doc = "Transmit Buffer Error Event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TBERI_A {
    #[doc = "0: A transmit buffer error event has not been detected."]
    VALUE1 = 0,
    #[doc = "1: A transmit buffer error event has been detected."]
    VALUE2 = 1,
}
impl From<TBERI_A> for bool {
    #[inline(always)]
    fn from(variant: TBERI_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TBERI` reader - Transmit Buffer Error Event"]
pub type TBERI_R = crate::BitReader<TBERI_A>;
impl TBERI_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TBERI_A {
        match self.bits {
            false => TBERI_A::VALUE1,
            true => TBERI_A::VALUE2,
        }
    }
    #[doc = "A transmit buffer error event has not been detected."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == TBERI_A::VALUE1
    }
    #[doc = "A transmit buffer error event has been detected."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == TBERI_A::VALUE2
    }
}
#[doc = "Field `TBERI` writer - Transmit Buffer Error Event"]
pub type TBERI_W<'a, REG> = crate::BitWriter<'a, REG, TBERI_A>;
impl<'a, REG> TBERI_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "A transmit buffer error event has not been detected."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(TBERI_A::VALUE1)
    }
    #[doc = "A transmit buffer error event has been detected."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(TBERI_A::VALUE2)
    }
}
#[doc = "Transmit Buffer Empty\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TEMPTY_A {
    #[doc = "0: The transmit buffer is not empty."]
    VALUE1 = 0,
    #[doc = "1: The transmit buffer is empty."]
    VALUE2 = 1,
}
impl From<TEMPTY_A> for bool {
    #[inline(always)]
    fn from(variant: TEMPTY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TEMPTY` reader - Transmit Buffer Empty"]
pub type TEMPTY_R = crate::BitReader<TEMPTY_A>;
impl TEMPTY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TEMPTY_A {
        match self.bits {
            false => TEMPTY_A::VALUE1,
            true => TEMPTY_A::VALUE2,
        }
    }
    #[doc = "The transmit buffer is not empty."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == TEMPTY_A::VALUE1
    }
    #[doc = "The transmit buffer is empty."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == TEMPTY_A::VALUE2
    }
}
#[doc = "Transmit Buffer Full\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TFULL_A {
    #[doc = "0: The transmit buffer is not full."]
    VALUE1 = 0,
    #[doc = "1: The transmit buffer is full."]
    VALUE2 = 1,
}
impl From<TFULL_A> for bool {
    #[inline(always)]
    fn from(variant: TFULL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TFULL` reader - Transmit Buffer Full"]
pub type TFULL_R = crate::BitReader<TFULL_A>;
impl TFULL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TFULL_A {
        match self.bits {
            false => TFULL_A::VALUE1,
            true => TFULL_A::VALUE2,
        }
    }
    #[doc = "The transmit buffer is not full."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == TFULL_A::VALUE1
    }
    #[doc = "The transmit buffer is full."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == TFULL_A::VALUE2
    }
}
#[doc = "Transmit Buffer Busy\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TBUS_A {
    #[doc = "0: The transmit buffer information has been completely updated."]
    VALUE1 = 0,
    #[doc = "1: The FIFO memory update after write to INx is ongoing. A write to INx will be delayed. FIFO pointers from the previous INx write are not yet updated."]
    VALUE2 = 1,
}
impl From<TBUS_A> for bool {
    #[inline(always)]
    fn from(variant: TBUS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TBUS` reader - Transmit Buffer Busy"]
pub type TBUS_R = crate::BitReader<TBUS_A>;
impl TBUS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TBUS_A {
        match self.bits {
            false => TBUS_A::VALUE1,
            true => TBUS_A::VALUE2,
        }
    }
    #[doc = "The transmit buffer information has been completely updated."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == TBUS_A::VALUE1
    }
    #[doc = "The FIFO memory update after write to INx is ongoing. A write to INx will be delayed. FIFO pointers from the previous INx write are not yet updated."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == TBUS_A::VALUE2
    }
}
#[doc = "Standard Transmit Buffer Event Trigger\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum STBT_A {
    #[doc = "0: A standard transmit buffer event is not triggered using this bit."]
    VALUE1 = 0,
    #[doc = "1: A standard transmit buffer event is triggered using this bit."]
    VALUE2 = 1,
}
impl From<STBT_A> for bool {
    #[inline(always)]
    fn from(variant: STBT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STBT` reader - Standard Transmit Buffer Event Trigger"]
pub type STBT_R = crate::BitReader<STBT_A>;
impl STBT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> STBT_A {
        match self.bits {
            false => STBT_A::VALUE1,
            true => STBT_A::VALUE2,
        }
    }
    #[doc = "A standard transmit buffer event is not triggered using this bit."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == STBT_A::VALUE1
    }
    #[doc = "A standard transmit buffer event is triggered using this bit."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == STBT_A::VALUE2
    }
}
#[doc = "Field `RBFLVL` reader - Receive Buffer Filling Level"]
pub type RBFLVL_R = crate::FieldReader;
#[doc = "Field `TBFLVL` reader - Transmit Buffer Filling Level"]
pub type TBFLVL_R = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - Standard Receive Buffer Event"]
    #[inline(always)]
    pub fn srbi(&self) -> SRBI_R {
        SRBI_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Receive Buffer Error Event"]
    #[inline(always)]
    pub fn rberi(&self) -> RBERI_R {
        RBERI_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Alternative Receive Buffer Event"]
    #[inline(always)]
    pub fn arbi(&self) -> ARBI_R {
        ARBI_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Receive Buffer Empty"]
    #[inline(always)]
    pub fn rempty(&self) -> REMPTY_R {
        REMPTY_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Receive Buffer Full"]
    #[inline(always)]
    pub fn rfull(&self) -> RFULL_R {
        RFULL_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Receive Buffer Busy"]
    #[inline(always)]
    pub fn rbus(&self) -> RBUS_R {
        RBUS_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Standard Receive Buffer Event Trigger"]
    #[inline(always)]
    pub fn srbt(&self) -> SRBT_R {
        SRBT_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - Standard Transmit Buffer Event"]
    #[inline(always)]
    pub fn stbi(&self) -> STBI_R {
        STBI_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Transmit Buffer Error Event"]
    #[inline(always)]
    pub fn tberi(&self) -> TBERI_R {
        TBERI_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 11 - Transmit Buffer Empty"]
    #[inline(always)]
    pub fn tempty(&self) -> TEMPTY_R {
        TEMPTY_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Transmit Buffer Full"]
    #[inline(always)]
    pub fn tfull(&self) -> TFULL_R {
        TFULL_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Transmit Buffer Busy"]
    #[inline(always)]
    pub fn tbus(&self) -> TBUS_R {
        TBUS_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Standard Transmit Buffer Event Trigger"]
    #[inline(always)]
    pub fn stbt(&self) -> STBT_R {
        STBT_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bits 16:22 - Receive Buffer Filling Level"]
    #[inline(always)]
    pub fn rbflvl(&self) -> RBFLVL_R {
        RBFLVL_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 24:30 - Transmit Buffer Filling Level"]
    #[inline(always)]
    pub fn tbflvl(&self) -> TBFLVL_R {
        TBFLVL_R::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Standard Receive Buffer Event"]
    #[inline(always)]
    #[must_use]
    pub fn srbi(&mut self) -> SRBI_W<TRBSR_SPEC> {
        SRBI_W::new(self, 0)
    }
    #[doc = "Bit 1 - Receive Buffer Error Event"]
    #[inline(always)]
    #[must_use]
    pub fn rberi(&mut self) -> RBERI_W<TRBSR_SPEC> {
        RBERI_W::new(self, 1)
    }
    #[doc = "Bit 2 - Alternative Receive Buffer Event"]
    #[inline(always)]
    #[must_use]
    pub fn arbi(&mut self) -> ARBI_W<TRBSR_SPEC> {
        ARBI_W::new(self, 2)
    }
    #[doc = "Bit 8 - Standard Transmit Buffer Event"]
    #[inline(always)]
    #[must_use]
    pub fn stbi(&mut self) -> STBI_W<TRBSR_SPEC> {
        STBI_W::new(self, 8)
    }
    #[doc = "Bit 9 - Transmit Buffer Error Event"]
    #[inline(always)]
    #[must_use]
    pub fn tberi(&mut self) -> TBERI_W<TRBSR_SPEC> {
        TBERI_W::new(self, 9)
    }
}
#[doc = "Transmit/Receive Buffer Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`trbsr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trbsr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TRBSR_SPEC;
impl crate::RegisterSpec for TRBSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`trbsr::R`](R) reader structure"]
impl crate::Readable for TRBSR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`trbsr::W`](W) writer structure"]
impl crate::Writable for TRBSR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TRBSR to value 0x0808"]
impl crate::Resettable for TRBSR_SPEC {
    const RESET_VALUE: u32 = 0x0808;
}
