#[doc = "Register `PIDR2` reader"]
pub struct R(crate::R<PIDR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PIDR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PIDR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PIDR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `JEP106ID` reader - JEP106 identity code bits \\[6:4\\]"]
pub type JEP106ID_R = crate::FieldReader<u8, u8>;
#[doc = "Field `JEDEC` reader - JEDEC assigned value"]
pub type JEDEC_R = crate::BitReader<bool>;
#[doc = "Field `REVISION` reader - component revision number"]
pub type REVISION_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:2 - JEP106 identity code bits \\[6:4\\]"]
    #[inline(always)]
    pub fn jep106id(&self) -> JEP106ID_R {
        JEP106ID_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - JEDEC assigned value"]
    #[inline(always)]
    pub fn jedec(&self) -> JEDEC_R {
        JEDEC_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7 - component revision number"]
    #[inline(always)]
    pub fn revision(&self) -> REVISION_R {
        REVISION_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
#[doc = "DBGMCU CoreSight peripheral identity register 2\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pidr2](index.html) module"]
pub struct PIDR2_SPEC;
impl crate::RegisterSpec for PIDR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pidr2::R](R) reader structure"]
impl crate::Readable for PIDR2_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PIDR2 to value 0x0a"]
impl crate::Resettable for PIDR2_SPEC {
    const RESET_VALUE: Self::Ux = 0x0a;
}
