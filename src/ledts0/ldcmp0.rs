#[doc = "Register `LDCMP0` reader"]
pub type R = crate::R<Ldcmp0Spec>;
#[doc = "Register `LDCMP0` writer"]
pub type W = crate::W<Ldcmp0Spec>;
#[doc = "Field `CMP_LD0` reader - Compare Value for LED COL\\[x\\]"]
pub type CmpLd0R = crate::FieldReader;
#[doc = "Field `CMP_LD0` writer - Compare Value for LED COL\\[x\\]"]
pub type CmpLd0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `CMP_LD1` reader - Compare Value for LED COL\\[x\\]"]
pub type CmpLd1R = crate::FieldReader;
#[doc = "Field `CMP_LD1` writer - Compare Value for LED COL\\[x\\]"]
pub type CmpLd1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `CMP_LD2` reader - Compare Value for LED COL\\[x\\]"]
pub type CmpLd2R = crate::FieldReader;
#[doc = "Field `CMP_LD2` writer - Compare Value for LED COL\\[x\\]"]
pub type CmpLd2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `CMP_LD3` reader - Compare Value for LED COL\\[x\\]"]
pub type CmpLd3R = crate::FieldReader;
#[doc = "Field `CMP_LD3` writer - Compare Value for LED COL\\[x\\]"]
pub type CmpLd3W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Compare Value for LED COL\\[x\\]"]
    #[inline(always)]
    pub fn cmp_ld0(&self) -> CmpLd0R {
        CmpLd0R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Compare Value for LED COL\\[x\\]"]
    #[inline(always)]
    pub fn cmp_ld1(&self) -> CmpLd1R {
        CmpLd1R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Compare Value for LED COL\\[x\\]"]
    #[inline(always)]
    pub fn cmp_ld2(&self) -> CmpLd2R {
        CmpLd2R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Compare Value for LED COL\\[x\\]"]
    #[inline(always)]
    pub fn cmp_ld3(&self) -> CmpLd3R {
        CmpLd3R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Compare Value for LED COL\\[x\\]"]
    #[inline(always)]
    #[must_use]
    pub fn cmp_ld0(&mut self) -> CmpLd0W<Ldcmp0Spec> {
        CmpLd0W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Compare Value for LED COL\\[x\\]"]
    #[inline(always)]
    #[must_use]
    pub fn cmp_ld1(&mut self) -> CmpLd1W<Ldcmp0Spec> {
        CmpLd1W::new(self, 8)
    }
    #[doc = "Bits 16:23 - Compare Value for LED COL\\[x\\]"]
    #[inline(always)]
    #[must_use]
    pub fn cmp_ld2(&mut self) -> CmpLd2W<Ldcmp0Spec> {
        CmpLd2W::new(self, 16)
    }
    #[doc = "Bits 24:31 - Compare Value for LED COL\\[x\\]"]
    #[inline(always)]
    #[must_use]
    pub fn cmp_ld3(&mut self) -> CmpLd3W<Ldcmp0Spec> {
        CmpLd3W::new(self, 24)
    }
}
#[doc = "LED Compare Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ldcmp0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ldcmp0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ldcmp0Spec;
impl crate::RegisterSpec for Ldcmp0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ldcmp0::R`](R) reader structure"]
impl crate::Readable for Ldcmp0Spec {}
#[doc = "`write(|w| ..)` method takes [`ldcmp0::W`](W) writer structure"]
impl crate::Writable for Ldcmp0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LDCMP0 to value 0"]
impl crate::Resettable for Ldcmp0Spec {
    const RESET_VALUE: u32 = 0;
}
