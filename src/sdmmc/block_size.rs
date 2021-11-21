#[doc = "Register `BLOCK_SIZE` reader"]
pub struct R(crate::R<BLOCK_SIZE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BLOCK_SIZE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BLOCK_SIZE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BLOCK_SIZE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BLOCK_SIZE` writer"]
pub struct W(crate::W<BLOCK_SIZE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BLOCK_SIZE_SPEC>;
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
impl From<crate::W<BLOCK_SIZE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BLOCK_SIZE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TX_BLOCK_SIZE_12` reader - Transfer Block Size 12th bit."]
pub struct TX_BLOCK_SIZE_12_R(crate::FieldReader<bool, bool>);
impl TX_BLOCK_SIZE_12_R {
    pub(crate) fn new(bits: bool) -> Self {
        TX_BLOCK_SIZE_12_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_BLOCK_SIZE_12_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_BLOCK_SIZE_12` writer - Transfer Block Size 12th bit."]
pub struct TX_BLOCK_SIZE_12_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_BLOCK_SIZE_12_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | ((value as u16 & 0x01) << 15);
        self.w
    }
}
#[doc = "Field `TX_BLOCK_SIZE` reader - Transfer Block Size"]
pub struct TX_BLOCK_SIZE_R(crate::FieldReader<u16, u16>);
impl TX_BLOCK_SIZE_R {
    pub(crate) fn new(bits: u16) -> Self {
        TX_BLOCK_SIZE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_BLOCK_SIZE_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_BLOCK_SIZE` writer - Transfer Block Size"]
pub struct TX_BLOCK_SIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_BLOCK_SIZE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | (value as u16 & 0x0fff);
        self.w
    }
}
impl R {
    #[doc = "Bit 15 - Transfer Block Size 12th bit."]
    #[inline(always)]
    pub fn tx_block_size_12(&self) -> TX_BLOCK_SIZE_12_R {
        TX_BLOCK_SIZE_12_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 0:11 - Transfer Block Size"]
    #[inline(always)]
    pub fn tx_block_size(&self) -> TX_BLOCK_SIZE_R {
        TX_BLOCK_SIZE_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bit 15 - Transfer Block Size 12th bit."]
    #[inline(always)]
    pub fn tx_block_size_12(&mut self) -> TX_BLOCK_SIZE_12_W {
        TX_BLOCK_SIZE_12_W { w: self }
    }
    #[doc = "Bits 0:11 - Transfer Block Size"]
    #[inline(always)]
    pub fn tx_block_size(&mut self) -> TX_BLOCK_SIZE_W {
        TX_BLOCK_SIZE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Block Size Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [block_size](index.html) module"]
pub struct BLOCK_SIZE_SPEC;
impl crate::RegisterSpec for BLOCK_SIZE_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [block_size::R](R) reader structure"]
impl crate::Readable for BLOCK_SIZE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [block_size::W](W) writer structure"]
impl crate::Writable for BLOCK_SIZE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BLOCK_SIZE to value 0"]
impl crate::Resettable for BLOCK_SIZE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
