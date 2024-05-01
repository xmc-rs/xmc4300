#[doc = "Register `TSCMP1` reader"]
pub type R = crate::R<Tscmp1Spec>;
#[doc = "Register `TSCMP1` writer"]
pub type W = crate::W<Tscmp1Spec>;
#[doc = "Field `CMP_TS4` reader - Compare Value for Touch-Sense TSIN\\[x\\]"]
pub type CmpTs4R = crate::FieldReader;
#[doc = "Field `CMP_TS4` writer - Compare Value for Touch-Sense TSIN\\[x\\]"]
pub type CmpTs4W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `CMP_TS5` reader - Compare Value for Touch-Sense TSIN\\[x\\]"]
pub type CmpTs5R = crate::FieldReader;
#[doc = "Field `CMP_TS5` writer - Compare Value for Touch-Sense TSIN\\[x\\]"]
pub type CmpTs5W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `CMP_TS6` reader - Compare Value for Touch-Sense TSIN\\[x\\]"]
pub type CmpTs6R = crate::FieldReader;
#[doc = "Field `CMP_TS6` writer - Compare Value for Touch-Sense TSIN\\[x\\]"]
pub type CmpTs6W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `CMP_TS7` reader - Compare Value for Touch-Sense TSIN\\[x\\]"]
pub type CmpTs7R = crate::FieldReader;
#[doc = "Field `CMP_TS7` writer - Compare Value for Touch-Sense TSIN\\[x\\]"]
pub type CmpTs7W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Compare Value for Touch-Sense TSIN\\[x\\]"]
    #[inline(always)]
    pub fn cmp_ts4(&self) -> CmpTs4R {
        CmpTs4R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Compare Value for Touch-Sense TSIN\\[x\\]"]
    #[inline(always)]
    pub fn cmp_ts5(&self) -> CmpTs5R {
        CmpTs5R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Compare Value for Touch-Sense TSIN\\[x\\]"]
    #[inline(always)]
    pub fn cmp_ts6(&self) -> CmpTs6R {
        CmpTs6R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Compare Value for Touch-Sense TSIN\\[x\\]"]
    #[inline(always)]
    pub fn cmp_ts7(&self) -> CmpTs7R {
        CmpTs7R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Compare Value for Touch-Sense TSIN\\[x\\]"]
    #[inline(always)]
    #[must_use]
    pub fn cmp_ts4(&mut self) -> CmpTs4W<Tscmp1Spec> {
        CmpTs4W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Compare Value for Touch-Sense TSIN\\[x\\]"]
    #[inline(always)]
    #[must_use]
    pub fn cmp_ts5(&mut self) -> CmpTs5W<Tscmp1Spec> {
        CmpTs5W::new(self, 8)
    }
    #[doc = "Bits 16:23 - Compare Value for Touch-Sense TSIN\\[x\\]"]
    #[inline(always)]
    #[must_use]
    pub fn cmp_ts6(&mut self) -> CmpTs6W<Tscmp1Spec> {
        CmpTs6W::new(self, 16)
    }
    #[doc = "Bits 24:31 - Compare Value for Touch-Sense TSIN\\[x\\]"]
    #[inline(always)]
    #[must_use]
    pub fn cmp_ts7(&mut self) -> CmpTs7W<Tscmp1Spec> {
        CmpTs7W::new(self, 24)
    }
}
#[doc = "Touch-sense Compare Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tscmp1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tscmp1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Tscmp1Spec;
impl crate::RegisterSpec for Tscmp1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tscmp1::R`](R) reader structure"]
impl crate::Readable for Tscmp1Spec {}
#[doc = "`write(|w| ..)` method takes [`tscmp1::W`](W) writer structure"]
impl crate::Writable for Tscmp1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TSCMP1 to value 0"]
impl crate::Resettable for Tscmp1Spec {
    const RESET_VALUE: u32 = 0;
}
