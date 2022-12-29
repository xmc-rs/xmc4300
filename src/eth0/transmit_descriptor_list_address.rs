#[doc = "Register `TRANSMIT_DESCRIPTOR_LIST_ADDRESS` reader"]
pub struct R(crate::R<TRANSMIT_DESCRIPTOR_LIST_ADDRESS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TRANSMIT_DESCRIPTOR_LIST_ADDRESS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TRANSMIT_DESCRIPTOR_LIST_ADDRESS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TRANSMIT_DESCRIPTOR_LIST_ADDRESS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TRANSMIT_DESCRIPTOR_LIST_ADDRESS` writer"]
pub struct W(crate::W<TRANSMIT_DESCRIPTOR_LIST_ADDRESS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TRANSMIT_DESCRIPTOR_LIST_ADDRESS_SPEC>;
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
impl From<crate::W<TRANSMIT_DESCRIPTOR_LIST_ADDRESS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TRANSMIT_DESCRIPTOR_LIST_ADDRESS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TDESLA_32bit` reader - Start of Transmit List"]
pub type TDESLA_32BIT_R = crate::FieldReader<u32, u32>;
#[doc = "Field `TDESLA_32bit` writer - Start of Transmit List"]
pub type TDESLA_32BIT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TRANSMIT_DESCRIPTOR_LIST_ADDRESS_SPEC, u32, u32, 30, O>;
impl R {
    #[doc = "Bits 2:31 - Start of Transmit List"]
    #[inline(always)]
    pub fn tdesla_32bit(&self) -> TDESLA_32BIT_R {
        TDESLA_32BIT_R::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bits 2:31 - Start of Transmit List"]
    #[inline(always)]
    #[must_use]
    pub fn tdesla_32bit(&mut self) -> TDESLA_32BIT_W<2> {
        TDESLA_32BIT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Transmit descripter Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [transmit_descriptor_list_address](index.html) module"]
pub struct TRANSMIT_DESCRIPTOR_LIST_ADDRESS_SPEC;
impl crate::RegisterSpec for TRANSMIT_DESCRIPTOR_LIST_ADDRESS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [transmit_descriptor_list_address::R](R) reader structure"]
impl crate::Readable for TRANSMIT_DESCRIPTOR_LIST_ADDRESS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [transmit_descriptor_list_address::W](W) writer structure"]
impl crate::Writable for TRANSMIT_DESCRIPTOR_LIST_ADDRESS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TRANSMIT_DESCRIPTOR_LIST_ADDRESS to value 0"]
impl crate::Resettable for TRANSMIT_DESCRIPTOR_LIST_ADDRESS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
