#[doc = "Register `FMAC_RDATA` reader"]
pub struct R(crate::R<FMAC_RDATA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FMAC_RDATA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FMAC_RDATA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FMAC_RDATA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RDATA` reader - Read data When a read access to this register occurs, the read data are the contents of the Y output buffer at the address offset indicated by the READ pointer. The pointer address is automatically incremented after each read access."]
pub type RDATA_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Read data When a read access to this register occurs, the read data are the contents of the Y output buffer at the address offset indicated by the READ pointer. The pointer address is automatically incremented after each read access."]
    #[inline(always)]
    pub fn rdata(&self) -> RDATA_R {
        RDATA_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "FMAC read data register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fmac_rdata](index.html) module"]
pub struct FMAC_RDATA_SPEC;
impl crate::RegisterSpec for FMAC_RDATA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fmac_rdata::R](R) reader structure"]
impl crate::Readable for FMAC_RDATA_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets FMAC_RDATA to value 0"]
impl crate::Resettable for FMAC_RDATA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
