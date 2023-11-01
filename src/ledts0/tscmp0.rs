#[doc = "Register `TSCMP0` reader"]
pub type R = crate::R<TSCMP0_SPEC>;
#[doc = "Register `TSCMP0` writer"]
pub type W = crate::W<TSCMP0_SPEC>;
#[doc = "Field `CMP_TS0` reader - Compare Value for Touch-Sense TSIN\\[x\\]"]
pub type CMP_TS0_R = crate::FieldReader;
#[doc = "Field `CMP_TS0` writer - Compare Value for Touch-Sense TSIN\\[x\\]"]
pub type CMP_TS0_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `CMP_TS1` reader - Compare Value for Touch-Sense TSIN\\[x\\]"]
pub type CMP_TS1_R = crate::FieldReader;
#[doc = "Field `CMP_TS1` writer - Compare Value for Touch-Sense TSIN\\[x\\]"]
pub type CMP_TS1_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `CMP_TS2` reader - Compare Value for Touch-Sense TSIN\\[x\\]"]
pub type CMP_TS2_R = crate::FieldReader;
#[doc = "Field `CMP_TS2` writer - Compare Value for Touch-Sense TSIN\\[x\\]"]
pub type CMP_TS2_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `CMP_TS3` reader - Compare Value for Touch-Sense TSIN\\[x\\]"]
pub type CMP_TS3_R = crate::FieldReader;
#[doc = "Field `CMP_TS3` writer - Compare Value for Touch-Sense TSIN\\[x\\]"]
pub type CMP_TS3_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Compare Value for Touch-Sense TSIN\\[x\\]"]
    #[inline(always)]
    pub fn cmp_ts0(&self) -> CMP_TS0_R {
        CMP_TS0_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Compare Value for Touch-Sense TSIN\\[x\\]"]
    #[inline(always)]
    pub fn cmp_ts1(&self) -> CMP_TS1_R {
        CMP_TS1_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Compare Value for Touch-Sense TSIN\\[x\\]"]
    #[inline(always)]
    pub fn cmp_ts2(&self) -> CMP_TS2_R {
        CMP_TS2_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Compare Value for Touch-Sense TSIN\\[x\\]"]
    #[inline(always)]
    pub fn cmp_ts3(&self) -> CMP_TS3_R {
        CMP_TS3_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Compare Value for Touch-Sense TSIN\\[x\\]"]
    #[inline(always)]
    #[must_use]
    pub fn cmp_ts0(&mut self) -> CMP_TS0_W<TSCMP0_SPEC, 0> {
        CMP_TS0_W::new(self)
    }
    #[doc = "Bits 8:15 - Compare Value for Touch-Sense TSIN\\[x\\]"]
    #[inline(always)]
    #[must_use]
    pub fn cmp_ts1(&mut self) -> CMP_TS1_W<TSCMP0_SPEC, 8> {
        CMP_TS1_W::new(self)
    }
    #[doc = "Bits 16:23 - Compare Value for Touch-Sense TSIN\\[x\\]"]
    #[inline(always)]
    #[must_use]
    pub fn cmp_ts2(&mut self) -> CMP_TS2_W<TSCMP0_SPEC, 16> {
        CMP_TS2_W::new(self)
    }
    #[doc = "Bits 24:31 - Compare Value for Touch-Sense TSIN\\[x\\]"]
    #[inline(always)]
    #[must_use]
    pub fn cmp_ts3(&mut self) -> CMP_TS3_W<TSCMP0_SPEC, 24> {
        CMP_TS3_W::new(self)
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
#[doc = "Touch-sense Compare Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tscmp0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tscmp0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TSCMP0_SPEC;
impl crate::RegisterSpec for TSCMP0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tscmp0::R`](R) reader structure"]
impl crate::Readable for TSCMP0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tscmp0::W`](W) writer structure"]
impl crate::Writable for TSCMP0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TSCMP0 to value 0"]
impl crate::Resettable for TSCMP0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
