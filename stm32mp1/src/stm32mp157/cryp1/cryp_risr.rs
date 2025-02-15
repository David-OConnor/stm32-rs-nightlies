#[doc = "Register `CRYP_RISR` reader"]
pub struct R(crate::R<CRYP_RISR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CRYP_RISR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CRYP_RISR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CRYP_RISR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `INRIS` reader - INRIS"]
pub type INRIS_R = crate::BitReader<bool>;
#[doc = "Field `OUTRIS` reader - OUTRIS"]
pub type OUTRIS_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - INRIS"]
    #[inline(always)]
    pub fn inris(&self) -> INRIS_R {
        INRIS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - OUTRIS"]
    #[inline(always)]
    pub fn outris(&self) -> OUTRIS_R {
        OUTRIS_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[doc = "The CRYP_RISR register is the raw interrupt status register. It is a read-only register. When a read operation is performed, this register gives the current raw status of the corresponding interrupt, i.e. the interrupt information without taking CRYP_IMSCR mask into account. Write operations have no effect.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cryp_risr](index.html) module"]
pub struct CRYP_RISR_SPEC;
impl crate::RegisterSpec for CRYP_RISR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cryp_risr::R](R) reader structure"]
impl crate::Readable for CRYP_RISR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CRYP_RISR to value 0x01"]
impl crate::Resettable for CRYP_RISR_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
