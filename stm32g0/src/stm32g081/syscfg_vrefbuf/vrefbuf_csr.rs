#[doc = "Register `VREFBUF_CSR` reader"]
pub struct R(crate::R<VREFBUF_CSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<VREFBUF_CSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<VREFBUF_CSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<VREFBUF_CSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `VREFBUF_CSR` writer"]
pub struct W(crate::W<VREFBUF_CSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<VREFBUF_CSR_SPEC>;
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
impl From<crate::W<VREFBUF_CSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<VREFBUF_CSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ENVR` reader - Voltage reference buffer mode enable This bit is used to enable the voltage reference buffer mode."]
pub type ENVR_R = crate::BitReader<bool>;
#[doc = "Field `ENVR` writer - Voltage reference buffer mode enable This bit is used to enable the voltage reference buffer mode."]
pub type ENVR_W<'a, const O: u8> = crate::BitWriter<'a, u32, VREFBUF_CSR_SPEC, bool, O>;
#[doc = "Field `HIZ` reader - High impedance mode This bit controls the analog switch to connect or not the VREF+ pin. Refer to Table196: VREF buffer modes for the mode descriptions depending on ENVR bit configuration."]
pub type HIZ_R = crate::BitReader<bool>;
#[doc = "Field `HIZ` writer - High impedance mode This bit controls the analog switch to connect or not the VREF+ pin. Refer to Table196: VREF buffer modes for the mode descriptions depending on ENVR bit configuration."]
pub type HIZ_W<'a, const O: u8> = crate::BitWriter<'a, u32, VREFBUF_CSR_SPEC, bool, O>;
#[doc = "Field `VRR` reader - Voltage reference buffer ready"]
pub type VRR_R = crate::BitReader<bool>;
#[doc = "Field `VRS` reader - Voltage reference scale These bits select the value generated by the voltage reference buffer. Other: Reserved"]
pub type VRS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `VRS` writer - Voltage reference scale These bits select the value generated by the voltage reference buffer. Other: Reserved"]
pub type VRS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, VREFBUF_CSR_SPEC, u8, u8, 3, O>;
impl R {
    #[doc = "Bit 0 - Voltage reference buffer mode enable This bit is used to enable the voltage reference buffer mode."]
    #[inline(always)]
    pub fn envr(&self) -> ENVR_R {
        ENVR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - High impedance mode This bit controls the analog switch to connect or not the VREF+ pin. Refer to Table196: VREF buffer modes for the mode descriptions depending on ENVR bit configuration."]
    #[inline(always)]
    pub fn hiz(&self) -> HIZ_R {
        HIZ_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - Voltage reference buffer ready"]
    #[inline(always)]
    pub fn vrr(&self) -> VRR_R {
        VRR_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - Voltage reference scale These bits select the value generated by the voltage reference buffer. Other: Reserved"]
    #[inline(always)]
    pub fn vrs(&self) -> VRS_R {
        VRS_R::new(((self.bits >> 4) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Voltage reference buffer mode enable This bit is used to enable the voltage reference buffer mode."]
    #[inline(always)]
    #[must_use]
    pub fn envr(&mut self) -> ENVR_W<0> {
        ENVR_W::new(self)
    }
    #[doc = "Bit 1 - High impedance mode This bit controls the analog switch to connect or not the VREF+ pin. Refer to Table196: VREF buffer modes for the mode descriptions depending on ENVR bit configuration."]
    #[inline(always)]
    #[must_use]
    pub fn hiz(&mut self) -> HIZ_W<1> {
        HIZ_W::new(self)
    }
    #[doc = "Bits 4:6 - Voltage reference scale These bits select the value generated by the voltage reference buffer. Other: Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn vrs(&mut self) -> VRS_W<4> {
        VRS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "VREFBUF control and status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [vrefbuf_csr](index.html) module"]
pub struct VREFBUF_CSR_SPEC;
impl crate::RegisterSpec for VREFBUF_CSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [vrefbuf_csr::R](R) reader structure"]
impl crate::Readable for VREFBUF_CSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [vrefbuf_csr::W](W) writer structure"]
impl crate::Writable for VREFBUF_CSR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets VREFBUF_CSR to value 0x02"]
impl crate::Resettable for VREFBUF_CSR_SPEC {
    const RESET_VALUE: Self::Ux = 0x02;
}