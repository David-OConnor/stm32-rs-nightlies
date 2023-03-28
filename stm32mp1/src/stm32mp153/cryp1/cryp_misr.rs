#[doc = "Register `CRYP_MISR` reader"]
pub struct R(crate::R<CRYP_MISR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CRYP_MISR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CRYP_MISR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CRYP_MISR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `INMIS` reader - INMIS"]
pub type INMIS_R = crate::BitReader<bool>;
#[doc = "Field `OUTMIS` reader - OUTMIS"]
pub type OUTMIS_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - INMIS"]
    #[inline(always)]
    pub fn inmis(&self) -> INMIS_R {
        INMIS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - OUTMIS"]
    #[inline(always)]
    pub fn outmis(&self) -> OUTMIS_R {
        OUTMIS_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[doc = "The CRYP_MISR register is the masked interrupt status register. It is a read-only register. When a read operation is performed, this register gives the current masked status of the corresponding interrupt, i.e. the interrupt information taking CRYP_IMSCR mask into account. Write operations have no effect.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cryp_misr](index.html) module"]
pub struct CRYP_MISR_SPEC;
impl crate::RegisterSpec for CRYP_MISR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cryp_misr::R](R) reader structure"]
impl crate::Readable for CRYP_MISR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CRYP_MISR to value 0"]
impl crate::Resettable for CRYP_MISR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
