#[doc = "Register `TSCMP1` reader"]
pub type R = crate::R<TSCMP1_SPEC>;
#[doc = "Register `TSCMP1` writer"]
pub type W = crate::W<TSCMP1_SPEC>;
#[doc = "Field `CMP_TS4` reader - Compare Value for Touch-Sense TSIN\\[x\\]"]
pub type CMP_TS4_R = crate::FieldReader;
#[doc = "Field `CMP_TS4` writer - Compare Value for Touch-Sense TSIN\\[x\\]"]
pub type CMP_TS4_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `CMP_TS5` reader - Compare Value for Touch-Sense TSIN\\[x\\]"]
pub type CMP_TS5_R = crate::FieldReader;
#[doc = "Field `CMP_TS5` writer - Compare Value for Touch-Sense TSIN\\[x\\]"]
pub type CMP_TS5_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `CMP_TS6` reader - Compare Value for Touch-Sense TSIN\\[x\\]"]
pub type CMP_TS6_R = crate::FieldReader;
#[doc = "Field `CMP_TS6` writer - Compare Value for Touch-Sense TSIN\\[x\\]"]
pub type CMP_TS6_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `CMP_TS7` reader - Compare Value for Touch-Sense TSIN\\[x\\]"]
pub type CMP_TS7_R = crate::FieldReader;
#[doc = "Field `CMP_TS7` writer - Compare Value for Touch-Sense TSIN\\[x\\]"]
pub type CMP_TS7_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Compare Value for Touch-Sense TSIN\\[x\\]"]
    #[inline(always)]
    pub fn cmp_ts4(&self) -> CMP_TS4_R {
        CMP_TS4_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Compare Value for Touch-Sense TSIN\\[x\\]"]
    #[inline(always)]
    pub fn cmp_ts5(&self) -> CMP_TS5_R {
        CMP_TS5_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Compare Value for Touch-Sense TSIN\\[x\\]"]
    #[inline(always)]
    pub fn cmp_ts6(&self) -> CMP_TS6_R {
        CMP_TS6_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Compare Value for Touch-Sense TSIN\\[x\\]"]
    #[inline(always)]
    pub fn cmp_ts7(&self) -> CMP_TS7_R {
        CMP_TS7_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Compare Value for Touch-Sense TSIN\\[x\\]"]
    #[inline(always)]
    #[must_use]
    pub fn cmp_ts4(&mut self) -> CMP_TS4_W<TSCMP1_SPEC> {
        CMP_TS4_W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Compare Value for Touch-Sense TSIN\\[x\\]"]
    #[inline(always)]
    #[must_use]
    pub fn cmp_ts5(&mut self) -> CMP_TS5_W<TSCMP1_SPEC> {
        CMP_TS5_W::new(self, 8)
    }
    #[doc = "Bits 16:23 - Compare Value for Touch-Sense TSIN\\[x\\]"]
    #[inline(always)]
    #[must_use]
    pub fn cmp_ts6(&mut self) -> CMP_TS6_W<TSCMP1_SPEC> {
        CMP_TS6_W::new(self, 16)
    }
    #[doc = "Bits 24:31 - Compare Value for Touch-Sense TSIN\\[x\\]"]
    #[inline(always)]
    #[must_use]
    pub fn cmp_ts7(&mut self) -> CMP_TS7_W<TSCMP1_SPEC> {
        CMP_TS7_W::new(self, 24)
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
#[doc = "Touch-sense Compare Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tscmp1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tscmp1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TSCMP1_SPEC;
impl crate::RegisterSpec for TSCMP1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tscmp1::R`](R) reader structure"]
impl crate::Readable for TSCMP1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tscmp1::W`](W) writer structure"]
impl crate::Writable for TSCMP1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TSCMP1 to value 0"]
impl crate::Resettable for TSCMP1_SPEC {
    const RESET_VALUE: u32 = 0;
}
