#[doc = "Register `ADC_JDR4` reader"]
pub struct R(crate::R<ADC_JDR4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADC_JDR4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADC_JDR4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADC_JDR4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `JDATA` reader - JDATA"]
pub type JDATA_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - JDATA"]
    #[inline(always)]
    pub fn jdata(&self) -> JDATA_R {
        JDATA_R::new(self.bits)
    }
}
#[doc = "ADC injected data register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc_jdr4](index.html) module"]
pub struct ADC_JDR4_SPEC;
impl crate::RegisterSpec for ADC_JDR4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adc_jdr4::R](R) reader structure"]
impl crate::Readable for ADC_JDR4_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ADC_JDR4 to value 0"]
impl crate::Resettable for ADC_JDR4_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
