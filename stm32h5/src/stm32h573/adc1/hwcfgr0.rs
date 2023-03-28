#[doc = "Register `HWCFGR0` reader"]
pub struct R(crate::R<HWCFGR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HWCFGR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HWCFGR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HWCFGR0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `ADCNUM` reader - Number of ADCs implemented"]
pub type ADCNUM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MULPIPE` reader - Number of pipeline stages"]
pub type MULPIPE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `OPBITS` reader - Number of option bits"]
pub type OPBITS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `IDLEVALUE` reader - Idle value for non-selected channels"]
pub type IDLEVALUE_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:3 - Number of ADCs implemented"]
    #[inline(always)]
    pub fn adcnum(&self) -> ADCNUM_R {
        ADCNUM_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Number of pipeline stages"]
    #[inline(always)]
    pub fn mulpipe(&self) -> MULPIPE_R {
        MULPIPE_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Number of option bits"]
    #[inline(always)]
    pub fn opbits(&self) -> OPBITS_R {
        OPBITS_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Idle value for non-selected channels"]
    #[inline(always)]
    pub fn idlevalue(&self) -> IDLEVALUE_R {
        IDLEVALUE_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
#[doc = "ADC hardware configuration register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hwcfgr0](index.html) module"]
pub struct HWCFGR0_SPEC;
impl crate::RegisterSpec for HWCFGR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hwcfgr0::R](R) reader structure"]
impl crate::Readable for HWCFGR0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets HWCFGR0 to value 0x1112"]
impl crate::Resettable for HWCFGR0_SPEC {
    const RESET_VALUE: Self::Ux = 0x1112;
}
