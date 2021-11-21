#[doc = "Register `MOFCR` reader"]
pub struct R(crate::R<MOFCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MOFCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MOFCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MOFCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MOFCR` writer"]
pub struct W(crate::W<MOFCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MOFCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<MOFCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MOFCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Message Mode Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MMC_A {
    #[doc = "0: Standard Message Object"]
    VALUE1 = 0,
    #[doc = "1: Receive FIFO Base Object"]
    VALUE2 = 1,
    #[doc = "2: Transmit FIFO Base Object"]
    VALUE3 = 2,
    #[doc = "3: Transmit FIFO Slave Object"]
    VALUE4 = 3,
    #[doc = "4: Gateway Source Object"]
    VALUE5 = 4,
}
impl From<MMC_A> for u8 {
    #[inline(always)]
    fn from(variant: MMC_A) -> Self {
        variant as _
    }
}
#[doc = "Field `MMC` reader - Message Mode Control"]
pub struct MMC_R(crate::FieldReader<u8, MMC_A>);
impl MMC_R {
    pub(crate) fn new(bits: u8) -> Self {
        MMC_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<MMC_A> {
        match self.bits {
            0 => Some(MMC_A::VALUE1),
            1 => Some(MMC_A::VALUE2),
            2 => Some(MMC_A::VALUE3),
            3 => Some(MMC_A::VALUE4),
            4 => Some(MMC_A::VALUE5),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == MMC_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == MMC_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        **self == MMC_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        **self == MMC_A::VALUE4
    }
    #[doc = "Checks if the value of the field is `VALUE5`"]
    #[inline(always)]
    pub fn is_value5(&self) -> bool {
        **self == MMC_A::VALUE5
    }
}
impl core::ops::Deref for MMC_R {
    type Target = crate::FieldReader<u8, MMC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MMC` writer - Message Mode Control"]
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
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
#[doc = "Receive Time-Out Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXTOE_A {
    #[doc = "0: Message does not take part in receive time-out check"]
    VALUE1 = 0,
    #[doc = "1: Message takes part in receive time-out check"]
    VALUE2 = 1,
}
impl From<RXTOE_A> for bool {
    #[inline(always)]
    fn from(variant: RXTOE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXTOE` reader - Receive Time-Out Enable"]
pub struct RXTOE_R(crate::FieldReader<bool, RXTOE_A>);
impl RXTOE_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXTOE_R(crate::FieldReader::new(bits))
    }
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
        **self == RXTOE_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == RXTOE_A::VALUE2
    }
}
impl core::ops::Deref for RXTOE_R {
    type Target = crate::FieldReader<bool, RXTOE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXTOE` writer - Receive Time-Out Enable"]
pub struct RXTOE_W<'a> {
    w: &'a mut W,
}
impl<'a> RXTOE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RXTOE_A) -> &'a mut W {
        self.bit(variant.into())
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "Gateway Data Frame Send\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GDFS_A {
    #[doc = "0: TXRQ is unchanged in the destination object."]
    VALUE1 = 0,
    #[doc = "1: TXRQ is set in the gateway destination object after the internal transfer from the gateway source to the gateway destination object."]
    VALUE2 = 1,
}
impl From<GDFS_A> for bool {
    #[inline(always)]
    fn from(variant: GDFS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GDFS` reader - Gateway Data Frame Send"]
pub struct GDFS_R(crate::FieldReader<bool, GDFS_A>);
impl GDFS_R {
    pub(crate) fn new(bits: bool) -> Self {
        GDFS_R(crate::FieldReader::new(bits))
    }
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
        **self == GDFS_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == GDFS_A::VALUE2
    }
}
impl core::ops::Deref for GDFS_R {
    type Target = crate::FieldReader<bool, GDFS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GDFS` writer - Gateway Data Frame Send"]
pub struct GDFS_W<'a> {
    w: &'a mut W,
}
impl<'a> GDFS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GDFS_A) -> &'a mut W {
        self.bit(variant.into())
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
#[doc = "Identifier Copy\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IDC_A {
    #[doc = "0: The identifier of the gateway source object is not copied."]
    VALUE1 = 0,
    #[doc = "1: The identifier of the gateway source object (after storing the received frame in the source) is copied to the gateway destination object."]
    VALUE2 = 1,
}
impl From<IDC_A> for bool {
    #[inline(always)]
    fn from(variant: IDC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IDC` reader - Identifier Copy"]
pub struct IDC_R(crate::FieldReader<bool, IDC_A>);
impl IDC_R {
    pub(crate) fn new(bits: bool) -> Self {
        IDC_R(crate::FieldReader::new(bits))
    }
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
        **self == IDC_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == IDC_A::VALUE2
    }
}
impl core::ops::Deref for IDC_R {
    type Target = crate::FieldReader<bool, IDC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IDC` writer - Identifier Copy"]
pub struct IDC_W<'a> {
    w: &'a mut W,
}
impl<'a> IDC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IDC_A) -> &'a mut W {
        self.bit(variant.into())
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
#[doc = "Data Length Code Copy\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DLCC_A {
    #[doc = "0: Data length code is not copied."]
    VALUE1 = 0,
    #[doc = "1: Data length code of the gateway source object (after storing the received frame in the source) is copied to the gateway destination object."]
    VALUE2 = 1,
}
impl From<DLCC_A> for bool {
    #[inline(always)]
    fn from(variant: DLCC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DLCC` reader - Data Length Code Copy"]
pub struct DLCC_R(crate::FieldReader<bool, DLCC_A>);
impl DLCC_R {
    pub(crate) fn new(bits: bool) -> Self {
        DLCC_R(crate::FieldReader::new(bits))
    }
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
        **self == DLCC_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == DLCC_A::VALUE2
    }
}
impl core::ops::Deref for DLCC_R {
    type Target = crate::FieldReader<bool, DLCC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DLCC` writer - Data Length Code Copy"]
pub struct DLCC_W<'a> {
    w: &'a mut W,
}
impl<'a> DLCC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DLCC_A) -> &'a mut W {
        self.bit(variant.into())
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
        self.w
    }
}
#[doc = "Data Copy\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DATC_A {
    #[doc = "0: Data fields are not copied."]
    VALUE1 = 0,
    #[doc = "1: Data fields in registers MODATALn and MODATAHn of the gateway source object (after storing the received frame in the source) are copied to the gateway destination."]
    VALUE2 = 1,
}
impl From<DATC_A> for bool {
    #[inline(always)]
    fn from(variant: DATC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DATC` reader - Data Copy"]
pub struct DATC_R(crate::FieldReader<bool, DATC_A>);
impl DATC_R {
    pub(crate) fn new(bits: bool) -> Self {
        DATC_R(crate::FieldReader::new(bits))
    }
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
        **self == DATC_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == DATC_A::VALUE2
    }
}
impl core::ops::Deref for DATC_R {
    type Target = crate::FieldReader<bool, DATC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DATC` writer - Data Copy"]
pub struct DATC_W<'a> {
    w: &'a mut W,
}
impl<'a> DATC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DATC_A) -> &'a mut W {
        self.bit(variant.into())
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u32 & 0x01) << 11);
        self.w
    }
}
#[doc = "Receive Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXIE_A {
    #[doc = "0: Message receive interrupt is disabled."]
    VALUE1 = 0,
    #[doc = "1: Message receive interrupt is enabled."]
    VALUE2 = 1,
}
impl From<RXIE_A> for bool {
    #[inline(always)]
    fn from(variant: RXIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXIE` reader - Receive Interrupt Enable"]
pub struct RXIE_R(crate::FieldReader<bool, RXIE_A>);
impl RXIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXIE_R(crate::FieldReader::new(bits))
    }
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
        **self == RXIE_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == RXIE_A::VALUE2
    }
}
impl core::ops::Deref for RXIE_R {
    type Target = crate::FieldReader<bool, RXIE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXIE` writer - Receive Interrupt Enable"]
pub struct RXIE_W<'a> {
    w: &'a mut W,
}
impl<'a> RXIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RXIE_A) -> &'a mut W {
        self.bit(variant.into())
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
#[doc = "Transmit Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXIE_A {
    #[doc = "0: Message transmit interrupt is disabled."]
    VALUE1 = 0,
    #[doc = "1: Message transmit interrupt is enabled."]
    VALUE2 = 1,
}
impl From<TXIE_A> for bool {
    #[inline(always)]
    fn from(variant: TXIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXIE` reader - Transmit Interrupt Enable"]
pub struct TXIE_R(crate::FieldReader<bool, TXIE_A>);
impl TXIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXIE_R(crate::FieldReader::new(bits))
    }
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
        **self == TXIE_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == TXIE_A::VALUE2
    }
}
impl core::ops::Deref for TXIE_R {
    type Target = crate::FieldReader<bool, TXIE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXIE` writer - Transmit Interrupt Enable"]
pub struct TXIE_W<'a> {
    w: &'a mut W,
}
impl<'a> TXIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TXIE_A) -> &'a mut W {
        self.bit(variant.into())
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | ((value as u32 & 0x01) << 17);
        self.w
    }
}
#[doc = "Overflow Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OVIE_A {
    #[doc = "0: FIFO full interrupt is disabled."]
    VALUE1 = 0,
    #[doc = "1: FIFO full interrupt is enabled."]
    VALUE2 = 1,
}
impl From<OVIE_A> for bool {
    #[inline(always)]
    fn from(variant: OVIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OVIE` reader - Overflow Interrupt Enable"]
pub struct OVIE_R(crate::FieldReader<bool, OVIE_A>);
impl OVIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        OVIE_R(crate::FieldReader::new(bits))
    }
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
        **self == OVIE_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == OVIE_A::VALUE2
    }
}
impl core::ops::Deref for OVIE_R {
    type Target = crate::FieldReader<bool, OVIE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OVIE` writer - Overflow Interrupt Enable"]
pub struct OVIE_W<'a> {
    w: &'a mut W,
}
impl<'a> OVIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OVIE_A) -> &'a mut W {
        self.bit(variant.into())
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | ((value as u32 & 0x01) << 18);
        self.w
    }
}
#[doc = "Foreign Remote Request Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FRREN_A {
    #[doc = "0: TXRQ of message object n is set on reception of a matching Remote Frame."]
    VALUE1 = 0,
    #[doc = "1: TXRQ of the message object referenced by the pointer CUR is set on reception of a matching Remote Frame."]
    VALUE2 = 1,
}
impl From<FRREN_A> for bool {
    #[inline(always)]
    fn from(variant: FRREN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FRREN` reader - Foreign Remote Request Enable"]
pub struct FRREN_R(crate::FieldReader<bool, FRREN_A>);
impl FRREN_R {
    pub(crate) fn new(bits: bool) -> Self {
        FRREN_R(crate::FieldReader::new(bits))
    }
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
        **self == FRREN_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == FRREN_A::VALUE2
    }
}
impl core::ops::Deref for FRREN_R {
    type Target = crate::FieldReader<bool, FRREN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FRREN` writer - Foreign Remote Request Enable"]
pub struct FRREN_W<'a> {
    w: &'a mut W,
}
impl<'a> FRREN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FRREN_A) -> &'a mut W {
        self.bit(variant.into())
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | ((value as u32 & 0x01) << 20);
        self.w
    }
}
#[doc = "Transmit Object Remote Monitoring\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RMM_A {
    #[doc = "0: Remote monitoring is disabled: Identifier, IDE bit, and DLC of message object n remain unchanged upon the reception of a matching Remote Frame."]
    VALUE1 = 0,
    #[doc = "1: Remote monitoring is enabled: Identifier, IDE bit, and DLC of a matching Remote Frame are copied to transmit object n in order to monitor incoming Remote Frames."]
    VALUE2 = 1,
}
impl From<RMM_A> for bool {
    #[inline(always)]
    fn from(variant: RMM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RMM` reader - Transmit Object Remote Monitoring"]
pub struct RMM_R(crate::FieldReader<bool, RMM_A>);
impl RMM_R {
    pub(crate) fn new(bits: bool) -> Self {
        RMM_R(crate::FieldReader::new(bits))
    }
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
        **self == RMM_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == RMM_A::VALUE2
    }
}
impl core::ops::Deref for RMM_R {
    type Target = crate::FieldReader<bool, RMM_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RMM` writer - Transmit Object Remote Monitoring"]
pub struct RMM_W<'a> {
    w: &'a mut W,
}
impl<'a> RMM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RMM_A) -> &'a mut W {
        self.bit(variant.into())
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | ((value as u32 & 0x01) << 21);
        self.w
    }
}
#[doc = "Field `SDT` reader - Single Data Transfer"]
pub struct SDT_R(crate::FieldReader<bool, bool>);
impl SDT_R {
    pub(crate) fn new(bits: bool) -> Self {
        SDT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SDT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SDT` writer - Single Data Transfer"]
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | ((value as u32 & 0x01) << 22);
        self.w
    }
}
#[doc = "Field `STT` reader - Single Transmit Trial"]
pub struct STT_R(crate::FieldReader<bool, bool>);
impl STT_R {
    pub(crate) fn new(bits: bool) -> Self {
        STT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STT` writer - Single Transmit Trial"]
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | ((value as u32 & 0x01) << 23);
        self.w
    }
}
#[doc = "Field `DLC` reader - Data Length Code"]
pub struct DLC_R(crate::FieldReader<u8, u8>);
impl DLC_R {
    pub(crate) fn new(bits: u8) -> Self {
        DLC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DLC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DLC` writer - Data Length Code"]
pub struct DLC_W<'a> {
    w: &'a mut W,
}
impl<'a> DLC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | ((value as u32 & 0x0f) << 24);
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
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Message Object Function Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mofcr](index.html) module"]
pub struct MOFCR_SPEC;
impl crate::RegisterSpec for MOFCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mofcr::R](R) reader structure"]
impl crate::Readable for MOFCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mofcr::W](W) writer structure"]
impl crate::Writable for MOFCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MOFCR to value 0"]
impl crate::Resettable for MOFCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
