#[doc = "Register `MOSTAT` reader"]
pub type R = crate::R<MostatSpec>;
#[doc = "Receive Pending\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rxpnd {
    #[doc = "0: No CAN message has been received."]
    Value1 = 0,
    #[doc = "1: A CAN message has been received by the message object n, either directly or via gateway copy action."]
    Value2 = 1,
}
impl From<Rxpnd> for bool {
    #[inline(always)]
    fn from(variant: Rxpnd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXPND` reader - Receive Pending"]
pub type RxpndR = crate::BitReader<Rxpnd>;
impl RxpndR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rxpnd {
        match self.bits {
            false => Rxpnd::Value1,
            true => Rxpnd::Value2,
        }
    }
    #[doc = "No CAN message has been received."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Rxpnd::Value1
    }
    #[doc = "A CAN message has been received by the message object n, either directly or via gateway copy action."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Rxpnd::Value2
    }
}
#[doc = "Transmit Pending\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Txpnd {
    #[doc = "0: No CAN message has been transmitted."]
    Value1 = 0,
    #[doc = "1: A CAN message from message object n has been transmitted successfully over the CAN bus."]
    Value2 = 1,
}
impl From<Txpnd> for bool {
    #[inline(always)]
    fn from(variant: Txpnd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXPND` reader - Transmit Pending"]
pub type TxpndR = crate::BitReader<Txpnd>;
impl TxpndR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Txpnd {
        match self.bits {
            false => Txpnd::Value1,
            true => Txpnd::Value2,
        }
    }
    #[doc = "No CAN message has been transmitted."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Txpnd::Value1
    }
    #[doc = "A CAN message from message object n has been transmitted successfully over the CAN bus."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Txpnd::Value2
    }
}
#[doc = "Receive Updating\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rxupd {
    #[doc = "0: No receive update ongoing."]
    Value1 = 0,
    #[doc = "1: Message identifier, DLC, and data of the message object are currently updated."]
    Value2 = 1,
}
impl From<Rxupd> for bool {
    #[inline(always)]
    fn from(variant: Rxupd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXUPD` reader - Receive Updating"]
pub type RxupdR = crate::BitReader<Rxupd>;
impl RxupdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rxupd {
        match self.bits {
            false => Rxupd::Value1,
            true => Rxupd::Value2,
        }
    }
    #[doc = "No receive update ongoing."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Rxupd::Value1
    }
    #[doc = "Message identifier, DLC, and data of the message object are currently updated."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Rxupd::Value2
    }
}
#[doc = "New Data\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Newdat {
    #[doc = "0: No update of the message object n since last flag reset."]
    Value1 = 0,
    #[doc = "1: Message object n has been updated."]
    Value2 = 1,
}
impl From<Newdat> for bool {
    #[inline(always)]
    fn from(variant: Newdat) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NEWDAT` reader - New Data"]
pub type NewdatR = crate::BitReader<Newdat>;
impl NewdatR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Newdat {
        match self.bits {
            false => Newdat::Value1,
            true => Newdat::Value2,
        }
    }
    #[doc = "No update of the message object n since last flag reset."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Newdat::Value1
    }
    #[doc = "Message object n has been updated."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Newdat::Value2
    }
}
#[doc = "Message Lost\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Msglst {
    #[doc = "0: No CAN message is lost."]
    Value1 = 0,
    #[doc = "1: A CAN message is lost because NEWDAT has become set again when it has already been set."]
    Value2 = 1,
}
impl From<Msglst> for bool {
    #[inline(always)]
    fn from(variant: Msglst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MSGLST` reader - Message Lost"]
pub type MsglstR = crate::BitReader<Msglst>;
impl MsglstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Msglst {
        match self.bits {
            false => Msglst::Value1,
            true => Msglst::Value2,
        }
    }
    #[doc = "No CAN message is lost."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Msglst::Value1
    }
    #[doc = "A CAN message is lost because NEWDAT has become set again when it has already been set."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Msglst::Value2
    }
}
#[doc = "Message Valid\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Msgval {
    #[doc = "0: Message object n is not valid."]
    Value1 = 0,
    #[doc = "1: Message object n is valid."]
    Value2 = 1,
}
impl From<Msgval> for bool {
    #[inline(always)]
    fn from(variant: Msgval) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MSGVAL` reader - Message Valid"]
pub type MsgvalR = crate::BitReader<Msgval>;
impl MsgvalR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Msgval {
        match self.bits {
            false => Msgval::Value1,
            true => Msgval::Value2,
        }
    }
    #[doc = "Message object n is not valid."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Msgval::Value1
    }
    #[doc = "Message object n is valid."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Msgval::Value2
    }
}
#[doc = "Receive/Transmit Selected\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rtsel {
    #[doc = "0: Message object n is not selected for receive or transmit operation."]
    Value1 = 0,
    #[doc = "1: Message object n is selected for receive or transmit operation."]
    Value2 = 1,
}
impl From<Rtsel> for bool {
    #[inline(always)]
    fn from(variant: Rtsel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RTSEL` reader - Receive/Transmit Selected"]
pub type RtselR = crate::BitReader<Rtsel>;
impl RtselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rtsel {
        match self.bits {
            false => Rtsel::Value1,
            true => Rtsel::Value2,
        }
    }
    #[doc = "Message object n is not selected for receive or transmit operation."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Rtsel::Value1
    }
    #[doc = "Message object n is selected for receive or transmit operation."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Rtsel::Value2
    }
}
#[doc = "Receive Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rxen {
    #[doc = "0: Message object n is not enabled for frame reception."]
    Value1 = 0,
    #[doc = "1: Message object n is enabled for frame reception."]
    Value2 = 1,
}
impl From<Rxen> for bool {
    #[inline(always)]
    fn from(variant: Rxen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXEN` reader - Receive Enable"]
pub type RxenR = crate::BitReader<Rxen>;
impl RxenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rxen {
        match self.bits {
            false => Rxen::Value1,
            true => Rxen::Value2,
        }
    }
    #[doc = "Message object n is not enabled for frame reception."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Rxen::Value1
    }
    #[doc = "Message object n is enabled for frame reception."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Rxen::Value2
    }
}
#[doc = "Transmit Request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Txrq {
    #[doc = "0: No transmission of message object n is requested."]
    Value1 = 0,
    #[doc = "1: Transmission of message object n on the CAN bus is requested."]
    Value2 = 1,
}
impl From<Txrq> for bool {
    #[inline(always)]
    fn from(variant: Txrq) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXRQ` reader - Transmit Request"]
pub type TxrqR = crate::BitReader<Txrq>;
impl TxrqR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Txrq {
        match self.bits {
            false => Txrq::Value1,
            true => Txrq::Value2,
        }
    }
    #[doc = "No transmission of message object n is requested."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Txrq::Value1
    }
    #[doc = "Transmission of message object n on the CAN bus is requested."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Txrq::Value2
    }
}
#[doc = "Transmit Enable 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Txen0 {
    #[doc = "0: Message object n is not enabled for frame transmission."]
    Value1 = 0,
    #[doc = "1: Message object n is enabled for frame transmission."]
    Value2 = 1,
}
impl From<Txen0> for bool {
    #[inline(always)]
    fn from(variant: Txen0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXEN0` reader - Transmit Enable 0"]
pub type Txen0R = crate::BitReader<Txen0>;
impl Txen0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Txen0 {
        match self.bits {
            false => Txen0::Value1,
            true => Txen0::Value2,
        }
    }
    #[doc = "Message object n is not enabled for frame transmission."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Txen0::Value1
    }
    #[doc = "Message object n is enabled for frame transmission."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Txen0::Value2
    }
}
#[doc = "Transmit Enable 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Txen1 {
    #[doc = "0: Message object n is not enabled for frame transmission."]
    Value1 = 0,
    #[doc = "1: Message object n is enabled for frame transmission."]
    Value2 = 1,
}
impl From<Txen1> for bool {
    #[inline(always)]
    fn from(variant: Txen1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXEN1` reader - Transmit Enable 1"]
pub type Txen1R = crate::BitReader<Txen1>;
impl Txen1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Txen1 {
        match self.bits {
            false => Txen1::Value1,
            true => Txen1::Value2,
        }
    }
    #[doc = "Message object n is not enabled for frame transmission."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Txen1::Value1
    }
    #[doc = "Message object n is enabled for frame transmission."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Txen1::Value2
    }
}
#[doc = "Message Direction\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dir {
    #[doc = "0: Receive Object selected: With TXRQ = 1, a Remote Frame with the identifier of message object n is scheduled for transmission. On reception of a Data Frame with matching identifier, the message is stored in message object n."]
    Value1 = 0,
    #[doc = "1: Transmit Object selected: If TXRQ = 1, message object n is scheduled for transmission of a Data Frame. On reception of a Remote Frame with matching identifier, bit TXRQ is set."]
    Value2 = 1,
}
impl From<Dir> for bool {
    #[inline(always)]
    fn from(variant: Dir) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIR` reader - Message Direction"]
pub type DirR = crate::BitReader<Dir>;
impl DirR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dir {
        match self.bits {
            false => Dir::Value1,
            true => Dir::Value2,
        }
    }
    #[doc = "Receive Object selected: With TXRQ = 1, a Remote Frame with the identifier of message object n is scheduled for transmission. On reception of a Data Frame with matching identifier, the message is stored in message object n."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Dir::Value1
    }
    #[doc = "Transmit Object selected: If TXRQ = 1, message object n is scheduled for transmission of a Data Frame. On reception of a Remote Frame with matching identifier, bit TXRQ is set."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Dir::Value2
    }
}
#[doc = "Field `LIST` reader - List Allocation"]
pub type ListR = crate::FieldReader;
#[doc = "Field `PPREV` reader - Pointer to Previous Message Object"]
pub type PprevR = crate::FieldReader;
#[doc = "Field `PNEXT` reader - Pointer to Next Message Object"]
pub type PnextR = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - Receive Pending"]
    #[inline(always)]
    pub fn rxpnd(&self) -> RxpndR {
        RxpndR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmit Pending"]
    #[inline(always)]
    pub fn txpnd(&self) -> TxpndR {
        TxpndR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Receive Updating"]
    #[inline(always)]
    pub fn rxupd(&self) -> RxupdR {
        RxupdR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - New Data"]
    #[inline(always)]
    pub fn newdat(&self) -> NewdatR {
        NewdatR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Message Lost"]
    #[inline(always)]
    pub fn msglst(&self) -> MsglstR {
        MsglstR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Message Valid"]
    #[inline(always)]
    pub fn msgval(&self) -> MsgvalR {
        MsgvalR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Receive/Transmit Selected"]
    #[inline(always)]
    pub fn rtsel(&self) -> RtselR {
        RtselR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Receive Enable"]
    #[inline(always)]
    pub fn rxen(&self) -> RxenR {
        RxenR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Transmit Request"]
    #[inline(always)]
    pub fn txrq(&self) -> TxrqR {
        TxrqR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Transmit Enable 0"]
    #[inline(always)]
    pub fn txen0(&self) -> Txen0R {
        Txen0R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Transmit Enable 1"]
    #[inline(always)]
    pub fn txen1(&self) -> Txen1R {
        Txen1R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Message Direction"]
    #[inline(always)]
    pub fn dir(&self) -> DirR {
        DirR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:15 - List Allocation"]
    #[inline(always)]
    pub fn list(&self) -> ListR {
        ListR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:23 - Pointer to Previous Message Object"]
    #[inline(always)]
    pub fn pprev(&self) -> PprevR {
        PprevR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Pointer to Next Message Object"]
    #[inline(always)]
    pub fn pnext(&self) -> PnextR {
        PnextR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[doc = "Message Object Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mostat::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MostatSpec;
impl crate::RegisterSpec for MostatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mostat::R`](R) reader structure"]
impl crate::Readable for MostatSpec {}
#[doc = "`reset()` method sets MOSTAT to value 0"]
impl crate::Resettable for MostatSpec {
    const RESET_VALUE: u32 = 0;
}
