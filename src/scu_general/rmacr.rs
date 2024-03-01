#[doc = "Register `RMACR` reader"]
pub type R = crate::R<RmacrSpec>;
#[doc = "Register `RMACR` writer"]
pub type W = crate::W<RmacrSpec>;
#[doc = "Hibernate Retention Memory Register Update Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rdwr {
    #[doc = "0: transfer data from Retention Memory in Hibernate domain to RMDATA register"]
    Const0 = 0,
    #[doc = "1: transfer data from RMDATA into Retention Memory in Hibernate domain"]
    Const1 = 1,
}
impl From<Rdwr> for bool {
    #[inline(always)]
    fn from(variant: Rdwr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RDWR` reader - Hibernate Retention Memory Register Update Control"]
pub type RdwrR = crate::BitReader<Rdwr>;
impl RdwrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rdwr {
        match self.bits {
            false => Rdwr::Const0,
            true => Rdwr::Const1,
        }
    }
    #[doc = "transfer data from Retention Memory in Hibernate domain to RMDATA register"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == Rdwr::Const0
    }
    #[doc = "transfer data from RMDATA into Retention Memory in Hibernate domain"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == Rdwr::Const1
    }
}
#[doc = "Field `RDWR` writer - Hibernate Retention Memory Register Update Control"]
pub type RdwrW<'a, REG> = crate::BitWriter<'a, REG, Rdwr>;
impl<'a, REG> RdwrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "transfer data from Retention Memory in Hibernate domain to RMDATA register"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(Rdwr::Const0)
    }
    #[doc = "transfer data from RMDATA into Retention Memory in Hibernate domain"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(Rdwr::Const1)
    }
}
#[doc = "Field `ADDR` reader - Hibernate Retention Memory Register Address Select"]
pub type AddrR = crate::FieldReader;
#[doc = "Field `ADDR` writer - Hibernate Retention Memory Register Address Select"]
pub type AddrW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bit 0 - Hibernate Retention Memory Register Update Control"]
    #[inline(always)]
    pub fn rdwr(&self) -> RdwrR {
        RdwrR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 16:19 - Hibernate Retention Memory Register Address Select"]
    #[inline(always)]
    pub fn addr(&self) -> AddrR {
        AddrR::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Hibernate Retention Memory Register Update Control"]
    #[inline(always)]
    #[must_use]
    pub fn rdwr(&mut self) -> RdwrW<RmacrSpec> {
        RdwrW::new(self, 0)
    }
    #[doc = "Bits 16:19 - Hibernate Retention Memory Register Address Select"]
    #[inline(always)]
    #[must_use]
    pub fn addr(&mut self) -> AddrW<RmacrSpec> {
        AddrW::new(self, 16)
    }
}
#[doc = "Retention Memory Access Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rmacr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rmacr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RmacrSpec;
impl crate::RegisterSpec for RmacrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rmacr::R`](R) reader structure"]
impl crate::Readable for RmacrSpec {}
#[doc = "`write(|w| ..)` method takes [`rmacr::W`](W) writer structure"]
impl crate::Writable for RmacrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RMACR to value 0"]
impl crate::Resettable for RmacrSpec {
    const RESET_VALUE: u32 = 0;
}
