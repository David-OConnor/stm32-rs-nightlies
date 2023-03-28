#[doc = "Register `DFSDM_FLT1ISR` reader"]
pub struct R(crate::R<DFSDM_FLT1ISR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DFSDM_FLT1ISR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DFSDM_FLT1ISR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DFSDM_FLT1ISR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `JEOCF` reader - JEOCF"]
pub type JEOCF_R = crate::BitReader<bool>;
#[doc = "Field `REOCF` reader - REOCF"]
pub type REOCF_R = crate::BitReader<bool>;
#[doc = "Field `JOVRF` reader - JOVRF"]
pub type JOVRF_R = crate::BitReader<bool>;
#[doc = "Field `ROVRF` reader - ROVRF"]
pub type ROVRF_R = crate::BitReader<bool>;
#[doc = "Field `AWDF` reader - AWDF"]
pub type AWDF_R = crate::BitReader<bool>;
#[doc = "Field `JCIP` reader - JCIP"]
pub type JCIP_R = crate::BitReader<bool>;
#[doc = "Field `RCIP` reader - RCIP"]
pub type RCIP_R = crate::BitReader<bool>;
#[doc = "Field `CKABF` reader - CKABF"]
pub type CKABF_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SCDF` reader - SCDF"]
pub type SCDF_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bit 0 - JEOCF"]
    #[inline(always)]
    pub fn jeocf(&self) -> JEOCF_R {
        JEOCF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - REOCF"]
    #[inline(always)]
    pub fn reocf(&self) -> REOCF_R {
        REOCF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - JOVRF"]
    #[inline(always)]
    pub fn jovrf(&self) -> JOVRF_R {
        JOVRF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - ROVRF"]
    #[inline(always)]
    pub fn rovrf(&self) -> ROVRF_R {
        ROVRF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - AWDF"]
    #[inline(always)]
    pub fn awdf(&self) -> AWDF_R {
        AWDF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 13 - JCIP"]
    #[inline(always)]
    pub fn jcip(&self) -> JCIP_R {
        JCIP_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - RCIP"]
    #[inline(always)]
    pub fn rcip(&self) -> RCIP_R {
        RCIP_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bits 16:23 - CKABF"]
    #[inline(always)]
    pub fn ckabf(&self) -> CKABF_R {
        CKABF_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - SCDF"]
    #[inline(always)]
    pub fn scdf(&self) -> SCDF_R {
        SCDF_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[doc = "DFSDM filter 1 interrupt and status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfsdm_flt1isr](index.html) module"]
pub struct DFSDM_FLT1ISR_SPEC;
impl crate::RegisterSpec for DFSDM_FLT1ISR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dfsdm_flt1isr::R](R) reader structure"]
impl crate::Readable for DFSDM_FLT1ISR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DFSDM_FLT1ISR to value 0x00ff_0000"]
impl crate::Resettable for DFSDM_FLT1ISR_SPEC {
    const RESET_VALUE: Self::Ux = 0x00ff_0000;
}
