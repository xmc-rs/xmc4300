#[doc = "Register `HASH_TABLE_LOW` reader"]
pub struct R(crate::R<HASH_TABLE_LOW_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HASH_TABLE_LOW_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HASH_TABLE_LOW_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HASH_TABLE_LOW_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HASH_TABLE_LOW` writer"]
pub struct W(crate::W<HASH_TABLE_LOW_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HASH_TABLE_LOW_SPEC>;
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
impl From<crate::W<HASH_TABLE_LOW_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HASH_TABLE_LOW_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HTL` reader - Hash Table Low"]
pub type HTL_R = crate::FieldReader<u32, u32>;
#[doc = "Field `HTL` writer - Hash Table Low"]
pub type HTL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, HASH_TABLE_LOW_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Hash Table Low"]
    #[inline(always)]
    pub fn htl(&self) -> HTL_R {
        HTL_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Hash Table Low"]
    #[inline(always)]
    #[must_use]
    pub fn htl(&mut self) -> HTL_W<0> {
        HTL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Hash Table Low Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hash_table_low](index.html) module"]
pub struct HASH_TABLE_LOW_SPEC;
impl crate::RegisterSpec for HASH_TABLE_LOW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hash_table_low::R](R) reader structure"]
impl crate::Readable for HASH_TABLE_LOW_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hash_table_low::W](W) writer structure"]
impl crate::Writable for HASH_TABLE_LOW_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HASH_TABLE_LOW to value 0"]
impl crate::Resettable for HASH_TABLE_LOW_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
