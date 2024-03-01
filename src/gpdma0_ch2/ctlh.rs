#[doc = "Register `CTLH` reader"]
pub type R = crate::R<CtlhSpec>;
#[doc = "Register `CTLH` writer"]
pub type W = crate::W<CtlhSpec>;
#[doc = "Field `BLOCK_TS` reader - Block Transfer Size"]
pub type BlockTsR = crate::FieldReader<u16>;
#[doc = "Field `BLOCK_TS` writer - Block Transfer Size"]
pub type BlockTsW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `DONE` reader - Done bit"]
pub type DoneR = crate::BitReader;
#[doc = "Field `DONE` writer - Done bit"]
pub type DoneW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:11 - Block Transfer Size"]
    #[inline(always)]
    pub fn block_ts(&self) -> BlockTsR {
        BlockTsR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bit 12 - Done bit"]
    #[inline(always)]
    pub fn done(&self) -> DoneR {
        DoneR::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:11 - Block Transfer Size"]
    #[inline(always)]
    #[must_use]
    pub fn block_ts(&mut self) -> BlockTsW<CtlhSpec> {
        BlockTsW::new(self, 0)
    }
    #[doc = "Bit 12 - Done bit"]
    #[inline(always)]
    #[must_use]
    pub fn done(&mut self) -> DoneW<CtlhSpec> {
        DoneW::new(self, 12)
    }
}
#[doc = "Control Register High\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlh::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlh::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlhSpec;
impl crate::RegisterSpec for CtlhSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlh::R`](R) reader structure"]
impl crate::Readable for CtlhSpec {}
#[doc = "`write(|w| ..)` method takes [`ctlh::W`](W) writer structure"]
impl crate::Writable for CtlhSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLH to value 0x02"]
impl crate::Resettable for CtlhSpec {
    const RESET_VALUE: u32 = 0x02;
}
