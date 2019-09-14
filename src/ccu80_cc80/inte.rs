#[doc = "Reader of register INTE"]
pub type R = crate::R<u32, super::INTE>;
#[doc = "Writer for register INTE"]
pub type W = crate::W<u32, super::INTE>;
#[doc = "Register INTE `reset()`'s with value 0"]
impl crate::ResetValue for super::INTE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Period match while counting up enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PME_A {
    #[doc = "0: Period Match interrupt is disabled"]
    VALUE1,
    #[doc = "1: Period Match interrupt is enabled"]
    VALUE2,
}
impl From<PME_A> for bool {
    #[inline(always)]
    fn from(variant: PME_A) -> Self {
        match variant {
            PME_A::VALUE1 => false,
            PME_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `PME`"]
pub type PME_R = crate::R<bool, PME_A>;
impl PME_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PME_A {
        match self.bits {
            false => PME_A::VALUE1,
            true => PME_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PME_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PME_A::VALUE2
    }
}
#[doc = "Write proxy for field `PME`"]
pub struct PME_W<'a> {
    w: &'a mut W,
}
impl<'a> PME_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PME_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Period Match interrupt is disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(PME_A::VALUE1)
    }
    #[doc = "Period Match interrupt is enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(PME_A::VALUE2)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "One match while counting down enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OME_A {
    #[doc = "0: One Match interrupt is disabled"]
    VALUE1,
    #[doc = "1: One Match interrupt is enabled"]
    VALUE2,
}
impl From<OME_A> for bool {
    #[inline(always)]
    fn from(variant: OME_A) -> Self {
        match variant {
            OME_A::VALUE1 => false,
            OME_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `OME`"]
pub type OME_R = crate::R<bool, OME_A>;
impl OME_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OME_A {
        match self.bits {
            false => OME_A::VALUE1,
            true => OME_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == OME_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == OME_A::VALUE2
    }
}
#[doc = "Write proxy for field `OME`"]
pub struct OME_W<'a> {
    w: &'a mut W,
}
impl<'a> OME_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OME_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "One Match interrupt is disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(OME_A::VALUE1)
    }
    #[doc = "One Match interrupt is enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(OME_A::VALUE2)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Channel 1 Compare match while counting up enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CMU1E_A {
    #[doc = "0: Compare Match while counting up interrupt is disabled"]
    VALUE1,
    #[doc = "1: Compare Match while counting up interrupt is enabled"]
    VALUE2,
}
impl From<CMU1E_A> for bool {
    #[inline(always)]
    fn from(variant: CMU1E_A) -> Self {
        match variant {
            CMU1E_A::VALUE1 => false,
            CMU1E_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `CMU1E`"]
pub type CMU1E_R = crate::R<bool, CMU1E_A>;
impl CMU1E_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMU1E_A {
        match self.bits {
            false => CMU1E_A::VALUE1,
            true => CMU1E_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CMU1E_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CMU1E_A::VALUE2
    }
}
#[doc = "Write proxy for field `CMU1E`"]
pub struct CMU1E_W<'a> {
    w: &'a mut W,
}
impl<'a> CMU1E_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CMU1E_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Compare Match while counting up interrupt is disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CMU1E_A::VALUE1)
    }
    #[doc = "Compare Match while counting up interrupt is enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CMU1E_A::VALUE2)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Channel 1 Compare match while counting down enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CMD1E_A {
    #[doc = "0: Compare Match while counting down interrupt is disabled"]
    VALUE1,
    #[doc = "1: Compare Match while counting down interrupt is enabled"]
    VALUE2,
}
impl From<CMD1E_A> for bool {
    #[inline(always)]
    fn from(variant: CMD1E_A) -> Self {
        match variant {
            CMD1E_A::VALUE1 => false,
            CMD1E_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `CMD1E`"]
pub type CMD1E_R = crate::R<bool, CMD1E_A>;
impl CMD1E_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMD1E_A {
        match self.bits {
            false => CMD1E_A::VALUE1,
            true => CMD1E_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CMD1E_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CMD1E_A::VALUE2
    }
}
#[doc = "Write proxy for field `CMD1E`"]
pub struct CMD1E_W<'a> {
    w: &'a mut W,
}
impl<'a> CMD1E_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CMD1E_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Compare Match while counting down interrupt is disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CMD1E_A::VALUE1)
    }
    #[doc = "Compare Match while counting down interrupt is enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CMD1E_A::VALUE2)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Channel 2 Compare match while counting up enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CMU2E_A {
    #[doc = "0: Compare Match while counting up interrupt is disabled"]
    VALUE1,
    #[doc = "1: Compare Match while counting up interrupt is enabled"]
    VALUE2,
}
impl From<CMU2E_A> for bool {
    #[inline(always)]
    fn from(variant: CMU2E_A) -> Self {
        match variant {
            CMU2E_A::VALUE1 => false,
            CMU2E_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `CMU2E`"]
pub type CMU2E_R = crate::R<bool, CMU2E_A>;
impl CMU2E_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMU2E_A {
        match self.bits {
            false => CMU2E_A::VALUE1,
            true => CMU2E_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CMU2E_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CMU2E_A::VALUE2
    }
}
#[doc = "Write proxy for field `CMU2E`"]
pub struct CMU2E_W<'a> {
    w: &'a mut W,
}
impl<'a> CMU2E_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CMU2E_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Compare Match while counting up interrupt is disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CMU2E_A::VALUE1)
    }
    #[doc = "Compare Match while counting up interrupt is enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CMU2E_A::VALUE2)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Channel 2 Compare match while counting down enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CMD2E_A {
    #[doc = "0: Compare Match while counting down interrupt is disabled"]
    VALUE1,
    #[doc = "1: Compare Match while counting down interrupt is enabled"]
    VALUE2,
}
impl From<CMD2E_A> for bool {
    #[inline(always)]
    fn from(variant: CMD2E_A) -> Self {
        match variant {
            CMD2E_A::VALUE1 => false,
            CMD2E_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `CMD2E`"]
pub type CMD2E_R = crate::R<bool, CMD2E_A>;
impl CMD2E_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMD2E_A {
        match self.bits {
            false => CMD2E_A::VALUE1,
            true => CMD2E_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CMD2E_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CMD2E_A::VALUE2
    }
}
#[doc = "Write proxy for field `CMD2E`"]
pub struct CMD2E_W<'a> {
    w: &'a mut W,
}
impl<'a> CMD2E_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CMD2E_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Compare Match while counting down interrupt is disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CMD2E_A::VALUE1)
    }
    #[doc = "Compare Match while counting down interrupt is enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CMD2E_A::VALUE2)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Event 0 interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum E0AE_A {
    #[doc = "0: Event 0 detection interrupt is disabled"]
    VALUE1,
    #[doc = "1: Event 0 detection interrupt is enabled"]
    VALUE2,
}
impl From<E0AE_A> for bool {
    #[inline(always)]
    fn from(variant: E0AE_A) -> Self {
        match variant {
            E0AE_A::VALUE1 => false,
            E0AE_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `E0AE`"]
pub type E0AE_R = crate::R<bool, E0AE_A>;
impl E0AE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> E0AE_A {
        match self.bits {
            false => E0AE_A::VALUE1,
            true => E0AE_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == E0AE_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == E0AE_A::VALUE2
    }
}
#[doc = "Write proxy for field `E0AE`"]
pub struct E0AE_W<'a> {
    w: &'a mut W,
}
impl<'a> E0AE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: E0AE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Event 0 detection interrupt is disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(E0AE_A::VALUE1)
    }
    #[doc = "Event 0 detection interrupt is enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(E0AE_A::VALUE2)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Event 1 interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum E1AE_A {
    #[doc = "0: Event 1 detection interrupt is disabled"]
    VALUE1,
    #[doc = "1: Event 1 detection interrupt is enabled"]
    VALUE2,
}
impl From<E1AE_A> for bool {
    #[inline(always)]
    fn from(variant: E1AE_A) -> Self {
        match variant {
            E1AE_A::VALUE1 => false,
            E1AE_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `E1AE`"]
pub type E1AE_R = crate::R<bool, E1AE_A>;
impl E1AE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> E1AE_A {
        match self.bits {
            false => E1AE_A::VALUE1,
            true => E1AE_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == E1AE_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == E1AE_A::VALUE2
    }
}
#[doc = "Write proxy for field `E1AE`"]
pub struct E1AE_W<'a> {
    w: &'a mut W,
}
impl<'a> E1AE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: E1AE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Event 1 detection interrupt is disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(E1AE_A::VALUE1)
    }
    #[doc = "Event 1 detection interrupt is enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(E1AE_A::VALUE2)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Event 2 interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum E2AE_A {
    #[doc = "0: Event 2 detection interrupt is disabled"]
    VALUE1,
    #[doc = "1: Event 2 detection interrupt is enabled"]
    VALUE2,
}
impl From<E2AE_A> for bool {
    #[inline(always)]
    fn from(variant: E2AE_A) -> Self {
        match variant {
            E2AE_A::VALUE1 => false,
            E2AE_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `E2AE`"]
pub type E2AE_R = crate::R<bool, E2AE_A>;
impl E2AE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> E2AE_A {
        match self.bits {
            false => E2AE_A::VALUE1,
            true => E2AE_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == E2AE_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == E2AE_A::VALUE2
    }
}
#[doc = "Write proxy for field `E2AE`"]
pub struct E2AE_W<'a> {
    w: &'a mut W,
}
impl<'a> E2AE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: E2AE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Event 2 detection interrupt is disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(E2AE_A::VALUE1)
    }
    #[doc = "Event 2 detection interrupt is enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(E2AE_A::VALUE2)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Period match while counting up enable"]
    #[inline(always)]
    pub fn pme(&self) -> PME_R {
        PME_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - One match while counting down enable"]
    #[inline(always)]
    pub fn ome(&self) -> OME_R {
        OME_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Channel 1 Compare match while counting up enable"]
    #[inline(always)]
    pub fn cmu1e(&self) -> CMU1E_R {
        CMU1E_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Channel 1 Compare match while counting down enable"]
    #[inline(always)]
    pub fn cmd1e(&self) -> CMD1E_R {
        CMD1E_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Channel 2 Compare match while counting up enable"]
    #[inline(always)]
    pub fn cmu2e(&self) -> CMU2E_R {
        CMU2E_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Channel 2 Compare match while counting down enable"]
    #[inline(always)]
    pub fn cmd2e(&self) -> CMD2E_R {
        CMD2E_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Event 0 interrupt enable"]
    #[inline(always)]
    pub fn e0ae(&self) -> E0AE_R {
        E0AE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Event 1 interrupt enable"]
    #[inline(always)]
    pub fn e1ae(&self) -> E1AE_R {
        E1AE_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Event 2 interrupt enable"]
    #[inline(always)]
    pub fn e2ae(&self) -> E2AE_R {
        E2AE_R::new(((self.bits >> 10) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Period match while counting up enable"]
    #[inline(always)]
    pub fn pme(&mut self) -> PME_W {
        PME_W { w: self }
    }
    #[doc = "Bit 1 - One match while counting down enable"]
    #[inline(always)]
    pub fn ome(&mut self) -> OME_W {
        OME_W { w: self }
    }
    #[doc = "Bit 2 - Channel 1 Compare match while counting up enable"]
    #[inline(always)]
    pub fn cmu1e(&mut self) -> CMU1E_W {
        CMU1E_W { w: self }
    }
    #[doc = "Bit 3 - Channel 1 Compare match while counting down enable"]
    #[inline(always)]
    pub fn cmd1e(&mut self) -> CMD1E_W {
        CMD1E_W { w: self }
    }
    #[doc = "Bit 4 - Channel 2 Compare match while counting up enable"]
    #[inline(always)]
    pub fn cmu2e(&mut self) -> CMU2E_W {
        CMU2E_W { w: self }
    }
    #[doc = "Bit 5 - Channel 2 Compare match while counting down enable"]
    #[inline(always)]
    pub fn cmd2e(&mut self) -> CMD2E_W {
        CMD2E_W { w: self }
    }
    #[doc = "Bit 8 - Event 0 interrupt enable"]
    #[inline(always)]
    pub fn e0ae(&mut self) -> E0AE_W {
        E0AE_W { w: self }
    }
    #[doc = "Bit 9 - Event 1 interrupt enable"]
    #[inline(always)]
    pub fn e1ae(&mut self) -> E1AE_W {
        E1AE_W { w: self }
    }
    #[doc = "Bit 10 - Event 2 interrupt enable"]
    #[inline(always)]
    pub fn e2ae(&mut self) -> E2AE_W {
        E2AE_W { w: self }
    }
}
