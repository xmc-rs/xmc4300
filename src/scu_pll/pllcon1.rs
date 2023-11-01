#[doc = "Register `PLLCON1` reader"]
pub type R = crate::R<PLLCON1_SPEC>;
#[doc = "Register `PLLCON1` writer"]
pub type W = crate::W<PLLCON1_SPEC>;
#[doc = "Field `K1DIV` reader - K1-Divider Value"]
pub type K1DIV_R = crate::FieldReader;
#[doc = "Field `K1DIV` writer - K1-Divider Value"]
pub type K1DIV_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 7, O>;
#[doc = "Field `NDIV` reader - N-Divider Value"]
pub type NDIV_R = crate::FieldReader;
#[doc = "Field `NDIV` writer - N-Divider Value"]
pub type NDIV_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 7, O>;
#[doc = "Field `K2DIV` reader - K2-Divider Value"]
pub type K2DIV_R = crate::FieldReader;
#[doc = "Field `K2DIV` writer - K2-Divider Value"]
pub type K2DIV_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 7, O>;
#[doc = "Field `PDIV` reader - P-Divider Value"]
pub type PDIV_R = crate::FieldReader;
#[doc = "Field `PDIV` writer - P-Divider Value"]
pub type PDIV_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
impl R {
    #[doc = "Bits 0:6 - K1-Divider Value"]
    #[inline(always)]
    pub fn k1div(&self) -> K1DIV_R {
        K1DIV_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - N-Divider Value"]
    #[inline(always)]
    pub fn ndiv(&self) -> NDIV_R {
        NDIV_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 16:22 - K2-Divider Value"]
    #[inline(always)]
    pub fn k2div(&self) -> K2DIV_R {
        K2DIV_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 24:27 - P-Divider Value"]
    #[inline(always)]
    pub fn pdiv(&self) -> PDIV_R {
        PDIV_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - K1-Divider Value"]
    #[inline(always)]
    #[must_use]
    pub fn k1div(&mut self) -> K1DIV_W<PLLCON1_SPEC, 0> {
        K1DIV_W::new(self)
    }
    #[doc = "Bits 8:14 - N-Divider Value"]
    #[inline(always)]
    #[must_use]
    pub fn ndiv(&mut self) -> NDIV_W<PLLCON1_SPEC, 8> {
        NDIV_W::new(self)
    }
    #[doc = "Bits 16:22 - K2-Divider Value"]
    #[inline(always)]
    #[must_use]
    pub fn k2div(&mut self) -> K2DIV_W<PLLCON1_SPEC, 16> {
        K2DIV_W::new(self)
    }
    #[doc = "Bits 24:27 - P-Divider Value"]
    #[inline(always)]
    #[must_use]
    pub fn pdiv(&mut self) -> PDIV_W<PLLCON1_SPEC, 24> {
        PDIV_W::new(self)
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
#[doc = "PLL Configuration 1 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pllcon1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pllcon1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PLLCON1_SPEC;
impl crate::RegisterSpec for PLLCON1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pllcon1::R`](R) reader structure"]
impl crate::Readable for PLLCON1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pllcon1::W`](W) writer structure"]
impl crate::Writable for PLLCON1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PLLCON1 to value 0"]
impl crate::Resettable for PLLCON1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
