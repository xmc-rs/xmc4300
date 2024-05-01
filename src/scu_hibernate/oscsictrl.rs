#[doc = "Register `OSCSICTRL` reader"]
pub type R = crate::R<OscsictrlSpec>;
#[doc = "Register `OSCSICTRL` writer"]
pub type W = crate::W<OscsictrlSpec>;
#[doc = "Turn OFF the fOSI Clock Source\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pwd {
    #[doc = "0: Enabled"]
    Const0 = 0,
    #[doc = "1: Disabled"]
    Const1 = 1,
}
impl From<Pwd> for bool {
    #[inline(always)]
    fn from(variant: Pwd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWD` reader - Turn OFF the fOSI Clock Source"]
pub type PwdR = crate::BitReader<Pwd>;
impl PwdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pwd {
        match self.bits {
            false => Pwd::Const0,
            true => Pwd::Const1,
        }
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == Pwd::Const0
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == Pwd::Const1
    }
}
#[doc = "Field `PWD` writer - Turn OFF the fOSI Clock Source"]
pub type PwdW<'a, REG> = crate::BitWriter<'a, REG, Pwd>;
impl<'a, REG> PwdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(Pwd::Const0)
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(Pwd::Const1)
    }
}
impl R {
    #[doc = "Bit 0 - Turn OFF the fOSI Clock Source"]
    #[inline(always)]
    pub fn pwd(&self) -> PwdR {
        PwdR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Turn OFF the fOSI Clock Source"]
    #[inline(always)]
    #[must_use]
    pub fn pwd(&mut self) -> PwdW<OscsictrlSpec> {
        PwdW::new(self, 0)
    }
}
#[doc = "fOSI Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`oscsictrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`oscsictrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OscsictrlSpec;
impl crate::RegisterSpec for OscsictrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`oscsictrl::R`](R) reader structure"]
impl crate::Readable for OscsictrlSpec {}
#[doc = "`write(|w| ..)` method takes [`oscsictrl::W`](W) writer structure"]
impl crate::Writable for OscsictrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OSCSICTRL to value 0x01"]
impl crate::Resettable for OscsictrlSpec {
    const RESET_VALUE: u32 = 0x01;
}
