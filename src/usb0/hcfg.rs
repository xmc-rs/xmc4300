#[doc = "Register `HCFG` reader"]
pub struct R(crate::R<HCFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HCFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HCFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HCFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HCFG` writer"]
pub struct W(crate::W<HCFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HCFG_SPEC>;
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
impl From<crate::W<HCFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HCFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "FS PHY Clock Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FSLSPCLKSEL_A {
    #[doc = "1: PHY clock is running at 48 MHz"]
    VALUE1 = 1,
}
impl From<FSLSPCLKSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: FSLSPCLKSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `FSLSPclkSel` reader - FS PHY Clock Select"]
pub struct FSLSPCLKSEL_R(crate::FieldReader<u8, FSLSPCLKSEL_A>);
impl FSLSPCLKSEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        FSLSPCLKSEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<FSLSPCLKSEL_A> {
        match self.bits {
            1 => Some(FSLSPCLKSEL_A::VALUE1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == FSLSPCLKSEL_A::VALUE1
    }
}
impl core::ops::Deref for FSLSPCLKSEL_R {
    type Target = crate::FieldReader<u8, FSLSPCLKSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FSLSPclkSel` writer - FS PHY Clock Select"]
pub struct FSLSPCLKSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> FSLSPCLKSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FSLSPCLKSEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "PHY clock is running at 48 MHz"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(FSLSPCLKSEL_A::VALUE1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
#[doc = "FS-Only Support\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FSLSSUPP_A {
    #[doc = "0: FS-only, connected device can supports also only FS."]
    VALUE1 = 0,
    #[doc = "1: FS-only, even if the connected device can support HS"]
    VALUE2 = 1,
}
impl From<FSLSSUPP_A> for bool {
    #[inline(always)]
    fn from(variant: FSLSSUPP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FSLSSupp` reader - FS-Only Support"]
pub struct FSLSSUPP_R(crate::FieldReader<bool, FSLSSUPP_A>);
impl FSLSSUPP_R {
    pub(crate) fn new(bits: bool) -> Self {
        FSLSSUPP_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FSLSSUPP_A {
        match self.bits {
            false => FSLSSUPP_A::VALUE1,
            true => FSLSSUPP_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == FSLSSUPP_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == FSLSSUPP_A::VALUE2
    }
}
impl core::ops::Deref for FSLSSUPP_R {
    type Target = crate::FieldReader<bool, FSLSSUPP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FSLSSupp` writer - FS-Only Support"]
pub struct FSLSSUPP_W<'a> {
    w: &'a mut W,
}
impl<'a> FSLSSUPP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FSLSSUPP_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "FS-only, connected device can supports also only FS."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(FSLSSUPP_A::VALUE1)
    }
    #[doc = "FS-only, even if the connected device can support HS"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(FSLSSUPP_A::VALUE2)
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
#[doc = "Field `DescDMA` reader - Enable Scatter/gather DMA in Host mode"]
pub struct DESCDMA_R(crate::FieldReader<bool, bool>);
impl DESCDMA_R {
    pub(crate) fn new(bits: bool) -> Self {
        DESCDMA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DESCDMA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DescDMA` writer - Enable Scatter/gather DMA in Host mode"]
pub struct DESCDMA_W<'a> {
    w: &'a mut W,
}
impl<'a> DESCDMA_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | ((value as u32 & 0x01) << 23);
        self.w
    }
}
#[doc = "Frame List Entries\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FRLISTEN_A {
    #[doc = "0: 8 Entries"]
    VALUE1 = 0,
    #[doc = "1: 16 Entries"]
    VALUE2 = 1,
    #[doc = "2: 32 Entries"]
    VALUE3 = 2,
    #[doc = "3: 64 Entries"]
    VALUE4 = 3,
}
impl From<FRLISTEN_A> for u8 {
    #[inline(always)]
    fn from(variant: FRLISTEN_A) -> Self {
        variant as _
    }
}
#[doc = "Field `FrListEn` reader - Frame List Entries"]
pub struct FRLISTEN_R(crate::FieldReader<u8, FRLISTEN_A>);
impl FRLISTEN_R {
    pub(crate) fn new(bits: u8) -> Self {
        FRLISTEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FRLISTEN_A {
        match self.bits {
            0 => FRLISTEN_A::VALUE1,
            1 => FRLISTEN_A::VALUE2,
            2 => FRLISTEN_A::VALUE3,
            3 => FRLISTEN_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == FRLISTEN_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == FRLISTEN_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        **self == FRLISTEN_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        **self == FRLISTEN_A::VALUE4
    }
}
impl core::ops::Deref for FRLISTEN_R {
    type Target = crate::FieldReader<u8, FRLISTEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FrListEn` writer - Frame List Entries"]
pub struct FRLISTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> FRLISTEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FRLISTEN_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "8 Entries"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(FRLISTEN_A::VALUE1)
    }
    #[doc = "16 Entries"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(FRLISTEN_A::VALUE2)
    }
    #[doc = "32 Entries"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(FRLISTEN_A::VALUE3)
    }
    #[doc = "64 Entries"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(FRLISTEN_A::VALUE4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | ((value as u32 & 0x03) << 24);
        self.w
    }
}
#[doc = "Field `PerSchedEna` reader - Enable Periodic Scheduling"]
pub struct PERSCHEDENA_R(crate::FieldReader<bool, bool>);
impl PERSCHEDENA_R {
    pub(crate) fn new(bits: bool) -> Self {
        PERSCHEDENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PERSCHEDENA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PerSchedEna` writer - Enable Periodic Scheduling"]
pub struct PERSCHEDENA_W<'a> {
    w: &'a mut W,
}
impl<'a> PERSCHEDENA_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | ((value as u32 & 0x01) << 26);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - FS PHY Clock Select"]
    #[inline(always)]
    pub fn fslspclk_sel(&self) -> FSLSPCLKSEL_R {
        FSLSPCLKSEL_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bit 2 - FS-Only Support"]
    #[inline(always)]
    pub fn fslssupp(&self) -> FSLSSUPP_R {
        FSLSSUPP_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Enable Scatter/gather DMA in Host mode"]
    #[inline(always)]
    pub fn desc_dma(&self) -> DESCDMA_R {
        DESCDMA_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bits 24:25 - Frame List Entries"]
    #[inline(always)]
    pub fn fr_list_en(&self) -> FRLISTEN_R {
        FRLISTEN_R::new(((self.bits >> 24) & 0x03) as u8)
    }
    #[doc = "Bit 26 - Enable Periodic Scheduling"]
    #[inline(always)]
    pub fn per_sched_ena(&self) -> PERSCHEDENA_R {
        PERSCHEDENA_R::new(((self.bits >> 26) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - FS PHY Clock Select"]
    #[inline(always)]
    pub fn fslspclk_sel(&mut self) -> FSLSPCLKSEL_W {
        FSLSPCLKSEL_W { w: self }
    }
    #[doc = "Bit 2 - FS-Only Support"]
    #[inline(always)]
    pub fn fslssupp(&mut self) -> FSLSSUPP_W {
        FSLSSUPP_W { w: self }
    }
    #[doc = "Bit 23 - Enable Scatter/gather DMA in Host mode"]
    #[inline(always)]
    pub fn desc_dma(&mut self) -> DESCDMA_W {
        DESCDMA_W { w: self }
    }
    #[doc = "Bits 24:25 - Frame List Entries"]
    #[inline(always)]
    pub fn fr_list_en(&mut self) -> FRLISTEN_W {
        FRLISTEN_W { w: self }
    }
    #[doc = "Bit 26 - Enable Periodic Scheduling"]
    #[inline(always)]
    pub fn per_sched_ena(&mut self) -> PERSCHEDENA_W {
        PERSCHEDENA_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Host Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hcfg](index.html) module"]
pub struct HCFG_SPEC;
impl crate::RegisterSpec for HCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hcfg::R](R) reader structure"]
impl crate::Readable for HCFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hcfg::W](W) writer structure"]
impl crate::Writable for HCFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HCFG to value 0x0200"]
impl crate::Resettable for HCFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0200
    }
}
