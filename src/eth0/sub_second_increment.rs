#[doc = "Register `SUB_SECOND_INCREMENT` reader"]
pub type R = crate::R<SubSecondIncrementSpec>;
#[doc = "Register `SUB_SECOND_INCREMENT` writer"]
pub type W = crate::W<SubSecondIncrementSpec>;
#[doc = "Field `SSINC` reader - Sub-second Increment Value"]
pub type SsincR = crate::FieldReader;
#[doc = "Field `SSINC` writer - Sub-second Increment Value"]
pub type SsincW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Sub-second Increment Value"]
    #[inline(always)]
    pub fn ssinc(&self) -> SsincR {
        SsincR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Sub-second Increment Value"]
    #[inline(always)]
    #[must_use]
    pub fn ssinc(&mut self) -> SsincW<SubSecondIncrementSpec> {
        SsincW::new(self, 0)
    }
}
#[doc = "Sub-Second Increment Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sub_second_increment::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sub_second_increment::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SubSecondIncrementSpec;
impl crate::RegisterSpec for SubSecondIncrementSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sub_second_increment::R`](R) reader structure"]
impl crate::Readable for SubSecondIncrementSpec {}
#[doc = "`write(|w| ..)` method takes [`sub_second_increment::W`](W) writer structure"]
impl crate::Writable for SubSecondIncrementSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SUB_SECOND_INCREMENT to value 0"]
impl crate::Resettable for SubSecondIncrementSpec {
    const RESET_VALUE: u32 = 0;
}
