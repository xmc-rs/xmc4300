#[doc = "Register `HASH_TABLE_HIGH` reader"]
pub struct R(crate::R<HASH_TABLE_HIGH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HASH_TABLE_HIGH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HASH_TABLE_HIGH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HASH_TABLE_HIGH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HASH_TABLE_HIGH` writer"]
pub struct W(crate::W<HASH_TABLE_HIGH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HASH_TABLE_HIGH_SPEC>;
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
impl From<crate::W<HASH_TABLE_HIGH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HASH_TABLE_HIGH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HTH` reader - Hash Table High"]
pub struct HTH_R(crate::FieldReader<u32, u32>);
impl HTH_R {
    pub(crate) fn new(bits: u32) -> Self {
        HTH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HTH_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HTH` writer - Hash Table High"]
pub struct HTH_W<'a> {
    w: &'a mut W,
}
impl<'a> HTH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Hash Table High"]
    #[inline(always)]
    pub fn hth(&self) -> HTH_R {
        HTH_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Hash Table High"]
    #[inline(always)]
    pub fn hth(&mut self) -> HTH_W {
        HTH_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Hash Table High Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hash_table_high](index.html) module"]
pub struct HASH_TABLE_HIGH_SPEC;
impl crate::RegisterSpec for HASH_TABLE_HIGH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hash_table_high::R](R) reader structure"]
impl crate::Readable for HASH_TABLE_HIGH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hash_table_high::W](W) writer structure"]
impl crate::Writable for HASH_TABLE_HIGH_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HASH_TABLE_HIGH to value 0"]
impl crate::Resettable for HASH_TABLE_HIGH_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
