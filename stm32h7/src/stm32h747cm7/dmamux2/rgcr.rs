#[doc = "Register `RGCR%s` reader"]
pub struct R(crate::R<RGCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RGCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RGCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RGCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RGCR%s` writer"]
pub struct W(crate::W<RGCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RGCR_SPEC>;
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
impl From<crate::W<RGCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RGCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SIG_ID` reader - DMA request trigger input selected"]
pub type SIG_ID_R = crate::FieldReader<u8, SIG_ID_A>;
#[doc = "DMA request trigger input selected\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SIG_ID_A {
    #[doc = "0: Signal `dmamux2_evt0` selected as trigger input"]
    Dmamux2Evt0 = 0,
    #[doc = "1: Signal `dmamux2_evt1` selected as trigger input"]
    Dmamux2Evt1 = 1,
    #[doc = "2: Signal `dmamux2_evt2` selected as trigger input"]
    Dmamux2Evt2 = 2,
    #[doc = "3: Signal `dmamux2_evt3` selected as trigger input"]
    Dmamux2Evt3 = 3,
    #[doc = "4: Signal `dmamux2_evt4` selected as trigger input"]
    Dmamux2Evt4 = 4,
    #[doc = "5: Signal `dmamux2_evt5` selected as trigger input"]
    Dmamux2Evt5 = 5,
    #[doc = "6: Signal `dmamux2_evt6` selected as trigger input"]
    Dmamux2Evt6 = 6,
    #[doc = "7: Signal `lpuart_rx_wkup` selected as trigger input"]
    LpuartRxWkup = 7,
    #[doc = "8: Signal `lpuart_tx_wkup` selected as trigger input"]
    LpuartTxWkup = 8,
    #[doc = "9: Signal `lptim2_wkup` selected as trigger input"]
    Lptim2Wkup = 9,
    #[doc = "10: Signal `lptim2_out` selected as trigger input"]
    Lptim2Out = 10,
    #[doc = "11: Signal `lptim3_wkup` selected as trigger input"]
    Lptim3Wkup = 11,
    #[doc = "12: Signal `lptim3_out` selected as trigger input"]
    Lptim3Out = 12,
    #[doc = "13: Signal `lptim4_ait` selected as trigger input"]
    Lptim4Ait = 13,
    #[doc = "14: Signal `lptim5_ait` selected as trigger input"]
    Lptim5Ait = 14,
    #[doc = "15: Signal `i2c4_wkup` selected as trigger input"]
    I2c4Wkup = 15,
    #[doc = "16: Signal `spi6_wkup` selected as trigger input"]
    Spi6Wkup = 16,
    #[doc = "17: Signal `comp1_out` selected as trigger input"]
    Comp1Out = 17,
    #[doc = "18: Signal `comp2_out` selected as trigger input"]
    Comp2Out = 18,
    #[doc = "19: Signal `rtc_wkup` selected as trigger input"]
    RtcWkup = 19,
    #[doc = "20: Signal `syscfg_exti0_mux` selected as trigger input"]
    SyscfgExti0Mux = 20,
    #[doc = "21: Signal `syscfg_exti2_mux` selected as trigger input"]
    SyscfgExti2Mux = 21,
    #[doc = "22: Signal `i2c4_event_it` selected as trigger input"]
    I2c4EventIt = 22,
    #[doc = "23: Signal `spi6_it` selected as trigger input"]
    Spi6It = 23,
    #[doc = "24: Signal `lpuart1_it_t` selected as trigger input"]
    Lpuart1ItT = 24,
    #[doc = "25: Signal `lpuart1_it_r` selected as trigger input"]
    Lpuart1ItR = 25,
    #[doc = "26: Signal `adc3_it` selected as trigger input"]
    Adc3It = 26,
    #[doc = "27: Signal `adc3_awd1` selected as trigger input"]
    Adc3Awd1 = 27,
    #[doc = "28: Signal `bdma_ch0_it` selected as trigger input"]
    BdmaCh0It = 28,
    #[doc = "29: Signal `bdma_ch1_it` selected as trigger input"]
    BdmaCh1It = 29,
}
impl From<SIG_ID_A> for u8 {
    #[inline(always)]
    fn from(variant: SIG_ID_A) -> Self {
        variant as _
    }
}
impl SIG_ID_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SIG_ID_A> {
        match self.bits {
            0 => Some(SIG_ID_A::Dmamux2Evt0),
            1 => Some(SIG_ID_A::Dmamux2Evt1),
            2 => Some(SIG_ID_A::Dmamux2Evt2),
            3 => Some(SIG_ID_A::Dmamux2Evt3),
            4 => Some(SIG_ID_A::Dmamux2Evt4),
            5 => Some(SIG_ID_A::Dmamux2Evt5),
            6 => Some(SIG_ID_A::Dmamux2Evt6),
            7 => Some(SIG_ID_A::LpuartRxWkup),
            8 => Some(SIG_ID_A::LpuartTxWkup),
            9 => Some(SIG_ID_A::Lptim2Wkup),
            10 => Some(SIG_ID_A::Lptim2Out),
            11 => Some(SIG_ID_A::Lptim3Wkup),
            12 => Some(SIG_ID_A::Lptim3Out),
            13 => Some(SIG_ID_A::Lptim4Ait),
            14 => Some(SIG_ID_A::Lptim5Ait),
            15 => Some(SIG_ID_A::I2c4Wkup),
            16 => Some(SIG_ID_A::Spi6Wkup),
            17 => Some(SIG_ID_A::Comp1Out),
            18 => Some(SIG_ID_A::Comp2Out),
            19 => Some(SIG_ID_A::RtcWkup),
            20 => Some(SIG_ID_A::SyscfgExti0Mux),
            21 => Some(SIG_ID_A::SyscfgExti2Mux),
            22 => Some(SIG_ID_A::I2c4EventIt),
            23 => Some(SIG_ID_A::Spi6It),
            24 => Some(SIG_ID_A::Lpuart1ItT),
            25 => Some(SIG_ID_A::Lpuart1ItR),
            26 => Some(SIG_ID_A::Adc3It),
            27 => Some(SIG_ID_A::Adc3Awd1),
            28 => Some(SIG_ID_A::BdmaCh0It),
            29 => Some(SIG_ID_A::BdmaCh1It),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `Dmamux2Evt0`"]
    #[inline(always)]
    pub fn is_dmamux2_evt0(&self) -> bool {
        *self == SIG_ID_A::Dmamux2Evt0
    }
    #[doc = "Checks if the value of the field is `Dmamux2Evt1`"]
    #[inline(always)]
    pub fn is_dmamux2_evt1(&self) -> bool {
        *self == SIG_ID_A::Dmamux2Evt1
    }
    #[doc = "Checks if the value of the field is `Dmamux2Evt2`"]
    #[inline(always)]
    pub fn is_dmamux2_evt2(&self) -> bool {
        *self == SIG_ID_A::Dmamux2Evt2
    }
    #[doc = "Checks if the value of the field is `Dmamux2Evt3`"]
    #[inline(always)]
    pub fn is_dmamux2_evt3(&self) -> bool {
        *self == SIG_ID_A::Dmamux2Evt3
    }
    #[doc = "Checks if the value of the field is `Dmamux2Evt4`"]
    #[inline(always)]
    pub fn is_dmamux2_evt4(&self) -> bool {
        *self == SIG_ID_A::Dmamux2Evt4
    }
    #[doc = "Checks if the value of the field is `Dmamux2Evt5`"]
    #[inline(always)]
    pub fn is_dmamux2_evt5(&self) -> bool {
        *self == SIG_ID_A::Dmamux2Evt5
    }
    #[doc = "Checks if the value of the field is `Dmamux2Evt6`"]
    #[inline(always)]
    pub fn is_dmamux2_evt6(&self) -> bool {
        *self == SIG_ID_A::Dmamux2Evt6
    }
    #[doc = "Checks if the value of the field is `LpuartRxWkup`"]
    #[inline(always)]
    pub fn is_lpuart_rx_wkup(&self) -> bool {
        *self == SIG_ID_A::LpuartRxWkup
    }
    #[doc = "Checks if the value of the field is `LpuartTxWkup`"]
    #[inline(always)]
    pub fn is_lpuart_tx_wkup(&self) -> bool {
        *self == SIG_ID_A::LpuartTxWkup
    }
    #[doc = "Checks if the value of the field is `Lptim2Wkup`"]
    #[inline(always)]
    pub fn is_lptim2_wkup(&self) -> bool {
        *self == SIG_ID_A::Lptim2Wkup
    }
    #[doc = "Checks if the value of the field is `Lptim2Out`"]
    #[inline(always)]
    pub fn is_lptim2_out(&self) -> bool {
        *self == SIG_ID_A::Lptim2Out
    }
    #[doc = "Checks if the value of the field is `Lptim3Wkup`"]
    #[inline(always)]
    pub fn is_lptim3_wkup(&self) -> bool {
        *self == SIG_ID_A::Lptim3Wkup
    }
    #[doc = "Checks if the value of the field is `Lptim3Out`"]
    #[inline(always)]
    pub fn is_lptim3_out(&self) -> bool {
        *self == SIG_ID_A::Lptim3Out
    }
    #[doc = "Checks if the value of the field is `Lptim4Ait`"]
    #[inline(always)]
    pub fn is_lptim4_ait(&self) -> bool {
        *self == SIG_ID_A::Lptim4Ait
    }
    #[doc = "Checks if the value of the field is `Lptim5Ait`"]
    #[inline(always)]
    pub fn is_lptim5_ait(&self) -> bool {
        *self == SIG_ID_A::Lptim5Ait
    }
    #[doc = "Checks if the value of the field is `I2c4Wkup`"]
    #[inline(always)]
    pub fn is_i2c4_wkup(&self) -> bool {
        *self == SIG_ID_A::I2c4Wkup
    }
    #[doc = "Checks if the value of the field is `Spi6Wkup`"]
    #[inline(always)]
    pub fn is_spi6_wkup(&self) -> bool {
        *self == SIG_ID_A::Spi6Wkup
    }
    #[doc = "Checks if the value of the field is `Comp1Out`"]
    #[inline(always)]
    pub fn is_comp1_out(&self) -> bool {
        *self == SIG_ID_A::Comp1Out
    }
    #[doc = "Checks if the value of the field is `Comp2Out`"]
    #[inline(always)]
    pub fn is_comp2_out(&self) -> bool {
        *self == SIG_ID_A::Comp2Out
    }
    #[doc = "Checks if the value of the field is `RtcWkup`"]
    #[inline(always)]
    pub fn is_rtc_wkup(&self) -> bool {
        *self == SIG_ID_A::RtcWkup
    }
    #[doc = "Checks if the value of the field is `SyscfgExti0Mux`"]
    #[inline(always)]
    pub fn is_syscfg_exti0_mux(&self) -> bool {
        *self == SIG_ID_A::SyscfgExti0Mux
    }
    #[doc = "Checks if the value of the field is `SyscfgExti2Mux`"]
    #[inline(always)]
    pub fn is_syscfg_exti2_mux(&self) -> bool {
        *self == SIG_ID_A::SyscfgExti2Mux
    }
    #[doc = "Checks if the value of the field is `I2c4EventIt`"]
    #[inline(always)]
    pub fn is_i2c4_event_it(&self) -> bool {
        *self == SIG_ID_A::I2c4EventIt
    }
    #[doc = "Checks if the value of the field is `Spi6It`"]
    #[inline(always)]
    pub fn is_spi6_it(&self) -> bool {
        *self == SIG_ID_A::Spi6It
    }
    #[doc = "Checks if the value of the field is `Lpuart1ItT`"]
    #[inline(always)]
    pub fn is_lpuart1_it_t(&self) -> bool {
        *self == SIG_ID_A::Lpuart1ItT
    }
    #[doc = "Checks if the value of the field is `Lpuart1ItR`"]
    #[inline(always)]
    pub fn is_lpuart1_it_r(&self) -> bool {
        *self == SIG_ID_A::Lpuart1ItR
    }
    #[doc = "Checks if the value of the field is `Adc3It`"]
    #[inline(always)]
    pub fn is_adc3_it(&self) -> bool {
        *self == SIG_ID_A::Adc3It
    }
    #[doc = "Checks if the value of the field is `Adc3Awd1`"]
    #[inline(always)]
    pub fn is_adc3_awd1(&self) -> bool {
        *self == SIG_ID_A::Adc3Awd1
    }
    #[doc = "Checks if the value of the field is `BdmaCh0It`"]
    #[inline(always)]
    pub fn is_bdma_ch0_it(&self) -> bool {
        *self == SIG_ID_A::BdmaCh0It
    }
    #[doc = "Checks if the value of the field is `BdmaCh1It`"]
    #[inline(always)]
    pub fn is_bdma_ch1_it(&self) -> bool {
        *self == SIG_ID_A::BdmaCh1It
    }
}
#[doc = "Field `SIG_ID` writer - DMA request trigger input selected"]
pub type SIG_ID_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RGCR_SPEC, u8, SIG_ID_A, 5, O>;
impl<'a, const O: u8> SIG_ID_W<'a, O> {
    #[doc = "Signal `dmamux2_evt0` selected as trigger input"]
    #[inline(always)]
    pub fn dmamux2_evt0(self) -> &'a mut W {
        self.variant(SIG_ID_A::Dmamux2Evt0)
    }
    #[doc = "Signal `dmamux2_evt1` selected as trigger input"]
    #[inline(always)]
    pub fn dmamux2_evt1(self) -> &'a mut W {
        self.variant(SIG_ID_A::Dmamux2Evt1)
    }
    #[doc = "Signal `dmamux2_evt2` selected as trigger input"]
    #[inline(always)]
    pub fn dmamux2_evt2(self) -> &'a mut W {
        self.variant(SIG_ID_A::Dmamux2Evt2)
    }
    #[doc = "Signal `dmamux2_evt3` selected as trigger input"]
    #[inline(always)]
    pub fn dmamux2_evt3(self) -> &'a mut W {
        self.variant(SIG_ID_A::Dmamux2Evt3)
    }
    #[doc = "Signal `dmamux2_evt4` selected as trigger input"]
    #[inline(always)]
    pub fn dmamux2_evt4(self) -> &'a mut W {
        self.variant(SIG_ID_A::Dmamux2Evt4)
    }
    #[doc = "Signal `dmamux2_evt5` selected as trigger input"]
    #[inline(always)]
    pub fn dmamux2_evt5(self) -> &'a mut W {
        self.variant(SIG_ID_A::Dmamux2Evt5)
    }
    #[doc = "Signal `dmamux2_evt6` selected as trigger input"]
    #[inline(always)]
    pub fn dmamux2_evt6(self) -> &'a mut W {
        self.variant(SIG_ID_A::Dmamux2Evt6)
    }
    #[doc = "Signal `lpuart_rx_wkup` selected as trigger input"]
    #[inline(always)]
    pub fn lpuart_rx_wkup(self) -> &'a mut W {
        self.variant(SIG_ID_A::LpuartRxWkup)
    }
    #[doc = "Signal `lpuart_tx_wkup` selected as trigger input"]
    #[inline(always)]
    pub fn lpuart_tx_wkup(self) -> &'a mut W {
        self.variant(SIG_ID_A::LpuartTxWkup)
    }
    #[doc = "Signal `lptim2_wkup` selected as trigger input"]
    #[inline(always)]
    pub fn lptim2_wkup(self) -> &'a mut W {
        self.variant(SIG_ID_A::Lptim2Wkup)
    }
    #[doc = "Signal `lptim2_out` selected as trigger input"]
    #[inline(always)]
    pub fn lptim2_out(self) -> &'a mut W {
        self.variant(SIG_ID_A::Lptim2Out)
    }
    #[doc = "Signal `lptim3_wkup` selected as trigger input"]
    #[inline(always)]
    pub fn lptim3_wkup(self) -> &'a mut W {
        self.variant(SIG_ID_A::Lptim3Wkup)
    }
    #[doc = "Signal `lptim3_out` selected as trigger input"]
    #[inline(always)]
    pub fn lptim3_out(self) -> &'a mut W {
        self.variant(SIG_ID_A::Lptim3Out)
    }
    #[doc = "Signal `lptim4_ait` selected as trigger input"]
    #[inline(always)]
    pub fn lptim4_ait(self) -> &'a mut W {
        self.variant(SIG_ID_A::Lptim4Ait)
    }
    #[doc = "Signal `lptim5_ait` selected as trigger input"]
    #[inline(always)]
    pub fn lptim5_ait(self) -> &'a mut W {
        self.variant(SIG_ID_A::Lptim5Ait)
    }
    #[doc = "Signal `i2c4_wkup` selected as trigger input"]
    #[inline(always)]
    pub fn i2c4_wkup(self) -> &'a mut W {
        self.variant(SIG_ID_A::I2c4Wkup)
    }
    #[doc = "Signal `spi6_wkup` selected as trigger input"]
    #[inline(always)]
    pub fn spi6_wkup(self) -> &'a mut W {
        self.variant(SIG_ID_A::Spi6Wkup)
    }
    #[doc = "Signal `comp1_out` selected as trigger input"]
    #[inline(always)]
    pub fn comp1_out(self) -> &'a mut W {
        self.variant(SIG_ID_A::Comp1Out)
    }
    #[doc = "Signal `comp2_out` selected as trigger input"]
    #[inline(always)]
    pub fn comp2_out(self) -> &'a mut W {
        self.variant(SIG_ID_A::Comp2Out)
    }
    #[doc = "Signal `rtc_wkup` selected as trigger input"]
    #[inline(always)]
    pub fn rtc_wkup(self) -> &'a mut W {
        self.variant(SIG_ID_A::RtcWkup)
    }
    #[doc = "Signal `syscfg_exti0_mux` selected as trigger input"]
    #[inline(always)]
    pub fn syscfg_exti0_mux(self) -> &'a mut W {
        self.variant(SIG_ID_A::SyscfgExti0Mux)
    }
    #[doc = "Signal `syscfg_exti2_mux` selected as trigger input"]
    #[inline(always)]
    pub fn syscfg_exti2_mux(self) -> &'a mut W {
        self.variant(SIG_ID_A::SyscfgExti2Mux)
    }
    #[doc = "Signal `i2c4_event_it` selected as trigger input"]
    #[inline(always)]
    pub fn i2c4_event_it(self) -> &'a mut W {
        self.variant(SIG_ID_A::I2c4EventIt)
    }
    #[doc = "Signal `spi6_it` selected as trigger input"]
    #[inline(always)]
    pub fn spi6_it(self) -> &'a mut W {
        self.variant(SIG_ID_A::Spi6It)
    }
    #[doc = "Signal `lpuart1_it_t` selected as trigger input"]
    #[inline(always)]
    pub fn lpuart1_it_t(self) -> &'a mut W {
        self.variant(SIG_ID_A::Lpuart1ItT)
    }
    #[doc = "Signal `lpuart1_it_r` selected as trigger input"]
    #[inline(always)]
    pub fn lpuart1_it_r(self) -> &'a mut W {
        self.variant(SIG_ID_A::Lpuart1ItR)
    }
    #[doc = "Signal `adc3_it` selected as trigger input"]
    #[inline(always)]
    pub fn adc3_it(self) -> &'a mut W {
        self.variant(SIG_ID_A::Adc3It)
    }
    #[doc = "Signal `adc3_awd1` selected as trigger input"]
    #[inline(always)]
    pub fn adc3_awd1(self) -> &'a mut W {
        self.variant(SIG_ID_A::Adc3Awd1)
    }
    #[doc = "Signal `bdma_ch0_it` selected as trigger input"]
    #[inline(always)]
    pub fn bdma_ch0_it(self) -> &'a mut W {
        self.variant(SIG_ID_A::BdmaCh0It)
    }
    #[doc = "Signal `bdma_ch1_it` selected as trigger input"]
    #[inline(always)]
    pub fn bdma_ch1_it(self) -> &'a mut W {
        self.variant(SIG_ID_A::BdmaCh1It)
    }
}
#[doc = "Field `OIE` reader - Interrupt enable at trigger event overrun"]
pub type OIE_R = crate::BitReader<OIE_A>;
#[doc = "Interrupt enable at trigger event overrun\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OIE_A {
    #[doc = "0: Trigger overrun interrupt disabled"]
    Disabled = 0,
    #[doc = "1: Trigger overrun interrupt enabled"]
    Enabled = 1,
}
impl From<OIE_A> for bool {
    #[inline(always)]
    fn from(variant: OIE_A) -> Self {
        variant as u8 != 0
    }
}
impl OIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OIE_A {
        match self.bits {
            false => OIE_A::Disabled,
            true => OIE_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == OIE_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == OIE_A::Enabled
    }
}
#[doc = "Field `OIE` writer - Interrupt enable at trigger event overrun"]
pub type OIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, RGCR_SPEC, OIE_A, O>;
impl<'a, const O: u8> OIE_W<'a, O> {
    #[doc = "Trigger overrun interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(OIE_A::Disabled)
    }
    #[doc = "Trigger overrun interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(OIE_A::Enabled)
    }
}
#[doc = "Field `GE` reader - DMA request generator channel enable/disable"]
pub type GE_R = crate::BitReader<GE_A>;
#[doc = "DMA request generator channel enable/disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GE_A {
    #[doc = "0: DMA request generation disabled"]
    Disabled = 0,
    #[doc = "1: DMA request enabled"]
    Enabled = 1,
}
impl From<GE_A> for bool {
    #[inline(always)]
    fn from(variant: GE_A) -> Self {
        variant as u8 != 0
    }
}
impl GE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GE_A {
        match self.bits {
            false => GE_A::Disabled,
            true => GE_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == GE_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == GE_A::Enabled
    }
}
#[doc = "Field `GE` writer - DMA request generator channel enable/disable"]
pub type GE_W<'a, const O: u8> = crate::BitWriter<'a, u32, RGCR_SPEC, GE_A, O>;
impl<'a, const O: u8> GE_W<'a, O> {
    #[doc = "DMA request generation disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(GE_A::Disabled)
    }
    #[doc = "DMA request enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(GE_A::Enabled)
    }
}
#[doc = "Field `GPOL` reader - DMA request generator trigger event type selection Defines the trigger event on the selected DMA request trigger input"]
pub type GPOL_R = crate::FieldReader<u8, GPOL_A>;
#[doc = "DMA request generator trigger event type selection Defines the trigger event on the selected DMA request trigger input\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum GPOL_A {
    #[doc = "0: No event, i.e. no detection nor generation"]
    NoEdge = 0,
    #[doc = "1: Rising edge"]
    RisingEdge = 1,
    #[doc = "2: Falling edge"]
    FallingEdge = 2,
    #[doc = "3: Rising and falling edges"]
    BothEdges = 3,
}
impl From<GPOL_A> for u8 {
    #[inline(always)]
    fn from(variant: GPOL_A) -> Self {
        variant as _
    }
}
impl GPOL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPOL_A {
        match self.bits {
            0 => GPOL_A::NoEdge,
            1 => GPOL_A::RisingEdge,
            2 => GPOL_A::FallingEdge,
            3 => GPOL_A::BothEdges,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NoEdge`"]
    #[inline(always)]
    pub fn is_no_edge(&self) -> bool {
        *self == GPOL_A::NoEdge
    }
    #[doc = "Checks if the value of the field is `RisingEdge`"]
    #[inline(always)]
    pub fn is_rising_edge(&self) -> bool {
        *self == GPOL_A::RisingEdge
    }
    #[doc = "Checks if the value of the field is `FallingEdge`"]
    #[inline(always)]
    pub fn is_falling_edge(&self) -> bool {
        *self == GPOL_A::FallingEdge
    }
    #[doc = "Checks if the value of the field is `BothEdges`"]
    #[inline(always)]
    pub fn is_both_edges(&self) -> bool {
        *self == GPOL_A::BothEdges
    }
}
#[doc = "Field `GPOL` writer - DMA request generator trigger event type selection Defines the trigger event on the selected DMA request trigger input"]
pub type GPOL_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, RGCR_SPEC, u8, GPOL_A, 2, O>;
impl<'a, const O: u8> GPOL_W<'a, O> {
    #[doc = "No event, i.e. no detection nor generation"]
    #[inline(always)]
    pub fn no_edge(self) -> &'a mut W {
        self.variant(GPOL_A::NoEdge)
    }
    #[doc = "Rising edge"]
    #[inline(always)]
    pub fn rising_edge(self) -> &'a mut W {
        self.variant(GPOL_A::RisingEdge)
    }
    #[doc = "Falling edge"]
    #[inline(always)]
    pub fn falling_edge(self) -> &'a mut W {
        self.variant(GPOL_A::FallingEdge)
    }
    #[doc = "Rising and falling edges"]
    #[inline(always)]
    pub fn both_edges(self) -> &'a mut W {
        self.variant(GPOL_A::BothEdges)
    }
}
#[doc = "Field `GNBREQ` reader - Number of DMA requests to generate Defines the number of DMA requests generated after a trigger event, then stop generating. The actual number of generated DMA requests is GNBREQ+1. Note: This field can only be written when GE bit is reset."]
pub type GNBREQ_R = crate::FieldReader<u8, u8>;
#[doc = "Field `GNBREQ` writer - Number of DMA requests to generate Defines the number of DMA requests generated after a trigger event, then stop generating. The actual number of generated DMA requests is GNBREQ+1. Note: This field can only be written when GE bit is reset."]
pub type GNBREQ_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, RGCR_SPEC, u8, u8, 5, O>;
impl R {
    #[doc = "Bits 0:4 - DMA request trigger input selected"]
    #[inline(always)]
    pub fn sig_id(&self) -> SIG_ID_R {
        SIG_ID_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 8 - Interrupt enable at trigger event overrun"]
    #[inline(always)]
    pub fn oie(&self) -> OIE_R {
        OIE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 16 - DMA request generator channel enable/disable"]
    #[inline(always)]
    pub fn ge(&self) -> GE_R {
        GE_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:18 - DMA request generator trigger event type selection Defines the trigger event on the selected DMA request trigger input"]
    #[inline(always)]
    pub fn gpol(&self) -> GPOL_R {
        GPOL_R::new(((self.bits >> 17) & 3) as u8)
    }
    #[doc = "Bits 19:23 - Number of DMA requests to generate Defines the number of DMA requests generated after a trigger event, then stop generating. The actual number of generated DMA requests is GNBREQ+1. Note: This field can only be written when GE bit is reset."]
    #[inline(always)]
    pub fn gnbreq(&self) -> GNBREQ_R {
        GNBREQ_R::new(((self.bits >> 19) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - DMA request trigger input selected"]
    #[inline(always)]
    #[must_use]
    pub fn sig_id(&mut self) -> SIG_ID_W<0> {
        SIG_ID_W::new(self)
    }
    #[doc = "Bit 8 - Interrupt enable at trigger event overrun"]
    #[inline(always)]
    #[must_use]
    pub fn oie(&mut self) -> OIE_W<8> {
        OIE_W::new(self)
    }
    #[doc = "Bit 16 - DMA request generator channel enable/disable"]
    #[inline(always)]
    #[must_use]
    pub fn ge(&mut self) -> GE_W<16> {
        GE_W::new(self)
    }
    #[doc = "Bits 17:18 - DMA request generator trigger event type selection Defines the trigger event on the selected DMA request trigger input"]
    #[inline(always)]
    #[must_use]
    pub fn gpol(&mut self) -> GPOL_W<17> {
        GPOL_W::new(self)
    }
    #[doc = "Bits 19:23 - Number of DMA requests to generate Defines the number of DMA requests generated after a trigger event, then stop generating. The actual number of generated DMA requests is GNBREQ+1. Note: This field can only be written when GE bit is reset."]
    #[inline(always)]
    #[must_use]
    pub fn gnbreq(&mut self) -> GNBREQ_W<19> {
        GNBREQ_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMAMux - DMA request generator channel x control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rgcr](index.html) module"]
pub struct RGCR_SPEC;
impl crate::RegisterSpec for RGCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rgcr::R](R) reader structure"]
impl crate::Readable for RGCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rgcr::W](W) writer structure"]
impl crate::Writable for RGCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RGCR%s to value 0"]
impl crate::Resettable for RGCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
