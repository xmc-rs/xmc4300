#[doc = "Register `TBCTR` reader"]
pub struct R(crate::R<TBCTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TBCTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TBCTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TBCTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TBCTR` writer"]
pub struct W(crate::W<TBCTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TBCTR_SPEC>;
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
impl From<crate::W<TBCTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TBCTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DPTR` writer - Data Pointer"]
pub struct DPTR_W<'a> {
    w: &'a mut W,
}
impl<'a> DPTR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | (value as u32 & 0x3f);
        self.w
    }
}
#[doc = "Field `LIMIT` reader - Limit For Interrupt Generation"]
pub struct LIMIT_R(crate::FieldReader<u8, u8>);
impl LIMIT_R {
    pub(crate) fn new(bits: u8) -> Self {
        LIMIT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LIMIT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LIMIT` writer - Limit For Interrupt Generation"]
pub struct LIMIT_W<'a> {
    w: &'a mut W,
}
impl<'a> LIMIT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 8)) | ((value as u32 & 0x3f) << 8);
        self.w
    }
}
#[doc = "Standard Transmit Buffer Trigger Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STBTM_A {
    #[doc = "0: Trigger mode 0: While TRBSR.STBT=1, a standard buffer event will be generated whenever there is a data transfer to TBUF or data write to INx (depending on TBCTR.LOF setting). STBT is cleared when TRBSR.TBFLVL=TBCTR.LIMIT."]
    VALUE1 = 0,
    #[doc = "1: Trigger mode 1: While TRBSR.STBT=1, a standard buffer event will be generated whenever there is a data transfer to TBUF or data write to INx (depending on TBCTR.LOF setting). STBT is cleared when TRBSR.TBFLVL=TBCTR.SIZE."]
    VALUE2 = 1,
}
impl From<STBTM_A> for bool {
    #[inline(always)]
    fn from(variant: STBTM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STBTM` reader - Standard Transmit Buffer Trigger Mode"]
pub struct STBTM_R(crate::FieldReader<bool, STBTM_A>);
impl STBTM_R {
    pub(crate) fn new(bits: bool) -> Self {
        STBTM_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STBTM_A {
        match self.bits {
            false => STBTM_A::VALUE1,
            true => STBTM_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == STBTM_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == STBTM_A::VALUE2
    }
}
impl core::ops::Deref for STBTM_R {
    type Target = crate::FieldReader<bool, STBTM_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STBTM` writer - Standard Transmit Buffer Trigger Mode"]
pub struct STBTM_W<'a> {
    w: &'a mut W,
}
impl<'a> STBTM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: STBTM_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Trigger mode 0: While TRBSR.STBT=1, a standard buffer event will be generated whenever there is a data transfer to TBUF or data write to INx (depending on TBCTR.LOF setting). STBT is cleared when TRBSR.TBFLVL=TBCTR.LIMIT."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(STBTM_A::VALUE1)
    }
    #[doc = "Trigger mode 1: While TRBSR.STBT=1, a standard buffer event will be generated whenever there is a data transfer to TBUF or data write to INx (depending on TBCTR.LOF setting). STBT is cleared when TRBSR.TBFLVL=TBCTR.SIZE."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(STBTM_A::VALUE2)
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
#[doc = "Standard Transmit Buffer Trigger Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STBTEN_A {
    #[doc = "0: The standard transmit buffer event trigger through bit TRBSR.STBT is disabled."]
    VALUE1 = 0,
    #[doc = "1: The standard transmit buffer event trigger through bit TRBSR.STBT is enabled."]
    VALUE2 = 1,
}
impl From<STBTEN_A> for bool {
    #[inline(always)]
    fn from(variant: STBTEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STBTEN` reader - Standard Transmit Buffer Trigger Enable"]
pub struct STBTEN_R(crate::FieldReader<bool, STBTEN_A>);
impl STBTEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        STBTEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STBTEN_A {
        match self.bits {
            false => STBTEN_A::VALUE1,
            true => STBTEN_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == STBTEN_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == STBTEN_A::VALUE2
    }
}
impl core::ops::Deref for STBTEN_R {
    type Target = crate::FieldReader<bool, STBTEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STBTEN` writer - Standard Transmit Buffer Trigger Enable"]
pub struct STBTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> STBTEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: STBTEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The standard transmit buffer event trigger through bit TRBSR.STBT is disabled."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(STBTEN_A::VALUE1)
    }
    #[doc = "The standard transmit buffer event trigger through bit TRBSR.STBT is enabled."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(STBTEN_A::VALUE2)
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
#[doc = "Standard Transmit Buffer Interrupt Node Pointer\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum STBINP_A {
    #[doc = "0: Output SR0 becomes activated."]
    VALUE1 = 0,
    #[doc = "1: Output SR1 becomes activated."]
    VALUE2 = 1,
    #[doc = "2: Output SR2 becomes activated."]
    VALUE3 = 2,
    #[doc = "3: Output SR3 becomes activated."]
    VALUE4 = 3,
    #[doc = "4: Output SR4 becomes activated."]
    VALUE5 = 4,
    #[doc = "5: Output SR5 becomes activated."]
    VALUE6 = 5,
}
impl From<STBINP_A> for u8 {
    #[inline(always)]
    fn from(variant: STBINP_A) -> Self {
        variant as _
    }
}
#[doc = "Field `STBINP` reader - Standard Transmit Buffer Interrupt Node Pointer"]
pub struct STBINP_R(crate::FieldReader<u8, STBINP_A>);
impl STBINP_R {
    pub(crate) fn new(bits: u8) -> Self {
        STBINP_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<STBINP_A> {
        match self.bits {
            0 => Some(STBINP_A::VALUE1),
            1 => Some(STBINP_A::VALUE2),
            2 => Some(STBINP_A::VALUE3),
            3 => Some(STBINP_A::VALUE4),
            4 => Some(STBINP_A::VALUE5),
            5 => Some(STBINP_A::VALUE6),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == STBINP_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == STBINP_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        **self == STBINP_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        **self == STBINP_A::VALUE4
    }
    #[doc = "Checks if the value of the field is `VALUE5`"]
    #[inline(always)]
    pub fn is_value5(&self) -> bool {
        **self == STBINP_A::VALUE5
    }
    #[doc = "Checks if the value of the field is `VALUE6`"]
    #[inline(always)]
    pub fn is_value6(&self) -> bool {
        **self == STBINP_A::VALUE6
    }
}
impl core::ops::Deref for STBINP_R {
    type Target = crate::FieldReader<u8, STBINP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STBINP` writer - Standard Transmit Buffer Interrupt Node Pointer"]
pub struct STBINP_W<'a> {
    w: &'a mut W,
}
impl<'a> STBINP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: STBINP_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Output SR0 becomes activated."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(STBINP_A::VALUE1)
    }
    #[doc = "Output SR1 becomes activated."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(STBINP_A::VALUE2)
    }
    #[doc = "Output SR2 becomes activated."]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(STBINP_A::VALUE3)
    }
    #[doc = "Output SR3 becomes activated."]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(STBINP_A::VALUE4)
    }
    #[doc = "Output SR4 becomes activated."]
    #[inline(always)]
    pub fn value5(self) -> &'a mut W {
        self.variant(STBINP_A::VALUE5)
    }
    #[doc = "Output SR5 becomes activated."]
    #[inline(always)]
    pub fn value6(self) -> &'a mut W {
        self.variant(STBINP_A::VALUE6)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 16)) | ((value as u32 & 0x07) << 16);
        self.w
    }
}
#[doc = "Alternative Transmit Buffer Interrupt Node Pointer\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ATBINP_A {
    #[doc = "0: Output SR0 becomes activated."]
    VALUE1 = 0,
    #[doc = "1: Output SR1 becomes activated."]
    VALUE2 = 1,
    #[doc = "2: Output SR2 becomes activated."]
    VALUE3 = 2,
    #[doc = "3: Output SR3 becomes activated."]
    VALUE4 = 3,
    #[doc = "4: Output SR4 becomes activated."]
    VALUE5 = 4,
    #[doc = "5: Output SR5 becomes activated."]
    VALUE6 = 5,
}
impl From<ATBINP_A> for u8 {
    #[inline(always)]
    fn from(variant: ATBINP_A) -> Self {
        variant as _
    }
}
#[doc = "Field `ATBINP` reader - Alternative Transmit Buffer Interrupt Node Pointer"]
pub struct ATBINP_R(crate::FieldReader<u8, ATBINP_A>);
impl ATBINP_R {
    pub(crate) fn new(bits: u8) -> Self {
        ATBINP_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<ATBINP_A> {
        match self.bits {
            0 => Some(ATBINP_A::VALUE1),
            1 => Some(ATBINP_A::VALUE2),
            2 => Some(ATBINP_A::VALUE3),
            3 => Some(ATBINP_A::VALUE4),
            4 => Some(ATBINP_A::VALUE5),
            5 => Some(ATBINP_A::VALUE6),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == ATBINP_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == ATBINP_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        **self == ATBINP_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        **self == ATBINP_A::VALUE4
    }
    #[doc = "Checks if the value of the field is `VALUE5`"]
    #[inline(always)]
    pub fn is_value5(&self) -> bool {
        **self == ATBINP_A::VALUE5
    }
    #[doc = "Checks if the value of the field is `VALUE6`"]
    #[inline(always)]
    pub fn is_value6(&self) -> bool {
        **self == ATBINP_A::VALUE6
    }
}
impl core::ops::Deref for ATBINP_R {
    type Target = crate::FieldReader<u8, ATBINP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ATBINP` writer - Alternative Transmit Buffer Interrupt Node Pointer"]
pub struct ATBINP_W<'a> {
    w: &'a mut W,
}
impl<'a> ATBINP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ATBINP_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Output SR0 becomes activated."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(ATBINP_A::VALUE1)
    }
    #[doc = "Output SR1 becomes activated."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(ATBINP_A::VALUE2)
    }
    #[doc = "Output SR2 becomes activated."]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(ATBINP_A::VALUE3)
    }
    #[doc = "Output SR3 becomes activated."]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(ATBINP_A::VALUE4)
    }
    #[doc = "Output SR4 becomes activated."]
    #[inline(always)]
    pub fn value5(self) -> &'a mut W {
        self.variant(ATBINP_A::VALUE5)
    }
    #[doc = "Output SR5 becomes activated."]
    #[inline(always)]
    pub fn value6(self) -> &'a mut W {
        self.variant(ATBINP_A::VALUE6)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 19)) | ((value as u32 & 0x07) << 19);
        self.w
    }
}
#[doc = "Buffer Size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SIZE_A {
    #[doc = "0: The FIFO mechanism is disabled. The buffer does not accept any request for data."]
    VALUE1 = 0,
    #[doc = "1: The FIFO buffer contains 2 entries."]
    VALUE2 = 1,
    #[doc = "2: The FIFO buffer contains 4 entries."]
    VALUE3 = 2,
    #[doc = "3: The FIFO buffer contains 8 entries."]
    VALUE4 = 3,
    #[doc = "4: The FIFO buffer contains 16 entries."]
    VALUE5 = 4,
    #[doc = "5: The FIFO buffer contains 32 entries."]
    VALUE6 = 5,
    #[doc = "6: The FIFO buffer contains 64 entries."]
    VALUE7 = 6,
}
impl From<SIZE_A> for u8 {
    #[inline(always)]
    fn from(variant: SIZE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SIZE` reader - Buffer Size"]
pub struct SIZE_R(crate::FieldReader<u8, SIZE_A>);
impl SIZE_R {
    pub(crate) fn new(bits: u8) -> Self {
        SIZE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SIZE_A> {
        match self.bits {
            0 => Some(SIZE_A::VALUE1),
            1 => Some(SIZE_A::VALUE2),
            2 => Some(SIZE_A::VALUE3),
            3 => Some(SIZE_A::VALUE4),
            4 => Some(SIZE_A::VALUE5),
            5 => Some(SIZE_A::VALUE6),
            6 => Some(SIZE_A::VALUE7),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == SIZE_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == SIZE_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        **self == SIZE_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        **self == SIZE_A::VALUE4
    }
    #[doc = "Checks if the value of the field is `VALUE5`"]
    #[inline(always)]
    pub fn is_value5(&self) -> bool {
        **self == SIZE_A::VALUE5
    }
    #[doc = "Checks if the value of the field is `VALUE6`"]
    #[inline(always)]
    pub fn is_value6(&self) -> bool {
        **self == SIZE_A::VALUE6
    }
    #[doc = "Checks if the value of the field is `VALUE7`"]
    #[inline(always)]
    pub fn is_value7(&self) -> bool {
        **self == SIZE_A::VALUE7
    }
}
impl core::ops::Deref for SIZE_R {
    type Target = crate::FieldReader<u8, SIZE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SIZE` writer - Buffer Size"]
pub struct SIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> SIZE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SIZE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "The FIFO mechanism is disabled. The buffer does not accept any request for data."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(SIZE_A::VALUE1)
    }
    #[doc = "The FIFO buffer contains 2 entries."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(SIZE_A::VALUE2)
    }
    #[doc = "The FIFO buffer contains 4 entries."]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(SIZE_A::VALUE3)
    }
    #[doc = "The FIFO buffer contains 8 entries."]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(SIZE_A::VALUE4)
    }
    #[doc = "The FIFO buffer contains 16 entries."]
    #[inline(always)]
    pub fn value5(self) -> &'a mut W {
        self.variant(SIZE_A::VALUE5)
    }
    #[doc = "The FIFO buffer contains 32 entries."]
    #[inline(always)]
    pub fn value6(self) -> &'a mut W {
        self.variant(SIZE_A::VALUE6)
    }
    #[doc = "The FIFO buffer contains 64 entries."]
    #[inline(always)]
    pub fn value7(self) -> &'a mut W {
        self.variant(SIZE_A::VALUE7)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 24)) | ((value as u32 & 0x07) << 24);
        self.w
    }
}
#[doc = "Buffer Event on Limit Overflow\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LOF_A {
    #[doc = "0: A standard transmit buffer event occurs when the filling level equals the limit value and gets lower due to transmission of a data word."]
    VALUE1 = 0,
    #[doc = "1: A standard transmit buffer interrupt event occurs when the filling level equals the limit value and gets bigger due to a write access to a data input location INx."]
    VALUE2 = 1,
}
impl From<LOF_A> for bool {
    #[inline(always)]
    fn from(variant: LOF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LOF` reader - Buffer Event on Limit Overflow"]
pub struct LOF_R(crate::FieldReader<bool, LOF_A>);
impl LOF_R {
    pub(crate) fn new(bits: bool) -> Self {
        LOF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LOF_A {
        match self.bits {
            false => LOF_A::VALUE1,
            true => LOF_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == LOF_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == LOF_A::VALUE2
    }
}
impl core::ops::Deref for LOF_R {
    type Target = crate::FieldReader<bool, LOF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LOF` writer - Buffer Event on Limit Overflow"]
pub struct LOF_W<'a> {
    w: &'a mut W,
}
impl<'a> LOF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LOF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "A standard transmit buffer event occurs when the filling level equals the limit value and gets lower due to transmission of a data word."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(LOF_A::VALUE1)
    }
    #[doc = "A standard transmit buffer interrupt event occurs when the filling level equals the limit value and gets bigger due to a write access to a data input location INx."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(LOF_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | ((value as u32 & 0x01) << 28);
        self.w
    }
}
#[doc = "Standard Transmit Buffer Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STBIEN_A {
    #[doc = "0: The standard transmit buffer interrupt generation is disabled."]
    VALUE1 = 0,
    #[doc = "1: The standard transmit buffer interrupt generation is enabled."]
    VALUE2 = 1,
}
impl From<STBIEN_A> for bool {
    #[inline(always)]
    fn from(variant: STBIEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STBIEN` reader - Standard Transmit Buffer Interrupt Enable"]
pub struct STBIEN_R(crate::FieldReader<bool, STBIEN_A>);
impl STBIEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        STBIEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STBIEN_A {
        match self.bits {
            false => STBIEN_A::VALUE1,
            true => STBIEN_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == STBIEN_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == STBIEN_A::VALUE2
    }
}
impl core::ops::Deref for STBIEN_R {
    type Target = crate::FieldReader<bool, STBIEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STBIEN` writer - Standard Transmit Buffer Interrupt Enable"]
pub struct STBIEN_W<'a> {
    w: &'a mut W,
}
impl<'a> STBIEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: STBIEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The standard transmit buffer interrupt generation is disabled."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(STBIEN_A::VALUE1)
    }
    #[doc = "The standard transmit buffer interrupt generation is enabled."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(STBIEN_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | ((value as u32 & 0x01) << 30);
        self.w
    }
}
#[doc = "Transmit Buffer Error Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TBERIEN_A {
    #[doc = "0: The transmit buffer error interrupt generation is disabled."]
    VALUE1 = 0,
    #[doc = "1: The transmit buffer error interrupt generation is enabled."]
    VALUE2 = 1,
}
impl From<TBERIEN_A> for bool {
    #[inline(always)]
    fn from(variant: TBERIEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TBERIEN` reader - Transmit Buffer Error Interrupt Enable"]
pub struct TBERIEN_R(crate::FieldReader<bool, TBERIEN_A>);
impl TBERIEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        TBERIEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TBERIEN_A {
        match self.bits {
            false => TBERIEN_A::VALUE1,
            true => TBERIEN_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == TBERIEN_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == TBERIEN_A::VALUE2
    }
}
impl core::ops::Deref for TBERIEN_R {
    type Target = crate::FieldReader<bool, TBERIEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TBERIEN` writer - Transmit Buffer Error Interrupt Enable"]
pub struct TBERIEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TBERIEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TBERIEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The transmit buffer error interrupt generation is disabled."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(TBERIEN_A::VALUE1)
    }
    #[doc = "The transmit buffer error interrupt generation is enabled."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(TBERIEN_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | ((value as u32 & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bits 8:13 - Limit For Interrupt Generation"]
    #[inline(always)]
    pub fn limit(&self) -> LIMIT_R {
        LIMIT_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bit 14 - Standard Transmit Buffer Trigger Mode"]
    #[inline(always)]
    pub fn stbtm(&self) -> STBTM_R {
        STBTM_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Standard Transmit Buffer Trigger Enable"]
    #[inline(always)]
    pub fn stbten(&self) -> STBTEN_R {
        STBTEN_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 16:18 - Standard Transmit Buffer Interrupt Node Pointer"]
    #[inline(always)]
    pub fn stbinp(&self) -> STBINP_R {
        STBINP_R::new(((self.bits >> 16) & 0x07) as u8)
    }
    #[doc = "Bits 19:21 - Alternative Transmit Buffer Interrupt Node Pointer"]
    #[inline(always)]
    pub fn atbinp(&self) -> ATBINP_R {
        ATBINP_R::new(((self.bits >> 19) & 0x07) as u8)
    }
    #[doc = "Bits 24:26 - Buffer Size"]
    #[inline(always)]
    pub fn size(&self) -> SIZE_R {
        SIZE_R::new(((self.bits >> 24) & 0x07) as u8)
    }
    #[doc = "Bit 28 - Buffer Event on Limit Overflow"]
    #[inline(always)]
    pub fn lof(&self) -> LOF_R {
        LOF_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Standard Transmit Buffer Interrupt Enable"]
    #[inline(always)]
    pub fn stbien(&self) -> STBIEN_R {
        STBIEN_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Transmit Buffer Error Interrupt Enable"]
    #[inline(always)]
    pub fn tberien(&self) -> TBERIEN_R {
        TBERIEN_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - Data Pointer"]
    #[inline(always)]
    pub fn dptr(&mut self) -> DPTR_W {
        DPTR_W { w: self }
    }
    #[doc = "Bits 8:13 - Limit For Interrupt Generation"]
    #[inline(always)]
    pub fn limit(&mut self) -> LIMIT_W {
        LIMIT_W { w: self }
    }
    #[doc = "Bit 14 - Standard Transmit Buffer Trigger Mode"]
    #[inline(always)]
    pub fn stbtm(&mut self) -> STBTM_W {
        STBTM_W { w: self }
    }
    #[doc = "Bit 15 - Standard Transmit Buffer Trigger Enable"]
    #[inline(always)]
    pub fn stbten(&mut self) -> STBTEN_W {
        STBTEN_W { w: self }
    }
    #[doc = "Bits 16:18 - Standard Transmit Buffer Interrupt Node Pointer"]
    #[inline(always)]
    pub fn stbinp(&mut self) -> STBINP_W {
        STBINP_W { w: self }
    }
    #[doc = "Bits 19:21 - Alternative Transmit Buffer Interrupt Node Pointer"]
    #[inline(always)]
    pub fn atbinp(&mut self) -> ATBINP_W {
        ATBINP_W { w: self }
    }
    #[doc = "Bits 24:26 - Buffer Size"]
    #[inline(always)]
    pub fn size(&mut self) -> SIZE_W {
        SIZE_W { w: self }
    }
    #[doc = "Bit 28 - Buffer Event on Limit Overflow"]
    #[inline(always)]
    pub fn lof(&mut self) -> LOF_W {
        LOF_W { w: self }
    }
    #[doc = "Bit 30 - Standard Transmit Buffer Interrupt Enable"]
    #[inline(always)]
    pub fn stbien(&mut self) -> STBIEN_W {
        STBIEN_W { w: self }
    }
    #[doc = "Bit 31 - Transmit Buffer Error Interrupt Enable"]
    #[inline(always)]
    pub fn tberien(&mut self) -> TBERIEN_W {
        TBERIEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Transmitter Buffer Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tbctr](index.html) module"]
pub struct TBCTR_SPEC;
impl crate::RegisterSpec for TBCTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tbctr::R](R) reader structure"]
impl crate::Readable for TBCTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tbctr::W](W) writer structure"]
impl crate::Writable for TBCTR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TBCTR to value 0"]
impl crate::Resettable for TBCTR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
