#[doc = "Register `I3C_RDR` reader"]
pub struct R(crate::R<I3C_RDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<I3C_RDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<I3C_RDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<I3C_RDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RDB0` reader - 8-bit received data on I3C bus."]
pub type RDB0_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - 8-bit received data on I3C bus."]
    #[inline(always)]
    pub fn rdb0(&self) -> RDB0_R {
        RDB0_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "I3C receive data byte register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i3c_rdr](index.html) module"]
pub struct I3C_RDR_SPEC;
impl crate::RegisterSpec for I3C_RDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [i3c_rdr::R](R) reader structure"]
impl crate::Readable for I3C_RDR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets I3C_RDR to value 0"]
impl crate::Resettable for I3C_RDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
