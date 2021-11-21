#[doc = "Register `OMR` writer"]
pub struct W(crate::W<OMR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<OMR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OMR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PS0` writer - Port n Set Bit 0"]
pub struct PS0_W<'a> {
    w: &'a mut W,
}
impl<'a> PS0_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "Field `PS1` writer - Port n Set Bit 1"]
pub struct PS1_W<'a> {
    w: &'a mut W,
}
impl<'a> PS1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `PS2` writer - Port n Set Bit 2"]
pub struct PS2_W<'a> {
    w: &'a mut W,
}
impl<'a> PS2_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `PS3` writer - Port n Set Bit 3"]
pub struct PS3_W<'a> {
    w: &'a mut W,
}
impl<'a> PS3_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "Field `PS4` writer - Port n Set Bit 4"]
pub struct PS4_W<'a> {
    w: &'a mut W,
}
impl<'a> PS4_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "Field `PS5` writer - Port n Set Bit 5"]
pub struct PS5_W<'a> {
    w: &'a mut W,
}
impl<'a> PS5_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "Field `PS6` writer - Port n Set Bit 6"]
pub struct PS6_W<'a> {
    w: &'a mut W,
}
impl<'a> PS6_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "Field `PS7` writer - Port n Set Bit 7"]
pub struct PS7_W<'a> {
    w: &'a mut W,
}
impl<'a> PS7_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
#[doc = "Field `PS8` writer - Port n Set Bit 8"]
pub struct PS8_W<'a> {
    w: &'a mut W,
}
impl<'a> PS8_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
#[doc = "Field `PS9` writer - Port n Set Bit 9"]
pub struct PS9_W<'a> {
    w: &'a mut W,
}
impl<'a> PS9_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
#[doc = "Field `PS10` writer - Port n Set Bit 10"]
pub struct PS10_W<'a> {
    w: &'a mut W,
}
impl<'a> PS10_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
        self.w
    }
}
#[doc = "Field `PS11` writer - Port n Set Bit 11"]
pub struct PS11_W<'a> {
    w: &'a mut W,
}
impl<'a> PS11_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u32 & 0x01) << 11);
        self.w
    }
}
#[doc = "Field `PS12` writer - Port n Set Bit 12"]
pub struct PS12_W<'a> {
    w: &'a mut W,
}
impl<'a> PS12_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u32 & 0x01) << 12);
        self.w
    }
}
#[doc = "Field `PS13` writer - Port n Set Bit 13"]
pub struct PS13_W<'a> {
    w: &'a mut W,
}
impl<'a> PS13_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | ((value as u32 & 0x01) << 13);
        self.w
    }
}
#[doc = "Field `PS14` writer - Port n Set Bit 14"]
pub struct PS14_W<'a> {
    w: &'a mut W,
}
impl<'a> PS14_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | ((value as u32 & 0x01) << 14);
        self.w
    }
}
#[doc = "Field `PS15` writer - Port n Set Bit 15"]
pub struct PS15_W<'a> {
    w: &'a mut W,
}
impl<'a> PS15_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | ((value as u32 & 0x01) << 15);
        self.w
    }
}
#[doc = "Field `PR0` writer - Port n Reset Bit 0"]
pub struct PR0_W<'a> {
    w: &'a mut W,
}
impl<'a> PR0_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
#[doc = "Field `PR1` writer - Port n Reset Bit 1"]
pub struct PR1_W<'a> {
    w: &'a mut W,
}
impl<'a> PR1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | ((value as u32 & 0x01) << 17);
        self.w
    }
}
#[doc = "Field `PR2` writer - Port n Reset Bit 2"]
pub struct PR2_W<'a> {
    w: &'a mut W,
}
impl<'a> PR2_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | ((value as u32 & 0x01) << 18);
        self.w
    }
}
#[doc = "Field `PR3` writer - Port n Reset Bit 3"]
pub struct PR3_W<'a> {
    w: &'a mut W,
}
impl<'a> PR3_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | ((value as u32 & 0x01) << 19);
        self.w
    }
}
#[doc = "Field `PR4` writer - Port n Reset Bit 4"]
pub struct PR4_W<'a> {
    w: &'a mut W,
}
impl<'a> PR4_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | ((value as u32 & 0x01) << 20);
        self.w
    }
}
#[doc = "Field `PR5` writer - Port n Reset Bit 5"]
pub struct PR5_W<'a> {
    w: &'a mut W,
}
impl<'a> PR5_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | ((value as u32 & 0x01) << 21);
        self.w
    }
}
#[doc = "Field `PR6` writer - Port n Reset Bit 6"]
pub struct PR6_W<'a> {
    w: &'a mut W,
}
impl<'a> PR6_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | ((value as u32 & 0x01) << 22);
        self.w
    }
}
#[doc = "Field `PR7` writer - Port n Reset Bit 7"]
pub struct PR7_W<'a> {
    w: &'a mut W,
}
impl<'a> PR7_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | ((value as u32 & 0x01) << 23);
        self.w
    }
}
#[doc = "Field `PR8` writer - Port n Reset Bit 8"]
pub struct PR8_W<'a> {
    w: &'a mut W,
}
impl<'a> PR8_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | ((value as u32 & 0x01) << 24);
        self.w
    }
}
#[doc = "Field `PR9` writer - Port n Reset Bit 9"]
pub struct PR9_W<'a> {
    w: &'a mut W,
}
impl<'a> PR9_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | ((value as u32 & 0x01) << 25);
        self.w
    }
}
#[doc = "Field `PR10` writer - Port n Reset Bit 10"]
pub struct PR10_W<'a> {
    w: &'a mut W,
}
impl<'a> PR10_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | ((value as u32 & 0x01) << 26);
        self.w
    }
}
#[doc = "Field `PR11` writer - Port n Reset Bit 11"]
pub struct PR11_W<'a> {
    w: &'a mut W,
}
impl<'a> PR11_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | ((value as u32 & 0x01) << 27);
        self.w
    }
}
#[doc = "Field `PR12` writer - Port n Reset Bit 12"]
pub struct PR12_W<'a> {
    w: &'a mut W,
}
impl<'a> PR12_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | ((value as u32 & 0x01) << 28);
        self.w
    }
}
#[doc = "Field `PR13` writer - Port n Reset Bit 13"]
pub struct PR13_W<'a> {
    w: &'a mut W,
}
impl<'a> PR13_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | ((value as u32 & 0x01) << 29);
        self.w
    }
}
#[doc = "Field `PR14` writer - Port n Reset Bit 14"]
pub struct PR14_W<'a> {
    w: &'a mut W,
}
impl<'a> PR14_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | ((value as u32 & 0x01) << 30);
        self.w
    }
}
#[doc = "Field `PR15` writer - Port n Reset Bit 15"]
pub struct PR15_W<'a> {
    w: &'a mut W,
}
impl<'a> PR15_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | ((value as u32 & 0x01) << 31);
        self.w
    }
}
impl W {
    #[doc = "Bit 0 - Port n Set Bit 0"]
    #[inline(always)]
    pub fn ps0(&mut self) -> PS0_W {
        PS0_W { w: self }
    }
    #[doc = "Bit 1 - Port n Set Bit 1"]
    #[inline(always)]
    pub fn ps1(&mut self) -> PS1_W {
        PS1_W { w: self }
    }
    #[doc = "Bit 2 - Port n Set Bit 2"]
    #[inline(always)]
    pub fn ps2(&mut self) -> PS2_W {
        PS2_W { w: self }
    }
    #[doc = "Bit 3 - Port n Set Bit 3"]
    #[inline(always)]
    pub fn ps3(&mut self) -> PS3_W {
        PS3_W { w: self }
    }
    #[doc = "Bit 4 - Port n Set Bit 4"]
    #[inline(always)]
    pub fn ps4(&mut self) -> PS4_W {
        PS4_W { w: self }
    }
    #[doc = "Bit 5 - Port n Set Bit 5"]
    #[inline(always)]
    pub fn ps5(&mut self) -> PS5_W {
        PS5_W { w: self }
    }
    #[doc = "Bit 6 - Port n Set Bit 6"]
    #[inline(always)]
    pub fn ps6(&mut self) -> PS6_W {
        PS6_W { w: self }
    }
    #[doc = "Bit 7 - Port n Set Bit 7"]
    #[inline(always)]
    pub fn ps7(&mut self) -> PS7_W {
        PS7_W { w: self }
    }
    #[doc = "Bit 8 - Port n Set Bit 8"]
    #[inline(always)]
    pub fn ps8(&mut self) -> PS8_W {
        PS8_W { w: self }
    }
    #[doc = "Bit 9 - Port n Set Bit 9"]
    #[inline(always)]
    pub fn ps9(&mut self) -> PS9_W {
        PS9_W { w: self }
    }
    #[doc = "Bit 10 - Port n Set Bit 10"]
    #[inline(always)]
    pub fn ps10(&mut self) -> PS10_W {
        PS10_W { w: self }
    }
    #[doc = "Bit 11 - Port n Set Bit 11"]
    #[inline(always)]
    pub fn ps11(&mut self) -> PS11_W {
        PS11_W { w: self }
    }
    #[doc = "Bit 12 - Port n Set Bit 12"]
    #[inline(always)]
    pub fn ps12(&mut self) -> PS12_W {
        PS12_W { w: self }
    }
    #[doc = "Bit 13 - Port n Set Bit 13"]
    #[inline(always)]
    pub fn ps13(&mut self) -> PS13_W {
        PS13_W { w: self }
    }
    #[doc = "Bit 14 - Port n Set Bit 14"]
    #[inline(always)]
    pub fn ps14(&mut self) -> PS14_W {
        PS14_W { w: self }
    }
    #[doc = "Bit 15 - Port n Set Bit 15"]
    #[inline(always)]
    pub fn ps15(&mut self) -> PS15_W {
        PS15_W { w: self }
    }
    #[doc = "Bit 16 - Port n Reset Bit 0"]
    #[inline(always)]
    pub fn pr0(&mut self) -> PR0_W {
        PR0_W { w: self }
    }
    #[doc = "Bit 17 - Port n Reset Bit 1"]
    #[inline(always)]
    pub fn pr1(&mut self) -> PR1_W {
        PR1_W { w: self }
    }
    #[doc = "Bit 18 - Port n Reset Bit 2"]
    #[inline(always)]
    pub fn pr2(&mut self) -> PR2_W {
        PR2_W { w: self }
    }
    #[doc = "Bit 19 - Port n Reset Bit 3"]
    #[inline(always)]
    pub fn pr3(&mut self) -> PR3_W {
        PR3_W { w: self }
    }
    #[doc = "Bit 20 - Port n Reset Bit 4"]
    #[inline(always)]
    pub fn pr4(&mut self) -> PR4_W {
        PR4_W { w: self }
    }
    #[doc = "Bit 21 - Port n Reset Bit 5"]
    #[inline(always)]
    pub fn pr5(&mut self) -> PR5_W {
        PR5_W { w: self }
    }
    #[doc = "Bit 22 - Port n Reset Bit 6"]
    #[inline(always)]
    pub fn pr6(&mut self) -> PR6_W {
        PR6_W { w: self }
    }
    #[doc = "Bit 23 - Port n Reset Bit 7"]
    #[inline(always)]
    pub fn pr7(&mut self) -> PR7_W {
        PR7_W { w: self }
    }
    #[doc = "Bit 24 - Port n Reset Bit 8"]
    #[inline(always)]
    pub fn pr8(&mut self) -> PR8_W {
        PR8_W { w: self }
    }
    #[doc = "Bit 25 - Port n Reset Bit 9"]
    #[inline(always)]
    pub fn pr9(&mut self) -> PR9_W {
        PR9_W { w: self }
    }
    #[doc = "Bit 26 - Port n Reset Bit 10"]
    #[inline(always)]
    pub fn pr10(&mut self) -> PR10_W {
        PR10_W { w: self }
    }
    #[doc = "Bit 27 - Port n Reset Bit 11"]
    #[inline(always)]
    pub fn pr11(&mut self) -> PR11_W {
        PR11_W { w: self }
    }
    #[doc = "Bit 28 - Port n Reset Bit 12"]
    #[inline(always)]
    pub fn pr12(&mut self) -> PR12_W {
        PR12_W { w: self }
    }
    #[doc = "Bit 29 - Port n Reset Bit 13"]
    #[inline(always)]
    pub fn pr13(&mut self) -> PR13_W {
        PR13_W { w: self }
    }
    #[doc = "Bit 30 - Port n Reset Bit 14"]
    #[inline(always)]
    pub fn pr14(&mut self) -> PR14_W {
        PR14_W { w: self }
    }
    #[doc = "Bit 31 - Port n Reset Bit 15"]
    #[inline(always)]
    pub fn pr15(&mut self) -> PR15_W {
        PR15_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Port 0 Output Modification Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [omr](index.html) module"]
pub struct OMR_SPEC;
impl crate::RegisterSpec for OMR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [omr::W](W) writer structure"]
impl crate::Writable for OMR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OMR to value 0"]
impl crate::Resettable for OMR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
