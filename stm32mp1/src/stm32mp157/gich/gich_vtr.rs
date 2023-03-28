#[doc = "Register `GICH_VTR` reader"]
pub struct R(crate::R<GICH_VTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GICH_VTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GICH_VTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GICH_VTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `LISTREGS` reader - LISTREGS"]
pub type LISTREGS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PREBITS` reader - PREBITS"]
pub type PREBITS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PRIBITS` reader - PRIBITS"]
pub type PRIBITS_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:4 - LISTREGS"]
    #[inline(always)]
    pub fn listregs(&self) -> LISTREGS_R {
        LISTREGS_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 26:28 - PREBITS"]
    #[inline(always)]
    pub fn prebits(&self) -> PREBITS_R {
        PREBITS_R::new(((self.bits >> 26) & 7) as u8)
    }
    #[doc = "Bits 29:31 - PRIBITS"]
    #[inline(always)]
    pub fn pribits(&self) -> PRIBITS_R {
        PRIBITS_R::new(((self.bits >> 29) & 7) as u8)
    }
}
#[doc = "GICH VGIC type register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gich_vtr](index.html) module"]
pub struct GICH_VTR_SPEC;
impl crate::RegisterSpec for GICH_VTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gich_vtr::R](R) reader structure"]
impl crate::Readable for GICH_VTR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets GICH_VTR to value 0x9000_0003"]
impl crate::Resettable for GICH_VTR_SPEC {
    const RESET_VALUE: Self::Ux = 0x9000_0003;
}
