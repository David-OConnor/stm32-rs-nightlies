#[doc = "Register `JDR1` reader"]
pub struct R(crate::R<JDR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<JDR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<JDR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<JDR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `JDATA1` reader - ADC group injected sequencer rank 1 conversion data"]
pub type JDATA1_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - ADC group injected sequencer rank 1 conversion data"]
    #[inline(always)]
    pub fn jdata1(&self) -> JDATA1_R {
        JDATA1_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "ADC group injected sequencer rank 1 register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [jdr1](index.html) module"]
pub struct JDR1_SPEC;
impl crate::RegisterSpec for JDR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [jdr1::R](R) reader structure"]
impl crate::Readable for JDR1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets JDR1 to value 0"]
impl crate::Resettable for JDR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
