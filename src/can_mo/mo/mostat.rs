#[doc = "Register `MOSTAT` reader"]
pub struct R(crate::R<MOSTAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MOSTAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MOSTAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MOSTAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Receive Pending\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
pub struct RXPND_R(crate::FieldReader<bool, RXPND_A>);
impl RXPND_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXPND_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXPND_A {
        match self.bits {
            false => RXPND_A::VALUE1,
            true => RXPND_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == RXPND_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == RXPND_A::VALUE2
    }
}
impl core::ops::Deref for RXPND_R {
    type Target = crate::FieldReader<bool, RXPND_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Transmit Pending\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
pub struct TXPND_R(crate::FieldReader<bool, TXPND_A>);
impl TXPND_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXPND_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXPND_A {
        match self.bits {
            false => TXPND_A::VALUE1,
            true => TXPND_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == TXPND_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == TXPND_A::VALUE2
    }
}
impl core::ops::Deref for TXPND_R {
    type Target = crate::FieldReader<bool, TXPND_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Receive Updating\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
pub struct RXUPD_R(crate::FieldReader<bool, RXUPD_A>);
impl RXUPD_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXUPD_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXUPD_A {
        match self.bits {
            false => RXUPD_A::VALUE1,
            true => RXUPD_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == RXUPD_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == RXUPD_A::VALUE2
    }
}
impl core::ops::Deref for RXUPD_R {
    type Target = crate::FieldReader<bool, RXUPD_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "New Data\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
pub struct NEWDAT_R(crate::FieldReader<bool, NEWDAT_A>);
impl NEWDAT_R {
    pub(crate) fn new(bits: bool) -> Self {
        NEWDAT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NEWDAT_A {
        match self.bits {
            false => NEWDAT_A::VALUE1,
            true => NEWDAT_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == NEWDAT_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == NEWDAT_A::VALUE2
    }
}
impl core::ops::Deref for NEWDAT_R {
    type Target = crate::FieldReader<bool, NEWDAT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Message Lost\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
pub struct MSGLST_R(crate::FieldReader<bool, MSGLST_A>);
impl MSGLST_R {
    pub(crate) fn new(bits: bool) -> Self {
        MSGLST_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MSGLST_A {
        match self.bits {
            false => MSGLST_A::VALUE1,
            true => MSGLST_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == MSGLST_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == MSGLST_A::VALUE2
    }
}
impl core::ops::Deref for MSGLST_R {
    type Target = crate::FieldReader<bool, MSGLST_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Message Valid\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
pub struct MSGVAL_R(crate::FieldReader<bool, MSGVAL_A>);
impl MSGVAL_R {
    pub(crate) fn new(bits: bool) -> Self {
        MSGVAL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MSGVAL_A {
        match self.bits {
            false => MSGVAL_A::VALUE1,
            true => MSGVAL_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == MSGVAL_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == MSGVAL_A::VALUE2
    }
}
impl core::ops::Deref for MSGVAL_R {
    type Target = crate::FieldReader<bool, MSGVAL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Receive/Transmit Selected\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
pub struct RTSEL_R(crate::FieldReader<bool, RTSEL_A>);
impl RTSEL_R {
    pub(crate) fn new(bits: bool) -> Self {
        RTSEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RTSEL_A {
        match self.bits {
            false => RTSEL_A::VALUE1,
            true => RTSEL_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == RTSEL_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == RTSEL_A::VALUE2
    }
}
impl core::ops::Deref for RTSEL_R {
    type Target = crate::FieldReader<bool, RTSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Receive Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
pub struct RXEN_R(crate::FieldReader<bool, RXEN_A>);
impl RXEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXEN_A {
        match self.bits {
            false => RXEN_A::VALUE1,
            true => RXEN_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == RXEN_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == RXEN_A::VALUE2
    }
}
impl core::ops::Deref for RXEN_R {
    type Target = crate::FieldReader<bool, RXEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Transmit Request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
pub struct TXRQ_R(crate::FieldReader<bool, TXRQ_A>);
impl TXRQ_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXRQ_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXRQ_A {
        match self.bits {
            false => TXRQ_A::VALUE1,
            true => TXRQ_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == TXRQ_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == TXRQ_A::VALUE2
    }
}
impl core::ops::Deref for TXRQ_R {
    type Target = crate::FieldReader<bool, TXRQ_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Transmit Enable 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
pub struct TXEN0_R(crate::FieldReader<bool, TXEN0_A>);
impl TXEN0_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXEN0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXEN0_A {
        match self.bits {
            false => TXEN0_A::VALUE1,
            true => TXEN0_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == TXEN0_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == TXEN0_A::VALUE2
    }
}
impl core::ops::Deref for TXEN0_R {
    type Target = crate::FieldReader<bool, TXEN0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Transmit Enable 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
pub struct TXEN1_R(crate::FieldReader<bool, TXEN1_A>);
impl TXEN1_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXEN1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXEN1_A {
        match self.bits {
            false => TXEN1_A::VALUE1,
            true => TXEN1_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == TXEN1_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == TXEN1_A::VALUE2
    }
}
impl core::ops::Deref for TXEN1_R {
    type Target = crate::FieldReader<bool, TXEN1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Message Direction\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
pub struct DIR_R(crate::FieldReader<bool, DIR_A>);
impl DIR_R {
    pub(crate) fn new(bits: bool) -> Self {
        DIR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIR_A {
        match self.bits {
            false => DIR_A::VALUE1,
            true => DIR_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == DIR_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == DIR_A::VALUE2
    }
}
impl core::ops::Deref for DIR_R {
    type Target = crate::FieldReader<bool, DIR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LIST` reader - List Allocation"]
pub struct LIST_R(crate::FieldReader<u8, u8>);
impl LIST_R {
    pub(crate) fn new(bits: u8) -> Self {
        LIST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LIST_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PPREV` reader - Pointer to Previous Message Object"]
pub struct PPREV_R(crate::FieldReader<u8, u8>);
impl PPREV_R {
    pub(crate) fn new(bits: u8) -> Self {
        PPREV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PPREV_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PNEXT` reader - Pointer to Next Message Object"]
pub struct PNEXT_R(crate::FieldReader<u8, u8>);
impl PNEXT_R {
    pub(crate) fn new(bits: u8) -> Self {
        PNEXT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PNEXT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Receive Pending"]
    #[inline(always)]
    pub fn rxpnd(&self) -> RXPND_R {
        RXPND_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Transmit Pending"]
    #[inline(always)]
    pub fn txpnd(&self) -> TXPND_R {
        TXPND_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Receive Updating"]
    #[inline(always)]
    pub fn rxupd(&self) -> RXUPD_R {
        RXUPD_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - New Data"]
    #[inline(always)]
    pub fn newdat(&self) -> NEWDAT_R {
        NEWDAT_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Message Lost"]
    #[inline(always)]
    pub fn msglst(&self) -> MSGLST_R {
        MSGLST_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Message Valid"]
    #[inline(always)]
    pub fn msgval(&self) -> MSGVAL_R {
        MSGVAL_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Receive/Transmit Selected"]
    #[inline(always)]
    pub fn rtsel(&self) -> RTSEL_R {
        RTSEL_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Receive Enable"]
    #[inline(always)]
    pub fn rxen(&self) -> RXEN_R {
        RXEN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Transmit Request"]
    #[inline(always)]
    pub fn txrq(&self) -> TXRQ_R {
        TXRQ_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Transmit Enable 0"]
    #[inline(always)]
    pub fn txen0(&self) -> TXEN0_R {
        TXEN0_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Transmit Enable 1"]
    #[inline(always)]
    pub fn txen1(&self) -> TXEN1_R {
        TXEN1_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Message Direction"]
    #[inline(always)]
    pub fn dir(&self) -> DIR_R {
        DIR_R::new(((self.bits >> 11) & 0x01) != 0)
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
#[doc = "Message Object Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mostat](index.html) module"]
pub struct MOSTAT_SPEC;
impl crate::RegisterSpec for MOSTAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mostat::R](R) reader structure"]
impl crate::Readable for MOSTAT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets MOSTAT to value 0"]
impl crate::Resettable for MOSTAT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
