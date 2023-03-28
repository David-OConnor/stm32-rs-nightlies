#[doc = "Register `IPCC_HWCFGR` reader"]
pub struct R(crate::R<IPCC_HWCFGR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IPCC_HWCFGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IPCC_HWCFGR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IPCC_HWCFGR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CHANNELS` reader - CHANNELS"]
pub type CHANNELS_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - CHANNELS"]
    #[inline(always)]
    pub fn channels(&self) -> CHANNELS_R {
        CHANNELS_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "IPCC Hardware configuration register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ipcc_hwcfgr](index.html) module"]
pub struct IPCC_HWCFGR_SPEC;
impl crate::RegisterSpec for IPCC_HWCFGR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ipcc_hwcfgr::R](R) reader structure"]
impl crate::Readable for IPCC_HWCFGR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets IPCC_HWCFGR to value 0x02"]
impl crate::Resettable for IPCC_HWCFGR_SPEC {
    const RESET_VALUE: Self::Ux = 0x02;
}
