#[doc = "Register `OTG_HNPTXSTS` reader"]
pub struct R(crate::R<OTG_HNPTXSTS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OTG_HNPTXSTS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OTG_HNPTXSTS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OTG_HNPTXSTS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `NPTXFSAV` reader - NPTXFSAV"]
pub type NPTXFSAV_R = crate::FieldReader<u16, u16>;
#[doc = "Field `NPTQXSAV` reader - NPTQXSAV"]
pub type NPTQXSAV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `NPTXQTOP` reader - NPTXQTOP"]
pub type NPTXQTOP_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:15 - NPTXFSAV"]
    #[inline(always)]
    pub fn nptxfsav(&self) -> NPTXFSAV_R {
        NPTXFSAV_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:23 - NPTQXSAV"]
    #[inline(always)]
    pub fn nptqxsav(&self) -> NPTQXSAV_R {
        NPTQXSAV_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:30 - NPTXQTOP"]
    #[inline(always)]
    pub fn nptxqtop(&self) -> NPTXQTOP_R {
        NPTXQTOP_R::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
#[doc = "In device mode, this register is not valid. This read-only register contains the free space information for the non-periodic Tx FIFO and the non-periodic transmit request queue.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hnptxsts](index.html) module"]
pub struct OTG_HNPTXSTS_SPEC;
impl crate::RegisterSpec for OTG_HNPTXSTS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [otg_hnptxsts::R](R) reader structure"]
impl crate::Readable for OTG_HNPTXSTS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets OTG_HNPTXSTS to value 0x0008_0400"]
impl crate::Resettable for OTG_HNPTXSTS_SPEC {
    const RESET_VALUE: Self::Ux = 0x0008_0400;
}
