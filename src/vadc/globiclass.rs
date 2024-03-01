#[doc = "Register `GLOBICLASS[%s]` reader"]
pub type R = crate::R<GlobiclassSpec>;
#[doc = "Register `GLOBICLASS[%s]` writer"]
pub type W = crate::W<GlobiclassSpec>;
#[doc = "Field `STCS` reader - Sample Time Control for Standard Conversions"]
pub type StcsR = crate::FieldReader;
#[doc = "Field `STCS` writer - Sample Time Control for Standard Conversions"]
pub type StcsW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Conversion Mode for Standard Conversions\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cms {
    #[doc = "0: 12-bit conversion"]
    Value1 = 0,
    #[doc = "1: 10-bit conversion"]
    Value2 = 1,
    #[doc = "2: 8-bit conversion"]
    Value3 = 2,
    #[doc = "5: 10-bit fast compare mode"]
    Value6 = 5,
}
impl From<Cms> for u8 {
    #[inline(always)]
    fn from(variant: Cms) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cms {
    type Ux = u8;
}
#[doc = "Field `CMS` reader - Conversion Mode for Standard Conversions"]
pub type CmsR = crate::FieldReader<Cms>;
impl CmsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Cms> {
        match self.bits {
            0 => Some(Cms::Value1),
            1 => Some(Cms::Value2),
            2 => Some(Cms::Value3),
            5 => Some(Cms::Value6),
            _ => None,
        }
    }
    #[doc = "12-bit conversion"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Cms::Value1
    }
    #[doc = "10-bit conversion"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Cms::Value2
    }
    #[doc = "8-bit conversion"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == Cms::Value3
    }
    #[doc = "10-bit fast compare mode"]
    #[inline(always)]
    pub fn is_value6(&self) -> bool {
        *self == Cms::Value6
    }
}
#[doc = "Field `CMS` writer - Conversion Mode for Standard Conversions"]
pub type CmsW<'a, REG> = crate::FieldWriter<'a, REG, 3, Cms>;
impl<'a, REG> CmsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "12-bit conversion"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Cms::Value1)
    }
    #[doc = "10-bit conversion"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Cms::Value2)
    }
    #[doc = "8-bit conversion"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(Cms::Value3)
    }
    #[doc = "10-bit fast compare mode"]
    #[inline(always)]
    pub fn value6(self) -> &'a mut crate::W<REG> {
        self.variant(Cms::Value6)
    }
}
#[doc = "Field `STCE` reader - Sample Time Control for EMUX Conversions"]
pub type StceR = crate::FieldReader;
#[doc = "Field `STCE` writer - Sample Time Control for EMUX Conversions"]
pub type StceW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Conversion Mode for EMUX Conversions\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cme {
    #[doc = "0: 12-bit conversion"]
    Value1 = 0,
    #[doc = "1: 10-bit conversion"]
    Value2 = 1,
    #[doc = "2: 8-bit conversion"]
    Value3 = 2,
    #[doc = "5: 10-bit fast compare mode"]
    Value6 = 5,
}
impl From<Cme> for u8 {
    #[inline(always)]
    fn from(variant: Cme) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cme {
    type Ux = u8;
}
#[doc = "Field `CME` reader - Conversion Mode for EMUX Conversions"]
pub type CmeR = crate::FieldReader<Cme>;
impl CmeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Cme> {
        match self.bits {
            0 => Some(Cme::Value1),
            1 => Some(Cme::Value2),
            2 => Some(Cme::Value3),
            5 => Some(Cme::Value6),
            _ => None,
        }
    }
    #[doc = "12-bit conversion"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Cme::Value1
    }
    #[doc = "10-bit conversion"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Cme::Value2
    }
    #[doc = "8-bit conversion"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == Cme::Value3
    }
    #[doc = "10-bit fast compare mode"]
    #[inline(always)]
    pub fn is_value6(&self) -> bool {
        *self == Cme::Value6
    }
}
#[doc = "Field `CME` writer - Conversion Mode for EMUX Conversions"]
pub type CmeW<'a, REG> = crate::FieldWriter<'a, REG, 3, Cme>;
impl<'a, REG> CmeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "12-bit conversion"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Cme::Value1)
    }
    #[doc = "10-bit conversion"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Cme::Value2)
    }
    #[doc = "8-bit conversion"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(Cme::Value3)
    }
    #[doc = "10-bit fast compare mode"]
    #[inline(always)]
    pub fn value6(self) -> &'a mut crate::W<REG> {
        self.variant(Cme::Value6)
    }
}
impl R {
    #[doc = "Bits 0:4 - Sample Time Control for Standard Conversions"]
    #[inline(always)]
    pub fn stcs(&self) -> StcsR {
        StcsR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:10 - Conversion Mode for Standard Conversions"]
    #[inline(always)]
    pub fn cms(&self) -> CmsR {
        CmsR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 16:20 - Sample Time Control for EMUX Conversions"]
    #[inline(always)]
    pub fn stce(&self) -> StceR {
        StceR::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:26 - Conversion Mode for EMUX Conversions"]
    #[inline(always)]
    pub fn cme(&self) -> CmeR {
        CmeR::new(((self.bits >> 24) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Sample Time Control for Standard Conversions"]
    #[inline(always)]
    #[must_use]
    pub fn stcs(&mut self) -> StcsW<GlobiclassSpec> {
        StcsW::new(self, 0)
    }
    #[doc = "Bits 8:10 - Conversion Mode for Standard Conversions"]
    #[inline(always)]
    #[must_use]
    pub fn cms(&mut self) -> CmsW<GlobiclassSpec> {
        CmsW::new(self, 8)
    }
    #[doc = "Bits 16:20 - Sample Time Control for EMUX Conversions"]
    #[inline(always)]
    #[must_use]
    pub fn stce(&mut self) -> StceW<GlobiclassSpec> {
        StceW::new(self, 16)
    }
    #[doc = "Bits 24:26 - Conversion Mode for EMUX Conversions"]
    #[inline(always)]
    #[must_use]
    pub fn cme(&mut self) -> CmeW<GlobiclassSpec> {
        CmeW::new(self, 24)
    }
}
#[doc = "Input Class Register, Global\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`globiclass::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`globiclass::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GlobiclassSpec;
impl crate::RegisterSpec for GlobiclassSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`globiclass::R`](R) reader structure"]
impl crate::Readable for GlobiclassSpec {}
#[doc = "`write(|w| ..)` method takes [`globiclass::W`](W) writer structure"]
impl crate::Writable for GlobiclassSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GLOBICLASS[%s]
to value 0"]
impl crate::Resettable for GlobiclassSpec {
    const RESET_VALUE: u32 = 0;
}
