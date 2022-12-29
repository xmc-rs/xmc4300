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
#[doc = "Field `TX_BLOCK_SIZE` reader - Transfer Block Size"]
pub type TX_BLOCK_SIZE_R = crate::FieldReader<u16, u16>;
#[doc = "Field `TX_BLOCK_SIZE` writer - Transfer Block Size"]
pub type TX_BLOCK_SIZE_W<'a, const O: u8> = crate::FieldWriter<'a, u16, BLOCK_SIZE_SPEC, u16, u16, 12, O>;
#[doc = "Field `TX_BLOCK_SIZE_12` reader - Transfer Block Size 12th bit."]
pub type TX_BLOCK_SIZE_12_R = crate::BitReader<bool>;
#[doc = "Field `TX_BLOCK_SIZE_12` writer - Transfer Block Size 12th bit."]
pub type TX_BLOCK_SIZE_12_W<'a, const O: u8> = crate::BitWriter<'a, u16, BLOCK_SIZE_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:11 - Transfer Block Size"]
    #[inline(always)]
    pub fn tx_block_size(&self) -> TX_BLOCK_SIZE_R {
        TX_BLOCK_SIZE_R::new(self.bits & 0x0fff)
    }
    #[doc = "Bit 15 - Transfer Block Size 12th bit."]
    #[inline(always)]
    pub fn tx_block_size_12(&self) -> TX_BLOCK_SIZE_12_R {
        TX_BLOCK_SIZE_12_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:11 - Transfer Block Size"]
    #[inline(always)]
    #[must_use]
    pub fn tx_block_size(&mut self) -> TX_BLOCK_SIZE_W<0> {
        TX_BLOCK_SIZE_W::new(self)
    }
    #[doc = "Bit 15 - Transfer Block Size 12th bit."]
    #[inline(always)]
    #[must_use]
    pub fn tx_block_size_12(&mut self) -> TX_BLOCK_SIZE_12_W<15> {
        TX_BLOCK_SIZE_12_W::new(self)
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
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BLOCK_SIZE to value 0"]
impl crate::Resettable for BLOCK_SIZE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
