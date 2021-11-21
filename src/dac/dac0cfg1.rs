#[doc = "Register `DAC0CFG1` reader"]
pub struct R(crate::R<DAC0CFG1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DAC0CFG1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DAC0CFG1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DAC0CFG1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DAC0CFG1` writer"]
pub struct W(crate::W<DAC0CFG1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DAC0CFG1_SPEC>;
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
impl From<crate::W<DAC0CFG1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DAC0CFG1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Scale value for up- or downscale of the DAC0 input data in steps by the power of 2 (=shift operation)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SCALE_A {
    #[doc = "0: no shift = multiplication/division by 1"]
    VALUE1 = 0,
    #[doc = "1: shift by 1 = multiplication/division by 2"]
    VALUE2 = 1,
    #[doc = "2: shift by 2 = multiplication/division by 4"]
    VALUE3 = 2,
    #[doc = "3: shift left by 3 = multiplication/division by 8"]
    VALUE4 = 3,
    #[doc = "4: shift left by 4 = multiplication/division by 16"]
    VALUE5 = 4,
    #[doc = "5: shift left by 5 = multiplication/division by 32"]
    VALUE6 = 5,
    #[doc = "6: shift left by 6 = multiplication/division by 64"]
    VALUE7 = 6,
    #[doc = "7: shift left by 7 = multiplication/division by 128"]
    VALUE8 = 7,
}
impl From<SCALE_A> for u8 {
    #[inline(always)]
    fn from(variant: SCALE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SCALE` reader - Scale value for up- or downscale of the DAC0 input data in steps by the power of 2 (=shift operation)"]
pub struct SCALE_R(crate::FieldReader<u8, SCALE_A>);
impl SCALE_R {
    pub(crate) fn new(bits: u8) -> Self {
        SCALE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SCALE_A {
        match self.bits {
            0 => SCALE_A::VALUE1,
            1 => SCALE_A::VALUE2,
            2 => SCALE_A::VALUE3,
            3 => SCALE_A::VALUE4,
            4 => SCALE_A::VALUE5,
            5 => SCALE_A::VALUE6,
            6 => SCALE_A::VALUE7,
            7 => SCALE_A::VALUE8,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == SCALE_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == SCALE_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        **self == SCALE_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        **self == SCALE_A::VALUE4
    }
    #[doc = "Checks if the value of the field is `VALUE5`"]
    #[inline(always)]
    pub fn is_value5(&self) -> bool {
        **self == SCALE_A::VALUE5
    }
    #[doc = "Checks if the value of the field is `VALUE6`"]
    #[inline(always)]
    pub fn is_value6(&self) -> bool {
        **self == SCALE_A::VALUE6
    }
    #[doc = "Checks if the value of the field is `VALUE7`"]
    #[inline(always)]
    pub fn is_value7(&self) -> bool {
        **self == SCALE_A::VALUE7
    }
    #[doc = "Checks if the value of the field is `VALUE8`"]
    #[inline(always)]
    pub fn is_value8(&self) -> bool {
        **self == SCALE_A::VALUE8
    }
}
impl core::ops::Deref for SCALE_R {
    type Target = crate::FieldReader<u8, SCALE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SCALE` writer - Scale value for up- or downscale of the DAC0 input data in steps by the power of 2 (=shift operation)"]
pub struct SCALE_W<'a> {
    w: &'a mut W,
}
impl<'a> SCALE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SCALE_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "no shift = multiplication/division by 1"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(SCALE_A::VALUE1)
    }
    #[doc = "shift by 1 = multiplication/division by 2"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(SCALE_A::VALUE2)
    }
    #[doc = "shift by 2 = multiplication/division by 4"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(SCALE_A::VALUE3)
    }
    #[doc = "shift left by 3 = multiplication/division by 8"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(SCALE_A::VALUE4)
    }
    #[doc = "shift left by 4 = multiplication/division by 16"]
    #[inline(always)]
    pub fn value5(self) -> &'a mut W {
        self.variant(SCALE_A::VALUE5)
    }
    #[doc = "shift left by 5 = multiplication/division by 32"]
    #[inline(always)]
    pub fn value6(self) -> &'a mut W {
        self.variant(SCALE_A::VALUE6)
    }
    #[doc = "shift left by 6 = multiplication/division by 64"]
    #[inline(always)]
    pub fn value7(self) -> &'a mut W {
        self.variant(SCALE_A::VALUE7)
    }
    #[doc = "shift left by 7 = multiplication/division by 128"]
    #[inline(always)]
    pub fn value8(self) -> &'a mut W {
        self.variant(SCALE_A::VALUE8)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u32 & 0x07);
        self.w
    }
}
#[doc = "Switch between up- and downscale of the DAC0 input data values\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MULDIV_A {
    #[doc = "0: downscale = division (shift SCALE positions to the right)"]
    VALUE1 = 0,
    #[doc = "1: upscale = multiplication (shift SCALE positions to the left)"]
    VALUE2 = 1,
}
impl From<MULDIV_A> for bool {
    #[inline(always)]
    fn from(variant: MULDIV_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MULDIV` reader - Switch between up- and downscale of the DAC0 input data values"]
pub struct MULDIV_R(crate::FieldReader<bool, MULDIV_A>);
impl MULDIV_R {
    pub(crate) fn new(bits: bool) -> Self {
        MULDIV_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MULDIV_A {
        match self.bits {
            false => MULDIV_A::VALUE1,
            true => MULDIV_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == MULDIV_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == MULDIV_A::VALUE2
    }
}
impl core::ops::Deref for MULDIV_R {
    type Target = crate::FieldReader<bool, MULDIV_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MULDIV` writer - Switch between up- and downscale of the DAC0 input data values"]
pub struct MULDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> MULDIV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MULDIV_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "downscale = division (shift SCALE positions to the right)"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(MULDIV_A::VALUE1)
    }
    #[doc = "upscale = multiplication (shift SCALE positions to the left)"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(MULDIV_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "Field `OFFS` reader - 8-bit offset value addition"]
pub struct OFFS_R(crate::FieldReader<u8, u8>);
impl OFFS_R {
    pub(crate) fn new(bits: u8) -> Self {
        OFFS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OFFS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OFFS` writer - 8-bit offset value addition"]
pub struct OFFS_W<'a> {
    w: &'a mut W,
}
impl<'a> OFFS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 4)) | ((value as u32 & 0xff) << 4);
        self.w
    }
}
#[doc = "Field `TRIGSEL` reader - Selects one of the eight external trigger sources for DAC0"]
pub struct TRIGSEL_R(crate::FieldReader<u8, u8>);
impl TRIGSEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        TRIGSEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TRIGSEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TRIGSEL` writer - Selects one of the eight external trigger sources for DAC0"]
pub struct TRIGSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> TRIGSEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 12)) | ((value as u32 & 0x07) << 12);
        self.w
    }
}
#[doc = "Switch between independent or simultaneous DAC mode and select the input data register for DAC0 and DAC1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DATMOD_A {
    #[doc = "0: independent data handling - process data from DATA0 register (bits 11:0) to DAC0 and data from DATA1 register (bits 11:0) to DAC1"]
    VALUE1 = 0,
    #[doc = "1: simultaneous data handling - process data from DAC01 register to both DACs (bits 11:0 to DAC0 and bits 23:12 to DAC1)."]
    VALUE2 = 1,
}
impl From<DATMOD_A> for bool {
    #[inline(always)]
    fn from(variant: DATMOD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DATMOD` reader - Switch between independent or simultaneous DAC mode and select the input data register for DAC0 and DAC1"]
pub struct DATMOD_R(crate::FieldReader<bool, DATMOD_A>);
impl DATMOD_R {
    pub(crate) fn new(bits: bool) -> Self {
        DATMOD_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DATMOD_A {
        match self.bits {
            false => DATMOD_A::VALUE1,
            true => DATMOD_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == DATMOD_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == DATMOD_A::VALUE2
    }
}
impl core::ops::Deref for DATMOD_R {
    type Target = crate::FieldReader<bool, DATMOD_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DATMOD` writer - Switch between independent or simultaneous DAC mode and select the input data register for DAC0 and DAC1"]
pub struct DATMOD_W<'a> {
    w: &'a mut W,
}
impl<'a> DATMOD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DATMOD_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "independent data handling - process data from DATA0 register (bits 11:0) to DAC0 and data from DATA1 register (bits 11:0) to DAC1"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(DATMOD_A::VALUE1)
    }
    #[doc = "simultaneous data handling - process data from DAC01 register to both DACs (bits 11:0 to DAC0 and bits 23:12 to DAC1)."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(DATMOD_A::VALUE2)
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
#[doc = "Field `SWTRIG` reader - Software Trigger"]
pub struct SWTRIG_R(crate::FieldReader<bool, bool>);
impl SWTRIG_R {
    pub(crate) fn new(bits: bool) -> Self {
        SWTRIG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SWTRIG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SWTRIG` writer - Software Trigger"]
pub struct SWTRIG_W<'a> {
    w: &'a mut W,
}
impl<'a> SWTRIG_W<'a> {
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
#[doc = "Select the trigger source for channel 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TRIGMOD_A {
    #[doc = "0: internal Trigger (integer divided clock - see FREQ parameter)"]
    VALUE1 = 0,
    #[doc = "1: external Trigger (preselected trigger by TRIGSEL parameter)"]
    VALUE2 = 1,
    #[doc = "2: software Trigger (see SWTRIG parameter)"]
    VALUE3 = 2,
}
impl From<TRIGMOD_A> for u8 {
    #[inline(always)]
    fn from(variant: TRIGMOD_A) -> Self {
        variant as _
    }
}
#[doc = "Field `TRIGMOD` reader - Select the trigger source for channel 0"]
pub struct TRIGMOD_R(crate::FieldReader<u8, TRIGMOD_A>);
impl TRIGMOD_R {
    pub(crate) fn new(bits: u8) -> Self {
        TRIGMOD_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TRIGMOD_A> {
        match self.bits {
            0 => Some(TRIGMOD_A::VALUE1),
            1 => Some(TRIGMOD_A::VALUE2),
            2 => Some(TRIGMOD_A::VALUE3),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == TRIGMOD_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == TRIGMOD_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        **self == TRIGMOD_A::VALUE3
    }
}
impl core::ops::Deref for TRIGMOD_R {
    type Target = crate::FieldReader<u8, TRIGMOD_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TRIGMOD` writer - Select the trigger source for channel 0"]
pub struct TRIGMOD_W<'a> {
    w: &'a mut W,
}
impl<'a> TRIGMOD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TRIGMOD_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "internal Trigger (integer divided clock - see FREQ parameter)"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(TRIGMOD_A::VALUE1)
    }
    #[doc = "external Trigger (preselected trigger by TRIGSEL parameter)"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(TRIGMOD_A::VALUE2)
    }
    #[doc = "software Trigger (see SWTRIG parameter)"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(TRIGMOD_A::VALUE3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 17)) | ((value as u32 & 0x03) << 17);
        self.w
    }
}
#[doc = "Field `ANACFG` reader - DAC0 analog configuration/calibration parameters"]
pub struct ANACFG_R(crate::FieldReader<u8, u8>);
impl ANACFG_R {
    pub(crate) fn new(bits: u8) -> Self {
        ANACFG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ANACFG_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ANACFG` writer - DAC0 analog configuration/calibration parameters"]
pub struct ANACFG_W<'a> {
    w: &'a mut W,
}
impl<'a> ANACFG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 19)) | ((value as u32 & 0x1f) << 19);
        self.w
    }
}
#[doc = "Enable analog DAC for channel 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ANAEN_A {
    #[doc = "0: DAC0 is set to standby (analog output only)"]
    VALUE1 = 0,
    #[doc = "1: enable DAC0 (analog output only)"]
    VALUE2 = 1,
}
impl From<ANAEN_A> for bool {
    #[inline(always)]
    fn from(variant: ANAEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ANAEN` reader - Enable analog DAC for channel 0"]
pub struct ANAEN_R(crate::FieldReader<bool, ANAEN_A>);
impl ANAEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        ANAEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ANAEN_A {
        match self.bits {
            false => ANAEN_A::VALUE1,
            true => ANAEN_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == ANAEN_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == ANAEN_A::VALUE2
    }
}
impl core::ops::Deref for ANAEN_R {
    type Target = crate::FieldReader<bool, ANAEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ANAEN` writer - Enable analog DAC for channel 0"]
pub struct ANAEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ANAEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ANAEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "DAC0 is set to standby (analog output only)"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(ANAEN_A::VALUE1)
    }
    #[doc = "enable DAC0 (analog output only)"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(ANAEN_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | ((value as u32 & 0x01) << 24);
        self.w
    }
}
#[doc = "Field `REFCFGL` reader - Lower 4 band-gap configuration/calibration parameters"]
pub struct REFCFGL_R(crate::FieldReader<u8, u8>);
impl REFCFGL_R {
    pub(crate) fn new(bits: u8) -> Self {
        REFCFGL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REFCFGL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REFCFGL` writer - Lower 4 band-gap configuration/calibration parameters"]
pub struct REFCFGL_W<'a> {
    w: &'a mut W,
}
impl<'a> REFCFGL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 28)) | ((value as u32 & 0x0f) << 28);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Scale value for up- or downscale of the DAC0 input data in steps by the power of 2 (=shift operation)"]
    #[inline(always)]
    pub fn scale(&self) -> SCALE_R {
        SCALE_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bit 3 - Switch between up- and downscale of the DAC0 input data values"]
    #[inline(always)]
    pub fn muldiv(&self) -> MULDIV_R {
        MULDIV_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 4:11 - 8-bit offset value addition"]
    #[inline(always)]
    pub fn offs(&self) -> OFFS_R {
        OFFS_R::new(((self.bits >> 4) & 0xff) as u8)
    }
    #[doc = "Bits 12:14 - Selects one of the eight external trigger sources for DAC0"]
    #[inline(always)]
    pub fn trigsel(&self) -> TRIGSEL_R {
        TRIGSEL_R::new(((self.bits >> 12) & 0x07) as u8)
    }
    #[doc = "Bit 15 - Switch between independent or simultaneous DAC mode and select the input data register for DAC0 and DAC1"]
    #[inline(always)]
    pub fn datmod(&self) -> DATMOD_R {
        DATMOD_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Software Trigger"]
    #[inline(always)]
    pub fn swtrig(&self) -> SWTRIG_R {
        SWTRIG_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bits 17:18 - Select the trigger source for channel 0"]
    #[inline(always)]
    pub fn trigmod(&self) -> TRIGMOD_R {
        TRIGMOD_R::new(((self.bits >> 17) & 0x03) as u8)
    }
    #[doc = "Bits 19:23 - DAC0 analog configuration/calibration parameters"]
    #[inline(always)]
    pub fn anacfg(&self) -> ANACFG_R {
        ANACFG_R::new(((self.bits >> 19) & 0x1f) as u8)
    }
    #[doc = "Bit 24 - Enable analog DAC for channel 0"]
    #[inline(always)]
    pub fn anaen(&self) -> ANAEN_R {
        ANAEN_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bits 28:31 - Lower 4 band-gap configuration/calibration parameters"]
    #[inline(always)]
    pub fn refcfgl(&self) -> REFCFGL_R {
        REFCFGL_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Scale value for up- or downscale of the DAC0 input data in steps by the power of 2 (=shift operation)"]
    #[inline(always)]
    pub fn scale(&mut self) -> SCALE_W {
        SCALE_W { w: self }
    }
    #[doc = "Bit 3 - Switch between up- and downscale of the DAC0 input data values"]
    #[inline(always)]
    pub fn muldiv(&mut self) -> MULDIV_W {
        MULDIV_W { w: self }
    }
    #[doc = "Bits 4:11 - 8-bit offset value addition"]
    #[inline(always)]
    pub fn offs(&mut self) -> OFFS_W {
        OFFS_W { w: self }
    }
    #[doc = "Bits 12:14 - Selects one of the eight external trigger sources for DAC0"]
    #[inline(always)]
    pub fn trigsel(&mut self) -> TRIGSEL_W {
        TRIGSEL_W { w: self }
    }
    #[doc = "Bit 15 - Switch between independent or simultaneous DAC mode and select the input data register for DAC0 and DAC1"]
    #[inline(always)]
    pub fn datmod(&mut self) -> DATMOD_W {
        DATMOD_W { w: self }
    }
    #[doc = "Bit 16 - Software Trigger"]
    #[inline(always)]
    pub fn swtrig(&mut self) -> SWTRIG_W {
        SWTRIG_W { w: self }
    }
    #[doc = "Bits 17:18 - Select the trigger source for channel 0"]
    #[inline(always)]
    pub fn trigmod(&mut self) -> TRIGMOD_W {
        TRIGMOD_W { w: self }
    }
    #[doc = "Bits 19:23 - DAC0 analog configuration/calibration parameters"]
    #[inline(always)]
    pub fn anacfg(&mut self) -> ANACFG_W {
        ANACFG_W { w: self }
    }
    #[doc = "Bit 24 - Enable analog DAC for channel 0"]
    #[inline(always)]
    pub fn anaen(&mut self) -> ANAEN_W {
        ANAEN_W { w: self }
    }
    #[doc = "Bits 28:31 - Lower 4 band-gap configuration/calibration parameters"]
    #[inline(always)]
    pub fn refcfgl(&mut self) -> REFCFGL_W {
        REFCFGL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DAC0 Configuration Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dac0cfg1](index.html) module"]
pub struct DAC0CFG1_SPEC;
impl crate::RegisterSpec for DAC0CFG1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dac0cfg1::R](R) reader structure"]
impl crate::Readable for DAC0CFG1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dac0cfg1::W](W) writer structure"]
impl crate::Writable for DAC0CFG1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DAC0CFG1 to value 0"]
impl crate::Resettable for DAC0CFG1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
