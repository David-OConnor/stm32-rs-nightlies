#[doc = "Register `PLL3CFGR` reader"]
pub struct R(crate::R<PLL3CFGR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PLL3CFGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PLL3CFGR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PLL3CFGR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PLL3CFGR` writer"]
pub struct W(crate::W<PLL3CFGR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PLL3CFGR_SPEC>;
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
impl From<crate::W<PLL3CFGR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PLL3CFGR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PLL3SRC` reader - DIVMx and PLLs clock source selection Set and reset by software to select the PLL clock source. These bits can be written only when all PLLs are disabled. In order to save power, when no PLL is used, the value of PLL3SRC must be set to '00'."]
pub type PLL3SRC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PLL3SRC` writer - DIVMx and PLLs clock source selection Set and reset by software to select the PLL clock source. These bits can be written only when all PLLs are disabled. In order to save power, when no PLL is used, the value of PLL3SRC must be set to '00'."]
pub type PLL3SRC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PLL3CFGR_SPEC, u8, u8, 2, O>;
#[doc = "Field `PLL3RGE` reader - PLL3 input frequency range Set and reset by software to select the proper reference frequency range used for PLL3. This bit must be written before enabling the PLL3."]
pub type PLL3RGE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PLL3RGE` writer - PLL3 input frequency range Set and reset by software to select the proper reference frequency range used for PLL3. This bit must be written before enabling the PLL3."]
pub type PLL3RGE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PLL3CFGR_SPEC, u8, u8, 2, O>;
#[doc = "Field `PLL3FRACEN` reader - PLL3 fractional latch enable Set and reset by software to latch the content of FRACN3 into the sigma-delta modulator. In order to latch the FRACN3 value into the sigma-delta modulator, PLL3FRACEN must be set to 0, then set to 1. The transition 0 to 1 transfers the content of FRACN3 into the modulator."]
pub type PLL3FRACEN_R = crate::BitReader<bool>;
#[doc = "Field `PLL3FRACEN` writer - PLL3 fractional latch enable Set and reset by software to latch the content of FRACN3 into the sigma-delta modulator. In order to latch the FRACN3 value into the sigma-delta modulator, PLL3FRACEN must be set to 0, then set to 1. The transition 0 to 1 transfers the content of FRACN3 into the modulator."]
pub type PLL3FRACEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, PLL3CFGR_SPEC, bool, O>;
#[doc = "Field `PLL3VCOSEL` reader - PLL3 VCO selection Set and reset by software to select the proper VCO frequency range used for PLL3. This bit must be written before enabling the PLL3."]
pub type PLL3VCOSEL_R = crate::BitReader<bool>;
#[doc = "Field `PLL3VCOSEL` writer - PLL3 VCO selection Set and reset by software to select the proper VCO frequency range used for PLL3. This bit must be written before enabling the PLL3."]
pub type PLL3VCOSEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, PLL3CFGR_SPEC, bool, O>;
#[doc = "Field `DIVM3` reader - prescaler for PLL3 Set and cleared by software to configure the prescaler of the PLL3. The hardware does not allow any modification of this prescaler when PLL3 is enabled (PLL3ON = 1 or PLL3RDY = 1). In order to save power when PLL3 is not used, the value of DIVM2 must be set to 0. ... ..."]
pub type DIVM3_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DIVM3` writer - prescaler for PLL3 Set and cleared by software to configure the prescaler of the PLL3. The hardware does not allow any modification of this prescaler when PLL3 is enabled (PLL3ON = 1 or PLL3RDY = 1). In order to save power when PLL3 is not used, the value of DIVM2 must be set to 0. ... ..."]
pub type DIVM3_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PLL3CFGR_SPEC, u8, u8, 6, O>;
#[doc = "Field `PLL3PEN` reader - PLL3 DIVP divider output enable Set and reset by software to enable the pll3_p_ck output of the PLL3. To save power, when the pll3_p_ck output of the PLL3 is not used, the pll3_p_ck must be disabled."]
pub type PLL3PEN_R = crate::BitReader<bool>;
#[doc = "Field `PLL3PEN` writer - PLL3 DIVP divider output enable Set and reset by software to enable the pll3_p_ck output of the PLL3. To save power, when the pll3_p_ck output of the PLL3 is not used, the pll3_p_ck must be disabled."]
pub type PLL3PEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, PLL3CFGR_SPEC, bool, O>;
#[doc = "Field `PLL3QEN` reader - PLL3 DIVQ divider output enable Set and reset by software to enable the pll3_q_ck output of the PLL3. To save power, when the pll3_q_ck output of the PLL3 is not used, the pll3_q_ck must be disabled."]
pub type PLL3QEN_R = crate::BitReader<bool>;
#[doc = "Field `PLL3QEN` writer - PLL3 DIVQ divider output enable Set and reset by software to enable the pll3_q_ck output of the PLL3. To save power, when the pll3_q_ck output of the PLL3 is not used, the pll3_q_ck must be disabled."]
pub type PLL3QEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, PLL3CFGR_SPEC, bool, O>;
#[doc = "Field `PLL3REN` reader - PLL3 DIVR divider output enable Set and reset by software to enable the pll3_r_ck output of the PLL3. To save power, DIVR2EN and DIVR2 bits must be set to 0 when the pll3_r_ck is not used."]
pub type PLL3REN_R = crate::BitReader<bool>;
#[doc = "Field `PLL3REN` writer - PLL3 DIVR divider output enable Set and reset by software to enable the pll3_r_ck output of the PLL3. To save power, DIVR2EN and DIVR2 bits must be set to 0 when the pll3_r_ck is not used."]
pub type PLL3REN_W<'a, const O: u8> = crate::BitWriter<'a, u32, PLL3CFGR_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:1 - DIVMx and PLLs clock source selection Set and reset by software to select the PLL clock source. These bits can be written only when all PLLs are disabled. In order to save power, when no PLL is used, the value of PLL3SRC must be set to '00'."]
    #[inline(always)]
    pub fn pll3src(&self) -> PLL3SRC_R {
        PLL3SRC_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - PLL3 input frequency range Set and reset by software to select the proper reference frequency range used for PLL3. This bit must be written before enabling the PLL3."]
    #[inline(always)]
    pub fn pll3rge(&self) -> PLL3RGE_R {
        PLL3RGE_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - PLL3 fractional latch enable Set and reset by software to latch the content of FRACN3 into the sigma-delta modulator. In order to latch the FRACN3 value into the sigma-delta modulator, PLL3FRACEN must be set to 0, then set to 1. The transition 0 to 1 transfers the content of FRACN3 into the modulator."]
    #[inline(always)]
    pub fn pll3fracen(&self) -> PLL3FRACEN_R {
        PLL3FRACEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - PLL3 VCO selection Set and reset by software to select the proper VCO frequency range used for PLL3. This bit must be written before enabling the PLL3."]
    #[inline(always)]
    pub fn pll3vcosel(&self) -> PLL3VCOSEL_R {
        PLL3VCOSEL_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 8:13 - prescaler for PLL3 Set and cleared by software to configure the prescaler of the PLL3. The hardware does not allow any modification of this prescaler when PLL3 is enabled (PLL3ON = 1 or PLL3RDY = 1). In order to save power when PLL3 is not used, the value of DIVM2 must be set to 0. ... ..."]
    #[inline(always)]
    pub fn divm3(&self) -> DIVM3_R {
        DIVM3_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bit 16 - PLL3 DIVP divider output enable Set and reset by software to enable the pll3_p_ck output of the PLL3. To save power, when the pll3_p_ck output of the PLL3 is not used, the pll3_p_ck must be disabled."]
    #[inline(always)]
    pub fn pll3pen(&self) -> PLL3PEN_R {
        PLL3PEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - PLL3 DIVQ divider output enable Set and reset by software to enable the pll3_q_ck output of the PLL3. To save power, when the pll3_q_ck output of the PLL3 is not used, the pll3_q_ck must be disabled."]
    #[inline(always)]
    pub fn pll3qen(&self) -> PLL3QEN_R {
        PLL3QEN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - PLL3 DIVR divider output enable Set and reset by software to enable the pll3_r_ck output of the PLL3. To save power, DIVR2EN and DIVR2 bits must be set to 0 when the pll3_r_ck is not used."]
    #[inline(always)]
    pub fn pll3ren(&self) -> PLL3REN_R {
        PLL3REN_R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - DIVMx and PLLs clock source selection Set and reset by software to select the PLL clock source. These bits can be written only when all PLLs are disabled. In order to save power, when no PLL is used, the value of PLL3SRC must be set to '00'."]
    #[inline(always)]
    #[must_use]
    pub fn pll3src(&mut self) -> PLL3SRC_W<0> {
        PLL3SRC_W::new(self)
    }
    #[doc = "Bits 2:3 - PLL3 input frequency range Set and reset by software to select the proper reference frequency range used for PLL3. This bit must be written before enabling the PLL3."]
    #[inline(always)]
    #[must_use]
    pub fn pll3rge(&mut self) -> PLL3RGE_W<2> {
        PLL3RGE_W::new(self)
    }
    #[doc = "Bit 4 - PLL3 fractional latch enable Set and reset by software to latch the content of FRACN3 into the sigma-delta modulator. In order to latch the FRACN3 value into the sigma-delta modulator, PLL3FRACEN must be set to 0, then set to 1. The transition 0 to 1 transfers the content of FRACN3 into the modulator."]
    #[inline(always)]
    #[must_use]
    pub fn pll3fracen(&mut self) -> PLL3FRACEN_W<4> {
        PLL3FRACEN_W::new(self)
    }
    #[doc = "Bit 5 - PLL3 VCO selection Set and reset by software to select the proper VCO frequency range used for PLL3. This bit must be written before enabling the PLL3."]
    #[inline(always)]
    #[must_use]
    pub fn pll3vcosel(&mut self) -> PLL3VCOSEL_W<5> {
        PLL3VCOSEL_W::new(self)
    }
    #[doc = "Bits 8:13 - prescaler for PLL3 Set and cleared by software to configure the prescaler of the PLL3. The hardware does not allow any modification of this prescaler when PLL3 is enabled (PLL3ON = 1 or PLL3RDY = 1). In order to save power when PLL3 is not used, the value of DIVM2 must be set to 0. ... ..."]
    #[inline(always)]
    #[must_use]
    pub fn divm3(&mut self) -> DIVM3_W<8> {
        DIVM3_W::new(self)
    }
    #[doc = "Bit 16 - PLL3 DIVP divider output enable Set and reset by software to enable the pll3_p_ck output of the PLL3. To save power, when the pll3_p_ck output of the PLL3 is not used, the pll3_p_ck must be disabled."]
    #[inline(always)]
    #[must_use]
    pub fn pll3pen(&mut self) -> PLL3PEN_W<16> {
        PLL3PEN_W::new(self)
    }
    #[doc = "Bit 17 - PLL3 DIVQ divider output enable Set and reset by software to enable the pll3_q_ck output of the PLL3. To save power, when the pll3_q_ck output of the PLL3 is not used, the pll3_q_ck must be disabled."]
    #[inline(always)]
    #[must_use]
    pub fn pll3qen(&mut self) -> PLL3QEN_W<17> {
        PLL3QEN_W::new(self)
    }
    #[doc = "Bit 18 - PLL3 DIVR divider output enable Set and reset by software to enable the pll3_r_ck output of the PLL3. To save power, DIVR2EN and DIVR2 bits must be set to 0 when the pll3_r_ck is not used."]
    #[inline(always)]
    #[must_use]
    pub fn pll3ren(&mut self) -> PLL3REN_W<18> {
        PLL3REN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RCC PLL clock source selection register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pll3cfgr](index.html) module"]
pub struct PLL3CFGR_SPEC;
impl crate::RegisterSpec for PLL3CFGR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pll3cfgr::R](R) reader structure"]
impl crate::Readable for PLL3CFGR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pll3cfgr::W](W) writer structure"]
impl crate::Writable for PLL3CFGR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PLL3CFGR to value 0"]
impl crate::Resettable for PLL3CFGR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
