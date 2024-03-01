#[doc = "Register `MODATAL` reader"]
pub type R = crate::R<ModatalSpec>;
#[doc = "Register `MODATAL` writer"]
pub type W = crate::W<ModatalSpec>;
#[doc = "Field `DB0` reader - Data Byte 0 of Message Object n"]
pub type Db0R = crate::FieldReader;
#[doc = "Field `DB0` writer - Data Byte 0 of Message Object n"]
pub type Db0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `DB1` reader - Data Byte 1 of Message Object n"]
pub type Db1R = crate::FieldReader;
#[doc = "Field `DB1` writer - Data Byte 1 of Message Object n"]
pub type Db1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `DB2` reader - Data Byte 2 of Message Object n"]
pub type Db2R = crate::FieldReader;
#[doc = "Field `DB2` writer - Data Byte 2 of Message Object n"]
pub type Db2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `DB3` reader - Data Byte 3 of Message Object n"]
pub type Db3R = crate::FieldReader;
#[doc = "Field `DB3` writer - Data Byte 3 of Message Object n"]
pub type Db3W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Data Byte 0 of Message Object n"]
    #[inline(always)]
    pub fn db0(&self) -> Db0R {
        Db0R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Data Byte 1 of Message Object n"]
    #[inline(always)]
    pub fn db1(&self) -> Db1R {
        Db1R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Data Byte 2 of Message Object n"]
    #[inline(always)]
    pub fn db2(&self) -> Db2R {
        Db2R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Data Byte 3 of Message Object n"]
    #[inline(always)]
    pub fn db3(&self) -> Db3R {
        Db3R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Data Byte 0 of Message Object n"]
    #[inline(always)]
    #[must_use]
    pub fn db0(&mut self) -> Db0W<ModatalSpec> {
        Db0W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Data Byte 1 of Message Object n"]
    #[inline(always)]
    #[must_use]
    pub fn db1(&mut self) -> Db1W<ModatalSpec> {
        Db1W::new(self, 8)
    }
    #[doc = "Bits 16:23 - Data Byte 2 of Message Object n"]
    #[inline(always)]
    #[must_use]
    pub fn db2(&mut self) -> Db2W<ModatalSpec> {
        Db2W::new(self, 16)
    }
    #[doc = "Bits 24:31 - Data Byte 3 of Message Object n"]
    #[inline(always)]
    #[must_use]
    pub fn db3(&mut self) -> Db3W<ModatalSpec> {
        Db3W::new(self, 24)
    }
}
#[doc = "Message Object Data Register Low\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`modatal::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`modatal::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ModatalSpec;
impl crate::RegisterSpec for ModatalSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`modatal::R`](R) reader structure"]
impl crate::Readable for ModatalSpec {}
#[doc = "`write(|w| ..)` method takes [`modatal::W`](W) writer structure"]
impl crate::Writable for ModatalSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MODATAL to value 0"]
impl crate::Resettable for ModatalSpec {
    const RESET_VALUE: u32 = 0;
}
