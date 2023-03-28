#[doc = "Register `PLL2CFGR` reader"]
pub struct R(crate::R<PLL2CFGR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PLL2CFGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PLL2CFGR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PLL2CFGR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PLL2CFGR` writer"]
pub struct W(crate::W<PLL2CFGR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PLL2CFGR_SPEC>;
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
impl From<crate::W<PLL2CFGR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PLL2CFGR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PLL2SRC` reader - DIVMx and PLLs clock source selection Set and reset by software to select the PLL clock source. These bits can be written only when all PLLs are disabled. In order to save power, when no PLL is used, the value of PLL2SRC must be set to '00'."]
pub type PLL2SRC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PLL2SRC` writer - DIVMx and PLLs clock source selection Set and reset by software to select the PLL clock source. These bits can be written only when all PLLs are disabled. In order to save power, when no PLL is used, the value of PLL2SRC must be set to '00'."]
pub type PLL2SRC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PLL2CFGR_SPEC, u8, u8, 2, O>;
#[doc = "Field `PLL2RGE` reader - PLL2 input frequency range Set and reset by software to select the proper reference frequency range used for PLL2. These bits must be written before enabling the PLL2."]
pub type PLL2RGE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PLL2RGE` writer - PLL2 input frequency range Set and reset by software to select the proper reference frequency range used for PLL2. These bits must be written before enabling the PLL2."]
pub type PLL2RGE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PLL2CFGR_SPEC, u8, u8, 2, O>;
#[doc = "Field `PLL2FRACEN` reader - PLL2 fractional latch enable Set and reset by software to enable the pll2_p_ck output of the PLL2. To save power, when the pll2_p_ck output of the PLL2 is not used, the pll2_p_ck must be disabled."]
pub type PLL2FRACEN_R = crate::BitReader<bool>;
#[doc = "Field `PLL2FRACEN` writer - PLL2 fractional latch enable Set and reset by software to enable the pll2_p_ck output of the PLL2. To save power, when the pll2_p_ck output of the PLL2 is not used, the pll2_p_ck must be disabled."]
pub type PLL2FRACEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, PLL2CFGR_SPEC, bool, O>;
#[doc = "Field `PLL2VCOSEL` reader - PLL2 VCO selection Set and reset by software to select the proper VCO frequency range used for PLL2. This bit must be written before enabling the PLL2."]
pub type PLL2VCOSEL_R = crate::BitReader<bool>;
#[doc = "Field `PLL2VCOSEL` writer - PLL2 VCO selection Set and reset by software to select the proper VCO frequency range used for PLL2. This bit must be written before enabling the PLL2."]
pub type PLL2VCOSEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, PLL2CFGR_SPEC, bool, O>;
#[doc = "Field `DIVM2` reader - prescaler for PLL2 Set and cleared by software to configure the prescaler of the PLL2. The hardware does not allow any modification of this prescaler when PLL2 is enabled (PLL2ON = 1 or PLL2RDY = 1). In order to save power when PLL2 is not used, the value of DIVM2 must be set to 0. ... ..."]
pub type DIVM2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DIVM2` writer - prescaler for PLL2 Set and cleared by software to configure the prescaler of the PLL2. The hardware does not allow any modification of this prescaler when PLL2 is enabled (PLL2ON = 1 or PLL2RDY = 1). In order to save power when PLL2 is not used, the value of DIVM2 must be set to 0. ... ..."]
pub type DIVM2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PLL2CFGR_SPEC, u8, u8, 6, O>;
#[doc = "Field `PLL2PEN` reader - PLL2 DIVP divider output enable Set and reset by software to enable the pll2_p_ck output of the PLL2. To save power, when the pll2_p_ck output of the PLL2 is not used, the pll2_p_ck must be disabled."]
pub type PLL2PEN_R = crate::BitReader<bool>;
#[doc = "Field `PLL2PEN` writer - PLL2 DIVP divider output enable Set and reset by software to enable the pll2_p_ck output of the PLL2. To save power, when the pll2_p_ck output of the PLL2 is not used, the pll2_p_ck must be disabled."]
pub type PLL2PEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, PLL2CFGR_SPEC, bool, O>;
#[doc = "Field `PLL2QEN` reader - PLL2 DIVQ divider output enable Set and reset by software to enable the pll2_q_ck output of the PLL2. To save power, when the pll2_q_ck output of the PLL2 is not used, the pll2_q_ck must be disabled."]
pub type PLL2QEN_R = crate::BitReader<bool>;
#[doc = "Field `PLL2QEN` writer - PLL2 DIVQ divider output enable Set and reset by software to enable the pll2_q_ck output of the PLL2. To save power, when the pll2_q_ck output of the PLL2 is not used, the pll2_q_ck must be disabled."]
pub type PLL2QEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, PLL2CFGR_SPEC, bool, O>;
#[doc = "Field `PLL2REN` reader - PLL2 DIVR divider output enable Set and reset by software to enable the pll2_r_ck output of the PLL2. To save power, DIVR2EN and DIVR2 bits must be set to 0 when the pll2_r_ck is not used."]
pub type PLL2REN_R = crate::BitReader<bool>;
#[doc = "Field `PLL2REN` writer - PLL2 DIVR divider output enable Set and reset by software to enable the pll2_r_ck output of the PLL2. To save power, DIVR2EN and DIVR2 bits must be set to 0 when the pll2_r_ck is not used."]
pub type PLL2REN_W<'a, const O: u8> = crate::BitWriter<'a, u32, PLL2CFGR_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:1 - DIVMx and PLLs clock source selection Set and reset by software to select the PLL clock source. These bits can be written only when all PLLs are disabled. In order to save power, when no PLL is used, the value of PLL2SRC must be set to '00'."]
    #[inline(always)]
    pub fn pll2src(&self) -> PLL2SRC_R {
        PLL2SRC_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - PLL2 input frequency range Set and reset by software to select the proper reference frequency range used for PLL2. These bits must be written before enabling the PLL2."]
    #[inline(always)]
    pub fn pll2rge(&self) -> PLL2RGE_R {
        PLL2RGE_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - PLL2 fractional latch enable Set and reset by software to enable the pll2_p_ck output of the PLL2. To save power, when the pll2_p_ck output of the PLL2 is not used, the pll2_p_ck must be disabled."]
    #[inline(always)]
    pub fn pll2fracen(&self) -> PLL2FRACEN_R {
        PLL2FRACEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - PLL2 VCO selection Set and reset by software to select the proper VCO frequency range used for PLL2. This bit must be written before enabling the PLL2."]
    #[inline(always)]
    pub fn pll2vcosel(&self) -> PLL2VCOSEL_R {
        PLL2VCOSEL_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 8:13 - prescaler for PLL2 Set and cleared by software to configure the prescaler of the PLL2. The hardware does not allow any modification of this prescaler when PLL2 is enabled (PLL2ON = 1 or PLL2RDY = 1). In order to save power when PLL2 is not used, the value of DIVM2 must be set to 0. ... ..."]
    #[inline(always)]
    pub fn divm2(&self) -> DIVM2_R {
        DIVM2_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bit 16 - PLL2 DIVP divider output enable Set and reset by software to enable the pll2_p_ck output of the PLL2. To save power, when the pll2_p_ck output of the PLL2 is not used, the pll2_p_ck must be disabled."]
    #[inline(always)]
    pub fn pll2pen(&self) -> PLL2PEN_R {
        PLL2PEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - PLL2 DIVQ divider output enable Set and reset by software to enable the pll2_q_ck output of the PLL2. To save power, when the pll2_q_ck output of the PLL2 is not used, the pll2_q_ck must be disabled."]
    #[inline(always)]
    pub fn pll2qen(&self) -> PLL2QEN_R {
        PLL2QEN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - PLL2 DIVR divider output enable Set and reset by software to enable the pll2_r_ck output of the PLL2. To save power, DIVR2EN and DIVR2 bits must be set to 0 when the pll2_r_ck is not used."]
    #[inline(always)]
    pub fn pll2ren(&self) -> PLL2REN_R {
        PLL2REN_R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - DIVMx and PLLs clock source selection Set and reset by software to select the PLL clock source. These bits can be written only when all PLLs are disabled. In order to save power, when no PLL is used, the value of PLL2SRC must be set to '00'."]
    #[inline(always)]
    #[must_use]
    pub fn pll2src(&mut self) -> PLL2SRC_W<0> {
        PLL2SRC_W::new(self)
    }
    #[doc = "Bits 2:3 - PLL2 input frequency range Set and reset by software to select the proper reference frequency range used for PLL2. These bits must be written before enabling the PLL2."]
    #[inline(always)]
    #[must_use]
    pub fn pll2rge(&mut self) -> PLL2RGE_W<2> {
        PLL2RGE_W::new(self)
    }
    #[doc = "Bit 4 - PLL2 fractional latch enable Set and reset by software to enable the pll2_p_ck output of the PLL2. To save power, when the pll2_p_ck output of the PLL2 is not used, the pll2_p_ck must be disabled."]
    #[inline(always)]
    #[must_use]
    pub fn pll2fracen(&mut self) -> PLL2FRACEN_W<4> {
        PLL2FRACEN_W::new(self)
    }
    #[doc = "Bit 5 - PLL2 VCO selection Set and reset by software to select the proper VCO frequency range used for PLL2. This bit must be written before enabling the PLL2."]
    #[inline(always)]
    #[must_use]
    pub fn pll2vcosel(&mut self) -> PLL2VCOSEL_W<5> {
        PLL2VCOSEL_W::new(self)
    }
    #[doc = "Bits 8:13 - prescaler for PLL2 Set and cleared by software to configure the prescaler of the PLL2. The hardware does not allow any modification of this prescaler when PLL2 is enabled (PLL2ON = 1 or PLL2RDY = 1). In order to save power when PLL2 is not used, the value of DIVM2 must be set to 0. ... ..."]
    #[inline(always)]
    #[must_use]
    pub fn divm2(&mut self) -> DIVM2_W<8> {
        DIVM2_W::new(self)
    }
    #[doc = "Bit 16 - PLL2 DIVP divider output enable Set and reset by software to enable the pll2_p_ck output of the PLL2. To save power, when the pll2_p_ck output of the PLL2 is not used, the pll2_p_ck must be disabled."]
    #[inline(always)]
    #[must_use]
    pub fn pll2pen(&mut self) -> PLL2PEN_W<16> {
        PLL2PEN_W::new(self)
    }
    #[doc = "Bit 17 - PLL2 DIVQ divider output enable Set and reset by software to enable the pll2_q_ck output of the PLL2. To save power, when the pll2_q_ck output of the PLL2 is not used, the pll2_q_ck must be disabled."]
    #[inline(always)]
    #[must_use]
    pub fn pll2qen(&mut self) -> PLL2QEN_W<17> {
        PLL2QEN_W::new(self)
    }
    #[doc = "Bit 18 - PLL2 DIVR divider output enable Set and reset by software to enable the pll2_r_ck output of the PLL2. To save power, DIVR2EN and DIVR2 bits must be set to 0 when the pll2_r_ck is not used."]
    #[inline(always)]
    #[must_use]
    pub fn pll2ren(&mut self) -> PLL2REN_W<18> {
        PLL2REN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RCC PLL clock source selection register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pll2cfgr](index.html) module"]
pub struct PLL2CFGR_SPEC;
impl crate::RegisterSpec for PLL2CFGR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pll2cfgr::R](R) reader structure"]
impl crate::Readable for PLL2CFGR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pll2cfgr::W](W) writer structure"]
impl crate::Writable for PLL2CFGR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PLL2CFGR to value 0"]
impl crate::Resettable for PLL2CFGR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
