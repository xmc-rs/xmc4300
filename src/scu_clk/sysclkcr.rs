#[doc = "Register `SYSCLKCR` reader"]
pub type R = crate::R<SysclkcrSpec>;
#[doc = "Register `SYSCLKCR` writer"]
pub type W = crate::W<SysclkcrSpec>;
#[doc = "Field `SYSDIV` reader - System Clock Division Value"]
pub type SysdivR = crate::FieldReader;
#[doc = "Field `SYSDIV` writer - System Clock Division Value"]
pub type SysdivW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "System Clock Selection Value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Syssel {
    #[doc = "0: fOFI clock"]
    Const0 = 0,
    #[doc = "1: fPLL clock"]
    Const1 = 1,
}
impl From<Syssel> for bool {
    #[inline(always)]
    fn from(variant: Syssel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SYSSEL` reader - System Clock Selection Value"]
pub type SysselR = crate::BitReader<Syssel>;
impl SysselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Syssel {
        match self.bits {
            false => Syssel::Const0,
            true => Syssel::Const1,
        }
    }
    #[doc = "fOFI clock"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == Syssel::Const0
    }
    #[doc = "fPLL clock"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == Syssel::Const1
    }
}
#[doc = "Field `SYSSEL` writer - System Clock Selection Value"]
pub type SysselW<'a, REG> = crate::BitWriter<'a, REG, Syssel>;
impl<'a, REG> SysselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "fOFI clock"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(Syssel::Const0)
    }
    #[doc = "fPLL clock"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(Syssel::Const1)
    }
}
impl R {
    #[doc = "Bits 0:7 - System Clock Division Value"]
    #[inline(always)]
    pub fn sysdiv(&self) -> SysdivR {
        SysdivR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 16 - System Clock Selection Value"]
    #[inline(always)]
    pub fn syssel(&self) -> SysselR {
        SysselR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - System Clock Division Value"]
    #[inline(always)]
    #[must_use]
    pub fn sysdiv(&mut self) -> SysdivW<SysclkcrSpec> {
        SysdivW::new(self, 0)
    }
    #[doc = "Bit 16 - System Clock Selection Value"]
    #[inline(always)]
    #[must_use]
    pub fn syssel(&mut self) -> SysselW<SysclkcrSpec> {
        SysselW::new(self, 16)
    }
}
#[doc = "System Clock Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sysclkcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sysclkcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SysclkcrSpec;
impl crate::RegisterSpec for SysclkcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sysclkcr::R`](R) reader structure"]
impl crate::Readable for SysclkcrSpec {}
#[doc = "`write(|w| ..)` method takes [`sysclkcr::W`](W) writer structure"]
impl crate::Writable for SysclkcrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SYSCLKCR to value 0"]
impl crate::Resettable for SysclkcrSpec {
    const RESET_VALUE: u32 = 0;
}
