#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::HCFG {
    #[doc = r" Modifies the contents of the register"]
    #[inline]
    pub fn modify<F>(&self, f: F)
    where
        for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
    {
        let bits = self.register.get();
        let r = R { bits: bits };
        let mut w = W { bits: bits };
        f(&r, &mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
    #[doc = r" Writes to the register"]
    #[inline]
    pub fn write<F>(&self, f: F)
    where
        F: FnOnce(&mut W) -> &mut W,
    {
        let mut w = W::reset_value();
        f(&mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Writes the reset value to the register"]
    #[inline]
    pub fn reset(&self) {
        self.write(|w| w)
    }
}
#[doc = "Possible values of the field `FSLSPclkSel`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FSLSPCLKSELR {
    #[doc = "PHY clock is running at 48 MHz"]
    VALUE1,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl FSLSPCLKSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            FSLSPCLKSELR::VALUE1 => 1,
            FSLSPCLKSELR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> FSLSPCLKSELR {
        match value {
            1 => FSLSPCLKSELR::VALUE1,
            i => FSLSPCLKSELR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == FSLSPCLKSELR::VALUE1
    }
}
#[doc = "Possible values of the field `FSLSSupp`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FSLSSUPPR {
    #[doc = "FS-only, connected device can supports also only FS."]
    VALUE1,
    #[doc = "FS-only, even if the connected device can support HS"]
    VALUE2,
}
impl FSLSSUPPR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            FSLSSUPPR::VALUE1 => false,
            FSLSSUPPR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FSLSSUPPR {
        match value {
            false => FSLSSUPPR::VALUE1,
            true => FSLSSUPPR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == FSLSSUPPR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == FSLSSUPPR::VALUE2
    }
}
#[doc = r" Value of the field"]
pub struct DESCDMAR {
    bits: bool,
}
impl DESCDMAR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = "Possible values of the field `FrListEn`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FRLISTENR {
    #[doc = "8 Entries"]
    VALUE1,
    #[doc = "16 Entries"]
    VALUE2,
    #[doc = "32 Entries"]
    VALUE3,
    #[doc = "64 Entries"]
    VALUE4,
}
impl FRLISTENR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            FRLISTENR::VALUE1 => 0,
            FRLISTENR::VALUE2 => 1,
            FRLISTENR::VALUE3 => 2,
            FRLISTENR::VALUE4 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> FRLISTENR {
        match value {
            0 => FRLISTENR::VALUE1,
            1 => FRLISTENR::VALUE2,
            2 => FRLISTENR::VALUE3,
            3 => FRLISTENR::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == FRLISTENR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == FRLISTENR::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == FRLISTENR::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == FRLISTENR::VALUE4
    }
}
#[doc = r" Value of the field"]
pub struct PERSCHEDENAR {
    bits: bool,
}
impl PERSCHEDENAR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = "Values that can be written to the field `FSLSPclkSel`"]
pub enum FSLSPCLKSELW {
    #[doc = "PHY clock is running at 48 MHz"]
    VALUE1,
}
impl FSLSPCLKSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            FSLSPCLKSELW::VALUE1 => 1,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FSLSPCLKSELW<'a> {
    w: &'a mut W,
}
impl<'a> _FSLSPCLKSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FSLSPCLKSELW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "PHY clock is running at 48 MHz"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(FSLSPCLKSELW::VALUE1)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `FSLSSupp`"]
pub enum FSLSSUPPW {
    #[doc = "FS-only, connected device can supports also only FS."]
    VALUE1,
    #[doc = "FS-only, even if the connected device can support HS"]
    VALUE2,
}
impl FSLSSUPPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FSLSSUPPW::VALUE1 => false,
            FSLSSUPPW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FSLSSUPPW<'a> {
    w: &'a mut W,
}
impl<'a> _FSLSSUPPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FSLSSUPPW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "FS-only, connected device can supports also only FS."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(FSLSSUPPW::VALUE1)
    }
    #[doc = "FS-only, even if the connected device can support HS"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(FSLSSUPPW::VALUE2)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DESCDMAW<'a> {
    w: &'a mut W,
}
impl<'a> _DESCDMAW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 23;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `FrListEn`"]
pub enum FRLISTENW {
    #[doc = "8 Entries"]
    VALUE1,
    #[doc = "16 Entries"]
    VALUE2,
    #[doc = "32 Entries"]
    VALUE3,
    #[doc = "64 Entries"]
    VALUE4,
}
impl FRLISTENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            FRLISTENW::VALUE1 => 0,
            FRLISTENW::VALUE2 => 1,
            FRLISTENW::VALUE3 => 2,
            FRLISTENW::VALUE4 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FRLISTENW<'a> {
    w: &'a mut W,
}
impl<'a> _FRLISTENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FRLISTENW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "8 Entries"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(FRLISTENW::VALUE1)
    }
    #[doc = "16 Entries"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(FRLISTENW::VALUE2)
    }
    #[doc = "32 Entries"]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(FRLISTENW::VALUE3)
    }
    #[doc = "64 Entries"]
    #[inline]
    pub fn value4(self) -> &'a mut W {
        self.variant(FRLISTENW::VALUE4)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PERSCHEDENAW<'a> {
    w: &'a mut W,
}
impl<'a> _PERSCHEDENAW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 26;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:1 - FS PHY Clock Select"]
    #[inline]
    pub fn fslspclk_sel(&self) -> FSLSPCLKSELR {
        FSLSPCLKSELR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 2 - FS-Only Support"]
    #[inline]
    pub fn fslssupp(&self) -> FSLSSUPPR {
        FSLSSUPPR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 23 - Enable Scatter/gather DMA in Host mode"]
    #[inline]
    pub fn desc_dma(&self) -> DESCDMAR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DESCDMAR { bits }
    }
    #[doc = "Bits 24:25 - Frame List Entries"]
    #[inline]
    pub fn fr_list_en(&self) -> FRLISTENR {
        FRLISTENR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 26 - Enable Periodic Scheduling"]
    #[inline]
    pub fn per_sched_ena(&self) -> PERSCHEDENAR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PERSCHEDENAR { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 512 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:1 - FS PHY Clock Select"]
    #[inline]
    pub fn fslspclk_sel(&mut self) -> _FSLSPCLKSELW {
        _FSLSPCLKSELW { w: self }
    }
    #[doc = "Bit 2 - FS-Only Support"]
    #[inline]
    pub fn fslssupp(&mut self) -> _FSLSSUPPW {
        _FSLSSUPPW { w: self }
    }
    #[doc = "Bit 23 - Enable Scatter/gather DMA in Host mode"]
    #[inline]
    pub fn desc_dma(&mut self) -> _DESCDMAW {
        _DESCDMAW { w: self }
    }
    #[doc = "Bits 24:25 - Frame List Entries"]
    #[inline]
    pub fn fr_list_en(&mut self) -> _FRLISTENW {
        _FRLISTENW { w: self }
    }
    #[doc = "Bit 26 - Enable Periodic Scheduling"]
    #[inline]
    pub fn per_sched_ena(&mut self) -> _PERSCHEDENAW {
        _PERSCHEDENAW { w: self }
    }
}
