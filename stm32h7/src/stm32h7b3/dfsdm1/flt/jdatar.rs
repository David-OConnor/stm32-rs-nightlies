#[doc = "Register `JDATAR` reader"]
pub struct R(crate::R<JDATAR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<JDATAR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<JDATAR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<JDATAR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `JDATACH` reader - Injected channel most recently converted When each conversion of a channel in the injected group finishes, JDATACH\\[2:0\\]
is updated to indicate which channel was converted. Thus, JDATA\\[23:0\\]
holds the data that corresponds to the channel indicated by JDATACH\\[2:0\\]."]
pub type JDATACH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `JDATA` reader - Injected group conversion data When each conversion of a channel in the injected group finishes, its resulting data is stored in this field. The data is valid when JEOCF=1. Reading this register clears the corresponding JEOCF."]
pub type JDATA_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:2 - Injected channel most recently converted When each conversion of a channel in the injected group finishes, JDATACH\\[2:0\\]
is updated to indicate which channel was converted. Thus, JDATA\\[23:0\\]
holds the data that corresponds to the channel indicated by JDATACH\\[2:0\\]."]
    #[inline(always)]
    pub fn jdatach(&self) -> JDATACH_R {
        JDATACH_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 8:31 - Injected group conversion data When each conversion of a channel in the injected group finishes, its resulting data is stored in this field. The data is valid when JEOCF=1. Reading this register clears the corresponding JEOCF."]
    #[inline(always)]
    pub fn jdata(&self) -> JDATA_R {
        JDATA_R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [jdatar](index.html) module"]
pub struct JDATAR_SPEC;
impl crate::RegisterSpec for JDATAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [jdatar::R](R) reader structure"]
impl crate::Readable for JDATAR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets JDATAR to value 0"]
impl crate::Resettable for JDATAR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
