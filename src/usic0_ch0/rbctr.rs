#[doc = "Register `RBCTR` reader"]
pub struct R(crate::R<RBCTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RBCTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RBCTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RBCTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RBCTR` writer"]
pub struct W(crate::W<RBCTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RBCTR_SPEC>;
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
impl From<crate::W<RBCTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RBCTR_SPEC>) -> Self {
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
#[doc = "Standard Receive Buffer Trigger Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRBTM_A {
    #[doc = "0: Trigger mode 0: While TRBSR.SRBT=1, a standard receive buffer event will be generated whenever there is a new data received or data read out (depending on RBCTR.LOF setting). SRBT is cleared when TRBSR.RBFLVL=RBCTR.LIMIT."]
    VALUE1 = 0,
    #[doc = "1: Trigger mode 1: While TRBSR.SRBT=1, a standard receive buffer event will be generated whenever there is a new data received or data read out (depending on RBCTR.LOF setting). SRBT is cleared when TRBSR.RBFLVL=0."]
    VALUE2 = 1,
}
impl From<SRBTM_A> for bool {
    #[inline(always)]
    fn from(variant: SRBTM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRBTM` reader - Standard Receive Buffer Trigger Mode"]
pub struct SRBTM_R(crate::FieldReader<bool, SRBTM_A>);
impl SRBTM_R {
    pub(crate) fn new(bits: bool) -> Self {
        SRBTM_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRBTM_A {
        match self.bits {
            false => SRBTM_A::VALUE1,
            true => SRBTM_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == SRBTM_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == SRBTM_A::VALUE2
    }
}
impl core::ops::Deref for SRBTM_R {
    type Target = crate::FieldReader<bool, SRBTM_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SRBTM` writer - Standard Receive Buffer Trigger Mode"]
pub struct SRBTM_W<'a> {
    w: &'a mut W,
}
impl<'a> SRBTM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SRBTM_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Trigger mode 0: While TRBSR.SRBT=1, a standard receive buffer event will be generated whenever there is a new data received or data read out (depending on RBCTR.LOF setting). SRBT is cleared when TRBSR.RBFLVL=RBCTR.LIMIT."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(SRBTM_A::VALUE1)
    }
    #[doc = "Trigger mode 1: While TRBSR.SRBT=1, a standard receive buffer event will be generated whenever there is a new data received or data read out (depending on RBCTR.LOF setting). SRBT is cleared when TRBSR.RBFLVL=0."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(SRBTM_A::VALUE2)
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
#[doc = "Standard Receive Buffer Trigger Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRBTEN_A {
    #[doc = "0: The standard receive buffer event trigger through bit TRBSR.SRBT is disabled."]
    VALUE1 = 0,
    #[doc = "1: The standard receive buffer event trigger through bit TRBSR.SRBT is enabled."]
    VALUE2 = 1,
}
impl From<SRBTEN_A> for bool {
    #[inline(always)]
    fn from(variant: SRBTEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRBTEN` reader - Standard Receive Buffer Trigger Enable"]
pub struct SRBTEN_R(crate::FieldReader<bool, SRBTEN_A>);
impl SRBTEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        SRBTEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRBTEN_A {
        match self.bits {
            false => SRBTEN_A::VALUE1,
            true => SRBTEN_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == SRBTEN_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == SRBTEN_A::VALUE2
    }
}
impl core::ops::Deref for SRBTEN_R {
    type Target = crate::FieldReader<bool, SRBTEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SRBTEN` writer - Standard Receive Buffer Trigger Enable"]
pub struct SRBTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SRBTEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SRBTEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The standard receive buffer event trigger through bit TRBSR.SRBT is disabled."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(SRBTEN_A::VALUE1)
    }
    #[doc = "The standard receive buffer event trigger through bit TRBSR.SRBT is enabled."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(SRBTEN_A::VALUE2)
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
#[doc = "Standard Receive Buffer Interrupt Node Pointer\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SRBINP_A {
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
impl From<SRBINP_A> for u8 {
    #[inline(always)]
    fn from(variant: SRBINP_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SRBINP` reader - Standard Receive Buffer Interrupt Node Pointer"]
pub struct SRBINP_R(crate::FieldReader<u8, SRBINP_A>);
impl SRBINP_R {
    pub(crate) fn new(bits: u8) -> Self {
        SRBINP_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SRBINP_A> {
        match self.bits {
            0 => Some(SRBINP_A::VALUE1),
            1 => Some(SRBINP_A::VALUE2),
            2 => Some(SRBINP_A::VALUE3),
            3 => Some(SRBINP_A::VALUE4),
            4 => Some(SRBINP_A::VALUE5),
            5 => Some(SRBINP_A::VALUE6),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == SRBINP_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == SRBINP_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        **self == SRBINP_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        **self == SRBINP_A::VALUE4
    }
    #[doc = "Checks if the value of the field is `VALUE5`"]
    #[inline(always)]
    pub fn is_value5(&self) -> bool {
        **self == SRBINP_A::VALUE5
    }
    #[doc = "Checks if the value of the field is `VALUE6`"]
    #[inline(always)]
    pub fn is_value6(&self) -> bool {
        **self == SRBINP_A::VALUE6
    }
}
impl core::ops::Deref for SRBINP_R {
    type Target = crate::FieldReader<u8, SRBINP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SRBINP` writer - Standard Receive Buffer Interrupt Node Pointer"]
pub struct SRBINP_W<'a> {
    w: &'a mut W,
}
impl<'a> SRBINP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SRBINP_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Output SR0 becomes activated."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(SRBINP_A::VALUE1)
    }
    #[doc = "Output SR1 becomes activated."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(SRBINP_A::VALUE2)
    }
    #[doc = "Output SR2 becomes activated."]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(SRBINP_A::VALUE3)
    }
    #[doc = "Output SR3 becomes activated."]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(SRBINP_A::VALUE4)
    }
    #[doc = "Output SR4 becomes activated."]
    #[inline(always)]
    pub fn value5(self) -> &'a mut W {
        self.variant(SRBINP_A::VALUE5)
    }
    #[doc = "Output SR5 becomes activated."]
    #[inline(always)]
    pub fn value6(self) -> &'a mut W {
        self.variant(SRBINP_A::VALUE6)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 16)) | ((value as u32 & 0x07) << 16);
        self.w
    }
}
#[doc = "Alternative Receive Buffer Interrupt Node Pointer\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ARBINP_A {
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
impl From<ARBINP_A> for u8 {
    #[inline(always)]
    fn from(variant: ARBINP_A) -> Self {
        variant as _
    }
}
#[doc = "Field `ARBINP` reader - Alternative Receive Buffer Interrupt Node Pointer"]
pub struct ARBINP_R(crate::FieldReader<u8, ARBINP_A>);
impl ARBINP_R {
    pub(crate) fn new(bits: u8) -> Self {
        ARBINP_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<ARBINP_A> {
        match self.bits {
            0 => Some(ARBINP_A::VALUE1),
            1 => Some(ARBINP_A::VALUE2),
            2 => Some(ARBINP_A::VALUE3),
            3 => Some(ARBINP_A::VALUE4),
            4 => Some(ARBINP_A::VALUE5),
            5 => Some(ARBINP_A::VALUE6),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == ARBINP_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == ARBINP_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        **self == ARBINP_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        **self == ARBINP_A::VALUE4
    }
    #[doc = "Checks if the value of the field is `VALUE5`"]
    #[inline(always)]
    pub fn is_value5(&self) -> bool {
        **self == ARBINP_A::VALUE5
    }
    #[doc = "Checks if the value of the field is `VALUE6`"]
    #[inline(always)]
    pub fn is_value6(&self) -> bool {
        **self == ARBINP_A::VALUE6
    }
}
impl core::ops::Deref for ARBINP_R {
    type Target = crate::FieldReader<u8, ARBINP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ARBINP` writer - Alternative Receive Buffer Interrupt Node Pointer"]
pub struct ARBINP_W<'a> {
    w: &'a mut W,
}
impl<'a> ARBINP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ARBINP_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Output SR0 becomes activated."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(ARBINP_A::VALUE1)
    }
    #[doc = "Output SR1 becomes activated."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(ARBINP_A::VALUE2)
    }
    #[doc = "Output SR2 becomes activated."]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(ARBINP_A::VALUE3)
    }
    #[doc = "Output SR3 becomes activated."]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(ARBINP_A::VALUE4)
    }
    #[doc = "Output SR4 becomes activated."]
    #[inline(always)]
    pub fn value5(self) -> &'a mut W {
        self.variant(ARBINP_A::VALUE5)
    }
    #[doc = "Output SR5 becomes activated."]
    #[inline(always)]
    pub fn value6(self) -> &'a mut W {
        self.variant(ARBINP_A::VALUE6)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 19)) | ((value as u32 & 0x07) << 19);
        self.w
    }
}
#[doc = "Receiver Control Information Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RCIM_A {
    #[doc = "0: RCI\\[4\\]
= PERR, RCI\\[3:0\\]
= WLEN"]
    VALUE1 = 0,
    #[doc = "1: RCI\\[4\\]
= SOF, RCI\\[3:0\\]
= WLEN"]
    VALUE2 = 1,
    #[doc = "2: RCI\\[4\\]
= 0, RCI\\[3:0\\]
= WLEN"]
    VALUE3 = 2,
    #[doc = "3: RCI\\[4\\]
= PERR, RCI\\[3\\]
= PAR, RCI\\[2:1\\]
= 00B, RCI\\[0\\]
= SOF"]
    VALUE4 = 3,
}
impl From<RCIM_A> for u8 {
    #[inline(always)]
    fn from(variant: RCIM_A) -> Self {
        variant as _
    }
}
#[doc = "Field `RCIM` reader - Receiver Control Information Mode"]
pub struct RCIM_R(crate::FieldReader<u8, RCIM_A>);
impl RCIM_R {
    pub(crate) fn new(bits: u8) -> Self {
        RCIM_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RCIM_A {
        match self.bits {
            0 => RCIM_A::VALUE1,
            1 => RCIM_A::VALUE2,
            2 => RCIM_A::VALUE3,
            3 => RCIM_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == RCIM_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == RCIM_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        **self == RCIM_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        **self == RCIM_A::VALUE4
    }
}
impl core::ops::Deref for RCIM_R {
    type Target = crate::FieldReader<u8, RCIM_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RCIM` writer - Receiver Control Information Mode"]
pub struct RCIM_W<'a> {
    w: &'a mut W,
}
impl<'a> RCIM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RCIM_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "RCI\\[4\\]
= PERR, RCI\\[3:0\\]
= WLEN"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(RCIM_A::VALUE1)
    }
    #[doc = "RCI\\[4\\]
= SOF, RCI\\[3:0\\]
= WLEN"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(RCIM_A::VALUE2)
    }
    #[doc = "RCI\\[4\\]
= 0, RCI\\[3:0\\]
= WLEN"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(RCIM_A::VALUE3)
    }
    #[doc = "RCI\\[4\\]
= PERR, RCI\\[3\\]
= PAR, RCI\\[2:1\\]
= 00B, RCI\\[0\\]
= SOF"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(RCIM_A::VALUE4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 22)) | ((value as u32 & 0x03) << 22);
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
#[doc = "Receiver Notification Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RNM_A {
    #[doc = "0: Filling level mode: A standard receive buffer event occurs when the filling level equals the limit value and changes, either due to a read access from OUTR (LOF = 0) or due to a new received data word (LOF = 1)."]
    VALUE1 = 0,
    #[doc = "1: RCI mode: A standard receive buffer event occurs when register OUTR is updated with a new value if the corresponding value in OUTR.RCI\\[4\\]
= 0. If OUTR.RCI\\[4\\]
= 1, an alternative receive buffer event occurs instead of the standard receive buffer event."]
    VALUE2 = 1,
}
impl From<RNM_A> for bool {
    #[inline(always)]
    fn from(variant: RNM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RNM` reader - Receiver Notification Mode"]
pub struct RNM_R(crate::FieldReader<bool, RNM_A>);
impl RNM_R {
    pub(crate) fn new(bits: bool) -> Self {
        RNM_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RNM_A {
        match self.bits {
            false => RNM_A::VALUE1,
            true => RNM_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == RNM_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == RNM_A::VALUE2
    }
}
impl core::ops::Deref for RNM_R {
    type Target = crate::FieldReader<bool, RNM_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RNM` writer - Receiver Notification Mode"]
pub struct RNM_W<'a> {
    w: &'a mut W,
}
impl<'a> RNM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RNM_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Filling level mode: A standard receive buffer event occurs when the filling level equals the limit value and changes, either due to a read access from OUTR (LOF = 0) or due to a new received data word (LOF = 1)."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(RNM_A::VALUE1)
    }
    #[doc = "RCI mode: A standard receive buffer event occurs when register OUTR is updated with a new value if the corresponding value in OUTR.RCI\\[4\\]
= 0. If OUTR.RCI\\[4\\]
= 1, an alternative receive buffer event occurs instead of the standard receive buffer event."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(RNM_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | ((value as u32 & 0x01) << 27);
        self.w
    }
}
#[doc = "Buffer Event on Limit Overflow\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LOF_A {
    #[doc = "0: A standard receive buffer event occurs when the filling level equals the limit value and gets lower due to a read access from OUTR."]
    VALUE1 = 0,
    #[doc = "1: A standard receive buffer event occurs when the filling level equals the limit value and gets bigger due to the reception of a new data word."]
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
    #[doc = "A standard receive buffer event occurs when the filling level equals the limit value and gets lower due to a read access from OUTR."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(LOF_A::VALUE1)
    }
    #[doc = "A standard receive buffer event occurs when the filling level equals the limit value and gets bigger due to the reception of a new data word."]
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
#[doc = "Alternative Receive Buffer Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ARBIEN_A {
    #[doc = "0: The alternative receive buffer interrupt generation is disabled."]
    VALUE1 = 0,
    #[doc = "1: The alternative receive buffer interrupt generation is enabled."]
    VALUE2 = 1,
}
impl From<ARBIEN_A> for bool {
    #[inline(always)]
    fn from(variant: ARBIEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ARBIEN` reader - Alternative Receive Buffer Interrupt Enable"]
pub struct ARBIEN_R(crate::FieldReader<bool, ARBIEN_A>);
impl ARBIEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        ARBIEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ARBIEN_A {
        match self.bits {
            false => ARBIEN_A::VALUE1,
            true => ARBIEN_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == ARBIEN_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == ARBIEN_A::VALUE2
    }
}
impl core::ops::Deref for ARBIEN_R {
    type Target = crate::FieldReader<bool, ARBIEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ARBIEN` writer - Alternative Receive Buffer Interrupt Enable"]
pub struct ARBIEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ARBIEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ARBIEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The alternative receive buffer interrupt generation is disabled."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(ARBIEN_A::VALUE1)
    }
    #[doc = "The alternative receive buffer interrupt generation is enabled."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(ARBIEN_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | ((value as u32 & 0x01) << 29);
        self.w
    }
}
#[doc = "Standard Receive Buffer Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRBIEN_A {
    #[doc = "0: The standard receive buffer interrupt generation is disabled."]
    VALUE1 = 0,
    #[doc = "1: The standard receive buffer interrupt generation is enabled."]
    VALUE2 = 1,
}
impl From<SRBIEN_A> for bool {
    #[inline(always)]
    fn from(variant: SRBIEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRBIEN` reader - Standard Receive Buffer Interrupt Enable"]
pub struct SRBIEN_R(crate::FieldReader<bool, SRBIEN_A>);
impl SRBIEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        SRBIEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRBIEN_A {
        match self.bits {
            false => SRBIEN_A::VALUE1,
            true => SRBIEN_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == SRBIEN_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == SRBIEN_A::VALUE2
    }
}
impl core::ops::Deref for SRBIEN_R {
    type Target = crate::FieldReader<bool, SRBIEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SRBIEN` writer - Standard Receive Buffer Interrupt Enable"]
pub struct SRBIEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SRBIEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SRBIEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The standard receive buffer interrupt generation is disabled."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(SRBIEN_A::VALUE1)
    }
    #[doc = "The standard receive buffer interrupt generation is enabled."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(SRBIEN_A::VALUE2)
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
#[doc = "Receive Buffer Error Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RBERIEN_A {
    #[doc = "0: The receive buffer error interrupt generation is disabled."]
    VALUE1 = 0,
    #[doc = "1: The receive buffer error interrupt generation is enabled."]
    VALUE2 = 1,
}
impl From<RBERIEN_A> for bool {
    #[inline(always)]
    fn from(variant: RBERIEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RBERIEN` reader - Receive Buffer Error Interrupt Enable"]
pub struct RBERIEN_R(crate::FieldReader<bool, RBERIEN_A>);
impl RBERIEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        RBERIEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RBERIEN_A {
        match self.bits {
            false => RBERIEN_A::VALUE1,
            true => RBERIEN_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == RBERIEN_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == RBERIEN_A::VALUE2
    }
}
impl core::ops::Deref for RBERIEN_R {
    type Target = crate::FieldReader<bool, RBERIEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RBERIEN` writer - Receive Buffer Error Interrupt Enable"]
pub struct RBERIEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RBERIEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RBERIEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The receive buffer error interrupt generation is disabled."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(RBERIEN_A::VALUE1)
    }
    #[doc = "The receive buffer error interrupt generation is enabled."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(RBERIEN_A::VALUE2)
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
    #[doc = "Bit 14 - Standard Receive Buffer Trigger Mode"]
    #[inline(always)]
    pub fn srbtm(&self) -> SRBTM_R {
        SRBTM_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Standard Receive Buffer Trigger Enable"]
    #[inline(always)]
    pub fn srbten(&self) -> SRBTEN_R {
        SRBTEN_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 16:18 - Standard Receive Buffer Interrupt Node Pointer"]
    #[inline(always)]
    pub fn srbinp(&self) -> SRBINP_R {
        SRBINP_R::new(((self.bits >> 16) & 0x07) as u8)
    }
    #[doc = "Bits 19:21 - Alternative Receive Buffer Interrupt Node Pointer"]
    #[inline(always)]
    pub fn arbinp(&self) -> ARBINP_R {
        ARBINP_R::new(((self.bits >> 19) & 0x07) as u8)
    }
    #[doc = "Bits 22:23 - Receiver Control Information Mode"]
    #[inline(always)]
    pub fn rcim(&self) -> RCIM_R {
        RCIM_R::new(((self.bits >> 22) & 0x03) as u8)
    }
    #[doc = "Bits 24:26 - Buffer Size"]
    #[inline(always)]
    pub fn size(&self) -> SIZE_R {
        SIZE_R::new(((self.bits >> 24) & 0x07) as u8)
    }
    #[doc = "Bit 27 - Receiver Notification Mode"]
    #[inline(always)]
    pub fn rnm(&self) -> RNM_R {
        RNM_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Buffer Event on Limit Overflow"]
    #[inline(always)]
    pub fn lof(&self) -> LOF_R {
        LOF_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Alternative Receive Buffer Interrupt Enable"]
    #[inline(always)]
    pub fn arbien(&self) -> ARBIEN_R {
        ARBIEN_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Standard Receive Buffer Interrupt Enable"]
    #[inline(always)]
    pub fn srbien(&self) -> SRBIEN_R {
        SRBIEN_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Receive Buffer Error Interrupt Enable"]
    #[inline(always)]
    pub fn rberien(&self) -> RBERIEN_R {
        RBERIEN_R::new(((self.bits >> 31) & 0x01) != 0)
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
    #[doc = "Bit 14 - Standard Receive Buffer Trigger Mode"]
    #[inline(always)]
    pub fn srbtm(&mut self) -> SRBTM_W {
        SRBTM_W { w: self }
    }
    #[doc = "Bit 15 - Standard Receive Buffer Trigger Enable"]
    #[inline(always)]
    pub fn srbten(&mut self) -> SRBTEN_W {
        SRBTEN_W { w: self }
    }
    #[doc = "Bits 16:18 - Standard Receive Buffer Interrupt Node Pointer"]
    #[inline(always)]
    pub fn srbinp(&mut self) -> SRBINP_W {
        SRBINP_W { w: self }
    }
    #[doc = "Bits 19:21 - Alternative Receive Buffer Interrupt Node Pointer"]
    #[inline(always)]
    pub fn arbinp(&mut self) -> ARBINP_W {
        ARBINP_W { w: self }
    }
    #[doc = "Bits 22:23 - Receiver Control Information Mode"]
    #[inline(always)]
    pub fn rcim(&mut self) -> RCIM_W {
        RCIM_W { w: self }
    }
    #[doc = "Bits 24:26 - Buffer Size"]
    #[inline(always)]
    pub fn size(&mut self) -> SIZE_W {
        SIZE_W { w: self }
    }
    #[doc = "Bit 27 - Receiver Notification Mode"]
    #[inline(always)]
    pub fn rnm(&mut self) -> RNM_W {
        RNM_W { w: self }
    }
    #[doc = "Bit 28 - Buffer Event on Limit Overflow"]
    #[inline(always)]
    pub fn lof(&mut self) -> LOF_W {
        LOF_W { w: self }
    }
    #[doc = "Bit 29 - Alternative Receive Buffer Interrupt Enable"]
    #[inline(always)]
    pub fn arbien(&mut self) -> ARBIEN_W {
        ARBIEN_W { w: self }
    }
    #[doc = "Bit 30 - Standard Receive Buffer Interrupt Enable"]
    #[inline(always)]
    pub fn srbien(&mut self) -> SRBIEN_W {
        SRBIEN_W { w: self }
    }
    #[doc = "Bit 31 - Receive Buffer Error Interrupt Enable"]
    #[inline(always)]
    pub fn rberien(&mut self) -> RBERIEN_W {
        RBERIEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Receiver Buffer Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rbctr](index.html) module"]
pub struct RBCTR_SPEC;
impl crate::RegisterSpec for RBCTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rbctr::R](R) reader structure"]
impl crate::Readable for RBCTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rbctr::W](W) writer structure"]
impl crate::Writable for RBCTR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RBCTR to value 0"]
impl crate::Resettable for RBCTR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
