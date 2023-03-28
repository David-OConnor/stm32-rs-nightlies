#[doc = "Register `RESP4R` reader"]
pub struct R(crate::R<RESP4R_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RESP4R_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RESP4R_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RESP4R_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CARDSTATUS4` reader - see Table 347"]
pub type CARDSTATUS4_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - see Table 347"]
    #[inline(always)]
    pub fn cardstatus4(&self) -> CARDSTATUS4_R {
        CARDSTATUS4_R::new(self.bits)
    }
}
#[doc = "response 1..4 register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [resp4r](index.html) module"]
pub struct RESP4R_SPEC;
impl crate::RegisterSpec for RESP4R_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [resp4r::R](R) reader structure"]
impl crate::Readable for RESP4R_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RESP4R to value 0"]
impl crate::Resettable for RESP4R_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
