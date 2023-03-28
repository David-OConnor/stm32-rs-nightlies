#[doc = "Register `DDRPERFM_STATUS` reader"]
pub struct R(crate::R<DDRPERFM_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DDRPERFM_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DDRPERFM_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DDRPERFM_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `COVF` reader - COVF"]
pub type COVF_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BUSY` reader - BUSY"]
pub type BUSY_R = crate::BitReader<bool>;
#[doc = "Field `TOVF` reader - TOVF"]
pub type TOVF_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bits 0:3 - COVF"]
    #[inline(always)]
    pub fn covf(&self) -> COVF_R {
        COVF_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 16 - BUSY"]
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 31 - TOVF"]
    #[inline(always)]
    pub fn tovf(&self) -> TOVF_R {
        TOVF_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "DDRPERFM status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrperfm_status](index.html) module"]
pub struct DDRPERFM_STATUS_SPEC;
impl crate::RegisterSpec for DDRPERFM_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ddrperfm_status::R](R) reader structure"]
impl crate::Readable for DDRPERFM_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DDRPERFM_STATUS to value 0"]
impl crate::Resettable for DDRPERFM_STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
