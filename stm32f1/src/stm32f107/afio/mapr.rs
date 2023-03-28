#[doc = "Register `MAPR` reader"]
pub struct R(crate::R<MAPR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MAPR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MAPR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MAPR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MAPR` writer"]
pub struct W(crate::W<MAPR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MAPR_SPEC>;
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
impl From<crate::W<MAPR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MAPR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SPI1_REMAP` reader - SPI1 remapping"]
pub type SPI1_REMAP_R = crate::BitReader<bool>;
#[doc = "Field `SPI1_REMAP` writer - SPI1 remapping"]
pub type SPI1_REMAP_W<'a, const O: u8> = crate::BitWriter<'a, u32, MAPR_SPEC, bool, O>;
#[doc = "Field `I2C1_REMAP` reader - I2C1 remapping"]
pub type I2C1_REMAP_R = crate::BitReader<bool>;
#[doc = "Field `I2C1_REMAP` writer - I2C1 remapping"]
pub type I2C1_REMAP_W<'a, const O: u8> = crate::BitWriter<'a, u32, MAPR_SPEC, bool, O>;
#[doc = "Field `USART1_REMAP` reader - USART1 remapping"]
pub type USART1_REMAP_R = crate::BitReader<bool>;
#[doc = "Field `USART1_REMAP` writer - USART1 remapping"]
pub type USART1_REMAP_W<'a, const O: u8> = crate::BitWriter<'a, u32, MAPR_SPEC, bool, O>;
#[doc = "Field `USART2_REMAP` reader - USART2 remapping"]
pub type USART2_REMAP_R = crate::BitReader<bool>;
#[doc = "Field `USART2_REMAP` writer - USART2 remapping"]
pub type USART2_REMAP_W<'a, const O: u8> = crate::BitWriter<'a, u32, MAPR_SPEC, bool, O>;
#[doc = "Field `USART3_REMAP` reader - USART3 remapping"]
pub type USART3_REMAP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `USART3_REMAP` writer - USART3 remapping"]
pub type USART3_REMAP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MAPR_SPEC, u8, u8, 2, O>;
#[doc = "Field `TIM1_REMAP` reader - TIM1 remapping"]
pub type TIM1_REMAP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TIM1_REMAP` writer - TIM1 remapping"]
pub type TIM1_REMAP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MAPR_SPEC, u8, u8, 2, O>;
#[doc = "Field `TIM2_REMAP` reader - TIM2 remapping"]
pub type TIM2_REMAP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TIM2_REMAP` writer - TIM2 remapping"]
pub type TIM2_REMAP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MAPR_SPEC, u8, u8, 2, O>;
#[doc = "Field `TIM3_REMAP` reader - TIM3 remapping"]
pub type TIM3_REMAP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TIM3_REMAP` writer - TIM3 remapping"]
pub type TIM3_REMAP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MAPR_SPEC, u8, u8, 2, O>;
#[doc = "Field `TIM4_REMAP` reader - TIM4 remapping"]
pub type TIM4_REMAP_R = crate::BitReader<bool>;
#[doc = "Field `TIM4_REMAP` writer - TIM4 remapping"]
pub type TIM4_REMAP_W<'a, const O: u8> = crate::BitWriter<'a, u32, MAPR_SPEC, bool, O>;
#[doc = "Field `CAN1_REMAP` reader - CAN1 remapping"]
pub type CAN1_REMAP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CAN1_REMAP` writer - CAN1 remapping"]
pub type CAN1_REMAP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MAPR_SPEC, u8, u8, 2, O>;
#[doc = "Field `PD01_REMAP` reader - Port D0/Port D1 mapping on OSCIN/OSCOUT"]
pub type PD01_REMAP_R = crate::BitReader<bool>;
#[doc = "Field `PD01_REMAP` writer - Port D0/Port D1 mapping on OSCIN/OSCOUT"]
pub type PD01_REMAP_W<'a, const O: u8> = crate::BitWriter<'a, u32, MAPR_SPEC, bool, O>;
#[doc = "Field `TIM5CH4_IREMAP` reader - Set and cleared by software"]
pub type TIM5CH4_IREMAP_R = crate::BitReader<bool>;
#[doc = "Field `TIM5CH4_IREMAP` writer - Set and cleared by software"]
pub type TIM5CH4_IREMAP_W<'a, const O: u8> = crate::BitWriter<'a, u32, MAPR_SPEC, bool, O>;
#[doc = "Field `ETH_REMAP` reader - Ethernet MAC I/O remapping"]
pub type ETH_REMAP_R = crate::BitReader<bool>;
#[doc = "Field `ETH_REMAP` writer - Ethernet MAC I/O remapping"]
pub type ETH_REMAP_W<'a, const O: u8> = crate::BitWriter<'a, u32, MAPR_SPEC, bool, O>;
#[doc = "Field `CAN2_REMAP` reader - CAN2 I/O remapping"]
pub type CAN2_REMAP_R = crate::BitReader<bool>;
#[doc = "Field `CAN2_REMAP` writer - CAN2 I/O remapping"]
pub type CAN2_REMAP_W<'a, const O: u8> = crate::BitWriter<'a, u32, MAPR_SPEC, bool, O>;
#[doc = "Field `MII_RMII_SEL` reader - MII or RMII selection"]
pub type MII_RMII_SEL_R = crate::BitReader<bool>;
#[doc = "Field `MII_RMII_SEL` writer - MII or RMII selection"]
pub type MII_RMII_SEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, MAPR_SPEC, bool, O>;
#[doc = "Field `SWJ_CFG` writer - Serial wire JTAG configuration"]
pub type SWJ_CFG_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MAPR_SPEC, u8, u8, 3, O>;
#[doc = "Field `SPI3_REMAP` reader - SPI3/I2S3 remapping"]
pub type SPI3_REMAP_R = crate::BitReader<bool>;
#[doc = "Field `SPI3_REMAP` writer - SPI3/I2S3 remapping"]
pub type SPI3_REMAP_W<'a, const O: u8> = crate::BitWriter<'a, u32, MAPR_SPEC, bool, O>;
#[doc = "Field `TIM2ITR1_IREMAP` reader - TIM2 internal trigger 1 remapping"]
pub type TIM2ITR1_IREMAP_R = crate::BitReader<bool>;
#[doc = "Field `TIM2ITR1_IREMAP` writer - TIM2 internal trigger 1 remapping"]
pub type TIM2ITR1_IREMAP_W<'a, const O: u8> = crate::BitWriter<'a, u32, MAPR_SPEC, bool, O>;
#[doc = "Field `PTP_PPS_REMAP` reader - Ethernet PTP PPS remapping"]
pub type PTP_PPS_REMAP_R = crate::BitReader<bool>;
#[doc = "Field `PTP_PPS_REMAP` writer - Ethernet PTP PPS remapping"]
pub type PTP_PPS_REMAP_W<'a, const O: u8> = crate::BitWriter<'a, u32, MAPR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - SPI1 remapping"]
    #[inline(always)]
    pub fn spi1_remap(&self) -> SPI1_REMAP_R {
        SPI1_REMAP_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - I2C1 remapping"]
    #[inline(always)]
    pub fn i2c1_remap(&self) -> I2C1_REMAP_R {
        I2C1_REMAP_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - USART1 remapping"]
    #[inline(always)]
    pub fn usart1_remap(&self) -> USART1_REMAP_R {
        USART1_REMAP_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - USART2 remapping"]
    #[inline(always)]
    pub fn usart2_remap(&self) -> USART2_REMAP_R {
        USART2_REMAP_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - USART3 remapping"]
    #[inline(always)]
    pub fn usart3_remap(&self) -> USART3_REMAP_R {
        USART3_REMAP_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - TIM1 remapping"]
    #[inline(always)]
    pub fn tim1_remap(&self) -> TIM1_REMAP_R {
        TIM1_REMAP_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - TIM2 remapping"]
    #[inline(always)]
    pub fn tim2_remap(&self) -> TIM2_REMAP_R {
        TIM2_REMAP_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - TIM3 remapping"]
    #[inline(always)]
    pub fn tim3_remap(&self) -> TIM3_REMAP_R {
        TIM3_REMAP_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bit 12 - TIM4 remapping"]
    #[inline(always)]
    pub fn tim4_remap(&self) -> TIM4_REMAP_R {
        TIM4_REMAP_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:14 - CAN1 remapping"]
    #[inline(always)]
    pub fn can1_remap(&self) -> CAN1_REMAP_R {
        CAN1_REMAP_R::new(((self.bits >> 13) & 3) as u8)
    }
    #[doc = "Bit 15 - Port D0/Port D1 mapping on OSCIN/OSCOUT"]
    #[inline(always)]
    pub fn pd01_remap(&self) -> PD01_REMAP_R {
        PD01_REMAP_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Set and cleared by software"]
    #[inline(always)]
    pub fn tim5ch4_iremap(&self) -> TIM5CH4_IREMAP_R {
        TIM5CH4_IREMAP_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 21 - Ethernet MAC I/O remapping"]
    #[inline(always)]
    pub fn eth_remap(&self) -> ETH_REMAP_R {
        ETH_REMAP_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - CAN2 I/O remapping"]
    #[inline(always)]
    pub fn can2_remap(&self) -> CAN2_REMAP_R {
        CAN2_REMAP_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - MII or RMII selection"]
    #[inline(always)]
    pub fn mii_rmii_sel(&self) -> MII_RMII_SEL_R {
        MII_RMII_SEL_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 28 - SPI3/I2S3 remapping"]
    #[inline(always)]
    pub fn spi3_remap(&self) -> SPI3_REMAP_R {
        SPI3_REMAP_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - TIM2 internal trigger 1 remapping"]
    #[inline(always)]
    pub fn tim2itr1_iremap(&self) -> TIM2ITR1_IREMAP_R {
        TIM2ITR1_IREMAP_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Ethernet PTP PPS remapping"]
    #[inline(always)]
    pub fn ptp_pps_remap(&self) -> PTP_PPS_REMAP_R {
        PTP_PPS_REMAP_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SPI1 remapping"]
    #[inline(always)]
    #[must_use]
    pub fn spi1_remap(&mut self) -> SPI1_REMAP_W<0> {
        SPI1_REMAP_W::new(self)
    }
    #[doc = "Bit 1 - I2C1 remapping"]
    #[inline(always)]
    #[must_use]
    pub fn i2c1_remap(&mut self) -> I2C1_REMAP_W<1> {
        I2C1_REMAP_W::new(self)
    }
    #[doc = "Bit 2 - USART1 remapping"]
    #[inline(always)]
    #[must_use]
    pub fn usart1_remap(&mut self) -> USART1_REMAP_W<2> {
        USART1_REMAP_W::new(self)
    }
    #[doc = "Bit 3 - USART2 remapping"]
    #[inline(always)]
    #[must_use]
    pub fn usart2_remap(&mut self) -> USART2_REMAP_W<3> {
        USART2_REMAP_W::new(self)
    }
    #[doc = "Bits 4:5 - USART3 remapping"]
    #[inline(always)]
    #[must_use]
    pub fn usart3_remap(&mut self) -> USART3_REMAP_W<4> {
        USART3_REMAP_W::new(self)
    }
    #[doc = "Bits 6:7 - TIM1 remapping"]
    #[inline(always)]
    #[must_use]
    pub fn tim1_remap(&mut self) -> TIM1_REMAP_W<6> {
        TIM1_REMAP_W::new(self)
    }
    #[doc = "Bits 8:9 - TIM2 remapping"]
    #[inline(always)]
    #[must_use]
    pub fn tim2_remap(&mut self) -> TIM2_REMAP_W<8> {
        TIM2_REMAP_W::new(self)
    }
    #[doc = "Bits 10:11 - TIM3 remapping"]
    #[inline(always)]
    #[must_use]
    pub fn tim3_remap(&mut self) -> TIM3_REMAP_W<10> {
        TIM3_REMAP_W::new(self)
    }
    #[doc = "Bit 12 - TIM4 remapping"]
    #[inline(always)]
    #[must_use]
    pub fn tim4_remap(&mut self) -> TIM4_REMAP_W<12> {
        TIM4_REMAP_W::new(self)
    }
    #[doc = "Bits 13:14 - CAN1 remapping"]
    #[inline(always)]
    #[must_use]
    pub fn can1_remap(&mut self) -> CAN1_REMAP_W<13> {
        CAN1_REMAP_W::new(self)
    }
    #[doc = "Bit 15 - Port D0/Port D1 mapping on OSCIN/OSCOUT"]
    #[inline(always)]
    #[must_use]
    pub fn pd01_remap(&mut self) -> PD01_REMAP_W<15> {
        PD01_REMAP_W::new(self)
    }
    #[doc = "Bit 16 - Set and cleared by software"]
    #[inline(always)]
    #[must_use]
    pub fn tim5ch4_iremap(&mut self) -> TIM5CH4_IREMAP_W<16> {
        TIM5CH4_IREMAP_W::new(self)
    }
    #[doc = "Bit 21 - Ethernet MAC I/O remapping"]
    #[inline(always)]
    #[must_use]
    pub fn eth_remap(&mut self) -> ETH_REMAP_W<21> {
        ETH_REMAP_W::new(self)
    }
    #[doc = "Bit 22 - CAN2 I/O remapping"]
    #[inline(always)]
    #[must_use]
    pub fn can2_remap(&mut self) -> CAN2_REMAP_W<22> {
        CAN2_REMAP_W::new(self)
    }
    #[doc = "Bit 23 - MII or RMII selection"]
    #[inline(always)]
    #[must_use]
    pub fn mii_rmii_sel(&mut self) -> MII_RMII_SEL_W<23> {
        MII_RMII_SEL_W::new(self)
    }
    #[doc = "Bits 24:26 - Serial wire JTAG configuration"]
    #[inline(always)]
    #[must_use]
    pub fn swj_cfg(&mut self) -> SWJ_CFG_W<24> {
        SWJ_CFG_W::new(self)
    }
    #[doc = "Bit 28 - SPI3/I2S3 remapping"]
    #[inline(always)]
    #[must_use]
    pub fn spi3_remap(&mut self) -> SPI3_REMAP_W<28> {
        SPI3_REMAP_W::new(self)
    }
    #[doc = "Bit 29 - TIM2 internal trigger 1 remapping"]
    #[inline(always)]
    #[must_use]
    pub fn tim2itr1_iremap(&mut self) -> TIM2ITR1_IREMAP_W<29> {
        TIM2ITR1_IREMAP_W::new(self)
    }
    #[doc = "Bit 30 - Ethernet PTP PPS remapping"]
    #[inline(always)]
    #[must_use]
    pub fn ptp_pps_remap(&mut self) -> PTP_PPS_REMAP_W<30> {
        PTP_PPS_REMAP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AF remap and debug I/O configuration register (AFIO_MAPR)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mapr](index.html) module"]
pub struct MAPR_SPEC;
impl crate::RegisterSpec for MAPR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mapr::R](R) reader structure"]
impl crate::Readable for MAPR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mapr::W](W) writer structure"]
impl crate::Writable for MAPR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MAPR to value 0"]
impl crate::Resettable for MAPR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
