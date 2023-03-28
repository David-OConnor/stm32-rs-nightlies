#[doc = "Register `OPAMP2_CSR` reader"]
pub struct R(crate::R<OPAMP2_CSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OPAMP2_CSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OPAMP2_CSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OPAMP2_CSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OPAMP2_CSR` writer"]
pub struct W(crate::W<OPAMP2_CSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OPAMP2_CSR_SPEC>;
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
impl From<crate::W<OPAMP2_CSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OPAMP2_CSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OPAEN` reader - Operational amplifier Enable"]
pub type OPAEN_R = crate::BitReader<bool>;
#[doc = "Field `OPAEN` writer - Operational amplifier Enable"]
pub type OPAEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, OPAMP2_CSR_SPEC, bool, O>;
#[doc = "Field `OPALPM` reader - Operational amplifier Low Power Mode"]
pub type OPALPM_R = crate::BitReader<bool>;
#[doc = "Field `OPALPM` writer - Operational amplifier Low Power Mode"]
pub type OPALPM_W<'a, const O: u8> = crate::BitWriter<'a, u32, OPAMP2_CSR_SPEC, bool, O>;
#[doc = "Field `OPAMODE` reader - Operational amplifier PGA mode"]
pub type OPAMODE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `OPAMODE` writer - Operational amplifier PGA mode"]
pub type OPAMODE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OPAMP2_CSR_SPEC, u8, u8, 2, O>;
#[doc = "Field `PGA_GAIN` reader - Operational amplifier Programmable amplifier gain value"]
pub type PGA_GAIN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PGA_GAIN` writer - Operational amplifier Programmable amplifier gain value"]
pub type PGA_GAIN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OPAMP2_CSR_SPEC, u8, u8, 2, O>;
#[doc = "Field `VM_SEL` reader - Inverting input selection"]
pub type VM_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `VM_SEL` writer - Inverting input selection"]
pub type VM_SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OPAMP2_CSR_SPEC, u8, u8, 2, O>;
#[doc = "Field `VP_SEL` reader - Non inverted input selection"]
pub type VP_SEL_R = crate::BitReader<bool>;
#[doc = "Field `VP_SEL` writer - Non inverted input selection"]
pub type VP_SEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, OPAMP2_CSR_SPEC, bool, O>;
#[doc = "Field `CALON` reader - Calibration mode enabled"]
pub type CALON_R = crate::BitReader<bool>;
#[doc = "Field `CALON` writer - Calibration mode enabled"]
pub type CALON_W<'a, const O: u8> = crate::BitWriter<'a, u32, OPAMP2_CSR_SPEC, bool, O>;
#[doc = "Field `CALSEL` reader - Calibration selection"]
pub type CALSEL_R = crate::BitReader<bool>;
#[doc = "Field `CALSEL` writer - Calibration selection"]
pub type CALSEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, OPAMP2_CSR_SPEC, bool, O>;
#[doc = "Field `USERTRIM` reader - allows to switch from AOP offset trimmed values to AOP offset"]
pub type USERTRIM_R = crate::BitReader<bool>;
#[doc = "Field `USERTRIM` writer - allows to switch from AOP offset trimmed values to AOP offset"]
pub type USERTRIM_W<'a, const O: u8> = crate::BitWriter<'a, u32, OPAMP2_CSR_SPEC, bool, O>;
#[doc = "Field `CALOUT` reader - Operational amplifier calibration output"]
pub type CALOUT_R = crate::BitReader<bool>;
#[doc = "Field `CALOUT` writer - Operational amplifier calibration output"]
pub type CALOUT_W<'a, const O: u8> = crate::BitWriter<'a, u32, OPAMP2_CSR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Operational amplifier Enable"]
    #[inline(always)]
    pub fn opaen(&self) -> OPAEN_R {
        OPAEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Operational amplifier Low Power Mode"]
    #[inline(always)]
    pub fn opalpm(&self) -> OPALPM_R {
        OPALPM_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - Operational amplifier PGA mode"]
    #[inline(always)]
    pub fn opamode(&self) -> OPAMODE_R {
        OPAMODE_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Operational amplifier Programmable amplifier gain value"]
    #[inline(always)]
    pub fn pga_gain(&self) -> PGA_GAIN_R {
        PGA_GAIN_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Inverting input selection"]
    #[inline(always)]
    pub fn vm_sel(&self) -> VM_SEL_R {
        VM_SEL_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 10 - Non inverted input selection"]
    #[inline(always)]
    pub fn vp_sel(&self) -> VP_SEL_R {
        VP_SEL_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 12 - Calibration mode enabled"]
    #[inline(always)]
    pub fn calon(&self) -> CALON_R {
        CALON_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Calibration selection"]
    #[inline(always)]
    pub fn calsel(&self) -> CALSEL_R {
        CALSEL_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - allows to switch from AOP offset trimmed values to AOP offset"]
    #[inline(always)]
    pub fn usertrim(&self) -> USERTRIM_R {
        USERTRIM_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Operational amplifier calibration output"]
    #[inline(always)]
    pub fn calout(&self) -> CALOUT_R {
        CALOUT_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Operational amplifier Enable"]
    #[inline(always)]
    #[must_use]
    pub fn opaen(&mut self) -> OPAEN_W<0> {
        OPAEN_W::new(self)
    }
    #[doc = "Bit 1 - Operational amplifier Low Power Mode"]
    #[inline(always)]
    #[must_use]
    pub fn opalpm(&mut self) -> OPALPM_W<1> {
        OPALPM_W::new(self)
    }
    #[doc = "Bits 2:3 - Operational amplifier PGA mode"]
    #[inline(always)]
    #[must_use]
    pub fn opamode(&mut self) -> OPAMODE_W<2> {
        OPAMODE_W::new(self)
    }
    #[doc = "Bits 4:5 - Operational amplifier Programmable amplifier gain value"]
    #[inline(always)]
    #[must_use]
    pub fn pga_gain(&mut self) -> PGA_GAIN_W<4> {
        PGA_GAIN_W::new(self)
    }
    #[doc = "Bits 8:9 - Inverting input selection"]
    #[inline(always)]
    #[must_use]
    pub fn vm_sel(&mut self) -> VM_SEL_W<8> {
        VM_SEL_W::new(self)
    }
    #[doc = "Bit 10 - Non inverted input selection"]
    #[inline(always)]
    #[must_use]
    pub fn vp_sel(&mut self) -> VP_SEL_W<10> {
        VP_SEL_W::new(self)
    }
    #[doc = "Bit 12 - Calibration mode enabled"]
    #[inline(always)]
    #[must_use]
    pub fn calon(&mut self) -> CALON_W<12> {
        CALON_W::new(self)
    }
    #[doc = "Bit 13 - Calibration selection"]
    #[inline(always)]
    #[must_use]
    pub fn calsel(&mut self) -> CALSEL_W<13> {
        CALSEL_W::new(self)
    }
    #[doc = "Bit 14 - allows to switch from AOP offset trimmed values to AOP offset"]
    #[inline(always)]
    #[must_use]
    pub fn usertrim(&mut self) -> USERTRIM_W<14> {
        USERTRIM_W::new(self)
    }
    #[doc = "Bit 15 - Operational amplifier calibration output"]
    #[inline(always)]
    #[must_use]
    pub fn calout(&mut self) -> CALOUT_W<15> {
        CALOUT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "OPAMP2 control/status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [opamp2_csr](index.html) module"]
pub struct OPAMP2_CSR_SPEC;
impl crate::RegisterSpec for OPAMP2_CSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [opamp2_csr::R](R) reader structure"]
impl crate::Readable for OPAMP2_CSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [opamp2_csr::W](W) writer structure"]
impl crate::Writable for OPAMP2_CSR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OPAMP2_CSR to value 0"]
impl crate::Resettable for OPAMP2_CSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
