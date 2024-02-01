#[doc = "Register `LDCMP0` reader"]
pub type R = crate::R<LDCMP0_SPEC>;
#[doc = "Register `LDCMP0` writer"]
pub type W = crate::W<LDCMP0_SPEC>;
#[doc = "Field `CMP_LD0` reader - Compare Value for LED COL\\[x\\]"]
pub type CMP_LD0_R = crate::FieldReader;
#[doc = "Field `CMP_LD0` writer - Compare Value for LED COL\\[x\\]"]
pub type CMP_LD0_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `CMP_LD1` reader - Compare Value for LED COL\\[x\\]"]
pub type CMP_LD1_R = crate::FieldReader;
#[doc = "Field `CMP_LD1` writer - Compare Value for LED COL\\[x\\]"]
pub type CMP_LD1_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `CMP_LD2` reader - Compare Value for LED COL\\[x\\]"]
pub type CMP_LD2_R = crate::FieldReader;
#[doc = "Field `CMP_LD2` writer - Compare Value for LED COL\\[x\\]"]
pub type CMP_LD2_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `CMP_LD3` reader - Compare Value for LED COL\\[x\\]"]
pub type CMP_LD3_R = crate::FieldReader;
#[doc = "Field `CMP_LD3` writer - Compare Value for LED COL\\[x\\]"]
pub type CMP_LD3_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Compare Value for LED COL\\[x\\]"]
    #[inline(always)]
    pub fn cmp_ld0(&self) -> CMP_LD0_R {
        CMP_LD0_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Compare Value for LED COL\\[x\\]"]
    #[inline(always)]
    pub fn cmp_ld1(&self) -> CMP_LD1_R {
        CMP_LD1_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Compare Value for LED COL\\[x\\]"]
    #[inline(always)]
    pub fn cmp_ld2(&self) -> CMP_LD2_R {
        CMP_LD2_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Compare Value for LED COL\\[x\\]"]
    #[inline(always)]
    pub fn cmp_ld3(&self) -> CMP_LD3_R {
        CMP_LD3_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Compare Value for LED COL\\[x\\]"]
    #[inline(always)]
    #[must_use]
    pub fn cmp_ld0(&mut self) -> CMP_LD0_W<LDCMP0_SPEC> {
        CMP_LD0_W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Compare Value for LED COL\\[x\\]"]
    #[inline(always)]
    #[must_use]
    pub fn cmp_ld1(&mut self) -> CMP_LD1_W<LDCMP0_SPEC> {
        CMP_LD1_W::new(self, 8)
    }
    #[doc = "Bits 16:23 - Compare Value for LED COL\\[x\\]"]
    #[inline(always)]
    #[must_use]
    pub fn cmp_ld2(&mut self) -> CMP_LD2_W<LDCMP0_SPEC> {
        CMP_LD2_W::new(self, 16)
    }
    #[doc = "Bits 24:31 - Compare Value for LED COL\\[x\\]"]
    #[inline(always)]
    #[must_use]
    pub fn cmp_ld3(&mut self) -> CMP_LD3_W<LDCMP0_SPEC> {
        CMP_LD3_W::new(self, 24)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "LED Compare Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ldcmp0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ldcmp0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LDCMP0_SPEC;
impl crate::RegisterSpec for LDCMP0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ldcmp0::R`](R) reader structure"]
impl crate::Readable for LDCMP0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ldcmp0::W`](W) writer structure"]
impl crate::Writable for LDCMP0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LDCMP0 to value 0"]
impl crate::Resettable for LDCMP0_SPEC {
    const RESET_VALUE: u32 = 0;
}
