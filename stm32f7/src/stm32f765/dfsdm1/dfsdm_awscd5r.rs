#[doc = "Register `DFSDM_AWSCD5R` reader"]
pub struct R(crate::R<DFSDM_AWSCD5R_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DFSDM_AWSCD5R_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DFSDM_AWSCD5R_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DFSDM_AWSCD5R_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DFSDM_AWSCD5R` writer"]
pub struct W(crate::W<DFSDM_AWSCD5R_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DFSDM_AWSCD5R_SPEC>;
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
impl From<crate::W<DFSDM_AWSCD5R_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DFSDM_AWSCD5R_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SCDT` reader - short-circuit detector threshold for channel 5"]
pub type SCDT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SCDT` writer - short-circuit detector threshold for channel 5"]
pub type SCDT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DFSDM_AWSCD5R_SPEC, u8, u8, 8, O>;
#[doc = "Field `BKSCD` reader - Break signal assignment for short-circuit detector on channel 5"]
pub type BKSCD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BKSCD` writer - Break signal assignment for short-circuit detector on channel 5"]
pub type BKSCD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DFSDM_AWSCD5R_SPEC, u8, u8, 4, O>;
#[doc = "Field `AWFOSR` reader - Analog watchdog filter oversampling ratio (decimation rate) on channel 5"]
pub type AWFOSR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `AWFOSR` writer - Analog watchdog filter oversampling ratio (decimation rate) on channel 5"]
pub type AWFOSR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DFSDM_AWSCD5R_SPEC, u8, u8, 5, O>;
#[doc = "Field `AWFORD` reader - Analog watchdog Sinc filter order on channel 5"]
pub type AWFORD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `AWFORD` writer - Analog watchdog Sinc filter order on channel 5"]
pub type AWFORD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DFSDM_AWSCD5R_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 0:7 - short-circuit detector threshold for channel 5"]
    #[inline(always)]
    pub fn scdt(&self) -> SCDT_R {
        SCDT_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 12:15 - Break signal assignment for short-circuit detector on channel 5"]
    #[inline(always)]
    pub fn bkscd(&self) -> BKSCD_R {
        BKSCD_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:20 - Analog watchdog filter oversampling ratio (decimation rate) on channel 5"]
    #[inline(always)]
    pub fn awfosr(&self) -> AWFOSR_R {
        AWFOSR_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 22:23 - Analog watchdog Sinc filter order on channel 5"]
    #[inline(always)]
    pub fn awford(&self) -> AWFORD_R {
        AWFORD_R::new(((self.bits >> 22) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - short-circuit detector threshold for channel 5"]
    #[inline(always)]
    #[must_use]
    pub fn scdt(&mut self) -> SCDT_W<0> {
        SCDT_W::new(self)
    }
    #[doc = "Bits 12:15 - Break signal assignment for short-circuit detector on channel 5"]
    #[inline(always)]
    #[must_use]
    pub fn bkscd(&mut self) -> BKSCD_W<12> {
        BKSCD_W::new(self)
    }
    #[doc = "Bits 16:20 - Analog watchdog filter oversampling ratio (decimation rate) on channel 5"]
    #[inline(always)]
    #[must_use]
    pub fn awfosr(&mut self) -> AWFOSR_W<16> {
        AWFOSR_W::new(self)
    }
    #[doc = "Bits 22:23 - Analog watchdog Sinc filter order on channel 5"]
    #[inline(always)]
    #[must_use]
    pub fn awford(&mut self) -> AWFORD_W<22> {
        AWFORD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DFSDM analog watchdog and short-circuit detector register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfsdm_awscd5r](index.html) module"]
pub struct DFSDM_AWSCD5R_SPEC;
impl crate::RegisterSpec for DFSDM_AWSCD5R_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dfsdm_awscd5r::R](R) reader structure"]
impl crate::Readable for DFSDM_AWSCD5R_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dfsdm_awscd5r::W](W) writer structure"]
impl crate::Writable for DFSDM_AWSCD5R_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DFSDM_AWSCD5R to value 0"]
impl crate::Resettable for DFSDM_AWSCD5R_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
