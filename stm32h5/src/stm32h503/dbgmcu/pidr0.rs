#[doc = "Register `PIDR0` reader"]
pub struct R(crate::R<PIDR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PIDR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PIDR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PIDR0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `PARTNUM` reader - part number bits \\[7:0\\]"]
pub type PARTNUM_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - part number bits \\[7:0\\]"]
    #[inline(always)]
    pub fn partnum(&self) -> PARTNUM_R {
        PARTNUM_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "DBGMCU CoreSight peripheral identity register 0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pidr0](index.html) module"]
pub struct PIDR0_SPEC;
impl crate::RegisterSpec for PIDR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pidr0::R](R) reader structure"]
impl crate::Readable for PIDR0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PIDR0 to value 0"]
impl crate::Resettable for PIDR0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
