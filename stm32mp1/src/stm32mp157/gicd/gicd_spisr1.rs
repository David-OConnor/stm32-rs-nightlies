#[doc = "Register `GICD_SPISR1` reader"]
pub struct R(crate::R<GICD_SPISR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GICD_SPISR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GICD_SPISR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GICD_SPISR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SPISR1` reader - SPISR1"]
pub type SPISR1_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - SPISR1"]
    #[inline(always)]
    pub fn spisr1(&self) -> SPISR1_R {
        SPISR1_R::new(self.bits)
    }
}
#[doc = "For interrupts ID = SPI number+32, from SPI \\[x*32+31\\]
to SPI \\[x*32\\]\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_spisr1](index.html) module"]
pub struct GICD_SPISR1_SPEC;
impl crate::RegisterSpec for GICD_SPISR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gicd_spisr1::R](R) reader structure"]
impl crate::Readable for GICD_SPISR1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets GICD_SPISR1 to value 0"]
impl crate::Resettable for GICD_SPISR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
