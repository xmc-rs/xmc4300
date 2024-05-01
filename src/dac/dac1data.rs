#[doc = "Register `DAC1DATA` reader"]
pub type R = crate::R<DAC1DATA_SPEC>;
#[doc = "Register `DAC1DATA` writer"]
pub type W = crate::W<DAC1DATA_SPEC>;
#[doc = "Field `DATA1` reader - DAC1 Data Bits"]
pub type DATA1_R = crate::FieldReader<u16>;
#[doc = "Field `DATA1` writer - DAC1 Data Bits"]
pub type DATA1_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - DAC1 Data Bits"]
    #[inline(always)]
    pub fn data1(&self) -> DATA1_R {
        DATA1_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - DAC1 Data Bits"]
    #[inline(always)]
    #[must_use]
    pub fn data1(&mut self) -> DATA1_W<DAC1DATA_SPEC> {
        DATA1_W::new(self, 0)
    }
}
#[doc = "DAC1 Data Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dac1data::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dac1data::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DAC1DATA_SPEC;
impl crate::RegisterSpec for DAC1DATA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dac1data::R`](R) reader structure"]
impl crate::Readable for DAC1DATA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dac1data::W`](W) writer structure"]
impl crate::Writable for DAC1DATA_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DAC1DATA to value 0"]
impl crate::Resettable for DAC1DATA_SPEC {
    const RESET_VALUE: u32 = 0;
}
