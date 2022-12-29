#[doc = "Register `GLOBCFG` reader"]
pub struct R(crate::R<GLOBCFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GLOBCFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GLOBCFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GLOBCFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GLOBCFG` writer"]
pub struct W(crate::W<GLOBCFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GLOBCFG_SPEC>;
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
impl From<crate::W<GLOBCFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GLOBCFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DIVA` reader - Divider Factor for the Analog Internal Clock"]
pub type DIVA_R = crate::FieldReader<u8, DIVA_A>;
#[doc = "Divider Factor for the Analog Internal Clock\n\nValue on reset: 15"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DIVA_A {
    #[doc = "0: fADCI = fADC / 2"]
    VALUE1 = 0,
    #[doc = "1: fADCI = fADC / 2"]
    VALUE2 = 1,
    #[doc = "2: fADCI = fADC / 3"]
    VALUE3 = 2,
    #[doc = "31: fADCI = fADC / 32"]
    VALUE4 = 31,
}
impl From<DIVA_A> for u8 {
    #[inline(always)]
    fn from(variant: DIVA_A) -> Self {
        variant as _
    }
}
impl DIVA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<DIVA_A> {
        match self.bits {
            0 => Some(DIVA_A::VALUE1),
            1 => Some(DIVA_A::VALUE2),
            2 => Some(DIVA_A::VALUE3),
            31 => Some(DIVA_A::VALUE4),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == DIVA_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == DIVA_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == DIVA_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == DIVA_A::VALUE4
    }
}
#[doc = "Field `DIVA` writer - Divider Factor for the Analog Internal Clock"]
pub type DIVA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GLOBCFG_SPEC, u8, DIVA_A, 5, O>;
impl<'a, const O: u8> DIVA_W<'a, O> {
    #[doc = "fADCI = fADC / 2"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(DIVA_A::VALUE1)
    }
    #[doc = "fADCI = fADC / 2"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(DIVA_A::VALUE2)
    }
    #[doc = "fADCI = fADC / 3"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(DIVA_A::VALUE3)
    }
    #[doc = "fADCI = fADC / 32"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(DIVA_A::VALUE4)
    }
}
#[doc = "Field `DCMSB` reader - Double Clock for the MSB Conversion"]
pub type DCMSB_R = crate::BitReader<DCMSB_A>;
#[doc = "Double Clock for the MSB Conversion\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DCMSB_A {
    #[doc = "0: 1 clock cycles for the MSB (standard)"]
    VALUE1 = 0,
    #[doc = "1: 2 clock cycles for the MSB (fADCI > 20 MHz)"]
    VALUE2 = 1,
}
impl From<DCMSB_A> for bool {
    #[inline(always)]
    fn from(variant: DCMSB_A) -> Self {
        variant as u8 != 0
    }
}
impl DCMSB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DCMSB_A {
        match self.bits {
            false => DCMSB_A::VALUE1,
            true => DCMSB_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == DCMSB_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == DCMSB_A::VALUE2
    }
}
#[doc = "Field `DCMSB` writer - Double Clock for the MSB Conversion"]
pub type DCMSB_W<'a, const O: u8> = crate::BitWriter<'a, u32, GLOBCFG_SPEC, DCMSB_A, O>;
impl<'a, const O: u8> DCMSB_W<'a, O> {
    #[doc = "1 clock cycles for the MSB (standard)"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(DCMSB_A::VALUE1)
    }
    #[doc = "2 clock cycles for the MSB (fADCI > 20 MHz)"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(DCMSB_A::VALUE2)
    }
}
#[doc = "Field `DIVD` reader - Divider Factor for the Arbiter Clock"]
pub type DIVD_R = crate::FieldReader<u8, DIVD_A>;
#[doc = "Divider Factor for the Arbiter Clock\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DIVD_A {
    #[doc = "0: fADCD = fADC"]
    VALUE1 = 0,
    #[doc = "1: fADCD = fADC / 2"]
    VALUE2 = 1,
    #[doc = "2: fADCD = fADC / 3"]
    VALUE3 = 2,
    #[doc = "3: fADCD = fADC / 4"]
    VALUE4 = 3,
}
impl From<DIVD_A> for u8 {
    #[inline(always)]
    fn from(variant: DIVD_A) -> Self {
        variant as _
    }
}
impl DIVD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIVD_A {
        match self.bits {
            0 => DIVD_A::VALUE1,
            1 => DIVD_A::VALUE2,
            2 => DIVD_A::VALUE3,
            3 => DIVD_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == DIVD_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == DIVD_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == DIVD_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == DIVD_A::VALUE4
    }
}
#[doc = "Field `DIVD` writer - Divider Factor for the Arbiter Clock"]
pub type DIVD_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, GLOBCFG_SPEC, u8, DIVD_A, 2, O>;
impl<'a, const O: u8> DIVD_W<'a, O> {
    #[doc = "fADCD = fADC"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(DIVD_A::VALUE1)
    }
    #[doc = "fADCD = fADC / 2"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(DIVD_A::VALUE2)
    }
    #[doc = "fADCD = fADC / 3"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(DIVD_A::VALUE3)
    }
    #[doc = "fADCD = fADC / 4"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(DIVD_A::VALUE4)
    }
}
#[doc = "Write Control for Divider Parameters\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DIVWC_AW {
    #[doc = "0: No write access to divider parameters"]
    VALUE1 = 0,
    #[doc = "1: Bitfields DIVA, DCMSB, DIVD can be written"]
    VALUE2 = 1,
}
impl From<DIVWC_AW> for bool {
    #[inline(always)]
    fn from(variant: DIVWC_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIVWC` writer - Write Control for Divider Parameters"]
pub type DIVWC_W<'a, const O: u8> = crate::BitWriter<'a, u32, GLOBCFG_SPEC, DIVWC_AW, O>;
impl<'a, const O: u8> DIVWC_W<'a, O> {
    #[doc = "No write access to divider parameters"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(DIVWC_AW::VALUE1)
    }
    #[doc = "Bitfields DIVA, DCMSB, DIVD can be written"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(DIVWC_AW::VALUE2)
    }
}
#[doc = "Field `DPCAL0` reader - Disable Post-Calibration"]
pub type DPCAL0_R = crate::BitReader<DPCAL0_A>;
#[doc = "Disable Post-Calibration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DPCAL0_A {
    #[doc = "0: Automatic post-calibration after each conversion of group x"]
    VALUE1 = 0,
    #[doc = "1: No post-calibration"]
    VALUE2 = 1,
}
impl From<DPCAL0_A> for bool {
    #[inline(always)]
    fn from(variant: DPCAL0_A) -> Self {
        variant as u8 != 0
    }
}
impl DPCAL0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DPCAL0_A {
        match self.bits {
            false => DPCAL0_A::VALUE1,
            true => DPCAL0_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == DPCAL0_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == DPCAL0_A::VALUE2
    }
}
#[doc = "Field `DPCAL0` writer - Disable Post-Calibration"]
pub type DPCAL0_W<'a, const O: u8> = crate::BitWriter<'a, u32, GLOBCFG_SPEC, DPCAL0_A, O>;
impl<'a, const O: u8> DPCAL0_W<'a, O> {
    #[doc = "Automatic post-calibration after each conversion of group x"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(DPCAL0_A::VALUE1)
    }
    #[doc = "No post-calibration"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(DPCAL0_A::VALUE2)
    }
}
#[doc = "Field `DPCAL1` reader - Disable Post-Calibration"]
pub type DPCAL1_R = crate::BitReader<DPCAL1_A>;
#[doc = "Disable Post-Calibration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DPCAL1_A {
    #[doc = "0: Automatic post-calibration after each conversion of group x"]
    VALUE1 = 0,
    #[doc = "1: No post-calibration"]
    VALUE2 = 1,
}
impl From<DPCAL1_A> for bool {
    #[inline(always)]
    fn from(variant: DPCAL1_A) -> Self {
        variant as u8 != 0
    }
}
impl DPCAL1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DPCAL1_A {
        match self.bits {
            false => DPCAL1_A::VALUE1,
            true => DPCAL1_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == DPCAL1_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == DPCAL1_A::VALUE2
    }
}
#[doc = "Field `DPCAL1` writer - Disable Post-Calibration"]
pub type DPCAL1_W<'a, const O: u8> = crate::BitWriter<'a, u32, GLOBCFG_SPEC, DPCAL1_A, O>;
impl<'a, const O: u8> DPCAL1_W<'a, O> {
    #[doc = "Automatic post-calibration after each conversion of group x"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(DPCAL1_A::VALUE1)
    }
    #[doc = "No post-calibration"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(DPCAL1_A::VALUE2)
    }
}
#[doc = "Field `DPCAL2` reader - Disable Post-Calibration"]
pub type DPCAL2_R = crate::BitReader<DPCAL2_A>;
#[doc = "Disable Post-Calibration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DPCAL2_A {
    #[doc = "0: Automatic post-calibration after each conversion of group x"]
    VALUE1 = 0,
    #[doc = "1: No post-calibration"]
    VALUE2 = 1,
}
impl From<DPCAL2_A> for bool {
    #[inline(always)]
    fn from(variant: DPCAL2_A) -> Self {
        variant as u8 != 0
    }
}
impl DPCAL2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DPCAL2_A {
        match self.bits {
            false => DPCAL2_A::VALUE1,
            true => DPCAL2_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == DPCAL2_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == DPCAL2_A::VALUE2
    }
}
#[doc = "Field `DPCAL2` writer - Disable Post-Calibration"]
pub type DPCAL2_W<'a, const O: u8> = crate::BitWriter<'a, u32, GLOBCFG_SPEC, DPCAL2_A, O>;
impl<'a, const O: u8> DPCAL2_W<'a, O> {
    #[doc = "Automatic post-calibration after each conversion of group x"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(DPCAL2_A::VALUE1)
    }
    #[doc = "No post-calibration"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(DPCAL2_A::VALUE2)
    }
}
#[doc = "Field `DPCAL3` reader - Disable Post-Calibration"]
pub type DPCAL3_R = crate::BitReader<DPCAL3_A>;
#[doc = "Disable Post-Calibration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DPCAL3_A {
    #[doc = "0: Automatic post-calibration after each conversion of group x"]
    VALUE1 = 0,
    #[doc = "1: No post-calibration"]
    VALUE2 = 1,
}
impl From<DPCAL3_A> for bool {
    #[inline(always)]
    fn from(variant: DPCAL3_A) -> Self {
        variant as u8 != 0
    }
}
impl DPCAL3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DPCAL3_A {
        match self.bits {
            false => DPCAL3_A::VALUE1,
            true => DPCAL3_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == DPCAL3_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == DPCAL3_A::VALUE2
    }
}
#[doc = "Field `DPCAL3` writer - Disable Post-Calibration"]
pub type DPCAL3_W<'a, const O: u8> = crate::BitWriter<'a, u32, GLOBCFG_SPEC, DPCAL3_A, O>;
impl<'a, const O: u8> DPCAL3_W<'a, O> {
    #[doc = "Automatic post-calibration after each conversion of group x"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(DPCAL3_A::VALUE1)
    }
    #[doc = "No post-calibration"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(DPCAL3_A::VALUE2)
    }
}
#[doc = "Start-Up Calibration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SUCAL_AW {
    #[doc = "0: No action"]
    VALUE1 = 0,
    #[doc = "1: Initiate the start-up calibration phase (indication in bit GxARBCFG.CAL)"]
    VALUE2 = 1,
}
impl From<SUCAL_AW> for bool {
    #[inline(always)]
    fn from(variant: SUCAL_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SUCAL` writer - Start-Up Calibration"]
pub type SUCAL_W<'a, const O: u8> = crate::BitWriter<'a, u32, GLOBCFG_SPEC, SUCAL_AW, O>;
impl<'a, const O: u8> SUCAL_W<'a, O> {
    #[doc = "No action"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(SUCAL_AW::VALUE1)
    }
    #[doc = "Initiate the start-up calibration phase (indication in bit GxARBCFG.CAL)"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(SUCAL_AW::VALUE2)
    }
}
impl R {
    #[doc = "Bits 0:4 - Divider Factor for the Analog Internal Clock"]
    #[inline(always)]
    pub fn diva(&self) -> DIVA_R {
        DIVA_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 7 - Double Clock for the MSB Conversion"]
    #[inline(always)]
    pub fn dcmsb(&self) -> DCMSB_R {
        DCMSB_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - Divider Factor for the Arbiter Clock"]
    #[inline(always)]
    pub fn divd(&self) -> DIVD_R {
        DIVD_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 16 - Disable Post-Calibration"]
    #[inline(always)]
    pub fn dpcal0(&self) -> DPCAL0_R {
        DPCAL0_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Disable Post-Calibration"]
    #[inline(always)]
    pub fn dpcal1(&self) -> DPCAL1_R {
        DPCAL1_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Disable Post-Calibration"]
    #[inline(always)]
    pub fn dpcal2(&self) -> DPCAL2_R {
        DPCAL2_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Disable Post-Calibration"]
    #[inline(always)]
    pub fn dpcal3(&self) -> DPCAL3_R {
        DPCAL3_R::new(((self.bits >> 19) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - Divider Factor for the Analog Internal Clock"]
    #[inline(always)]
    #[must_use]
    pub fn diva(&mut self) -> DIVA_W<0> {
        DIVA_W::new(self)
    }
    #[doc = "Bit 7 - Double Clock for the MSB Conversion"]
    #[inline(always)]
    #[must_use]
    pub fn dcmsb(&mut self) -> DCMSB_W<7> {
        DCMSB_W::new(self)
    }
    #[doc = "Bits 8:9 - Divider Factor for the Arbiter Clock"]
    #[inline(always)]
    #[must_use]
    pub fn divd(&mut self) -> DIVD_W<8> {
        DIVD_W::new(self)
    }
    #[doc = "Bit 15 - Write Control for Divider Parameters"]
    #[inline(always)]
    #[must_use]
    pub fn divwc(&mut self) -> DIVWC_W<15> {
        DIVWC_W::new(self)
    }
    #[doc = "Bit 16 - Disable Post-Calibration"]
    #[inline(always)]
    #[must_use]
    pub fn dpcal0(&mut self) -> DPCAL0_W<16> {
        DPCAL0_W::new(self)
    }
    #[doc = "Bit 17 - Disable Post-Calibration"]
    #[inline(always)]
    #[must_use]
    pub fn dpcal1(&mut self) -> DPCAL1_W<17> {
        DPCAL1_W::new(self)
    }
    #[doc = "Bit 18 - Disable Post-Calibration"]
    #[inline(always)]
    #[must_use]
    pub fn dpcal2(&mut self) -> DPCAL2_W<18> {
        DPCAL2_W::new(self)
    }
    #[doc = "Bit 19 - Disable Post-Calibration"]
    #[inline(always)]
    #[must_use]
    pub fn dpcal3(&mut self) -> DPCAL3_W<19> {
        DPCAL3_W::new(self)
    }
    #[doc = "Bit 31 - Start-Up Calibration"]
    #[inline(always)]
    #[must_use]
    pub fn sucal(&mut self) -> SUCAL_W<31> {
        SUCAL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Global Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [globcfg](index.html) module"]
pub struct GLOBCFG_SPEC;
impl crate::RegisterSpec for GLOBCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [globcfg::R](R) reader structure"]
impl crate::Readable for GLOBCFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [globcfg::W](W) writer structure"]
impl crate::Writable for GLOBCFG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GLOBCFG to value 0x0f"]
impl crate::Resettable for GLOBCFG_SPEC {
    const RESET_VALUE: Self::Ux = 0x0f;
}
