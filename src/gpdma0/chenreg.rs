#[doc = "Register `CHENREG` reader"]
pub type R = crate::R<CHENREG_SPEC>;
#[doc = "Register `CHENREG` writer"]
pub type W = crate::W<CHENREG_SPEC>;
#[doc = "Enables/Disables the channel\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CH_A {
    #[doc = "0: Disable the Channel"]
    VALUE1 = 0,
    #[doc = "1: Enable the Channel"]
    VALUE2 = 1,
}
impl From<CH_A> for u8 {
    #[inline(always)]
    fn from(variant: CH_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CH_A {
    type Ux = u8;
}
impl crate::IsEnum for CH_A {}
#[doc = "Field `CH` reader - Enables/Disables the channel"]
pub type CH_R = crate::FieldReader<CH_A>;
impl CH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<CH_A> {
        match self.bits {
            0 => Some(CH_A::VALUE1),
            1 => Some(CH_A::VALUE2),
            _ => None,
        }
    }
    #[doc = "Disable the Channel"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CH_A::VALUE1
    }
    #[doc = "Enable the Channel"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CH_A::VALUE2
    }
}
#[doc = "Field `CH` writer - Enables/Disables the channel"]
pub type CH_W<'a, REG> = crate::FieldWriter<'a, REG, 8, CH_A>;
impl<'a, REG> CH_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Disable the Channel"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(CH_A::VALUE1)
    }
    #[doc = "Enable the Channel"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(CH_A::VALUE2)
    }
}
#[doc = "Field `WE_CH` writer - Channel enable write enable"]
pub type WE_CH_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Enables/Disables the channel"]
    #[inline(always)]
    pub fn ch(&self) -> CH_R {
        CH_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Enables/Disables the channel"]
    #[inline(always)]
    #[must_use]
    pub fn ch(&mut self) -> CH_W<CHENREG_SPEC> {
        CH_W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Channel enable write enable"]
    #[inline(always)]
    #[must_use]
    pub fn we_ch(&mut self) -> WE_CH_W<CHENREG_SPEC> {
        WE_CH_W::new(self, 8)
    }
}
#[doc = "GPDMA Channel Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`chenreg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chenreg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CHENREG_SPEC;
impl crate::RegisterSpec for CHENREG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`chenreg::R`](R) reader structure"]
impl crate::Readable for CHENREG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`chenreg::W`](W) writer structure"]
impl crate::Writable for CHENREG_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CHENREG to value 0"]
impl crate::Resettable for CHENREG_SPEC {
    const RESET_VALUE: u32 = 0;
}
