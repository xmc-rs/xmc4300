#[doc = "Register `AIRCR` reader"]
pub type R = crate::R<AIRCR_SPEC>;
#[doc = "Register `AIRCR` writer"]
pub type W = crate::W<AIRCR_SPEC>;
#[doc = "Field `VECTRESET` writer - Reserved for Debug use."]
pub type VECTRESET_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VECTCLRACTIVE` writer - Reserved for Debug use."]
pub type VECTCLRACTIVE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "System reset request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SYSRESETREQ_A {
    #[doc = "0: no system reset request"]
    VALUE1 = 0,
    #[doc = "1: asserts a signal to the outer system that requests a reset."]
    VALUE2 = 1,
}
impl From<SYSRESETREQ_A> for bool {
    #[inline(always)]
    fn from(variant: SYSRESETREQ_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SYSRESETREQ` writer - System reset request"]
pub type SYSRESETREQ_W<'a, REG> = crate::BitWriter<'a, REG, SYSRESETREQ_A>;
impl<'a, REG> SYSRESETREQ_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "no system reset request"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(SYSRESETREQ_A::VALUE1)
    }
    #[doc = "asserts a signal to the outer system that requests a reset."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(SYSRESETREQ_A::VALUE2)
    }
}
#[doc = "Field `PRIGROUP` reader - Interrupt priority grouping field"]
pub type PRIGROUP_R = crate::FieldReader;
#[doc = "Field `PRIGROUP` writer - Interrupt priority grouping field"]
pub type PRIGROUP_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Data endianness bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ENDIANNESS_A {
    #[doc = "0: Little-endian"]
    VALUE1 = 0,
    #[doc = "1: Big-endian."]
    VALUE2 = 1,
}
impl From<ENDIANNESS_A> for bool {
    #[inline(always)]
    fn from(variant: ENDIANNESS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENDIANNESS` reader - Data endianness bit"]
pub type ENDIANNESS_R = crate::BitReader<ENDIANNESS_A>;
impl ENDIANNESS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ENDIANNESS_A {
        match self.bits {
            false => ENDIANNESS_A::VALUE1,
            true => ENDIANNESS_A::VALUE2,
        }
    }
    #[doc = "Little-endian"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == ENDIANNESS_A::VALUE1
    }
    #[doc = "Big-endian."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == ENDIANNESS_A::VALUE2
    }
}
#[doc = "Field `VECTKEY` reader - Register key"]
pub type VECTKEY_R = crate::FieldReader<u16>;
#[doc = "Field `VECTKEY` writer - Register key"]
pub type VECTKEY_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 8:10 - Interrupt priority grouping field"]
    #[inline(always)]
    pub fn prigroup(&self) -> PRIGROUP_R {
        PRIGROUP_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 15 - Data endianness bit"]
    #[inline(always)]
    pub fn endianness(&self) -> ENDIANNESS_R {
        ENDIANNESS_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:31 - Register key"]
    #[inline(always)]
    pub fn vectkey(&self) -> VECTKEY_R {
        VECTKEY_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - Reserved for Debug use."]
    #[inline(always)]
    #[must_use]
    pub fn vectreset(&mut self) -> VECTRESET_W<AIRCR_SPEC> {
        VECTRESET_W::new(self, 0)
    }
    #[doc = "Bit 1 - Reserved for Debug use."]
    #[inline(always)]
    #[must_use]
    pub fn vectclractive(&mut self) -> VECTCLRACTIVE_W<AIRCR_SPEC> {
        VECTCLRACTIVE_W::new(self, 1)
    }
    #[doc = "Bit 2 - System reset request"]
    #[inline(always)]
    #[must_use]
    pub fn sysresetreq(&mut self) -> SYSRESETREQ_W<AIRCR_SPEC> {
        SYSRESETREQ_W::new(self, 2)
    }
    #[doc = "Bits 8:10 - Interrupt priority grouping field"]
    #[inline(always)]
    #[must_use]
    pub fn prigroup(&mut self) -> PRIGROUP_W<AIRCR_SPEC> {
        PRIGROUP_W::new(self, 8)
    }
    #[doc = "Bits 16:31 - Register key"]
    #[inline(always)]
    #[must_use]
    pub fn vectkey(&mut self) -> VECTKEY_W<AIRCR_SPEC> {
        VECTKEY_W::new(self, 16)
    }
}
#[doc = "Application Interrupt and Reset Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`aircr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aircr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AIRCR_SPEC;
impl crate::RegisterSpec for AIRCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aircr::R`](R) reader structure"]
impl crate::Readable for AIRCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`aircr::W`](W) writer structure"]
impl crate::Writable for AIRCR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AIRCR to value 0xfa05_0000"]
impl crate::Resettable for AIRCR_SPEC {
    const RESET_VALUE: u32 = 0xfa05_0000;
}
