#[doc = "Register `FLT0RDATAR` reader"]
pub struct R(crate::R<FLT0RDATAR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FLT0RDATAR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FLT0RDATAR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FLT0RDATAR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RDATACH` reader - Regular channel most recently converted"]
pub type RDATACH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RPEND` reader - Regular channel pending data"]
pub type RPEND_R = crate::BitReader<bool>;
#[doc = "Field `RDATA` reader - Regular channel conversion data"]
pub type RDATA_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:2 - Regular channel most recently converted"]
    #[inline(always)]
    pub fn rdatach(&self) -> RDATACH_R {
        RDATACH_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 4 - Regular channel pending data"]
    #[inline(always)]
    pub fn rpend(&self) -> RPEND_R {
        RPEND_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 8:31 - Regular channel conversion data"]
    #[inline(always)]
    pub fn rdata(&self) -> RDATA_R {
        RDATA_R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
#[doc = "data register for the regular channel\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flt0rdatar](index.html) module"]
pub struct FLT0RDATAR_SPEC;
impl crate::RegisterSpec for FLT0RDATAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [flt0rdatar::R](R) reader structure"]
impl crate::Readable for FLT0RDATAR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets FLT0RDATAR to value 0"]
impl crate::Resettable for FLT0RDATAR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
