#[doc = "Register `FDCAN_TXESC` reader"]
pub struct R(crate::R<FDCAN_TXESC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FDCAN_TXESC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FDCAN_TXESC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FDCAN_TXESC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TBDS` reader - TBDS"]
pub type TBDS_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:2 - TBDS"]
    #[inline(always)]
    pub fn tbds(&self) -> TBDS_R {
        TBDS_R::new((self.bits & 7) as u8)
    }
}
#[doc = "Configures the number of data bytes belonging to a Tx buffer element. Data field sizes &gt;8 bytes are intended for CAN FD operation only.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fdcan_txesc](index.html) module"]
pub struct FDCAN_TXESC_SPEC;
impl crate::RegisterSpec for FDCAN_TXESC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fdcan_txesc::R](R) reader structure"]
impl crate::Readable for FDCAN_TXESC_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets FDCAN_TXESC to value 0"]
impl crate::Resettable for FDCAN_TXESC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
