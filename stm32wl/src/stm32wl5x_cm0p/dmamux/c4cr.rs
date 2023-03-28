#[doc = "Register `C4CR` reader"]
pub struct R(crate::R<C4CR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<C4CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<C4CR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<C4CR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `C4CR` writer"]
pub struct W(crate::W<C4CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<C4CR_SPEC>;
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
impl From<crate::W<C4CR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<C4CR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DMAREQ_ID` reader - DMA request identification"]
pub type DMAREQ_ID_R = crate::FieldReader<u8, DMAREQ_ID_A>;
#[doc = "DMA request identification\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DMAREQ_ID_A {
    #[doc = "0: No signal selected as request input"]
    None = 0,
    #[doc = "1: Signal `dmamux1_req_gen0` selected as request input"]
    Dmamux1ReqGen0 = 1,
    #[doc = "2: Signal `dmamux1_req_gen1` selected as request input"]
    Dmamux1ReqGen1 = 2,
    #[doc = "3: Signal `dmamux1_req_gen2` selected as request input"]
    Dmamux1ReqGen2 = 3,
    #[doc = "4: Signal `dmamux1_req_gen3` selected as request input"]
    Dmamux1ReqGen3 = 4,
    #[doc = "5: Signal `adc1_dma` selected as request input"]
    Adc = 5,
    #[doc = "6: Signal `dac_out1_dma` selected as request input"]
    DatOut1 = 6,
    #[doc = "7: Signal `spi1_rx_dma` selected as request input"]
    Spi1RxDma = 7,
    #[doc = "8: Signal `spi1_tx_dma` selected as request input"]
    Spi1TxDma = 8,
    #[doc = "9: Signal `spi2_rx_dma` selected as request input"]
    Spi2RxDma = 9,
    #[doc = "10: Signal `spi2_tx_dma` selected as request input"]
    Spi2TxDma = 10,
    #[doc = "11: Signal `i2c1_rx_dma` selected as request input"]
    I2c1RxDma = 11,
    #[doc = "12: Signal `i2c1_tx_dma` selected as request input"]
    I2c1TxDma = 12,
    #[doc = "13: Signal `i2c2_rx_dma` selected as request input"]
    I2c2RxDma = 13,
    #[doc = "14: Signal `i2c2_tx_dma` selected as request input"]
    I2c2TxDma = 14,
    #[doc = "15: Signal `i2c3_rx_dma` selected as request input"]
    I2c3RxDma = 15,
    #[doc = "16: Signal `i2c3_tx_dma` selected as request input"]
    I2c3TxDma = 16,
    #[doc = "17: Signal `usart1_rx_dma` selected as request input"]
    Usart1RxDma = 17,
    #[doc = "18: Signal `usart1_tx_dma` selected as request input"]
    Usart1TxDma = 18,
    #[doc = "19: Signal `usart2_rx_dma` selected as request input"]
    Usart2RxDma = 19,
    #[doc = "20: Signal `usart2_tx_dma` selected as request input"]
    Usart2TxDma = 20,
    #[doc = "21: Signal `lpuart1_rx_dma` selected as request input"]
    Lpuart1RxDma = 21,
    #[doc = "22: Signal `lpuart1_tx_dma` selected as request input"]
    Lpuart1TxDma = 22,
    #[doc = "23: Signal `tim1_ch1` selected as request input"]
    Tim1Ch1 = 23,
    #[doc = "24: Signal `tim1_ch2` selected as request input"]
    Tim1Ch2 = 24,
    #[doc = "25: Signal `tim1_ch3` selected as request input"]
    Tim1Ch3 = 25,
    #[doc = "26: Signal `tim1_ch4` selected as request input"]
    Tim1Ch4 = 26,
    #[doc = "27: Signal `tim1_up` selected as request input"]
    Tim1Up = 27,
    #[doc = "28: Signal `tim1_trig` selected as request input"]
    Tim1Trig = 28,
    #[doc = "29: Signal `tim1_com` selected as request input"]
    Tim1Com = 29,
    #[doc = "30: Signal `tim2_ch1` selected as request input"]
    Tim2Ch1 = 30,
    #[doc = "31: Signal `tim2_ch2` selected as request input"]
    Tim2Ch2 = 31,
    #[doc = "32: Signal `tim2_ch3` selected as request input"]
    Tim2Ch3 = 32,
    #[doc = "33: Signal `tim2_ch4` selected as request input"]
    Tim2Ch4 = 33,
    #[doc = "34: Signal `tim2_up` selected as request input"]
    Tim2Up = 34,
    #[doc = "35: Signal `tim16_ch1` selected as request input"]
    Tim16Ch1 = 35,
    #[doc = "36: Signal `tim16_up` selected as request input"]
    Tim16Up = 36,
    #[doc = "37: Signal `tim17_ch1` selected as request input"]
    Tim17Ch1 = 37,
    #[doc = "38: Signal `tim17_up` selected as request input"]
    Tim17Up = 38,
    #[doc = "39: Signal `aes_in` selected as request input"]
    AesIn = 39,
    #[doc = "40: Signal `aes_out` selected as request input"]
    AesOut = 40,
    #[doc = "41: Signal `subghzspi_rx` selected as request input"]
    SubghzspiRx = 41,
    #[doc = "42: Signal `subghzspi_tx` selected as request input"]
    SubghzspiTx = 42,
}
impl From<DMAREQ_ID_A> for u8 {
    #[inline(always)]
    fn from(variant: DMAREQ_ID_A) -> Self {
        variant as _
    }
}
impl DMAREQ_ID_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<DMAREQ_ID_A> {
        match self.bits {
            0 => Some(DMAREQ_ID_A::None),
            1 => Some(DMAREQ_ID_A::Dmamux1ReqGen0),
            2 => Some(DMAREQ_ID_A::Dmamux1ReqGen1),
            3 => Some(DMAREQ_ID_A::Dmamux1ReqGen2),
            4 => Some(DMAREQ_ID_A::Dmamux1ReqGen3),
            5 => Some(DMAREQ_ID_A::Adc),
            6 => Some(DMAREQ_ID_A::DatOut1),
            7 => Some(DMAREQ_ID_A::Spi1RxDma),
            8 => Some(DMAREQ_ID_A::Spi1TxDma),
            9 => Some(DMAREQ_ID_A::Spi2RxDma),
            10 => Some(DMAREQ_ID_A::Spi2TxDma),
            11 => Some(DMAREQ_ID_A::I2c1RxDma),
            12 => Some(DMAREQ_ID_A::I2c1TxDma),
            13 => Some(DMAREQ_ID_A::I2c2RxDma),
            14 => Some(DMAREQ_ID_A::I2c2TxDma),
            15 => Some(DMAREQ_ID_A::I2c3RxDma),
            16 => Some(DMAREQ_ID_A::I2c3TxDma),
            17 => Some(DMAREQ_ID_A::Usart1RxDma),
            18 => Some(DMAREQ_ID_A::Usart1TxDma),
            19 => Some(DMAREQ_ID_A::Usart2RxDma),
            20 => Some(DMAREQ_ID_A::Usart2TxDma),
            21 => Some(DMAREQ_ID_A::Lpuart1RxDma),
            22 => Some(DMAREQ_ID_A::Lpuart1TxDma),
            23 => Some(DMAREQ_ID_A::Tim1Ch1),
            24 => Some(DMAREQ_ID_A::Tim1Ch2),
            25 => Some(DMAREQ_ID_A::Tim1Ch3),
            26 => Some(DMAREQ_ID_A::Tim1Ch4),
            27 => Some(DMAREQ_ID_A::Tim1Up),
            28 => Some(DMAREQ_ID_A::Tim1Trig),
            29 => Some(DMAREQ_ID_A::Tim1Com),
            30 => Some(DMAREQ_ID_A::Tim2Ch1),
            31 => Some(DMAREQ_ID_A::Tim2Ch2),
            32 => Some(DMAREQ_ID_A::Tim2Ch3),
            33 => Some(DMAREQ_ID_A::Tim2Ch4),
            34 => Some(DMAREQ_ID_A::Tim2Up),
            35 => Some(DMAREQ_ID_A::Tim16Ch1),
            36 => Some(DMAREQ_ID_A::Tim16Up),
            37 => Some(DMAREQ_ID_A::Tim17Ch1),
            38 => Some(DMAREQ_ID_A::Tim17Up),
            39 => Some(DMAREQ_ID_A::AesIn),
            40 => Some(DMAREQ_ID_A::AesOut),
            41 => Some(DMAREQ_ID_A::SubghzspiRx),
            42 => Some(DMAREQ_ID_A::SubghzspiTx),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `None`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == DMAREQ_ID_A::None
    }
    #[doc = "Checks if the value of the field is `Dmamux1ReqGen0`"]
    #[inline(always)]
    pub fn is_dmamux1_req_gen0(&self) -> bool {
        *self == DMAREQ_ID_A::Dmamux1ReqGen0
    }
    #[doc = "Checks if the value of the field is `Dmamux1ReqGen1`"]
    #[inline(always)]
    pub fn is_dmamux1_req_gen1(&self) -> bool {
        *self == DMAREQ_ID_A::Dmamux1ReqGen1
    }
    #[doc = "Checks if the value of the field is `Dmamux1ReqGen2`"]
    #[inline(always)]
    pub fn is_dmamux1_req_gen2(&self) -> bool {
        *self == DMAREQ_ID_A::Dmamux1ReqGen2
    }
    #[doc = "Checks if the value of the field is `Dmamux1ReqGen3`"]
    #[inline(always)]
    pub fn is_dmamux1_req_gen3(&self) -> bool {
        *self == DMAREQ_ID_A::Dmamux1ReqGen3
    }
    #[doc = "Checks if the value of the field is `Adc`"]
    #[inline(always)]
    pub fn is_adc(&self) -> bool {
        *self == DMAREQ_ID_A::Adc
    }
    #[doc = "Checks if the value of the field is `DatOut1`"]
    #[inline(always)]
    pub fn is_dat_out1(&self) -> bool {
        *self == DMAREQ_ID_A::DatOut1
    }
    #[doc = "Checks if the value of the field is `Spi1RxDma`"]
    #[inline(always)]
    pub fn is_spi1_rx_dma(&self) -> bool {
        *self == DMAREQ_ID_A::Spi1RxDma
    }
    #[doc = "Checks if the value of the field is `Spi1TxDma`"]
    #[inline(always)]
    pub fn is_spi1_tx_dma(&self) -> bool {
        *self == DMAREQ_ID_A::Spi1TxDma
    }
    #[doc = "Checks if the value of the field is `Spi2RxDma`"]
    #[inline(always)]
    pub fn is_spi2_rx_dma(&self) -> bool {
        *self == DMAREQ_ID_A::Spi2RxDma
    }
    #[doc = "Checks if the value of the field is `Spi2TxDma`"]
    #[inline(always)]
    pub fn is_spi2_tx_dma(&self) -> bool {
        *self == DMAREQ_ID_A::Spi2TxDma
    }
    #[doc = "Checks if the value of the field is `I2c1RxDma`"]
    #[inline(always)]
    pub fn is_i2c1_rx_dma(&self) -> bool {
        *self == DMAREQ_ID_A::I2c1RxDma
    }
    #[doc = "Checks if the value of the field is `I2c1TxDma`"]
    #[inline(always)]
    pub fn is_i2c1_tx_dma(&self) -> bool {
        *self == DMAREQ_ID_A::I2c1TxDma
    }
    #[doc = "Checks if the value of the field is `I2c2RxDma`"]
    #[inline(always)]
    pub fn is_i2c2_rx_dma(&self) -> bool {
        *self == DMAREQ_ID_A::I2c2RxDma
    }
    #[doc = "Checks if the value of the field is `I2c2TxDma`"]
    #[inline(always)]
    pub fn is_i2c2_tx_dma(&self) -> bool {
        *self == DMAREQ_ID_A::I2c2TxDma
    }
    #[doc = "Checks if the value of the field is `I2c3RxDma`"]
    #[inline(always)]
    pub fn is_i2c3_rx_dma(&self) -> bool {
        *self == DMAREQ_ID_A::I2c3RxDma
    }
    #[doc = "Checks if the value of the field is `I2c3TxDma`"]
    #[inline(always)]
    pub fn is_i2c3_tx_dma(&self) -> bool {
        *self == DMAREQ_ID_A::I2c3TxDma
    }
    #[doc = "Checks if the value of the field is `Usart1RxDma`"]
    #[inline(always)]
    pub fn is_usart1_rx_dma(&self) -> bool {
        *self == DMAREQ_ID_A::Usart1RxDma
    }
    #[doc = "Checks if the value of the field is `Usart1TxDma`"]
    #[inline(always)]
    pub fn is_usart1_tx_dma(&self) -> bool {
        *self == DMAREQ_ID_A::Usart1TxDma
    }
    #[doc = "Checks if the value of the field is `Usart2RxDma`"]
    #[inline(always)]
    pub fn is_usart2_rx_dma(&self) -> bool {
        *self == DMAREQ_ID_A::Usart2RxDma
    }
    #[doc = "Checks if the value of the field is `Usart2TxDma`"]
    #[inline(always)]
    pub fn is_usart2_tx_dma(&self) -> bool {
        *self == DMAREQ_ID_A::Usart2TxDma
    }
    #[doc = "Checks if the value of the field is `Lpuart1RxDma`"]
    #[inline(always)]
    pub fn is_lpuart1_rx_dma(&self) -> bool {
        *self == DMAREQ_ID_A::Lpuart1RxDma
    }
    #[doc = "Checks if the value of the field is `Lpuart1TxDma`"]
    #[inline(always)]
    pub fn is_lpuart1_tx_dma(&self) -> bool {
        *self == DMAREQ_ID_A::Lpuart1TxDma
    }
    #[doc = "Checks if the value of the field is `Tim1Ch1`"]
    #[inline(always)]
    pub fn is_tim1_ch1(&self) -> bool {
        *self == DMAREQ_ID_A::Tim1Ch1
    }
    #[doc = "Checks if the value of the field is `Tim1Ch2`"]
    #[inline(always)]
    pub fn is_tim1_ch2(&self) -> bool {
        *self == DMAREQ_ID_A::Tim1Ch2
    }
    #[doc = "Checks if the value of the field is `Tim1Ch3`"]
    #[inline(always)]
    pub fn is_tim1_ch3(&self) -> bool {
        *self == DMAREQ_ID_A::Tim1Ch3
    }
    #[doc = "Checks if the value of the field is `Tim1Ch4`"]
    #[inline(always)]
    pub fn is_tim1_ch4(&self) -> bool {
        *self == DMAREQ_ID_A::Tim1Ch4
    }
    #[doc = "Checks if the value of the field is `Tim1Up`"]
    #[inline(always)]
    pub fn is_tim1_up(&self) -> bool {
        *self == DMAREQ_ID_A::Tim1Up
    }
    #[doc = "Checks if the value of the field is `Tim1Trig`"]
    #[inline(always)]
    pub fn is_tim1_trig(&self) -> bool {
        *self == DMAREQ_ID_A::Tim1Trig
    }
    #[doc = "Checks if the value of the field is `Tim1Com`"]
    #[inline(always)]
    pub fn is_tim1_com(&self) -> bool {
        *self == DMAREQ_ID_A::Tim1Com
    }
    #[doc = "Checks if the value of the field is `Tim2Ch1`"]
    #[inline(always)]
    pub fn is_tim2_ch1(&self) -> bool {
        *self == DMAREQ_ID_A::Tim2Ch1
    }
    #[doc = "Checks if the value of the field is `Tim2Ch2`"]
    #[inline(always)]
    pub fn is_tim2_ch2(&self) -> bool {
        *self == DMAREQ_ID_A::Tim2Ch2
    }
    #[doc = "Checks if the value of the field is `Tim2Ch3`"]
    #[inline(always)]
    pub fn is_tim2_ch3(&self) -> bool {
        *self == DMAREQ_ID_A::Tim2Ch3
    }
    #[doc = "Checks if the value of the field is `Tim2Ch4`"]
    #[inline(always)]
    pub fn is_tim2_ch4(&self) -> bool {
        *self == DMAREQ_ID_A::Tim2Ch4
    }
    #[doc = "Checks if the value of the field is `Tim2Up`"]
    #[inline(always)]
    pub fn is_tim2_up(&self) -> bool {
        *self == DMAREQ_ID_A::Tim2Up
    }
    #[doc = "Checks if the value of the field is `Tim16Ch1`"]
    #[inline(always)]
    pub fn is_tim16_ch1(&self) -> bool {
        *self == DMAREQ_ID_A::Tim16Ch1
    }
    #[doc = "Checks if the value of the field is `Tim16Up`"]
    #[inline(always)]
    pub fn is_tim16_up(&self) -> bool {
        *self == DMAREQ_ID_A::Tim16Up
    }
    #[doc = "Checks if the value of the field is `Tim17Ch1`"]
    #[inline(always)]
    pub fn is_tim17_ch1(&self) -> bool {
        *self == DMAREQ_ID_A::Tim17Ch1
    }
    #[doc = "Checks if the value of the field is `Tim17Up`"]
    #[inline(always)]
    pub fn is_tim17_up(&self) -> bool {
        *self == DMAREQ_ID_A::Tim17Up
    }
    #[doc = "Checks if the value of the field is `AesIn`"]
    #[inline(always)]
    pub fn is_aes_in(&self) -> bool {
        *self == DMAREQ_ID_A::AesIn
    }
    #[doc = "Checks if the value of the field is `AesOut`"]
    #[inline(always)]
    pub fn is_aes_out(&self) -> bool {
        *self == DMAREQ_ID_A::AesOut
    }
    #[doc = "Checks if the value of the field is `SubghzspiRx`"]
    #[inline(always)]
    pub fn is_subghzspi_rx(&self) -> bool {
        *self == DMAREQ_ID_A::SubghzspiRx
    }
    #[doc = "Checks if the value of the field is `SubghzspiTx`"]
    #[inline(always)]
    pub fn is_subghzspi_tx(&self) -> bool {
        *self == DMAREQ_ID_A::SubghzspiTx
    }
}
#[doc = "Field `DMAREQ_ID` writer - DMA request identification"]
pub type DMAREQ_ID_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, C4CR_SPEC, u8, DMAREQ_ID_A, 8, O>;
impl<'a, const O: u8> DMAREQ_ID_W<'a, O> {
    #[doc = "No signal selected as request input"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::None)
    }
    #[doc = "Signal `dmamux1_req_gen0` selected as request input"]
    #[inline(always)]
    pub fn dmamux1_req_gen0(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::Dmamux1ReqGen0)
    }
    #[doc = "Signal `dmamux1_req_gen1` selected as request input"]
    #[inline(always)]
    pub fn dmamux1_req_gen1(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::Dmamux1ReqGen1)
    }
    #[doc = "Signal `dmamux1_req_gen2` selected as request input"]
    #[inline(always)]
    pub fn dmamux1_req_gen2(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::Dmamux1ReqGen2)
    }
    #[doc = "Signal `dmamux1_req_gen3` selected as request input"]
    #[inline(always)]
    pub fn dmamux1_req_gen3(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::Dmamux1ReqGen3)
    }
    #[doc = "Signal `adc1_dma` selected as request input"]
    #[inline(always)]
    pub fn adc(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::Adc)
    }
    #[doc = "Signal `dac_out1_dma` selected as request input"]
    #[inline(always)]
    pub fn dat_out1(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::DatOut1)
    }
    #[doc = "Signal `spi1_rx_dma` selected as request input"]
    #[inline(always)]
    pub fn spi1_rx_dma(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::Spi1RxDma)
    }
    #[doc = "Signal `spi1_tx_dma` selected as request input"]
    #[inline(always)]
    pub fn spi1_tx_dma(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::Spi1TxDma)
    }
    #[doc = "Signal `spi2_rx_dma` selected as request input"]
    #[inline(always)]
    pub fn spi2_rx_dma(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::Spi2RxDma)
    }
    #[doc = "Signal `spi2_tx_dma` selected as request input"]
    #[inline(always)]
    pub fn spi2_tx_dma(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::Spi2TxDma)
    }
    #[doc = "Signal `i2c1_rx_dma` selected as request input"]
    #[inline(always)]
    pub fn i2c1_rx_dma(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::I2c1RxDma)
    }
    #[doc = "Signal `i2c1_tx_dma` selected as request input"]
    #[inline(always)]
    pub fn i2c1_tx_dma(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::I2c1TxDma)
    }
    #[doc = "Signal `i2c2_rx_dma` selected as request input"]
    #[inline(always)]
    pub fn i2c2_rx_dma(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::I2c2RxDma)
    }
    #[doc = "Signal `i2c2_tx_dma` selected as request input"]
    #[inline(always)]
    pub fn i2c2_tx_dma(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::I2c2TxDma)
    }
    #[doc = "Signal `i2c3_rx_dma` selected as request input"]
    #[inline(always)]
    pub fn i2c3_rx_dma(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::I2c3RxDma)
    }
    #[doc = "Signal `i2c3_tx_dma` selected as request input"]
    #[inline(always)]
    pub fn i2c3_tx_dma(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::I2c3TxDma)
    }
    #[doc = "Signal `usart1_rx_dma` selected as request input"]
    #[inline(always)]
    pub fn usart1_rx_dma(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::Usart1RxDma)
    }
    #[doc = "Signal `usart1_tx_dma` selected as request input"]
    #[inline(always)]
    pub fn usart1_tx_dma(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::Usart1TxDma)
    }
    #[doc = "Signal `usart2_rx_dma` selected as request input"]
    #[inline(always)]
    pub fn usart2_rx_dma(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::Usart2RxDma)
    }
    #[doc = "Signal `usart2_tx_dma` selected as request input"]
    #[inline(always)]
    pub fn usart2_tx_dma(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::Usart2TxDma)
    }
    #[doc = "Signal `lpuart1_rx_dma` selected as request input"]
    #[inline(always)]
    pub fn lpuart1_rx_dma(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::Lpuart1RxDma)
    }
    #[doc = "Signal `lpuart1_tx_dma` selected as request input"]
    #[inline(always)]
    pub fn lpuart1_tx_dma(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::Lpuart1TxDma)
    }
    #[doc = "Signal `tim1_ch1` selected as request input"]
    #[inline(always)]
    pub fn tim1_ch1(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::Tim1Ch1)
    }
    #[doc = "Signal `tim1_ch2` selected as request input"]
    #[inline(always)]
    pub fn tim1_ch2(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::Tim1Ch2)
    }
    #[doc = "Signal `tim1_ch3` selected as request input"]
    #[inline(always)]
    pub fn tim1_ch3(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::Tim1Ch3)
    }
    #[doc = "Signal `tim1_ch4` selected as request input"]
    #[inline(always)]
    pub fn tim1_ch4(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::Tim1Ch4)
    }
    #[doc = "Signal `tim1_up` selected as request input"]
    #[inline(always)]
    pub fn tim1_up(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::Tim1Up)
    }
    #[doc = "Signal `tim1_trig` selected as request input"]
    #[inline(always)]
    pub fn tim1_trig(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::Tim1Trig)
    }
    #[doc = "Signal `tim1_com` selected as request input"]
    #[inline(always)]
    pub fn tim1_com(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::Tim1Com)
    }
    #[doc = "Signal `tim2_ch1` selected as request input"]
    #[inline(always)]
    pub fn tim2_ch1(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::Tim2Ch1)
    }
    #[doc = "Signal `tim2_ch2` selected as request input"]
    #[inline(always)]
    pub fn tim2_ch2(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::Tim2Ch2)
    }
    #[doc = "Signal `tim2_ch3` selected as request input"]
    #[inline(always)]
    pub fn tim2_ch3(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::Tim2Ch3)
    }
    #[doc = "Signal `tim2_ch4` selected as request input"]
    #[inline(always)]
    pub fn tim2_ch4(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::Tim2Ch4)
    }
    #[doc = "Signal `tim2_up` selected as request input"]
    #[inline(always)]
    pub fn tim2_up(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::Tim2Up)
    }
    #[doc = "Signal `tim16_ch1` selected as request input"]
    #[inline(always)]
    pub fn tim16_ch1(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::Tim16Ch1)
    }
    #[doc = "Signal `tim16_up` selected as request input"]
    #[inline(always)]
    pub fn tim16_up(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::Tim16Up)
    }
    #[doc = "Signal `tim17_ch1` selected as request input"]
    #[inline(always)]
    pub fn tim17_ch1(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::Tim17Ch1)
    }
    #[doc = "Signal `tim17_up` selected as request input"]
    #[inline(always)]
    pub fn tim17_up(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::Tim17Up)
    }
    #[doc = "Signal `aes_in` selected as request input"]
    #[inline(always)]
    pub fn aes_in(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::AesIn)
    }
    #[doc = "Signal `aes_out` selected as request input"]
    #[inline(always)]
    pub fn aes_out(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::AesOut)
    }
    #[doc = "Signal `subghzspi_rx` selected as request input"]
    #[inline(always)]
    pub fn subghzspi_rx(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::SubghzspiRx)
    }
    #[doc = "Signal `subghzspi_tx` selected as request input"]
    #[inline(always)]
    pub fn subghzspi_tx(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::SubghzspiTx)
    }
}
#[doc = "Field `SOIE` reader - Synchronization overrun interrupt enable"]
pub type SOIE_R = crate::BitReader<SOIE_A>;
#[doc = "Synchronization overrun interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SOIE_A {
    #[doc = "0: Synchronization overrun interrupt disabled"]
    Disabled = 0,
    #[doc = "1: Synchronization overrun interrupt enabled"]
    Enabled = 1,
}
impl From<SOIE_A> for bool {
    #[inline(always)]
    fn from(variant: SOIE_A) -> Self {
        variant as u8 != 0
    }
}
impl SOIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SOIE_A {
        match self.bits {
            false => SOIE_A::Disabled,
            true => SOIE_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SOIE_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SOIE_A::Enabled
    }
}
#[doc = "Field `SOIE` writer - Synchronization overrun interrupt enable"]
pub type SOIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, C4CR_SPEC, SOIE_A, O>;
impl<'a, const O: u8> SOIE_W<'a, O> {
    #[doc = "Synchronization overrun interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SOIE_A::Disabled)
    }
    #[doc = "Synchronization overrun interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SOIE_A::Enabled)
    }
}
#[doc = "Field `EGE` reader - Event generation enable"]
pub type EGE_R = crate::BitReader<EGE_A>;
#[doc = "Event generation enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EGE_A {
    #[doc = "0: Event generation disabled"]
    Disabled = 0,
    #[doc = "1: Event generation enabled"]
    Enabled = 1,
}
impl From<EGE_A> for bool {
    #[inline(always)]
    fn from(variant: EGE_A) -> Self {
        variant as u8 != 0
    }
}
impl EGE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EGE_A {
        match self.bits {
            false => EGE_A::Disabled,
            true => EGE_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EGE_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == EGE_A::Enabled
    }
}
#[doc = "Field `EGE` writer - Event generation enable"]
pub type EGE_W<'a, const O: u8> = crate::BitWriter<'a, u32, C4CR_SPEC, EGE_A, O>;
impl<'a, const O: u8> EGE_W<'a, O> {
    #[doc = "Event generation disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(EGE_A::Disabled)
    }
    #[doc = "Event generation enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(EGE_A::Enabled)
    }
}
#[doc = "Field `SE` reader - Synchronization enable"]
pub type SE_R = crate::BitReader<SE_A>;
#[doc = "Synchronization enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SE_A {
    #[doc = "0: Synchronization disabled"]
    Disabled = 0,
    #[doc = "1: Synchronization enabled"]
    Enabled = 1,
}
impl From<SE_A> for bool {
    #[inline(always)]
    fn from(variant: SE_A) -> Self {
        variant as u8 != 0
    }
}
impl SE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SE_A {
        match self.bits {
            false => SE_A::Disabled,
            true => SE_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SE_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SE_A::Enabled
    }
}
#[doc = "Field `SE` writer - Synchronization enable"]
pub type SE_W<'a, const O: u8> = crate::BitWriter<'a, u32, C4CR_SPEC, SE_A, O>;
impl<'a, const O: u8> SE_W<'a, O> {
    #[doc = "Synchronization disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SE_A::Disabled)
    }
    #[doc = "Synchronization enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SE_A::Enabled)
    }
}
#[doc = "Field `SPOL` reader - Synchronization polarity"]
pub type SPOL_R = crate::FieldReader<u8, SPOL_A>;
#[doc = "Synchronization polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SPOL_A {
    #[doc = "0: No event, i.e. no synchronization nor detection"]
    NoEdge = 0,
    #[doc = "1: Rising edge"]
    RisingEdge = 1,
    #[doc = "2: Falling edge"]
    FallingEdge = 2,
    #[doc = "3: Rising and falling edges"]
    BothEdges = 3,
}
impl From<SPOL_A> for u8 {
    #[inline(always)]
    fn from(variant: SPOL_A) -> Self {
        variant as _
    }
}
impl SPOL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPOL_A {
        match self.bits {
            0 => SPOL_A::NoEdge,
            1 => SPOL_A::RisingEdge,
            2 => SPOL_A::FallingEdge,
            3 => SPOL_A::BothEdges,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NoEdge`"]
    #[inline(always)]
    pub fn is_no_edge(&self) -> bool {
        *self == SPOL_A::NoEdge
    }
    #[doc = "Checks if the value of the field is `RisingEdge`"]
    #[inline(always)]
    pub fn is_rising_edge(&self) -> bool {
        *self == SPOL_A::RisingEdge
    }
    #[doc = "Checks if the value of the field is `FallingEdge`"]
    #[inline(always)]
    pub fn is_falling_edge(&self) -> bool {
        *self == SPOL_A::FallingEdge
    }
    #[doc = "Checks if the value of the field is `BothEdges`"]
    #[inline(always)]
    pub fn is_both_edges(&self) -> bool {
        *self == SPOL_A::BothEdges
    }
}
#[doc = "Field `SPOL` writer - Synchronization polarity"]
pub type SPOL_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, C4CR_SPEC, u8, SPOL_A, 2, O>;
impl<'a, const O: u8> SPOL_W<'a, O> {
    #[doc = "No event, i.e. no synchronization nor detection"]
    #[inline(always)]
    pub fn no_edge(self) -> &'a mut W {
        self.variant(SPOL_A::NoEdge)
    }
    #[doc = "Rising edge"]
    #[inline(always)]
    pub fn rising_edge(self) -> &'a mut W {
        self.variant(SPOL_A::RisingEdge)
    }
    #[doc = "Falling edge"]
    #[inline(always)]
    pub fn falling_edge(self) -> &'a mut W {
        self.variant(SPOL_A::FallingEdge)
    }
    #[doc = "Rising and falling edges"]
    #[inline(always)]
    pub fn both_edges(self) -> &'a mut W {
        self.variant(SPOL_A::BothEdges)
    }
}
#[doc = "Field `NBREQ` reader - Number of DMA requests minus 1 to forward"]
pub type NBREQ_R = crate::FieldReader<u8, u8>;
#[doc = "Field `NBREQ` writer - Number of DMA requests minus 1 to forward"]
pub type NBREQ_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, C4CR_SPEC, u8, u8, 5, O>;
#[doc = "Field `SYNC_ID` reader - Synchronization identification"]
pub type SYNC_ID_R = crate::FieldReader<u8, SYNC_ID_A>;
#[doc = "Synchronization identification\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SYNC_ID_A {
    #[doc = "0: Signal `EXTIx` selected as synchronization input"]
    Exti0 = 0,
    #[doc = "1: Signal `EXTIx` selected as synchronization input"]
    Exti1 = 1,
    #[doc = "2: Signal `EXTIx` selected as synchronization input"]
    Exti2 = 2,
    #[doc = "3: Signal `EXTIx` selected as synchronization input"]
    Exti3 = 3,
    #[doc = "4: Signal `EXTIx` selected as synchronization input"]
    Exti4 = 4,
    #[doc = "5: Signal `EXTIx` selected as synchronization input"]
    Exti5 = 5,
    #[doc = "6: Signal `EXTIx` selected as synchronization input"]
    Exti6 = 6,
    #[doc = "7: Signal `EXTIx` selected as synchronization input"]
    Exti7 = 7,
    #[doc = "8: Signal `EXTIx` selected as synchronization input"]
    Exti8 = 8,
    #[doc = "9: Signal `EXTIx` selected as synchronization input"]
    Exti9 = 9,
    #[doc = "10: Signal `EXTIx` selected as synchronization input"]
    Exti10 = 10,
    #[doc = "11: Signal `EXTIx` selected as synchronization input"]
    Exti11 = 11,
    #[doc = "12: Signal `EXTIx` selected as synchronization input"]
    Exti12 = 12,
    #[doc = "13: Signal `EXTIx` selected as synchronization input"]
    Exti13 = 13,
    #[doc = "14: Signal `EXTIx` selected as synchronization input"]
    Exti14 = 14,
    #[doc = "15: Signal `EXTIx` selected as synchronization input"]
    Exti15 = 15,
    #[doc = "16: Signal `dmamux1_evt0` selected as synchronization input"]
    Dmamux1Evt0 = 16,
    #[doc = "17: Signal `dmamux1_evt1` selected as synchronization input"]
    Dmamux1Evt1 = 17,
    #[doc = "18: Signal `lptim1_out` selected as synchronization input"]
    Lptim1Out = 18,
    #[doc = "19: Signal `lptim2_out` selected as synchronization input"]
    Lptim2Out = 19,
    #[doc = "20: Signal `lptim3_out` selected as synchronization input"]
    Lptim3Out = 20,
}
impl From<SYNC_ID_A> for u8 {
    #[inline(always)]
    fn from(variant: SYNC_ID_A) -> Self {
        variant as _
    }
}
impl SYNC_ID_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SYNC_ID_A> {
        match self.bits {
            0 => Some(SYNC_ID_A::Exti0),
            1 => Some(SYNC_ID_A::Exti1),
            2 => Some(SYNC_ID_A::Exti2),
            3 => Some(SYNC_ID_A::Exti3),
            4 => Some(SYNC_ID_A::Exti4),
            5 => Some(SYNC_ID_A::Exti5),
            6 => Some(SYNC_ID_A::Exti6),
            7 => Some(SYNC_ID_A::Exti7),
            8 => Some(SYNC_ID_A::Exti8),
            9 => Some(SYNC_ID_A::Exti9),
            10 => Some(SYNC_ID_A::Exti10),
            11 => Some(SYNC_ID_A::Exti11),
            12 => Some(SYNC_ID_A::Exti12),
            13 => Some(SYNC_ID_A::Exti13),
            14 => Some(SYNC_ID_A::Exti14),
            15 => Some(SYNC_ID_A::Exti15),
            16 => Some(SYNC_ID_A::Dmamux1Evt0),
            17 => Some(SYNC_ID_A::Dmamux1Evt1),
            18 => Some(SYNC_ID_A::Lptim1Out),
            19 => Some(SYNC_ID_A::Lptim2Out),
            20 => Some(SYNC_ID_A::Lptim3Out),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `Exti0`"]
    #[inline(always)]
    pub fn is_exti0(&self) -> bool {
        *self == SYNC_ID_A::Exti0
    }
    #[doc = "Checks if the value of the field is `Exti1`"]
    #[inline(always)]
    pub fn is_exti1(&self) -> bool {
        *self == SYNC_ID_A::Exti1
    }
    #[doc = "Checks if the value of the field is `Exti2`"]
    #[inline(always)]
    pub fn is_exti2(&self) -> bool {
        *self == SYNC_ID_A::Exti2
    }
    #[doc = "Checks if the value of the field is `Exti3`"]
    #[inline(always)]
    pub fn is_exti3(&self) -> bool {
        *self == SYNC_ID_A::Exti3
    }
    #[doc = "Checks if the value of the field is `Exti4`"]
    #[inline(always)]
    pub fn is_exti4(&self) -> bool {
        *self == SYNC_ID_A::Exti4
    }
    #[doc = "Checks if the value of the field is `Exti5`"]
    #[inline(always)]
    pub fn is_exti5(&self) -> bool {
        *self == SYNC_ID_A::Exti5
    }
    #[doc = "Checks if the value of the field is `Exti6`"]
    #[inline(always)]
    pub fn is_exti6(&self) -> bool {
        *self == SYNC_ID_A::Exti6
    }
    #[doc = "Checks if the value of the field is `Exti7`"]
    #[inline(always)]
    pub fn is_exti7(&self) -> bool {
        *self == SYNC_ID_A::Exti7
    }
    #[doc = "Checks if the value of the field is `Exti8`"]
    #[inline(always)]
    pub fn is_exti8(&self) -> bool {
        *self == SYNC_ID_A::Exti8
    }
    #[doc = "Checks if the value of the field is `Exti9`"]
    #[inline(always)]
    pub fn is_exti9(&self) -> bool {
        *self == SYNC_ID_A::Exti9
    }
    #[doc = "Checks if the value of the field is `Exti10`"]
    #[inline(always)]
    pub fn is_exti10(&self) -> bool {
        *self == SYNC_ID_A::Exti10
    }
    #[doc = "Checks if the value of the field is `Exti11`"]
    #[inline(always)]
    pub fn is_exti11(&self) -> bool {
        *self == SYNC_ID_A::Exti11
    }
    #[doc = "Checks if the value of the field is `Exti12`"]
    #[inline(always)]
    pub fn is_exti12(&self) -> bool {
        *self == SYNC_ID_A::Exti12
    }
    #[doc = "Checks if the value of the field is `Exti13`"]
    #[inline(always)]
    pub fn is_exti13(&self) -> bool {
        *self == SYNC_ID_A::Exti13
    }
    #[doc = "Checks if the value of the field is `Exti14`"]
    #[inline(always)]
    pub fn is_exti14(&self) -> bool {
        *self == SYNC_ID_A::Exti14
    }
    #[doc = "Checks if the value of the field is `Exti15`"]
    #[inline(always)]
    pub fn is_exti15(&self) -> bool {
        *self == SYNC_ID_A::Exti15
    }
    #[doc = "Checks if the value of the field is `Dmamux1Evt0`"]
    #[inline(always)]
    pub fn is_dmamux1_evt0(&self) -> bool {
        *self == SYNC_ID_A::Dmamux1Evt0
    }
    #[doc = "Checks if the value of the field is `Dmamux1Evt1`"]
    #[inline(always)]
    pub fn is_dmamux1_evt1(&self) -> bool {
        *self == SYNC_ID_A::Dmamux1Evt1
    }
    #[doc = "Checks if the value of the field is `Lptim1Out`"]
    #[inline(always)]
    pub fn is_lptim1_out(&self) -> bool {
        *self == SYNC_ID_A::Lptim1Out
    }
    #[doc = "Checks if the value of the field is `Lptim2Out`"]
    #[inline(always)]
    pub fn is_lptim2_out(&self) -> bool {
        *self == SYNC_ID_A::Lptim2Out
    }
    #[doc = "Checks if the value of the field is `Lptim3Out`"]
    #[inline(always)]
    pub fn is_lptim3_out(&self) -> bool {
        *self == SYNC_ID_A::Lptim3Out
    }
}
#[doc = "Field `SYNC_ID` writer - Synchronization identification"]
pub type SYNC_ID_W<'a, const O: u8> = crate::FieldWriter<'a, u32, C4CR_SPEC, u8, SYNC_ID_A, 5, O>;
impl<'a, const O: u8> SYNC_ID_W<'a, O> {
    #[doc = "Signal `EXTIx` selected as synchronization input"]
    #[inline(always)]
    pub fn exti0(self) -> &'a mut W {
        self.variant(SYNC_ID_A::Exti0)
    }
    #[doc = "Signal `EXTIx` selected as synchronization input"]
    #[inline(always)]
    pub fn exti1(self) -> &'a mut W {
        self.variant(SYNC_ID_A::Exti1)
    }
    #[doc = "Signal `EXTIx` selected as synchronization input"]
    #[inline(always)]
    pub fn exti2(self) -> &'a mut W {
        self.variant(SYNC_ID_A::Exti2)
    }
    #[doc = "Signal `EXTIx` selected as synchronization input"]
    #[inline(always)]
    pub fn exti3(self) -> &'a mut W {
        self.variant(SYNC_ID_A::Exti3)
    }
    #[doc = "Signal `EXTIx` selected as synchronization input"]
    #[inline(always)]
    pub fn exti4(self) -> &'a mut W {
        self.variant(SYNC_ID_A::Exti4)
    }
    #[doc = "Signal `EXTIx` selected as synchronization input"]
    #[inline(always)]
    pub fn exti5(self) -> &'a mut W {
        self.variant(SYNC_ID_A::Exti5)
    }
    #[doc = "Signal `EXTIx` selected as synchronization input"]
    #[inline(always)]
    pub fn exti6(self) -> &'a mut W {
        self.variant(SYNC_ID_A::Exti6)
    }
    #[doc = "Signal `EXTIx` selected as synchronization input"]
    #[inline(always)]
    pub fn exti7(self) -> &'a mut W {
        self.variant(SYNC_ID_A::Exti7)
    }
    #[doc = "Signal `EXTIx` selected as synchronization input"]
    #[inline(always)]
    pub fn exti8(self) -> &'a mut W {
        self.variant(SYNC_ID_A::Exti8)
    }
    #[doc = "Signal `EXTIx` selected as synchronization input"]
    #[inline(always)]
    pub fn exti9(self) -> &'a mut W {
        self.variant(SYNC_ID_A::Exti9)
    }
    #[doc = "Signal `EXTIx` selected as synchronization input"]
    #[inline(always)]
    pub fn exti10(self) -> &'a mut W {
        self.variant(SYNC_ID_A::Exti10)
    }
    #[doc = "Signal `EXTIx` selected as synchronization input"]
    #[inline(always)]
    pub fn exti11(self) -> &'a mut W {
        self.variant(SYNC_ID_A::Exti11)
    }
    #[doc = "Signal `EXTIx` selected as synchronization input"]
    #[inline(always)]
    pub fn exti12(self) -> &'a mut W {
        self.variant(SYNC_ID_A::Exti12)
    }
    #[doc = "Signal `EXTIx` selected as synchronization input"]
    #[inline(always)]
    pub fn exti13(self) -> &'a mut W {
        self.variant(SYNC_ID_A::Exti13)
    }
    #[doc = "Signal `EXTIx` selected as synchronization input"]
    #[inline(always)]
    pub fn exti14(self) -> &'a mut W {
        self.variant(SYNC_ID_A::Exti14)
    }
    #[doc = "Signal `EXTIx` selected as synchronization input"]
    #[inline(always)]
    pub fn exti15(self) -> &'a mut W {
        self.variant(SYNC_ID_A::Exti15)
    }
    #[doc = "Signal `dmamux1_evt0` selected as synchronization input"]
    #[inline(always)]
    pub fn dmamux1_evt0(self) -> &'a mut W {
        self.variant(SYNC_ID_A::Dmamux1Evt0)
    }
    #[doc = "Signal `dmamux1_evt1` selected as synchronization input"]
    #[inline(always)]
    pub fn dmamux1_evt1(self) -> &'a mut W {
        self.variant(SYNC_ID_A::Dmamux1Evt1)
    }
    #[doc = "Signal `lptim1_out` selected as synchronization input"]
    #[inline(always)]
    pub fn lptim1_out(self) -> &'a mut W {
        self.variant(SYNC_ID_A::Lptim1Out)
    }
    #[doc = "Signal `lptim2_out` selected as synchronization input"]
    #[inline(always)]
    pub fn lptim2_out(self) -> &'a mut W {
        self.variant(SYNC_ID_A::Lptim2Out)
    }
    #[doc = "Signal `lptim3_out` selected as synchronization input"]
    #[inline(always)]
    pub fn lptim3_out(self) -> &'a mut W {
        self.variant(SYNC_ID_A::Lptim3Out)
    }
}
impl R {
    #[doc = "Bits 0:7 - DMA request identification"]
    #[inline(always)]
    pub fn dmareq_id(&self) -> DMAREQ_ID_R {
        DMAREQ_ID_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - Synchronization overrun interrupt enable"]
    #[inline(always)]
    pub fn soie(&self) -> SOIE_R {
        SOIE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Event generation enable"]
    #[inline(always)]
    pub fn ege(&self) -> EGE_R {
        EGE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 16 - Synchronization enable"]
    #[inline(always)]
    pub fn se(&self) -> SE_R {
        SE_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:18 - Synchronization polarity"]
    #[inline(always)]
    pub fn spol(&self) -> SPOL_R {
        SPOL_R::new(((self.bits >> 17) & 3) as u8)
    }
    #[doc = "Bits 19:23 - Number of DMA requests minus 1 to forward"]
    #[inline(always)]
    pub fn nbreq(&self) -> NBREQ_R {
        NBREQ_R::new(((self.bits >> 19) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28 - Synchronization identification"]
    #[inline(always)]
    pub fn sync_id(&self) -> SYNC_ID_R {
        SYNC_ID_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - DMA request identification"]
    #[inline(always)]
    #[must_use]
    pub fn dmareq_id(&mut self) -> DMAREQ_ID_W<0> {
        DMAREQ_ID_W::new(self)
    }
    #[doc = "Bit 8 - Synchronization overrun interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn soie(&mut self) -> SOIE_W<8> {
        SOIE_W::new(self)
    }
    #[doc = "Bit 9 - Event generation enable"]
    #[inline(always)]
    #[must_use]
    pub fn ege(&mut self) -> EGE_W<9> {
        EGE_W::new(self)
    }
    #[doc = "Bit 16 - Synchronization enable"]
    #[inline(always)]
    #[must_use]
    pub fn se(&mut self) -> SE_W<16> {
        SE_W::new(self)
    }
    #[doc = "Bits 17:18 - Synchronization polarity"]
    #[inline(always)]
    #[must_use]
    pub fn spol(&mut self) -> SPOL_W<17> {
        SPOL_W::new(self)
    }
    #[doc = "Bits 19:23 - Number of DMA requests minus 1 to forward"]
    #[inline(always)]
    #[must_use]
    pub fn nbreq(&mut self) -> NBREQ_W<19> {
        NBREQ_W::new(self)
    }
    #[doc = "Bits 24:28 - Synchronization identification"]
    #[inline(always)]
    #[must_use]
    pub fn sync_id(&mut self) -> SYNC_ID_W<24> {
        SYNC_ID_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "request line multiplexer channel x configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c4cr](index.html) module"]
pub struct C4CR_SPEC;
impl crate::RegisterSpec for C4CR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [c4cr::R](R) reader structure"]
impl crate::Readable for C4CR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [c4cr::W](W) writer structure"]
impl crate::Writable for C4CR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets C4CR to value 0"]
impl crate::Resettable for C4CR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
