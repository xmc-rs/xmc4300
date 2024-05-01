#[doc = "Register `EMUXSEL` reader"]
pub type R = crate::R<EmuxselSpec>;
#[doc = "Register `EMUXSEL` writer"]
pub type W = crate::W<EmuxselSpec>;
#[doc = "Field `EMUXGRP0` reader - External Multiplexer Group for Interface x"]
pub type Emuxgrp0R = crate::FieldReader;
#[doc = "Field `EMUXGRP0` writer - External Multiplexer Group for Interface x"]
pub type Emuxgrp0W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `EMUXGRP1` reader - External Multiplexer Group for Interface x"]
pub type Emuxgrp1R = crate::FieldReader;
#[doc = "Field `EMUXGRP1` writer - External Multiplexer Group for Interface x"]
pub type Emuxgrp1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - External Multiplexer Group for Interface x"]
    #[inline(always)]
    pub fn emuxgrp0(&self) -> Emuxgrp0R {
        Emuxgrp0R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - External Multiplexer Group for Interface x"]
    #[inline(always)]
    pub fn emuxgrp1(&self) -> Emuxgrp1R {
        Emuxgrp1R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - External Multiplexer Group for Interface x"]
    #[inline(always)]
    #[must_use]
    pub fn emuxgrp0(&mut self) -> Emuxgrp0W<EmuxselSpec> {
        Emuxgrp0W::new(self, 0)
    }
    #[doc = "Bits 4:7 - External Multiplexer Group for Interface x"]
    #[inline(always)]
    #[must_use]
    pub fn emuxgrp1(&mut self) -> Emuxgrp1W<EmuxselSpec> {
        Emuxgrp1W::new(self, 4)
    }
}
#[doc = "External Multiplexer Select Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`emuxsel::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`emuxsel::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EmuxselSpec;
impl crate::RegisterSpec for EmuxselSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`emuxsel::R`](R) reader structure"]
impl crate::Readable for EmuxselSpec {}
#[doc = "`write(|w| ..)` method takes [`emuxsel::W`](W) writer structure"]
impl crate::Writable for EmuxselSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EMUXSEL to value 0"]
impl crate::Resettable for EmuxselSpec {
    const RESET_VALUE: u32 = 0;
}
