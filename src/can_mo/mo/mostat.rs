#[doc = "Register `MOSTAT` reader"]
pub type R = crate::R<MOSTAT_SPEC>;
#[doc = "Receive Pending\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXPND_A {
    #[doc = "0: No CAN message has been received."]
    VALUE1 = 0,
    #[doc = "1: A CAN message has been received by the message object n, either directly or via gateway copy action."]
    VALUE2 = 1,
}
impl From<RXPND_A> for bool {
    #[inline(always)]
    fn from(variant: RXPND_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXPND` reader - Receive Pending"]
pub type RXPND_R = crate::BitReader<RXPND_A>;
impl RXPND_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RXPND_A {
        match self.bits {
            false => RXPND_A::VALUE1,
            true => RXPND_A::VALUE2,
        }
    }
    #[doc = "No CAN message has been received."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == RXPND_A::VALUE1
    }
    #[doc = "A CAN message has been received by the message object n, either directly or via gateway copy action."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == RXPND_A::VALUE2
    }
}
#[doc = "Transmit Pending\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXPND_A {
    #[doc = "0: No CAN message has been transmitted."]
    VALUE1 = 0,
    #[doc = "1: A CAN message from message object n has been transmitted successfully over the CAN bus."]
    VALUE2 = 1,
}
impl From<TXPND_A> for bool {
    #[inline(always)]
    fn from(variant: TXPND_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXPND` reader - Transmit Pending"]
pub type TXPND_R = crate::BitReader<TXPND_A>;
impl TXPND_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TXPND_A {
        match self.bits {
            false => TXPND_A::VALUE1,
            true => TXPND_A::VALUE2,
        }
    }
    #[doc = "No CAN message has been transmitted."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == TXPND_A::VALUE1
    }
    #[doc = "A CAN message from message object n has been transmitted successfully over the CAN bus."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == TXPND_A::VALUE2
    }
}
#[doc = "Receive Updating\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXUPD_A {
    #[doc = "0: No receive update ongoing."]
    VALUE1 = 0,
    #[doc = "1: Message identifier, DLC, and data of the message object are currently updated."]
    VALUE2 = 1,
}
impl From<RXUPD_A> for bool {
    #[inline(always)]
    fn from(variant: RXUPD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXUPD` reader - Receive Updating"]
pub type RXUPD_R = crate::BitReader<RXUPD_A>;
impl RXUPD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RXUPD_A {
        match self.bits {
            false => RXUPD_A::VALUE1,
            true => RXUPD_A::VALUE2,
        }
    }
    #[doc = "No receive update ongoing."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == RXUPD_A::VALUE1
    }
    #[doc = "Message identifier, DLC, and data of the message object are currently updated."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == RXUPD_A::VALUE2
    }
}
#[doc = "New Data\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NEWDAT_A {
    #[doc = "0: No update of the message object n since last flag reset."]
    VALUE1 = 0,
    #[doc = "1: Message object n has been updated."]
    VALUE2 = 1,
}
impl From<NEWDAT_A> for bool {
    #[inline(always)]
    fn from(variant: NEWDAT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NEWDAT` reader - New Data"]
pub type NEWDAT_R = crate::BitReader<NEWDAT_A>;
impl NEWDAT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> NEWDAT_A {
        match self.bits {
            false => NEWDAT_A::VALUE1,
            true => NEWDAT_A::VALUE2,
        }
    }
    #[doc = "No update of the message object n since last flag reset."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == NEWDAT_A::VALUE1
    }
    #[doc = "Message object n has been updated."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == NEWDAT_A::VALUE2
    }
}
#[doc = "Message Lost\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSGLST_A {
    #[doc = "0: No CAN message is lost."]
    VALUE1 = 0,
    #[doc = "1: A CAN message is lost because NEWDAT has become set again when it has already been set."]
    VALUE2 = 1,
}
impl From<MSGLST_A> for bool {
    #[inline(always)]
    fn from(variant: MSGLST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MSGLST` reader - Message Lost"]
pub type MSGLST_R = crate::BitReader<MSGLST_A>;
impl MSGLST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MSGLST_A {
        match self.bits {
            false => MSGLST_A::VALUE1,
            true => MSGLST_A::VALUE2,
        }
    }
    #[doc = "No CAN message is lost."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == MSGLST_A::VALUE1
    }
    #[doc = "A CAN message is lost because NEWDAT has become set again when it has already been set."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == MSGLST_A::VALUE2
    }
}
#[doc = "Message Valid\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSGVAL_A {
    #[doc = "0: Message object n is not valid."]
    VALUE1 = 0,
    #[doc = "1: Message object n is valid."]
    VALUE2 = 1,
}
impl From<MSGVAL_A> for bool {
    #[inline(always)]
    fn from(variant: MSGVAL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MSGVAL` reader - Message Valid"]
pub type MSGVAL_R = crate::BitReader<MSGVAL_A>;
impl MSGVAL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MSGVAL_A {
        match self.bits {
            false => MSGVAL_A::VALUE1,
            true => MSGVAL_A::VALUE2,
        }
    }
    #[doc = "Message object n is not valid."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == MSGVAL_A::VALUE1
    }
    #[doc = "Message object n is valid."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == MSGVAL_A::VALUE2
    }
}
#[doc = "Receive/Transmit Selected\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RTSEL_A {
    #[doc = "0: Message object n is not selected for receive or transmit operation."]
    VALUE1 = 0,
    #[doc = "1: Message object n is selected for receive or transmit operation."]
    VALUE2 = 1,
}
impl From<RTSEL_A> for bool {
    #[inline(always)]
    fn from(variant: RTSEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RTSEL` reader - Receive/Transmit Selected"]
pub type RTSEL_R = crate::BitReader<RTSEL_A>;
impl RTSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RTSEL_A {
        match self.bits {
            false => RTSEL_A::VALUE1,
            true => RTSEL_A::VALUE2,
        }
    }
    #[doc = "Message object n is not selected for receive or transmit operation."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == RTSEL_A::VALUE1
    }
    #[doc = "Message object n is selected for receive or transmit operation."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == RTSEL_A::VALUE2
    }
}
#[doc = "Receive Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXEN_A {
    #[doc = "0: Message object n is not enabled for frame reception."]
    VALUE1 = 0,
    #[doc = "1: Message object n is enabled for frame reception."]
    VALUE2 = 1,
}
impl From<RXEN_A> for bool {
    #[inline(always)]
    fn from(variant: RXEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXEN` reader - Receive Enable"]
pub type RXEN_R = crate::BitReader<RXEN_A>;
impl RXEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RXEN_A {
        match self.bits {
            false => RXEN_A::VALUE1,
            true => RXEN_A::VALUE2,
        }
    }
    #[doc = "Message object n is not enabled for frame reception."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == RXEN_A::VALUE1
    }
    #[doc = "Message object n is enabled for frame reception."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == RXEN_A::VALUE2
    }
}
#[doc = "Transmit Request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXRQ_A {
    #[doc = "0: No transmission of message object n is requested."]
    VALUE1 = 0,
    #[doc = "1: Transmission of message object n on the CAN bus is requested."]
    VALUE2 = 1,
}
impl From<TXRQ_A> for bool {
    #[inline(always)]
    fn from(variant: TXRQ_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXRQ` reader - Transmit Request"]
pub type TXRQ_R = crate::BitReader<TXRQ_A>;
impl TXRQ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TXRQ_A {
        match self.bits {
            false => TXRQ_A::VALUE1,
            true => TXRQ_A::VALUE2,
        }
    }
    #[doc = "No transmission of message object n is requested."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == TXRQ_A::VALUE1
    }
    #[doc = "Transmission of message object n on the CAN bus is requested."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == TXRQ_A::VALUE2
    }
}
#[doc = "Transmit Enable 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXEN0_A {
    #[doc = "0: Message object n is not enabled for frame transmission."]
    VALUE1 = 0,
    #[doc = "1: Message object n is enabled for frame transmission."]
    VALUE2 = 1,
}
impl From<TXEN0_A> for bool {
    #[inline(always)]
    fn from(variant: TXEN0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXEN0` reader - Transmit Enable 0"]
pub type TXEN0_R = crate::BitReader<TXEN0_A>;
impl TXEN0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TXEN0_A {
        match self.bits {
            false => TXEN0_A::VALUE1,
            true => TXEN0_A::VALUE2,
        }
    }
    #[doc = "Message object n is not enabled for frame transmission."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == TXEN0_A::VALUE1
    }
    #[doc = "Message object n is enabled for frame transmission."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == TXEN0_A::VALUE2
    }
}
#[doc = "Transmit Enable 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXEN1_A {
    #[doc = "0: Message object n is not enabled for frame transmission."]
    VALUE1 = 0,
    #[doc = "1: Message object n is enabled for frame transmission."]
    VALUE2 = 1,
}
impl From<TXEN1_A> for bool {
    #[inline(always)]
    fn from(variant: TXEN1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXEN1` reader - Transmit Enable 1"]
pub type TXEN1_R = crate::BitReader<TXEN1_A>;
impl TXEN1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TXEN1_A {
        match self.bits {
            false => TXEN1_A::VALUE1,
            true => TXEN1_A::VALUE2,
        }
    }
    #[doc = "Message object n is not enabled for frame transmission."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == TXEN1_A::VALUE1
    }
    #[doc = "Message object n is enabled for frame transmission."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == TXEN1_A::VALUE2
    }
}
#[doc = "Message Direction\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DIR_A {
    #[doc = "0: Receive Object selected: With TXRQ = 1, a Remote Frame with the identifier of message object n is scheduled for transmission. On reception of a Data Frame with matching identifier, the message is stored in message object n."]
    VALUE1 = 0,
    #[doc = "1: Transmit Object selected: If TXRQ = 1, message object n is scheduled for transmission of a Data Frame. On reception of a Remote Frame with matching identifier, bit TXRQ is set."]
    VALUE2 = 1,
}
impl From<DIR_A> for bool {
    #[inline(always)]
    fn from(variant: DIR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIR` reader - Message Direction"]
pub type DIR_R = crate::BitReader<DIR_A>;
impl DIR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DIR_A {
        match self.bits {
            false => DIR_A::VALUE1,
            true => DIR_A::VALUE2,
        }
    }
    #[doc = "Receive Object selected: With TXRQ = 1, a Remote Frame with the identifier of message object n is scheduled for transmission. On reception of a Data Frame with matching identifier, the message is stored in message object n."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == DIR_A::VALUE1
    }
    #[doc = "Transmit Object selected: If TXRQ = 1, message object n is scheduled for transmission of a Data Frame. On reception of a Remote Frame with matching identifier, bit TXRQ is set."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == DIR_A::VALUE2
    }
}
#[doc = "Field `LIST` reader - List Allocation"]
pub type LIST_R = crate::FieldReader;
#[doc = "Field `PPREV` reader - Pointer to Previous Message Object"]
pub type PPREV_R = crate::FieldReader;
#[doc = "Field `PNEXT` reader - Pointer to Next Message Object"]
pub type PNEXT_R = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - Receive Pending"]
    #[inline(always)]
    pub fn rxpnd(&self) -> RXPND_R {
        RXPND_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmit Pending"]
    #[inline(always)]
    pub fn txpnd(&self) -> TXPND_R {
        TXPND_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Receive Updating"]
    #[inline(always)]
    pub fn rxupd(&self) -> RXUPD_R {
        RXUPD_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - New Data"]
    #[inline(always)]
    pub fn newdat(&self) -> NEWDAT_R {
        NEWDAT_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Message Lost"]
    #[inline(always)]
    pub fn msglst(&self) -> MSGLST_R {
        MSGLST_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Message Valid"]
    #[inline(always)]
    pub fn msgval(&self) -> MSGVAL_R {
        MSGVAL_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Receive/Transmit Selected"]
    #[inline(always)]
    pub fn rtsel(&self) -> RTSEL_R {
        RTSEL_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Receive Enable"]
    #[inline(always)]
    pub fn rxen(&self) -> RXEN_R {
        RXEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Transmit Request"]
    #[inline(always)]
    pub fn txrq(&self) -> TXRQ_R {
        TXRQ_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Transmit Enable 0"]
    #[inline(always)]
    pub fn txen0(&self) -> TXEN0_R {
        TXEN0_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Transmit Enable 1"]
    #[inline(always)]
    pub fn txen1(&self) -> TXEN1_R {
        TXEN1_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Message Direction"]
    #[inline(always)]
    pub fn dir(&self) -> DIR_R {
        DIR_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:15 - List Allocation"]
    #[inline(always)]
    pub fn list(&self) -> LIST_R {
        LIST_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:23 - Pointer to Previous Message Object"]
    #[inline(always)]
    pub fn pprev(&self) -> PPREV_R {
        PPREV_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Pointer to Next Message Object"]
    #[inline(always)]
    pub fn pnext(&self) -> PNEXT_R {
        PNEXT_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[doc = "Message Object Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mostat::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MOSTAT_SPEC;
impl crate::RegisterSpec for MOSTAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mostat::R`](R) reader structure"]
impl crate::Readable for MOSTAT_SPEC {}
#[doc = "`reset()` method sets MOSTAT to value 0"]
impl crate::Resettable for MOSTAT_SPEC {
    const RESET_VALUE: u32 = 0;
}
