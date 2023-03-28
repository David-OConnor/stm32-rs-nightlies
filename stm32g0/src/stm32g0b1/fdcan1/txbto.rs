#[doc = "Register `TXBTO` reader"]
pub struct R(crate::R<TXBTO_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TXBTO_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TXBTO_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TXBTO_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TO` reader - Transmission occurred. Each Tx buffer has its own TO bit. The bits are set when the corresponding TXBRP bit is cleared after a successful transmission. The bits are reset when a new transmission is requested by writing a 1 to the corresponding bit of register TXBAR."]
pub type TO_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:2 - Transmission occurred. Each Tx buffer has its own TO bit. The bits are set when the corresponding TXBRP bit is cleared after a successful transmission. The bits are reset when a new transmission is requested by writing a 1 to the corresponding bit of register TXBAR."]
    #[inline(always)]
    pub fn to(&self) -> TO_R {
        TO_R::new((self.bits & 7) as u8)
    }
}
#[doc = "FDCAN Tx buffer transmission occurred register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txbto](index.html) module"]
pub struct TXBTO_SPEC;
impl crate::RegisterSpec for TXBTO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [txbto::R](R) reader structure"]
impl crate::Readable for TXBTO_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets TXBTO to value 0"]
impl crate::Resettable for TXBTO_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
