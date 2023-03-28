#[doc = "Register `LTDC_GC2R` reader"]
pub struct R(crate::R<LTDC_GC2R_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LTDC_GC2R_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LTDC_GC2R_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LTDC_GC2R_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `EDCEN` reader - EDCEN"]
pub type EDCEN_R = crate::BitReader<bool>;
#[doc = "Field `STSAEN` reader - STSAEN"]
pub type STSAEN_R = crate::BitReader<bool>;
#[doc = "Field `DVAEN` reader - DVAEN"]
pub type DVAEN_R = crate::BitReader<bool>;
#[doc = "Field `DPAEN` reader - DPAEN"]
pub type DPAEN_R = crate::BitReader<bool>;
#[doc = "Field `BW` reader - BW"]
pub type BW_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EDCA` reader - EDCA"]
pub type EDCA_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - EDCEN"]
    #[inline(always)]
    pub fn edcen(&self) -> EDCEN_R {
        EDCEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - STSAEN"]
    #[inline(always)]
    pub fn stsaen(&self) -> STSAEN_R {
        STSAEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DVAEN"]
    #[inline(always)]
    pub fn dvaen(&self) -> DVAEN_R {
        DVAEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DPAEN"]
    #[inline(always)]
    pub fn dpaen(&self) -> DPAEN_R {
        DPAEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - BW"]
    #[inline(always)]
    pub fn bw(&self) -> BW_R {
        BW_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - EDCA"]
    #[inline(always)]
    pub fn edca(&self) -> EDCA_R {
        EDCA_R::new(((self.bits >> 7) & 1) != 0)
    }
}
#[doc = "LTDC global configuration 2 register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltdc_gc2r](index.html) module"]
pub struct LTDC_GC2R_SPEC;
impl crate::RegisterSpec for LTDC_GC2R_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ltdc_gc2r::R](R) reader structure"]
impl crate::Readable for LTDC_GC2R_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets LTDC_GC2R to value 0x30"]
impl crate::Resettable for LTDC_GC2R_SPEC {
    const RESET_VALUE: Self::Ux = 0x30;
}
