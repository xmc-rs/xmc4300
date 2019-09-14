#[doc = "Reader of register MOFCR"]
pub type R = crate::R<u32, super::MOFCR>;
#[doc = "Writer for register MOFCR"]
pub type W = crate::W<u32, super::MOFCR>;
#[doc = "Register MOFCR `reset()`'s with value 0"]
impl crate::ResetValue for super::MOFCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Message Mode Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MMC_A {
    #[doc = "0: Standard Message Object"]
    VALUE1,
    #[doc = "1: Receive FIFO Base Object"]
    VALUE2,
    #[doc = "2: Transmit FIFO Base Object"]
    VALUE3,
    #[doc = "3: Transmit FIFO Slave Object"]
    VALUE4,
    #[doc = "4: Gateway Source Object"]
    VALUE5,
}
impl From<MMC_A> for u8 {
    #[inline(always)]
    fn from(variant: MMC_A) -> Self {
        match variant {
            MMC_A::VALUE1 => 0,
            MMC_A::VALUE2 => 1,
            MMC_A::VALUE3 => 2,
            MMC_A::VALUE4 => 3,
            MMC_A::VALUE5 => 4,
        }
    }
}
#[doc = "Reader of field `MMC`"]
pub type MMC_R = crate::R<u8, MMC_A>;
impl MMC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, MMC_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(MMC_A::VALUE1),
            1 => Val(MMC_A::VALUE2),
            2 => Val(MMC_A::VALUE3),
            3 => Val(MMC_A::VALUE4),
            4 => Val(MMC_A::VALUE5),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == MMC_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == MMC_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == MMC_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == MMC_A::VALUE4
    }
    #[doc = "Checks if the value of the field is `VALUE5`"]
    #[inline(always)]
    pub fn is_value5(&self) -> bool {
        *self == MMC_A::VALUE5
    }
}
#[doc = "Write proxy for field `MMC`"]
pub struct MMC_W<'a> {
    w: &'a mut W,
}
impl<'a> MMC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MMC_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Standard Message Object"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(MMC_A::VALUE1)
    }
    #[doc = "Receive FIFO Base Object"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(MMC_A::VALUE2)
    }
    #[doc = "Transmit FIFO Base Object"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(MMC_A::VALUE3)
    }
    #[doc = "Transmit FIFO Slave Object"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(MMC_A::VALUE4)
    }
    #[doc = "Gateway Source Object"]
    #[inline(always)]
    pub fn value5(self) -> &'a mut W {
        self.variant(MMC_A::VALUE5)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "Receive Time-Out Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXTOE_A {
    #[doc = "0: Message does not take part in receive time-out check"]
    VALUE1,
    #[doc = "1: Message takes part in receive time-out check"]
    VALUE2,
}
impl From<RXTOE_A> for bool {
    #[inline(always)]
    fn from(variant: RXTOE_A) -> Self {
        match variant {
            RXTOE_A::VALUE1 => false,
            RXTOE_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `RXTOE`"]
pub type RXTOE_R = crate::R<bool, RXTOE_A>;
impl RXTOE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXTOE_A {
        match self.bits {
            false => RXTOE_A::VALUE1,
            true => RXTOE_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == RXTOE_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == RXTOE_A::VALUE2
    }
}
#[doc = "Write proxy for field `RXTOE`"]
pub struct RXTOE_W<'a> {
    w: &'a mut W,
}
impl<'a> RXTOE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RXTOE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Message does not take part in receive time-out check"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(RXTOE_A::VALUE1)
    }
    #[doc = "Message takes part in receive time-out check"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(RXTOE_A::VALUE2)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Gateway Data Frame Send\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GDFS_A {
    #[doc = "0: TXRQ is unchanged in the destination object."]
    VALUE1,
    #[doc = "1: TXRQ is set in the gateway destination object after the internal transfer from the gateway source to the gateway destination object."]
    VALUE2,
}
impl From<GDFS_A> for bool {
    #[inline(always)]
    fn from(variant: GDFS_A) -> Self {
        match variant {
            GDFS_A::VALUE1 => false,
            GDFS_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `GDFS`"]
pub type GDFS_R = crate::R<bool, GDFS_A>;
impl GDFS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GDFS_A {
        match self.bits {
            false => GDFS_A::VALUE1,
            true => GDFS_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == GDFS_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == GDFS_A::VALUE2
    }
}
#[doc = "Write proxy for field `GDFS`"]
pub struct GDFS_W<'a> {
    w: &'a mut W,
}
impl<'a> GDFS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GDFS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "TXRQ is unchanged in the destination object."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(GDFS_A::VALUE1)
    }
    #[doc = "TXRQ is set in the gateway destination object after the internal transfer from the gateway source to the gateway destination object."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(GDFS_A::VALUE2)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Identifier Copy\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IDC_A {
    #[doc = "0: The identifier of the gateway source object is not copied."]
    VALUE1,
    #[doc = "1: The identifier of the gateway source object (after storing the received frame in the source) is copied to the gateway destination object."]
    VALUE2,
}
impl From<IDC_A> for bool {
    #[inline(always)]
    fn from(variant: IDC_A) -> Self {
        match variant {
            IDC_A::VALUE1 => false,
            IDC_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `IDC`"]
pub type IDC_R = crate::R<bool, IDC_A>;
impl IDC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IDC_A {
        match self.bits {
            false => IDC_A::VALUE1,
            true => IDC_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == IDC_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == IDC_A::VALUE2
    }
}
#[doc = "Write proxy for field `IDC`"]
pub struct IDC_W<'a> {
    w: &'a mut W,
}
impl<'a> IDC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IDC_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The identifier of the gateway source object is not copied."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(IDC_A::VALUE1)
    }
    #[doc = "The identifier of the gateway source object (after storing the received frame in the source) is copied to the gateway destination object."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(IDC_A::VALUE2)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Data Length Code Copy\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DLCC_A {
    #[doc = "0: Data length code is not copied."]
    VALUE1,
    #[doc = "1: Data length code of the gateway source object (after storing the received frame in the source) is copied to the gateway destination object."]
    VALUE2,
}
impl From<DLCC_A> for bool {
    #[inline(always)]
    fn from(variant: DLCC_A) -> Self {
        match variant {
            DLCC_A::VALUE1 => false,
            DLCC_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `DLCC`"]
pub type DLCC_R = crate::R<bool, DLCC_A>;
impl DLCC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DLCC_A {
        match self.bits {
            false => DLCC_A::VALUE1,
            true => DLCC_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == DLCC_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == DLCC_A::VALUE2
    }
}
#[doc = "Write proxy for field `DLCC`"]
pub struct DLCC_W<'a> {
    w: &'a mut W,
}
impl<'a> DLCC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DLCC_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Data length code is not copied."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(DLCC_A::VALUE1)
    }
    #[doc = "Data length code of the gateway source object (after storing the received frame in the source) is copied to the gateway destination object."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(DLCC_A::VALUE2)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Data Copy\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DATC_A {
    #[doc = "0: Data fields are not copied."]
    VALUE1,
    #[doc = "1: Data fields in registers MODATALn and MODATAHn of the gateway source object (after storing the received frame in the source) are copied to the gateway destination."]
    VALUE2,
}
impl From<DATC_A> for bool {
    #[inline(always)]
    fn from(variant: DATC_A) -> Self {
        match variant {
            DATC_A::VALUE1 => false,
            DATC_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `DATC`"]
pub type DATC_R = crate::R<bool, DATC_A>;
impl DATC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DATC_A {
        match self.bits {
            false => DATC_A::VALUE1,
            true => DATC_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == DATC_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == DATC_A::VALUE2
    }
}
#[doc = "Write proxy for field `DATC`"]
pub struct DATC_W<'a> {
    w: &'a mut W,
}
impl<'a> DATC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DATC_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Data fields are not copied."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(DATC_A::VALUE1)
    }
    #[doc = "Data fields in registers MODATALn and MODATAHn of the gateway source object (after storing the received frame in the source) are copied to the gateway destination."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(DATC_A::VALUE2)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Receive Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXIE_A {
    #[doc = "0: Message receive interrupt is disabled."]
    VALUE1,
    #[doc = "1: Message receive interrupt is enabled."]
    VALUE2,
}
impl From<RXIE_A> for bool {
    #[inline(always)]
    fn from(variant: RXIE_A) -> Self {
        match variant {
            RXIE_A::VALUE1 => false,
            RXIE_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `RXIE`"]
pub type RXIE_R = crate::R<bool, RXIE_A>;
impl RXIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXIE_A {
        match self.bits {
            false => RXIE_A::VALUE1,
            true => RXIE_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == RXIE_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == RXIE_A::VALUE2
    }
}
#[doc = "Write proxy for field `RXIE`"]
pub struct RXIE_W<'a> {
    w: &'a mut W,
}
impl<'a> RXIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RXIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Message receive interrupt is disabled."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(RXIE_A::VALUE1)
    }
    #[doc = "Message receive interrupt is enabled."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(RXIE_A::VALUE2)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Transmit Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXIE_A {
    #[doc = "0: Message transmit interrupt is disabled."]
    VALUE1,
    #[doc = "1: Message transmit interrupt is enabled."]
    VALUE2,
}
impl From<TXIE_A> for bool {
    #[inline(always)]
    fn from(variant: TXIE_A) -> Self {
        match variant {
            TXIE_A::VALUE1 => false,
            TXIE_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `TXIE`"]
pub type TXIE_R = crate::R<bool, TXIE_A>;
impl TXIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXIE_A {
        match self.bits {
            false => TXIE_A::VALUE1,
            true => TXIE_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == TXIE_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == TXIE_A::VALUE2
    }
}
#[doc = "Write proxy for field `TXIE`"]
pub struct TXIE_W<'a> {
    w: &'a mut W,
}
impl<'a> TXIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TXIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Message transmit interrupt is disabled."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(TXIE_A::VALUE1)
    }
    #[doc = "Message transmit interrupt is enabled."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(TXIE_A::VALUE2)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "Overflow Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OVIE_A {
    #[doc = "0: FIFO full interrupt is disabled."]
    VALUE1,
    #[doc = "1: FIFO full interrupt is enabled."]
    VALUE2,
}
impl From<OVIE_A> for bool {
    #[inline(always)]
    fn from(variant: OVIE_A) -> Self {
        match variant {
            OVIE_A::VALUE1 => false,
            OVIE_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `OVIE`"]
pub type OVIE_R = crate::R<bool, OVIE_A>;
impl OVIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OVIE_A {
        match self.bits {
            false => OVIE_A::VALUE1,
            true => OVIE_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == OVIE_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == OVIE_A::VALUE2
    }
}
#[doc = "Write proxy for field `OVIE`"]
pub struct OVIE_W<'a> {
    w: &'a mut W,
}
impl<'a> OVIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OVIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "FIFO full interrupt is disabled."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(OVIE_A::VALUE1)
    }
    #[doc = "FIFO full interrupt is enabled."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(OVIE_A::VALUE2)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = "Foreign Remote Request Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FRREN_A {
    #[doc = "0: TXRQ of message object n is set on reception of a matching Remote Frame."]
    VALUE1,
    #[doc = "1: TXRQ of the message object referenced by the pointer CUR is set on reception of a matching Remote Frame."]
    VALUE2,
}
impl From<FRREN_A> for bool {
    #[inline(always)]
    fn from(variant: FRREN_A) -> Self {
        match variant {
            FRREN_A::VALUE1 => false,
            FRREN_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `FRREN`"]
pub type FRREN_R = crate::R<bool, FRREN_A>;
impl FRREN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FRREN_A {
        match self.bits {
            false => FRREN_A::VALUE1,
            true => FRREN_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == FRREN_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == FRREN_A::VALUE2
    }
}
#[doc = "Write proxy for field `FRREN`"]
pub struct FRREN_W<'a> {
    w: &'a mut W,
}
impl<'a> FRREN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FRREN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "TXRQ of message object n is set on reception of a matching Remote Frame."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(FRREN_A::VALUE1)
    }
    #[doc = "TXRQ of the message object referenced by the pointer CUR is set on reception of a matching Remote Frame."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(FRREN_A::VALUE2)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
#[doc = "Transmit Object Remote Monitoring\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RMM_A {
    #[doc = "0: Remote monitoring is disabled: Identifier, IDE bit, and DLC of message object n remain unchanged upon the reception of a matching Remote Frame."]
    VALUE1,
    #[doc = "1: Remote monitoring is enabled: Identifier, IDE bit, and DLC of a matching Remote Frame are copied to transmit object n in order to monitor incoming Remote Frames."]
    VALUE2,
}
impl From<RMM_A> for bool {
    #[inline(always)]
    fn from(variant: RMM_A) -> Self {
        match variant {
            RMM_A::VALUE1 => false,
            RMM_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `RMM`"]
pub type RMM_R = crate::R<bool, RMM_A>;
impl RMM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RMM_A {
        match self.bits {
            false => RMM_A::VALUE1,
            true => RMM_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == RMM_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == RMM_A::VALUE2
    }
}
#[doc = "Write proxy for field `RMM`"]
pub struct RMM_W<'a> {
    w: &'a mut W,
}
impl<'a> RMM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RMM_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Remote monitoring is disabled: Identifier, IDE bit, and DLC of message object n remain unchanged upon the reception of a matching Remote Frame."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(RMM_A::VALUE1)
    }
    #[doc = "Remote monitoring is enabled: Identifier, IDE bit, and DLC of a matching Remote Frame are copied to transmit object n in order to monitor incoming Remote Frames."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(RMM_A::VALUE2)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
        self.w
    }
}
#[doc = "Reader of field `SDT`"]
pub type SDT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SDT`"]
pub struct SDT_W<'a> {
    w: &'a mut W,
}
impl<'a> SDT_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
        self.w
    }
}
#[doc = "Reader of field `STT`"]
pub type STT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `STT`"]
pub struct STT_W<'a> {
    w: &'a mut W,
}
impl<'a> STT_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
        self.w
    }
}
#[doc = "Reader of field `DLC`"]
pub type DLC_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DLC`"]
pub struct DLC_W<'a> {
    w: &'a mut W,
}
impl<'a> DLC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | (((value as u32) & 0x0f) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Message Mode Control"]
    #[inline(always)]
    pub fn mmc(&self) -> MMC_R {
        MMC_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - Receive Time-Out Enable"]
    #[inline(always)]
    pub fn rxtoe(&self) -> RXTOE_R {
        RXTOE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Gateway Data Frame Send"]
    #[inline(always)]
    pub fn gdfs(&self) -> GDFS_R {
        GDFS_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Identifier Copy"]
    #[inline(always)]
    pub fn idc(&self) -> IDC_R {
        IDC_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Data Length Code Copy"]
    #[inline(always)]
    pub fn dlcc(&self) -> DLCC_R {
        DLCC_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Data Copy"]
    #[inline(always)]
    pub fn datc(&self) -> DATC_R {
        DATC_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Receive Interrupt Enable"]
    #[inline(always)]
    pub fn rxie(&self) -> RXIE_R {
        RXIE_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Transmit Interrupt Enable"]
    #[inline(always)]
    pub fn txie(&self) -> TXIE_R {
        TXIE_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Overflow Interrupt Enable"]
    #[inline(always)]
    pub fn ovie(&self) -> OVIE_R {
        OVIE_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Foreign Remote Request Enable"]
    #[inline(always)]
    pub fn frren(&self) -> FRREN_R {
        FRREN_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Transmit Object Remote Monitoring"]
    #[inline(always)]
    pub fn rmm(&self) -> RMM_R {
        RMM_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Single Data Transfer"]
    #[inline(always)]
    pub fn sdt(&self) -> SDT_R {
        SDT_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Single Transmit Trial"]
    #[inline(always)]
    pub fn stt(&self) -> STT_R {
        STT_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bits 24:27 - Data Length Code"]
    #[inline(always)]
    pub fn dlc(&self) -> DLC_R {
        DLC_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Message Mode Control"]
    #[inline(always)]
    pub fn mmc(&mut self) -> MMC_W {
        MMC_W { w: self }
    }
    #[doc = "Bit 4 - Receive Time-Out Enable"]
    #[inline(always)]
    pub fn rxtoe(&mut self) -> RXTOE_W {
        RXTOE_W { w: self }
    }
    #[doc = "Bit 8 - Gateway Data Frame Send"]
    #[inline(always)]
    pub fn gdfs(&mut self) -> GDFS_W {
        GDFS_W { w: self }
    }
    #[doc = "Bit 9 - Identifier Copy"]
    #[inline(always)]
    pub fn idc(&mut self) -> IDC_W {
        IDC_W { w: self }
    }
    #[doc = "Bit 10 - Data Length Code Copy"]
    #[inline(always)]
    pub fn dlcc(&mut self) -> DLCC_W {
        DLCC_W { w: self }
    }
    #[doc = "Bit 11 - Data Copy"]
    #[inline(always)]
    pub fn datc(&mut self) -> DATC_W {
        DATC_W { w: self }
    }
    #[doc = "Bit 16 - Receive Interrupt Enable"]
    #[inline(always)]
    pub fn rxie(&mut self) -> RXIE_W {
        RXIE_W { w: self }
    }
    #[doc = "Bit 17 - Transmit Interrupt Enable"]
    #[inline(always)]
    pub fn txie(&mut self) -> TXIE_W {
        TXIE_W { w: self }
    }
    #[doc = "Bit 18 - Overflow Interrupt Enable"]
    #[inline(always)]
    pub fn ovie(&mut self) -> OVIE_W {
        OVIE_W { w: self }
    }
    #[doc = "Bit 20 - Foreign Remote Request Enable"]
    #[inline(always)]
    pub fn frren(&mut self) -> FRREN_W {
        FRREN_W { w: self }
    }
    #[doc = "Bit 21 - Transmit Object Remote Monitoring"]
    #[inline(always)]
    pub fn rmm(&mut self) -> RMM_W {
        RMM_W { w: self }
    }
    #[doc = "Bit 22 - Single Data Transfer"]
    #[inline(always)]
    pub fn sdt(&mut self) -> SDT_W {
        SDT_W { w: self }
    }
    #[doc = "Bit 23 - Single Transmit Trial"]
    #[inline(always)]
    pub fn stt(&mut self) -> STT_W {
        STT_W { w: self }
    }
    #[doc = "Bits 24:27 - Data Length Code"]
    #[inline(always)]
    pub fn dlc(&mut self) -> DLC_W {
        DLC_W { w: self }
    }
}
