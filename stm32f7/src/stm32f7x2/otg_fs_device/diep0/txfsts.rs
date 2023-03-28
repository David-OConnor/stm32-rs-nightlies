#[doc = "Register `TXFSTS` reader"]
pub struct R(crate::R<TXFSTS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TXFSTS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TXFSTS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TXFSTS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `INEPTFSAV` reader - IN endpoint TxFIFO space available"]
pub type INEPTFSAV_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - IN endpoint TxFIFO space available"]
    #[inline(always)]
    pub fn ineptfsav(&self) -> INEPTFSAV_R {
        INEPTFSAV_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "OTG_FS device IN endpoint transmit FIFO status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txfsts](index.html) module"]
pub struct TXFSTS_SPEC;
impl crate::RegisterSpec for TXFSTS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [txfsts::R](R) reader structure"]
impl crate::Readable for TXFSTS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets TXFSTS to value 0"]
impl crate::Resettable for TXFSTS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
