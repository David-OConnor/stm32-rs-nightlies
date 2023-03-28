#[doc = "Register `DTS_CFGR1` reader"]
pub struct R(crate::R<DTS_CFGR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DTS_CFGR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DTS_CFGR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DTS_CFGR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DTS_CFGR1` writer"]
pub struct W(crate::W<DTS_CFGR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DTS_CFGR1_SPEC>;
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
impl From<crate::W<DTS_CFGR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DTS_CFGR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TS1_EN` reader - Temperature sensor 1 enable bit This bit is set and cleared by software. Note: Once enabled, the temperature sensor is active after a specific delay time. The TS1_RDY flag will be set when the sensor is ready."]
pub type TS1_EN_R = crate::BitReader<bool>;
#[doc = "Field `TS1_EN` writer - Temperature sensor 1 enable bit This bit is set and cleared by software. Note: Once enabled, the temperature sensor is active after a specific delay time. The TS1_RDY flag will be set when the sensor is ready."]
pub type TS1_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, DTS_CFGR1_SPEC, bool, O>;
#[doc = "Field `TS1_START` reader - Start frequency measurement on temperature sensor 1 This bit is set and cleared by software."]
pub type TS1_START_R = crate::BitReader<bool>;
#[doc = "Field `TS1_START` writer - Start frequency measurement on temperature sensor 1 This bit is set and cleared by software."]
pub type TS1_START_W<'a, const O: u8> = crate::BitWriter<'a, u32, DTS_CFGR1_SPEC, bool, O>;
#[doc = "Field `TS1_INTRIG_SEL` reader - Input trigger selection bit for temperature sensor 1 These bits are set and cleared by software. They select which input triggers a temperature measurement. Refer to Section 19.3.10: Trigger input."]
pub type TS1_INTRIG_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TS1_INTRIG_SEL` writer - Input trigger selection bit for temperature sensor 1 These bits are set and cleared by software. They select which input triggers a temperature measurement. Refer to Section 19.3.10: Trigger input."]
pub type TS1_INTRIG_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DTS_CFGR1_SPEC, u8, u8, 4, O>;
#[doc = "Field `TS1_SMP_TIME` reader - Sampling time for temperature sensor 1 These bits allow increasing the sampling time to improve measurement precision. When the PCLK clock is selected as reference clock (REFCLK_SEL = 0), the measurement will be performed at TS1_SMP_TIME period of CLK_PTAT. When the LSE is selected as reference clock (REFCLK_SEL =1), the measurement will be performed at TS1_SMP_TIME period of LSE."]
pub type TS1_SMP_TIME_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TS1_SMP_TIME` writer - Sampling time for temperature sensor 1 These bits allow increasing the sampling time to improve measurement precision. When the PCLK clock is selected as reference clock (REFCLK_SEL = 0), the measurement will be performed at TS1_SMP_TIME period of CLK_PTAT. When the LSE is selected as reference clock (REFCLK_SEL =1), the measurement will be performed at TS1_SMP_TIME period of LSE."]
pub type TS1_SMP_TIME_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DTS_CFGR1_SPEC, u8, u8, 4, O>;
#[doc = "Field `REFCLK_SEL` reader - Reference clock selection bit This bit is set and cleared by software. It indicates whether the reference clock is the high speed clock (PCLK) or the low speed clock (LSE)."]
pub type REFCLK_SEL_R = crate::BitReader<bool>;
#[doc = "Field `REFCLK_SEL` writer - Reference clock selection bit This bit is set and cleared by software. It indicates whether the reference clock is the high speed clock (PCLK) or the low speed clock (LSE)."]
pub type REFCLK_SEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, DTS_CFGR1_SPEC, bool, O>;
#[doc = "Field `Q_MEAS_OPT` reader - Quick measurement option bit This bit is set and cleared by software. It is used to increase the measurement speed by suppressing the calibration step. It is effective only when the LSE clock is used as reference clock (REFCLK_SEL=1)."]
pub type Q_MEAS_OPT_R = crate::BitReader<bool>;
#[doc = "Field `Q_MEAS_OPT` writer - Quick measurement option bit This bit is set and cleared by software. It is used to increase the measurement speed by suppressing the calibration step. It is effective only when the LSE clock is used as reference clock (REFCLK_SEL=1)."]
pub type Q_MEAS_OPT_W<'a, const O: u8> = crate::BitWriter<'a, u32, DTS_CFGR1_SPEC, bool, O>;
#[doc = "Field `HSREF_CLK_DIV` reader - High speed clock division ratio These bits are set and cleared by software. They can be used to define the division ratio for the main clock in order to obtain the internal frequency lower than 1 MHz required for the calibration. They are applicable only for calibration when PCLK is selected as reference clock (REFCLK_SEL=0). ..."]
pub type HSREF_CLK_DIV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `HSREF_CLK_DIV` writer - High speed clock division ratio These bits are set and cleared by software. They can be used to define the division ratio for the main clock in order to obtain the internal frequency lower than 1 MHz required for the calibration. They are applicable only for calibration when PCLK is selected as reference clock (REFCLK_SEL=0). ..."]
pub type HSREF_CLK_DIV_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DTS_CFGR1_SPEC, u8, u8, 7, O>;
impl R {
    #[doc = "Bit 0 - Temperature sensor 1 enable bit This bit is set and cleared by software. Note: Once enabled, the temperature sensor is active after a specific delay time. The TS1_RDY flag will be set when the sensor is ready."]
    #[inline(always)]
    pub fn ts1_en(&self) -> TS1_EN_R {
        TS1_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - Start frequency measurement on temperature sensor 1 This bit is set and cleared by software."]
    #[inline(always)]
    pub fn ts1_start(&self) -> TS1_START_R {
        TS1_START_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 8:11 - Input trigger selection bit for temperature sensor 1 These bits are set and cleared by software. They select which input triggers a temperature measurement. Refer to Section 19.3.10: Trigger input."]
    #[inline(always)]
    pub fn ts1_intrig_sel(&self) -> TS1_INTRIG_SEL_R {
        TS1_INTRIG_SEL_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Sampling time for temperature sensor 1 These bits allow increasing the sampling time to improve measurement precision. When the PCLK clock is selected as reference clock (REFCLK_SEL = 0), the measurement will be performed at TS1_SMP_TIME period of CLK_PTAT. When the LSE is selected as reference clock (REFCLK_SEL =1), the measurement will be performed at TS1_SMP_TIME period of LSE."]
    #[inline(always)]
    pub fn ts1_smp_time(&self) -> TS1_SMP_TIME_R {
        TS1_SMP_TIME_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 20 - Reference clock selection bit This bit is set and cleared by software. It indicates whether the reference clock is the high speed clock (PCLK) or the low speed clock (LSE)."]
    #[inline(always)]
    pub fn refclk_sel(&self) -> REFCLK_SEL_R {
        REFCLK_SEL_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Quick measurement option bit This bit is set and cleared by software. It is used to increase the measurement speed by suppressing the calibration step. It is effective only when the LSE clock is used as reference clock (REFCLK_SEL=1)."]
    #[inline(always)]
    pub fn q_meas_opt(&self) -> Q_MEAS_OPT_R {
        Q_MEAS_OPT_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bits 24:30 - High speed clock division ratio These bits are set and cleared by software. They can be used to define the division ratio for the main clock in order to obtain the internal frequency lower than 1 MHz required for the calibration. They are applicable only for calibration when PCLK is selected as reference clock (REFCLK_SEL=0). ..."]
    #[inline(always)]
    pub fn hsref_clk_div(&self) -> HSREF_CLK_DIV_R {
        HSREF_CLK_DIV_R::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Temperature sensor 1 enable bit This bit is set and cleared by software. Note: Once enabled, the temperature sensor is active after a specific delay time. The TS1_RDY flag will be set when the sensor is ready."]
    #[inline(always)]
    #[must_use]
    pub fn ts1_en(&mut self) -> TS1_EN_W<0> {
        TS1_EN_W::new(self)
    }
    #[doc = "Bit 4 - Start frequency measurement on temperature sensor 1 This bit is set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn ts1_start(&mut self) -> TS1_START_W<4> {
        TS1_START_W::new(self)
    }
    #[doc = "Bits 8:11 - Input trigger selection bit for temperature sensor 1 These bits are set and cleared by software. They select which input triggers a temperature measurement. Refer to Section 19.3.10: Trigger input."]
    #[inline(always)]
    #[must_use]
    pub fn ts1_intrig_sel(&mut self) -> TS1_INTRIG_SEL_W<8> {
        TS1_INTRIG_SEL_W::new(self)
    }
    #[doc = "Bits 16:19 - Sampling time for temperature sensor 1 These bits allow increasing the sampling time to improve measurement precision. When the PCLK clock is selected as reference clock (REFCLK_SEL = 0), the measurement will be performed at TS1_SMP_TIME period of CLK_PTAT. When the LSE is selected as reference clock (REFCLK_SEL =1), the measurement will be performed at TS1_SMP_TIME period of LSE."]
    #[inline(always)]
    #[must_use]
    pub fn ts1_smp_time(&mut self) -> TS1_SMP_TIME_W<16> {
        TS1_SMP_TIME_W::new(self)
    }
    #[doc = "Bit 20 - Reference clock selection bit This bit is set and cleared by software. It indicates whether the reference clock is the high speed clock (PCLK) or the low speed clock (LSE)."]
    #[inline(always)]
    #[must_use]
    pub fn refclk_sel(&mut self) -> REFCLK_SEL_W<20> {
        REFCLK_SEL_W::new(self)
    }
    #[doc = "Bit 21 - Quick measurement option bit This bit is set and cleared by software. It is used to increase the measurement speed by suppressing the calibration step. It is effective only when the LSE clock is used as reference clock (REFCLK_SEL=1)."]
    #[inline(always)]
    #[must_use]
    pub fn q_meas_opt(&mut self) -> Q_MEAS_OPT_W<21> {
        Q_MEAS_OPT_W::new(self)
    }
    #[doc = "Bits 24:30 - High speed clock division ratio These bits are set and cleared by software. They can be used to define the division ratio for the main clock in order to obtain the internal frequency lower than 1 MHz required for the calibration. They are applicable only for calibration when PCLK is selected as reference clock (REFCLK_SEL=0). ..."]
    #[inline(always)]
    #[must_use]
    pub fn hsref_clk_div(&mut self) -> HSREF_CLK_DIV_W<24> {
        HSREF_CLK_DIV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Temperature sensor configuration register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dts_cfgr1](index.html) module"]
pub struct DTS_CFGR1_SPEC;
impl crate::RegisterSpec for DTS_CFGR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dts_cfgr1::R](R) reader structure"]
impl crate::Readable for DTS_CFGR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dts_cfgr1::W](W) writer structure"]
impl crate::Writable for DTS_CFGR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DTS_CFGR1 to value 0"]
impl crate::Resettable for DTS_CFGR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
