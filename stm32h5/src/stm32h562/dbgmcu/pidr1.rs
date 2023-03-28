#[doc = "Register `PIDR1` reader"]
pub struct R(crate::R<PIDR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PIDR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PIDR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PIDR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `PARTNUM` reader - part number bits \\[11:8\\]"]
pub type PARTNUM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `JEP106ID` reader - JEP106 identity code bits \\[3:0\\]"]
pub type JEP106ID_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:3 - part number bits \\[11:8\\]"]
    #[inline(always)]
    pub fn partnum(&self) -> PARTNUM_R {
        PARTNUM_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - JEP106 identity code bits \\[3:0\\]"]
    #[inline(always)]
    pub fn jep106id(&self) -> JEP106ID_R {
        JEP106ID_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
#[doc = "DBGMCU CoreSight peripheral identity register 1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pidr1](index.html) module"]
pub struct PIDR1_SPEC;
impl crate::RegisterSpec for PIDR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pidr1::R](R) reader structure"]
impl crate::Readable for PIDR1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PIDR1 to value 0"]
impl crate::Resettable for PIDR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
