#[doc = "Register `EXOCON[%s]` reader"]
pub struct R(crate::R<EXOCON_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EXOCON_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EXOCON_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EXOCON_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EXOCON[%s]` writer"]
pub struct W(crate::W<EXOCON_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EXOCON_SPEC>;
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
impl From<crate::W<EXOCON_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EXOCON_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ISS` reader - Internal Trigger Source Selection"]
pub type ISS_R = crate::FieldReader<u8, ISS_A>;
#[doc = "Internal Trigger Source Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ISS_A {
    #[doc = "0: The peripheral trigger function is disabled"]
    VALUE1 = 0,
    #[doc = "1: Input ERU_OGUy1 is selected"]
    VALUE2 = 1,
    #[doc = "2: Input ERU_OGUy2 is selected"]
    VALUE3 = 2,
    #[doc = "3: Input ERU_OGUy3 is selected"]
    VALUE4 = 3,
}
impl From<ISS_A> for u8 {
    #[inline(always)]
    fn from(variant: ISS_A) -> Self {
        variant as _
    }
}
impl ISS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ISS_A {
        match self.bits {
            0 => ISS_A::VALUE1,
            1 => ISS_A::VALUE2,
            2 => ISS_A::VALUE3,
            3 => ISS_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == ISS_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == ISS_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == ISS_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == ISS_A::VALUE4
    }
}
#[doc = "Field `ISS` writer - Internal Trigger Source Selection"]
pub type ISS_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, EXOCON_SPEC, u8, ISS_A, 2, O>;
impl<'a, const O: u8> ISS_W<'a, O> {
    #[doc = "The peripheral trigger function is disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(ISS_A::VALUE1)
    }
    #[doc = "Input ERU_OGUy1 is selected"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(ISS_A::VALUE2)
    }
    #[doc = "Input ERU_OGUy2 is selected"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(ISS_A::VALUE3)
    }
    #[doc = "Input ERU_OGUy3 is selected"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(ISS_A::VALUE4)
    }
}
#[doc = "Field `GEEN` reader - Gating Event Enable"]
pub type GEEN_R = crate::BitReader<GEEN_A>;
#[doc = "Gating Event Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GEEN_A {
    #[doc = "0: The event detection is disabled"]
    VALUE1 = 0,
    #[doc = "1: The event detection is enabled"]
    VALUE2 = 1,
}
impl From<GEEN_A> for bool {
    #[inline(always)]
    fn from(variant: GEEN_A) -> Self {
        variant as u8 != 0
    }
}
impl GEEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GEEN_A {
        match self.bits {
            false => GEEN_A::VALUE1,
            true => GEEN_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == GEEN_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == GEEN_A::VALUE2
    }
}
#[doc = "Field `GEEN` writer - Gating Event Enable"]
pub type GEEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, EXOCON_SPEC, GEEN_A, O>;
impl<'a, const O: u8> GEEN_W<'a, O> {
    #[doc = "The event detection is disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(GEEN_A::VALUE1)
    }
    #[doc = "The event detection is enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(GEEN_A::VALUE2)
    }
}
#[doc = "Field `PDR` reader - Pattern Detection Result Flag"]
pub type PDR_R = crate::BitReader<PDR_A>;
#[doc = "Pattern Detection Result Flag\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PDR_A {
    #[doc = "0: A pattern miss is detected"]
    VALUE1 = 0,
    #[doc = "1: A pattern match is detected"]
    VALUE2 = 1,
}
impl From<PDR_A> for bool {
    #[inline(always)]
    fn from(variant: PDR_A) -> Self {
        variant as u8 != 0
    }
}
impl PDR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDR_A {
        match self.bits {
            false => PDR_A::VALUE1,
            true => PDR_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PDR_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PDR_A::VALUE2
    }
}
#[doc = "Field `GP` reader - Gating Selection for Pattern Detection Result"]
pub type GP_R = crate::FieldReader<u8, GP_A>;
#[doc = "Gating Selection for Pattern Detection Result\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum GP_A {
    #[doc = "0: ERU_GOUTy is always disabled and ERU_IOUTy can not be activated"]
    VALUE1 = 0,
    #[doc = "1: ERU_GOUTy is always enabled and ERU_IOUTy becomes activated with each activation of ERU_TOUTy"]
    VALUE2 = 1,
    #[doc = "2: ERU_GOUTy is equal to ERU_PDOUTy and ERU_IOUTy becomes activated with an activation of ERU_TOUTy while the desired pattern is detected (pattern match PDR = 1)"]
    VALUE3 = 2,
    #[doc = "3: ERU_GOUTy is inverted to ERU_PDOUTy and ERU_IOUTy becomes activated with an activation of ERU_TOUTy while the desired pattern is not detected (pattern miss PDR = 0)"]
    VALUE4 = 3,
}
impl From<GP_A> for u8 {
    #[inline(always)]
    fn from(variant: GP_A) -> Self {
        variant as _
    }
}
impl GP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GP_A {
        match self.bits {
            0 => GP_A::VALUE1,
            1 => GP_A::VALUE2,
            2 => GP_A::VALUE3,
            3 => GP_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == GP_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == GP_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == GP_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == GP_A::VALUE4
    }
}
#[doc = "Field `GP` writer - Gating Selection for Pattern Detection Result"]
pub type GP_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, EXOCON_SPEC, u8, GP_A, 2, O>;
impl<'a, const O: u8> GP_W<'a, O> {
    #[doc = "ERU_GOUTy is always disabled and ERU_IOUTy can not be activated"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(GP_A::VALUE1)
    }
    #[doc = "ERU_GOUTy is always enabled and ERU_IOUTy becomes activated with each activation of ERU_TOUTy"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(GP_A::VALUE2)
    }
    #[doc = "ERU_GOUTy is equal to ERU_PDOUTy and ERU_IOUTy becomes activated with an activation of ERU_TOUTy while the desired pattern is detected (pattern match PDR = 1)"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(GP_A::VALUE3)
    }
    #[doc = "ERU_GOUTy is inverted to ERU_PDOUTy and ERU_IOUTy becomes activated with an activation of ERU_TOUTy while the desired pattern is not detected (pattern miss PDR = 0)"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(GP_A::VALUE4)
    }
}
#[doc = "Field `IPEN0` reader - Pattern Detection Enable for ETL0"]
pub type IPEN0_R = crate::BitReader<IPEN0_A>;
#[doc = "Pattern Detection Enable for ETL0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IPEN0_A {
    #[doc = "0: Flag EXICONx.FL is excluded from the pattern detection"]
    VALUE1 = 0,
    #[doc = "1: Flag EXICONx.FL is included in the pattern detection"]
    VALUE2 = 1,
}
impl From<IPEN0_A> for bool {
    #[inline(always)]
    fn from(variant: IPEN0_A) -> Self {
        variant as u8 != 0
    }
}
impl IPEN0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IPEN0_A {
        match self.bits {
            false => IPEN0_A::VALUE1,
            true => IPEN0_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == IPEN0_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == IPEN0_A::VALUE2
    }
}
#[doc = "Field `IPEN0` writer - Pattern Detection Enable for ETL0"]
pub type IPEN0_W<'a, const O: u8> = crate::BitWriter<'a, u32, EXOCON_SPEC, IPEN0_A, O>;
impl<'a, const O: u8> IPEN0_W<'a, O> {
    #[doc = "Flag EXICONx.FL is excluded from the pattern detection"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(IPEN0_A::VALUE1)
    }
    #[doc = "Flag EXICONx.FL is included in the pattern detection"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(IPEN0_A::VALUE2)
    }
}
#[doc = "Field `IPEN1` reader - Pattern Detection Enable for ETL1"]
pub type IPEN1_R = crate::BitReader<IPEN1_A>;
#[doc = "Pattern Detection Enable for ETL1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IPEN1_A {
    #[doc = "0: Flag EXICONx.FL is excluded from the pattern detection"]
    VALUE1 = 0,
    #[doc = "1: Flag EXICONx.FL is included in the pattern detection"]
    VALUE2 = 1,
}
impl From<IPEN1_A> for bool {
    #[inline(always)]
    fn from(variant: IPEN1_A) -> Self {
        variant as u8 != 0
    }
}
impl IPEN1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IPEN1_A {
        match self.bits {
            false => IPEN1_A::VALUE1,
            true => IPEN1_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == IPEN1_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == IPEN1_A::VALUE2
    }
}
#[doc = "Field `IPEN1` writer - Pattern Detection Enable for ETL1"]
pub type IPEN1_W<'a, const O: u8> = crate::BitWriter<'a, u32, EXOCON_SPEC, IPEN1_A, O>;
impl<'a, const O: u8> IPEN1_W<'a, O> {
    #[doc = "Flag EXICONx.FL is excluded from the pattern detection"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(IPEN1_A::VALUE1)
    }
    #[doc = "Flag EXICONx.FL is included in the pattern detection"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(IPEN1_A::VALUE2)
    }
}
#[doc = "Field `IPEN2` reader - Pattern Detection Enable for ETL2"]
pub type IPEN2_R = crate::BitReader<IPEN2_A>;
#[doc = "Pattern Detection Enable for ETL2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IPEN2_A {
    #[doc = "0: Flag EXICONx.FL is excluded from the pattern detection"]
    VALUE1 = 0,
    #[doc = "1: Flag EXICONx.FL is included in the pattern detection"]
    VALUE2 = 1,
}
impl From<IPEN2_A> for bool {
    #[inline(always)]
    fn from(variant: IPEN2_A) -> Self {
        variant as u8 != 0
    }
}
impl IPEN2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IPEN2_A {
        match self.bits {
            false => IPEN2_A::VALUE1,
            true => IPEN2_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == IPEN2_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == IPEN2_A::VALUE2
    }
}
#[doc = "Field `IPEN2` writer - Pattern Detection Enable for ETL2"]
pub type IPEN2_W<'a, const O: u8> = crate::BitWriter<'a, u32, EXOCON_SPEC, IPEN2_A, O>;
impl<'a, const O: u8> IPEN2_W<'a, O> {
    #[doc = "Flag EXICONx.FL is excluded from the pattern detection"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(IPEN2_A::VALUE1)
    }
    #[doc = "Flag EXICONx.FL is included in the pattern detection"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(IPEN2_A::VALUE2)
    }
}
#[doc = "Field `IPEN3` reader - Pattern Detection Enable for ETL3"]
pub type IPEN3_R = crate::BitReader<IPEN3_A>;
#[doc = "Pattern Detection Enable for ETL3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IPEN3_A {
    #[doc = "0: Flag EXICONx.FL is excluded from the pattern detection"]
    VALUE1 = 0,
    #[doc = "1: Flag EXICONx.FL is included in the pattern detection"]
    VALUE2 = 1,
}
impl From<IPEN3_A> for bool {
    #[inline(always)]
    fn from(variant: IPEN3_A) -> Self {
        variant as u8 != 0
    }
}
impl IPEN3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IPEN3_A {
        match self.bits {
            false => IPEN3_A::VALUE1,
            true => IPEN3_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == IPEN3_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == IPEN3_A::VALUE2
    }
}
#[doc = "Field `IPEN3` writer - Pattern Detection Enable for ETL3"]
pub type IPEN3_W<'a, const O: u8> = crate::BitWriter<'a, u32, EXOCON_SPEC, IPEN3_A, O>;
impl<'a, const O: u8> IPEN3_W<'a, O> {
    #[doc = "Flag EXICONx.FL is excluded from the pattern detection"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(IPEN3_A::VALUE1)
    }
    #[doc = "Flag EXICONx.FL is included in the pattern detection"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(IPEN3_A::VALUE2)
    }
}
impl R {
    #[doc = "Bits 0:1 - Internal Trigger Source Selection"]
    #[inline(always)]
    pub fn iss(&self) -> ISS_R {
        ISS_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - Gating Event Enable"]
    #[inline(always)]
    pub fn geen(&self) -> GEEN_R {
        GEEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Pattern Detection Result Flag"]
    #[inline(always)]
    pub fn pdr(&self) -> PDR_R {
        PDR_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - Gating Selection for Pattern Detection Result"]
    #[inline(always)]
    pub fn gp(&self) -> GP_R {
        GP_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 12 - Pattern Detection Enable for ETL0"]
    #[inline(always)]
    pub fn ipen0(&self) -> IPEN0_R {
        IPEN0_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Pattern Detection Enable for ETL1"]
    #[inline(always)]
    pub fn ipen1(&self) -> IPEN1_R {
        IPEN1_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Pattern Detection Enable for ETL2"]
    #[inline(always)]
    pub fn ipen2(&self) -> IPEN2_R {
        IPEN2_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Pattern Detection Enable for ETL3"]
    #[inline(always)]
    pub fn ipen3(&self) -> IPEN3_R {
        IPEN3_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Internal Trigger Source Selection"]
    #[inline(always)]
    #[must_use]
    pub fn iss(&mut self) -> ISS_W<0> {
        ISS_W::new(self)
    }
    #[doc = "Bit 2 - Gating Event Enable"]
    #[inline(always)]
    #[must_use]
    pub fn geen(&mut self) -> GEEN_W<2> {
        GEEN_W::new(self)
    }
    #[doc = "Bits 4:5 - Gating Selection for Pattern Detection Result"]
    #[inline(always)]
    #[must_use]
    pub fn gp(&mut self) -> GP_W<4> {
        GP_W::new(self)
    }
    #[doc = "Bit 12 - Pattern Detection Enable for ETL0"]
    #[inline(always)]
    #[must_use]
    pub fn ipen0(&mut self) -> IPEN0_W<12> {
        IPEN0_W::new(self)
    }
    #[doc = "Bit 13 - Pattern Detection Enable for ETL1"]
    #[inline(always)]
    #[must_use]
    pub fn ipen1(&mut self) -> IPEN1_W<13> {
        IPEN1_W::new(self)
    }
    #[doc = "Bit 14 - Pattern Detection Enable for ETL2"]
    #[inline(always)]
    #[must_use]
    pub fn ipen2(&mut self) -> IPEN2_W<14> {
        IPEN2_W::new(self)
    }
    #[doc = "Bit 15 - Pattern Detection Enable for ETL3"]
    #[inline(always)]
    #[must_use]
    pub fn ipen3(&mut self) -> IPEN3_W<15> {
        IPEN3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Event Output Trigger Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [exocon](index.html) module"]
pub struct EXOCON_SPEC;
impl crate::RegisterSpec for EXOCON_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [exocon::R](R) reader structure"]
impl crate::Readable for EXOCON_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [exocon::W](W) writer structure"]
impl crate::Writable for EXOCON_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EXOCON[%s]
to value 0x08"]
impl crate::Resettable for EXOCON_SPEC {
    const RESET_VALUE: Self::Ux = 0x08;
}
