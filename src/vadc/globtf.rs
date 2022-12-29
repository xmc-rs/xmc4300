#[doc = "Register `GLOBTF` reader"]
pub struct R(crate::R<GLOBTF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GLOBTF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GLOBTF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GLOBTF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GLOBTF` writer"]
pub struct W(crate::W<GLOBTF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GLOBTF_SPEC>;
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
impl From<crate::W<GLOBTF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GLOBTF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CDGR` reader - Converter Diagnostics Group"]
pub type CDGR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CDGR` writer - Converter Diagnostics Group"]
pub type CDGR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GLOBTF_SPEC, u8, u8, 4, O>;
#[doc = "Field `CDEN` reader - Converter Diagnostics Enable"]
pub type CDEN_R = crate::BitReader<CDEN_A>;
#[doc = "Converter Diagnostics Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CDEN_A {
    #[doc = "0: All diagnostic pull devices are disconnected"]
    VALUE1 = 0,
    #[doc = "1: Diagnostic pull devices connected as selected by bitfield CDSEL"]
    VALUE2 = 1,
}
impl From<CDEN_A> for bool {
    #[inline(always)]
    fn from(variant: CDEN_A) -> Self {
        variant as u8 != 0
    }
}
impl CDEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CDEN_A {
        match self.bits {
            false => CDEN_A::VALUE1,
            true => CDEN_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CDEN_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CDEN_A::VALUE2
    }
}
#[doc = "Field `CDEN` writer - Converter Diagnostics Enable"]
pub type CDEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, GLOBTF_SPEC, CDEN_A, O>;
impl<'a, const O: u8> CDEN_W<'a, O> {
    #[doc = "All diagnostic pull devices are disconnected"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CDEN_A::VALUE1)
    }
    #[doc = "Diagnostic pull devices connected as selected by bitfield CDSEL"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CDEN_A::VALUE2)
    }
}
#[doc = "Field `CDSEL` reader - Converter Diagnostics Pull-Devices Select"]
pub type CDSEL_R = crate::FieldReader<u8, CDSEL_A>;
#[doc = "Converter Diagnostics Pull-Devices Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CDSEL_A {
    #[doc = "0: Connected to VAREF"]
    VALUE1 = 0,
    #[doc = "1: Connected to VAGND"]
    VALUE2 = 1,
    #[doc = "2: Connected to 1/3rd VAREF"]
    VALUE3 = 2,
    #[doc = "3: Connected to 2/3rd VAREF"]
    VALUE4 = 3,
}
impl From<CDSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: CDSEL_A) -> Self {
        variant as _
    }
}
impl CDSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CDSEL_A {
        match self.bits {
            0 => CDSEL_A::VALUE1,
            1 => CDSEL_A::VALUE2,
            2 => CDSEL_A::VALUE3,
            3 => CDSEL_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CDSEL_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CDSEL_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == CDSEL_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == CDSEL_A::VALUE4
    }
}
#[doc = "Field `CDSEL` writer - Converter Diagnostics Pull-Devices Select"]
pub type CDSEL_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, GLOBTF_SPEC, u8, CDSEL_A, 2, O>;
impl<'a, const O: u8> CDSEL_W<'a, O> {
    #[doc = "Connected to VAREF"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CDSEL_A::VALUE1)
    }
    #[doc = "Connected to VAGND"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CDSEL_A::VALUE2)
    }
    #[doc = "Connected to 1/3rd VAREF"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(CDSEL_A::VALUE3)
    }
    #[doc = "Connected to 2/3rd VAREF"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(CDSEL_A::VALUE4)
    }
}
#[doc = "Write Control for Conversion Diagnostics\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CDWC_AW {
    #[doc = "0: No write access to parameters"]
    VALUE1 = 0,
    #[doc = "1: Bitfields CDSEL, CDEN, CDGR can be written"]
    VALUE2 = 1,
}
impl From<CDWC_AW> for bool {
    #[inline(always)]
    fn from(variant: CDWC_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CDWC` writer - Write Control for Conversion Diagnostics"]
pub type CDWC_W<'a, const O: u8> = crate::BitWriter<'a, u32, GLOBTF_SPEC, CDWC_AW, O>;
impl<'a, const O: u8> CDWC_W<'a, O> {
    #[doc = "No write access to parameters"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CDWC_AW::VALUE1)
    }
    #[doc = "Bitfields CDSEL, CDEN, CDGR can be written"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CDWC_AW::VALUE2)
    }
}
#[doc = "Field `PDD` reader - Pull-Down Diagnostics Enable"]
pub type PDD_R = crate::BitReader<PDD_A>;
#[doc = "Pull-Down Diagnostics Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PDD_A {
    #[doc = "0: Disconnected"]
    VALUE1 = 0,
    #[doc = "1: The pull-down diagnostics device is active"]
    VALUE2 = 1,
}
impl From<PDD_A> for bool {
    #[inline(always)]
    fn from(variant: PDD_A) -> Self {
        variant as u8 != 0
    }
}
impl PDD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDD_A {
        match self.bits {
            false => PDD_A::VALUE1,
            true => PDD_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PDD_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PDD_A::VALUE2
    }
}
#[doc = "Field `PDD` writer - Pull-Down Diagnostics Enable"]
pub type PDD_W<'a, const O: u8> = crate::BitWriter<'a, u32, GLOBTF_SPEC, PDD_A, O>;
impl<'a, const O: u8> PDD_W<'a, O> {
    #[doc = "Disconnected"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(PDD_A::VALUE1)
    }
    #[doc = "The pull-down diagnostics device is active"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(PDD_A::VALUE2)
    }
}
#[doc = "Write Control for Multiplexer Diagnostics\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MDWC_AW {
    #[doc = "0: No write access to parameters"]
    VALUE1 = 0,
    #[doc = "1: Bitfield PDD can be written"]
    VALUE2 = 1,
}
impl From<MDWC_AW> for bool {
    #[inline(always)]
    fn from(variant: MDWC_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MDWC` writer - Write Control for Multiplexer Diagnostics"]
pub type MDWC_W<'a, const O: u8> = crate::BitWriter<'a, u32, GLOBTF_SPEC, MDWC_AW, O>;
impl<'a, const O: u8> MDWC_W<'a, O> {
    #[doc = "No write access to parameters"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(MDWC_AW::VALUE1)
    }
    #[doc = "Bitfield PDD can be written"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(MDWC_AW::VALUE2)
    }
}
impl R {
    #[doc = "Bits 4:7 - Converter Diagnostics Group"]
    #[inline(always)]
    pub fn cdgr(&self) -> CDGR_R {
        CDGR_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - Converter Diagnostics Enable"]
    #[inline(always)]
    pub fn cden(&self) -> CDEN_R {
        CDEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:10 - Converter Diagnostics Pull-Devices Select"]
    #[inline(always)]
    pub fn cdsel(&self) -> CDSEL_R {
        CDSEL_R::new(((self.bits >> 9) & 3) as u8)
    }
    #[doc = "Bit 16 - Pull-Down Diagnostics Enable"]
    #[inline(always)]
    pub fn pdd(&self) -> PDD_R {
        PDD_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 4:7 - Converter Diagnostics Group"]
    #[inline(always)]
    #[must_use]
    pub fn cdgr(&mut self) -> CDGR_W<4> {
        CDGR_W::new(self)
    }
    #[doc = "Bit 8 - Converter Diagnostics Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cden(&mut self) -> CDEN_W<8> {
        CDEN_W::new(self)
    }
    #[doc = "Bits 9:10 - Converter Diagnostics Pull-Devices Select"]
    #[inline(always)]
    #[must_use]
    pub fn cdsel(&mut self) -> CDSEL_W<9> {
        CDSEL_W::new(self)
    }
    #[doc = "Bit 15 - Write Control for Conversion Diagnostics"]
    #[inline(always)]
    #[must_use]
    pub fn cdwc(&mut self) -> CDWC_W<15> {
        CDWC_W::new(self)
    }
    #[doc = "Bit 16 - Pull-Down Diagnostics Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pdd(&mut self) -> PDD_W<16> {
        PDD_W::new(self)
    }
    #[doc = "Bit 23 - Write Control for Multiplexer Diagnostics"]
    #[inline(always)]
    #[must_use]
    pub fn mdwc(&mut self) -> MDWC_W<23> {
        MDWC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Global Test Functions Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [globtf](index.html) module"]
pub struct GLOBTF_SPEC;
impl crate::RegisterSpec for GLOBTF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [globtf::R](R) reader structure"]
impl crate::Readable for GLOBTF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [globtf::W](W) writer structure"]
impl crate::Writable for GLOBTF_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GLOBTF to value 0"]
impl crate::Resettable for GLOBTF_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
