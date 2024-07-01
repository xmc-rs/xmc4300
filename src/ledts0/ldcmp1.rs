#[doc = "Register `LDCMP1` reader"]
pub type R = crate::R<LDCMP1_SPEC>;
#[doc = "Register `LDCMP1` writer"]
pub type W = crate::W<LDCMP1_SPEC>;
#[doc = "Field `CMP_LD4` reader - Compare Value for LED COL\\[x\\]"]
pub type CMP_LD4_R = crate::FieldReader;
#[doc = "Field `CMP_LD4` writer - Compare Value for LED COL\\[x\\]"]
pub type CMP_LD4_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `CMP_LD5` reader - Compare Value for LED COL\\[x\\]"]
pub type CMP_LD5_R = crate::FieldReader;
#[doc = "Field `CMP_LD5` writer - Compare Value for LED COL\\[x\\]"]
pub type CMP_LD5_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `CMP_LD6` reader - Compare Value for LED COL\\[x\\]"]
pub type CMP_LD6_R = crate::FieldReader;
#[doc = "Field `CMP_LD6` writer - Compare Value for LED COL\\[x\\]"]
pub type CMP_LD6_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `CMP_LDA_TSCOM` reader - Compare Value for LED COLA / Common Compare Value for Touch-sense Pad Turns"]
pub type CMP_LDA_TSCOM_R = crate::FieldReader;
#[doc = "Field `CMP_LDA_TSCOM` writer - Compare Value for LED COLA / Common Compare Value for Touch-sense Pad Turns"]
pub type CMP_LDA_TSCOM_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Compare Value for LED COL\\[x\\]"]
    #[inline(always)]
    pub fn cmp_ld4(&self) -> CMP_LD4_R {
        CMP_LD4_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Compare Value for LED COL\\[x\\]"]
    #[inline(always)]
    pub fn cmp_ld5(&self) -> CMP_LD5_R {
        CMP_LD5_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Compare Value for LED COL\\[x\\]"]
    #[inline(always)]
    pub fn cmp_ld6(&self) -> CMP_LD6_R {
        CMP_LD6_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Compare Value for LED COLA / Common Compare Value for Touch-sense Pad Turns"]
    #[inline(always)]
    pub fn cmp_lda_tscom(&self) -> CMP_LDA_TSCOM_R {
        CMP_LDA_TSCOM_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Compare Value for LED COL\\[x\\]"]
    #[inline(always)]
    #[must_use]
    pub fn cmp_ld4(&mut self) -> CMP_LD4_W<LDCMP1_SPEC> {
        CMP_LD4_W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Compare Value for LED COL\\[x\\]"]
    #[inline(always)]
    #[must_use]
    pub fn cmp_ld5(&mut self) -> CMP_LD5_W<LDCMP1_SPEC> {
        CMP_LD5_W::new(self, 8)
    }
    #[doc = "Bits 16:23 - Compare Value for LED COL\\[x\\]"]
    #[inline(always)]
    #[must_use]
    pub fn cmp_ld6(&mut self) -> CMP_LD6_W<LDCMP1_SPEC> {
        CMP_LD6_W::new(self, 16)
    }
    #[doc = "Bits 24:31 - Compare Value for LED COLA / Common Compare Value for Touch-sense Pad Turns"]
    #[inline(always)]
    #[must_use]
    pub fn cmp_lda_tscom(&mut self) -> CMP_LDA_TSCOM_W<LDCMP1_SPEC> {
        CMP_LDA_TSCOM_W::new(self, 24)
    }
}
#[doc = "LED Compare Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`ldcmp1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ldcmp1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LDCMP1_SPEC;
impl crate::RegisterSpec for LDCMP1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ldcmp1::R`](R) reader structure"]
impl crate::Readable for LDCMP1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ldcmp1::W`](W) writer structure"]
impl crate::Writable for LDCMP1_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LDCMP1 to value 0"]
impl crate::Resettable for LDCMP1_SPEC {
    const RESET_VALUE: u32 = 0;
}
