#[doc = "Register `TBUF[%s]` reader"]
pub type R = crate::R<TbufSpec>;
#[doc = "Register `TBUF[%s]` writer"]
pub type W = crate::W<TbufSpec>;
#[doc = "Field `TDATA` reader - Transmit Data"]
pub type TdataR = crate::FieldReader<u16>;
#[doc = "Field `TDATA` writer - Transmit Data"]
pub type TdataW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Transmit Data"]
    #[inline(always)]
    pub fn tdata(&self) -> TdataR {
        TdataR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Transmit Data"]
    #[inline(always)]
    #[must_use]
    pub fn tdata(&mut self) -> TdataW<TbufSpec> {
        TdataW::new(self, 0)
    }
}
#[doc = "Transmit Buffer\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tbuf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tbuf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TbufSpec;
impl crate::RegisterSpec for TbufSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tbuf::R`](R) reader structure"]
impl crate::Readable for TbufSpec {}
#[doc = "`write(|w| ..)` method takes [`tbuf::W`](W) writer structure"]
impl crate::Writable for TbufSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TBUF[%s]
to value 0"]
impl crate::Resettable for TbufSpec {
    const RESET_VALUE: u32 = 0;
}
