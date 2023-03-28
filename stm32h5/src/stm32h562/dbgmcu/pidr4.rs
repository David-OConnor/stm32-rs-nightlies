#[doc = "Register `PIDR4` reader"]
pub struct R(crate::R<PIDR4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PIDR4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PIDR4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PIDR4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `JEP106CON` reader - JEP106 continuation code"]
pub type JEP106CON_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SIZE` reader - register file size"]
pub type SIZE_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:3 - JEP106 continuation code"]
    #[inline(always)]
    pub fn jep106con(&self) -> JEP106CON_R {
        JEP106CON_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - register file size"]
    #[inline(always)]
    pub fn size(&self) -> SIZE_R {
        SIZE_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
#[doc = "DBGMCU CoreSight peripheral identity register 4\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pidr4](index.html) module"]
pub struct PIDR4_SPEC;
impl crate::RegisterSpec for PIDR4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pidr4::R](R) reader structure"]
impl crate::Readable for PIDR4_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PIDR4 to value 0"]
impl crate::Resettable for PIDR4_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
