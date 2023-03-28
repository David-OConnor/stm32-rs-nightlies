#[doc = "Register `FDCAN_RXF1S` reader"]
pub struct R(crate::R<FDCAN_RXF1S_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FDCAN_RXF1S_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FDCAN_RXF1S_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FDCAN_RXF1S_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `F1FL` reader - F1FL"]
pub type F1FL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `F1GI` reader - F1GI"]
pub type F1GI_R = crate::FieldReader<u8, u8>;
#[doc = "Field `F1PI` reader - F1PI"]
pub type F1PI_R = crate::FieldReader<u8, u8>;
#[doc = "Field `F1F` reader - F1F"]
pub type F1F_R = crate::BitReader<bool>;
#[doc = "Field `RF1L` reader - RF1L"]
pub type RF1L_R = crate::BitReader<bool>;
#[doc = "Field `DMS` reader - DMS"]
pub type DMS_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:6 - F1FL"]
    #[inline(always)]
    pub fn f1fl(&self) -> F1FL_R {
        F1FL_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:13 - F1GI"]
    #[inline(always)]
    pub fn f1gi(&self) -> F1GI_R {
        F1GI_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:21 - F1PI"]
    #[inline(always)]
    pub fn f1pi(&self) -> F1PI_R {
        F1PI_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bit 24 - F1F"]
    #[inline(always)]
    pub fn f1f(&self) -> F1F_R {
        F1F_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - RF1L"]
    #[inline(always)]
    pub fn rf1l(&self) -> RF1L_R {
        RF1L_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bits 30:31 - DMS"]
    #[inline(always)]
    pub fn dms(&self) -> DMS_R {
        DMS_R::new(((self.bits >> 30) & 3) as u8)
    }
}
#[doc = "FDCAN Rx FIFO 1 status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fdcan_rxf1s](index.html) module"]
pub struct FDCAN_RXF1S_SPEC;
impl crate::RegisterSpec for FDCAN_RXF1S_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fdcan_rxf1s::R](R) reader structure"]
impl crate::Readable for FDCAN_RXF1S_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets FDCAN_RXF1S to value 0"]
impl crate::Resettable for FDCAN_RXF1S_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
