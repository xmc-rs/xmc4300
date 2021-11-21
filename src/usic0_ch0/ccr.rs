#[doc = "Register `CCR` reader"]
pub struct R(crate::R<CCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CCR` writer"]
pub struct W(crate::W<CCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CCR_SPEC>;
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
impl From<crate::W<CCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Operating Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MODE_A {
    #[doc = "0: The USIC channel is disabled. All protocol-related state machines are set to an idle state."]
    VALUE1 = 0,
    #[doc = "1: The SSC (SPI) protocol is selected."]
    VALUE2 = 1,
    #[doc = "2: The ASC (SCI, UART) protocol is selected."]
    VALUE3 = 2,
    #[doc = "3: The IIS protocol is selected."]
    VALUE4 = 3,
    #[doc = "4: The IIC protocol is selected."]
    VALUE5 = 4,
}
impl From<MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: MODE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `MODE` reader - Operating Mode"]
pub struct MODE_R(crate::FieldReader<u8, MODE_A>);
impl MODE_R {
    pub(crate) fn new(bits: u8) -> Self {
        MODE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<MODE_A> {
        match self.bits {
            0 => Some(MODE_A::VALUE1),
            1 => Some(MODE_A::VALUE2),
            2 => Some(MODE_A::VALUE3),
            3 => Some(MODE_A::VALUE4),
            4 => Some(MODE_A::VALUE5),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == MODE_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == MODE_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        **self == MODE_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        **self == MODE_A::VALUE4
    }
    #[doc = "Checks if the value of the field is `VALUE5`"]
    #[inline(always)]
    pub fn is_value5(&self) -> bool {
        **self == MODE_A::VALUE5
    }
}
impl core::ops::Deref for MODE_R {
    type Target = crate::FieldReader<u8, MODE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MODE` writer - Operating Mode"]
pub struct MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MODE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "The USIC channel is disabled. All protocol-related state machines are set to an idle state."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(MODE_A::VALUE1)
    }
    #[doc = "The SSC (SPI) protocol is selected."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(MODE_A::VALUE2)
    }
    #[doc = "The ASC (SCI, UART) protocol is selected."]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(MODE_A::VALUE3)
    }
    #[doc = "The IIS protocol is selected."]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(MODE_A::VALUE4)
    }
    #[doc = "The IIC protocol is selected."]
    #[inline(always)]
    pub fn value5(self) -> &'a mut W {
        self.variant(MODE_A::VALUE5)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
#[doc = "Hardware Port Control Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum HPCEN_A {
    #[doc = "0: The hardware port control is disabled."]
    VALUE1 = 0,
    #[doc = "1: The hardware port control is enabled for DX0 and DOUT0."]
    VALUE2 = 1,
    #[doc = "2: The hardware port control is enabled for DX3, DX0 and DOUT\\[1:0\\]."]
    VALUE3 = 2,
    #[doc = "3: The hardware port control is enabled for DX0, DX\\[5:3\\]
and DOUT\\[3:0\\]."]
    VALUE4 = 3,
}
impl From<HPCEN_A> for u8 {
    #[inline(always)]
    fn from(variant: HPCEN_A) -> Self {
        variant as _
    }
}
#[doc = "Field `HPCEN` reader - Hardware Port Control Enable"]
pub struct HPCEN_R(crate::FieldReader<u8, HPCEN_A>);
impl HPCEN_R {
    pub(crate) fn new(bits: u8) -> Self {
        HPCEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HPCEN_A {
        match self.bits {
            0 => HPCEN_A::VALUE1,
            1 => HPCEN_A::VALUE2,
            2 => HPCEN_A::VALUE3,
            3 => HPCEN_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == HPCEN_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == HPCEN_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        **self == HPCEN_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        **self == HPCEN_A::VALUE4
    }
}
impl core::ops::Deref for HPCEN_R {
    type Target = crate::FieldReader<u8, HPCEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HPCEN` writer - Hardware Port Control Enable"]
pub struct HPCEN_W<'a> {
    w: &'a mut W,
}
impl<'a> HPCEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HPCEN_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "The hardware port control is disabled."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(HPCEN_A::VALUE1)
    }
    #[doc = "The hardware port control is enabled for DX0 and DOUT0."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(HPCEN_A::VALUE2)
    }
    #[doc = "The hardware port control is enabled for DX3, DX0 and DOUT\\[1:0\\]."]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(HPCEN_A::VALUE3)
    }
    #[doc = "The hardware port control is enabled for DX0, DX\\[5:3\\]
and DOUT\\[3:0\\]."]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(HPCEN_A::VALUE4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | ((value as u32 & 0x03) << 6);
        self.w
    }
}
#[doc = "Parity Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PM_A {
    #[doc = "0: The parity generation is disabled."]
    VALUE1 = 0,
    #[doc = "2: Even parity is selected (parity bit = 1 on odd number of 1s in data, parity bit = 0 on even number of 1s in data)."]
    VALUE3 = 2,
    #[doc = "3: Odd parity is selected (parity bit = 0 on odd number of 1s in data, parity bit = 1 on even number of 1s in data)."]
    VALUE4 = 3,
}
impl From<PM_A> for u8 {
    #[inline(always)]
    fn from(variant: PM_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PM` reader - Parity Mode"]
pub struct PM_R(crate::FieldReader<u8, PM_A>);
impl PM_R {
    pub(crate) fn new(bits: u8) -> Self {
        PM_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PM_A> {
        match self.bits {
            0 => Some(PM_A::VALUE1),
            2 => Some(PM_A::VALUE3),
            3 => Some(PM_A::VALUE4),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == PM_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        **self == PM_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        **self == PM_A::VALUE4
    }
}
impl core::ops::Deref for PM_R {
    type Target = crate::FieldReader<u8, PM_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PM` writer - Parity Mode"]
pub struct PM_W<'a> {
    w: &'a mut W,
}
impl<'a> PM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PM_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "The parity generation is disabled."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(PM_A::VALUE1)
    }
    #[doc = "Even parity is selected (parity bit = 1 on odd number of 1s in data, parity bit = 0 on even number of 1s in data)."]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(PM_A::VALUE3)
    }
    #[doc = "Odd parity is selected (parity bit = 0 on odd number of 1s in data, parity bit = 1 on even number of 1s in data)."]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(PM_A::VALUE4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | ((value as u32 & 0x03) << 8);
        self.w
    }
}
#[doc = "Receiver Start Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RSIEN_A {
    #[doc = "0: The receiver start interrupt is disabled."]
    VALUE1 = 0,
    #[doc = "1: The receiver start interrupt is enabled. In case of a receiver start event, the service request output SRx indicated by INPR.TBINP is activated."]
    VALUE2 = 1,
}
impl From<RSIEN_A> for bool {
    #[inline(always)]
    fn from(variant: RSIEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RSIEN` reader - Receiver Start Interrupt Enable"]
pub struct RSIEN_R(crate::FieldReader<bool, RSIEN_A>);
impl RSIEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        RSIEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RSIEN_A {
        match self.bits {
            false => RSIEN_A::VALUE1,
            true => RSIEN_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == RSIEN_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == RSIEN_A::VALUE2
    }
}
impl core::ops::Deref for RSIEN_R {
    type Target = crate::FieldReader<bool, RSIEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RSIEN` writer - Receiver Start Interrupt Enable"]
pub struct RSIEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RSIEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RSIEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The receiver start interrupt is disabled."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(RSIEN_A::VALUE1)
    }
    #[doc = "The receiver start interrupt is enabled. In case of a receiver start event, the service request output SRx indicated by INPR.TBINP is activated."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(RSIEN_A::VALUE2)
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
#[doc = "Data Lost Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DLIEN_A {
    #[doc = "0: The data lost interrupt is disabled."]
    VALUE1 = 0,
    #[doc = "1: The data lost interrupt is enabled. In case of a data lost event, the service request output SRx indicated by INPR.PINP is activated."]
    VALUE2 = 1,
}
impl From<DLIEN_A> for bool {
    #[inline(always)]
    fn from(variant: DLIEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DLIEN` reader - Data Lost Interrupt Enable"]
pub struct DLIEN_R(crate::FieldReader<bool, DLIEN_A>);
impl DLIEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        DLIEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DLIEN_A {
        match self.bits {
            false => DLIEN_A::VALUE1,
            true => DLIEN_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == DLIEN_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == DLIEN_A::VALUE2
    }
}
impl core::ops::Deref for DLIEN_R {
    type Target = crate::FieldReader<bool, DLIEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DLIEN` writer - Data Lost Interrupt Enable"]
pub struct DLIEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DLIEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DLIEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The data lost interrupt is disabled."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(DLIEN_A::VALUE1)
    }
    #[doc = "The data lost interrupt is enabled. In case of a data lost event, the service request output SRx indicated by INPR.PINP is activated."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(DLIEN_A::VALUE2)
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
#[doc = "Transmit Shift Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TSIEN_A {
    #[doc = "0: The transmit shift interrupt is disabled."]
    VALUE1 = 0,
    #[doc = "1: The transmit shift interrupt is enabled. In case of a transmit shift interrupt event, the service request output SRx indicated by INPR.TSINP is activated."]
    VALUE2 = 1,
}
impl From<TSIEN_A> for bool {
    #[inline(always)]
    fn from(variant: TSIEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TSIEN` reader - Transmit Shift Interrupt Enable"]
pub struct TSIEN_R(crate::FieldReader<bool, TSIEN_A>);
impl TSIEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        TSIEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TSIEN_A {
        match self.bits {
            false => TSIEN_A::VALUE1,
            true => TSIEN_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == TSIEN_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == TSIEN_A::VALUE2
    }
}
impl core::ops::Deref for TSIEN_R {
    type Target = crate::FieldReader<bool, TSIEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TSIEN` writer - Transmit Shift Interrupt Enable"]
pub struct TSIEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TSIEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TSIEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The transmit shift interrupt is disabled."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(TSIEN_A::VALUE1)
    }
    #[doc = "The transmit shift interrupt is enabled. In case of a transmit shift interrupt event, the service request output SRx indicated by INPR.TSINP is activated."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(TSIEN_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u32 & 0x01) << 12);
        self.w
    }
}
#[doc = "Transmit Buffer Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TBIEN_A {
    #[doc = "0: The transmit buffer interrupt is disabled."]
    VALUE1 = 0,
    #[doc = "1: The transmit buffer interrupt is enabled. In case of a transmit buffer event, the service request output SRx indicated by INPR.TBINP is activated."]
    VALUE2 = 1,
}
impl From<TBIEN_A> for bool {
    #[inline(always)]
    fn from(variant: TBIEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TBIEN` reader - Transmit Buffer Interrupt Enable"]
pub struct TBIEN_R(crate::FieldReader<bool, TBIEN_A>);
impl TBIEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        TBIEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TBIEN_A {
        match self.bits {
            false => TBIEN_A::VALUE1,
            true => TBIEN_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == TBIEN_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == TBIEN_A::VALUE2
    }
}
impl core::ops::Deref for TBIEN_R {
    type Target = crate::FieldReader<bool, TBIEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TBIEN` writer - Transmit Buffer Interrupt Enable"]
pub struct TBIEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TBIEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TBIEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The transmit buffer interrupt is disabled."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(TBIEN_A::VALUE1)
    }
    #[doc = "The transmit buffer interrupt is enabled. In case of a transmit buffer event, the service request output SRx indicated by INPR.TBINP is activated."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(TBIEN_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | ((value as u32 & 0x01) << 13);
        self.w
    }
}
#[doc = "Receive Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RIEN_A {
    #[doc = "0: The receive interrupt is disabled."]
    VALUE1 = 0,
    #[doc = "1: The receive interrupt is enabled. In case of a receive event, the service request output SRx indicated by INPR.RINP is activated."]
    VALUE2 = 1,
}
impl From<RIEN_A> for bool {
    #[inline(always)]
    fn from(variant: RIEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RIEN` reader - Receive Interrupt Enable"]
pub struct RIEN_R(crate::FieldReader<bool, RIEN_A>);
impl RIEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        RIEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RIEN_A {
        match self.bits {
            false => RIEN_A::VALUE1,
            true => RIEN_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == RIEN_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == RIEN_A::VALUE2
    }
}
impl core::ops::Deref for RIEN_R {
    type Target = crate::FieldReader<bool, RIEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RIEN` writer - Receive Interrupt Enable"]
pub struct RIEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RIEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RIEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The receive interrupt is disabled."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(RIEN_A::VALUE1)
    }
    #[doc = "The receive interrupt is enabled. In case of a receive event, the service request output SRx indicated by INPR.RINP is activated."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(RIEN_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | ((value as u32 & 0x01) << 14);
        self.w
    }
}
#[doc = "Alternative Receive Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AIEN_A {
    #[doc = "0: The alternative receive interrupt is disabled."]
    VALUE1 = 0,
    #[doc = "1: The alternative receive interrupt is enabled. In case of an alternative receive event, the service request output SRx indicated by INPR.AINP is activated."]
    VALUE2 = 1,
}
impl From<AIEN_A> for bool {
    #[inline(always)]
    fn from(variant: AIEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AIEN` reader - Alternative Receive Interrupt Enable"]
pub struct AIEN_R(crate::FieldReader<bool, AIEN_A>);
impl AIEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        AIEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AIEN_A {
        match self.bits {
            false => AIEN_A::VALUE1,
            true => AIEN_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == AIEN_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == AIEN_A::VALUE2
    }
}
impl core::ops::Deref for AIEN_R {
    type Target = crate::FieldReader<bool, AIEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AIEN` writer - Alternative Receive Interrupt Enable"]
pub struct AIEN_W<'a> {
    w: &'a mut W,
}
impl<'a> AIEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AIEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The alternative receive interrupt is disabled."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(AIEN_A::VALUE1)
    }
    #[doc = "The alternative receive interrupt is enabled. In case of an alternative receive event, the service request output SRx indicated by INPR.AINP is activated."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(AIEN_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | ((value as u32 & 0x01) << 15);
        self.w
    }
}
#[doc = "Baud Rate Generator Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BRGIEN_A {
    #[doc = "0: The baud rate generator interrupt is disabled."]
    VALUE1 = 0,
    #[doc = "1: The baud rate generator interrupt is enabled. In case of a baud rate generator event, the service request output SRx indicated by INPR.PINP is activated."]
    VALUE2 = 1,
}
impl From<BRGIEN_A> for bool {
    #[inline(always)]
    fn from(variant: BRGIEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BRGIEN` reader - Baud Rate Generator Interrupt Enable"]
pub struct BRGIEN_R(crate::FieldReader<bool, BRGIEN_A>);
impl BRGIEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        BRGIEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BRGIEN_A {
        match self.bits {
            false => BRGIEN_A::VALUE1,
            true => BRGIEN_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == BRGIEN_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == BRGIEN_A::VALUE2
    }
}
impl core::ops::Deref for BRGIEN_R {
    type Target = crate::FieldReader<bool, BRGIEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BRGIEN` writer - Baud Rate Generator Interrupt Enable"]
pub struct BRGIEN_W<'a> {
    w: &'a mut W,
}
impl<'a> BRGIEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BRGIEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The baud rate generator interrupt is disabled."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(BRGIEN_A::VALUE1)
    }
    #[doc = "The baud rate generator interrupt is enabled. In case of a baud rate generator event, the service request output SRx indicated by INPR.PINP is activated."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(BRGIEN_A::VALUE2)
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
impl R {
    #[doc = "Bits 0:3 - Operating Mode"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 6:7 - Hardware Port Control Enable"]
    #[inline(always)]
    pub fn hpcen(&self) -> HPCEN_R {
        HPCEN_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 8:9 - Parity Mode"]
    #[inline(always)]
    pub fn pm(&self) -> PM_R {
        PM_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bit 10 - Receiver Start Interrupt Enable"]
    #[inline(always)]
    pub fn rsien(&self) -> RSIEN_R {
        RSIEN_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Data Lost Interrupt Enable"]
    #[inline(always)]
    pub fn dlien(&self) -> DLIEN_R {
        DLIEN_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Transmit Shift Interrupt Enable"]
    #[inline(always)]
    pub fn tsien(&self) -> TSIEN_R {
        TSIEN_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Transmit Buffer Interrupt Enable"]
    #[inline(always)]
    pub fn tbien(&self) -> TBIEN_R {
        TBIEN_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Receive Interrupt Enable"]
    #[inline(always)]
    pub fn rien(&self) -> RIEN_R {
        RIEN_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Alternative Receive Interrupt Enable"]
    #[inline(always)]
    pub fn aien(&self) -> AIEN_R {
        AIEN_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Baud Rate Generator Interrupt Enable"]
    #[inline(always)]
    pub fn brgien(&self) -> BRGIEN_R {
        BRGIEN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Operating Mode"]
    #[inline(always)]
    pub fn mode(&mut self) -> MODE_W {
        MODE_W { w: self }
    }
    #[doc = "Bits 6:7 - Hardware Port Control Enable"]
    #[inline(always)]
    pub fn hpcen(&mut self) -> HPCEN_W {
        HPCEN_W { w: self }
    }
    #[doc = "Bits 8:9 - Parity Mode"]
    #[inline(always)]
    pub fn pm(&mut self) -> PM_W {
        PM_W { w: self }
    }
    #[doc = "Bit 10 - Receiver Start Interrupt Enable"]
    #[inline(always)]
    pub fn rsien(&mut self) -> RSIEN_W {
        RSIEN_W { w: self }
    }
    #[doc = "Bit 11 - Data Lost Interrupt Enable"]
    #[inline(always)]
    pub fn dlien(&mut self) -> DLIEN_W {
        DLIEN_W { w: self }
    }
    #[doc = "Bit 12 - Transmit Shift Interrupt Enable"]
    #[inline(always)]
    pub fn tsien(&mut self) -> TSIEN_W {
        TSIEN_W { w: self }
    }
    #[doc = "Bit 13 - Transmit Buffer Interrupt Enable"]
    #[inline(always)]
    pub fn tbien(&mut self) -> TBIEN_W {
        TBIEN_W { w: self }
    }
    #[doc = "Bit 14 - Receive Interrupt Enable"]
    #[inline(always)]
    pub fn rien(&mut self) -> RIEN_W {
        RIEN_W { w: self }
    }
    #[doc = "Bit 15 - Alternative Receive Interrupt Enable"]
    #[inline(always)]
    pub fn aien(&mut self) -> AIEN_W {
        AIEN_W { w: self }
    }
    #[doc = "Bit 16 - Baud Rate Generator Interrupt Enable"]
    #[inline(always)]
    pub fn brgien(&mut self) -> BRGIEN_W {
        BRGIEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Channel Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ccr](index.html) module"]
pub struct CCR_SPEC;
impl crate::RegisterSpec for CCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ccr::R](R) reader structure"]
impl crate::Readable for CCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ccr::W](W) writer structure"]
impl crate::Writable for CCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CCR to value 0"]
impl crate::Resettable for CCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
