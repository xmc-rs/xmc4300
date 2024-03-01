#[doc = "Register `CHENREG` reader"]
pub type R = crate::R<ChenregSpec>;
#[doc = "Register `CHENREG` writer"]
pub type W = crate::W<ChenregSpec>;
#[doc = "Enables/Disables the channel\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ch {
    #[doc = "0: Disable the Channel"]
    Value1 = 0,
    #[doc = "1: Enable the Channel"]
    Value2 = 1,
}
impl From<Ch> for u8 {
    #[inline(always)]
    fn from(variant: Ch) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ch {
    type Ux = u8;
}
#[doc = "Field `CH` reader - Enables/Disables the channel"]
pub type ChR = crate::FieldReader<Ch>;
impl ChR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Ch> {
        match self.bits {
            0 => Some(Ch::Value1),
            1 => Some(Ch::Value2),
            _ => None,
        }
    }
    #[doc = "Disable the Channel"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Ch::Value1
    }
    #[doc = "Enable the Channel"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Ch::Value2
    }
}
#[doc = "Field `CH` writer - Enables/Disables the channel"]
pub type ChW<'a, REG> = crate::FieldWriter<'a, REG, 8, Ch>;
impl<'a, REG> ChW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Disable the Channel"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Ch::Value1)
    }
    #[doc = "Enable the Channel"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Ch::Value2)
    }
}
#[doc = "Field `WE_CH` writer - Channel enable write enable"]
pub type WeChW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Enables/Disables the channel"]
    #[inline(always)]
    pub fn ch(&self) -> ChR {
        ChR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Enables/Disables the channel"]
    #[inline(always)]
    #[must_use]
    pub fn ch(&mut self) -> ChW<ChenregSpec> {
        ChW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Channel enable write enable"]
    #[inline(always)]
    #[must_use]
    pub fn we_ch(&mut self) -> WeChW<ChenregSpec> {
        WeChW::new(self, 8)
    }
}
#[doc = "GPDMA Channel Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`chenreg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`chenreg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ChenregSpec;
impl crate::RegisterSpec for ChenregSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`chenreg::R`](R) reader structure"]
impl crate::Readable for ChenregSpec {}
#[doc = "`write(|w| ..)` method takes [`chenreg::W`](W) writer structure"]
impl crate::Writable for ChenregSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CHENREG to value 0"]
impl crate::Resettable for ChenregSpec {
    const RESET_VALUE: u32 = 0;
}
