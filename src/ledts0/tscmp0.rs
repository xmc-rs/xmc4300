#[doc = "Register `TSCMP0` reader"]
pub type R = crate::R<Tscmp0Spec>;
#[doc = "Register `TSCMP0` writer"]
pub type W = crate::W<Tscmp0Spec>;
#[doc = "Field `CMP_TS0` reader - Compare Value for Touch-Sense TSIN\\[x\\]"]
pub type CmpTs0R = crate::FieldReader;
#[doc = "Field `CMP_TS0` writer - Compare Value for Touch-Sense TSIN\\[x\\]"]
pub type CmpTs0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `CMP_TS1` reader - Compare Value for Touch-Sense TSIN\\[x\\]"]
pub type CmpTs1R = crate::FieldReader;
#[doc = "Field `CMP_TS1` writer - Compare Value for Touch-Sense TSIN\\[x\\]"]
pub type CmpTs1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `CMP_TS2` reader - Compare Value for Touch-Sense TSIN\\[x\\]"]
pub type CmpTs2R = crate::FieldReader;
#[doc = "Field `CMP_TS2` writer - Compare Value for Touch-Sense TSIN\\[x\\]"]
pub type CmpTs2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `CMP_TS3` reader - Compare Value for Touch-Sense TSIN\\[x\\]"]
pub type CmpTs3R = crate::FieldReader;
#[doc = "Field `CMP_TS3` writer - Compare Value for Touch-Sense TSIN\\[x\\]"]
pub type CmpTs3W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Compare Value for Touch-Sense TSIN\\[x\\]"]
    #[inline(always)]
    pub fn cmp_ts0(&self) -> CmpTs0R {
        CmpTs0R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Compare Value for Touch-Sense TSIN\\[x\\]"]
    #[inline(always)]
    pub fn cmp_ts1(&self) -> CmpTs1R {
        CmpTs1R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Compare Value for Touch-Sense TSIN\\[x\\]"]
    #[inline(always)]
    pub fn cmp_ts2(&self) -> CmpTs2R {
        CmpTs2R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Compare Value for Touch-Sense TSIN\\[x\\]"]
    #[inline(always)]
    pub fn cmp_ts3(&self) -> CmpTs3R {
        CmpTs3R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Compare Value for Touch-Sense TSIN\\[x\\]"]
    #[inline(always)]
    #[must_use]
    pub fn cmp_ts0(&mut self) -> CmpTs0W<Tscmp0Spec> {
        CmpTs0W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Compare Value for Touch-Sense TSIN\\[x\\]"]
    #[inline(always)]
    #[must_use]
    pub fn cmp_ts1(&mut self) -> CmpTs1W<Tscmp0Spec> {
        CmpTs1W::new(self, 8)
    }
    #[doc = "Bits 16:23 - Compare Value for Touch-Sense TSIN\\[x\\]"]
    #[inline(always)]
    #[must_use]
    pub fn cmp_ts2(&mut self) -> CmpTs2W<Tscmp0Spec> {
        CmpTs2W::new(self, 16)
    }
    #[doc = "Bits 24:31 - Compare Value for Touch-Sense TSIN\\[x\\]"]
    #[inline(always)]
    #[must_use]
    pub fn cmp_ts3(&mut self) -> CmpTs3W<Tscmp0Spec> {
        CmpTs3W::new(self, 24)
    }
}
#[doc = "Touch-sense Compare Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tscmp0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tscmp0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Tscmp0Spec;
impl crate::RegisterSpec for Tscmp0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tscmp0::R`](R) reader structure"]
impl crate::Readable for Tscmp0Spec {}
#[doc = "`write(|w| ..)` method takes [`tscmp0::W`](W) writer structure"]
impl crate::Writable for Tscmp0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TSCMP0 to value 0"]
impl crate::Resettable for Tscmp0Spec {
    const RESET_VALUE: u32 = 0;
}
