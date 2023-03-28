#[doc = "Register `TIM3_CR2` reader"]
pub struct R(crate::R<TIM3_CR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIM3_CR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIM3_CR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIM3_CR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TIM3_CR2` writer"]
pub struct W(crate::W<TIM3_CR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIM3_CR2_SPEC>;
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
impl From<crate::W<TIM3_CR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIM3_CR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CCDS` reader - Capture/compare DMA selection"]
pub type CCDS_R = crate::BitReader<bool>;
#[doc = "Field `CCDS` writer - Capture/compare DMA selection"]
pub type CCDS_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIM3_CR2_SPEC, bool, O>;
#[doc = "Field `MMS1` reader - Master mode selection These bits allow to select the information to be sent in master mode to slave timers for synchronization (tim_trgo). The combination is as follows: tim_trgo, except if the master/slave mode is selected (see the MSM bit description in TIMx_SMCR register). Others: Reserved Note: The clock of the slave timer or ADC must be enabled prior to receive events from the master timer, and must not be changed on-the-fly while triggers are received from the master timer."]
pub type MMS1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MMS1` writer - Master mode selection These bits allow to select the information to be sent in master mode to slave timers for synchronization (tim_trgo). The combination is as follows: tim_trgo, except if the master/slave mode is selected (see the MSM bit description in TIMx_SMCR register). Others: Reserved Note: The clock of the slave timer or ADC must be enabled prior to receive events from the master timer, and must not be changed on-the-fly while triggers are received from the master timer."]
pub type MMS1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TIM3_CR2_SPEC, u8, u8, 3, O>;
#[doc = "Field `TI1S` reader - tim_ti1 selection"]
pub type TI1S_R = crate::BitReader<bool>;
#[doc = "Field `TI1S` writer - tim_ti1 selection"]
pub type TI1S_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIM3_CR2_SPEC, bool, O>;
#[doc = "Field `MMS2` reader - Master mode selection These bits allow to select the information to be sent in master mode to slave timers for synchronization (tim_trgo). The combination is as follows: tim_trgo, except if the master/slave mode is selected (see the MSM bit description in TIMx_SMCR register). Others: Reserved Note: The clock of the slave timer or ADC must be enabled prior to receive events from the master timer, and must not be changed on-the-fly while triggers are received from the master timer."]
pub type MMS2_R = crate::BitReader<bool>;
#[doc = "Field `MMS2` writer - Master mode selection These bits allow to select the information to be sent in master mode to slave timers for synchronization (tim_trgo). The combination is as follows: tim_trgo, except if the master/slave mode is selected (see the MSM bit description in TIMx_SMCR register). Others: Reserved Note: The clock of the slave timer or ADC must be enabled prior to receive events from the master timer, and must not be changed on-the-fly while triggers are received from the master timer."]
pub type MMS2_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIM3_CR2_SPEC, bool, O>;
impl R {
    #[doc = "Bit 3 - Capture/compare DMA selection"]
    #[inline(always)]
    pub fn ccds(&self) -> CCDS_R {
        CCDS_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - Master mode selection These bits allow to select the information to be sent in master mode to slave timers for synchronization (tim_trgo). The combination is as follows: tim_trgo, except if the master/slave mode is selected (see the MSM bit description in TIMx_SMCR register). Others: Reserved Note: The clock of the slave timer or ADC must be enabled prior to receive events from the master timer, and must not be changed on-the-fly while triggers are received from the master timer."]
    #[inline(always)]
    pub fn mms1(&self) -> MMS1_R {
        MMS1_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - tim_ti1 selection"]
    #[inline(always)]
    pub fn ti1s(&self) -> TI1S_R {
        TI1S_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 25 - Master mode selection These bits allow to select the information to be sent in master mode to slave timers for synchronization (tim_trgo). The combination is as follows: tim_trgo, except if the master/slave mode is selected (see the MSM bit description in TIMx_SMCR register). Others: Reserved Note: The clock of the slave timer or ADC must be enabled prior to receive events from the master timer, and must not be changed on-the-fly while triggers are received from the master timer."]
    #[inline(always)]
    pub fn mms2(&self) -> MMS2_R {
        MMS2_R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - Capture/compare DMA selection"]
    #[inline(always)]
    #[must_use]
    pub fn ccds(&mut self) -> CCDS_W<3> {
        CCDS_W::new(self)
    }
    #[doc = "Bits 4:6 - Master mode selection These bits allow to select the information to be sent in master mode to slave timers for synchronization (tim_trgo). The combination is as follows: tim_trgo, except if the master/slave mode is selected (see the MSM bit description in TIMx_SMCR register). Others: Reserved Note: The clock of the slave timer or ADC must be enabled prior to receive events from the master timer, and must not be changed on-the-fly while triggers are received from the master timer."]
    #[inline(always)]
    #[must_use]
    pub fn mms1(&mut self) -> MMS1_W<4> {
        MMS1_W::new(self)
    }
    #[doc = "Bit 7 - tim_ti1 selection"]
    #[inline(always)]
    #[must_use]
    pub fn ti1s(&mut self) -> TI1S_W<7> {
        TI1S_W::new(self)
    }
    #[doc = "Bit 25 - Master mode selection These bits allow to select the information to be sent in master mode to slave timers for synchronization (tim_trgo). The combination is as follows: tim_trgo, except if the master/slave mode is selected (see the MSM bit description in TIMx_SMCR register). Others: Reserved Note: The clock of the slave timer or ADC must be enabled prior to receive events from the master timer, and must not be changed on-the-fly while triggers are received from the master timer."]
    #[inline(always)]
    #[must_use]
    pub fn mms2(&mut self) -> MMS2_W<25> {
        MMS2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TIM3 control register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim3_cr2](index.html) module"]
pub struct TIM3_CR2_SPEC;
impl crate::RegisterSpec for TIM3_CR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tim3_cr2::R](R) reader structure"]
impl crate::Readable for TIM3_CR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tim3_cr2::W](W) writer structure"]
impl crate::Writable for TIM3_CR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TIM3_CR2 to value 0"]
impl crate::Resettable for TIM3_CR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
