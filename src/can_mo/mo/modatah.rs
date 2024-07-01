#[doc = "Register `MODATAH` reader"]
pub type R = crate::R<MODATAH_SPEC>;
#[doc = "Register `MODATAH` writer"]
pub type W = crate::W<MODATAH_SPEC>;
#[doc = "Field `DB4` reader - Data Byte 4 of Message Object n"]
pub type DB4_R = crate::FieldReader;
#[doc = "Field `DB4` writer - Data Byte 4 of Message Object n"]
pub type DB4_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `DB5` reader - Data Byte 5 of Message Object n"]
pub type DB5_R = crate::FieldReader;
#[doc = "Field `DB5` writer - Data Byte 5 of Message Object n"]
pub type DB5_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `DB6` reader - Data Byte 6 of Message Object n"]
pub type DB6_R = crate::FieldReader;
#[doc = "Field `DB6` writer - Data Byte 6 of Message Object n"]
pub type DB6_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `DB7` reader - Data Byte 7 of Message Object n"]
pub type DB7_R = crate::FieldReader;
#[doc = "Field `DB7` writer - Data Byte 7 of Message Object n"]
pub type DB7_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Data Byte 4 of Message Object n"]
    #[inline(always)]
    pub fn db4(&self) -> DB4_R {
        DB4_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Data Byte 5 of Message Object n"]
    #[inline(always)]
    pub fn db5(&self) -> DB5_R {
        DB5_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Data Byte 6 of Message Object n"]
    #[inline(always)]
    pub fn db6(&self) -> DB6_R {
        DB6_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Data Byte 7 of Message Object n"]
    #[inline(always)]
    pub fn db7(&self) -> DB7_R {
        DB7_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Data Byte 4 of Message Object n"]
    #[inline(always)]
    #[must_use]
    pub fn db4(&mut self) -> DB4_W<MODATAH_SPEC> {
        DB4_W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Data Byte 5 of Message Object n"]
    #[inline(always)]
    #[must_use]
    pub fn db5(&mut self) -> DB5_W<MODATAH_SPEC> {
        DB5_W::new(self, 8)
    }
    #[doc = "Bits 16:23 - Data Byte 6 of Message Object n"]
    #[inline(always)]
    #[must_use]
    pub fn db6(&mut self) -> DB6_W<MODATAH_SPEC> {
        DB6_W::new(self, 16)
    }
    #[doc = "Bits 24:31 - Data Byte 7 of Message Object n"]
    #[inline(always)]
    #[must_use]
    pub fn db7(&mut self) -> DB7_W<MODATAH_SPEC> {
        DB7_W::new(self, 24)
    }
}
#[doc = "Message Object Data Register High\n\nYou can [`read`](crate::Reg::read) this register and get [`modatah::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`modatah::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MODATAH_SPEC;
impl crate::RegisterSpec for MODATAH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`modatah::R`](R) reader structure"]
impl crate::Readable for MODATAH_SPEC {}
#[doc = "`write(|w| ..)` method takes [`modatah::W`](W) writer structure"]
impl crate::Writable for MODATAH_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MODATAH to value 0"]
impl crate::Resettable for MODATAH_SPEC {
    const RESET_VALUE: u32 = 0;
}
