#[doc = "Register `DFSDM_FLT1RDATAR` reader"]
pub struct R(crate::R<DFSDM_FLT1RDATAR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DFSDM_FLT1RDATAR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DFSDM_FLT1RDATAR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DFSDM_FLT1RDATAR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RDATACH` reader - RDATACH"]
pub type RDATACH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RPEND` reader - RPEND"]
pub type RPEND_R = crate::BitReader<bool>;
#[doc = "Field `RDATA` reader - RDATA"]
pub type RDATA_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:2 - RDATACH"]
    #[inline(always)]
    pub fn rdatach(&self) -> RDATACH_R {
        RDATACH_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 4 - RPEND"]
    #[inline(always)]
    pub fn rpend(&self) -> RPEND_R {
        RPEND_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 8:31 - RDATA"]
    #[inline(always)]
    pub fn rdata(&self) -> RDATA_R {
        RDATA_R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
#[doc = "DFSDM filter 1 data register for the regular channel\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfsdm_flt1rdatar](index.html) module"]
pub struct DFSDM_FLT1RDATAR_SPEC;
impl crate::RegisterSpec for DFSDM_FLT1RDATAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dfsdm_flt1rdatar::R](R) reader structure"]
impl crate::Readable for DFSDM_FLT1RDATAR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DFSDM_FLT1RDATAR to value 0"]
impl crate::Resettable for DFSDM_FLT1RDATAR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}