#[doc = "Register `DATA_BUFFER` reader"]
pub type R = crate::R<DATA_BUFFER_SPEC>;
#[doc = "Register `DATA_BUFFER` writer"]
pub type W = crate::W<DATA_BUFFER_SPEC>;
#[doc = "Field `DATA_BUFFER` reader - Data Buffer"]
pub type DATA_BUFFER_R = crate::FieldReader<u32>;
#[doc = "Field `DATA_BUFFER` writer - Data Buffer"]
pub type DATA_BUFFER_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Data Buffer"]
    #[inline(always)]
    pub fn data_buffer(&self) -> DATA_BUFFER_R {
        DATA_BUFFER_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Data Buffer"]
    #[inline(always)]
    pub fn data_buffer(&mut self) -> DATA_BUFFER_W<DATA_BUFFER_SPEC> {
        DATA_BUFFER_W::new(self, 0)
    }
}
#[doc = "Data Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`data_buffer::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data_buffer::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DATA_BUFFER_SPEC;
impl crate::RegisterSpec for DATA_BUFFER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`data_buffer::R`](R) reader structure"]
impl crate::Readable for DATA_BUFFER_SPEC {}
#[doc = "`write(|w| ..)` method takes [`data_buffer::W`](W) writer structure"]
impl crate::Writable for DATA_BUFFER_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DATA_BUFFER to value 0"]
impl crate::Resettable for DATA_BUFFER_SPEC {
    const RESET_VALUE: u32 = 0;
}
