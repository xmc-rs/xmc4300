#[doc = "Register `HFNUM` reader"]
pub type R = crate::R<HfnumSpec>;
#[doc = "Register `HFNUM` writer"]
pub type W = crate::W<HfnumSpec>;
#[doc = "Field `FrNum` reader - Frame Number"]
pub type FrNumR = crate::FieldReader<u16>;
#[doc = "Field `FrNum` writer - Frame Number"]
pub type FrNumW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `FrRem` reader - Frame Time Remaining"]
pub type FrRemR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Frame Number"]
    #[inline(always)]
    pub fn fr_num(&self) -> FrNumR {
        FrNumR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Frame Time Remaining"]
    #[inline(always)]
    pub fn fr_rem(&self) -> FrRemR {
        FrRemR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Frame Number"]
    #[inline(always)]
    #[must_use]
    pub fn fr_num(&mut self) -> FrNumW<HfnumSpec> {
        FrNumW::new(self, 0)
    }
}
#[doc = "Host Frame Number/Frame Time Remaining Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hfnum::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hfnum::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HfnumSpec;
impl crate::RegisterSpec for HfnumSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hfnum::R`](R) reader structure"]
impl crate::Readable for HfnumSpec {}
#[doc = "`write(|w| ..)` method takes [`hfnum::W`](W) writer structure"]
impl crate::Writable for HfnumSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HFNUM to value 0x3fff"]
impl crate::Resettable for HfnumSpec {
    const RESET_VALUE: u32 = 0x3fff;
}
