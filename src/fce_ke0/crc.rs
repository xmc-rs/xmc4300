#[doc = "Register `CRC` reader"]
pub type R = crate::R<CrcSpec>;
#[doc = "Register `CRC` writer"]
pub type W = crate::W<CrcSpec>;
#[doc = "Field `CRC` reader - CRC Register"]
pub type CrcR = crate::FieldReader<u32>;
#[doc = "Field `CRC` writer - CRC Register"]
pub type CrcW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - CRC Register"]
    #[inline(always)]
    pub fn crc(&self) -> CrcR {
        CrcR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - CRC Register"]
    #[inline(always)]
    #[must_use]
    pub fn crc(&mut self) -> CrcW<CrcSpec> {
        CrcW::new(self, 0)
    }
}
#[doc = "CRC Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`crc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`crc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrcSpec;
impl crate::RegisterSpec for CrcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`crc::R`](R) reader structure"]
impl crate::Readable for CrcSpec {}
#[doc = "`write(|w| ..)` method takes [`crc::W`](W) writer structure"]
impl crate::Writable for CrcSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CRC to value 0"]
impl crate::Resettable for CrcSpec {
    const RESET_VALUE: u32 = 0;
}
