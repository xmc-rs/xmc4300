#[doc = "Register `PANCTR` reader"]
pub type R = crate::R<PANCTR_SPEC>;
#[doc = "Register `PANCTR` writer"]
pub type W = crate::W<PANCTR_SPEC>;
#[doc = "Field `PANCMD` reader - Panel Command"]
pub type PANCMD_R = crate::FieldReader;
#[doc = "Field `PANCMD` writer - Panel Command"]
pub type PANCMD_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Panel Busy Flag\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BUSY_A {
    #[doc = "0: Panel has finished command and is ready to accept a new command."]
    VALUE1 = 0,
    #[doc = "1: Panel operation is in progress."]
    VALUE2 = 1,
}
impl From<BUSY_A> for bool {
    #[inline(always)]
    fn from(variant: BUSY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BUSY` reader - Panel Busy Flag"]
pub type BUSY_R = crate::BitReader<BUSY_A>;
impl BUSY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BUSY_A {
        match self.bits {
            false => BUSY_A::VALUE1,
            true => BUSY_A::VALUE2,
        }
    }
    #[doc = "Panel has finished command and is ready to accept a new command."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == BUSY_A::VALUE1
    }
    #[doc = "Panel operation is in progress."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == BUSY_A::VALUE2
    }
}
#[doc = "Result Busy Flag\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RBUSY_A {
    #[doc = "0: No update of PANAR1 and PANAR2 is scheduled by the list controller."]
    VALUE1 = 0,
    #[doc = "1: A list command is running (BUSY = 1) that will write results to PANAR1 and PANAR2, but the results are not yet available."]
    VALUE2 = 1,
}
impl From<RBUSY_A> for bool {
    #[inline(always)]
    fn from(variant: RBUSY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RBUSY` reader - Result Busy Flag"]
pub type RBUSY_R = crate::BitReader<RBUSY_A>;
impl RBUSY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RBUSY_A {
        match self.bits {
            false => RBUSY_A::VALUE1,
            true => RBUSY_A::VALUE2,
        }
    }
    #[doc = "No update of PANAR1 and PANAR2 is scheduled by the list controller."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == RBUSY_A::VALUE1
    }
    #[doc = "A list command is running (BUSY = 1) that will write results to PANAR1 and PANAR2, but the results are not yet available."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == RBUSY_A::VALUE2
    }
}
#[doc = "Field `PANAR1` reader - Panel Argument 1"]
pub type PANAR1_R = crate::FieldReader;
#[doc = "Field `PANAR1` writer - Panel Argument 1"]
pub type PANAR1_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PANAR2` reader - Panel Argument 2"]
pub type PANAR2_R = crate::FieldReader;
#[doc = "Field `PANAR2` writer - Panel Argument 2"]
pub type PANAR2_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Panel Command"]
    #[inline(always)]
    pub fn pancmd(&self) -> PANCMD_R {
        PANCMD_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - Panel Busy Flag"]
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Result Busy Flag"]
    #[inline(always)]
    pub fn rbusy(&self) -> RBUSY_R {
        RBUSY_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 16:23 - Panel Argument 1"]
    #[inline(always)]
    pub fn panar1(&self) -> PANAR1_R {
        PANAR1_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Panel Argument 2"]
    #[inline(always)]
    pub fn panar2(&self) -> PANAR2_R {
        PANAR2_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Panel Command"]
    #[inline(always)]
    pub fn pancmd(&mut self) -> PANCMD_W<PANCTR_SPEC> {
        PANCMD_W::new(self, 0)
    }
    #[doc = "Bits 16:23 - Panel Argument 1"]
    #[inline(always)]
    pub fn panar1(&mut self) -> PANAR1_W<PANCTR_SPEC> {
        PANAR1_W::new(self, 16)
    }
    #[doc = "Bits 24:31 - Panel Argument 2"]
    #[inline(always)]
    pub fn panar2(&mut self) -> PANAR2_W<PANCTR_SPEC> {
        PANAR2_W::new(self, 24)
    }
}
#[doc = "Panel Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`panctr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`panctr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PANCTR_SPEC;
impl crate::RegisterSpec for PANCTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`panctr::R`](R) reader structure"]
impl crate::Readable for PANCTR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`panctr::W`](W) writer structure"]
impl crate::Writable for PANCTR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PANCTR to value 0x0301"]
impl crate::Resettable for PANCTR_SPEC {
    const RESET_VALUE: u32 = 0x0301;
}
