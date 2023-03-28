#[doc = "Register `CORDIC_RDATA` reader"]
pub struct R(crate::R<CORDIC_RDATA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CORDIC_RDATA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CORDIC_RDATA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CORDIC_RDATA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RES` reader - Function result If 32-bit format is selected (CORDIC_CSR.RESSIZE = 0) and two output values are expected (CORDIC_CSR.NRES = 1), this register must be read twice when the RRDY flag is set. The first read fetches the primary result (RES1). The second read fetches the secondary result (RES2) and resets RRDY. If 32-bit format is selected and only one output value is expected (NRES = 0), only one read of this register is required to fetch the primary result (RES1) and reset the RRDY flag. If 16-bit format is selected (CORDIC_CSR.RESSIZE = 1), this register contains the primary result (RES1) in the lower half, RES\\[15:0\\], and the secondary result (RES2) in the upper half, RES\\[31:16\\]. In this case, NRES must be set to 0, and only one read performed. A read from this register resets the RRDY flag in the CORDIC_CSR register."]
pub type RES_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Function result If 32-bit format is selected (CORDIC_CSR.RESSIZE = 0) and two output values are expected (CORDIC_CSR.NRES = 1), this register must be read twice when the RRDY flag is set. The first read fetches the primary result (RES1). The second read fetches the secondary result (RES2) and resets RRDY. If 32-bit format is selected and only one output value is expected (NRES = 0), only one read of this register is required to fetch the primary result (RES1) and reset the RRDY flag. If 16-bit format is selected (CORDIC_CSR.RESSIZE = 1), this register contains the primary result (RES1) in the lower half, RES\\[15:0\\], and the secondary result (RES2) in the upper half, RES\\[31:16\\]. In this case, NRES must be set to 0, and only one read performed. A read from this register resets the RRDY flag in the CORDIC_CSR register."]
    #[inline(always)]
    pub fn res(&self) -> RES_R {
        RES_R::new(self.bits)
    }
}
#[doc = "CORDIC result register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cordic_rdata](index.html) module"]
pub struct CORDIC_RDATA_SPEC;
impl crate::RegisterSpec for CORDIC_RDATA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cordic_rdata::R](R) reader structure"]
impl crate::Readable for CORDIC_RDATA_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CORDIC_RDATA to value 0"]
impl crate::Resettable for CORDIC_RDATA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
