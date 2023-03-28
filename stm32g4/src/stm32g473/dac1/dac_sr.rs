#[doc = "Register `DAC_SR` reader"]
pub struct R(crate::R<DAC_SR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DAC_SR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DAC_SR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DAC_SR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DAC_SR` writer"]
pub struct W(crate::W<DAC_SR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DAC_SR_SPEC>;
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
impl From<crate::W<DAC_SR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DAC_SR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DAC1RDY` reader - DAC channel1 ready status bit"]
pub type DAC1RDY_R = crate::BitReader<bool>;
#[doc = "Field `DAC1RDY` writer - DAC channel1 ready status bit"]
pub type DAC1RDY_W<'a, const O: u8> = crate::BitWriter<'a, u32, DAC_SR_SPEC, bool, O>;
#[doc = "Field `DORSTAT1` reader - DAC channel1 output register status bit"]
pub type DORSTAT1_R = crate::BitReader<bool>;
#[doc = "Field `DORSTAT1` writer - DAC channel1 output register status bit"]
pub type DORSTAT1_W<'a, const O: u8> = crate::BitWriter<'a, u32, DAC_SR_SPEC, bool, O>;
#[doc = "Field `DMAUDR1` reader - DAC channel1 DMA underrun flag This bit is set by hardware and cleared by software (by writing it to 1)."]
pub type DMAUDR1_R = crate::BitReader<bool>;
#[doc = "Field `DMAUDR1` writer - DAC channel1 DMA underrun flag This bit is set by hardware and cleared by software (by writing it to 1)."]
pub type DMAUDR1_W<'a, const O: u8> = crate::BitWriter<'a, u32, DAC_SR_SPEC, bool, O>;
#[doc = "Field `CAL_FLAG1` reader - DAC Channel 1 calibration offset status This bit is set and cleared by hardware"]
pub type CAL_FLAG1_R = crate::BitReader<bool>;
#[doc = "Field `BWST1` reader - DAC Channel 1 busy writing sample time flag This bit is systematically set just after Sample & Hold mode enable and is set each time the software writes the register DAC_SHSR1, It is cleared by hardware when the write operation of DAC_SHSR1 is complete. (It takes about 3LSI periods of synchronization)."]
pub type BWST1_R = crate::BitReader<bool>;
#[doc = "Field `DAC2RDY` reader - DAC channel 2 ready status bit"]
pub type DAC2RDY_R = crate::BitReader<bool>;
#[doc = "Field `DAC2RDY` writer - DAC channel 2 ready status bit"]
pub type DAC2RDY_W<'a, const O: u8> = crate::BitWriter<'a, u32, DAC_SR_SPEC, bool, O>;
#[doc = "Field `DORSTAT2` reader - DAC channel 2 output register status bit"]
pub type DORSTAT2_R = crate::BitReader<bool>;
#[doc = "Field `DORSTAT2` writer - DAC channel 2 output register status bit"]
pub type DORSTAT2_W<'a, const O: u8> = crate::BitWriter<'a, u32, DAC_SR_SPEC, bool, O>;
#[doc = "Field `DMAUDR2` reader - DAC channel2 DMA underrun flag This bit is set by hardware and cleared by software (by writing it to 1)."]
pub type DMAUDR2_R = crate::BitReader<bool>;
#[doc = "Field `DMAUDR2` writer - DAC channel2 DMA underrun flag This bit is set by hardware and cleared by software (by writing it to 1)."]
pub type DMAUDR2_W<'a, const O: u8> = crate::BitWriter<'a, u32, DAC_SR_SPEC, bool, O>;
#[doc = "Field `CAL_FLAG2` reader - DAC Channel 2 calibration offset status This bit is set and cleared by hardware"]
pub type CAL_FLAG2_R = crate::BitReader<bool>;
#[doc = "Field `BWST2` reader - DAC Channel 2 busy writing sample time flag This bit is systematically set just after Sample & Hold mode enable and is set each time the software writes the register DAC_SHSR2, It is cleared by hardware when the write operation of DAC_SHSR2 is complete. (It takes about 3 LSI periods of synchronization)."]
pub type BWST2_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 11 - DAC channel1 ready status bit"]
    #[inline(always)]
    pub fn dac1rdy(&self) -> DAC1RDY_R {
        DAC1RDY_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - DAC channel1 output register status bit"]
    #[inline(always)]
    pub fn dorstat1(&self) -> DORSTAT1_R {
        DORSTAT1_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - DAC channel1 DMA underrun flag This bit is set by hardware and cleared by software (by writing it to 1)."]
    #[inline(always)]
    pub fn dmaudr1(&self) -> DMAUDR1_R {
        DMAUDR1_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - DAC Channel 1 calibration offset status This bit is set and cleared by hardware"]
    #[inline(always)]
    pub fn cal_flag1(&self) -> CAL_FLAG1_R {
        CAL_FLAG1_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - DAC Channel 1 busy writing sample time flag This bit is systematically set just after Sample & Hold mode enable and is set each time the software writes the register DAC_SHSR1, It is cleared by hardware when the write operation of DAC_SHSR1 is complete. (It takes about 3LSI periods of synchronization)."]
    #[inline(always)]
    pub fn bwst1(&self) -> BWST1_R {
        BWST1_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 27 - DAC channel 2 ready status bit"]
    #[inline(always)]
    pub fn dac2rdy(&self) -> DAC2RDY_R {
        DAC2RDY_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - DAC channel 2 output register status bit"]
    #[inline(always)]
    pub fn dorstat2(&self) -> DORSTAT2_R {
        DORSTAT2_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - DAC channel2 DMA underrun flag This bit is set by hardware and cleared by software (by writing it to 1)."]
    #[inline(always)]
    pub fn dmaudr2(&self) -> DMAUDR2_R {
        DMAUDR2_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - DAC Channel 2 calibration offset status This bit is set and cleared by hardware"]
    #[inline(always)]
    pub fn cal_flag2(&self) -> CAL_FLAG2_R {
        CAL_FLAG2_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - DAC Channel 2 busy writing sample time flag This bit is systematically set just after Sample & Hold mode enable and is set each time the software writes the register DAC_SHSR2, It is cleared by hardware when the write operation of DAC_SHSR2 is complete. (It takes about 3 LSI periods of synchronization)."]
    #[inline(always)]
    pub fn bwst2(&self) -> BWST2_R {
        BWST2_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 11 - DAC channel1 ready status bit"]
    #[inline(always)]
    #[must_use]
    pub fn dac1rdy(&mut self) -> DAC1RDY_W<11> {
        DAC1RDY_W::new(self)
    }
    #[doc = "Bit 12 - DAC channel1 output register status bit"]
    #[inline(always)]
    #[must_use]
    pub fn dorstat1(&mut self) -> DORSTAT1_W<12> {
        DORSTAT1_W::new(self)
    }
    #[doc = "Bit 13 - DAC channel1 DMA underrun flag This bit is set by hardware and cleared by software (by writing it to 1)."]
    #[inline(always)]
    #[must_use]
    pub fn dmaudr1(&mut self) -> DMAUDR1_W<13> {
        DMAUDR1_W::new(self)
    }
    #[doc = "Bit 27 - DAC channel 2 ready status bit"]
    #[inline(always)]
    #[must_use]
    pub fn dac2rdy(&mut self) -> DAC2RDY_W<27> {
        DAC2RDY_W::new(self)
    }
    #[doc = "Bit 28 - DAC channel 2 output register status bit"]
    #[inline(always)]
    #[must_use]
    pub fn dorstat2(&mut self) -> DORSTAT2_W<28> {
        DORSTAT2_W::new(self)
    }
    #[doc = "Bit 29 - DAC channel2 DMA underrun flag This bit is set by hardware and cleared by software (by writing it to 1)."]
    #[inline(always)]
    #[must_use]
    pub fn dmaudr2(&mut self) -> DMAUDR2_W<29> {
        DMAUDR2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DAC status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dac_sr](index.html) module"]
pub struct DAC_SR_SPEC;
impl crate::RegisterSpec for DAC_SR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dac_sr::R](R) reader structure"]
impl crate::Readable for DAC_SR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dac_sr::W](W) writer structure"]
impl crate::Writable for DAC_SR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DAC_SR to value 0"]
impl crate::Resettable for DAC_SR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
