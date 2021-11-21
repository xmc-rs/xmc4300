#[doc = "Register `BYPCR` reader"]
pub struct R(crate::R<BYPCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BYPCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BYPCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BYPCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BYPCR` writer"]
pub struct W(crate::W<BYPCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BYPCR_SPEC>;
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
impl From<crate::W<BYPCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BYPCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BWLE` reader - Bypass Word Length"]
pub struct BWLE_R(crate::FieldReader<u8, u8>);
impl BWLE_R {
    pub(crate) fn new(bits: u8) -> Self {
        BWLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BWLE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BWLE` writer - Bypass Word Length"]
pub struct BWLE_W<'a> {
    w: &'a mut W,
}
impl<'a> BWLE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
#[doc = "Bypass Data Single Shot Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BDSSM_A {
    #[doc = "0: The bypass data is still considered as valid after it has been loaded into TBUF. The loading of the data into TBUF does not clear BDV."]
    VALUE1 = 0,
    #[doc = "1: The bypass data is considered as invalid after it has been loaded into TBUF. The loading of the data into TBUF clears BDV."]
    VALUE2 = 1,
}
impl From<BDSSM_A> for bool {
    #[inline(always)]
    fn from(variant: BDSSM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BDSSM` reader - Bypass Data Single Shot Mode"]
pub struct BDSSM_R(crate::FieldReader<bool, BDSSM_A>);
impl BDSSM_R {
    pub(crate) fn new(bits: bool) -> Self {
        BDSSM_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BDSSM_A {
        match self.bits {
            false => BDSSM_A::VALUE1,
            true => BDSSM_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == BDSSM_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == BDSSM_A::VALUE2
    }
}
impl core::ops::Deref for BDSSM_R {
    type Target = crate::FieldReader<bool, BDSSM_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BDSSM` writer - Bypass Data Single Shot Mode"]
pub struct BDSSM_W<'a> {
    w: &'a mut W,
}
impl<'a> BDSSM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BDSSM_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The bypass data is still considered as valid after it has been loaded into TBUF. The loading of the data into TBUF does not clear BDV."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(BDSSM_A::VALUE1)
    }
    #[doc = "The bypass data is considered as invalid after it has been loaded into TBUF. The loading of the data into TBUF clears BDV."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(BDSSM_A::VALUE2)
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
#[doc = "Bypass Data Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum BDEN_A {
    #[doc = "0: The transfer of bypass data is disabled."]
    VALUE1 = 0,
    #[doc = "1: The transfer of bypass data to TBUF is possible. Bypass data will be transferred to TBUF according to its priority if BDV = 1."]
    VALUE2 = 1,
    #[doc = "2: Gated bypass data transfer is enabled. Bypass data will be transferred to TBUF according to its priority if BDV = 1 and while DX2S = 0."]
    VALUE3 = 2,
    #[doc = "3: Gated bypass data transfer is enabled. Bypass data will be transferred to TBUF according to its priority if BDV = 1 and while DX2S = 1."]
    VALUE4 = 3,
}
impl From<BDEN_A> for u8 {
    #[inline(always)]
    fn from(variant: BDEN_A) -> Self {
        variant as _
    }
}
#[doc = "Field `BDEN` reader - Bypass Data Enable"]
pub struct BDEN_R(crate::FieldReader<u8, BDEN_A>);
impl BDEN_R {
    pub(crate) fn new(bits: u8) -> Self {
        BDEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BDEN_A {
        match self.bits {
            0 => BDEN_A::VALUE1,
            1 => BDEN_A::VALUE2,
            2 => BDEN_A::VALUE3,
            3 => BDEN_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == BDEN_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == BDEN_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        **self == BDEN_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        **self == BDEN_A::VALUE4
    }
}
impl core::ops::Deref for BDEN_R {
    type Target = crate::FieldReader<u8, BDEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BDEN` writer - Bypass Data Enable"]
pub struct BDEN_W<'a> {
    w: &'a mut W,
}
impl<'a> BDEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BDEN_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "The transfer of bypass data is disabled."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(BDEN_A::VALUE1)
    }
    #[doc = "The transfer of bypass data to TBUF is possible. Bypass data will be transferred to TBUF according to its priority if BDV = 1."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(BDEN_A::VALUE2)
    }
    #[doc = "Gated bypass data transfer is enabled. Bypass data will be transferred to TBUF according to its priority if BDV = 1 and while DX2S = 0."]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(BDEN_A::VALUE3)
    }
    #[doc = "Gated bypass data transfer is enabled. Bypass data will be transferred to TBUF according to its priority if BDV = 1 and while DX2S = 1."]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(BDEN_A::VALUE4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | ((value as u32 & 0x03) << 10);
        self.w
    }
}
#[doc = "Bypass Data Valid Trigger\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BDVTR_A {
    #[doc = "0: Bit BDV is not influenced by DX2T."]
    VALUE1 = 0,
    #[doc = "1: Bit BDV is set if DX2T is active."]
    VALUE2 = 1,
}
impl From<BDVTR_A> for bool {
    #[inline(always)]
    fn from(variant: BDVTR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BDVTR` reader - Bypass Data Valid Trigger"]
pub struct BDVTR_R(crate::FieldReader<bool, BDVTR_A>);
impl BDVTR_R {
    pub(crate) fn new(bits: bool) -> Self {
        BDVTR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BDVTR_A {
        match self.bits {
            false => BDVTR_A::VALUE1,
            true => BDVTR_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == BDVTR_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == BDVTR_A::VALUE2
    }
}
impl core::ops::Deref for BDVTR_R {
    type Target = crate::FieldReader<bool, BDVTR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BDVTR` writer - Bypass Data Valid Trigger"]
pub struct BDVTR_W<'a> {
    w: &'a mut W,
}
impl<'a> BDVTR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BDVTR_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Bit BDV is not influenced by DX2T."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(BDVTR_A::VALUE1)
    }
    #[doc = "Bit BDV is set if DX2T is active."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(BDVTR_A::VALUE2)
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
#[doc = "Bypass Priority\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BPRIO_A {
    #[doc = "0: The transmit FIFO data has a higher priority than the bypass data."]
    VALUE1 = 0,
    #[doc = "1: The bypass data has a higher priority than the transmit FIFO data."]
    VALUE2 = 1,
}
impl From<BPRIO_A> for bool {
    #[inline(always)]
    fn from(variant: BPRIO_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPRIO` reader - Bypass Priority"]
pub struct BPRIO_R(crate::FieldReader<bool, BPRIO_A>);
impl BPRIO_R {
    pub(crate) fn new(bits: bool) -> Self {
        BPRIO_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BPRIO_A {
        match self.bits {
            false => BPRIO_A::VALUE1,
            true => BPRIO_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == BPRIO_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == BPRIO_A::VALUE2
    }
}
impl core::ops::Deref for BPRIO_R {
    type Target = crate::FieldReader<bool, BPRIO_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BPRIO` writer - Bypass Priority"]
pub struct BPRIO_W<'a> {
    w: &'a mut W,
}
impl<'a> BPRIO_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BPRIO_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The transmit FIFO data has a higher priority than the bypass data."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(BPRIO_A::VALUE1)
    }
    #[doc = "The bypass data has a higher priority than the transmit FIFO data."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(BPRIO_A::VALUE2)
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
#[doc = "Bypass Data Valid\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BDV_A {
    #[doc = "0: The bypass data is not valid."]
    VALUE1 = 0,
    #[doc = "1: The bypass data is valid."]
    VALUE2 = 1,
}
impl From<BDV_A> for bool {
    #[inline(always)]
    fn from(variant: BDV_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BDV` reader - Bypass Data Valid"]
pub struct BDV_R(crate::FieldReader<bool, BDV_A>);
impl BDV_R {
    pub(crate) fn new(bits: bool) -> Self {
        BDV_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BDV_A {
        match self.bits {
            false => BDV_A::VALUE1,
            true => BDV_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == BDV_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == BDV_A::VALUE2
    }
}
impl core::ops::Deref for BDV_R {
    type Target = crate::FieldReader<bool, BDV_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BSELO` reader - Bypass Select Outputs"]
pub struct BSELO_R(crate::FieldReader<u8, u8>);
impl BSELO_R {
    pub(crate) fn new(bits: u8) -> Self {
        BSELO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BSELO_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BSELO` writer - Bypass Select Outputs"]
pub struct BSELO_W<'a> {
    w: &'a mut W,
}
impl<'a> BSELO_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 16)) | ((value as u32 & 0x1f) << 16);
        self.w
    }
}
#[doc = "Field `BHPC` reader - Bypass Hardware Port Control"]
pub struct BHPC_R(crate::FieldReader<u8, u8>);
impl BHPC_R {
    pub(crate) fn new(bits: u8) -> Self {
        BHPC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BHPC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BHPC` writer - Bypass Hardware Port Control"]
pub struct BHPC_W<'a> {
    w: &'a mut W,
}
impl<'a> BHPC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 21)) | ((value as u32 & 0x07) << 21);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Bypass Word Length"]
    #[inline(always)]
    pub fn bwle(&self) -> BWLE_R {
        BWLE_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 8 - Bypass Data Single Shot Mode"]
    #[inline(always)]
    pub fn bdssm(&self) -> BDSSM_R {
        BDSSM_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 10:11 - Bypass Data Enable"]
    #[inline(always)]
    pub fn bden(&self) -> BDEN_R {
        BDEN_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bit 12 - Bypass Data Valid Trigger"]
    #[inline(always)]
    pub fn bdvtr(&self) -> BDVTR_R {
        BDVTR_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Bypass Priority"]
    #[inline(always)]
    pub fn bprio(&self) -> BPRIO_R {
        BPRIO_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Bypass Data Valid"]
    #[inline(always)]
    pub fn bdv(&self) -> BDV_R {
        BDV_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 16:20 - Bypass Select Outputs"]
    #[inline(always)]
    pub fn bselo(&self) -> BSELO_R {
        BSELO_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 21:23 - Bypass Hardware Port Control"]
    #[inline(always)]
    pub fn bhpc(&self) -> BHPC_R {
        BHPC_R::new(((self.bits >> 21) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Bypass Word Length"]
    #[inline(always)]
    pub fn bwle(&mut self) -> BWLE_W {
        BWLE_W { w: self }
    }
    #[doc = "Bit 8 - Bypass Data Single Shot Mode"]
    #[inline(always)]
    pub fn bdssm(&mut self) -> BDSSM_W {
        BDSSM_W { w: self }
    }
    #[doc = "Bits 10:11 - Bypass Data Enable"]
    #[inline(always)]
    pub fn bden(&mut self) -> BDEN_W {
        BDEN_W { w: self }
    }
    #[doc = "Bit 12 - Bypass Data Valid Trigger"]
    #[inline(always)]
    pub fn bdvtr(&mut self) -> BDVTR_W {
        BDVTR_W { w: self }
    }
    #[doc = "Bit 13 - Bypass Priority"]
    #[inline(always)]
    pub fn bprio(&mut self) -> BPRIO_W {
        BPRIO_W { w: self }
    }
    #[doc = "Bits 16:20 - Bypass Select Outputs"]
    #[inline(always)]
    pub fn bselo(&mut self) -> BSELO_W {
        BSELO_W { w: self }
    }
    #[doc = "Bits 21:23 - Bypass Hardware Port Control"]
    #[inline(always)]
    pub fn bhpc(&mut self) -> BHPC_W {
        BHPC_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Bypass Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bypcr](index.html) module"]
pub struct BYPCR_SPEC;
impl crate::RegisterSpec for BYPCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bypcr::R](R) reader structure"]
impl crate::Readable for BYPCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bypcr::W](W) writer structure"]
impl crate::Writable for BYPCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BYPCR to value 0"]
impl crate::Resettable for BYPCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
