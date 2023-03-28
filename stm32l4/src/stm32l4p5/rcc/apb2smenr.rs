#[doc = "Register `APB2SMENR` reader"]
pub struct R(crate::R<APB2SMENR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<APB2SMENR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<APB2SMENR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<APB2SMENR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `APB2SMENR` writer"]
pub struct W(crate::W<APB2SMENR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<APB2SMENR_SPEC>;
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
impl From<crate::W<APB2SMENR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<APB2SMENR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SYSCFGSMEN` reader - SYSCFG clocks enable during Sleep and Stop modes"]
pub type SYSCFGSMEN_R = crate::BitReader<SYSCFGSMEN_A>;
#[doc = "SYSCFG clocks enable during Sleep and Stop modes\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SYSCFGSMEN_A {
    #[doc = "0: SYSCFG + COMP + VREFBUF clocks disabled by the clock gating during Sleep and Stop modes"]
    Disabled = 0,
    #[doc = "1: SYSCFG + COMP + VREFBUF clocks enabled by the clock gating(1) during Sleep and Stop modes"]
    Enabled = 1,
}
impl From<SYSCFGSMEN_A> for bool {
    #[inline(always)]
    fn from(variant: SYSCFGSMEN_A) -> Self {
        variant as u8 != 0
    }
}
impl SYSCFGSMEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SYSCFGSMEN_A {
        match self.bits {
            false => SYSCFGSMEN_A::Disabled,
            true => SYSCFGSMEN_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SYSCFGSMEN_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SYSCFGSMEN_A::Enabled
    }
}
#[doc = "Field `SYSCFGSMEN` writer - SYSCFG clocks enable during Sleep and Stop modes"]
pub type SYSCFGSMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB2SMENR_SPEC, SYSCFGSMEN_A, O>;
impl<'a, const O: u8> SYSCFGSMEN_W<'a, O> {
    #[doc = "SYSCFG + COMP + VREFBUF clocks disabled by the clock gating during Sleep and Stop modes"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SYSCFGSMEN_A::Disabled)
    }
    #[doc = "SYSCFG + COMP + VREFBUF clocks enabled by the clock gating(1) during Sleep and Stop modes"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SYSCFGSMEN_A::Enabled)
    }
}
#[doc = "Field `TIM1SMEN` reader - TIM1 timer clocks enable during Sleep and Stop modes"]
pub type TIM1SMEN_R = crate::BitReader<TIM1SMEN_A>;
#[doc = "TIM1 timer clocks enable during Sleep and Stop modes\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIM1SMEN_A {
    #[doc = "0: TIMx clocks disabled by the clock gating during Sleep and Stop modes"]
    Disabled = 0,
    #[doc = "1: TIMx clocks enabled by the clock gating(1) during Sleep and Stop modes"]
    Enabled = 1,
}
impl From<TIM1SMEN_A> for bool {
    #[inline(always)]
    fn from(variant: TIM1SMEN_A) -> Self {
        variant as u8 != 0
    }
}
impl TIM1SMEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIM1SMEN_A {
        match self.bits {
            false => TIM1SMEN_A::Disabled,
            true => TIM1SMEN_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TIM1SMEN_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TIM1SMEN_A::Enabled
    }
}
#[doc = "Field `TIM1SMEN` writer - TIM1 timer clocks enable during Sleep and Stop modes"]
pub type TIM1SMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB2SMENR_SPEC, TIM1SMEN_A, O>;
impl<'a, const O: u8> TIM1SMEN_W<'a, O> {
    #[doc = "TIMx clocks disabled by the clock gating during Sleep and Stop modes"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TIM1SMEN_A::Disabled)
    }
    #[doc = "TIMx clocks enabled by the clock gating(1) during Sleep and Stop modes"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TIM1SMEN_A::Enabled)
    }
}
#[doc = "Field `SPI1SMEN` reader - SPI1 clocks enable during Sleep and Stop modes"]
pub type SPI1SMEN_R = crate::BitReader<SPI1SMEN_A>;
#[doc = "SPI1 clocks enable during Sleep and Stop modes\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SPI1SMEN_A {
    #[doc = "0: SPI1 clocks disabled by the clock gating during Sleep and Stop modes"]
    Disabled = 0,
    #[doc = "1: SPI1 clocks enabled by the clock gating(1) during Sleep and Stop modes"]
    Enabled = 1,
}
impl From<SPI1SMEN_A> for bool {
    #[inline(always)]
    fn from(variant: SPI1SMEN_A) -> Self {
        variant as u8 != 0
    }
}
impl SPI1SMEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPI1SMEN_A {
        match self.bits {
            false => SPI1SMEN_A::Disabled,
            true => SPI1SMEN_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SPI1SMEN_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SPI1SMEN_A::Enabled
    }
}
#[doc = "Field `SPI1SMEN` writer - SPI1 clocks enable during Sleep and Stop modes"]
pub type SPI1SMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB2SMENR_SPEC, SPI1SMEN_A, O>;
impl<'a, const O: u8> SPI1SMEN_W<'a, O> {
    #[doc = "SPI1 clocks disabled by the clock gating during Sleep and Stop modes"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SPI1SMEN_A::Disabled)
    }
    #[doc = "SPI1 clocks enabled by the clock gating(1) during Sleep and Stop modes"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SPI1SMEN_A::Enabled)
    }
}
#[doc = "Field `TIM8SMEN` reader - TIM8 timer clocks enable during Sleep and Stop modes"]
pub use TIM1SMEN_R as TIM8SMEN_R;
#[doc = "Field `TIM8SMEN` writer - TIM8 timer clocks enable during Sleep and Stop modes"]
pub use TIM1SMEN_W as TIM8SMEN_W;
#[doc = "Field `USART1SMEN` reader - USART1clocks enable during Sleep and Stop modes"]
pub type USART1SMEN_R = crate::BitReader<USART1SMEN_A>;
#[doc = "USART1clocks enable during Sleep and Stop modes\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USART1SMEN_A {
    #[doc = "0: USART1 clocks disabled by the clock gating during Sleep and Stop modes"]
    Disabled = 0,
    #[doc = "1: USART1 clocks enabled by the clock gating(1) during Sleep and Stop modes"]
    Enabled = 1,
}
impl From<USART1SMEN_A> for bool {
    #[inline(always)]
    fn from(variant: USART1SMEN_A) -> Self {
        variant as u8 != 0
    }
}
impl USART1SMEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USART1SMEN_A {
        match self.bits {
            false => USART1SMEN_A::Disabled,
            true => USART1SMEN_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == USART1SMEN_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == USART1SMEN_A::Enabled
    }
}
#[doc = "Field `USART1SMEN` writer - USART1clocks enable during Sleep and Stop modes"]
pub type USART1SMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB2SMENR_SPEC, USART1SMEN_A, O>;
impl<'a, const O: u8> USART1SMEN_W<'a, O> {
    #[doc = "USART1 clocks disabled by the clock gating during Sleep and Stop modes"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(USART1SMEN_A::Disabled)
    }
    #[doc = "USART1 clocks enabled by the clock gating(1) during Sleep and Stop modes"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(USART1SMEN_A::Enabled)
    }
}
#[doc = "Field `TIM15SMEN` reader - TIM15 timer clocks enable during Sleep and Stop modes"]
pub use TIM1SMEN_R as TIM15SMEN_R;
#[doc = "Field `TIM16SMEN` reader - TIM16 timer clocks enable during Sleep and Stop modes"]
pub use TIM1SMEN_R as TIM16SMEN_R;
#[doc = "Field `TIM17SMEN` reader - TIM17 timer clocks enable during Sleep and Stop modes"]
pub use TIM1SMEN_R as TIM17SMEN_R;
#[doc = "Field `TIM15SMEN` writer - TIM15 timer clocks enable during Sleep and Stop modes"]
pub use TIM1SMEN_W as TIM15SMEN_W;
#[doc = "Field `TIM16SMEN` writer - TIM16 timer clocks enable during Sleep and Stop modes"]
pub use TIM1SMEN_W as TIM16SMEN_W;
#[doc = "Field `TIM17SMEN` writer - TIM17 timer clocks enable during Sleep and Stop modes"]
pub use TIM1SMEN_W as TIM17SMEN_W;
#[doc = "Field `SAI1SMEN` reader - SAI1 clocks enable during Sleep and Stop modes"]
pub type SAI1SMEN_R = crate::BitReader<SAI1SMEN_A>;
#[doc = "SAI1 clocks enable during Sleep and Stop modes\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SAI1SMEN_A {
    #[doc = "0: SAIx clocks disabled by the clock gating during Sleep and Stop modes"]
    Disabled = 0,
    #[doc = "1: SAIx clocks enabled by the clock gating(1) during Sleep and Stop modes"]
    Enabled = 1,
}
impl From<SAI1SMEN_A> for bool {
    #[inline(always)]
    fn from(variant: SAI1SMEN_A) -> Self {
        variant as u8 != 0
    }
}
impl SAI1SMEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SAI1SMEN_A {
        match self.bits {
            false => SAI1SMEN_A::Disabled,
            true => SAI1SMEN_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SAI1SMEN_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SAI1SMEN_A::Enabled
    }
}
#[doc = "Field `SAI1SMEN` writer - SAI1 clocks enable during Sleep and Stop modes"]
pub type SAI1SMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB2SMENR_SPEC, SAI1SMEN_A, O>;
impl<'a, const O: u8> SAI1SMEN_W<'a, O> {
    #[doc = "SAIx clocks disabled by the clock gating during Sleep and Stop modes"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SAI1SMEN_A::Disabled)
    }
    #[doc = "SAIx clocks enabled by the clock gating(1) during Sleep and Stop modes"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SAI1SMEN_A::Enabled)
    }
}
#[doc = "Field `SAI2SMEN` reader - SAI2 clocks enable during Sleep and Stop modes"]
pub use SAI1SMEN_R as SAI2SMEN_R;
#[doc = "Field `SAI2SMEN` writer - SAI2 clocks enable during Sleep and Stop modes"]
pub use SAI1SMEN_W as SAI2SMEN_W;
#[doc = "Field `DFSDM1SMEN` reader - DFSDM timer clocks enable during Sleep and Stop modes"]
pub type DFSDM1SMEN_R = crate::BitReader<DFSDM1SMEN_A>;
#[doc = "DFSDM timer clocks enable during Sleep and Stop modes\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DFSDM1SMEN_A {
    #[doc = "0: DFSDM1 clocks disabled by the clock gating during Sleep and Stop modes"]
    Disabled = 0,
    #[doc = "1: DFSDM1 clocks enabled by the clock gating(1) during Sleep and Stop modes"]
    Enabled = 1,
}
impl From<DFSDM1SMEN_A> for bool {
    #[inline(always)]
    fn from(variant: DFSDM1SMEN_A) -> Self {
        variant as u8 != 0
    }
}
impl DFSDM1SMEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DFSDM1SMEN_A {
        match self.bits {
            false => DFSDM1SMEN_A::Disabled,
            true => DFSDM1SMEN_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DFSDM1SMEN_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DFSDM1SMEN_A::Enabled
    }
}
#[doc = "Field `DFSDM1SMEN` writer - DFSDM timer clocks enable during Sleep and Stop modes"]
pub type DFSDM1SMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB2SMENR_SPEC, DFSDM1SMEN_A, O>;
impl<'a, const O: u8> DFSDM1SMEN_W<'a, O> {
    #[doc = "DFSDM1 clocks disabled by the clock gating during Sleep and Stop modes"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DFSDM1SMEN_A::Disabled)
    }
    #[doc = "DFSDM1 clocks enabled by the clock gating(1) during Sleep and Stop modes"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DFSDM1SMEN_A::Enabled)
    }
}
#[doc = "Field `LTDCSMEN` reader - LCD-TFT timer clocks enable during Sleep and Stop modes"]
pub type LTDCSMEN_R = crate::BitReader<LTDCSMEN_A>;
#[doc = "LCD-TFT timer clocks enable during Sleep and Stop modes\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LTDCSMEN_A {
    #[doc = "0: LCD-TFT clocks disabled by the clock gating during Sleep and Stop modes"]
    Disabled = 0,
    #[doc = "1: LCD-TFT clocks enabled by the clock gating(1) during Sleep and Stop modes"]
    Enabled = 1,
}
impl From<LTDCSMEN_A> for bool {
    #[inline(always)]
    fn from(variant: LTDCSMEN_A) -> Self {
        variant as u8 != 0
    }
}
impl LTDCSMEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LTDCSMEN_A {
        match self.bits {
            false => LTDCSMEN_A::Disabled,
            true => LTDCSMEN_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == LTDCSMEN_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == LTDCSMEN_A::Enabled
    }
}
#[doc = "Field `LTDCSMEN` writer - LCD-TFT timer clocks enable during Sleep and Stop modes"]
pub type LTDCSMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB2SMENR_SPEC, LTDCSMEN_A, O>;
impl<'a, const O: u8> LTDCSMEN_W<'a, O> {
    #[doc = "LCD-TFT clocks disabled by the clock gating during Sleep and Stop modes"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(LTDCSMEN_A::Disabled)
    }
    #[doc = "LCD-TFT clocks enabled by the clock gating(1) during Sleep and Stop modes"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(LTDCSMEN_A::Enabled)
    }
}
#[doc = "Field `DSISMEN` reader - DSI clocks enable during Sleep and Stop modes"]
pub type DSISMEN_R = crate::BitReader<DSISMEN_A>;
#[doc = "DSI clocks enable during Sleep and Stop modes\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DSISMEN_A {
    #[doc = "0: DSI clocks disabled by the clock gating during Sleep and Stop modes"]
    Disabled = 0,
    #[doc = "1: DSI clocks enabled by the clock gating(1) during Sleep and Stop modes"]
    Enabled = 1,
}
impl From<DSISMEN_A> for bool {
    #[inline(always)]
    fn from(variant: DSISMEN_A) -> Self {
        variant as u8 != 0
    }
}
impl DSISMEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DSISMEN_A {
        match self.bits {
            false => DSISMEN_A::Disabled,
            true => DSISMEN_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DSISMEN_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DSISMEN_A::Enabled
    }
}
#[doc = "Field `DSISMEN` writer - DSI clocks enable during Sleep and Stop modes"]
pub type DSISMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB2SMENR_SPEC, DSISMEN_A, O>;
impl<'a, const O: u8> DSISMEN_W<'a, O> {
    #[doc = "DSI clocks disabled by the clock gating during Sleep and Stop modes"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DSISMEN_A::Disabled)
    }
    #[doc = "DSI clocks enabled by the clock gating(1) during Sleep and Stop modes"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DSISMEN_A::Enabled)
    }
}
impl R {
    #[doc = "Bit 0 - SYSCFG clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn syscfgsmen(&self) -> SYSCFGSMEN_R {
        SYSCFGSMEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 11 - TIM1 timer clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn tim1smen(&self) -> TIM1SMEN_R {
        TIM1SMEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - SPI1 clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn spi1smen(&self) -> SPI1SMEN_R {
        SPI1SMEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - TIM8 timer clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn tim8smen(&self) -> TIM8SMEN_R {
        TIM8SMEN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - USART1clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn usart1smen(&self) -> USART1SMEN_R {
        USART1SMEN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - TIM15 timer clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn tim15smen(&self) -> TIM15SMEN_R {
        TIM15SMEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - TIM16 timer clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn tim16smen(&self) -> TIM16SMEN_R {
        TIM16SMEN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - TIM17 timer clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn tim17smen(&self) -> TIM17SMEN_R {
        TIM17SMEN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 21 - SAI1 clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn sai1smen(&self) -> SAI1SMEN_R {
        SAI1SMEN_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - SAI2 clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn sai2smen(&self) -> SAI2SMEN_R {
        SAI2SMEN_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 24 - DFSDM timer clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn dfsdm1smen(&self) -> DFSDM1SMEN_R {
        DFSDM1SMEN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 26 - LCD-TFT timer clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn ltdcsmen(&self) -> LTDCSMEN_R {
        LTDCSMEN_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - DSI clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn dsismen(&self) -> DSISMEN_R {
        DSISMEN_R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SYSCFG clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    #[must_use]
    pub fn syscfgsmen(&mut self) -> SYSCFGSMEN_W<0> {
        SYSCFGSMEN_W::new(self)
    }
    #[doc = "Bit 11 - TIM1 timer clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    #[must_use]
    pub fn tim1smen(&mut self) -> TIM1SMEN_W<11> {
        TIM1SMEN_W::new(self)
    }
    #[doc = "Bit 12 - SPI1 clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    #[must_use]
    pub fn spi1smen(&mut self) -> SPI1SMEN_W<12> {
        SPI1SMEN_W::new(self)
    }
    #[doc = "Bit 13 - TIM8 timer clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    #[must_use]
    pub fn tim8smen(&mut self) -> TIM8SMEN_W<13> {
        TIM8SMEN_W::new(self)
    }
    #[doc = "Bit 14 - USART1clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    #[must_use]
    pub fn usart1smen(&mut self) -> USART1SMEN_W<14> {
        USART1SMEN_W::new(self)
    }
    #[doc = "Bit 16 - TIM15 timer clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    #[must_use]
    pub fn tim15smen(&mut self) -> TIM15SMEN_W<16> {
        TIM15SMEN_W::new(self)
    }
    #[doc = "Bit 17 - TIM16 timer clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    #[must_use]
    pub fn tim16smen(&mut self) -> TIM16SMEN_W<17> {
        TIM16SMEN_W::new(self)
    }
    #[doc = "Bit 18 - TIM17 timer clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    #[must_use]
    pub fn tim17smen(&mut self) -> TIM17SMEN_W<18> {
        TIM17SMEN_W::new(self)
    }
    #[doc = "Bit 21 - SAI1 clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    #[must_use]
    pub fn sai1smen(&mut self) -> SAI1SMEN_W<21> {
        SAI1SMEN_W::new(self)
    }
    #[doc = "Bit 22 - SAI2 clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    #[must_use]
    pub fn sai2smen(&mut self) -> SAI2SMEN_W<22> {
        SAI2SMEN_W::new(self)
    }
    #[doc = "Bit 24 - DFSDM timer clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    #[must_use]
    pub fn dfsdm1smen(&mut self) -> DFSDM1SMEN_W<24> {
        DFSDM1SMEN_W::new(self)
    }
    #[doc = "Bit 26 - LCD-TFT timer clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    #[must_use]
    pub fn ltdcsmen(&mut self) -> LTDCSMEN_W<26> {
        LTDCSMEN_W::new(self)
    }
    #[doc = "Bit 27 - DSI clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    #[must_use]
    pub fn dsismen(&mut self) -> DSISMEN_W<27> {
        DSISMEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "APB2SMENR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apb2smenr](index.html) module"]
pub struct APB2SMENR_SPEC;
impl crate::RegisterSpec for APB2SMENR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [apb2smenr::R](R) reader structure"]
impl crate::Readable for APB2SMENR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [apb2smenr::W](W) writer structure"]
impl crate::Writable for APB2SMENR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets APB2SMENR to value 0x0d67_7801"]
impl crate::Resettable for APB2SMENR_SPEC {
    const RESET_VALUE: Self::Ux = 0x0d67_7801;
}
