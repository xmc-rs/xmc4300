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
#[doc = "Field `MSLSEN` reader - MSLS Enable"]
pub type MSLSEN_R = crate::BitReader<MSLSEN_A>;
#[doc = "MSLS Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl MSLSEN_R {
    #[doc = "Get enumerated values variant"]
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
        *self == MSLSEN_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == MSLSEN_A::VALUE2
    }
}
#[doc = "Field `MSLSEN` writer - MSLS Enable"]
pub type MSLSEN_W<'a, const O: u8> = crate::BitWriter<'a, PCR_SSCMODE_SPEC, O, MSLSEN_A>;
impl<'a, const O: u8> MSLSEN_W<'a, O> {
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
}
#[doc = "Field `SELCTR` reader - Select Control"]
pub type SELCTR_R = crate::BitReader<SELCTR_A>;
#[doc = "Select Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl SELCTR_R {
    #[doc = "Get enumerated values variant"]
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
        *self == SELCTR_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SELCTR_A::VALUE2
    }
}
#[doc = "Field `SELCTR` writer - Select Control"]
pub type SELCTR_W<'a, const O: u8> = crate::BitWriter<'a, PCR_SSCMODE_SPEC, O, SELCTR_A>;
impl<'a, const O: u8> SELCTR_W<'a, O> {
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
}
#[doc = "Field `SELINV` reader - Select Inversion"]
pub type SELINV_R = crate::BitReader<SELINV_A>;
#[doc = "Select Inversion\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl SELINV_R {
    #[doc = "Get enumerated values variant"]
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
        *self == SELINV_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SELINV_A::VALUE2
    }
}
#[doc = "Field `SELINV` writer - Select Inversion"]
pub type SELINV_W<'a, const O: u8> = crate::BitWriter<'a, PCR_SSCMODE_SPEC, O, SELINV_A>;
impl<'a, const O: u8> SELINV_W<'a, O> {
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
}
#[doc = "Field `FEM` reader - Frame End Mode"]
pub type FEM_R = crate::BitReader<FEM_A>;
#[doc = "Frame End Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl FEM_R {
    #[doc = "Get enumerated values variant"]
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
        *self == FEM_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == FEM_A::VALUE2
    }
}
#[doc = "Field `FEM` writer - Frame End Mode"]
pub type FEM_W<'a, const O: u8> = crate::BitWriter<'a, PCR_SSCMODE_SPEC, O, FEM_A>;
impl<'a, const O: u8> FEM_W<'a, O> {
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
}
#[doc = "Field `CTQSEL1` reader - Input Frequency Selection"]
pub type CTQSEL1_R = crate::FieldReader<CTQSEL1_A>;
#[doc = "Input Frequency Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl crate::FieldSpec for CTQSEL1_A {
    type Ux = u8;
}
impl CTQSEL1_R {
    #[doc = "Get enumerated values variant"]
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
        *self == CTQSEL1_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CTQSEL1_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == CTQSEL1_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == CTQSEL1_A::VALUE4
    }
}
#[doc = "Field `CTQSEL1` writer - Input Frequency Selection"]
pub type CTQSEL1_W<'a, const O: u8> = crate::FieldWriterSafe<'a, PCR_SSCMODE_SPEC, 2, O, CTQSEL1_A>;
impl<'a, const O: u8> CTQSEL1_W<'a, O> {
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
}
#[doc = "Field `PCTQ1` reader - Divider Factor PCTQ1 for Tiw and Tnf"]
pub type PCTQ1_R = crate::FieldReader;
#[doc = "Field `PCTQ1` writer - Divider Factor PCTQ1 for Tiw and Tnf"]
pub type PCTQ1_W<'a, const O: u8> = crate::FieldWriter<'a, PCR_SSCMODE_SPEC, 2, O>;
#[doc = "Field `DCTQ1` reader - Divider Factor DCTQ1 for Tiw and Tnf"]
pub type DCTQ1_R = crate::FieldReader;
#[doc = "Field `DCTQ1` writer - Divider Factor DCTQ1 for Tiw and Tnf"]
pub type DCTQ1_W<'a, const O: u8> = crate::FieldWriter<'a, PCR_SSCMODE_SPEC, 5, O>;
#[doc = "Field `PARIEN` reader - Parity Error Interrupt Enable"]
pub type PARIEN_R = crate::BitReader<PARIEN_A>;
#[doc = "Parity Error Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl PARIEN_R {
    #[doc = "Get enumerated values variant"]
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
        *self == PARIEN_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PARIEN_A::VALUE2
    }
}
#[doc = "Field `PARIEN` writer - Parity Error Interrupt Enable"]
pub type PARIEN_W<'a, const O: u8> = crate::BitWriter<'a, PCR_SSCMODE_SPEC, O, PARIEN_A>;
impl<'a, const O: u8> PARIEN_W<'a, O> {
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
}
#[doc = "Field `MSLSIEN` reader - MSLS Interrupt Enable"]
pub type MSLSIEN_R = crate::BitReader<MSLSIEN_A>;
#[doc = "MSLS Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl MSLSIEN_R {
    #[doc = "Get enumerated values variant"]
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
        *self == MSLSIEN_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == MSLSIEN_A::VALUE2
    }
}
#[doc = "Field `MSLSIEN` writer - MSLS Interrupt Enable"]
pub type MSLSIEN_W<'a, const O: u8> = crate::BitWriter<'a, PCR_SSCMODE_SPEC, O, MSLSIEN_A>;
impl<'a, const O: u8> MSLSIEN_W<'a, O> {
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
}
#[doc = "Field `DX2TIEN` reader - DX2T Interrupt Enable"]
pub type DX2TIEN_R = crate::BitReader<DX2TIEN_A>;
#[doc = "DX2T Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl DX2TIEN_R {
    #[doc = "Get enumerated values variant"]
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
        *self == DX2TIEN_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == DX2TIEN_A::VALUE2
    }
}
#[doc = "Field `DX2TIEN` writer - DX2T Interrupt Enable"]
pub type DX2TIEN_W<'a, const O: u8> = crate::BitWriter<'a, PCR_SSCMODE_SPEC, O, DX2TIEN_A>;
impl<'a, const O: u8> DX2TIEN_W<'a, O> {
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
}
#[doc = "Field `SELO` reader - Select Output"]
pub type SELO_R = crate::FieldReader<SELO_A>;
#[doc = "Select Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl crate::FieldSpec for SELO_A {
    type Ux = u8;
}
impl SELO_R {
    #[doc = "Get enumerated values variant"]
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
        *self == SELO_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SELO_A::VALUE2
    }
}
#[doc = "Field `SELO` writer - Select Output"]
pub type SELO_W<'a, const O: u8> = crate::FieldWriter<'a, PCR_SSCMODE_SPEC, 8, O, SELO_A>;
impl<'a, const O: u8> SELO_W<'a, O> {
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
}
#[doc = "Field `TIWEN` reader - Enable Inter-Word Delay Tiw"]
pub type TIWEN_R = crate::BitReader<TIWEN_A>;
#[doc = "Enable Inter-Word Delay Tiw\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl TIWEN_R {
    #[doc = "Get enumerated values variant"]
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
        *self == TIWEN_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == TIWEN_A::VALUE2
    }
}
#[doc = "Field `TIWEN` writer - Enable Inter-Word Delay Tiw"]
pub type TIWEN_W<'a, const O: u8> = crate::BitWriter<'a, PCR_SSCMODE_SPEC, O, TIWEN_A>;
impl<'a, const O: u8> TIWEN_W<'a, O> {
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
}
#[doc = "Field `SLPHSEL` reader - Slave Mode Clock Phase Select"]
pub type SLPHSEL_R = crate::BitReader<SLPHSEL_A>;
#[doc = "Slave Mode Clock Phase Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl SLPHSEL_R {
    #[doc = "Get enumerated values variant"]
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
        *self == SLPHSEL_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SLPHSEL_A::VALUE2
    }
}
#[doc = "Field `SLPHSEL` writer - Slave Mode Clock Phase Select"]
pub type SLPHSEL_W<'a, const O: u8> = crate::BitWriter<'a, PCR_SSCMODE_SPEC, O, SLPHSEL_A>;
impl<'a, const O: u8> SLPHSEL_W<'a, O> {
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
}
#[doc = "Field `MCLK` reader - Master Clock Enable"]
pub type MCLK_R = crate::BitReader<MCLK_A>;
#[doc = "Master Clock Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl MCLK_R {
    #[doc = "Get enumerated values variant"]
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
        *self == MCLK_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == MCLK_A::VALUE2
    }
}
#[doc = "Field `MCLK` writer - Master Clock Enable"]
pub type MCLK_W<'a, const O: u8> = crate::BitWriter<'a, PCR_SSCMODE_SPEC, O, MCLK_A>;
impl<'a, const O: u8> MCLK_W<'a, O> {
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
}
impl R {
    #[doc = "Bit 0 - MSLS Enable"]
    #[inline(always)]
    pub fn mslsen(&self) -> MSLSEN_R {
        MSLSEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Select Control"]
    #[inline(always)]
    pub fn selctr(&self) -> SELCTR_R {
        SELCTR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Select Inversion"]
    #[inline(always)]
    pub fn selinv(&self) -> SELINV_R {
        SELINV_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Frame End Mode"]
    #[inline(always)]
    pub fn fem(&self) -> FEM_R {
        FEM_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - Input Frequency Selection"]
    #[inline(always)]
    pub fn ctqsel1(&self) -> CTQSEL1_R {
        CTQSEL1_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Divider Factor PCTQ1 for Tiw and Tnf"]
    #[inline(always)]
    pub fn pctq1(&self) -> PCTQ1_R {
        PCTQ1_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:12 - Divider Factor DCTQ1 for Tiw and Tnf"]
    #[inline(always)]
    pub fn dctq1(&self) -> DCTQ1_R {
        DCTQ1_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bit 13 - Parity Error Interrupt Enable"]
    #[inline(always)]
    pub fn parien(&self) -> PARIEN_R {
        PARIEN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - MSLS Interrupt Enable"]
    #[inline(always)]
    pub fn mslsien(&self) -> MSLSIEN_R {
        MSLSIEN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - DX2T Interrupt Enable"]
    #[inline(always)]
    pub fn dx2tien(&self) -> DX2TIEN_R {
        DX2TIEN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:23 - Select Output"]
    #[inline(always)]
    pub fn selo(&self) -> SELO_R {
        SELO_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 24 - Enable Inter-Word Delay Tiw"]
    #[inline(always)]
    pub fn tiwen(&self) -> TIWEN_R {
        TIWEN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Slave Mode Clock Phase Select"]
    #[inline(always)]
    pub fn slphsel(&self) -> SLPHSEL_R {
        SLPHSEL_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 31 - Master Clock Enable"]
    #[inline(always)]
    pub fn mclk(&self) -> MCLK_R {
        MCLK_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - MSLS Enable"]
    #[inline(always)]
    #[must_use]
    pub fn mslsen(&mut self) -> MSLSEN_W<0> {
        MSLSEN_W::new(self)
    }
    #[doc = "Bit 1 - Select Control"]
    #[inline(always)]
    #[must_use]
    pub fn selctr(&mut self) -> SELCTR_W<1> {
        SELCTR_W::new(self)
    }
    #[doc = "Bit 2 - Select Inversion"]
    #[inline(always)]
    #[must_use]
    pub fn selinv(&mut self) -> SELINV_W<2> {
        SELINV_W::new(self)
    }
    #[doc = "Bit 3 - Frame End Mode"]
    #[inline(always)]
    #[must_use]
    pub fn fem(&mut self) -> FEM_W<3> {
        FEM_W::new(self)
    }
    #[doc = "Bits 4:5 - Input Frequency Selection"]
    #[inline(always)]
    #[must_use]
    pub fn ctqsel1(&mut self) -> CTQSEL1_W<4> {
        CTQSEL1_W::new(self)
    }
    #[doc = "Bits 6:7 - Divider Factor PCTQ1 for Tiw and Tnf"]
    #[inline(always)]
    #[must_use]
    pub fn pctq1(&mut self) -> PCTQ1_W<6> {
        PCTQ1_W::new(self)
    }
    #[doc = "Bits 8:12 - Divider Factor DCTQ1 for Tiw and Tnf"]
    #[inline(always)]
    #[must_use]
    pub fn dctq1(&mut self) -> DCTQ1_W<8> {
        DCTQ1_W::new(self)
    }
    #[doc = "Bit 13 - Parity Error Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn parien(&mut self) -> PARIEN_W<13> {
        PARIEN_W::new(self)
    }
    #[doc = "Bit 14 - MSLS Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn mslsien(&mut self) -> MSLSIEN_W<14> {
        MSLSIEN_W::new(self)
    }
    #[doc = "Bit 15 - DX2T Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dx2tien(&mut self) -> DX2TIEN_W<15> {
        DX2TIEN_W::new(self)
    }
    #[doc = "Bits 16:23 - Select Output"]
    #[inline(always)]
    #[must_use]
    pub fn selo(&mut self) -> SELO_W<16> {
        SELO_W::new(self)
    }
    #[doc = "Bit 24 - Enable Inter-Word Delay Tiw"]
    #[inline(always)]
    #[must_use]
    pub fn tiwen(&mut self) -> TIWEN_W<24> {
        TIWEN_W::new(self)
    }
    #[doc = "Bit 25 - Slave Mode Clock Phase Select"]
    #[inline(always)]
    #[must_use]
    pub fn slphsel(&mut self) -> SLPHSEL_W<25> {
        SLPHSEL_W::new(self)
    }
    #[doc = "Bit 31 - Master Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn mclk(&mut self) -> MCLK_W<31> {
        MCLK_W::new(self)
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
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PCR_SSCMode to value 0"]
impl crate::Resettable for PCR_SSCMODE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
