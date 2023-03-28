#[doc = "Register `OTG_HFNUM` reader"]
pub struct R(crate::R<OTG_HFNUM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OTG_HFNUM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OTG_HFNUM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OTG_HFNUM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `FRNUM` reader - FRNUM"]
pub type FRNUM_R = crate::FieldReader<u16, u16>;
#[doc = "Field `FTREM` reader - FTREM"]
pub type FTREM_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - FRNUM"]
    #[inline(always)]
    pub fn frnum(&self) -> FRNUM_R {
        FRNUM_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - FTREM"]
    #[inline(always)]
    pub fn ftrem(&self) -> FTREM_R {
        FTREM_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "This register indicates the current frame number. It also indicates the time remaining (in terms of the number of PHY clocks) in the current frame.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hfnum](index.html) module"]
pub struct OTG_HFNUM_SPEC;
impl crate::RegisterSpec for OTG_HFNUM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [otg_hfnum::R](R) reader structure"]
impl crate::Readable for OTG_HFNUM_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets OTG_HFNUM to value 0x3fff"]
impl crate::Resettable for OTG_HFNUM_SPEC {
    const RESET_VALUE: Self::Ux = 0x3fff;
}
