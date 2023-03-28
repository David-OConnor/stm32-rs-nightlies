#[doc = "Register `FDCAN_ENDN` reader"]
pub struct R(crate::R<FDCAN_ENDN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FDCAN_ENDN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FDCAN_ENDN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FDCAN_ENDN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `ETV` reader - Endiannes Test Value"]
pub type ETV_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Endiannes Test Value"]
    #[inline(always)]
    pub fn etv(&self) -> ETV_R {
        ETV_R::new(self.bits)
    }
}
#[doc = "FDCAN Core Release Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fdcan_endn](index.html) module"]
pub struct FDCAN_ENDN_SPEC;
impl crate::RegisterSpec for FDCAN_ENDN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fdcan_endn::R](R) reader structure"]
impl crate::Readable for FDCAN_ENDN_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets FDCAN_ENDN to value 0x8765_4321"]
impl crate::Resettable for FDCAN_ENDN_SPEC {
    const RESET_VALUE: Self::Ux = 0x8765_4321;
}
