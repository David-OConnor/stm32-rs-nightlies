#[doc = "Register `RX_ORDSETR` reader"]
pub struct R(crate::R<RX_ORDSETR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RX_ORDSETR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RX_ORDSETR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RX_ORDSETR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RXORDSET` reader - Rx ordered set code detected"]
pub type RXORDSET_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RXSOP3OF4` reader - The bit indicates the number of correct K‑codes. For debug purposes only."]
pub type RXSOP3OF4_R = crate::BitReader<bool>;
#[doc = "Field `RXSOPKINVALID` reader - The bitfield is for debug purposes only. Others: Invalid"]
pub type RXSOPKINVALID_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:2 - Rx ordered set code detected"]
    #[inline(always)]
    pub fn rxordset(&self) -> RXORDSET_R {
        RXORDSET_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - The bit indicates the number of correct K‑codes. For debug purposes only."]
    #[inline(always)]
    pub fn rxsop3of4(&self) -> RXSOP3OF4_R {
        RXSOP3OF4_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - The bitfield is for debug purposes only. Others: Invalid"]
    #[inline(always)]
    pub fn rxsopkinvalid(&self) -> RXSOPKINVALID_R {
        RXSOPKINVALID_R::new(((self.bits >> 4) & 7) as u8)
    }
}
#[doc = "UCPD Rx ordered set register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rx_ordsetr](index.html) module"]
pub struct RX_ORDSETR_SPEC;
impl crate::RegisterSpec for RX_ORDSETR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rx_ordsetr::R](R) reader structure"]
impl crate::Readable for RX_ORDSETR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RX_ORDSETR to value 0"]
impl crate::Resettable for RX_ORDSETR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
