#[doc = "Register `RMACR` reader"]
pub type R = crate::R<RMACR_SPEC>;
#[doc = "Register `RMACR` writer"]
pub type W = crate::W<RMACR_SPEC>;
#[doc = "Hibernate Retention Memory Register Update Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RDWR_A {
    #[doc = "0: transfer data from Retention Memory in Hibernate domain to RMDATA register"]
    CONST_0 = 0,
    #[doc = "1: transfer data from RMDATA into Retention Memory in Hibernate domain"]
    CONST_1 = 1,
}
impl From<RDWR_A> for bool {
    #[inline(always)]
    fn from(variant: RDWR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RDWR` reader - Hibernate Retention Memory Register Update Control"]
pub type RDWR_R = crate::BitReader<RDWR_A>;
impl RDWR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RDWR_A {
        match self.bits {
            false => RDWR_A::CONST_0,
            true => RDWR_A::CONST_1,
        }
    }
    #[doc = "transfer data from Retention Memory in Hibernate domain to RMDATA register"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == RDWR_A::CONST_0
    }
    #[doc = "transfer data from RMDATA into Retention Memory in Hibernate domain"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == RDWR_A::CONST_1
    }
}
#[doc = "Field `RDWR` writer - Hibernate Retention Memory Register Update Control"]
pub type RDWR_W<'a, REG> = crate::BitWriter<'a, REG, RDWR_A>;
impl<'a, REG> RDWR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "transfer data from Retention Memory in Hibernate domain to RMDATA register"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(RDWR_A::CONST_0)
    }
    #[doc = "transfer data from RMDATA into Retention Memory in Hibernate domain"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(RDWR_A::CONST_1)
    }
}
#[doc = "Field `ADDR` reader - Hibernate Retention Memory Register Address Select"]
pub type ADDR_R = crate::FieldReader;
#[doc = "Field `ADDR` writer - Hibernate Retention Memory Register Address Select"]
pub type ADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bit 0 - Hibernate Retention Memory Register Update Control"]
    #[inline(always)]
    pub fn rdwr(&self) -> RDWR_R {
        RDWR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 16:19 - Hibernate Retention Memory Register Address Select"]
    #[inline(always)]
    pub fn addr(&self) -> ADDR_R {
        ADDR_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Hibernate Retention Memory Register Update Control"]
    #[inline(always)]
    #[must_use]
    pub fn rdwr(&mut self) -> RDWR_W<RMACR_SPEC> {
        RDWR_W::new(self, 0)
    }
    #[doc = "Bits 16:19 - Hibernate Retention Memory Register Address Select"]
    #[inline(always)]
    #[must_use]
    pub fn addr(&mut self) -> ADDR_W<RMACR_SPEC> {
        ADDR_W::new(self, 16)
    }
}
#[doc = "Retention Memory Access Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rmacr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rmacr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RMACR_SPEC;
impl crate::RegisterSpec for RMACR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rmacr::R`](R) reader structure"]
impl crate::Readable for RMACR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rmacr::W`](W) writer structure"]
impl crate::Writable for RMACR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RMACR to value 0"]
impl crate::Resettable for RMACR_SPEC {
    const RESET_VALUE: u32 = 0;
}
