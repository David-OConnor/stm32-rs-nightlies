#[doc = "Register `PLL3DIVR` reader"]
pub struct R(crate::R<PLL3DIVR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PLL3DIVR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PLL3DIVR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PLL3DIVR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PLL3DIVR` writer"]
pub struct W(crate::W<PLL3DIVR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PLL3DIVR_SPEC>;
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
impl From<crate::W<PLL3DIVR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PLL3DIVR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PLL3N` reader - Multiplication factor for PLL3VCO Set and reset by software to control the multiplication factor of the VCO. These bits can be written only when the PLL is disabled (PLL3ON = 0 and PLL3RDY = 0). ... ... Others: reserved"]
pub type PLL3N_R = crate::FieldReader<u16, u16>;
#[doc = "Field `PLL3N` writer - Multiplication factor for PLL3VCO Set and reset by software to control the multiplication factor of the VCO. These bits can be written only when the PLL is disabled (PLL3ON = 0 and PLL3RDY = 0). ... ... Others: reserved"]
pub type PLL3N_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PLL3DIVR_SPEC, u16, u16, 9, O>;
#[doc = "Field `PLL3P` reader - PLL3 DIVP division factor Set and reset by software to control the frequency of the pll3_p_ck clock. These bits can be written only when the PLL3 is disabled (PLL3ON = 0 and PLL3RDY = 0). ..."]
pub type PLL3P_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PLL3P` writer - PLL3 DIVP division factor Set and reset by software to control the frequency of the pll3_p_ck clock. These bits can be written only when the PLL3 is disabled (PLL3ON = 0 and PLL3RDY = 0). ..."]
pub type PLL3P_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PLL3DIVR_SPEC, u8, u8, 7, O>;
#[doc = "Field `PLL3Q` reader - PLL3 DIVQ division factor Set and reset by software to control the frequency of the pll3_q_ck clock. These bits can be written only when the PLL3 is disabled (PLL3ON = 0 and PLL3RDY = 0). ..."]
pub type PLL3Q_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PLL3Q` writer - PLL3 DIVQ division factor Set and reset by software to control the frequency of the pll3_q_ck clock. These bits can be written only when the PLL3 is disabled (PLL3ON = 0 and PLL3RDY = 0). ..."]
pub type PLL3Q_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PLL3DIVR_SPEC, u8, u8, 7, O>;
#[doc = "Field `PLL3R` reader - PLL3 DIVR division factor Set and reset by software to control the frequency of the pll3_r_ck clock. These bits can be written only when the PLL1 is disabled (PLL3ON = 0 and PLL3RDY = 0). ..."]
pub type PLL3R_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PLL3R` writer - PLL3 DIVR division factor Set and reset by software to control the frequency of the pll3_r_ck clock. These bits can be written only when the PLL1 is disabled (PLL3ON = 0 and PLL3RDY = 0). ..."]
pub type PLL3R_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PLL3DIVR_SPEC, u8, u8, 7, O>;
impl R {
    #[doc = "Bits 0:8 - Multiplication factor for PLL3VCO Set and reset by software to control the multiplication factor of the VCO. These bits can be written only when the PLL is disabled (PLL3ON = 0 and PLL3RDY = 0). ... ... Others: reserved"]
    #[inline(always)]
    pub fn pll3n(&self) -> PLL3N_R {
        PLL3N_R::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bits 9:15 - PLL3 DIVP division factor Set and reset by software to control the frequency of the pll3_p_ck clock. These bits can be written only when the PLL3 is disabled (PLL3ON = 0 and PLL3RDY = 0). ..."]
    #[inline(always)]
    pub fn pll3p(&self) -> PLL3P_R {
        PLL3P_R::new(((self.bits >> 9) & 0x7f) as u8)
    }
    #[doc = "Bits 16:22 - PLL3 DIVQ division factor Set and reset by software to control the frequency of the pll3_q_ck clock. These bits can be written only when the PLL3 is disabled (PLL3ON = 0 and PLL3RDY = 0). ..."]
    #[inline(always)]
    pub fn pll3q(&self) -> PLL3Q_R {
        PLL3Q_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 24:30 - PLL3 DIVR division factor Set and reset by software to control the frequency of the pll3_r_ck clock. These bits can be written only when the PLL1 is disabled (PLL3ON = 0 and PLL3RDY = 0). ..."]
    #[inline(always)]
    pub fn pll3r(&self) -> PLL3R_R {
        PLL3R_R::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:8 - Multiplication factor for PLL3VCO Set and reset by software to control the multiplication factor of the VCO. These bits can be written only when the PLL is disabled (PLL3ON = 0 and PLL3RDY = 0). ... ... Others: reserved"]
    #[inline(always)]
    #[must_use]
    pub fn pll3n(&mut self) -> PLL3N_W<0> {
        PLL3N_W::new(self)
    }
    #[doc = "Bits 9:15 - PLL3 DIVP division factor Set and reset by software to control the frequency of the pll3_p_ck clock. These bits can be written only when the PLL3 is disabled (PLL3ON = 0 and PLL3RDY = 0). ..."]
    #[inline(always)]
    #[must_use]
    pub fn pll3p(&mut self) -> PLL3P_W<9> {
        PLL3P_W::new(self)
    }
    #[doc = "Bits 16:22 - PLL3 DIVQ division factor Set and reset by software to control the frequency of the pll3_q_ck clock. These bits can be written only when the PLL3 is disabled (PLL3ON = 0 and PLL3RDY = 0). ..."]
    #[inline(always)]
    #[must_use]
    pub fn pll3q(&mut self) -> PLL3Q_W<16> {
        PLL3Q_W::new(self)
    }
    #[doc = "Bits 24:30 - PLL3 DIVR division factor Set and reset by software to control the frequency of the pll3_r_ck clock. These bits can be written only when the PLL1 is disabled (PLL3ON = 0 and PLL3RDY = 0). ..."]
    #[inline(always)]
    #[must_use]
    pub fn pll3r(&mut self) -> PLL3R_W<24> {
        PLL3R_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RCC PLL3 dividers register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pll3divr](index.html) module"]
pub struct PLL3DIVR_SPEC;
impl crate::RegisterSpec for PLL3DIVR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pll3divr::R](R) reader structure"]
impl crate::Readable for PLL3DIVR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pll3divr::W](W) writer structure"]
impl crate::Writable for PLL3DIVR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PLL3DIVR to value 0x0101_0280"]
impl crate::Resettable for PLL3DIVR_SPEC {
    const RESET_VALUE: Self::Ux = 0x0101_0280;
}
