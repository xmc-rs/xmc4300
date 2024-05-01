#[doc = "Register `DATA_BUFFER` reader"]
pub type R = crate::R<DataBufferSpec>;
#[doc = "Register `DATA_BUFFER` writer"]
pub type W = crate::W<DataBufferSpec>;
#[doc = "Field `DATA_BUFFER` reader - Data Buffer"]
pub type DataBufferR = crate::FieldReader<u32>;
#[doc = "Field `DATA_BUFFER` writer - Data Buffer"]
pub type DataBufferW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Data Buffer"]
    #[inline(always)]
    pub fn data_buffer(&self) -> DataBufferR {
        DataBufferR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Data Buffer"]
    #[inline(always)]
    #[must_use]
    pub fn data_buffer(&mut self) -> DataBufferW<DataBufferSpec> {
        DataBufferW::new(self, 0)
    }
}
#[doc = "Data Buffer Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`data_buffer::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`data_buffer::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DataBufferSpec;
impl crate::RegisterSpec for DataBufferSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`data_buffer::R`](R) reader structure"]
impl crate::Readable for DataBufferSpec {}
#[doc = "`write(|w| ..)` method takes [`data_buffer::W`](W) writer structure"]
impl crate::Writable for DataBufferSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DATA_BUFFER to value 0"]
impl crate::Resettable for DataBufferSpec {
    const RESET_VALUE: u32 = 0;
}
