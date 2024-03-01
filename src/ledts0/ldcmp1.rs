#[doc = "Register `LDCMP1` reader"]
pub type R = crate::R<Ldcmp1Spec>;
#[doc = "Register `LDCMP1` writer"]
pub type W = crate::W<Ldcmp1Spec>;
#[doc = "Field `CMP_LD4` reader - Compare Value for LED COL\\[x\\]"]
pub type CmpLd4R = crate::FieldReader;
#[doc = "Field `CMP_LD4` writer - Compare Value for LED COL\\[x\\]"]
pub type CmpLd4W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `CMP_LD5` reader - Compare Value for LED COL\\[x\\]"]
pub type CmpLd5R = crate::FieldReader;
#[doc = "Field `CMP_LD5` writer - Compare Value for LED COL\\[x\\]"]
pub type CmpLd5W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `CMP_LD6` reader - Compare Value for LED COL\\[x\\]"]
pub type CmpLd6R = crate::FieldReader;
#[doc = "Field `CMP_LD6` writer - Compare Value for LED COL\\[x\\]"]
pub type CmpLd6W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `CMP_LDA_TSCOM` reader - Compare Value for LED COLA / Common Compare Value for Touch-sense Pad Turns"]
pub type CmpLdaTscomR = crate::FieldReader;
#[doc = "Field `CMP_LDA_TSCOM` writer - Compare Value for LED COLA / Common Compare Value for Touch-sense Pad Turns"]
pub type CmpLdaTscomW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Compare Value for LED COL\\[x\\]"]
    #[inline(always)]
    pub fn cmp_ld4(&self) -> CmpLd4R {
        CmpLd4R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Compare Value for LED COL\\[x\\]"]
    #[inline(always)]
    pub fn cmp_ld5(&self) -> CmpLd5R {
        CmpLd5R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Compare Value for LED COL\\[x\\]"]
    #[inline(always)]
    pub fn cmp_ld6(&self) -> CmpLd6R {
        CmpLd6R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Compare Value for LED COLA / Common Compare Value for Touch-sense Pad Turns"]
    #[inline(always)]
    pub fn cmp_lda_tscom(&self) -> CmpLdaTscomR {
        CmpLdaTscomR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Compare Value for LED COL\\[x\\]"]
    #[inline(always)]
    #[must_use]
    pub fn cmp_ld4(&mut self) -> CmpLd4W<Ldcmp1Spec> {
        CmpLd4W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Compare Value for LED COL\\[x\\]"]
    #[inline(always)]
    #[must_use]
    pub fn cmp_ld5(&mut self) -> CmpLd5W<Ldcmp1Spec> {
        CmpLd5W::new(self, 8)
    }
    #[doc = "Bits 16:23 - Compare Value for LED COL\\[x\\]"]
    #[inline(always)]
    #[must_use]
    pub fn cmp_ld6(&mut self) -> CmpLd6W<Ldcmp1Spec> {
        CmpLd6W::new(self, 16)
    }
    #[doc = "Bits 24:31 - Compare Value for LED COLA / Common Compare Value for Touch-sense Pad Turns"]
    #[inline(always)]
    #[must_use]
    pub fn cmp_lda_tscom(&mut self) -> CmpLdaTscomW<Ldcmp1Spec> {
        CmpLdaTscomW::new(self, 24)
    }
}
#[doc = "LED Compare Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ldcmp1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ldcmp1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ldcmp1Spec;
impl crate::RegisterSpec for Ldcmp1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ldcmp1::R`](R) reader structure"]
impl crate::Readable for Ldcmp1Spec {}
#[doc = "`write(|w| ..)` method takes [`ldcmp1::W`](W) writer structure"]
impl crate::Writable for Ldcmp1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LDCMP1 to value 0"]
impl crate::Resettable for Ldcmp1Spec {
    const RESET_VALUE: u32 = 0;
}
