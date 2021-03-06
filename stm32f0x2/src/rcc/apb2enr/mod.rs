#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::APB2ENR {
    #[doc = r" Modifies the contents of the register"]
    #[inline(always)]
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
    #[inline(always)]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
    #[doc = r" Writes to the register"]
    #[inline(always)]
    pub fn write<F>(&self, f: F)
    where
        F: FnOnce(&mut W) -> &mut W,
    {
        let mut w = W::reset_value();
        f(&mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Writes the reset value to the register"]
    #[inline(always)]
    pub fn reset(&self) {
        self.write(|w| w)
    }
}
#[doc = "Possible values of the field `SYSCFGEN`"]
pub type SYSCFGENR = super::ahbenr::DMA1ENR;
#[doc = "Possible values of the field `ADCEN`"]
pub type ADCENR = super::ahbenr::DMA1ENR;
#[doc = "Possible values of the field `TIM1EN`"]
pub type TIM1ENR = super::ahbenr::DMA1ENR;
#[doc = "Possible values of the field `SPI1EN`"]
pub type SPI1ENR = super::ahbenr::DMA1ENR;
#[doc = "Possible values of the field `USART1EN`"]
pub type USART1ENR = super::ahbenr::DMA1ENR;
#[doc = "Possible values of the field `TIM15EN`"]
pub type TIM15ENR = super::ahbenr::DMA1ENR;
#[doc = "Possible values of the field `TIM16EN`"]
pub type TIM16ENR = super::ahbenr::DMA1ENR;
#[doc = "Possible values of the field `TIM17EN`"]
pub type TIM17ENR = super::ahbenr::DMA1ENR;
#[doc = "Possible values of the field `DBGMCUEN`"]
pub type DBGMCUENR = super::ahbenr::DMA1ENR;
#[doc = "Values that can be written to the field `SYSCFGEN`"]
pub type SYSCFGENW = super::ahbenr::DMA1ENW;
#[doc = r" Proxy"]
pub struct _SYSCFGENW<'a> {
    w: &'a mut W,
}
impl<'a> _SYSCFGENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SYSCFGENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(super::ahbenr::DMA1ENW::DISABLED)
    }
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(super::ahbenr::DMA1ENW::ENABLED)
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
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ADCEN`"]
pub type ADCENW = super::ahbenr::DMA1ENW;
#[doc = r" Proxy"]
pub struct _ADCENW<'a> {
    w: &'a mut W,
}
impl<'a> _ADCENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADCENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(super::ahbenr::DMA1ENW::DISABLED)
    }
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(super::ahbenr::DMA1ENW::ENABLED)
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
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 9;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TIM1EN`"]
pub type TIM1ENW = super::ahbenr::DMA1ENW;
#[doc = r" Proxy"]
pub struct _TIM1ENW<'a> {
    w: &'a mut W,
}
impl<'a> _TIM1ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIM1ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(super::ahbenr::DMA1ENW::DISABLED)
    }
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(super::ahbenr::DMA1ENW::ENABLED)
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
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 11;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SPI1EN`"]
pub type SPI1ENW = super::ahbenr::DMA1ENW;
#[doc = r" Proxy"]
pub struct _SPI1ENW<'a> {
    w: &'a mut W,
}
impl<'a> _SPI1ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SPI1ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(super::ahbenr::DMA1ENW::DISABLED)
    }
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(super::ahbenr::DMA1ENW::ENABLED)
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
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `USART1EN`"]
pub type USART1ENW = super::ahbenr::DMA1ENW;
#[doc = r" Proxy"]
pub struct _USART1ENW<'a> {
    w: &'a mut W,
}
impl<'a> _USART1ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USART1ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(super::ahbenr::DMA1ENW::DISABLED)
    }
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(super::ahbenr::DMA1ENW::ENABLED)
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
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 14;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TIM15EN`"]
pub type TIM15ENW = super::ahbenr::DMA1ENW;
#[doc = r" Proxy"]
pub struct _TIM15ENW<'a> {
    w: &'a mut W,
}
impl<'a> _TIM15ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIM15ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(super::ahbenr::DMA1ENW::DISABLED)
    }
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(super::ahbenr::DMA1ENW::ENABLED)
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
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TIM16EN`"]
pub type TIM16ENW = super::ahbenr::DMA1ENW;
#[doc = r" Proxy"]
pub struct _TIM16ENW<'a> {
    w: &'a mut W,
}
impl<'a> _TIM16ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIM16ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(super::ahbenr::DMA1ENW::DISABLED)
    }
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(super::ahbenr::DMA1ENW::ENABLED)
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
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 17;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TIM17EN`"]
pub type TIM17ENW = super::ahbenr::DMA1ENW;
#[doc = r" Proxy"]
pub struct _TIM17ENW<'a> {
    w: &'a mut W,
}
impl<'a> _TIM17ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIM17ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(super::ahbenr::DMA1ENW::DISABLED)
    }
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(super::ahbenr::DMA1ENW::ENABLED)
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
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 18;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `DBGMCUEN`"]
pub type DBGMCUENW = super::ahbenr::DMA1ENW;
#[doc = r" Proxy"]
pub struct _DBGMCUENW<'a> {
    w: &'a mut W,
}
impl<'a> _DBGMCUENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DBGMCUENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(super::ahbenr::DMA1ENW::DISABLED)
    }
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(super::ahbenr::DMA1ENW::ENABLED)
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
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 22;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - SYSCFG clock enable"]
    #[inline(always)]
    pub fn syscfgen(&self) -> SYSCFGENR {
        SYSCFGENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - ADC 1 interface clock enable"]
    #[inline(always)]
    pub fn adcen(&self) -> ADCENR {
        ADCENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - TIM1 Timer clock enable"]
    #[inline(always)]
    pub fn tim1en(&self) -> TIM1ENR {
        TIM1ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 12 - SPI 1 clock enable"]
    #[inline(always)]
    pub fn spi1en(&self) -> SPI1ENR {
        SPI1ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 14 - USART1 clock enable"]
    #[inline(always)]
    pub fn usart1en(&self) -> USART1ENR {
        USART1ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 16 - TIM15 timer clock enable"]
    #[inline(always)]
    pub fn tim15en(&self) -> TIM15ENR {
        TIM15ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 17 - TIM16 timer clock enable"]
    #[inline(always)]
    pub fn tim16en(&self) -> TIM16ENR {
        TIM16ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 18 - TIM17 timer clock enable"]
    #[inline(always)]
    pub fn tim17en(&self) -> TIM17ENR {
        TIM17ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 22 - MCU debug module clock enable"]
    #[inline(always)]
    pub fn dbgmcuen(&self) -> DBGMCUENR {
        DBGMCUENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline(always)]
    pub fn reset_value() -> W {
        W { bits: 0 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - SYSCFG clock enable"]
    #[inline(always)]
    pub fn syscfgen(&mut self) -> _SYSCFGENW {
        _SYSCFGENW { w: self }
    }
    #[doc = "Bit 9 - ADC 1 interface clock enable"]
    #[inline(always)]
    pub fn adcen(&mut self) -> _ADCENW {
        _ADCENW { w: self }
    }
    #[doc = "Bit 11 - TIM1 Timer clock enable"]
    #[inline(always)]
    pub fn tim1en(&mut self) -> _TIM1ENW {
        _TIM1ENW { w: self }
    }
    #[doc = "Bit 12 - SPI 1 clock enable"]
    #[inline(always)]
    pub fn spi1en(&mut self) -> _SPI1ENW {
        _SPI1ENW { w: self }
    }
    #[doc = "Bit 14 - USART1 clock enable"]
    #[inline(always)]
    pub fn usart1en(&mut self) -> _USART1ENW {
        _USART1ENW { w: self }
    }
    #[doc = "Bit 16 - TIM15 timer clock enable"]
    #[inline(always)]
    pub fn tim15en(&mut self) -> _TIM15ENW {
        _TIM15ENW { w: self }
    }
    #[doc = "Bit 17 - TIM16 timer clock enable"]
    #[inline(always)]
    pub fn tim16en(&mut self) -> _TIM16ENW {
        _TIM16ENW { w: self }
    }
    #[doc = "Bit 18 - TIM17 timer clock enable"]
    #[inline(always)]
    pub fn tim17en(&mut self) -> _TIM17ENW {
        _TIM17ENW { w: self }
    }
    #[doc = "Bit 22 - MCU debug module clock enable"]
    #[inline(always)]
    pub fn dbgmcuen(&mut self) -> _DBGMCUENW {
        _DBGMCUENW { w: self }
    }
}
