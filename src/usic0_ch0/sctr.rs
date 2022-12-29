#[doc = "Register `SCTR` reader"]
pub struct R(crate::R<SCTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SCTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SCTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SCTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SCTR` writer"]
pub struct W(crate::W<SCTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SCTR_SPEC>;
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
impl From<crate::W<SCTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SCTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SDIR` reader - Shift Direction"]
pub type SDIR_R = crate::BitReader<SDIR_A>;
#[doc = "Shift Direction\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SDIR_A {
    #[doc = "0: Shift LSB first. The first data bit of a data word is located at bit position 0."]
    VALUE1 = 0,
    #[doc = "1: Shift MSB first. The first data bit of a data word is located at the bit position given by bit field SCTR.WLE."]
    VALUE2 = 1,
}
impl From<SDIR_A> for bool {
    #[inline(always)]
    fn from(variant: SDIR_A) -> Self {
        variant as u8 != 0
    }
}
impl SDIR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SDIR_A {
        match self.bits {
            false => SDIR_A::VALUE1,
            true => SDIR_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SDIR_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SDIR_A::VALUE2
    }
}
#[doc = "Field `SDIR` writer - Shift Direction"]
pub type SDIR_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCTR_SPEC, SDIR_A, O>;
impl<'a, const O: u8> SDIR_W<'a, O> {
    #[doc = "Shift LSB first. The first data bit of a data word is located at bit position 0."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(SDIR_A::VALUE1)
    }
    #[doc = "Shift MSB first. The first data bit of a data word is located at the bit position given by bit field SCTR.WLE."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(SDIR_A::VALUE2)
    }
}
#[doc = "Field `PDL` reader - Passive Data Level"]
pub type PDL_R = crate::BitReader<PDL_A>;
#[doc = "Passive Data Level\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PDL_A {
    #[doc = "0: The passive data level is 0."]
    VALUE1 = 0,
    #[doc = "1: The passive data level is 1."]
    VALUE2 = 1,
}
impl From<PDL_A> for bool {
    #[inline(always)]
    fn from(variant: PDL_A) -> Self {
        variant as u8 != 0
    }
}
impl PDL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDL_A {
        match self.bits {
            false => PDL_A::VALUE1,
            true => PDL_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PDL_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PDL_A::VALUE2
    }
}
#[doc = "Field `PDL` writer - Passive Data Level"]
pub type PDL_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCTR_SPEC, PDL_A, O>;
impl<'a, const O: u8> PDL_W<'a, O> {
    #[doc = "The passive data level is 0."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(PDL_A::VALUE1)
    }
    #[doc = "The passive data level is 1."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(PDL_A::VALUE2)
    }
}
#[doc = "Field `DSM` reader - Data Shift Mode"]
pub type DSM_R = crate::FieldReader<u8, DSM_A>;
#[doc = "Data Shift Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DSM_A {
    #[doc = "0: Receive and transmit data is shifted in and out one bit at a time through DX0 and DOUT0."]
    VALUE1 = 0,
    #[doc = "2: Receive and transmit data is shifted in and out two bits at a time through two input stages (DX0 and DX3) and DOUT\\[1:0\\]
respectively."]
    VALUE3 = 2,
    #[doc = "3: Receive and transmit data is shifted in and out four bits at a time through four input stages (DX0, DX\\[5:3\\]) and DOUT\\[3:0\\]
respectively."]
    VALUE4 = 3,
}
impl From<DSM_A> for u8 {
    #[inline(always)]
    fn from(variant: DSM_A) -> Self {
        variant as _
    }
}
impl DSM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<DSM_A> {
        match self.bits {
            0 => Some(DSM_A::VALUE1),
            2 => Some(DSM_A::VALUE3),
            3 => Some(DSM_A::VALUE4),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == DSM_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == DSM_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == DSM_A::VALUE4
    }
}
#[doc = "Field `DSM` writer - Data Shift Mode"]
pub type DSM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SCTR_SPEC, u8, DSM_A, 2, O>;
impl<'a, const O: u8> DSM_W<'a, O> {
    #[doc = "Receive and transmit data is shifted in and out one bit at a time through DX0 and DOUT0."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(DSM_A::VALUE1)
    }
    #[doc = "Receive and transmit data is shifted in and out two bits at a time through two input stages (DX0 and DX3) and DOUT\\[1:0\\]
respectively."]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(DSM_A::VALUE3)
    }
    #[doc = "Receive and transmit data is shifted in and out four bits at a time through four input stages (DX0, DX\\[5:3\\]) and DOUT\\[3:0\\]
respectively."]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(DSM_A::VALUE4)
    }
}
#[doc = "Field `HPCDIR` reader - Port Control Direction"]
pub type HPCDIR_R = crate::BitReader<HPCDIR_A>;
#[doc = "Port Control Direction\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HPCDIR_A {
    #[doc = "0: The pin(s) with hardware pin control enabled are selected to be in input mode."]
    VALUE1 = 0,
    #[doc = "1: The pin(s) with hardware pin control enabled are selected to be in output mode."]
    VALUE2 = 1,
}
impl From<HPCDIR_A> for bool {
    #[inline(always)]
    fn from(variant: HPCDIR_A) -> Self {
        variant as u8 != 0
    }
}
impl HPCDIR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HPCDIR_A {
        match self.bits {
            false => HPCDIR_A::VALUE1,
            true => HPCDIR_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == HPCDIR_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == HPCDIR_A::VALUE2
    }
}
#[doc = "Field `HPCDIR` writer - Port Control Direction"]
pub type HPCDIR_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCTR_SPEC, HPCDIR_A, O>;
impl<'a, const O: u8> HPCDIR_W<'a, O> {
    #[doc = "The pin(s) with hardware pin control enabled are selected to be in input mode."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(HPCDIR_A::VALUE1)
    }
    #[doc = "The pin(s) with hardware pin control enabled are selected to be in output mode."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(HPCDIR_A::VALUE2)
    }
}
#[doc = "Field `DOCFG` reader - Data Output Configuration"]
pub type DOCFG_R = crate::FieldReader<u8, DOCFG_A>;
#[doc = "Data Output Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DOCFG_A {
    #[doc = "0: DOUTx = shift data value"]
    VALUE1 = 0,
    #[doc = "1: DOUTx = inverted shift data value"]
    VALUE2 = 1,
}
impl From<DOCFG_A> for u8 {
    #[inline(always)]
    fn from(variant: DOCFG_A) -> Self {
        variant as _
    }
}
impl DOCFG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<DOCFG_A> {
        match self.bits {
            0 => Some(DOCFG_A::VALUE1),
            1 => Some(DOCFG_A::VALUE2),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == DOCFG_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == DOCFG_A::VALUE2
    }
}
#[doc = "Field `DOCFG` writer - Data Output Configuration"]
pub type DOCFG_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SCTR_SPEC, u8, DOCFG_A, 2, O>;
impl<'a, const O: u8> DOCFG_W<'a, O> {
    #[doc = "DOUTx = shift data value"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(DOCFG_A::VALUE1)
    }
    #[doc = "DOUTx = inverted shift data value"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(DOCFG_A::VALUE2)
    }
}
#[doc = "Field `TRM` reader - Transmission Mode"]
pub type TRM_R = crate::FieldReader<u8, TRM_A>;
#[doc = "Transmission Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TRM_A {
    #[doc = "0: The shift control signal is considered as inactive and data frame transfers are not possible."]
    VALUE1 = 0,
    #[doc = "1: The shift control signal is considered active if it is at 1-level. This is the setting to be programmed to allow data transfers."]
    VALUE2 = 1,
    #[doc = "2: The shift control signal is considered active if it is at 0-level. It is recommended to avoid this setting and to use the inversion in the DX2 stage in case of a low-active signal."]
    VALUE3 = 2,
    #[doc = "3: The shift control signal is considered active without referring to the actual signal level. Data frame transfer is possible after each edge of the signal."]
    VALUE4 = 3,
}
impl From<TRM_A> for u8 {
    #[inline(always)]
    fn from(variant: TRM_A) -> Self {
        variant as _
    }
}
impl TRM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRM_A {
        match self.bits {
            0 => TRM_A::VALUE1,
            1 => TRM_A::VALUE2,
            2 => TRM_A::VALUE3,
            3 => TRM_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == TRM_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == TRM_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == TRM_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == TRM_A::VALUE4
    }
}
#[doc = "Field `TRM` writer - Transmission Mode"]
pub type TRM_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, SCTR_SPEC, u8, TRM_A, 2, O>;
impl<'a, const O: u8> TRM_W<'a, O> {
    #[doc = "The shift control signal is considered as inactive and data frame transfers are not possible."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(TRM_A::VALUE1)
    }
    #[doc = "The shift control signal is considered active if it is at 1-level. This is the setting to be programmed to allow data transfers."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(TRM_A::VALUE2)
    }
    #[doc = "The shift control signal is considered active if it is at 0-level. It is recommended to avoid this setting and to use the inversion in the DX2 stage in case of a low-active signal."]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(TRM_A::VALUE3)
    }
    #[doc = "The shift control signal is considered active without referring to the actual signal level. Data frame transfer is possible after each edge of the signal."]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(TRM_A::VALUE4)
    }
}
#[doc = "Field `FLE` reader - Frame Length"]
pub type FLE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FLE` writer - Frame Length"]
pub type FLE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SCTR_SPEC, u8, u8, 6, O>;
#[doc = "Field `WLE` reader - Word Length"]
pub type WLE_R = crate::FieldReader<u8, WLE_A>;
#[doc = "Word Length\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum WLE_A {
    #[doc = "0: The data word contains 1 data bit located at bit position 0."]
    VALUE1 = 0,
    #[doc = "1: The data word contains 2 data bits located at bit positions \\[1:0\\]."]
    VALUE2 = 1,
    #[doc = "14: The data word contains 15 data bits located at bit positions \\[14:0\\]."]
    VALUE3 = 14,
    #[doc = "15: The data word contains 16 data bits located at bit positions \\[15:0\\]."]
    VALUE4 = 15,
}
impl From<WLE_A> for u8 {
    #[inline(always)]
    fn from(variant: WLE_A) -> Self {
        variant as _
    }
}
impl WLE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<WLE_A> {
        match self.bits {
            0 => Some(WLE_A::VALUE1),
            1 => Some(WLE_A::VALUE2),
            14 => Some(WLE_A::VALUE3),
            15 => Some(WLE_A::VALUE4),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == WLE_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == WLE_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == WLE_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == WLE_A::VALUE4
    }
}
#[doc = "Field `WLE` writer - Word Length"]
pub type WLE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SCTR_SPEC, u8, WLE_A, 4, O>;
impl<'a, const O: u8> WLE_W<'a, O> {
    #[doc = "The data word contains 1 data bit located at bit position 0."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(WLE_A::VALUE1)
    }
    #[doc = "The data word contains 2 data bits located at bit positions \\[1:0\\]."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(WLE_A::VALUE2)
    }
    #[doc = "The data word contains 15 data bits located at bit positions \\[14:0\\]."]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(WLE_A::VALUE3)
    }
    #[doc = "The data word contains 16 data bits located at bit positions \\[15:0\\]."]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(WLE_A::VALUE4)
    }
}
impl R {
    #[doc = "Bit 0 - Shift Direction"]
    #[inline(always)]
    pub fn sdir(&self) -> SDIR_R {
        SDIR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Passive Data Level"]
    #[inline(always)]
    pub fn pdl(&self) -> PDL_R {
        PDL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - Data Shift Mode"]
    #[inline(always)]
    pub fn dsm(&self) -> DSM_R {
        DSM_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - Port Control Direction"]
    #[inline(always)]
    pub fn hpcdir(&self) -> HPCDIR_R {
        HPCDIR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 6:7 - Data Output Configuration"]
    #[inline(always)]
    pub fn docfg(&self) -> DOCFG_R {
        DOCFG_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Transmission Mode"]
    #[inline(always)]
    pub fn trm(&self) -> TRM_R {
        TRM_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 16:21 - Frame Length"]
    #[inline(always)]
    pub fn fle(&self) -> FLE_R {
        FLE_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 24:27 - Word Length"]
    #[inline(always)]
    pub fn wle(&self) -> WLE_R {
        WLE_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Shift Direction"]
    #[inline(always)]
    #[must_use]
    pub fn sdir(&mut self) -> SDIR_W<0> {
        SDIR_W::new(self)
    }
    #[doc = "Bit 1 - Passive Data Level"]
    #[inline(always)]
    #[must_use]
    pub fn pdl(&mut self) -> PDL_W<1> {
        PDL_W::new(self)
    }
    #[doc = "Bits 2:3 - Data Shift Mode"]
    #[inline(always)]
    #[must_use]
    pub fn dsm(&mut self) -> DSM_W<2> {
        DSM_W::new(self)
    }
    #[doc = "Bit 4 - Port Control Direction"]
    #[inline(always)]
    #[must_use]
    pub fn hpcdir(&mut self) -> HPCDIR_W<4> {
        HPCDIR_W::new(self)
    }
    #[doc = "Bits 6:7 - Data Output Configuration"]
    #[inline(always)]
    #[must_use]
    pub fn docfg(&mut self) -> DOCFG_W<6> {
        DOCFG_W::new(self)
    }
    #[doc = "Bits 8:9 - Transmission Mode"]
    #[inline(always)]
    #[must_use]
    pub fn trm(&mut self) -> TRM_W<8> {
        TRM_W::new(self)
    }
    #[doc = "Bits 16:21 - Frame Length"]
    #[inline(always)]
    #[must_use]
    pub fn fle(&mut self) -> FLE_W<16> {
        FLE_W::new(self)
    }
    #[doc = "Bits 24:27 - Word Length"]
    #[inline(always)]
    #[must_use]
    pub fn wle(&mut self) -> WLE_W<24> {
        WLE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Shift Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sctr](index.html) module"]
pub struct SCTR_SPEC;
impl crate::RegisterSpec for SCTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sctr::R](R) reader structure"]
impl crate::Readable for SCTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sctr::W](W) writer structure"]
impl crate::Writable for SCTR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SCTR to value 0"]
impl crate::Resettable for SCTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
