#[doc = "Register `PCR_SSCMode` reader"]
pub struct R(crate::R<PCR_SSCMODE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PCR_SSCMODE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PCR_SSCMODE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PCR_SSCMODE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PCR_SSCMode` writer"]
pub struct W(crate::W<PCR_SSCMODE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PCR_SSCMODE_SPEC>;
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
impl From<crate::W<PCR_SSCMODE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PCR_SSCMODE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "MSLS Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MSLSEN_A {
    #[doc = "0: The MSLS generation is disabled (MSLS = 0). This is the setting for SSC slave mode."]
    VALUE1 = 0,
    #[doc = "1: The MSLS generation is enabled. This is the setting for SSC master mode."]
    VALUE2 = 1,
}
impl From<MSLSEN_A> for bool {
    #[inline(always)]
    fn from(variant: MSLSEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MSLSEN` reader - MSLS Enable"]
pub struct MSLSEN_R(crate::FieldReader<bool, MSLSEN_A>);
impl MSLSEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        MSLSEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MSLSEN_A {
        match self.bits {
            false => MSLSEN_A::VALUE1,
            true => MSLSEN_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == MSLSEN_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == MSLSEN_A::VALUE2
    }
}
impl core::ops::Deref for MSLSEN_R {
    type Target = crate::FieldReader<bool, MSLSEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MSLSEN` writer - MSLS Enable"]
pub struct MSLSEN_W<'a> {
    w: &'a mut W,
}
impl<'a> MSLSEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MSLSEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The MSLS generation is disabled (MSLS = 0). This is the setting for SSC slave mode."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(MSLSEN_A::VALUE1)
    }
    #[doc = "The MSLS generation is enabled. This is the setting for SSC master mode."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(MSLSEN_A::VALUE2)
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "Select Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SELCTR_A {
    #[doc = "0: The coded select mode is enabled."]
    VALUE1 = 0,
    #[doc = "1: The direct select mode is enabled."]
    VALUE2 = 1,
}
impl From<SELCTR_A> for bool {
    #[inline(always)]
    fn from(variant: SELCTR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SELCTR` reader - Select Control"]
pub struct SELCTR_R(crate::FieldReader<bool, SELCTR_A>);
impl SELCTR_R {
    pub(crate) fn new(bits: bool) -> Self {
        SELCTR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SELCTR_A {
        match self.bits {
            false => SELCTR_A::VALUE1,
            true => SELCTR_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == SELCTR_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == SELCTR_A::VALUE2
    }
}
impl core::ops::Deref for SELCTR_R {
    type Target = crate::FieldReader<bool, SELCTR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SELCTR` writer - Select Control"]
pub struct SELCTR_W<'a> {
    w: &'a mut W,
}
impl<'a> SELCTR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SELCTR_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The coded select mode is enabled."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(SELCTR_A::VALUE1)
    }
    #[doc = "The direct select mode is enabled."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(SELCTR_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Select Inversion\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SELINV_A {
    #[doc = "0: The SELO outputs have the same polarity as the MSLS signal (active high)."]
    VALUE1 = 0,
    #[doc = "1: The SELO outputs have the inverted polarity to the MSLS signal (active low)."]
    VALUE2 = 1,
}
impl From<SELINV_A> for bool {
    #[inline(always)]
    fn from(variant: SELINV_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SELINV` reader - Select Inversion"]
pub struct SELINV_R(crate::FieldReader<bool, SELINV_A>);
impl SELINV_R {
    pub(crate) fn new(bits: bool) -> Self {
        SELINV_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SELINV_A {
        match self.bits {
            false => SELINV_A::VALUE1,
            true => SELINV_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == SELINV_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == SELINV_A::VALUE2
    }
}
impl core::ops::Deref for SELINV_R {
    type Target = crate::FieldReader<bool, SELINV_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SELINV` writer - Select Inversion"]
pub struct SELINV_W<'a> {
    w: &'a mut W,
}
impl<'a> SELINV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SELINV_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The SELO outputs have the same polarity as the MSLS signal (active high)."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(SELINV_A::VALUE1)
    }
    #[doc = "The SELO outputs have the inverted polarity to the MSLS signal (active low)."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(SELINV_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Frame End Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FEM_A {
    #[doc = "0: The current data frame is considered as finished when the last bit of a data word has been sent out and the transmit buffer TBUF does not contain new data (TDV = 0)."]
    VALUE1 = 0,
    #[doc = "1: The MSLS signal is kept active also while no new data is available and no other end of frame condition is reached. In this case, the software can accept delays in delivering the data without automatic deactivation of MSLS in multi-word data frames."]
    VALUE2 = 1,
}
impl From<FEM_A> for bool {
    #[inline(always)]
    fn from(variant: FEM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FEM` reader - Frame End Mode"]
pub struct FEM_R(crate::FieldReader<bool, FEM_A>);
impl FEM_R {
    pub(crate) fn new(bits: bool) -> Self {
        FEM_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FEM_A {
        match self.bits {
            false => FEM_A::VALUE1,
            true => FEM_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == FEM_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == FEM_A::VALUE2
    }
}
impl core::ops::Deref for FEM_R {
    type Target = crate::FieldReader<bool, FEM_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FEM` writer - Frame End Mode"]
pub struct FEM_W<'a> {
    w: &'a mut W,
}
impl<'a> FEM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FEM_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The current data frame is considered as finished when the last bit of a data word has been sent out and the transmit buffer TBUF does not contain new data (TDV = 0)."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(FEM_A::VALUE1)
    }
    #[doc = "The MSLS signal is kept active also while no new data is available and no other end of frame condition is reached. In this case, the software can accept delays in delivering the data without automatic deactivation of MSLS in multi-word data frames."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(FEM_A::VALUE2)
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
#[doc = "Input Frequency Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CTQSEL1_A {
    #[doc = "0: fCTQIN = fPDIV"]
    VALUE1 = 0,
    #[doc = "1: fCTQIN = fPPP"]
    VALUE2 = 1,
    #[doc = "2: fCTQIN = fSCLK"]
    VALUE3 = 2,
    #[doc = "3: fCTQIN = fMCLK"]
    VALUE4 = 3,
}
impl From<CTQSEL1_A> for u8 {
    #[inline(always)]
    fn from(variant: CTQSEL1_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CTQSEL1` reader - Input Frequency Selection"]
pub struct CTQSEL1_R(crate::FieldReader<u8, CTQSEL1_A>);
impl CTQSEL1_R {
    pub(crate) fn new(bits: u8) -> Self {
        CTQSEL1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTQSEL1_A {
        match self.bits {
            0 => CTQSEL1_A::VALUE1,
            1 => CTQSEL1_A::VALUE2,
            2 => CTQSEL1_A::VALUE3,
            3 => CTQSEL1_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == CTQSEL1_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == CTQSEL1_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        **self == CTQSEL1_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        **self == CTQSEL1_A::VALUE4
    }
}
impl core::ops::Deref for CTQSEL1_R {
    type Target = crate::FieldReader<u8, CTQSEL1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CTQSEL1` writer - Input Frequency Selection"]
pub struct CTQSEL1_W<'a> {
    w: &'a mut W,
}
impl<'a> CTQSEL1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CTQSEL1_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "fCTQIN = fPDIV"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CTQSEL1_A::VALUE1)
    }
    #[doc = "fCTQIN = fPPP"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CTQSEL1_A::VALUE2)
    }
    #[doc = "fCTQIN = fSCLK"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(CTQSEL1_A::VALUE3)
    }
    #[doc = "fCTQIN = fMCLK"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(CTQSEL1_A::VALUE4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | ((value as u32 & 0x03) << 4);
        self.w
    }
}
#[doc = "Field `PCTQ1` reader - Divider Factor PCTQ1 for Tiw and Tnf"]
pub struct PCTQ1_R(crate::FieldReader<u8, u8>);
impl PCTQ1_R {
    pub(crate) fn new(bits: u8) -> Self {
        PCTQ1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PCTQ1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PCTQ1` writer - Divider Factor PCTQ1 for Tiw and Tnf"]
pub struct PCTQ1_W<'a> {
    w: &'a mut W,
}
impl<'a> PCTQ1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | ((value as u32 & 0x03) << 6);
        self.w
    }
}
#[doc = "Field `DCTQ1` reader - Divider Factor DCTQ1 for Tiw and Tnf"]
pub struct DCTQ1_R(crate::FieldReader<u8, u8>);
impl DCTQ1_R {
    pub(crate) fn new(bits: u8) -> Self {
        DCTQ1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DCTQ1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DCTQ1` writer - Divider Factor DCTQ1 for Tiw and Tnf"]
pub struct DCTQ1_W<'a> {
    w: &'a mut W,
}
impl<'a> DCTQ1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 8)) | ((value as u32 & 0x1f) << 8);
        self.w
    }
}
#[doc = "Parity Error Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PARIEN_A {
    #[doc = "0: A protocol interrupt is not generated with the detection of a parity error."]
    VALUE1 = 0,
    #[doc = "1: A protocol interrupt is generated with the detection of a parity error."]
    VALUE2 = 1,
}
impl From<PARIEN_A> for bool {
    #[inline(always)]
    fn from(variant: PARIEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PARIEN` reader - Parity Error Interrupt Enable"]
pub struct PARIEN_R(crate::FieldReader<bool, PARIEN_A>);
impl PARIEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        PARIEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PARIEN_A {
        match self.bits {
            false => PARIEN_A::VALUE1,
            true => PARIEN_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == PARIEN_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == PARIEN_A::VALUE2
    }
}
impl core::ops::Deref for PARIEN_R {
    type Target = crate::FieldReader<bool, PARIEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PARIEN` writer - Parity Error Interrupt Enable"]
pub struct PARIEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PARIEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PARIEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "A protocol interrupt is not generated with the detection of a parity error."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(PARIEN_A::VALUE1)
    }
    #[doc = "A protocol interrupt is generated with the detection of a parity error."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(PARIEN_A::VALUE2)
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
#[doc = "MSLS Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MSLSIEN_A {
    #[doc = "0: A protocol interrupt is not generated if a change of signal MSLS is detected."]
    VALUE1 = 0,
    #[doc = "1: A protocol interrupt is generated if a change of signal MSLS is detected."]
    VALUE2 = 1,
}
impl From<MSLSIEN_A> for bool {
    #[inline(always)]
    fn from(variant: MSLSIEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MSLSIEN` reader - MSLS Interrupt Enable"]
pub struct MSLSIEN_R(crate::FieldReader<bool, MSLSIEN_A>);
impl MSLSIEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        MSLSIEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MSLSIEN_A {
        match self.bits {
            false => MSLSIEN_A::VALUE1,
            true => MSLSIEN_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == MSLSIEN_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == MSLSIEN_A::VALUE2
    }
}
impl core::ops::Deref for MSLSIEN_R {
    type Target = crate::FieldReader<bool, MSLSIEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MSLSIEN` writer - MSLS Interrupt Enable"]
pub struct MSLSIEN_W<'a> {
    w: &'a mut W,
}
impl<'a> MSLSIEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MSLSIEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "A protocol interrupt is not generated if a change of signal MSLS is detected."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(MSLSIEN_A::VALUE1)
    }
    #[doc = "A protocol interrupt is generated if a change of signal MSLS is detected."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(MSLSIEN_A::VALUE2)
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
#[doc = "DX2T Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DX2TIEN_A {
    #[doc = "0: A protocol interrupt is not generated if DX2T is activated."]
    VALUE1 = 0,
    #[doc = "1: A protocol interrupt is generated if DX2T is activated."]
    VALUE2 = 1,
}
impl From<DX2TIEN_A> for bool {
    #[inline(always)]
    fn from(variant: DX2TIEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DX2TIEN` reader - DX2T Interrupt Enable"]
pub struct DX2TIEN_R(crate::FieldReader<bool, DX2TIEN_A>);
impl DX2TIEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        DX2TIEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DX2TIEN_A {
        match self.bits {
            false => DX2TIEN_A::VALUE1,
            true => DX2TIEN_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == DX2TIEN_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == DX2TIEN_A::VALUE2
    }
}
impl core::ops::Deref for DX2TIEN_R {
    type Target = crate::FieldReader<bool, DX2TIEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DX2TIEN` writer - DX2T Interrupt Enable"]
pub struct DX2TIEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DX2TIEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DX2TIEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "A protocol interrupt is not generated if DX2T is activated."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(DX2TIEN_A::VALUE1)
    }
    #[doc = "A protocol interrupt is generated if DX2T is activated."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(DX2TIEN_A::VALUE2)
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
#[doc = "Select Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SELO_A {
    #[doc = "0: The corresponding SELOx line cannot be activated."]
    VALUE1 = 0,
    #[doc = "1: The corresponding SELOx line can be activated (according to the mode selected by SELCTR)."]
    VALUE2 = 1,
}
impl From<SELO_A> for u8 {
    #[inline(always)]
    fn from(variant: SELO_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SELO` reader - Select Output"]
pub struct SELO_R(crate::FieldReader<u8, SELO_A>);
impl SELO_R {
    pub(crate) fn new(bits: u8) -> Self {
        SELO_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SELO_A> {
        match self.bits {
            0 => Some(SELO_A::VALUE1),
            1 => Some(SELO_A::VALUE2),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == SELO_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == SELO_A::VALUE2
    }
}
impl core::ops::Deref for SELO_R {
    type Target = crate::FieldReader<u8, SELO_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SELO` writer - Select Output"]
pub struct SELO_W<'a> {
    w: &'a mut W,
}
impl<'a> SELO_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SELO_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "The corresponding SELOx line cannot be activated."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(SELO_A::VALUE1)
    }
    #[doc = "The corresponding SELOx line can be activated (according to the mode selected by SELCTR)."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(SELO_A::VALUE2)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | ((value as u32 & 0xff) << 16);
        self.w
    }
}
#[doc = "Enable Inter-Word Delay Tiw\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIWEN_A {
    #[doc = "0: No delay between data words of the same frame."]
    VALUE1 = 0,
    #[doc = "1: The inter-word delay Tiw is enabled and introduced between data words of the same frame."]
    VALUE2 = 1,
}
impl From<TIWEN_A> for bool {
    #[inline(always)]
    fn from(variant: TIWEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIWEN` reader - Enable Inter-Word Delay Tiw"]
pub struct TIWEN_R(crate::FieldReader<bool, TIWEN_A>);
impl TIWEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        TIWEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIWEN_A {
        match self.bits {
            false => TIWEN_A::VALUE1,
            true => TIWEN_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == TIWEN_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == TIWEN_A::VALUE2
    }
}
impl core::ops::Deref for TIWEN_R {
    type Target = crate::FieldReader<bool, TIWEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIWEN` writer - Enable Inter-Word Delay Tiw"]
pub struct TIWEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TIWEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIWEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No delay between data words of the same frame."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(TIWEN_A::VALUE1)
    }
    #[doc = "The inter-word delay Tiw is enabled and introduced between data words of the same frame."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(TIWEN_A::VALUE2)
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
#[doc = "Slave Mode Clock Phase Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SLPHSEL_A {
    #[doc = "0: Data bits are shifted out with the leading edge of the shift clock signal and latched in with the trailing edge."]
    VALUE1 = 0,
    #[doc = "1: The first data bit is shifted out when the data shift unit receives a low to high transition from the DX2 stage. Subsequent bits are shifted out with the trailing edge of the shift clock signal. Data bits are always latched in with the leading edge."]
    VALUE2 = 1,
}
impl From<SLPHSEL_A> for bool {
    #[inline(always)]
    fn from(variant: SLPHSEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SLPHSEL` reader - Slave Mode Clock Phase Select"]
pub struct SLPHSEL_R(crate::FieldReader<bool, SLPHSEL_A>);
impl SLPHSEL_R {
    pub(crate) fn new(bits: bool) -> Self {
        SLPHSEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SLPHSEL_A {
        match self.bits {
            false => SLPHSEL_A::VALUE1,
            true => SLPHSEL_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == SLPHSEL_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == SLPHSEL_A::VALUE2
    }
}
impl core::ops::Deref for SLPHSEL_R {
    type Target = crate::FieldReader<bool, SLPHSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLPHSEL` writer - Slave Mode Clock Phase Select"]
pub struct SLPHSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SLPHSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SLPHSEL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Data bits are shifted out with the leading edge of the shift clock signal and latched in with the trailing edge."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(SLPHSEL_A::VALUE1)
    }
    #[doc = "The first data bit is shifted out when the data shift unit receives a low to high transition from the DX2 stage. Subsequent bits are shifted out with the trailing edge of the shift clock signal. Data bits are always latched in with the leading edge."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(SLPHSEL_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | ((value as u32 & 0x01) << 25);
        self.w
    }
}
#[doc = "Master Clock Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MCLK_A {
    #[doc = "0: The MCLK generation is disabled and output MCLK = 0."]
    VALUE1 = 0,
    #[doc = "1: The MCLK generation is enabled."]
    VALUE2 = 1,
}
impl From<MCLK_A> for bool {
    #[inline(always)]
    fn from(variant: MCLK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MCLK` reader - Master Clock Enable"]
pub struct MCLK_R(crate::FieldReader<bool, MCLK_A>);
impl MCLK_R {
    pub(crate) fn new(bits: bool) -> Self {
        MCLK_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MCLK_A {
        match self.bits {
            false => MCLK_A::VALUE1,
            true => MCLK_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == MCLK_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == MCLK_A::VALUE2
    }
}
impl core::ops::Deref for MCLK_R {
    type Target = crate::FieldReader<bool, MCLK_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MCLK` writer - Master Clock Enable"]
pub struct MCLK_W<'a> {
    w: &'a mut W,
}
impl<'a> MCLK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MCLK_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The MCLK generation is disabled and output MCLK = 0."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(MCLK_A::VALUE1)
    }
    #[doc = "The MCLK generation is enabled."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(MCLK_A::VALUE2)
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
    #[doc = "Bit 0 - MSLS Enable"]
    #[inline(always)]
    pub fn mslsen(&self) -> MSLSEN_R {
        MSLSEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Select Control"]
    #[inline(always)]
    pub fn selctr(&self) -> SELCTR_R {
        SELCTR_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Select Inversion"]
    #[inline(always)]
    pub fn selinv(&self) -> SELINV_R {
        SELINV_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Frame End Mode"]
    #[inline(always)]
    pub fn fem(&self) -> FEM_R {
        FEM_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 4:5 - Input Frequency Selection"]
    #[inline(always)]
    pub fn ctqsel1(&self) -> CTQSEL1_R {
        CTQSEL1_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 6:7 - Divider Factor PCTQ1 for Tiw and Tnf"]
    #[inline(always)]
    pub fn pctq1(&self) -> PCTQ1_R {
        PCTQ1_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 8:12 - Divider Factor DCTQ1 for Tiw and Tnf"]
    #[inline(always)]
    pub fn dctq1(&self) -> DCTQ1_R {
        DCTQ1_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bit 13 - Parity Error Interrupt Enable"]
    #[inline(always)]
    pub fn parien(&self) -> PARIEN_R {
        PARIEN_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - MSLS Interrupt Enable"]
    #[inline(always)]
    pub fn mslsien(&self) -> MSLSIEN_R {
        MSLSIEN_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - DX2T Interrupt Enable"]
    #[inline(always)]
    pub fn dx2tien(&self) -> DX2TIEN_R {
        DX2TIEN_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 16:23 - Select Output"]
    #[inline(always)]
    pub fn selo(&self) -> SELO_R {
        SELO_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 24 - Enable Inter-Word Delay Tiw"]
    #[inline(always)]
    pub fn tiwen(&self) -> TIWEN_R {
        TIWEN_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Slave Mode Clock Phase Select"]
    #[inline(always)]
    pub fn slphsel(&self) -> SLPHSEL_R {
        SLPHSEL_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Master Clock Enable"]
    #[inline(always)]
    pub fn mclk(&self) -> MCLK_R {
        MCLK_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - MSLS Enable"]
    #[inline(always)]
    pub fn mslsen(&mut self) -> MSLSEN_W {
        MSLSEN_W { w: self }
    }
    #[doc = "Bit 1 - Select Control"]
    #[inline(always)]
    pub fn selctr(&mut self) -> SELCTR_W {
        SELCTR_W { w: self }
    }
    #[doc = "Bit 2 - Select Inversion"]
    #[inline(always)]
    pub fn selinv(&mut self) -> SELINV_W {
        SELINV_W { w: self }
    }
    #[doc = "Bit 3 - Frame End Mode"]
    #[inline(always)]
    pub fn fem(&mut self) -> FEM_W {
        FEM_W { w: self }
    }
    #[doc = "Bits 4:5 - Input Frequency Selection"]
    #[inline(always)]
    pub fn ctqsel1(&mut self) -> CTQSEL1_W {
        CTQSEL1_W { w: self }
    }
    #[doc = "Bits 6:7 - Divider Factor PCTQ1 for Tiw and Tnf"]
    #[inline(always)]
    pub fn pctq1(&mut self) -> PCTQ1_W {
        PCTQ1_W { w: self }
    }
    #[doc = "Bits 8:12 - Divider Factor DCTQ1 for Tiw and Tnf"]
    #[inline(always)]
    pub fn dctq1(&mut self) -> DCTQ1_W {
        DCTQ1_W { w: self }
    }
    #[doc = "Bit 13 - Parity Error Interrupt Enable"]
    #[inline(always)]
    pub fn parien(&mut self) -> PARIEN_W {
        PARIEN_W { w: self }
    }
    #[doc = "Bit 14 - MSLS Interrupt Enable"]
    #[inline(always)]
    pub fn mslsien(&mut self) -> MSLSIEN_W {
        MSLSIEN_W { w: self }
    }
    #[doc = "Bit 15 - DX2T Interrupt Enable"]
    #[inline(always)]
    pub fn dx2tien(&mut self) -> DX2TIEN_W {
        DX2TIEN_W { w: self }
    }
    #[doc = "Bits 16:23 - Select Output"]
    #[inline(always)]
    pub fn selo(&mut self) -> SELO_W {
        SELO_W { w: self }
    }
    #[doc = "Bit 24 - Enable Inter-Word Delay Tiw"]
    #[inline(always)]
    pub fn tiwen(&mut self) -> TIWEN_W {
        TIWEN_W { w: self }
    }
    #[doc = "Bit 25 - Slave Mode Clock Phase Select"]
    #[inline(always)]
    pub fn slphsel(&mut self) -> SLPHSEL_W {
        SLPHSEL_W { w: self }
    }
    #[doc = "Bit 31 - Master Clock Enable"]
    #[inline(always)]
    pub fn mclk(&mut self) -> MCLK_W {
        MCLK_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Protocol Control Register \\[SSC Mode\\]\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pcr_sscmode](index.html) module"]
pub struct PCR_SSCMODE_SPEC;
impl crate::RegisterSpec for PCR_SSCMODE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pcr_sscmode::R](R) reader structure"]
impl crate::Readable for PCR_SSCMODE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pcr_sscmode::W](W) writer structure"]
impl crate::Writable for PCR_SSCMODE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PCR_SSCMode to value 0"]
impl crate::Resettable for PCR_SSCMODE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
