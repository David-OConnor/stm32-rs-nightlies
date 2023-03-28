#[doc = "Register `SWTRGR` writer"]
pub struct W(crate::W<SWTRGR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SWTRGR_SPEC>;
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
impl From<crate::W<SWTRGR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SWTRGR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SWTRIG1` writer - DAC channel1 software trigger This bit is set by software to trigger the DAC in software trigger mode. Note: This bit is cleared by hardware (one dac_pclk clock cycle later) once the DAC_DHR1 register value has been loaded into the DAC_DOR1 register."]
pub type SWTRIG1_W<'a, const O: u8> = crate::BitWriter<'a, u32, SWTRGR_SPEC, bool, O>;
#[doc = "Field `SWTRIG2` writer - DAC channel2 software trigger This bit is set by software to trigger the DAC in software trigger mode. Note: This bit is cleared by hardware (one dac_pclk clock cycle later) once the DAC_DHR2 register value has been loaded into the DAC_DOR2 register. This bit is available only on dual-channel DACs. Refer to implementation."]
pub type SWTRIG2_W<'a, const O: u8> = crate::BitWriter<'a, u32, SWTRGR_SPEC, bool, O>;
impl W {
    #[doc = "Bit 0 - DAC channel1 software trigger This bit is set by software to trigger the DAC in software trigger mode. Note: This bit is cleared by hardware (one dac_pclk clock cycle later) once the DAC_DHR1 register value has been loaded into the DAC_DOR1 register."]
    #[inline(always)]
    #[must_use]
    pub fn swtrig1(&mut self) -> SWTRIG1_W<0> {
        SWTRIG1_W::new(self)
    }
    #[doc = "Bit 1 - DAC channel2 software trigger This bit is set by software to trigger the DAC in software trigger mode. Note: This bit is cleared by hardware (one dac_pclk clock cycle later) once the DAC_DHR2 register value has been loaded into the DAC_DOR2 register. This bit is available only on dual-channel DACs. Refer to implementation."]
    #[inline(always)]
    #[must_use]
    pub fn swtrig2(&mut self) -> SWTRIG2_W<1> {
        SWTRIG2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DAC software trigger register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [swtrgr](index.html) module"]
pub struct SWTRGR_SPEC;
impl crate::RegisterSpec for SWTRGR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [swtrgr::W](W) writer structure"]
impl crate::Writable for SWTRGR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SWTRGR to value 0"]
impl crate::Resettable for SWTRGR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
