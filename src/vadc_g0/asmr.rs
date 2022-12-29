#[doc = "Register `ASMR` reader"]
pub struct R(crate::R<ASMR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ASMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ASMR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ASMR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ASMR` writer"]
pub struct W(crate::W<ASMR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ASMR_SPEC>;
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
impl From<crate::W<ASMR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ASMR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ENGT` reader - Enable Gate"]
pub type ENGT_R = crate::FieldReader<u8, ENGT_A>;
#[doc = "Enable Gate\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ENGT_A {
    #[doc = "0: No conversion requests are issued"]
    VALUE1 = 0,
    #[doc = "1: Conversion requests are issued if at least one pending bit is set"]
    VALUE2 = 1,
    #[doc = "2: Conversion requests are issued if at least one pending bit is set and REQGTx = 1."]
    VALUE3 = 2,
    #[doc = "3: Conversion requests are issued if at least one pending bit is set and REQGTx = 0."]
    VALUE4 = 3,
}
impl From<ENGT_A> for u8 {
    #[inline(always)]
    fn from(variant: ENGT_A) -> Self {
        variant as _
    }
}
impl ENGT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENGT_A {
        match self.bits {
            0 => ENGT_A::VALUE1,
            1 => ENGT_A::VALUE2,
            2 => ENGT_A::VALUE3,
            3 => ENGT_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == ENGT_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == ENGT_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == ENGT_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == ENGT_A::VALUE4
    }
}
#[doc = "Field `ENGT` writer - Enable Gate"]
pub type ENGT_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, ASMR_SPEC, u8, ENGT_A, 2, O>;
impl<'a, const O: u8> ENGT_W<'a, O> {
    #[doc = "No conversion requests are issued"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(ENGT_A::VALUE1)
    }
    #[doc = "Conversion requests are issued if at least one pending bit is set"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(ENGT_A::VALUE2)
    }
    #[doc = "Conversion requests are issued if at least one pending bit is set and REQGTx = 1."]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(ENGT_A::VALUE3)
    }
    #[doc = "Conversion requests are issued if at least one pending bit is set and REQGTx = 0."]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(ENGT_A::VALUE4)
    }
}
#[doc = "Field `ENTR` reader - Enable External Trigger"]
pub type ENTR_R = crate::BitReader<ENTR_A>;
#[doc = "Enable External Trigger\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ENTR_A {
    #[doc = "0: External trigger disabled"]
    VALUE1 = 0,
    #[doc = "1: The selected edge at the selected trigger input signal REQTR generates the load event"]
    VALUE2 = 1,
}
impl From<ENTR_A> for bool {
    #[inline(always)]
    fn from(variant: ENTR_A) -> Self {
        variant as u8 != 0
    }
}
impl ENTR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENTR_A {
        match self.bits {
            false => ENTR_A::VALUE1,
            true => ENTR_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == ENTR_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == ENTR_A::VALUE2
    }
}
#[doc = "Field `ENTR` writer - Enable External Trigger"]
pub type ENTR_W<'a, const O: u8> = crate::BitWriter<'a, u32, ASMR_SPEC, ENTR_A, O>;
impl<'a, const O: u8> ENTR_W<'a, O> {
    #[doc = "External trigger disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(ENTR_A::VALUE1)
    }
    #[doc = "The selected edge at the selected trigger input signal REQTR generates the load event"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(ENTR_A::VALUE2)
    }
}
#[doc = "Field `ENSI` reader - Enable Source Interrupt"]
pub type ENSI_R = crate::BitReader<ENSI_A>;
#[doc = "Enable Source Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ENSI_A {
    #[doc = "0: No request source interrupt"]
    VALUE1 = 0,
    #[doc = "1: A request source interrupt is generated upon a request source event (last pending conversion is finished)"]
    VALUE2 = 1,
}
impl From<ENSI_A> for bool {
    #[inline(always)]
    fn from(variant: ENSI_A) -> Self {
        variant as u8 != 0
    }
}
impl ENSI_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENSI_A {
        match self.bits {
            false => ENSI_A::VALUE1,
            true => ENSI_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == ENSI_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == ENSI_A::VALUE2
    }
}
#[doc = "Field `ENSI` writer - Enable Source Interrupt"]
pub type ENSI_W<'a, const O: u8> = crate::BitWriter<'a, u32, ASMR_SPEC, ENSI_A, O>;
impl<'a, const O: u8> ENSI_W<'a, O> {
    #[doc = "No request source interrupt"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(ENSI_A::VALUE1)
    }
    #[doc = "A request source interrupt is generated upon a request source event (last pending conversion is finished)"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(ENSI_A::VALUE2)
    }
}
#[doc = "Field `SCAN` reader - Autoscan Enable"]
pub type SCAN_R = crate::BitReader<SCAN_A>;
#[doc = "Autoscan Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SCAN_A {
    #[doc = "0: No autoscan"]
    VALUE1 = 0,
    #[doc = "1: Autoscan functionality enabled: a request source event automatically generates a load event"]
    VALUE2 = 1,
}
impl From<SCAN_A> for bool {
    #[inline(always)]
    fn from(variant: SCAN_A) -> Self {
        variant as u8 != 0
    }
}
impl SCAN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SCAN_A {
        match self.bits {
            false => SCAN_A::VALUE1,
            true => SCAN_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SCAN_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SCAN_A::VALUE2
    }
}
#[doc = "Field `SCAN` writer - Autoscan Enable"]
pub type SCAN_W<'a, const O: u8> = crate::BitWriter<'a, u32, ASMR_SPEC, SCAN_A, O>;
impl<'a, const O: u8> SCAN_W<'a, O> {
    #[doc = "No autoscan"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(SCAN_A::VALUE1)
    }
    #[doc = "Autoscan functionality enabled: a request source event automatically generates a load event"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(SCAN_A::VALUE2)
    }
}
#[doc = "Field `LDM` reader - Autoscan Source Load Event Mode"]
pub type LDM_R = crate::BitReader<LDM_A>;
#[doc = "Autoscan Source Load Event Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LDM_A {
    #[doc = "0: Overwrite mode: Copy all bits from the select registers to the pending registers upon a load event"]
    VALUE1 = 0,
    #[doc = "1: Combine mode: Set all pending bits that are set in the select registers upon a load event (logic OR)"]
    VALUE2 = 1,
}
impl From<LDM_A> for bool {
    #[inline(always)]
    fn from(variant: LDM_A) -> Self {
        variant as u8 != 0
    }
}
impl LDM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LDM_A {
        match self.bits {
            false => LDM_A::VALUE1,
            true => LDM_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == LDM_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == LDM_A::VALUE2
    }
}
#[doc = "Field `LDM` writer - Autoscan Source Load Event Mode"]
pub type LDM_W<'a, const O: u8> = crate::BitWriter<'a, u32, ASMR_SPEC, LDM_A, O>;
impl<'a, const O: u8> LDM_W<'a, O> {
    #[doc = "Overwrite mode: Copy all bits from the select registers to the pending registers upon a load event"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(LDM_A::VALUE1)
    }
    #[doc = "Combine mode: Set all pending bits that are set in the select registers upon a load event (logic OR)"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(LDM_A::VALUE2)
    }
}
#[doc = "Field `REQGT` reader - Request Gate Level"]
pub type REQGT_R = crate::BitReader<REQGT_A>;
#[doc = "Request Gate Level\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REQGT_A {
    #[doc = "0: The gate input is low"]
    VALUE1 = 0,
    #[doc = "1: The gate input is high"]
    VALUE2 = 1,
}
impl From<REQGT_A> for bool {
    #[inline(always)]
    fn from(variant: REQGT_A) -> Self {
        variant as u8 != 0
    }
}
impl REQGT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REQGT_A {
        match self.bits {
            false => REQGT_A::VALUE1,
            true => REQGT_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == REQGT_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == REQGT_A::VALUE2
    }
}
#[doc = "Clear Pending Bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CLRPND_AW {
    #[doc = "0: No action"]
    VALUE1 = 0,
    #[doc = "1: The bits in register GxASPNDx are cleared"]
    VALUE2 = 1,
}
impl From<CLRPND_AW> for bool {
    #[inline(always)]
    fn from(variant: CLRPND_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLRPND` writer - Clear Pending Bits"]
pub type CLRPND_W<'a, const O: u8> = crate::BitWriter<'a, u32, ASMR_SPEC, CLRPND_AW, O>;
impl<'a, const O: u8> CLRPND_W<'a, O> {
    #[doc = "No action"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CLRPND_AW::VALUE1)
    }
    #[doc = "The bits in register GxASPNDx are cleared"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CLRPND_AW::VALUE2)
    }
}
#[doc = "Generate Load Event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LDEV_AW {
    #[doc = "0: No action"]
    VALUE1 = 0,
    #[doc = "1: A load event is generated"]
    VALUE2 = 1,
}
impl From<LDEV_AW> for bool {
    #[inline(always)]
    fn from(variant: LDEV_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LDEV` writer - Generate Load Event"]
pub type LDEV_W<'a, const O: u8> = crate::BitWriter<'a, u32, ASMR_SPEC, LDEV_AW, O>;
impl<'a, const O: u8> LDEV_W<'a, O> {
    #[doc = "No action"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(LDEV_AW::VALUE1)
    }
    #[doc = "A load event is generated"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(LDEV_AW::VALUE2)
    }
}
#[doc = "Field `RPTDIS` reader - Repeat Disable"]
pub type RPTDIS_R = crate::BitReader<RPTDIS_A>;
#[doc = "Repeat Disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RPTDIS_A {
    #[doc = "0: A cancelled conversion is repeated"]
    VALUE1 = 0,
    #[doc = "1: A cancelled conversion is discarded"]
    VALUE2 = 1,
}
impl From<RPTDIS_A> for bool {
    #[inline(always)]
    fn from(variant: RPTDIS_A) -> Self {
        variant as u8 != 0
    }
}
impl RPTDIS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RPTDIS_A {
        match self.bits {
            false => RPTDIS_A::VALUE1,
            true => RPTDIS_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == RPTDIS_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == RPTDIS_A::VALUE2
    }
}
#[doc = "Field `RPTDIS` writer - Repeat Disable"]
pub type RPTDIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, ASMR_SPEC, RPTDIS_A, O>;
impl<'a, const O: u8> RPTDIS_W<'a, O> {
    #[doc = "A cancelled conversion is repeated"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(RPTDIS_A::VALUE1)
    }
    #[doc = "A cancelled conversion is discarded"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(RPTDIS_A::VALUE2)
    }
}
impl R {
    #[doc = "Bits 0:1 - Enable Gate"]
    #[inline(always)]
    pub fn engt(&self) -> ENGT_R {
        ENGT_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - Enable External Trigger"]
    #[inline(always)]
    pub fn entr(&self) -> ENTR_R {
        ENTR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable Source Interrupt"]
    #[inline(always)]
    pub fn ensi(&self) -> ENSI_R {
        ENSI_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Autoscan Enable"]
    #[inline(always)]
    pub fn scan(&self) -> SCAN_R {
        SCAN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Autoscan Source Load Event Mode"]
    #[inline(always)]
    pub fn ldm(&self) -> LDM_R {
        LDM_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - Request Gate Level"]
    #[inline(always)]
    pub fn reqgt(&self) -> REQGT_R {
        REQGT_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 16 - Repeat Disable"]
    #[inline(always)]
    pub fn rptdis(&self) -> RPTDIS_R {
        RPTDIS_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Enable Gate"]
    #[inline(always)]
    #[must_use]
    pub fn engt(&mut self) -> ENGT_W<0> {
        ENGT_W::new(self)
    }
    #[doc = "Bit 2 - Enable External Trigger"]
    #[inline(always)]
    #[must_use]
    pub fn entr(&mut self) -> ENTR_W<2> {
        ENTR_W::new(self)
    }
    #[doc = "Bit 3 - Enable Source Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn ensi(&mut self) -> ENSI_W<3> {
        ENSI_W::new(self)
    }
    #[doc = "Bit 4 - Autoscan Enable"]
    #[inline(always)]
    #[must_use]
    pub fn scan(&mut self) -> SCAN_W<4> {
        SCAN_W::new(self)
    }
    #[doc = "Bit 5 - Autoscan Source Load Event Mode"]
    #[inline(always)]
    #[must_use]
    pub fn ldm(&mut self) -> LDM_W<5> {
        LDM_W::new(self)
    }
    #[doc = "Bit 8 - Clear Pending Bits"]
    #[inline(always)]
    #[must_use]
    pub fn clrpnd(&mut self) -> CLRPND_W<8> {
        CLRPND_W::new(self)
    }
    #[doc = "Bit 9 - Generate Load Event"]
    #[inline(always)]
    #[must_use]
    pub fn ldev(&mut self) -> LDEV_W<9> {
        LDEV_W::new(self)
    }
    #[doc = "Bit 16 - Repeat Disable"]
    #[inline(always)]
    #[must_use]
    pub fn rptdis(&mut self) -> RPTDIS_W<16> {
        RPTDIS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Autoscan Source Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [asmr](index.html) module"]
pub struct ASMR_SPEC;
impl crate::RegisterSpec for ASMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [asmr::R](R) reader structure"]
impl crate::Readable for ASMR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [asmr::W](W) writer structure"]
impl crate::Writable for ASMR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ASMR to value 0"]
impl crate::Resettable for ASMR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
