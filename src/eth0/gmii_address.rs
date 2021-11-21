#[doc = "Register `GMII_ADDRESS` reader"]
pub struct R(crate::R<GMII_ADDRESS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GMII_ADDRESS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GMII_ADDRESS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GMII_ADDRESS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GMII_ADDRESS` writer"]
pub struct W(crate::W<GMII_ADDRESS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GMII_ADDRESS_SPEC>;
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
impl From<crate::W<GMII_ADDRESS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GMII_ADDRESS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MB` reader - MII Busy"]
pub struct MB_R(crate::FieldReader<bool, bool>);
impl MB_R {
    pub(crate) fn new(bits: bool) -> Self {
        MB_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MB_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MB` writer - MII Busy"]
pub struct MB_W<'a> {
    w: &'a mut W,
}
impl<'a> MB_W<'a> {
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
#[doc = "Field `MW` reader - MII Write"]
pub struct MW_R(crate::FieldReader<bool, bool>);
impl MW_R {
    pub(crate) fn new(bits: bool) -> Self {
        MW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MW` writer - MII Write"]
pub struct MW_W<'a> {
    w: &'a mut W,
}
impl<'a> MW_W<'a> {
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
#[doc = "Field `CR` reader - CSR Clock Range"]
pub struct CR_R(crate::FieldReader<u8, u8>);
impl CR_R {
    pub(crate) fn new(bits: u8) -> Self {
        CR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CR` writer - CSR Clock Range"]
pub struct CR_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 2)) | ((value as u32 & 0x0f) << 2);
        self.w
    }
}
#[doc = "Field `MR` reader - MII Register"]
pub struct MR_R(crate::FieldReader<u8, u8>);
impl MR_R {
    pub(crate) fn new(bits: u8) -> Self {
        MR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MR` writer - MII Register"]
pub struct MR_W<'a> {
    w: &'a mut W,
}
impl<'a> MR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 6)) | ((value as u32 & 0x1f) << 6);
        self.w
    }
}
#[doc = "Field `PA` reader - Physical Layer Address"]
pub struct PA_R(crate::FieldReader<u8, u8>);
impl PA_R {
    pub(crate) fn new(bits: u8) -> Self {
        PA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PA_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PA` writer - Physical Layer Address"]
pub struct PA_W<'a> {
    w: &'a mut W,
}
impl<'a> PA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 11)) | ((value as u32 & 0x1f) << 11);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - MII Busy"]
    #[inline(always)]
    pub fn mb(&self) -> MB_R {
        MB_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - MII Write"]
    #[inline(always)]
    pub fn mw(&self) -> MW_R {
        MW_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bits 2:5 - CSR Clock Range"]
    #[inline(always)]
    pub fn cr(&self) -> CR_R {
        CR_R::new(((self.bits >> 2) & 0x0f) as u8)
    }
    #[doc = "Bits 6:10 - MII Register"]
    #[inline(always)]
    pub fn mr(&self) -> MR_R {
        MR_R::new(((self.bits >> 6) & 0x1f) as u8)
    }
    #[doc = "Bits 11:15 - Physical Layer Address"]
    #[inline(always)]
    pub fn pa(&self) -> PA_R {
        PA_R::new(((self.bits >> 11) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - MII Busy"]
    #[inline(always)]
    pub fn mb(&mut self) -> MB_W {
        MB_W { w: self }
    }
    #[doc = "Bit 1 - MII Write"]
    #[inline(always)]
    pub fn mw(&mut self) -> MW_W {
        MW_W { w: self }
    }
    #[doc = "Bits 2:5 - CSR Clock Range"]
    #[inline(always)]
    pub fn cr(&mut self) -> CR_W {
        CR_W { w: self }
    }
    #[doc = "Bits 6:10 - MII Register"]
    #[inline(always)]
    pub fn mr(&mut self) -> MR_W {
        MR_W { w: self }
    }
    #[doc = "Bits 11:15 - Physical Layer Address"]
    #[inline(always)]
    pub fn pa(&mut self) -> PA_W {
        PA_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MII Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gmii_address](index.html) module"]
pub struct GMII_ADDRESS_SPEC;
impl crate::RegisterSpec for GMII_ADDRESS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gmii_address::R](R) reader structure"]
impl crate::Readable for GMII_ADDRESS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gmii_address::W](W) writer structure"]
impl crate::Writable for GMII_ADDRESS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets GMII_ADDRESS to value 0"]
impl crate::Resettable for GMII_ADDRESS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
