#[doc = "Register `PLL1FRACR` reader"]
pub struct R(crate::R<PLL1FRACR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PLL1FRACR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PLL1FRACR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PLL1FRACR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PLL1FRACR` writer"]
pub struct W(crate::W<PLL1FRACR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PLL1FRACR_SPEC>;
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
impl From<crate::W<PLL1FRACR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PLL1FRACR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PLL1FRACN` reader - fractional part of the multiplication factor for PLL1 VCO Set and reset by software to control the fractional part of the multiplication factor of the VCO. These bits can be written at any time, allowing dynamic fine-tuning of the PLL1 VCO. The software must set correctly these bits to insure that the VCO output frequency is between its valid frequency range, that is: * 128 to 560 MHz if PLL1VCOSEL = 0 * 150 to 420 MHz if PLL1VCOSEL = 1 VCO output frequency = Fref1_ck x (PLL1N + (PLL1FRACN / 213)), with * PLL1N between 8 and 420 * PLL1FRACN can be between 0 and 213- 1 * The input frequency Fref1_ck must be between 1 and 16 MHz. To change the PLL1FRACN value on-the-fly even if the PLL is enabled, the application must proceed as follows: * Set the bit PLL1FRACEN to 0 * Write the new fractional value into PLL1FRACN * Set the bit PLL1FRACEN to 1"]
pub type PLL1FRACN_R = crate::FieldReader<u16, u16>;
#[doc = "Field `PLL1FRACN` writer - fractional part of the multiplication factor for PLL1 VCO Set and reset by software to control the fractional part of the multiplication factor of the VCO. These bits can be written at any time, allowing dynamic fine-tuning of the PLL1 VCO. The software must set correctly these bits to insure that the VCO output frequency is between its valid frequency range, that is: * 128 to 560 MHz if PLL1VCOSEL = 0 * 150 to 420 MHz if PLL1VCOSEL = 1 VCO output frequency = Fref1_ck x (PLL1N + (PLL1FRACN / 213)), with * PLL1N between 8 and 420 * PLL1FRACN can be between 0 and 213- 1 * The input frequency Fref1_ck must be between 1 and 16 MHz. To change the PLL1FRACN value on-the-fly even if the PLL is enabled, the application must proceed as follows: * Set the bit PLL1FRACEN to 0 * Write the new fractional value into PLL1FRACN * Set the bit PLL1FRACEN to 1"]
pub type PLL1FRACN_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PLL1FRACR_SPEC, u16, u16, 13, O>;
impl R {
    #[doc = "Bits 3:15 - fractional part of the multiplication factor for PLL1 VCO Set and reset by software to control the fractional part of the multiplication factor of the VCO. These bits can be written at any time, allowing dynamic fine-tuning of the PLL1 VCO. The software must set correctly these bits to insure that the VCO output frequency is between its valid frequency range, that is: * 128 to 560 MHz if PLL1VCOSEL = 0 * 150 to 420 MHz if PLL1VCOSEL = 1 VCO output frequency = Fref1_ck x (PLL1N + (PLL1FRACN / 213)), with * PLL1N between 8 and 420 * PLL1FRACN can be between 0 and 213- 1 * The input frequency Fref1_ck must be between 1 and 16 MHz. To change the PLL1FRACN value on-the-fly even if the PLL is enabled, the application must proceed as follows: * Set the bit PLL1FRACEN to 0 * Write the new fractional value into PLL1FRACN * Set the bit PLL1FRACEN to 1"]
    #[inline(always)]
    pub fn pll1fracn(&self) -> PLL1FRACN_R {
        PLL1FRACN_R::new(((self.bits >> 3) & 0x1fff) as u16)
    }
}
impl W {
    #[doc = "Bits 3:15 - fractional part of the multiplication factor for PLL1 VCO Set and reset by software to control the fractional part of the multiplication factor of the VCO. These bits can be written at any time, allowing dynamic fine-tuning of the PLL1 VCO. The software must set correctly these bits to insure that the VCO output frequency is between its valid frequency range, that is: * 128 to 560 MHz if PLL1VCOSEL = 0 * 150 to 420 MHz if PLL1VCOSEL = 1 VCO output frequency = Fref1_ck x (PLL1N + (PLL1FRACN / 213)), with * PLL1N between 8 and 420 * PLL1FRACN can be between 0 and 213- 1 * The input frequency Fref1_ck must be between 1 and 16 MHz. To change the PLL1FRACN value on-the-fly even if the PLL is enabled, the application must proceed as follows: * Set the bit PLL1FRACEN to 0 * Write the new fractional value into PLL1FRACN * Set the bit PLL1FRACEN to 1"]
    #[inline(always)]
    #[must_use]
    pub fn pll1fracn(&mut self) -> PLL1FRACN_W<3> {
        PLL1FRACN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RCC PLL1 fractional divider register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pll1fracr](index.html) module"]
pub struct PLL1FRACR_SPEC;
impl crate::RegisterSpec for PLL1FRACR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pll1fracr::R](R) reader structure"]
impl crate::Readable for PLL1FRACR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pll1fracr::W](W) writer structure"]
impl crate::Writable for PLL1FRACR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PLL1FRACR to value 0"]
impl crate::Resettable for PLL1FRACR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
