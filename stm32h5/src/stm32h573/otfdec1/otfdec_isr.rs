#[doc = "Register `OTFDEC_ISR` reader"]
pub struct R(crate::R<OTFDEC_ISR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OTFDEC_ISR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OTFDEC_ISR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OTFDEC_ISR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SEIF` reader - Security error interrupt flag status This bit is set by hardware and read only by application. This bit is set when at least one security error has been detected. This bit is cleared when application sets in OTFDEC_ICR the corresponding bit to 1."]
pub type SEIF_R = crate::BitReader<bool>;
#[doc = "Field `XONEIF` reader - Execute-only execute-never error interrupt flag status This bit is set by hardware and read only by application. This bit is set when a read access and not an instruction fetch is detected on any encrypted region with MODE bits set to 11. Lastly, XONEIF is also set when an execute access is detected while encryption mode is enabled. This bit is cleared when application sets in OTFDEC_ICR the corresponding bit to 1."]
pub type XONEIF_R = crate::BitReader<bool>;
#[doc = "Field `KEIF` reader - Key error interrupt flag status This bit is set by hardware and read only by application. The bit is set when a read access occurs on an encrypted region, while its key registers is null or not properly initialized (KEYCRC = 0x0). This bit is cleared when the application sets in OTFDEC_ICR the corresponding bit to 1. After KEIF is set any subsequent read to the region with bad key registers returns a zeroed value. This state remains until those key registers are properly initialized (KEYCRC not zero)."]
pub type KEIF_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - Security error interrupt flag status This bit is set by hardware and read only by application. This bit is set when at least one security error has been detected. This bit is cleared when application sets in OTFDEC_ICR the corresponding bit to 1."]
    #[inline(always)]
    pub fn seif(&self) -> SEIF_R {
        SEIF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Execute-only execute-never error interrupt flag status This bit is set by hardware and read only by application. This bit is set when a read access and not an instruction fetch is detected on any encrypted region with MODE bits set to 11. Lastly, XONEIF is also set when an execute access is detected while encryption mode is enabled. This bit is cleared when application sets in OTFDEC_ICR the corresponding bit to 1."]
    #[inline(always)]
    pub fn xoneif(&self) -> XONEIF_R {
        XONEIF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Key error interrupt flag status This bit is set by hardware and read only by application. The bit is set when a read access occurs on an encrypted region, while its key registers is null or not properly initialized (KEYCRC = 0x0). This bit is cleared when the application sets in OTFDEC_ICR the corresponding bit to 1. After KEIF is set any subsequent read to the region with bad key registers returns a zeroed value. This state remains until those key registers are properly initialized (KEYCRC not zero)."]
    #[inline(always)]
    pub fn keif(&self) -> KEIF_R {
        KEIF_R::new(((self.bits >> 2) & 1) != 0)
    }
}
#[doc = "OTFDEC interrupt status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otfdec_isr](index.html) module"]
pub struct OTFDEC_ISR_SPEC;
impl crate::RegisterSpec for OTFDEC_ISR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [otfdec_isr::R](R) reader structure"]
impl crate::Readable for OTFDEC_ISR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets OTFDEC_ISR to value 0"]
impl crate::Resettable for OTFDEC_ISR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
