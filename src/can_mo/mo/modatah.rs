#[doc = "Register `MODATAH` reader"]
pub type R = crate::R<ModatahSpec>;
#[doc = "Register `MODATAH` writer"]
pub type W = crate::W<ModatahSpec>;
#[doc = "Field `DB4` reader - Data Byte 4 of Message Object n"]
pub type Db4R = crate::FieldReader;
#[doc = "Field `DB4` writer - Data Byte 4 of Message Object n"]
pub type Db4W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `DB5` reader - Data Byte 5 of Message Object n"]
pub type Db5R = crate::FieldReader;
#[doc = "Field `DB5` writer - Data Byte 5 of Message Object n"]
pub type Db5W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `DB6` reader - Data Byte 6 of Message Object n"]
pub type Db6R = crate::FieldReader;
#[doc = "Field `DB6` writer - Data Byte 6 of Message Object n"]
pub type Db6W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `DB7` reader - Data Byte 7 of Message Object n"]
pub type Db7R = crate::FieldReader;
#[doc = "Field `DB7` writer - Data Byte 7 of Message Object n"]
pub type Db7W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Data Byte 4 of Message Object n"]
    #[inline(always)]
    pub fn db4(&self) -> Db4R {
        Db4R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Data Byte 5 of Message Object n"]
    #[inline(always)]
    pub fn db5(&self) -> Db5R {
        Db5R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Data Byte 6 of Message Object n"]
    #[inline(always)]
    pub fn db6(&self) -> Db6R {
        Db6R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Data Byte 7 of Message Object n"]
    #[inline(always)]
    pub fn db7(&self) -> Db7R {
        Db7R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Data Byte 4 of Message Object n"]
    #[inline(always)]
    #[must_use]
    pub fn db4(&mut self) -> Db4W<ModatahSpec> {
        Db4W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Data Byte 5 of Message Object n"]
    #[inline(always)]
    #[must_use]
    pub fn db5(&mut self) -> Db5W<ModatahSpec> {
        Db5W::new(self, 8)
    }
    #[doc = "Bits 16:23 - Data Byte 6 of Message Object n"]
    #[inline(always)]
    #[must_use]
    pub fn db6(&mut self) -> Db6W<ModatahSpec> {
        Db6W::new(self, 16)
    }
    #[doc = "Bits 24:31 - Data Byte 7 of Message Object n"]
    #[inline(always)]
    #[must_use]
    pub fn db7(&mut self) -> Db7W<ModatahSpec> {
        Db7W::new(self, 24)
    }
}
#[doc = "Message Object Data Register High\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`modatah::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`modatah::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ModatahSpec;
impl crate::RegisterSpec for ModatahSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`modatah::R`](R) reader structure"]
impl crate::Readable for ModatahSpec {}
#[doc = "`write(|w| ..)` method takes [`modatah::W`](W) writer structure"]
impl crate::Writable for ModatahSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MODATAH to value 0"]
impl crate::Resettable for ModatahSpec {
    const RESET_VALUE: u32 = 0;
}
