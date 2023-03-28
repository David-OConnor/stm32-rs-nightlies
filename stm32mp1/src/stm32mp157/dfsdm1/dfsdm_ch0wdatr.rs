#[doc = "Register `DFSDM_CH0WDATR` reader"]
pub struct R(crate::R<DFSDM_CH0WDATR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DFSDM_CH0WDATR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DFSDM_CH0WDATR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DFSDM_CH0WDATR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `WDATA` reader - WDATA"]
pub type WDATA_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - WDATA"]
    #[inline(always)]
    pub fn wdata(&self) -> WDATA_R {
        WDATA_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "This register contains the data resulting from the analog watchdog filter associated to the input channel y.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfsdm_ch0wdatr](index.html) module"]
pub struct DFSDM_CH0WDATR_SPEC;
impl crate::RegisterSpec for DFSDM_CH0WDATR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dfsdm_ch0wdatr::R](R) reader structure"]
impl crate::Readable for DFSDM_CH0WDATR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DFSDM_CH0WDATR to value 0"]
impl crate::Resettable for DFSDM_CH0WDATR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
