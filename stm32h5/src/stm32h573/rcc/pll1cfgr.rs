#[doc = "Register `PLL1CFGR` reader"]
pub struct R(crate::R<PLL1CFGR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PLL1CFGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PLL1CFGR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PLL1CFGR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PLL1CFGR` writer"]
pub struct W(crate::W<PLL1CFGR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PLL1CFGR_SPEC>;
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
impl From<crate::W<PLL1CFGR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PLL1CFGR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PLL1SRC` reader - DIVMx and PLLs clock source selection Set and reset by software to select the PLL clock source. These bits can be written only when all PLLs are disabled. In order to save power, when no PLL is used, the value of PLL1SRC must be set to '00'. 00: no clock send to DIVMx divider and PLLs (default after reset)."]
pub type PLL1SRC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PLL1SRC` writer - DIVMx and PLLs clock source selection Set and reset by software to select the PLL clock source. These bits can be written only when all PLLs are disabled. In order to save power, when no PLL is used, the value of PLL1SRC must be set to '00'. 00: no clock send to DIVMx divider and PLLs (default after reset)."]
pub type PLL1SRC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PLL1CFGR_SPEC, u8, u8, 2, O>;
#[doc = "Field `PLL1RGE` reader - PLL1 input frequency range Set and reset by software to select the proper reference frequency range used for PLL1. This bit must be written before enabling the PLL1."]
pub type PLL1RGE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PLL1RGE` writer - PLL1 input frequency range Set and reset by software to select the proper reference frequency range used for PLL1. This bit must be written before enabling the PLL1."]
pub type PLL1RGE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PLL1CFGR_SPEC, u8, u8, 2, O>;
#[doc = "Field `PLL1FRACEN` reader - PLL1 fractional latch enable Set and reset by software to latch the content of FRACN1 into the sigma-delta modulator. In order to latch the FRACN1 value into the sigma-delta modulator, PLL1FRACEN must be set to 0, then set to 1. The transition 0 to 1 transfers the content of FRACN1 into the modulator."]
pub type PLL1FRACEN_R = crate::BitReader<bool>;
#[doc = "Field `PLL1FRACEN` writer - PLL1 fractional latch enable Set and reset by software to latch the content of FRACN1 into the sigma-delta modulator. In order to latch the FRACN1 value into the sigma-delta modulator, PLL1FRACEN must be set to 0, then set to 1. The transition 0 to 1 transfers the content of FRACN1 into the modulator."]
pub type PLL1FRACEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, PLL1CFGR_SPEC, bool, O>;
#[doc = "Field `PLL1VCOSEL` reader - PLL1 VCO selection Set and reset by software to select the proper VCO frequency range used for PLL1. This bit must be written before enabling the PLL1."]
pub type PLL1VCOSEL_R = crate::BitReader<bool>;
#[doc = "Field `PLL1VCOSEL` writer - PLL1 VCO selection Set and reset by software to select the proper VCO frequency range used for PLL1. This bit must be written before enabling the PLL1."]
pub type PLL1VCOSEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, PLL1CFGR_SPEC, bool, O>;
#[doc = "Field `DIVM1` reader - prescaler for PLL1 Set and cleared by software to configure the prescaler of the PLL1. The hardware does not allow any modification of this prescaler when PLL1 is enabled (PLL1ON = 1 or PLL1RDY = 1). In order to save power when PLL1 is not used, the value of DIVM1 must be set to 0. ... ..."]
pub type DIVM1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DIVM1` writer - prescaler for PLL1 Set and cleared by software to configure the prescaler of the PLL1. The hardware does not allow any modification of this prescaler when PLL1 is enabled (PLL1ON = 1 or PLL1RDY = 1). In order to save power when PLL1 is not used, the value of DIVM1 must be set to 0. ... ..."]
pub type DIVM1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PLL1CFGR_SPEC, u8, u8, 6, O>;
#[doc = "Field `PLL1PEN` reader - PLL1 DIVP divider output enable Set and reset by software to enable the pll1_p_ck output of the PLL1. This bit can be written only when the PLL1 is disabled (PLL1ON = 0 and PLL1RDY = 0). In order to save power, when the pll1_p_ck output of the PLL1 is not used, the pll1_p_ck must be disabled."]
pub type PLL1PEN_R = crate::BitReader<bool>;
#[doc = "Field `PLL1PEN` writer - PLL1 DIVP divider output enable Set and reset by software to enable the pll1_p_ck output of the PLL1. This bit can be written only when the PLL1 is disabled (PLL1ON = 0 and PLL1RDY = 0). In order to save power, when the pll1_p_ck output of the PLL1 is not used, the pll1_p_ck must be disabled."]
pub type PLL1PEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, PLL1CFGR_SPEC, bool, O>;
#[doc = "Field `PLL1QEN` reader - PLL1 DIVQ divider output enable Set and reset by software to enable the pll1_q_ck output of the PLL1. In order to save power, when the pll1_q_ck output of the PLL1 is not used, the pll1_q_ck must be disabled. This bit can be written only when the PLL1 is disabled (PLL1ON = 0 and PLL1RDY = 0)."]
pub type PLL1QEN_R = crate::BitReader<bool>;
#[doc = "Field `PLL1QEN` writer - PLL1 DIVQ divider output enable Set and reset by software to enable the pll1_q_ck output of the PLL1. In order to save power, when the pll1_q_ck output of the PLL1 is not used, the pll1_q_ck must be disabled. This bit can be written only when the PLL1 is disabled (PLL1ON = 0 and PLL1RDY = 0)."]
pub type PLL1QEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, PLL1CFGR_SPEC, bool, O>;
#[doc = "Field `PLL1REN` reader - PLL1 DIVR divider output enable Set and reset by software to enable the pll1_r_ck output of the PLL1. To save power, DIVR1EN and DIVR1 bits must be set to 0 when the pll1_r_ck is not used. This bit can be written only when the PLL1 is disabled (PLL1ON = 0 and PLL1RDY = 0)."]
pub type PLL1REN_R = crate::BitReader<bool>;
#[doc = "Field `PLL1REN` writer - PLL1 DIVR divider output enable Set and reset by software to enable the pll1_r_ck output of the PLL1. To save power, DIVR1EN and DIVR1 bits must be set to 0 when the pll1_r_ck is not used. This bit can be written only when the PLL1 is disabled (PLL1ON = 0 and PLL1RDY = 0)."]
pub type PLL1REN_W<'a, const O: u8> = crate::BitWriter<'a, u32, PLL1CFGR_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:1 - DIVMx and PLLs clock source selection Set and reset by software to select the PLL clock source. These bits can be written only when all PLLs are disabled. In order to save power, when no PLL is used, the value of PLL1SRC must be set to '00'. 00: no clock send to DIVMx divider and PLLs (default after reset)."]
    #[inline(always)]
    pub fn pll1src(&self) -> PLL1SRC_R {
        PLL1SRC_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - PLL1 input frequency range Set and reset by software to select the proper reference frequency range used for PLL1. This bit must be written before enabling the PLL1."]
    #[inline(always)]
    pub fn pll1rge(&self) -> PLL1RGE_R {
        PLL1RGE_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - PLL1 fractional latch enable Set and reset by software to latch the content of FRACN1 into the sigma-delta modulator. In order to latch the FRACN1 value into the sigma-delta modulator, PLL1FRACEN must be set to 0, then set to 1. The transition 0 to 1 transfers the content of FRACN1 into the modulator."]
    #[inline(always)]
    pub fn pll1fracen(&self) -> PLL1FRACEN_R {
        PLL1FRACEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - PLL1 VCO selection Set and reset by software to select the proper VCO frequency range used for PLL1. This bit must be written before enabling the PLL1."]
    #[inline(always)]
    pub fn pll1vcosel(&self) -> PLL1VCOSEL_R {
        PLL1VCOSEL_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 8:13 - prescaler for PLL1 Set and cleared by software to configure the prescaler of the PLL1. The hardware does not allow any modification of this prescaler when PLL1 is enabled (PLL1ON = 1 or PLL1RDY = 1). In order to save power when PLL1 is not used, the value of DIVM1 must be set to 0. ... ..."]
    #[inline(always)]
    pub fn divm1(&self) -> DIVM1_R {
        DIVM1_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bit 16 - PLL1 DIVP divider output enable Set and reset by software to enable the pll1_p_ck output of the PLL1. This bit can be written only when the PLL1 is disabled (PLL1ON = 0 and PLL1RDY = 0). In order to save power, when the pll1_p_ck output of the PLL1 is not used, the pll1_p_ck must be disabled."]
    #[inline(always)]
    pub fn pll1pen(&self) -> PLL1PEN_R {
        PLL1PEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - PLL1 DIVQ divider output enable Set and reset by software to enable the pll1_q_ck output of the PLL1. In order to save power, when the pll1_q_ck output of the PLL1 is not used, the pll1_q_ck must be disabled. This bit can be written only when the PLL1 is disabled (PLL1ON = 0 and PLL1RDY = 0)."]
    #[inline(always)]
    pub fn pll1qen(&self) -> PLL1QEN_R {
        PLL1QEN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - PLL1 DIVR divider output enable Set and reset by software to enable the pll1_r_ck output of the PLL1. To save power, DIVR1EN and DIVR1 bits must be set to 0 when the pll1_r_ck is not used. This bit can be written only when the PLL1 is disabled (PLL1ON = 0 and PLL1RDY = 0)."]
    #[inline(always)]
    pub fn pll1ren(&self) -> PLL1REN_R {
        PLL1REN_R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - DIVMx and PLLs clock source selection Set and reset by software to select the PLL clock source. These bits can be written only when all PLLs are disabled. In order to save power, when no PLL is used, the value of PLL1SRC must be set to '00'. 00: no clock send to DIVMx divider and PLLs (default after reset)."]
    #[inline(always)]
    #[must_use]
    pub fn pll1src(&mut self) -> PLL1SRC_W<0> {
        PLL1SRC_W::new(self)
    }
    #[doc = "Bits 2:3 - PLL1 input frequency range Set and reset by software to select the proper reference frequency range used for PLL1. This bit must be written before enabling the PLL1."]
    #[inline(always)]
    #[must_use]
    pub fn pll1rge(&mut self) -> PLL1RGE_W<2> {
        PLL1RGE_W::new(self)
    }
    #[doc = "Bit 4 - PLL1 fractional latch enable Set and reset by software to latch the content of FRACN1 into the sigma-delta modulator. In order to latch the FRACN1 value into the sigma-delta modulator, PLL1FRACEN must be set to 0, then set to 1. The transition 0 to 1 transfers the content of FRACN1 into the modulator."]
    #[inline(always)]
    #[must_use]
    pub fn pll1fracen(&mut self) -> PLL1FRACEN_W<4> {
        PLL1FRACEN_W::new(self)
    }
    #[doc = "Bit 5 - PLL1 VCO selection Set and reset by software to select the proper VCO frequency range used for PLL1. This bit must be written before enabling the PLL1."]
    #[inline(always)]
    #[must_use]
    pub fn pll1vcosel(&mut self) -> PLL1VCOSEL_W<5> {
        PLL1VCOSEL_W::new(self)
    }
    #[doc = "Bits 8:13 - prescaler for PLL1 Set and cleared by software to configure the prescaler of the PLL1. The hardware does not allow any modification of this prescaler when PLL1 is enabled (PLL1ON = 1 or PLL1RDY = 1). In order to save power when PLL1 is not used, the value of DIVM1 must be set to 0. ... ..."]
    #[inline(always)]
    #[must_use]
    pub fn divm1(&mut self) -> DIVM1_W<8> {
        DIVM1_W::new(self)
    }
    #[doc = "Bit 16 - PLL1 DIVP divider output enable Set and reset by software to enable the pll1_p_ck output of the PLL1. This bit can be written only when the PLL1 is disabled (PLL1ON = 0 and PLL1RDY = 0). In order to save power, when the pll1_p_ck output of the PLL1 is not used, the pll1_p_ck must be disabled."]
    #[inline(always)]
    #[must_use]
    pub fn pll1pen(&mut self) -> PLL1PEN_W<16> {
        PLL1PEN_W::new(self)
    }
    #[doc = "Bit 17 - PLL1 DIVQ divider output enable Set and reset by software to enable the pll1_q_ck output of the PLL1. In order to save power, when the pll1_q_ck output of the PLL1 is not used, the pll1_q_ck must be disabled. This bit can be written only when the PLL1 is disabled (PLL1ON = 0 and PLL1RDY = 0)."]
    #[inline(always)]
    #[must_use]
    pub fn pll1qen(&mut self) -> PLL1QEN_W<17> {
        PLL1QEN_W::new(self)
    }
    #[doc = "Bit 18 - PLL1 DIVR divider output enable Set and reset by software to enable the pll1_r_ck output of the PLL1. To save power, DIVR1EN and DIVR1 bits must be set to 0 when the pll1_r_ck is not used. This bit can be written only when the PLL1 is disabled (PLL1ON = 0 and PLL1RDY = 0)."]
    #[inline(always)]
    #[must_use]
    pub fn pll1ren(&mut self) -> PLL1REN_W<18> {
        PLL1REN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RCC PLL clock source selection register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pll1cfgr](index.html) module"]
pub struct PLL1CFGR_SPEC;
impl crate::RegisterSpec for PLL1CFGR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pll1cfgr::R](R) reader structure"]
impl crate::Readable for PLL1CFGR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pll1cfgr::W](W) writer structure"]
impl crate::Writable for PLL1CFGR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PLL1CFGR to value 0"]
impl crate::Resettable for PLL1CFGR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
