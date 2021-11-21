#[doc = "Register `BFLC` reader"]
pub struct R(crate::R<BFLC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BFLC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BFLC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BFLC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BFLC` writer"]
pub struct W(crate::W<BFLC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BFLC_SPEC>;
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
impl From<crate::W<BFLC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BFLC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Boundary Flag y Mode Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum BFM0_A {
    #[doc = "0: Disable boundary flag, BFLy is not changed"]
    VALUE1 = 0,
    #[doc = "1: Always enable boundary flag (follow compare results)"]
    VALUE2 = 1,
    #[doc = "2: Enable boundary flag while gate of source 0 is active, clear BFLy while gate is inactive"]
    VALUE3 = 2,
    #[doc = "3: Enable boundary flag while gate of source 1 is active, clear BFLy while gate is inactive"]
    VALUE4 = 3,
}
impl From<BFM0_A> for u8 {
    #[inline(always)]
    fn from(variant: BFM0_A) -> Self {
        variant as _
    }
}
#[doc = "Field `BFM0` reader - Boundary Flag y Mode Control"]
pub struct BFM0_R(crate::FieldReader<u8, BFM0_A>);
impl BFM0_R {
    pub(crate) fn new(bits: u8) -> Self {
        BFM0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<BFM0_A> {
        match self.bits {
            0 => Some(BFM0_A::VALUE1),
            1 => Some(BFM0_A::VALUE2),
            2 => Some(BFM0_A::VALUE3),
            3 => Some(BFM0_A::VALUE4),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == BFM0_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == BFM0_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        **self == BFM0_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        **self == BFM0_A::VALUE4
    }
}
impl core::ops::Deref for BFM0_R {
    type Target = crate::FieldReader<u8, BFM0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BFM0` writer - Boundary Flag y Mode Control"]
pub struct BFM0_W<'a> {
    w: &'a mut W,
}
impl<'a> BFM0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BFM0_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Disable boundary flag, BFLy is not changed"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(BFM0_A::VALUE1)
    }
    #[doc = "Always enable boundary flag (follow compare results)"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(BFM0_A::VALUE2)
    }
    #[doc = "Enable boundary flag while gate of source 0 is active, clear BFLy while gate is inactive"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(BFM0_A::VALUE3)
    }
    #[doc = "Enable boundary flag while gate of source 1 is active, clear BFLy while gate is inactive"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(BFM0_A::VALUE4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
#[doc = "Boundary Flag y Mode Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum BFM1_A {
    #[doc = "0: Disable boundary flag, BFLy is not changed"]
    VALUE1 = 0,
    #[doc = "1: Always enable boundary flag (follow compare results)"]
    VALUE2 = 1,
    #[doc = "2: Enable boundary flag while gate of source 0 is active, clear BFLy while gate is inactive"]
    VALUE3 = 2,
    #[doc = "3: Enable boundary flag while gate of source 1 is active, clear BFLy while gate is inactive"]
    VALUE4 = 3,
}
impl From<BFM1_A> for u8 {
    #[inline(always)]
    fn from(variant: BFM1_A) -> Self {
        variant as _
    }
}
#[doc = "Field `BFM1` reader - Boundary Flag y Mode Control"]
pub struct BFM1_R(crate::FieldReader<u8, BFM1_A>);
impl BFM1_R {
    pub(crate) fn new(bits: u8) -> Self {
        BFM1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<BFM1_A> {
        match self.bits {
            0 => Some(BFM1_A::VALUE1),
            1 => Some(BFM1_A::VALUE2),
            2 => Some(BFM1_A::VALUE3),
            3 => Some(BFM1_A::VALUE4),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == BFM1_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == BFM1_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        **self == BFM1_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        **self == BFM1_A::VALUE4
    }
}
impl core::ops::Deref for BFM1_R {
    type Target = crate::FieldReader<u8, BFM1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BFM1` writer - Boundary Flag y Mode Control"]
pub struct BFM1_W<'a> {
    w: &'a mut W,
}
impl<'a> BFM1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BFM1_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Disable boundary flag, BFLy is not changed"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(BFM1_A::VALUE1)
    }
    #[doc = "Always enable boundary flag (follow compare results)"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(BFM1_A::VALUE2)
    }
    #[doc = "Enable boundary flag while gate of source 0 is active, clear BFLy while gate is inactive"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(BFM1_A::VALUE3)
    }
    #[doc = "Enable boundary flag while gate of source 1 is active, clear BFLy while gate is inactive"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(BFM1_A::VALUE4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | ((value as u32 & 0x0f) << 4);
        self.w
    }
}
#[doc = "Boundary Flag y Mode Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum BFM2_A {
    #[doc = "0: Disable boundary flag, BFLy is not changed"]
    VALUE1 = 0,
    #[doc = "1: Always enable boundary flag (follow compare results)"]
    VALUE2 = 1,
    #[doc = "2: Enable boundary flag while gate of source 0 is active, clear BFLy while gate is inactive"]
    VALUE3 = 2,
    #[doc = "3: Enable boundary flag while gate of source 1 is active, clear BFLy while gate is inactive"]
    VALUE4 = 3,
}
impl From<BFM2_A> for u8 {
    #[inline(always)]
    fn from(variant: BFM2_A) -> Self {
        variant as _
    }
}
#[doc = "Field `BFM2` reader - Boundary Flag y Mode Control"]
pub struct BFM2_R(crate::FieldReader<u8, BFM2_A>);
impl BFM2_R {
    pub(crate) fn new(bits: u8) -> Self {
        BFM2_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<BFM2_A> {
        match self.bits {
            0 => Some(BFM2_A::VALUE1),
            1 => Some(BFM2_A::VALUE2),
            2 => Some(BFM2_A::VALUE3),
            3 => Some(BFM2_A::VALUE4),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == BFM2_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == BFM2_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        **self == BFM2_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        **self == BFM2_A::VALUE4
    }
}
impl core::ops::Deref for BFM2_R {
    type Target = crate::FieldReader<u8, BFM2_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BFM2` writer - Boundary Flag y Mode Control"]
pub struct BFM2_W<'a> {
    w: &'a mut W,
}
impl<'a> BFM2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BFM2_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Disable boundary flag, BFLy is not changed"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(BFM2_A::VALUE1)
    }
    #[doc = "Always enable boundary flag (follow compare results)"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(BFM2_A::VALUE2)
    }
    #[doc = "Enable boundary flag while gate of source 0 is active, clear BFLy while gate is inactive"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(BFM2_A::VALUE3)
    }
    #[doc = "Enable boundary flag while gate of source 1 is active, clear BFLy while gate is inactive"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(BFM2_A::VALUE4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | ((value as u32 & 0x0f) << 8);
        self.w
    }
}
#[doc = "Boundary Flag y Mode Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum BFM3_A {
    #[doc = "0: Disable boundary flag, BFLy is not changed"]
    VALUE1 = 0,
    #[doc = "1: Always enable boundary flag (follow compare results)"]
    VALUE2 = 1,
    #[doc = "2: Enable boundary flag while gate of source 0 is active, clear BFLy while gate is inactive"]
    VALUE3 = 2,
    #[doc = "3: Enable boundary flag while gate of source 1 is active, clear BFLy while gate is inactive"]
    VALUE4 = 3,
}
impl From<BFM3_A> for u8 {
    #[inline(always)]
    fn from(variant: BFM3_A) -> Self {
        variant as _
    }
}
#[doc = "Field `BFM3` reader - Boundary Flag y Mode Control"]
pub struct BFM3_R(crate::FieldReader<u8, BFM3_A>);
impl BFM3_R {
    pub(crate) fn new(bits: u8) -> Self {
        BFM3_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<BFM3_A> {
        match self.bits {
            0 => Some(BFM3_A::VALUE1),
            1 => Some(BFM3_A::VALUE2),
            2 => Some(BFM3_A::VALUE3),
            3 => Some(BFM3_A::VALUE4),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == BFM3_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == BFM3_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        **self == BFM3_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        **self == BFM3_A::VALUE4
    }
}
impl core::ops::Deref for BFM3_R {
    type Target = crate::FieldReader<u8, BFM3_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BFM3` writer - Boundary Flag y Mode Control"]
pub struct BFM3_W<'a> {
    w: &'a mut W,
}
impl<'a> BFM3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BFM3_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Disable boundary flag, BFLy is not changed"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(BFM3_A::VALUE1)
    }
    #[doc = "Always enable boundary flag (follow compare results)"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(BFM3_A::VALUE2)
    }
    #[doc = "Enable boundary flag while gate of source 0 is active, clear BFLy while gate is inactive"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(BFM3_A::VALUE3)
    }
    #[doc = "Enable boundary flag while gate of source 1 is active, clear BFLy while gate is inactive"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(BFM3_A::VALUE4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | ((value as u32 & 0x0f) << 12);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Boundary Flag y Mode Control"]
    #[inline(always)]
    pub fn bfm0(&self) -> BFM0_R {
        BFM0_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Boundary Flag y Mode Control"]
    #[inline(always)]
    pub fn bfm1(&self) -> BFM1_R {
        BFM1_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Boundary Flag y Mode Control"]
    #[inline(always)]
    pub fn bfm2(&self) -> BFM2_R {
        BFM2_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Boundary Flag y Mode Control"]
    #[inline(always)]
    pub fn bfm3(&self) -> BFM3_R {
        BFM3_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Boundary Flag y Mode Control"]
    #[inline(always)]
    pub fn bfm0(&mut self) -> BFM0_W {
        BFM0_W { w: self }
    }
    #[doc = "Bits 4:7 - Boundary Flag y Mode Control"]
    #[inline(always)]
    pub fn bfm1(&mut self) -> BFM1_W {
        BFM1_W { w: self }
    }
    #[doc = "Bits 8:11 - Boundary Flag y Mode Control"]
    #[inline(always)]
    pub fn bfm2(&mut self) -> BFM2_W {
        BFM2_W { w: self }
    }
    #[doc = "Bits 12:15 - Boundary Flag y Mode Control"]
    #[inline(always)]
    pub fn bfm3(&mut self) -> BFM3_W {
        BFM3_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Boundary Flag Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bflc](index.html) module"]
pub struct BFLC_SPEC;
impl crate::RegisterSpec for BFLC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bflc::R](R) reader structure"]
impl crate::Readable for BFLC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bflc::W](W) writer structure"]
impl crate::Writable for BFLC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BFLC to value 0"]
impl crate::Resettable for BFLC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
