#[doc = "Register `MOFCR` reader"]
pub type R = crate::R<MofcrSpec>;
#[doc = "Register `MOFCR` writer"]
pub type W = crate::W<MofcrSpec>;
#[doc = "Message Mode Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Mmc {
    #[doc = "0: Standard Message Object"]
    Value1 = 0,
    #[doc = "1: Receive FIFO Base Object"]
    Value2 = 1,
    #[doc = "2: Transmit FIFO Base Object"]
    Value3 = 2,
    #[doc = "3: Transmit FIFO Slave Object"]
    Value4 = 3,
    #[doc = "4: Gateway Source Object"]
    Value5 = 4,
}
impl From<Mmc> for u8 {
    #[inline(always)]
    fn from(variant: Mmc) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Mmc {
    type Ux = u8;
}
#[doc = "Field `MMC` reader - Message Mode Control"]
pub type MmcR = crate::FieldReader<Mmc>;
impl MmcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Mmc> {
        match self.bits {
            0 => Some(Mmc::Value1),
            1 => Some(Mmc::Value2),
            2 => Some(Mmc::Value3),
            3 => Some(Mmc::Value4),
            4 => Some(Mmc::Value5),
            _ => None,
        }
    }
    #[doc = "Standard Message Object"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Mmc::Value1
    }
    #[doc = "Receive FIFO Base Object"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Mmc::Value2
    }
    #[doc = "Transmit FIFO Base Object"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == Mmc::Value3
    }
    #[doc = "Transmit FIFO Slave Object"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == Mmc::Value4
    }
    #[doc = "Gateway Source Object"]
    #[inline(always)]
    pub fn is_value5(&self) -> bool {
        *self == Mmc::Value5
    }
}
#[doc = "Field `MMC` writer - Message Mode Control"]
pub type MmcW<'a, REG> = crate::FieldWriter<'a, REG, 4, Mmc>;
impl<'a, REG> MmcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Standard Message Object"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Mmc::Value1)
    }
    #[doc = "Receive FIFO Base Object"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Mmc::Value2)
    }
    #[doc = "Transmit FIFO Base Object"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(Mmc::Value3)
    }
    #[doc = "Transmit FIFO Slave Object"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(Mmc::Value4)
    }
    #[doc = "Gateway Source Object"]
    #[inline(always)]
    pub fn value5(self) -> &'a mut crate::W<REG> {
        self.variant(Mmc::Value5)
    }
}
#[doc = "Receive Time-Out Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rxtoe {
    #[doc = "0: Message does not take part in receive time-out check"]
    Value1 = 0,
    #[doc = "1: Message takes part in receive time-out check"]
    Value2 = 1,
}
impl From<Rxtoe> for bool {
    #[inline(always)]
    fn from(variant: Rxtoe) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXTOE` reader - Receive Time-Out Enable"]
pub type RxtoeR = crate::BitReader<Rxtoe>;
impl RxtoeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rxtoe {
        match self.bits {
            false => Rxtoe::Value1,
            true => Rxtoe::Value2,
        }
    }
    #[doc = "Message does not take part in receive time-out check"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Rxtoe::Value1
    }
    #[doc = "Message takes part in receive time-out check"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Rxtoe::Value2
    }
}
#[doc = "Field `RXTOE` writer - Receive Time-Out Enable"]
pub type RxtoeW<'a, REG> = crate::BitWriter<'a, REG, Rxtoe>;
impl<'a, REG> RxtoeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Message does not take part in receive time-out check"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Rxtoe::Value1)
    }
    #[doc = "Message takes part in receive time-out check"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Rxtoe::Value2)
    }
}
#[doc = "Gateway Data Frame Send\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gdfs {
    #[doc = "0: TXRQ is unchanged in the destination object."]
    Value1 = 0,
    #[doc = "1: TXRQ is set in the gateway destination object after the internal transfer from the gateway source to the gateway destination object."]
    Value2 = 1,
}
impl From<Gdfs> for bool {
    #[inline(always)]
    fn from(variant: Gdfs) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GDFS` reader - Gateway Data Frame Send"]
pub type GdfsR = crate::BitReader<Gdfs>;
impl GdfsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gdfs {
        match self.bits {
            false => Gdfs::Value1,
            true => Gdfs::Value2,
        }
    }
    #[doc = "TXRQ is unchanged in the destination object."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Gdfs::Value1
    }
    #[doc = "TXRQ is set in the gateway destination object after the internal transfer from the gateway source to the gateway destination object."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Gdfs::Value2
    }
}
#[doc = "Field `GDFS` writer - Gateway Data Frame Send"]
pub type GdfsW<'a, REG> = crate::BitWriter<'a, REG, Gdfs>;
impl<'a, REG> GdfsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "TXRQ is unchanged in the destination object."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Gdfs::Value1)
    }
    #[doc = "TXRQ is set in the gateway destination object after the internal transfer from the gateway source to the gateway destination object."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Gdfs::Value2)
    }
}
#[doc = "Identifier Copy\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Idc {
    #[doc = "0: The identifier of the gateway source object is not copied."]
    Value1 = 0,
    #[doc = "1: The identifier of the gateway source object (after storing the received frame in the source) is copied to the gateway destination object."]
    Value2 = 1,
}
impl From<Idc> for bool {
    #[inline(always)]
    fn from(variant: Idc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IDC` reader - Identifier Copy"]
pub type IdcR = crate::BitReader<Idc>;
impl IdcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Idc {
        match self.bits {
            false => Idc::Value1,
            true => Idc::Value2,
        }
    }
    #[doc = "The identifier of the gateway source object is not copied."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Idc::Value1
    }
    #[doc = "The identifier of the gateway source object (after storing the received frame in the source) is copied to the gateway destination object."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Idc::Value2
    }
}
#[doc = "Field `IDC` writer - Identifier Copy"]
pub type IdcW<'a, REG> = crate::BitWriter<'a, REG, Idc>;
impl<'a, REG> IdcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The identifier of the gateway source object is not copied."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Idc::Value1)
    }
    #[doc = "The identifier of the gateway source object (after storing the received frame in the source) is copied to the gateway destination object."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Idc::Value2)
    }
}
#[doc = "Data Length Code Copy\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dlcc {
    #[doc = "0: Data length code is not copied."]
    Value1 = 0,
    #[doc = "1: Data length code of the gateway source object (after storing the received frame in the source) is copied to the gateway destination object."]
    Value2 = 1,
}
impl From<Dlcc> for bool {
    #[inline(always)]
    fn from(variant: Dlcc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DLCC` reader - Data Length Code Copy"]
pub type DlccR = crate::BitReader<Dlcc>;
impl DlccR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dlcc {
        match self.bits {
            false => Dlcc::Value1,
            true => Dlcc::Value2,
        }
    }
    #[doc = "Data length code is not copied."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Dlcc::Value1
    }
    #[doc = "Data length code of the gateway source object (after storing the received frame in the source) is copied to the gateway destination object."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Dlcc::Value2
    }
}
#[doc = "Field `DLCC` writer - Data Length Code Copy"]
pub type DlccW<'a, REG> = crate::BitWriter<'a, REG, Dlcc>;
impl<'a, REG> DlccW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Data length code is not copied."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Dlcc::Value1)
    }
    #[doc = "Data length code of the gateway source object (after storing the received frame in the source) is copied to the gateway destination object."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Dlcc::Value2)
    }
}
#[doc = "Data Copy\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Datc {
    #[doc = "0: Data fields are not copied."]
    Value1 = 0,
    #[doc = "1: Data fields in registers MODATALn and MODATAHn of the gateway source object (after storing the received frame in the source) are copied to the gateway destination."]
    Value2 = 1,
}
impl From<Datc> for bool {
    #[inline(always)]
    fn from(variant: Datc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DATC` reader - Data Copy"]
pub type DatcR = crate::BitReader<Datc>;
impl DatcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Datc {
        match self.bits {
            false => Datc::Value1,
            true => Datc::Value2,
        }
    }
    #[doc = "Data fields are not copied."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Datc::Value1
    }
    #[doc = "Data fields in registers MODATALn and MODATAHn of the gateway source object (after storing the received frame in the source) are copied to the gateway destination."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Datc::Value2
    }
}
#[doc = "Field `DATC` writer - Data Copy"]
pub type DatcW<'a, REG> = crate::BitWriter<'a, REG, Datc>;
impl<'a, REG> DatcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Data fields are not copied."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Datc::Value1)
    }
    #[doc = "Data fields in registers MODATALn and MODATAHn of the gateway source object (after storing the received frame in the source) are copied to the gateway destination."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Datc::Value2)
    }
}
#[doc = "Receive Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rxie {
    #[doc = "0: Message receive interrupt is disabled."]
    Value1 = 0,
    #[doc = "1: Message receive interrupt is enabled."]
    Value2 = 1,
}
impl From<Rxie> for bool {
    #[inline(always)]
    fn from(variant: Rxie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXIE` reader - Receive Interrupt Enable"]
pub type RxieR = crate::BitReader<Rxie>;
impl RxieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rxie {
        match self.bits {
            false => Rxie::Value1,
            true => Rxie::Value2,
        }
    }
    #[doc = "Message receive interrupt is disabled."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Rxie::Value1
    }
    #[doc = "Message receive interrupt is enabled."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Rxie::Value2
    }
}
#[doc = "Field `RXIE` writer - Receive Interrupt Enable"]
pub type RxieW<'a, REG> = crate::BitWriter<'a, REG, Rxie>;
impl<'a, REG> RxieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Message receive interrupt is disabled."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Rxie::Value1)
    }
    #[doc = "Message receive interrupt is enabled."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Rxie::Value2)
    }
}
#[doc = "Transmit Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Txie {
    #[doc = "0: Message transmit interrupt is disabled."]
    Value1 = 0,
    #[doc = "1: Message transmit interrupt is enabled."]
    Value2 = 1,
}
impl From<Txie> for bool {
    #[inline(always)]
    fn from(variant: Txie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXIE` reader - Transmit Interrupt Enable"]
pub type TxieR = crate::BitReader<Txie>;
impl TxieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Txie {
        match self.bits {
            false => Txie::Value1,
            true => Txie::Value2,
        }
    }
    #[doc = "Message transmit interrupt is disabled."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Txie::Value1
    }
    #[doc = "Message transmit interrupt is enabled."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Txie::Value2
    }
}
#[doc = "Field `TXIE` writer - Transmit Interrupt Enable"]
pub type TxieW<'a, REG> = crate::BitWriter<'a, REG, Txie>;
impl<'a, REG> TxieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Message transmit interrupt is disabled."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Txie::Value1)
    }
    #[doc = "Message transmit interrupt is enabled."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Txie::Value2)
    }
}
#[doc = "Overflow Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ovie {
    #[doc = "0: FIFO full interrupt is disabled."]
    Value1 = 0,
    #[doc = "1: FIFO full interrupt is enabled."]
    Value2 = 1,
}
impl From<Ovie> for bool {
    #[inline(always)]
    fn from(variant: Ovie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OVIE` reader - Overflow Interrupt Enable"]
pub type OvieR = crate::BitReader<Ovie>;
impl OvieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ovie {
        match self.bits {
            false => Ovie::Value1,
            true => Ovie::Value2,
        }
    }
    #[doc = "FIFO full interrupt is disabled."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Ovie::Value1
    }
    #[doc = "FIFO full interrupt is enabled."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Ovie::Value2
    }
}
#[doc = "Field `OVIE` writer - Overflow Interrupt Enable"]
pub type OvieW<'a, REG> = crate::BitWriter<'a, REG, Ovie>;
impl<'a, REG> OvieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "FIFO full interrupt is disabled."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Ovie::Value1)
    }
    #[doc = "FIFO full interrupt is enabled."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Ovie::Value2)
    }
}
#[doc = "Foreign Remote Request Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Frren {
    #[doc = "0: TXRQ of message object n is set on reception of a matching Remote Frame."]
    Value1 = 0,
    #[doc = "1: TXRQ of the message object referenced by the pointer CUR is set on reception of a matching Remote Frame."]
    Value2 = 1,
}
impl From<Frren> for bool {
    #[inline(always)]
    fn from(variant: Frren) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FRREN` reader - Foreign Remote Request Enable"]
pub type FrrenR = crate::BitReader<Frren>;
impl FrrenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Frren {
        match self.bits {
            false => Frren::Value1,
            true => Frren::Value2,
        }
    }
    #[doc = "TXRQ of message object n is set on reception of a matching Remote Frame."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Frren::Value1
    }
    #[doc = "TXRQ of the message object referenced by the pointer CUR is set on reception of a matching Remote Frame."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Frren::Value2
    }
}
#[doc = "Field `FRREN` writer - Foreign Remote Request Enable"]
pub type FrrenW<'a, REG> = crate::BitWriter<'a, REG, Frren>;
impl<'a, REG> FrrenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "TXRQ of message object n is set on reception of a matching Remote Frame."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Frren::Value1)
    }
    #[doc = "TXRQ of the message object referenced by the pointer CUR is set on reception of a matching Remote Frame."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Frren::Value2)
    }
}
#[doc = "Transmit Object Remote Monitoring\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rmm {
    #[doc = "0: Remote monitoring is disabled: Identifier, IDE bit, and DLC of message object n remain unchanged upon the reception of a matching Remote Frame."]
    Value1 = 0,
    #[doc = "1: Remote monitoring is enabled: Identifier, IDE bit, and DLC of a matching Remote Frame are copied to transmit object n in order to monitor incoming Remote Frames."]
    Value2 = 1,
}
impl From<Rmm> for bool {
    #[inline(always)]
    fn from(variant: Rmm) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RMM` reader - Transmit Object Remote Monitoring"]
pub type RmmR = crate::BitReader<Rmm>;
impl RmmR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rmm {
        match self.bits {
            false => Rmm::Value1,
            true => Rmm::Value2,
        }
    }
    #[doc = "Remote monitoring is disabled: Identifier, IDE bit, and DLC of message object n remain unchanged upon the reception of a matching Remote Frame."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Rmm::Value1
    }
    #[doc = "Remote monitoring is enabled: Identifier, IDE bit, and DLC of a matching Remote Frame are copied to transmit object n in order to monitor incoming Remote Frames."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Rmm::Value2
    }
}
#[doc = "Field `RMM` writer - Transmit Object Remote Monitoring"]
pub type RmmW<'a, REG> = crate::BitWriter<'a, REG, Rmm>;
impl<'a, REG> RmmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Remote monitoring is disabled: Identifier, IDE bit, and DLC of message object n remain unchanged upon the reception of a matching Remote Frame."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Rmm::Value1)
    }
    #[doc = "Remote monitoring is enabled: Identifier, IDE bit, and DLC of a matching Remote Frame are copied to transmit object n in order to monitor incoming Remote Frames."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Rmm::Value2)
    }
}
#[doc = "Field `SDT` reader - Single Data Transfer"]
pub type SdtR = crate::BitReader;
#[doc = "Field `SDT` writer - Single Data Transfer"]
pub type SdtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STT` reader - Single Transmit Trial"]
pub type SttR = crate::BitReader;
#[doc = "Field `STT` writer - Single Transmit Trial"]
pub type SttW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DLC` reader - Data Length Code"]
pub type DlcR = crate::FieldReader;
#[doc = "Field `DLC` writer - Data Length Code"]
pub type DlcW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - Message Mode Control"]
    #[inline(always)]
    pub fn mmc(&self) -> MmcR {
        MmcR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - Receive Time-Out Enable"]
    #[inline(always)]
    pub fn rxtoe(&self) -> RxtoeR {
        RxtoeR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - Gateway Data Frame Send"]
    #[inline(always)]
    pub fn gdfs(&self) -> GdfsR {
        GdfsR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Identifier Copy"]
    #[inline(always)]
    pub fn idc(&self) -> IdcR {
        IdcR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Data Length Code Copy"]
    #[inline(always)]
    pub fn dlcc(&self) -> DlccR {
        DlccR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Data Copy"]
    #[inline(always)]
    pub fn datc(&self) -> DatcR {
        DatcR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 16 - Receive Interrupt Enable"]
    #[inline(always)]
    pub fn rxie(&self) -> RxieR {
        RxieR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Transmit Interrupt Enable"]
    #[inline(always)]
    pub fn txie(&self) -> TxieR {
        TxieR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Overflow Interrupt Enable"]
    #[inline(always)]
    pub fn ovie(&self) -> OvieR {
        OvieR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 20 - Foreign Remote Request Enable"]
    #[inline(always)]
    pub fn frren(&self) -> FrrenR {
        FrrenR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Transmit Object Remote Monitoring"]
    #[inline(always)]
    pub fn rmm(&self) -> RmmR {
        RmmR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Single Data Transfer"]
    #[inline(always)]
    pub fn sdt(&self) -> SdtR {
        SdtR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Single Transmit Trial"]
    #[inline(always)]
    pub fn stt(&self) -> SttR {
        SttR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:27 - Data Length Code"]
    #[inline(always)]
    pub fn dlc(&self) -> DlcR {
        DlcR::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Message Mode Control"]
    #[inline(always)]
    #[must_use]
    pub fn mmc(&mut self) -> MmcW<MofcrSpec> {
        MmcW::new(self, 0)
    }
    #[doc = "Bit 4 - Receive Time-Out Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rxtoe(&mut self) -> RxtoeW<MofcrSpec> {
        RxtoeW::new(self, 4)
    }
    #[doc = "Bit 8 - Gateway Data Frame Send"]
    #[inline(always)]
    #[must_use]
    pub fn gdfs(&mut self) -> GdfsW<MofcrSpec> {
        GdfsW::new(self, 8)
    }
    #[doc = "Bit 9 - Identifier Copy"]
    #[inline(always)]
    #[must_use]
    pub fn idc(&mut self) -> IdcW<MofcrSpec> {
        IdcW::new(self, 9)
    }
    #[doc = "Bit 10 - Data Length Code Copy"]
    #[inline(always)]
    #[must_use]
    pub fn dlcc(&mut self) -> DlccW<MofcrSpec> {
        DlccW::new(self, 10)
    }
    #[doc = "Bit 11 - Data Copy"]
    #[inline(always)]
    #[must_use]
    pub fn datc(&mut self) -> DatcW<MofcrSpec> {
        DatcW::new(self, 11)
    }
    #[doc = "Bit 16 - Receive Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rxie(&mut self) -> RxieW<MofcrSpec> {
        RxieW::new(self, 16)
    }
    #[doc = "Bit 17 - Transmit Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn txie(&mut self) -> TxieW<MofcrSpec> {
        TxieW::new(self, 17)
    }
    #[doc = "Bit 18 - Overflow Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ovie(&mut self) -> OvieW<MofcrSpec> {
        OvieW::new(self, 18)
    }
    #[doc = "Bit 20 - Foreign Remote Request Enable"]
    #[inline(always)]
    #[must_use]
    pub fn frren(&mut self) -> FrrenW<MofcrSpec> {
        FrrenW::new(self, 20)
    }
    #[doc = "Bit 21 - Transmit Object Remote Monitoring"]
    #[inline(always)]
    #[must_use]
    pub fn rmm(&mut self) -> RmmW<MofcrSpec> {
        RmmW::new(self, 21)
    }
    #[doc = "Bit 22 - Single Data Transfer"]
    #[inline(always)]
    #[must_use]
    pub fn sdt(&mut self) -> SdtW<MofcrSpec> {
        SdtW::new(self, 22)
    }
    #[doc = "Bit 23 - Single Transmit Trial"]
    #[inline(always)]
    #[must_use]
    pub fn stt(&mut self) -> SttW<MofcrSpec> {
        SttW::new(self, 23)
    }
    #[doc = "Bits 24:27 - Data Length Code"]
    #[inline(always)]
    #[must_use]
    pub fn dlc(&mut self) -> DlcW<MofcrSpec> {
        DlcW::new(self, 24)
    }
}
#[doc = "Message Object Function Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mofcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mofcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MofcrSpec;
impl crate::RegisterSpec for MofcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mofcr::R`](R) reader structure"]
impl crate::Readable for MofcrSpec {}
#[doc = "`write(|w| ..)` method takes [`mofcr::W`](W) writer structure"]
impl crate::Writable for MofcrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MOFCR to value 0"]
impl crate::Resettable for MofcrSpec {
    const RESET_VALUE: u32 = 0;
}
