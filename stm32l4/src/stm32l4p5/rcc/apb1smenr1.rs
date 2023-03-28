#[doc = "Register `APB1SMENR1` reader"]
pub struct R(crate::R<APB1SMENR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<APB1SMENR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<APB1SMENR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<APB1SMENR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `APB1SMENR1` writer"]
pub struct W(crate::W<APB1SMENR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<APB1SMENR1_SPEC>;
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
impl From<crate::W<APB1SMENR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<APB1SMENR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TIM2SMEN` reader - TIM2 timer clocks enable during Sleep and Stop modes"]
pub type TIM2SMEN_R = crate::BitReader<TIM2SMEN_A>;
#[doc = "TIM2 timer clocks enable during Sleep and Stop modes\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIM2SMEN_A {
    #[doc = "0: TIMx clocks disabled by the clock gating during Sleep and Stop modes"]
    Disabled = 0,
    #[doc = "1: TIMx clocks enabled by the clock gating(1) during Sleep and Stop modes"]
    Enabled = 1,
}
impl From<TIM2SMEN_A> for bool {
    #[inline(always)]
    fn from(variant: TIM2SMEN_A) -> Self {
        variant as u8 != 0
    }
}
impl TIM2SMEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIM2SMEN_A {
        match self.bits {
            false => TIM2SMEN_A::Disabled,
            true => TIM2SMEN_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TIM2SMEN_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TIM2SMEN_A::Enabled
    }
}
#[doc = "Field `TIM2SMEN` writer - TIM2 timer clocks enable during Sleep and Stop modes"]
pub type TIM2SMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1SMENR1_SPEC, TIM2SMEN_A, O>;
impl<'a, const O: u8> TIM2SMEN_W<'a, O> {
    #[doc = "TIMx clocks disabled by the clock gating during Sleep and Stop modes"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TIM2SMEN_A::Disabled)
    }
    #[doc = "TIMx clocks enabled by the clock gating(1) during Sleep and Stop modes"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TIM2SMEN_A::Enabled)
    }
}
#[doc = "Field `TIM3SMEN` reader - TIM3 timer clocks enable during Sleep and Stop modes"]
pub use TIM2SMEN_R as TIM3SMEN_R;
#[doc = "Field `TIM4SMEN` reader - TIM4 timer clocks enable during Sleep and Stop modes"]
pub use TIM2SMEN_R as TIM4SMEN_R;
#[doc = "Field `TIM5SMEN` reader - TIM5 timer clocks enable during Sleep and Stop modes"]
pub use TIM2SMEN_R as TIM5SMEN_R;
#[doc = "Field `TIM6SMEN` reader - TIM6 timer clocks enable during Sleep and Stop modes"]
pub use TIM2SMEN_R as TIM6SMEN_R;
#[doc = "Field `TIM7SMEN` reader - TIM7 timer clocks enable during Sleep and Stop modes"]
pub use TIM2SMEN_R as TIM7SMEN_R;
#[doc = "Field `TIM3SMEN` writer - TIM3 timer clocks enable during Sleep and Stop modes"]
pub use TIM2SMEN_W as TIM3SMEN_W;
#[doc = "Field `TIM4SMEN` writer - TIM4 timer clocks enable during Sleep and Stop modes"]
pub use TIM2SMEN_W as TIM4SMEN_W;
#[doc = "Field `TIM5SMEN` writer - TIM5 timer clocks enable during Sleep and Stop modes"]
pub use TIM2SMEN_W as TIM5SMEN_W;
#[doc = "Field `TIM6SMEN` writer - TIM6 timer clocks enable during Sleep and Stop modes"]
pub use TIM2SMEN_W as TIM6SMEN_W;
#[doc = "Field `TIM7SMEN` writer - TIM7 timer clocks enable during Sleep and Stop modes"]
pub use TIM2SMEN_W as TIM7SMEN_W;
#[doc = "Field `RTCAPBSMEN` reader - RTC APB clock enable during Sleep and Stop modes"]
pub type RTCAPBSMEN_R = crate::BitReader<RTCAPBSMEN_A>;
#[doc = "RTC APB clock enable during Sleep and Stop modes\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RTCAPBSMEN_A {
    #[doc = "0: RTC APB clocks disabled by the clock gating during Sleep and Stop modes"]
    Disabled = 0,
    #[doc = "1: RTC APB clocks enabled by the clock gating(1) during Sleep and Stop modes"]
    Enabled = 1,
}
impl From<RTCAPBSMEN_A> for bool {
    #[inline(always)]
    fn from(variant: RTCAPBSMEN_A) -> Self {
        variant as u8 != 0
    }
}
impl RTCAPBSMEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RTCAPBSMEN_A {
        match self.bits {
            false => RTCAPBSMEN_A::Disabled,
            true => RTCAPBSMEN_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RTCAPBSMEN_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RTCAPBSMEN_A::Enabled
    }
}
#[doc = "Field `RTCAPBSMEN` writer - RTC APB clock enable during Sleep and Stop modes"]
pub type RTCAPBSMEN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, APB1SMENR1_SPEC, RTCAPBSMEN_A, O>;
impl<'a, const O: u8> RTCAPBSMEN_W<'a, O> {
    #[doc = "RTC APB clocks disabled by the clock gating during Sleep and Stop modes"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RTCAPBSMEN_A::Disabled)
    }
    #[doc = "RTC APB clocks enabled by the clock gating(1) during Sleep and Stop modes"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RTCAPBSMEN_A::Enabled)
    }
}
#[doc = "Field `WWDGSMEN` reader - Window watchdog clocks enable during Sleep and Stop modes"]
pub type WWDGSMEN_R = crate::BitReader<WWDGSMEN_A>;
#[doc = "Window watchdog clocks enable during Sleep and Stop modes\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WWDGSMEN_A {
    #[doc = "0: Window watchdog clocks disabled by the clock gating during Sleep and Stop modes"]
    Disabled = 0,
    #[doc = "1: Window watchdog clocks enabled by the clock gating(1) during Sleep and Stop modes"]
    Enabled = 1,
}
impl From<WWDGSMEN_A> for bool {
    #[inline(always)]
    fn from(variant: WWDGSMEN_A) -> Self {
        variant as u8 != 0
    }
}
impl WWDGSMEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WWDGSMEN_A {
        match self.bits {
            false => WWDGSMEN_A::Disabled,
            true => WWDGSMEN_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == WWDGSMEN_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == WWDGSMEN_A::Enabled
    }
}
#[doc = "Field `WWDGSMEN` writer - Window watchdog clocks enable during Sleep and Stop modes"]
pub type WWDGSMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1SMENR1_SPEC, WWDGSMEN_A, O>;
impl<'a, const O: u8> WWDGSMEN_W<'a, O> {
    #[doc = "Window watchdog clocks disabled by the clock gating during Sleep and Stop modes"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(WWDGSMEN_A::Disabled)
    }
    #[doc = "Window watchdog clocks enabled by the clock gating(1) during Sleep and Stop modes"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(WWDGSMEN_A::Enabled)
    }
}
#[doc = "Field `SPI2SMEN` reader - SPI2 clocks enable during Sleep and Stop modes"]
pub type SPI2SMEN_R = crate::BitReader<SPI2SMEN_A>;
#[doc = "SPI2 clocks enable during Sleep and Stop modes\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SPI2SMEN_A {
    #[doc = "0: SPIx clocks disabled by the clock gating during Sleep and Stop modes"]
    Disabled = 0,
    #[doc = "1: SPIx clocks enabled by the clock gating(1) during Sleep and Stop modes"]
    Enabled = 1,
}
impl From<SPI2SMEN_A> for bool {
    #[inline(always)]
    fn from(variant: SPI2SMEN_A) -> Self {
        variant as u8 != 0
    }
}
impl SPI2SMEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPI2SMEN_A {
        match self.bits {
            false => SPI2SMEN_A::Disabled,
            true => SPI2SMEN_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SPI2SMEN_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SPI2SMEN_A::Enabled
    }
}
#[doc = "Field `SPI2SMEN` writer - SPI2 clocks enable during Sleep and Stop modes"]
pub type SPI2SMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1SMENR1_SPEC, SPI2SMEN_A, O>;
impl<'a, const O: u8> SPI2SMEN_W<'a, O> {
    #[doc = "SPIx clocks disabled by the clock gating during Sleep and Stop modes"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SPI2SMEN_A::Disabled)
    }
    #[doc = "SPIx clocks enabled by the clock gating(1) during Sleep and Stop modes"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SPI2SMEN_A::Enabled)
    }
}
#[doc = "Field `SP3SMEN` reader - SPI3 clocks enable during Sleep and Stop modes"]
pub type SP3SMEN_R = crate::BitReader<bool>;
#[doc = "Field `SP3SMEN` writer - SPI3 clocks enable during Sleep and Stop modes"]
pub type SP3SMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1SMENR1_SPEC, bool, O>;
#[doc = "Field `USART2SMEN` reader - USART2 clocks enable during Sleep and Stop modes"]
pub type USART2SMEN_R = crate::BitReader<USART2SMEN_A>;
#[doc = "USART2 clocks enable during Sleep and Stop modes\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USART2SMEN_A {
    #[doc = "0: USARTx clocks disabled by the clock gating during Sleep and Stop modes"]
    Disabled = 0,
    #[doc = "1: USARTx clocks enabled by the clock gating(1) during Sleep and Stop modes"]
    Enabled = 1,
}
impl From<USART2SMEN_A> for bool {
    #[inline(always)]
    fn from(variant: USART2SMEN_A) -> Self {
        variant as u8 != 0
    }
}
impl USART2SMEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USART2SMEN_A {
        match self.bits {
            false => USART2SMEN_A::Disabled,
            true => USART2SMEN_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == USART2SMEN_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == USART2SMEN_A::Enabled
    }
}
#[doc = "Field `USART2SMEN` writer - USART2 clocks enable during Sleep and Stop modes"]
pub type USART2SMEN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, APB1SMENR1_SPEC, USART2SMEN_A, O>;
impl<'a, const O: u8> USART2SMEN_W<'a, O> {
    #[doc = "USARTx clocks disabled by the clock gating during Sleep and Stop modes"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(USART2SMEN_A::Disabled)
    }
    #[doc = "USARTx clocks enabled by the clock gating(1) during Sleep and Stop modes"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(USART2SMEN_A::Enabled)
    }
}
#[doc = "Field `USART3SMEN` reader - USART3 clocks enable during Sleep and Stop modes"]
pub use USART2SMEN_R as USART3SMEN_R;
#[doc = "Field `USART3SMEN` writer - USART3 clocks enable during Sleep and Stop modes"]
pub use USART2SMEN_W as USART3SMEN_W;
#[doc = "Field `UART4SMEN` reader - UART4 clocks enable during Sleep and Stop modes"]
pub type UART4SMEN_R = crate::BitReader<UART4SMEN_A>;
#[doc = "UART4 clocks enable during Sleep and Stop modes\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UART4SMEN_A {
    #[doc = "0: UARTx clocks disabled by the clock gating during Sleep and Stop modes"]
    Disabled = 0,
    #[doc = "1: UARTx clocks enabled by the clock gating(1) during Sleep and Stop modes"]
    Enabled = 1,
}
impl From<UART4SMEN_A> for bool {
    #[inline(always)]
    fn from(variant: UART4SMEN_A) -> Self {
        variant as u8 != 0
    }
}
impl UART4SMEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UART4SMEN_A {
        match self.bits {
            false => UART4SMEN_A::Disabled,
            true => UART4SMEN_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == UART4SMEN_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == UART4SMEN_A::Enabled
    }
}
#[doc = "Field `UART4SMEN` writer - UART4 clocks enable during Sleep and Stop modes"]
pub type UART4SMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1SMENR1_SPEC, UART4SMEN_A, O>;
impl<'a, const O: u8> UART4SMEN_W<'a, O> {
    #[doc = "UARTx clocks disabled by the clock gating during Sleep and Stop modes"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(UART4SMEN_A::Disabled)
    }
    #[doc = "UARTx clocks enabled by the clock gating(1) during Sleep and Stop modes"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(UART4SMEN_A::Enabled)
    }
}
#[doc = "Field `UART5SMEN` reader - UART5 clocks enable during Sleep and Stop modes"]
pub use UART4SMEN_R as UART5SMEN_R;
#[doc = "Field `UART5SMEN` writer - UART5 clocks enable during Sleep and Stop modes"]
pub use UART4SMEN_W as UART5SMEN_W;
#[doc = "Field `I2C1SMEN` reader - I2C1 clocks enable during Sleep and Stop modes"]
pub type I2C1SMEN_R = crate::BitReader<I2C1SMEN_A>;
#[doc = "I2C1 clocks enable during Sleep and Stop modes\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum I2C1SMEN_A {
    #[doc = "0: I2Cx clocks disabled by the clock gating during Sleep and Stop modes"]
    Disabled = 0,
    #[doc = "1: I2Cx clocks enabled by the clock gating(1) during Sleep and Stop modes"]
    Enabled = 1,
}
impl From<I2C1SMEN_A> for bool {
    #[inline(always)]
    fn from(variant: I2C1SMEN_A) -> Self {
        variant as u8 != 0
    }
}
impl I2C1SMEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> I2C1SMEN_A {
        match self.bits {
            false => I2C1SMEN_A::Disabled,
            true => I2C1SMEN_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == I2C1SMEN_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == I2C1SMEN_A::Enabled
    }
}
#[doc = "Field `I2C1SMEN` writer - I2C1 clocks enable during Sleep and Stop modes"]
pub type I2C1SMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1SMENR1_SPEC, I2C1SMEN_A, O>;
impl<'a, const O: u8> I2C1SMEN_W<'a, O> {
    #[doc = "I2Cx clocks disabled by the clock gating during Sleep and Stop modes"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(I2C1SMEN_A::Disabled)
    }
    #[doc = "I2Cx clocks enabled by the clock gating(1) during Sleep and Stop modes"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(I2C1SMEN_A::Enabled)
    }
}
#[doc = "Field `I2C2SMEN` reader - I2C2 clocks enable during Sleep and Stop modes"]
pub use I2C1SMEN_R as I2C2SMEN_R;
#[doc = "Field `I2C3SMEN` reader - I2C3 clocks enable during Sleep and Stop modes"]
pub use I2C1SMEN_R as I2C3SMEN_R;
#[doc = "Field `I2C2SMEN` writer - I2C2 clocks enable during Sleep and Stop modes"]
pub use I2C1SMEN_W as I2C2SMEN_W;
#[doc = "Field `I2C3SMEN` writer - I2C3 clocks enable during Sleep and Stop modes"]
pub use I2C1SMEN_W as I2C3SMEN_W;
#[doc = "Field `CRSSMEN` reader - CRS clock enable during Sleep and Stop modes"]
pub type CRSSMEN_R = crate::BitReader<CRSSMEN_A>;
#[doc = "CRS clock enable during Sleep and Stop modes\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CRSSMEN_A {
    #[doc = "0: CRS clocks disabled by the clock gating during Sleep and Stop modes"]
    Disabled = 0,
    #[doc = "1: CRS clocks enabled by the clock gating(1) during Sleep and Stop modes"]
    Enabled = 1,
}
impl From<CRSSMEN_A> for bool {
    #[inline(always)]
    fn from(variant: CRSSMEN_A) -> Self {
        variant as u8 != 0
    }
}
impl CRSSMEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CRSSMEN_A {
        match self.bits {
            false => CRSSMEN_A::Disabled,
            true => CRSSMEN_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CRSSMEN_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CRSSMEN_A::Enabled
    }
}
#[doc = "Field `CRSSMEN` writer - CRS clock enable during Sleep and Stop modes"]
pub type CRSSMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1SMENR1_SPEC, CRSSMEN_A, O>;
impl<'a, const O: u8> CRSSMEN_W<'a, O> {
    #[doc = "CRS clocks disabled by the clock gating during Sleep and Stop modes"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CRSSMEN_A::Disabled)
    }
    #[doc = "CRS clocks enabled by the clock gating(1) during Sleep and Stop modes"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CRSSMEN_A::Enabled)
    }
}
#[doc = "Field `CAN1SMEN` reader - CAN1 clocks enable during Sleep and Stop modes"]
pub type CAN1SMEN_R = crate::BitReader<CAN1SMEN_A>;
#[doc = "CAN1 clocks enable during Sleep and Stop modes\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CAN1SMEN_A {
    #[doc = "0: CAN1 clocks disabled by the clock gating during Sleep and Stop modes"]
    Disabled = 0,
    #[doc = "1: CAN1 clocks enabled by the clock gating(1) during Sleep and Stop modes"]
    Enabled = 1,
}
impl From<CAN1SMEN_A> for bool {
    #[inline(always)]
    fn from(variant: CAN1SMEN_A) -> Self {
        variant as u8 != 0
    }
}
impl CAN1SMEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CAN1SMEN_A {
        match self.bits {
            false => CAN1SMEN_A::Disabled,
            true => CAN1SMEN_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CAN1SMEN_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CAN1SMEN_A::Enabled
    }
}
#[doc = "Field `CAN1SMEN` writer - CAN1 clocks enable during Sleep and Stop modes"]
pub type CAN1SMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1SMENR1_SPEC, CAN1SMEN_A, O>;
impl<'a, const O: u8> CAN1SMEN_W<'a, O> {
    #[doc = "CAN1 clocks disabled by the clock gating during Sleep and Stop modes"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CAN1SMEN_A::Disabled)
    }
    #[doc = "CAN1 clocks enabled by the clock gating(1) during Sleep and Stop modes"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CAN1SMEN_A::Enabled)
    }
}
#[doc = "Field `PWRSMEN` reader - Power interface clocks enable during Sleep and Stop modes"]
pub type PWRSMEN_R = crate::BitReader<PWRSMEN_A>;
#[doc = "Power interface clocks enable during Sleep and Stop modes\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PWRSMEN_A {
    #[doc = "0: Power interface clocks disabled by the clock gating during Sleep and Stop modes"]
    Disabled = 0,
    #[doc = "1: Power interface clocks enabled by the clock gating(1) during Sleep and Stop modes"]
    Enabled = 1,
}
impl From<PWRSMEN_A> for bool {
    #[inline(always)]
    fn from(variant: PWRSMEN_A) -> Self {
        variant as u8 != 0
    }
}
impl PWRSMEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PWRSMEN_A {
        match self.bits {
            false => PWRSMEN_A::Disabled,
            true => PWRSMEN_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PWRSMEN_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PWRSMEN_A::Enabled
    }
}
#[doc = "Field `PWRSMEN` writer - Power interface clocks enable during Sleep and Stop modes"]
pub type PWRSMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1SMENR1_SPEC, PWRSMEN_A, O>;
impl<'a, const O: u8> PWRSMEN_W<'a, O> {
    #[doc = "Power interface clocks disabled by the clock gating during Sleep and Stop modes"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PWRSMEN_A::Disabled)
    }
    #[doc = "Power interface clocks enabled by the clock gating(1) during Sleep and Stop modes"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PWRSMEN_A::Enabled)
    }
}
#[doc = "Field `DAC1SMEN` reader - DAC1 interface clocks enable during Sleep and Stop modes"]
pub type DAC1SMEN_R = crate::BitReader<DAC1SMEN_A>;
#[doc = "DAC1 interface clocks enable during Sleep and Stop modes\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DAC1SMEN_A {
    #[doc = "0: DAC1 interface clocks disabled by the clock gating during Sleep and Stop modes"]
    Disabled = 0,
    #[doc = "1: DAC1 interface clocks enabled by the clock gating(1) during Sleep and Stop modes"]
    Enabled = 1,
}
impl From<DAC1SMEN_A> for bool {
    #[inline(always)]
    fn from(variant: DAC1SMEN_A) -> Self {
        variant as u8 != 0
    }
}
impl DAC1SMEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DAC1SMEN_A {
        match self.bits {
            false => DAC1SMEN_A::Disabled,
            true => DAC1SMEN_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DAC1SMEN_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DAC1SMEN_A::Enabled
    }
}
#[doc = "Field `DAC1SMEN` writer - DAC1 interface clocks enable during Sleep and Stop modes"]
pub type DAC1SMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1SMENR1_SPEC, DAC1SMEN_A, O>;
impl<'a, const O: u8> DAC1SMEN_W<'a, O> {
    #[doc = "DAC1 interface clocks disabled by the clock gating during Sleep and Stop modes"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DAC1SMEN_A::Disabled)
    }
    #[doc = "DAC1 interface clocks enabled by the clock gating(1) during Sleep and Stop modes"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DAC1SMEN_A::Enabled)
    }
}
#[doc = "Field `OPAMPSMEN` reader - OPAMP interface clocks enable during Sleep and Stop modes"]
pub type OPAMPSMEN_R = crate::BitReader<OPAMPSMEN_A>;
#[doc = "OPAMP interface clocks enable during Sleep and Stop modes\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OPAMPSMEN_A {
    #[doc = "0: OPAMP interface clocks disabled by the clock gating during Sleep and Stop modes"]
    Disabled = 0,
    #[doc = "1: OPAMP interface clocks enabled by the clock gating(1) during Sleep and Stop modes"]
    Enabled = 1,
}
impl From<OPAMPSMEN_A> for bool {
    #[inline(always)]
    fn from(variant: OPAMPSMEN_A) -> Self {
        variant as u8 != 0
    }
}
impl OPAMPSMEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OPAMPSMEN_A {
        match self.bits {
            false => OPAMPSMEN_A::Disabled,
            true => OPAMPSMEN_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == OPAMPSMEN_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == OPAMPSMEN_A::Enabled
    }
}
#[doc = "Field `OPAMPSMEN` writer - OPAMP interface clocks enable during Sleep and Stop modes"]
pub type OPAMPSMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1SMENR1_SPEC, OPAMPSMEN_A, O>;
impl<'a, const O: u8> OPAMPSMEN_W<'a, O> {
    #[doc = "OPAMP interface clocks disabled by the clock gating during Sleep and Stop modes"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(OPAMPSMEN_A::Disabled)
    }
    #[doc = "OPAMP interface clocks enabled by the clock gating(1) during Sleep and Stop modes"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(OPAMPSMEN_A::Enabled)
    }
}
#[doc = "Field `LPTIM1SMEN` reader - Low power timer 1 clocks enable during Sleep and Stop modes"]
pub type LPTIM1SMEN_R = crate::BitReader<LPTIM1SMEN_A>;
#[doc = "Low power timer 1 clocks enable during Sleep and Stop modes\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LPTIM1SMEN_A {
    #[doc = "0: LPTIM1 clocks disabled by the clock gating during Sleep and Stop modes"]
    Disabled = 0,
    #[doc = "1: LPTIM1 clocks enabled by the clock gating(1) during Sleep and Stop modes"]
    Enabled = 1,
}
impl From<LPTIM1SMEN_A> for bool {
    #[inline(always)]
    fn from(variant: LPTIM1SMEN_A) -> Self {
        variant as u8 != 0
    }
}
impl LPTIM1SMEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPTIM1SMEN_A {
        match self.bits {
            false => LPTIM1SMEN_A::Disabled,
            true => LPTIM1SMEN_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == LPTIM1SMEN_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == LPTIM1SMEN_A::Enabled
    }
}
#[doc = "Field `LPTIM1SMEN` writer - Low power timer 1 clocks enable during Sleep and Stop modes"]
pub type LPTIM1SMEN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, APB1SMENR1_SPEC, LPTIM1SMEN_A, O>;
impl<'a, const O: u8> LPTIM1SMEN_W<'a, O> {
    #[doc = "LPTIM1 clocks disabled by the clock gating during Sleep and Stop modes"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(LPTIM1SMEN_A::Disabled)
    }
    #[doc = "LPTIM1 clocks enabled by the clock gating(1) during Sleep and Stop modes"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(LPTIM1SMEN_A::Enabled)
    }
}
impl R {
    #[doc = "Bit 0 - TIM2 timer clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn tim2smen(&self) -> TIM2SMEN_R {
        TIM2SMEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TIM3 timer clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn tim3smen(&self) -> TIM3SMEN_R {
        TIM3SMEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - TIM4 timer clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn tim4smen(&self) -> TIM4SMEN_R {
        TIM4SMEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - TIM5 timer clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn tim5smen(&self) -> TIM5SMEN_R {
        TIM5SMEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - TIM6 timer clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn tim6smen(&self) -> TIM6SMEN_R {
        TIM6SMEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - TIM7 timer clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn tim7smen(&self) -> TIM7SMEN_R {
        TIM7SMEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 10 - RTC APB clock enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn rtcapbsmen(&self) -> RTCAPBSMEN_R {
        RTCAPBSMEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Window watchdog clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn wwdgsmen(&self) -> WWDGSMEN_R {
        WWDGSMEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 14 - SPI2 clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn spi2smen(&self) -> SPI2SMEN_R {
        SPI2SMEN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - SPI3 clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn sp3smen(&self) -> SP3SMEN_R {
        SP3SMEN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 17 - USART2 clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn usart2smen(&self) -> USART2SMEN_R {
        USART2SMEN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - USART3 clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn usart3smen(&self) -> USART3SMEN_R {
        USART3SMEN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - UART4 clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn uart4smen(&self) -> UART4SMEN_R {
        UART4SMEN_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - UART5 clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn uart5smen(&self) -> UART5SMEN_R {
        UART5SMEN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - I2C1 clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn i2c1smen(&self) -> I2C1SMEN_R {
        I2C1SMEN_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - I2C2 clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn i2c2smen(&self) -> I2C2SMEN_R {
        I2C2SMEN_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - I2C3 clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn i2c3smen(&self) -> I2C3SMEN_R {
        I2C3SMEN_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - CRS clock enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn crssmen(&self) -> CRSSMEN_R {
        CRSSMEN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - CAN1 clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn can1smen(&self) -> CAN1SMEN_R {
        CAN1SMEN_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 28 - Power interface clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn pwrsmen(&self) -> PWRSMEN_R {
        PWRSMEN_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - DAC1 interface clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn dac1smen(&self) -> DAC1SMEN_R {
        DAC1SMEN_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - OPAMP interface clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn opampsmen(&self) -> OPAMPSMEN_R {
        OPAMPSMEN_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Low power timer 1 clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn lptim1smen(&self) -> LPTIM1SMEN_R {
        LPTIM1SMEN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TIM2 timer clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    #[must_use]
    pub fn tim2smen(&mut self) -> TIM2SMEN_W<0> {
        TIM2SMEN_W::new(self)
    }
    #[doc = "Bit 1 - TIM3 timer clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    #[must_use]
    pub fn tim3smen(&mut self) -> TIM3SMEN_W<1> {
        TIM3SMEN_W::new(self)
    }
    #[doc = "Bit 2 - TIM4 timer clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    #[must_use]
    pub fn tim4smen(&mut self) -> TIM4SMEN_W<2> {
        TIM4SMEN_W::new(self)
    }
    #[doc = "Bit 3 - TIM5 timer clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    #[must_use]
    pub fn tim5smen(&mut self) -> TIM5SMEN_W<3> {
        TIM5SMEN_W::new(self)
    }
    #[doc = "Bit 4 - TIM6 timer clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    #[must_use]
    pub fn tim6smen(&mut self) -> TIM6SMEN_W<4> {
        TIM6SMEN_W::new(self)
    }
    #[doc = "Bit 5 - TIM7 timer clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    #[must_use]
    pub fn tim7smen(&mut self) -> TIM7SMEN_W<5> {
        TIM7SMEN_W::new(self)
    }
    #[doc = "Bit 10 - RTC APB clock enable during Sleep and Stop modes"]
    #[inline(always)]
    #[must_use]
    pub fn rtcapbsmen(&mut self) -> RTCAPBSMEN_W<10> {
        RTCAPBSMEN_W::new(self)
    }
    #[doc = "Bit 11 - Window watchdog clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    #[must_use]
    pub fn wwdgsmen(&mut self) -> WWDGSMEN_W<11> {
        WWDGSMEN_W::new(self)
    }
    #[doc = "Bit 14 - SPI2 clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    #[must_use]
    pub fn spi2smen(&mut self) -> SPI2SMEN_W<14> {
        SPI2SMEN_W::new(self)
    }
    #[doc = "Bit 15 - SPI3 clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    #[must_use]
    pub fn sp3smen(&mut self) -> SP3SMEN_W<15> {
        SP3SMEN_W::new(self)
    }
    #[doc = "Bit 17 - USART2 clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    #[must_use]
    pub fn usart2smen(&mut self) -> USART2SMEN_W<17> {
        USART2SMEN_W::new(self)
    }
    #[doc = "Bit 18 - USART3 clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    #[must_use]
    pub fn usart3smen(&mut self) -> USART3SMEN_W<18> {
        USART3SMEN_W::new(self)
    }
    #[doc = "Bit 19 - UART4 clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    #[must_use]
    pub fn uart4smen(&mut self) -> UART4SMEN_W<19> {
        UART4SMEN_W::new(self)
    }
    #[doc = "Bit 20 - UART5 clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    #[must_use]
    pub fn uart5smen(&mut self) -> UART5SMEN_W<20> {
        UART5SMEN_W::new(self)
    }
    #[doc = "Bit 21 - I2C1 clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    #[must_use]
    pub fn i2c1smen(&mut self) -> I2C1SMEN_W<21> {
        I2C1SMEN_W::new(self)
    }
    #[doc = "Bit 22 - I2C2 clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    #[must_use]
    pub fn i2c2smen(&mut self) -> I2C2SMEN_W<22> {
        I2C2SMEN_W::new(self)
    }
    #[doc = "Bit 23 - I2C3 clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    #[must_use]
    pub fn i2c3smen(&mut self) -> I2C3SMEN_W<23> {
        I2C3SMEN_W::new(self)
    }
    #[doc = "Bit 24 - CRS clock enable during Sleep and Stop modes"]
    #[inline(always)]
    #[must_use]
    pub fn crssmen(&mut self) -> CRSSMEN_W<24> {
        CRSSMEN_W::new(self)
    }
    #[doc = "Bit 25 - CAN1 clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    #[must_use]
    pub fn can1smen(&mut self) -> CAN1SMEN_W<25> {
        CAN1SMEN_W::new(self)
    }
    #[doc = "Bit 28 - Power interface clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    #[must_use]
    pub fn pwrsmen(&mut self) -> PWRSMEN_W<28> {
        PWRSMEN_W::new(self)
    }
    #[doc = "Bit 29 - DAC1 interface clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    #[must_use]
    pub fn dac1smen(&mut self) -> DAC1SMEN_W<29> {
        DAC1SMEN_W::new(self)
    }
    #[doc = "Bit 30 - OPAMP interface clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    #[must_use]
    pub fn opampsmen(&mut self) -> OPAMPSMEN_W<30> {
        OPAMPSMEN_W::new(self)
    }
    #[doc = "Bit 31 - Low power timer 1 clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    #[must_use]
    pub fn lptim1smen(&mut self) -> LPTIM1SMEN_W<31> {
        LPTIM1SMEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "APB1SMENR1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apb1smenr1](index.html) module"]
pub struct APB1SMENR1_SPEC;
impl crate::RegisterSpec for APB1SMENR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [apb1smenr1::R](R) reader structure"]
impl crate::Readable for APB1SMENR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [apb1smenr1::W](W) writer structure"]
impl crate::Writable for APB1SMENR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets APB1SMENR1 to value 0xf3fe_cc3f"]
impl crate::Resettable for APB1SMENR1_SPEC {
    const RESET_VALUE: Self::Ux = 0xf3fe_cc3f;
}
