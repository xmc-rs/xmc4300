#[doc = "Register `MSID[%s]` reader"]
pub struct R(crate::R<MSID_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MSID_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MSID_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MSID_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `INDEX` reader - Message Pending Index"]
pub struct INDEX_R(crate::FieldReader<u8, u8>);
impl INDEX_R {
    pub(crate) fn new(bits: u8) -> Self {
        INDEX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INDEX_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:5 - Message Pending Index"]
    #[inline(always)]
    pub fn index(&self) -> INDEX_R {
        INDEX_R::new((self.bits & 0x3f) as u8)
    }
}
#[doc = "Message Index Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [msid](index.html) module"]
pub struct MSID_SPEC;
impl crate::RegisterSpec for MSID_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [msid::R](R) reader structure"]
impl crate::Readable for MSID_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets MSID[%s]
to value 0x20"]
impl crate::Resettable for MSID_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x20
    }
}
