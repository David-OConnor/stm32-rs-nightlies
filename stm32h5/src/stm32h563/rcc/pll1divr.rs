#[doc = "Register `PLL1DIVR` reader"]
pub struct R(crate::R<PLL1DIVR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PLL1DIVR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PLL1DIVR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PLL1DIVR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PLL1DIVR` writer"]
pub struct W(crate::W<PLL1DIVR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PLL1DIVR_SPEC>;
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
impl From<crate::W<PLL1DIVR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PLL1DIVR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PLL1N` reader - Multiplication factor for PLL1VCO Set and reset by software to control the multiplication factor of the VCO. These bits can be written only when the PLL is disabled (PLL1ON = 0 and PLL1RDY = 0). ... ... Others: reserved"]
pub type PLL1N_R = crate::FieldReader<u16, u16>;
#[doc = "Field `PLL1N` writer - Multiplication factor for PLL1VCO Set and reset by software to control the multiplication factor of the VCO. These bits can be written only when the PLL is disabled (PLL1ON = 0 and PLL1RDY = 0). ... ... Others: reserved"]
pub type PLL1N_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PLL1DIVR_SPEC, u16, u16, 9, O>;
#[doc = "Field `PLL1P` reader - PLL1 DIVP division factor Set and reset by software to control the frequency of the pll1_p_ck clock. These bits can be written only when the PLL1 is disabled (PLL1ON = 0 and PLL1RDY = 0). Note that odd division factors are not allowed. ..."]
pub type PLL1P_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PLL1P` writer - PLL1 DIVP division factor Set and reset by software to control the frequency of the pll1_p_ck clock. These bits can be written only when the PLL1 is disabled (PLL1ON = 0 and PLL1RDY = 0). Note that odd division factors are not allowed. ..."]
pub type PLL1P_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PLL1DIVR_SPEC, u8, u8, 7, O>;
#[doc = "Field `PLL1Q` reader - PLL1 DIVQ division factor Set and reset by software to control the frequency of the pll1_q_ck clock. These bits can be written only when the PLL1 is disabled (PLL1ON = 0 and PLL1RDY = 0). ..."]
pub type PLL1Q_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PLL1Q` writer - PLL1 DIVQ division factor Set and reset by software to control the frequency of the pll1_q_ck clock. These bits can be written only when the PLL1 is disabled (PLL1ON = 0 and PLL1RDY = 0). ..."]
pub type PLL1Q_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PLL1DIVR_SPEC, u8, u8, 7, O>;
#[doc = "Field `PLL1R` reader - PLL1 DIVR division factor Set and reset by software to control the frequency of the pll1_r_ck clock. These bits can be written only when the PLL1 is disabled (PLL1ON = 0 and PLL1RDY = 0). ..."]
pub type PLL1R_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PLL1R` writer - PLL1 DIVR division factor Set and reset by software to control the frequency of the pll1_r_ck clock. These bits can be written only when the PLL1 is disabled (PLL1ON = 0 and PLL1RDY = 0). ..."]
pub type PLL1R_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PLL1DIVR_SPEC, u8, u8, 7, O>;
impl R {
    #[doc = "Bits 0:8 - Multiplication factor for PLL1VCO Set and reset by software to control the multiplication factor of the VCO. These bits can be written only when the PLL is disabled (PLL1ON = 0 and PLL1RDY = 0). ... ... Others: reserved"]
    #[inline(always)]
    pub fn pll1n(&self) -> PLL1N_R {
        PLL1N_R::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bits 9:15 - PLL1 DIVP division factor Set and reset by software to control the frequency of the pll1_p_ck clock. These bits can be written only when the PLL1 is disabled (PLL1ON = 0 and PLL1RDY = 0). Note that odd division factors are not allowed. ..."]
    #[inline(always)]
    pub fn pll1p(&self) -> PLL1P_R {
        PLL1P_R::new(((self.bits >> 9) & 0x7f) as u8)
    }
    #[doc = "Bits 16:22 - PLL1 DIVQ division factor Set and reset by software to control the frequency of the pll1_q_ck clock. These bits can be written only when the PLL1 is disabled (PLL1ON = 0 and PLL1RDY = 0). ..."]
    #[inline(always)]
    pub fn pll1q(&self) -> PLL1Q_R {
        PLL1Q_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 24:30 - PLL1 DIVR division factor Set and reset by software to control the frequency of the pll1_r_ck clock. These bits can be written only when the PLL1 is disabled (PLL1ON = 0 and PLL1RDY = 0). ..."]
    #[inline(always)]
    pub fn pll1r(&self) -> PLL1R_R {
        PLL1R_R::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:8 - Multiplication factor for PLL1VCO Set and reset by software to control the multiplication factor of the VCO. These bits can be written only when the PLL is disabled (PLL1ON = 0 and PLL1RDY = 0). ... ... Others: reserved"]
    #[inline(always)]
    #[must_use]
    pub fn pll1n(&mut self) -> PLL1N_W<0> {
        PLL1N_W::new(self)
    }
    #[doc = "Bits 9:15 - PLL1 DIVP division factor Set and reset by software to control the frequency of the pll1_p_ck clock. These bits can be written only when the PLL1 is disabled (PLL1ON = 0 and PLL1RDY = 0). Note that odd division factors are not allowed. ..."]
    #[inline(always)]
    #[must_use]
    pub fn pll1p(&mut self) -> PLL1P_W<9> {
        PLL1P_W::new(self)
    }
    #[doc = "Bits 16:22 - PLL1 DIVQ division factor Set and reset by software to control the frequency of the pll1_q_ck clock. These bits can be written only when the PLL1 is disabled (PLL1ON = 0 and PLL1RDY = 0). ..."]
    #[inline(always)]
    #[must_use]
    pub fn pll1q(&mut self) -> PLL1Q_W<16> {
        PLL1Q_W::new(self)
    }
    #[doc = "Bits 24:30 - PLL1 DIVR division factor Set and reset by software to control the frequency of the pll1_r_ck clock. These bits can be written only when the PLL1 is disabled (PLL1ON = 0 and PLL1RDY = 0). ..."]
    #[inline(always)]
    #[must_use]
    pub fn pll1r(&mut self) -> PLL1R_W<24> {
        PLL1R_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RCC PLL1 dividers register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pll1divr](index.html) module"]
pub struct PLL1DIVR_SPEC;
impl crate::RegisterSpec for PLL1DIVR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pll1divr::R](R) reader structure"]
impl crate::Readable for PLL1DIVR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pll1divr::W](W) writer structure"]
impl crate::Writable for PLL1DIVR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PLL1DIVR to value 0x0101_0280"]
impl crate::Resettable for PLL1DIVR_SPEC {
    const RESET_VALUE: Self::Ux = 0x0101_0280;
}
