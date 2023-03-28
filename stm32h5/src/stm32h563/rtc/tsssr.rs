#[doc = "Register `TSSSR` reader"]
pub struct R(crate::R<TSSSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TSSSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TSSSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TSSSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SS` reader - Sub second value/Synchronous binary counter values SS\\[31:0\\]
is the value of the synchronous prescaler counter when the timestamp event occurred."]
pub type SS_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Sub second value/Synchronous binary counter values SS\\[31:0\\]
is the value of the synchronous prescaler counter when the timestamp event occurred."]
    #[inline(always)]
    pub fn ss(&self) -> SS_R {
        SS_R::new(self.bits)
    }
}
#[doc = "RTC timestamp sub second register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tsssr](index.html) module"]
pub struct TSSSR_SPEC;
impl crate::RegisterSpec for TSSSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tsssr::R](R) reader structure"]
impl crate::Readable for TSSSR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets TSSSR to value 0"]
impl crate::Resettable for TSSSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
