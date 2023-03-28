#[doc = "Register `CORDIC_CSR` reader"]
pub struct R(crate::R<CORDIC_CSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CORDIC_CSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CORDIC_CSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CORDIC_CSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CORDIC_CSR` writer"]
pub struct W(crate::W<CORDIC_CSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CORDIC_CSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<CORDIC_CSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CORDIC_CSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FUNC` reader - Function 2: Phase 3: Modulus 4: Arctangent 5: Hyperbolic cosine 6: Hyperbolic sine 7: Arctanh 8: Natural logarithm 9: Square Root"]
pub type FUNC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FUNC` writer - Function 2: Phase 3: Modulus 4: Arctangent 5: Hyperbolic cosine 6: Hyperbolic sine 7: Arctanh 8: Natural logarithm 9: Square Root"]
pub type FUNC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CORDIC_CSR_SPEC, u8, u8, 4, O>;
#[doc = "Field `PRECISION` reader - Precision required (number of iterations) To determine the number of iterations needed for a given accuracy refer to . Note that for most functions, the recommended range for this field is 3 to 6."]
pub type PRECISION_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PRECISION` writer - Precision required (number of iterations) To determine the number of iterations needed for a given accuracy refer to . Note that for most functions, the recommended range for this field is 3 to 6."]
pub type PRECISION_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CORDIC_CSR_SPEC, u8, u8, 4, O>;
#[doc = "Field `SCALE` reader - Scaling factor The value of this field indicates the scaling factor applied to the arguments and/or results. A value n implies that the arguments have been multiplied by a factor 2-n, and/or the results need to be multiplied by 2n. Refer to for the applicability of the scaling factor for each function and the appropriate range."]
pub type SCALE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SCALE` writer - Scaling factor The value of this field indicates the scaling factor applied to the arguments and/or results. A value n implies that the arguments have been multiplied by a factor 2-n, and/or the results need to be multiplied by 2n. Refer to for the applicability of the scaling factor for each function and the appropriate range."]
pub type SCALE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CORDIC_CSR_SPEC, u8, u8, 3, O>;
#[doc = "Field `IEN` reader - Enable interrupt. This bit is set and cleared by software. A read returns the current state of the bit."]
pub type IEN_R = crate::BitReader<bool>;
#[doc = "Field `IEN` writer - Enable interrupt. This bit is set and cleared by software. A read returns the current state of the bit."]
pub type IEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CORDIC_CSR_SPEC, bool, O>;
#[doc = "Field `DMAREN` reader - Enable DMA read channel This bit is set and cleared by software. A read returns the current state of the bit."]
pub type DMAREN_R = crate::BitReader<bool>;
#[doc = "Field `DMAREN` writer - Enable DMA read channel This bit is set and cleared by software. A read returns the current state of the bit."]
pub type DMAREN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CORDIC_CSR_SPEC, bool, O>;
#[doc = "Field `DMAWEN` reader - Enable DMA write channel This bit is set and cleared by software. A read returns the current state of the bit."]
pub type DMAWEN_R = crate::BitReader<bool>;
#[doc = "Field `DMAWEN` writer - Enable DMA write channel This bit is set and cleared by software. A read returns the current state of the bit."]
pub type DMAWEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CORDIC_CSR_SPEC, bool, O>;
#[doc = "Field `NRES` reader - Number of results in the CORDIC_RDATA register Reads return the current state of the bit."]
pub type NRES_R = crate::BitReader<bool>;
#[doc = "Field `NRES` writer - Number of results in the CORDIC_RDATA register Reads return the current state of the bit."]
pub type NRES_W<'a, const O: u8> = crate::BitWriter<'a, u32, CORDIC_CSR_SPEC, bool, O>;
#[doc = "Field `NARGS` reader - Number of arguments expected by the CORDIC_WDATA register Reads return the current state of the bit."]
pub type NARGS_R = crate::BitReader<bool>;
#[doc = "Field `NARGS` writer - Number of arguments expected by the CORDIC_WDATA register Reads return the current state of the bit."]
pub type NARGS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CORDIC_CSR_SPEC, bool, O>;
#[doc = "Field `RESSIZE` reader - Width of output data RESSIZE selects the number of bits used to represent output data. If 32-bit data is selected, the CORDIC_RDATA register contains results in q1.31 format. If 16-bit data is selected, the least significant half-word of CORDIC_RDATA contains the primary result (RES1) in q1.15 format, and the most significant half-word contains the secondary result (RES2), also in q1.15 format."]
pub type RESSIZE_R = crate::BitReader<bool>;
#[doc = "Field `RESSIZE` writer - Width of output data RESSIZE selects the number of bits used to represent output data. If 32-bit data is selected, the CORDIC_RDATA register contains results in q1.31 format. If 16-bit data is selected, the least significant half-word of CORDIC_RDATA contains the primary result (RES1) in q1.15 format, and the most significant half-word contains the secondary result (RES2), also in q1.15 format."]
pub type RESSIZE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CORDIC_CSR_SPEC, bool, O>;
#[doc = "Field `ARGSIZE` reader - Width of input data ARGSIZE selects the number of bits used to represent input data. If 32-bit data is selected, the CORDIC_WDATA register expects arguments in q1.31 format. If 16-bit data is selected, the CORDIC_WDATA register expects arguments in q1.15 format. The primary argument (ARG1) is written to the least significant half-word, and the secondary argument (ARG2) to the most significant half-word."]
pub type ARGSIZE_R = crate::BitReader<bool>;
#[doc = "Field `ARGSIZE` writer - Width of input data ARGSIZE selects the number of bits used to represent input data. If 32-bit data is selected, the CORDIC_WDATA register expects arguments in q1.31 format. If 16-bit data is selected, the CORDIC_WDATA register expects arguments in q1.15 format. The primary argument (ARG1) is written to the least significant half-word, and the secondary argument (ARG2) to the most significant half-word."]
pub type ARGSIZE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CORDIC_CSR_SPEC, bool, O>;
#[doc = "Field `RRDY` reader - Result ready flag This bit is set by hardware when a CORDIC operation completes. It is reset by hardware when the CORDIC_RDATA register is read (NRES+1) times. When this bit is set, if the IEN bit is also set, the CORDIC interrupt is asserted. If the DMAREN bit is set, a DMA read channel request is generated. While this bit is set, no new calculation is started."]
pub type RRDY_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bits 0:3 - Function 2: Phase 3: Modulus 4: Arctangent 5: Hyperbolic cosine 6: Hyperbolic sine 7: Arctanh 8: Natural logarithm 9: Square Root"]
    #[inline(always)]
    pub fn func(&self) -> FUNC_R {
        FUNC_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Precision required (number of iterations) To determine the number of iterations needed for a given accuracy refer to . Note that for most functions, the recommended range for this field is 3 to 6."]
    #[inline(always)]
    pub fn precision(&self) -> PRECISION_R {
        PRECISION_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:10 - Scaling factor The value of this field indicates the scaling factor applied to the arguments and/or results. A value n implies that the arguments have been multiplied by a factor 2-n, and/or the results need to be multiplied by 2n. Refer to for the applicability of the scaling factor for each function and the appropriate range."]
    #[inline(always)]
    pub fn scale(&self) -> SCALE_R {
        SCALE_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 16 - Enable interrupt. This bit is set and cleared by software. A read returns the current state of the bit."]
    #[inline(always)]
    pub fn ien(&self) -> IEN_R {
        IEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Enable DMA read channel This bit is set and cleared by software. A read returns the current state of the bit."]
    #[inline(always)]
    pub fn dmaren(&self) -> DMAREN_R {
        DMAREN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Enable DMA write channel This bit is set and cleared by software. A read returns the current state of the bit."]
    #[inline(always)]
    pub fn dmawen(&self) -> DMAWEN_R {
        DMAWEN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Number of results in the CORDIC_RDATA register Reads return the current state of the bit."]
    #[inline(always)]
    pub fn nres(&self) -> NRES_R {
        NRES_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Number of arguments expected by the CORDIC_WDATA register Reads return the current state of the bit."]
    #[inline(always)]
    pub fn nargs(&self) -> NARGS_R {
        NARGS_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Width of output data RESSIZE selects the number of bits used to represent output data. If 32-bit data is selected, the CORDIC_RDATA register contains results in q1.31 format. If 16-bit data is selected, the least significant half-word of CORDIC_RDATA contains the primary result (RES1) in q1.15 format, and the most significant half-word contains the secondary result (RES2), also in q1.15 format."]
    #[inline(always)]
    pub fn ressize(&self) -> RESSIZE_R {
        RESSIZE_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Width of input data ARGSIZE selects the number of bits used to represent input data. If 32-bit data is selected, the CORDIC_WDATA register expects arguments in q1.31 format. If 16-bit data is selected, the CORDIC_WDATA register expects arguments in q1.15 format. The primary argument (ARG1) is written to the least significant half-word, and the secondary argument (ARG2) to the most significant half-word."]
    #[inline(always)]
    pub fn argsize(&self) -> ARGSIZE_R {
        ARGSIZE_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 31 - Result ready flag This bit is set by hardware when a CORDIC operation completes. It is reset by hardware when the CORDIC_RDATA register is read (NRES+1) times. When this bit is set, if the IEN bit is also set, the CORDIC interrupt is asserted. If the DMAREN bit is set, a DMA read channel request is generated. While this bit is set, no new calculation is started."]
    #[inline(always)]
    pub fn rrdy(&self) -> RRDY_R {
        RRDY_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Function 2: Phase 3: Modulus 4: Arctangent 5: Hyperbolic cosine 6: Hyperbolic sine 7: Arctanh 8: Natural logarithm 9: Square Root"]
    #[inline(always)]
    #[must_use]
    pub fn func(&mut self) -> FUNC_W<0> {
        FUNC_W::new(self)
    }
    #[doc = "Bits 4:7 - Precision required (number of iterations) To determine the number of iterations needed for a given accuracy refer to . Note that for most functions, the recommended range for this field is 3 to 6."]
    #[inline(always)]
    #[must_use]
    pub fn precision(&mut self) -> PRECISION_W<4> {
        PRECISION_W::new(self)
    }
    #[doc = "Bits 8:10 - Scaling factor The value of this field indicates the scaling factor applied to the arguments and/or results. A value n implies that the arguments have been multiplied by a factor 2-n, and/or the results need to be multiplied by 2n. Refer to for the applicability of the scaling factor for each function and the appropriate range."]
    #[inline(always)]
    #[must_use]
    pub fn scale(&mut self) -> SCALE_W<8> {
        SCALE_W::new(self)
    }
    #[doc = "Bit 16 - Enable interrupt. This bit is set and cleared by software. A read returns the current state of the bit."]
    #[inline(always)]
    #[must_use]
    pub fn ien(&mut self) -> IEN_W<16> {
        IEN_W::new(self)
    }
    #[doc = "Bit 17 - Enable DMA read channel This bit is set and cleared by software. A read returns the current state of the bit."]
    #[inline(always)]
    #[must_use]
    pub fn dmaren(&mut self) -> DMAREN_W<17> {
        DMAREN_W::new(self)
    }
    #[doc = "Bit 18 - Enable DMA write channel This bit is set and cleared by software. A read returns the current state of the bit."]
    #[inline(always)]
    #[must_use]
    pub fn dmawen(&mut self) -> DMAWEN_W<18> {
        DMAWEN_W::new(self)
    }
    #[doc = "Bit 19 - Number of results in the CORDIC_RDATA register Reads return the current state of the bit."]
    #[inline(always)]
    #[must_use]
    pub fn nres(&mut self) -> NRES_W<19> {
        NRES_W::new(self)
    }
    #[doc = "Bit 20 - Number of arguments expected by the CORDIC_WDATA register Reads return the current state of the bit."]
    #[inline(always)]
    #[must_use]
    pub fn nargs(&mut self) -> NARGS_W<20> {
        NARGS_W::new(self)
    }
    #[doc = "Bit 21 - Width of output data RESSIZE selects the number of bits used to represent output data. If 32-bit data is selected, the CORDIC_RDATA register contains results in q1.31 format. If 16-bit data is selected, the least significant half-word of CORDIC_RDATA contains the primary result (RES1) in q1.15 format, and the most significant half-word contains the secondary result (RES2), also in q1.15 format."]
    #[inline(always)]
    #[must_use]
    pub fn ressize(&mut self) -> RESSIZE_W<21> {
        RESSIZE_W::new(self)
    }
    #[doc = "Bit 22 - Width of input data ARGSIZE selects the number of bits used to represent input data. If 32-bit data is selected, the CORDIC_WDATA register expects arguments in q1.31 format. If 16-bit data is selected, the CORDIC_WDATA register expects arguments in q1.15 format. The primary argument (ARG1) is written to the least significant half-word, and the secondary argument (ARG2) to the most significant half-word."]
    #[inline(always)]
    #[must_use]
    pub fn argsize(&mut self) -> ARGSIZE_W<22> {
        ARGSIZE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CORDIC control/status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cordic_csr](index.html) module"]
pub struct CORDIC_CSR_SPEC;
impl crate::RegisterSpec for CORDIC_CSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cordic_csr::R](R) reader structure"]
impl crate::Readable for CORDIC_CSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cordic_csr::W](W) writer structure"]
impl crate::Writable for CORDIC_CSR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CORDIC_CSR to value 0x50"]
impl crate::Resettable for CORDIC_CSR_SPEC {
    const RESET_VALUE: Self::Ux = 0x50;
}
